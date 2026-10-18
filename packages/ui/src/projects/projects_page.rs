use api::account_projects::account_projects_controller::{
    batch_upsert_account_projects, get_account_projects,
};
use api::auth::auth_controller::me;
use api::projects::projects_controller::get_projects_by_ids;
use api::users::users_controller::get_users_by_project_id;
use dioxus::core::Task;
use dioxus::fullstack::Json;
use dioxus::prelude::*;
use crate::tid;
use shared::{Account, BatchProject, EditableProject, ProjectDto, ProjectPayload, ProjectStatus};
use uuid::Uuid;

use std::collections::VecDeque;

use crate::common::{
    apply_pull, clear_user_id, copy_missing, copy_text, error_message, haptic, is_offline_error, key_of,
    leave_project_and_forget, left_elsewhere, mark_converted, mark_synced, pending,
    read_from_ls, remove_project, set_cached_projects_list, share_link_for, share_text, sleep, to_push, update_ls,
    update_project_fresh, write_queue, QUICK_ADD,
    write_to_ls,
    ConfirmModal, DropdownButton, DropdownItem, Mascot, MascotPose, PageTitle,
    Flash, Haptic, LocalStorageState, OpKind, ProjectEdit, ProjectKey, ProjectStatusItems, ProjectTile, PullToRefresh,
    QueuedOp, SpeedDialAction, SpeedDialFab, LEAVE_CONFIRM_MESSAGE, LEAVE_CONFIRM_TITLE,
};
use crate::common::fx_cache::read_fx;
use crate::crypto::decrypt_json;
use crate::decrypted::{project_status, status_edit};
use crate::expenses::helpers::my_spending::{cached_ledgers, load_ledgers, GroupLedger};
use crate::expenses::{EditProjectModal, OPEN_REIMBURSEMENTS};
use crate::icons::{ChevronRightIcon, ImportIcon, LinkIcon, LockIcon, PlusIcon, UsersIcon, WalletIcon};
use crate::payment_methods::refill_project_copies;
use crate::projects::home::{row_subtitle, BalanceCard, RowBalance};
use crate::projects::home_balances::{debts, glance, home_currency, totals, Debt, ProjectGlance};
use crate::projects::personal::{personal_project, PersonalExpenseSheet};
use crate::projects::personal_cards::{is_personal_dto, PersonalCard, PersonalCreateCard, PersonalCreateSheet};
use crate::projects::{AddProjectModal, ImportTricountModal, JoinProjectModal};
use crate::friends::{InvitationsCard, NotificationsBell};
use crate::route::Route;

/// Whether anything has been archived. The filter is noise until something has been, so the page
/// renders no control at all while this is false - which is most devices, most of the time.
///
/// `Closed` does not count: it is a distinct status that stays in the list, and only `Archived`
/// hides a row.
fn has_archived(state: &LocalStorageState) -> bool {
    state.cached_projects_list.iter().flatten().any(|p| status_of(state, p) == ProjectStatus::Archived)
}

fn status_of(state: &LocalStorageState, p: &ProjectDto) -> ProjectStatus {
    project_status(state.projects.iter().find(|lp| lp.project_id == p.id).and_then(key_of).as_ref(), p)
}

#[component]
pub fn ProjectsPage() -> Element {
    let auth_ctx = use_context::<Signal<Option<Account>>>();
    let account_key_ctx = use_context::<Signal<Option<[u8; 32]>>>();
    let ls_ctx = use_context::<Signal<LocalStorageState>>();
    // False first so SSR and the first client render agree; the client-only effect corrects it
    // after hydration. `peek` stops the effect subscribing to its own write.
    let mut show_archived = use_signal(|| false);
    use_effect(move || {
        let stored = read_from_ls().show_archived;
        if stored != *show_archived.peek() {
            show_archived.set(stored);
        }
    });

    let mut show_add_project = use_signal(|| false);
    let mut show_import_tricount = use_signal(|| false);
    let mut show_join_project = use_signal(|| false);
    let mut show_personal_expense = use_signal(|| false);
    let mut show_personal_create = use_signal(|| false);

    // Reconciliation, both directions. Reads `account_key_ctx` as well as `auth_ctx` so it re-runs
    // when either resolves - on a fresh login both are set before this page renders, but a session
    // restored from the cookie resolves `me()` after the key is already seeded, and a device that
    // signs in later must still push what it was holding beforehand.
    use_effect(move || {
        let Some(account) = auth_ctx() else {
            return;
        };
        let account_key = account_key_ctx();
        spawn(async move {
            let Ok(server) = get_account_projects().await else { return };

            // Forget first: a project this account was seen holding that it no longer holds was
            // left on another device. Dropped before the push reads the store, or it would be
            // pushed straight back and the leave undone. See common::account_sync.
            let left = left_elsewhere(&read_from_ls(), &server, account.id);
            if !left.is_empty() {
                update_ls(ls_ctx, |state| {
                    for project_id in &left {
                        remove_project(state, *project_id);
                    }
                });
            }

            // Push next: what this device holds and the account does not. Without it a project
            // created while logged out never reaches `account_projects`, so no other device - and
            // no escrow - ever learns about it.
            let push = to_push(&read_from_ls(), &server, account_key.as_ref(), Some(&account));
            let mut rejected: Vec<Uuid> = vec![];
            let mut accepted: Vec<Uuid> = vec![];
            if !push.is_empty() {
                if let Ok(result) = batch_upsert_account_projects(Json(push)).await {
                    rejected = result.identity_rejected;
                    accepted = result.accepted;
                }
            }

            // A project claimed here that the account's shared payment methods never reached -
            // saved on a device without its key, or claimed since. Refilled from a `me()` made
            // now, never from `account` above: that one is as old as this session, and a copy
            // derived from it could restore what another device has since withdrawn.
            if let Some(ak) = account_key {
                if copy_missing(&read_from_ls(), &server) {
                    if let Ok(Some(fresh)) = me().await {
                        let _ = refill_project_copies(&fresh, &ak).await;
                        // Guarded: this effect reads `auth_ctx`, and a refill the server refused
                        // would otherwise re-run it forever.
                        if fresh != account {
                            let mut auth_ctx = auth_ctx;
                            auth_ctx.set(Some(fresh));
                        }
                    }
                }
            }

            update_ls(ls_ctx, |state| {
                apply_pull(state, &server, account_key.as_ref(), account.id);
                mark_synced(state, &accepted, account.id);
                // The identity this device pushed is held by another account. Drop it locally so the
                // "who am I?" picker reopens, and so `to_push` stops re-sending a claim that will be
                // refused again on every load - the server has no second way to say no.
                //
                // Only when the account holds no identity of its own here. If it does, `apply_pull`
                // just installed that one and it is the right answer: clearing on top of it would
                // throw away a perfectly valid identity and demand a pointless re-pick.
                for project_id in &rejected {
                    let account_has_one = server
                        .iter()
                        .any(|p| p.project_id == *project_id && p.user_id.is_some());
                    if !account_has_one {
                        clear_user_id(state, *project_id);
                    }
                }
            });

            // The leak docs/plans/participant-links-encryption.md closes ties these expenses to this
            // account's email, so its devices convert every project they hold, opened or not. One
            // at a time, once per device and project.
            let pending: Vec<(Uuid, [u8; 32])> = read_from_ls()
                .projects
                .iter()
                .filter(|p| !p.converted)
                .filter_map(|p| Some((p.project_id, key_of(p)?)))
                .collect();
            for (project_id, key) in pending {
                if crate::conversion::convert(key, project_id).await.is_ok() {
                    update_ls(ls_ctx, |state| mark_converted(state, project_id));
                }
            }
        });
    });

    let cached_list: Option<Vec<ProjectDto>> = ls_ctx().cached_projects_list.clone();

    let resource_version = use_context::<Signal<u64>>();

    let mut projects = use_resource(move || {
        let ls_ctx = ls_ctx;
        async move {
            let _v = resource_version();
            // Every membership, archived or not: the toggle only filters what renders. Skipping the
            // archived ones meant a project un-archived on another device stayed archived here.
            let ids: Vec<Uuid> = ls_ctx().projects.iter().map(|p| p.project_id).collect();
            if ids.is_empty() {
                return Ok(vec![]);
            }
            let result = get_projects_by_ids(Json(BatchProject { ids })).await;
            if let Ok(ref list) = result {
                // read_from_ls() and no `ls_ctx.set()` - deliberately *not* `update_ls`: this
                // closure reads `ls_ctx()` above, so refreshing the signal here would re-run the
                // resource forever. Safe as a disk-only write only because the cached list is read
                // back through `read_from_ls` (`has_archived` below) or the signal's own seed, and
                // because every other writer now bases on disk too.
                let mut state = read_from_ls();
                set_cached_projects_list(&mut state, list.clone());
                write_to_ls(&state);
            }
            result
        }
    });

    // Personal projects are for accounts only: signed out, the shortcut never shows.
    let personal = use_memo(move || match (&*projects.read(), auth_ctx()) {
        (Some(Ok(list)), Some(_)) => personal_project(list, &ls_ctx.read()),
        _ => None,
    });

    // `counted://add`: the form once the list is known, or the page as is when there is no
    // personal project to add to - its card offers to create one.
    use_effect(move || {
        if QUICK_ADD() && projects.read().is_some() {
            *QUICK_ADD.write() = false;
            if personal().is_some() {
                show_personal_expense.set(true);
            }
        }
    });

    // Every group's ledger: this device's cache at once, then one conditional sync per project in
    // place of the participant fetch each card used to make. Empty on the server render, so the
    // first client render matches it.
    let mut ledgers: Signal<Vec<GroupLedger>> = use_signal(Vec::new);
    let mut synced = use_signal(|| false);
    use_effect(move || {
        let _ = projects.read();
        ledgers.set(cached_ledgers(&read_from_ls(), Uuid::nil()));
        spawn(async move {
            ledgers.set(load_ledgers(Uuid::nil()).await);
            synced.set(true);
        });
    });
    let currency = use_memo(move || home_currency(personal().map(|p| p.currency), &ledgers.read()));
    let home_totals = use_memo(move || totals(&ledgers.read(), &currency(), read_fx().as_ref()));
    let home_debts = use_memo(move || debts(&ledgers.read()));
    let glance_of = move |id: Uuid| ledgers.read().iter().find(|l| l.project_id == id).map(glance);

    let mut flash = use_context::<Signal<Option<Flash>>>();
    let on_pay = move |project_id: Uuid| {
        *OPEN_REIMBURSEMENTS.write() = Some(project_id);
        navigator().push(Route::ExpensesPage { project_id });
    };
    let on_remind = move |debt: Debt| {
        let Some(key) = read_from_ls().projects.iter().find(|p| p.project_id == debt.project_id).and_then(key_of) else {
            return;
        };
        let text = format!(
            "{}\n{}",
            tid!("home-remind-message", project: debt.project_name.clone()),
            share_link_for(debt.project_id, &key)
        );
        spawn(async move {
            if share_text(&text).await {
                return;
            }
            if copy_text(&text).await {
                flash.set(Some(Flash::ok(tid!("link-copied"))));
            } else {
                flash.set(Some(Flash::err(tid!("copy-link-failed"))));
            }
        });
    };

    // read_from_ls(), not `cached_list`: archiving while online goes through `update_project_by_id`
    // and the resource writes the merged cache with `write_to_ls` alone - `ls_ctx` is never set on
    // that path, so it stays stale for the rest of the session and the filter would not appear
    // until a reload. Reading `projects` re-runs this once the resource has written.
    // False first, like `show_archived`: SSR reads an empty store, so reading it during render
    // would add a tablist the server markup lacks and break hydration.
    let mut archived_exist = use_signal(|| false);
    use_effect(move || {
        let _ = projects.read();
        let exist = has_archived(&read_from_ls());
        if exist != *archived_exist.peek() {
            archived_exist.set(exist);
        }
    });

    let choose_archived = move |value: bool| {
        // Not `update_ls`: refreshing the signal would restart the resource, which reads `ls_ctx`,
        // for a flag that only filters the rows already loaded. Safe as a disk-only write because
        // `show_archived` is read back with `read_from_ls`, never off the signal. Copied locally so
        // the closure stays Fn + Copy and both tabs can hold it.
        let mut show_archived = show_archived;
        show_archived.set(value);
        let mut state = read_from_ls();
        state.show_archived = value;
        write_to_ls(&state);
    };

    rsx! {
        PageTitle { text: format!("{} · counted", tid!("nav-projects")) }
        div { class: "container app-container bg-base-100 overflow-auto p-4 pb-24 max-w-md w-full mx-auto flex flex-col gap-4",
            PullToRefresh { on_refresh: move |_| projects.restart(), busy: pending(&projects) }
            header { class: "flex items-center justify-between min-h-14 pl-1",
                h1 { id: "projects-title", class: "font-display font-bold text-[26px] tracking-tight", {tid!("nav-projects")} }
                NotificationsBell {}
            }

            if !ledgers.read().is_empty() {
                BalanceCard {
                    totals: home_totals(),
                    currency: currency(),
                    debts: home_debts(),
                    on_pay,
                    on_remind,
                }
            }

            InvitationsCard { on_accepted: move |_| projects.restart() }

            // `show_archived` is an *include* flag, so the second tab is "All", not "Archived" -
            // labelling it the latter would promise a list that never renders.
            if archived_exist() {
                div { role: "tablist", class: "tabs tabs-box bg-base-300 shadow-none grid grid-flow-col auto-cols-fr",
                    button {
                        id: "filter-active",
                        r#type: "button",
                        role: "tab",
                        aria_selected: !show_archived(),
                        class: if show_archived() { "tab flex-1 text-sm text-base-content/70" } else { "tab tab-active flex-1 text-sm font-semibold" },
                        onclick: move |_| choose_archived(false),
                        {tid!("projects-filter-active")}
                    }
                    button {
                        id: "filter-all",
                        r#type: "button",
                        role: "tab",
                        aria_selected: show_archived(),
                        class: if show_archived() { "tab tab-active flex-1 text-sm font-semibold" } else { "tab flex-1 text-sm text-base-content/70" },
                        onclick: move |_| choose_archived(true),
                        {tid!("projects-filter-all")}
                    }
                }
            }

            match &*projects.read() {
                None => rsx! {
                    div { class: "flex flex-col gap-3", aria_hidden: "true",
                        for _ in 0..3u8 {
                            div { class: "card bg-base-100 shadow-soft",
                                div { class: "card-body p-4 gap-2",
                                    div { class: "skeleton h-4 w-3/4" }
                                    div { class: "skeleton h-3 w-1/2" }
                                    div { class: "flex justify-between mt-1",
                                        div { class: "skeleton h-4 w-16" }
                                        div { class: "skeleton h-6 w-24" }
                                    }
                                }
                            }
                        }
                    }
                },
                Some(Err(e)) if is_offline_error(e) => {
                    match &cached_list {
                        Some(list) => {
                            let filtered: Vec<ProjectDto> = list
                                .iter()
                                .filter(|p| show_archived() || status_of(&ls_ctx.peek(), p) != ProjectStatus::Archived)
                                .cloned()
                                .collect();
                            let mine = filtered.iter().find(|p| is_personal_dto(p, &ls_ctx.peek())).cloned();
                            let filtered: Vec<ProjectDto> =
                                filtered.into_iter().filter(|p| Some(p.id) != mine.as_ref().map(|m| m.id)).collect();
                            rsx! {
                                div { role: "alert",
                                    class: "alert alert-warning alert-soft text-sm",
                                    {tid!("projects-offline-banner")}
                                }
                                if let Some(project) = mine {
                                    PersonalCard { project }
                                }
                                if filtered.is_empty() {
                                    div { class: "flex flex-col items-center gap-2 py-12 text-base-content/70",
                                        Mascot { pose: MascotPose::Empty }
                                        span { class: "font-semibold", {tid!("projects-empty")} }
                                    }
                                } else {
                                    div { id: "projects-list", class: "shrink-0 rounded-[20px] bg-base-100 shadow-soft flex flex-col overflow-hidden",
                                        for (i, project) in filtered.into_iter().enumerate() {
                                            if i > 0 {
                                                div { class: "h-px bg-base-200 ml-[70px]" }
                                            }
                                            ProjectCard { glance: glance_of(project.id), synced: synced(), project: project.clone(), on_change: move |_| projects.restart() }
                                        }
                                    }
                                }
                            }
                        }
                        None => rsx! {
                            div { class: "flex flex-col items-center gap-4 py-12 text-base-content/70",
                                Mascot { pose: MascotPose::Offline }
                                p { class: "font-semibold", {tid!("projects-no-local-data")} }
                                p { class: "text-sm text-center",
                                    {tid!("projects-no-local-data-hint")}
                                }
                            }
                        }
                    }
                }
                Some(Err(e)) => rsx! {
                    div { role: "alert", class: "alert alert-error", "{error_message(e)}" }
                },
                Some(Ok(list)) => {
                    // One per account, pinned above the groups; while there is none, offered last.
                    let mine = list.iter().find(|p| is_personal_dto(p, &ls_ctx.peek())).cloned();
                    let pinned = mine.clone().filter(|p| show_archived() || status_of(&ls_ctx.peek(), p) != ProjectStatus::Archived);
                    let filtered: Vec<ProjectDto> = list
                        .iter()
                        .filter(|p| show_archived() || status_of(&ls_ctx.peek(), p) != ProjectStatus::Archived)
                        .filter(|p| Some(p.id) != mine.as_ref().map(|m| m.id))
                        .cloned()
                        .collect();
                    rsx! {
                        if let Some(project) = pinned {
                            PersonalCard { project }
                        }
                        if filtered.is_empty() {
                            div { class: "flex flex-col items-center gap-2 py-12 text-base-content/70",
                                Mascot { pose: MascotPose::Empty }
                                span { class: "font-semibold", {tid!("projects-empty")} }
                            }
                            GroupCreateCard { on_create: move |_| show_add_project.set(true) }
                        } else {
                            div { id: "projects-list", class: "shrink-0 rounded-[20px] bg-base-100 shadow-soft flex flex-col overflow-hidden",
                                for (i, project) in filtered.into_iter().enumerate() {
                                    if i > 0 {
                                        div { class: "h-px bg-base-200 ml-[70px]" }
                                    }
                                    ProjectCard { glance: glance_of(project.id), synced: synced(), project: project.clone(), on_change: move |_| projects.restart() }
                                }
                            }
                        }
                        if mine.is_none() {
                            PersonalCreateCard {
                                signed_in: auth_ctx().is_some(),
                                on_create: move |_| show_personal_create.set(true),
                            }
                        }
                    }
                }
            }

            // FAB - fixed above the bottom dock, expands into import/join/create
            SpeedDialFab { id: "projects-fab", label: tid!("projects-add"),
                SpeedDialAction {
                    id: "import-tricount-action",
                    label: tid!("projects-import-tricount"),
                    icon: rsx! { ImportIcon {} },
                    onclick: move |_| show_import_tricount.set(true),
                }
                SpeedDialAction {
                    id: "join-project-action",
                    label: tid!("projects-join"),
                    icon: rsx! { LinkIcon {} },
                    onclick: move |_| show_join_project.set(true),
                }
                SpeedDialAction {
                    id: "add-project-action",
                    label: tid!("projects-create"),
                    icon: rsx! { PlusIcon {} },
                    onclick: move |_| show_add_project.set(true),
                }
                if personal().is_some() {
                    SpeedDialAction {
                        id: "personal-expense-action",
                        label: tid!("projects-personal-expense"),
                        icon: rsx! { WalletIcon {} },
                        primary: true,
                        onclick: move |_| show_personal_expense.set(true),
                    }
                }
            }

            if show_personal_create() {
                PersonalCreateSheet { on_close: move |_| show_personal_create.set(false) }
            }

            if show_personal_expense() {
                if let Some(project) = personal() {
                    PersonalExpenseSheet { project, on_close: move |_| show_personal_expense.set(false) }
                }
            }

            if show_add_project() {
                AddProjectModal { on_close: move |_| show_add_project.set(false) }
            }
            if show_import_tricount() {
                ImportTricountModal { on_close: move |_| show_import_tricount.set(false) }
            }
            if show_join_project() {
                JoinProjectModal { on_close: move |_| show_join_project.set(false) }
            }
        }
    }
}

#[component]
fn GroupCreateCard(on_create: EventHandler<()>) -> Element {
    rsx! {
        div { id: "group-create-card", class: "flex flex-col gap-3 rounded-box bg-primary/10 border border-primary/25 p-4",
            div { class: "flex items-center gap-3",
                span { class: "size-10 shrink-0 rounded-xl bg-primary text-primary-content flex items-center justify-center",
                    UsersIcon {}
                }
                span { class: "flex-1 min-w-0 flex flex-col",
                    span { class: "font-semibold", {tid!("group-create-title")} }
                    span { class: "text-xs text-base-content/70", {tid!("group-create-hint")} }
                }
            }
            button {
                id: "group-create-btn",
                r#type: "button",
                class: "btn btn-primary w-full rounded-full",
                onclick: move |_| on_create.call(()),
                {tid!("projects-create")}
            }
        }
    }
}

#[derive(PartialEq, Props, Clone)]
struct ProjectCardProps {
    project: ProjectDto,
    /// What this device knows of the group's money and members; `None` before its first sync.
    glance: Option<ProjectGlance>,
    /// The page's syncs have answered, so `glance` is as fresh as it will get.
    synced: bool,
    on_change: EventHandler<()>,
}

/// How long a finger has to stay down before the card's actions menu opens.
const LONG_PRESS_MS: u32 = 500;

#[component]
fn ProjectCard(props: ProjectCardProps) -> Element {
    let nav = use_navigator();
    let project = props.project.clone();
    let project_id = project.id;
    let mut show_edit_modal = use_signal(|| false);

    let is_online = use_context::<Signal<bool>>();
    let mut pending_ops = use_context::<Signal<VecDeque<QueuedOp>>>();
    let ls_ctx = use_context::<Signal<LocalStorageState>>();
    let mut flash = use_context::<Signal<Option<Flash>>>();
    let mut show_leave_confirm = use_signal(|| false);
    let mut show_unlock_modal = use_signal(|| false);

    // `ls_ctx()`, not `read_from_ls()`: the signal is authoritative now that every writer goes
    // through `update_ls`, and reading it subscribes - so a key arriving (escrow unwrapped by the
    // account sync, or the unlock modal) re-renders this card instead of leaving it locked until a
    // reload. It also stops a full store parse per card per render.
    let stored_key: Option<[u8; 32]> =
        ls_ctx().projects.iter().find(|p| p.project_id == project_id).and_then(key_of);

    let mut key_ctx = use_context_provider(|| ProjectKey(Signal::new(stored_key))).0;
    // The provider's initializer runs once, so without this the context stays `None` after an
    // unlock and `EditProjectModal` would refuse to save on a card that is visibly decrypted.
    use_effect(move || {
        if *key_ctx.peek() != stored_key {
            key_ctx.set(stored_key);
        }
    });

    // Long press anywhere opens the same menu as the `...` trigger, so the whole card is the
    // target rather than a 32px button.
    let mut menu_open = use_signal(|| false);
    let mut press_task: Signal<Option<Task>> = use_signal(|| None);
    let mut long_pressed = use_signal(|| false);
    let mut cancel_press = move || {
        if let Some(t) = press_task.write().take() {
            t.cancel();
        }
    };

    let decrypted_project =
        stored_key.and_then(|k| decrypt_json::<ProjectPayload>(&k, &project.payload).ok());
    // No key, or a key that does not open this payload. Everything derived below is empty in that
    // case, which is what used to render as a blank card - so the card renders a locked state
    // instead, and offers the one action that fixes it.
    let locked = decrypted_project.is_none();
    let project_status =
        decrypted_project.as_ref().and_then(|p| p.status.clone()).unwrap_or_else(|| project.status.clone());
    let display_name = decrypted_project.as_ref().map(|p| p.name.clone()).unwrap_or_default();
    let display_currency =
        decrypted_project.as_ref().map(|p| p.currency.clone()).unwrap_or_default();
    let today = chrono::Utc::now().date_naive();
    let subtitle = {
        let mut parts: Vec<String> = props.glance.as_ref().map(|g| row_subtitle(g, today)).into_iter().collect();
        match project_status {
            ProjectStatus::Ongoing => {}
            ProjectStatus::Closed => parts.push(tid!("status-closed")),
            ProjectStatus::Archived => parts.push(tid!("status-archived")),
        }
        if parts.is_empty() { display_currency.clone() } else { parts.join(" · ") }
    };

    // Only for the edit sheet, and only once it opens: the row itself takes its member count from
    // the page's sync, so the list makes no participant request of its own.
    let users = use_resource(move || async move {
        if !show_edit_modal() {
            return Ok(vec![]);
        }
        get_users_by_project_id(project_id).await
    });

    // Offline this queues for replay and updates the cached list optimistically, so the card
    // reflects the new status until the queue drains.
    let queue_label = display_name.clone();
    let status_source = project.clone();
    let apply_status = move |status: ProjectStatus| {
        let payload = stored_key
            .and_then(|k| status_edit(&k, &status_source, &status).ok())
            .and_then(|e| e.payload);
        let on_change = props.on_change;
        if !is_online() {
            pending_ops.write().push_back(QueuedOp {
                id: Uuid::new_v4(),
                label: queue_label.clone(),
                op: OpKind::EditProject(ProjectEdit {
                    project_id,
                    name_description: None,
                    status: Some(status.clone()),
                    history: None,
                }),
            });
            write_queue(&pending_ops.read());
            update_ls(ls_ctx, |state| {
                if let Some(ref mut list) = state.cached_projects_list {
                    if let Some(p) = list.iter_mut().find(|p| p.id == project_id) {
                        p.status = status;
                        if let Some(payload) = payload {
                            p.payload = payload;
                        }
                    }
                }
            });
            on_change.call(());
            return;
        }
        spawn(async move {
            // Without the key nothing can be written into the payload; the column is all there is.
            let edit = |stored: &ProjectDto| match stored_key {
                Some(k) => status_edit(&k, stored, &status),
                None => Ok(EditableProject {
                    id: project_id,
                    payload: None,
                    status: Some(status.clone()),
                    history: None,
                    expected_iv: None,
                    notify_closed: false,
                }),
            };
            let _ = update_project_fresh(project_id, edit).await;
            on_change.call(());
        });
    };

    rsx! {
        div {
            id: "project-card-{project_id}",
            class: "relative flex items-center",
            "data-synced": props.synced,
            // No `ontouchmove`: on mobile every dispatched event costs a *synchronous* XHR to
            // wry's custom protocol, so a ~60Hz listener blocks the main thread through every
            // scroll. The two cancel events below fire once per gesture, when the UA claims it for
            // panning - so scrolling off a card never opens the menu.
            ontouchstart: move |_| {
                long_pressed.set(false);
                let task = spawn(async move {
                    sleep(LONG_PRESS_MS).await;
                    // Inside the task, not on touchstart: the press has to actually complete. This
                    // is the only signal that the hold worked - the menu appears in the same frame.
                    haptic(Haptic::Medium);
                    long_pressed.set(true);
                    menu_open.set(true);
                });
                press_task.set(Some(task));
            },
            ontouchcancel: move |_| cancel_press(),
            onpointercancel: move |_| cancel_press(),
            ontouchend: move |e| {
                cancel_press();
                // preventDefault on touchend suppresses the synthesised click; without it the
                // click hits the menu's own `fixed inset-0` scrim and closes it instantly. Only
                // touchend does this - pointerup would not.
                if long_pressed() {
                    e.prevent_default();
                }
            },
            div {
            class: if locked { "flex-1 min-w-0 flex items-center gap-3 ps-4 pe-3 py-3 cursor-pointer active:bg-base-200" } else { "flex-1 min-w-0 flex items-center gap-3 px-4 py-3 cursor-pointer active:bg-base-200" },
            role: "button",
            tabindex: "0",
            // Locked opens the unlock modal instead: `ExpensesPage` would only reach
            // `MissingKeyScreen`, which is the same dead end one navigation further away.
            onclick: move |_| {
                // Second line of defence for a click that slipped past prevent_default.
                if long_pressed() {
                    long_pressed.set(false);
                    return;
                }
                if locked {
                    show_unlock_modal.set(true);
                } else {
                    nav.push(Route::ExpensesPage { project_id });
                }
            },
            onkeydown: move |e| {
                let key = e.key();
                if key == Key::Enter || key == Key::Character(" ".to_string()) {
                    e.prevent_default();
                    if locked {
                        show_unlock_modal.set(true);
                    } else {
                        nav.push(Route::ExpensesPage { project_id });
                    }
                }
            },
            if locked {
                span { class: "size-[42px] shrink-0 rounded-[13px] bg-tile-amber text-tile-amber-content flex items-center justify-center",
                    LockIcon {}
                }
            } else {
                ProjectTile { name: display_name.clone(), id: project_id }
            }
            span { class: "flex-1 min-w-0 flex flex-col gap-0.5",
                if locked {
                    span { class: "font-semibold text-[15px] truncate text-base-content/70", {tid!("missing-encryption-key-title")} }
                    span { class: "text-xs text-base-content/70 line-clamp-2", {tid!("project-locked-hint")} }
                } else {
                    span { class: "font-semibold text-[15px] truncate", "{display_name}" }
                    span { class: "text-xs text-base-content/70 truncate", "{subtitle}" }
                }
            }
            if locked {
                span { class: "shrink-0 text-base-content/50", ChevronRightIcon {} }
            } else if let Some(balance) = props.glance.as_ref().and_then(|g| g.balance) {
                RowBalance { balance, currency: display_currency.clone() }
            }
            }
            // The row's actions open on a long press; the trigger stays in the tab order for the
            // keyboard and is drawn only for a locked row, whose actions have no other way in.
            DropdownButton {
                open: menu_open,
                label: if locked { tid!("project-actions") } else { format!("{}, {display_name}", tid!("project-actions")) },
                button_class: if locked { "btn btn-ghost btn-circle btn-sm relative z-50 me-4" } else { "sr-only focus:not-sr-only focus:me-4" },
                if locked {
                    DropdownItem {
                        variant: "primary",
                        label: tid!("project-unlock"),
                        onclick: move |_| show_unlock_modal.set(true),
                    }
                } else {
                    DropdownItem {
                        variant: "primary",
                        label: tid!("edit"),
                        onclick: move |_| show_edit_modal.set(true),
                    }
                }
                DropdownItem {
                    variant: "error",
                    label: tid!("leave"),
                    onclick: move |_| show_leave_confirm.set(true),
                }
                // Hidden while locked: a status change writes an encrypted history
                // entry, and this device has nothing to encrypt it with.
                if !locked {
                    ProjectStatusItems {
                        status: project_status.clone(),
                        on_apply: apply_status,
                    }
                }
            }
        }
        if show_unlock_modal() {
            JoinProjectModal {
                expect_project_id: project_id,
                on_close: move |_| show_unlock_modal.set(false),
            }
        }
        if show_edit_modal() {
            if let Some(Ok(user_list)) = users.read().as_ref() {
                    EditProjectModal {
                        project: project.clone(),
                        users: user_list.clone(),
                        on_close: move |_| show_edit_modal.set(false),
                        on_saved: move |_| {
                            show_edit_modal.set(false);
                            props.on_change.call(());
                        },
                    }
            }
        }
        if show_leave_confirm() {
            ConfirmModal {
                title: tid!(LEAVE_CONFIRM_TITLE),
                message: tid!(LEAVE_CONFIRM_MESSAGE),
                confirm_label: tid!("leave"),
                on_cancel: move |_| show_leave_confirm.set(false),
                on_confirm: move |_| {
                    let on_change = props.on_change;
                    show_leave_confirm.set(false);
                    spawn(async move {
                        match leave_project_and_forget(ls_ctx, project_id).await {
                            // `leave_project_and_forget` already refreshed `ls_ctx`.
                            Ok(_) => {}
                            // Membership survives on the server, so keep the local entry -
                            // dropping it leaves a member nobody can remove.
                            Err(e) => flash.set(Some(Flash::err(e))),
                        }
                        on_change.call(());
                    });
                },
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::{test_fixtures, LocalStorageProject};

    fn id(n: u8) -> Uuid {
        Uuid::from_bytes([n; 16])
    }

    fn project(n: u8, status: ProjectStatus) -> ProjectDto {
        ProjectDto {
            status,
            ..test_fixtures::make_project(&test_fixtures::TEST_KEY, id(n), "p", "EUR")
        }
    }

    fn state(member_ids: &[u8], cached: Option<Vec<ProjectDto>>) -> LocalStorageState {
        LocalStorageState {
            projects: member_ids
                .iter()
                .map(|n| LocalStorageProject { project_id: id(*n), ..Default::default() })
                .collect(),
            cached_projects_list: cached,
            ..Default::default()
        }
    }

    /// The filter is rendered only when this is true, so a false positive puts a dead control on
    /// the page of every user who has never archived anything.
    #[test]
    fn a_device_with_no_cache_shows_no_archived_filter() {
        assert!(!has_archived(&state(&[1, 2], None)));
    }

    #[test]
    fn ongoing_and_closed_projects_alone_show_no_archived_filter() {
        let s = state(
            &[1, 2],
            Some(vec![project(1, ProjectStatus::Ongoing), project(2, ProjectStatus::Closed)]),
        );
        assert!(!has_archived(&s));
    }

    #[test]
    fn one_archived_project_is_enough_to_show_the_filter() {
        let s = state(
            &[1, 2],
            Some(vec![project(1, ProjectStatus::Ongoing), project(2, ProjectStatus::Archived)]),
        );
        assert!(has_archived(&s));
    }
}
