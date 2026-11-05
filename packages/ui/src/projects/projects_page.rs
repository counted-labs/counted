use api::account_projects::account_projects_controller::{
    batch_upsert_account_projects, get_account_projects,
};
use api::auth::auth_controller::me;
use api::users::users_controller::get_users_by_project_id;
use dioxus::core::Task;
use dioxus::fullstack::Json;
use dioxus::prelude::*;
use crate::tid;
use shared::{Account, AccountProject, EditableProject, ProjectDto, ProjectPayload, ProjectStatus};
use uuid::Uuid;

use std::collections::{HashSet, VecDeque};

use crate::common::{
    apply_pull, clear_user_id, copy_missing, copy_text, error_message, haptic, is_offline_error, is_rate_limited, key_of,
    forget_left, leave_project_and_forget, left_elsewhere, mark_converted, mark_synced, pending,
    read_from_ls, set_cached_projects_list, share_link_for, share_text, sleep, to_push, update_ls,
    update_project_fresh, write_queue, QUICK_ADD,
    write_to_ls,
    ConfirmModal, Mascot, MascotPose, PageTitle,
    Flash, Haptic, LocalStorageState, OpKind, ProjectEdit, ProjectKey, ProjectTile, PullToRefresh,
    QueuedOp, SpeedDialAction, SpeedDialFab, LEAVE_CONFIRM_MESSAGE, LEAVE_CONFIRM_TITLE,
};
use crate::common::fx_cache::read_fx;
use crate::crypto::decrypt_json;
use crate::conversion::{conversion_plan, rows_verdicts, Verdict};
use crate::rotation_flow::{adopt_all, lost_projects, rotated_among, rotated_ids};
use crate::decrypted::{project_status, status_edit};
use crate::expenses::helpers::my_spending::{cached_ledgers, sync_all, synced_ledgers, GroupLedger};
use crate::expenses::{EditProjectModal, OPEN_REIMBURSEMENTS};
use crate::icons::{ChevronRightIcon, ImportIcon, KeyIcon, LinkIcon, LockIcon, PlusIcon, UsersIcon, WalletIcon};
use crate::payment_methods::refill_project_copies;
use crate::projects::home::{row_subtitle, BalanceCard, RowBalance};
use crate::projects::home_balances::{debts, glance, home_currency, totals, Debt, ProjectGlance};
use crate::projects::personal::{personal_project, PersonalExpenseSheet};
use crate::projects::project_row_sheet::ProjectRowSheet;
use crate::projects::personal_cards::{is_personal_dto, PersonalCard, PersonalCreateCard, PersonalCreateSheet};
use crate::projects::{AddProjectModal, ImportTricountModal, JoinProjectModal};
use crate::friends::friends_service::my_private_key;
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

const CONVERSION_PACE_MS: u32 = 1_000;

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

    // What the account and the list's sync each answered, for the conversion further down. It needs
    // both, and must not start before reconciliation has dropped the projects left elsewhere.
    let account_answer: Signal<Option<Vec<AccountProject>>> = use_signal(|| None);
    let mut verdicts: Signal<Option<Vec<Verdict>>> = use_signal(|| None);

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
            // left on another device, unless a rotation removed the account (D14). Dropped before
            // the push reads the store, or it would be pushed straight back and the leave undone.
            // See common::account_sync.
            let reset = crate::auth::recovery_flow::memberships_reset(&read_from_ls(), &account);
            let left = if reset { vec![] } else { left_elsewhere(&read_from_ls(), &server, account.id) };
            if !left.is_empty() {
                let Ok(rotated) = rotated_among(&read_from_ls(), &left).await else { return };
                update_ls(ls_ctx, |state| forget_left(state, &left, &rotated));
            }

            // Push next: what this device holds and the account does not. Without it a project
            // created while logged out never reaches `account_projects`, so no other device - and
            // no escrow - ever learns about it.
            let push = to_push(&read_from_ls(), &server, account_key.as_ref(), Some(&account));
            let mut rejected: Vec<Uuid> = vec![];
            let mut accepted: Vec<Uuid> = vec![];
            let mut pushed = true;
            if !push.is_empty() {
                let written = batch_upsert_account_projects(Json(push)).await;
                crate::auth::account_key::note(&written);
                pushed = written.is_ok();
                if let Ok(result) = written {
                    rejected = result.identity_rejected;
                    accepted = result.accepted;
                }
            }
            if reset && pushed {
                update_ls(ls_ctx, |state| {
                    state.membership_epochs.insert(account.id, account.membership_epoch);
                });
            }

            // A project claimed here that the account's shared payment methods never reached -
            // saved on a device without its key, or claimed since. Refilled from a `me()` made
            // now, never from `account` above: that one is as old as this session, and a copy
            // derived from it could restore what another device has since withdrawn. An account
            // with no methods has nothing to copy, and its projects stay "missing" for ever: it
            // only decides whether to ask.
            if let Some(ak) = account_key {
                if account.payment_methods.is_some() && copy_missing(&read_from_ls(), &server) {
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

            let own_private_key = account_key.and_then(|ak| my_private_key(&account, &ak, ls_ctx.peek().raw_wraps_ok(account.id)));
            update_ls(ls_ctx, |state| {
                apply_pull(state, &server, account_key.as_ref(), account.id, own_private_key);
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
            crate::common::creator_proof::claim_created_projects(ls_ctx).await;

            let mut account_answer = account_answer;
            account_answer.set(Some(server));
        });
    });

    let cached_list: Option<Vec<ProjectDto>> = ls_ctx.read().cached_projects_list.clone();

    let resource_version = use_context::<Signal<u64>>();

    // Every membership, archived or not: the toggle only filters what renders. Skipping the
    // archived ones meant a project un-archived on another device stayed archived here.
    // A memo, not `ls_ctx()` in the resource: every `update_ls` would re-send the batch, and the
    // login sync writes once per converted project - enough to trip nginx's burst limit (429).
    let project_ids = use_memo(move || ls_ctx.read().projects.iter().map(|p| p.project_id).collect::<Vec<Uuid>>());

    // Every group's ledger: this device's cache at once, then what the resource's sync answers.
    // Empty on the server render, so the first client render matches it.
    let mut ledgers: Signal<Vec<GroupLedger>> = use_signal(Vec::new);
    let mut synced = use_signal(|| false);
    use_effect(move || ledgers.set(cached_ledgers(&read_from_ls(), Uuid::nil())));

    let mut projects = use_resource(move || {
        async move {
            let _v = resource_version();
            let ids = project_ids();
            if ids.is_empty() {
                return Ok(vec![]);
            }
            // One request for the list and every group's balances: one per project tripped the
            // proxy's burst limit on accounts with a few groups.
            let mut result = sync_all(ids.clone()).await;
            // D17: a rotated project switches through a box or the escrow, then syncs under the new
            // key; one nothing reaches keeps a row (`lost_projects`) that opens artboard T10.
            let rotated = result.as_ref().map(|(_, syncs)| rotated_ids(syncs)).unwrap_or_default();
            let (account, account_key) = (auth_ctx.peek().clone(), *account_key_ctx.peek());
            if !rotated.is_empty() && adopt_all(ls_ctx, &rotated, account, account_key).await {
                result = sync_all(ids).await;
            }
            synced.set(true);
            let (before, syncs) = result?;
            ledgers.set(synced_ledgers(&before, &syncs, Uuid::nil()));
            verdicts.set(Some(rows_verdicts(&before, &syncs)));
            let list: Vec<ProjectDto> = syncs.into_iter().filter_map(|s| s.project).collect();
            // read_from_ls() and no `ls_ctx.set()` - deliberately *not* `update_ls`: this
            // closure reads `project_ids`, derived from `ls_ctx`, so refreshing the signal here
            // could re-run the resource. Safe as a disk-only write only because the cached list is read
            // back through `read_from_ls` (`has_archived` below) or the signal's own seed, and
            // because every other writer now bases on disk too.
            let mut state = read_from_ls();
            set_cached_projects_list(&mut state, list.clone());
            write_to_ls(&state);
            Ok(list)
        }
    });

    // The leak docs/plans/participant-links-encryption.md closes ties these expenses to this
    // account's email, so its devices convert every project they hold, opened or not - decided from
    // what the list and the account already answered (`conversion_plan`), so a project with nothing
    // left costs no request. The rest one at a time, paced: each is two to five requests, and back
    // to back on a fast link they outrun nginx's 10 a second within a dozen projects.
    let account_id = use_memo(move || auth_ctx.read().as_ref().map(|a| a.id));
    let mut converting = use_signal(|| false);
    let mut attempted: Signal<HashSet<Uuid>> = use_signal(HashSet::new);
    use_effect(move || {
        let Some(account) = account_id() else { return };
        if converting() {
            return;
        }
        let plan = {
            let (verdicts, answer) = (verdicts.read(), account_answer.read());
            let (Some(verdicts), Some(answer)) = (verdicts.as_ref(), answer.as_ref()) else { return };
            conversion_plan(&read_from_ls(), verdicts, answer, &attempted.peek())
        };
        if !plan.mark.is_empty() {
            update_ls(ls_ctx, |state| {
                for project_id in &plan.mark {
                    mark_converted(state, *project_id);
                }
            });
        }
        if plan.convert.is_empty() {
            return;
        }
        attempted.write().extend(plan.convert.iter().map(|(id, _)| *id));
        converting.set(true);
        spawn(async move {
            for (i, (project_id, key)) in plan.convert.into_iter().enumerate() {
                if i > 0 {
                    sleep(CONVERSION_PACE_MS).await;
                }
                if *account_id.peek() != Some(account) {
                    break;
                }
                let held = read_from_ls().projects.iter().any(|p| p.project_id == project_id && key_of(p) == Some(key));
                if held && crate::conversion::convert(key, project_id).await.is_ok() {
                    update_ls(ls_ctx, |state| mark_converted(state, project_id));
                }
            }
            converting.set(false);
        });
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

            crate::auth::recovery_ui::PendingResetBanner {}
            crate::auth::keypair_banner::KeypairMismatchBanner {}
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
                // A refused refresh is no reason to drop a list already saved: nginx's 429 means this
                // client asked too fast, and `sync_all` has already retried once.
                Some(Err(e)) if is_offline_error(e) || is_rate_limited(e) => {
                    let rate_limited = is_rate_limited(e);
                    // Disk as well as the signal: the resource saves the list to disk only, so after
                    // a first sync in this session the signal can still hold none.
                    match &cached_list.clone().or_else(|| read_from_ls().cached_projects_list) {
                        Some(list) => {
                            let filtered: Vec<ProjectDto> = list
                                .iter()
                                .filter(|p| show_archived() || status_of(&ls_ctx.peek(), p) != ProjectStatus::Archived)
                                .cloned()
                                .collect();
                            let mine = filtered.iter().find(|p| is_personal_dto(p, &ls_ctx.peek())).cloned();
                            let filtered: Vec<ProjectDto> =
                                filtered.into_iter().filter(|p| Some(p.id) != mine.as_ref().map(|m| m.id)).collect();
                            let lost = lost_projects(&ls_ctx.read(), &list.iter().map(|p| p.id).collect::<Vec<_>>());
                            rsx! {
                                div { id: "projects-cached-banner", role: "alert",
                                    class: "alert alert-warning alert-soft text-sm",
                                    if rate_limited { {tid!("error-rate-limited")} } else { {tid!("projects-offline-banner")} }
                                }
                                if let Some(project) = mine {
                                    PersonalCard { project, ledgers }
                                }
                                if filtered.is_empty() && lost.is_empty() {
                                    div { class: "flex flex-col items-center gap-2 py-12 text-base-content/70",
                                        Mascot { pose: MascotPose::Empty }
                                        span { class: "font-semibold", {tid!("projects-empty")} }
                                    }
                                } else if !filtered.is_empty() {
                                    div { id: "projects-list", class: "shrink-0 rounded-[20px] bg-base-100 shadow-soft flex flex-col",
                                        for (i, project) in filtered.into_iter().enumerate() {
                                            if i > 0 {
                                                div { class: "h-px bg-base-200 ml-[70px]" }
                                            }
                                            ProjectCard { glance: glance_of(project.id), synced: synced(), project: project.clone(), on_change: move |_| projects.restart() }
                                        }
                                    }
                                }
                                LostProjects { lost }
                            }
                        }
                        None if rate_limited => rsx! {
                            div { role: "alert", class: "alert alert-error", "{error_message(e)}" }
                        },
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
                    let lost = lost_projects(&ls_ctx.read(), &list.iter().map(|p| p.id).collect::<Vec<_>>());
                    rsx! {
                        if let Some(project) = pinned {
                            PersonalCard { project, ledgers }
                        }
                        if filtered.is_empty() && lost.is_empty() {
                            div { class: "flex flex-col items-center gap-2 py-12 text-base-content/70",
                                Mascot { pose: MascotPose::Empty }
                                span { class: "font-semibold", {tid!("projects-empty")} }
                            }
                            GroupCreateCard { on_create: move |_| show_add_project.set(true) }
                        } else if !filtered.is_empty() {
                            div { id: "projects-list", class: "shrink-0 rounded-[20px] bg-base-100 shadow-soft flex flex-col",
                                for (i, project) in filtered.into_iter().enumerate() {
                                    if i > 0 {
                                        div { class: "h-px bg-base-200 ml-[70px]" }
                                    }
                                    ProjectCard { glance: glance_of(project.id), synced: synced(), project: project.clone(), on_change: move |_| projects.restart() }
                                }
                            }
                        }
                        LostProjects { lost }
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

/// Projects whose key changed with nothing to switch to (D17). No name is left to show; the row
/// opens the page that says what happened and offers a new link or removal (artboard T10).
#[component]
fn LostProjects(lost: Vec<Uuid>) -> Element {
    let nav = use_navigator();
    if lost.is_empty() {
        return rsx! {};
    }
    rsx! {
        div { id: "projects-lost", class: "shrink-0 rounded-[20px] bg-base-100 shadow-soft flex flex-col",
            for (i, project_id) in lost.into_iter().enumerate() {
                if i > 0 {
                    div { class: "h-px bg-base-200 ml-[70px]" }
                }
                button {
                    id: "project-lost-{project_id}",
                    r#type: "button",
                    class: "flex items-center gap-3 px-4 py-3 text-start first:rounded-t-[20px] last:rounded-b-[20px] active:bg-base-200",
                    onclick: move |_| { nav.push(Route::ExpensesPage { project_id }); },
                    span { class: "size-[42px] shrink-0 rounded-[13px] bg-tile-amber text-tile-amber-content flex items-center justify-center",
                        KeyIcon {}
                    }
                    span { class: "flex-1 min-w-0 flex flex-col gap-0.5",
                        span { class: "font-semibold text-[15px] truncate text-base-content/70", {tid!("project-rotated-title")} }
                        span { class: "text-xs text-base-content/70 line-clamp-2", {tid!("project-rotated-row-hint")} }
                    }
                    span { class: "shrink-0 text-base-content/50", ChevronRightIcon {} }
                }
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

    // `ls_ctx.read()`, not `read_from_ls()`: the signal is authoritative now that every writer goes
    // through `update_ls`, and reading it subscribes - so a key arriving (escrow unwrapped by the
    // account sync, or the unlock modal) re-renders this card instead of leaving it locked until a
    // reload. `.read()`, not `ls_ctx()`, which would clone every cached row per card per render.
    let stored_key: Option<[u8; 32]> =
        ls_ctx.read().projects.iter().find(|p| p.project_id == project_id).and_then(key_of);

    let mut key_ctx = use_context_provider(|| ProjectKey(Signal::new(stored_key))).0;
    // The provider's initializer runs once, so without this the context stays `None` after an
    // unlock and `EditProjectModal` would refuse to save on a card that is visibly decrypted.
    use_effect(move || {
        if *key_ctx.peek() != stored_key {
            key_ctx.set(stored_key);
        }
    });

    // Long press anywhere opens the same sheet as the `...` trigger, so the whole card is the
    // target rather than a 32px button.
    let mut sheet_open = use_signal(|| false);
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
    let display_emoji = decrypted_project.as_ref().and_then(|p| p.emoji());
    let display_currency =
        decrypted_project.as_ref().map(|p| p.currency.clone()).unwrap_or_default();
    let today = crate::common::local_today();
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
        crate::common::project_token::scope(project_id);
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
            pending_ops.write().push_back(QueuedOp::new(
                Uuid::new_v4(),
                &queue_label,
                OpKind::EditProject(ProjectEdit {
                    project_id,
                    name_description: None,
                    status: Some(status.clone()),
                    history: None,
                    emoji: None,
                }),
            ));
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
            class: "relative flex items-center first:rounded-t-[20px] last:rounded-b-[20px] has-[>[role=button]:active]:bg-base-200",
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
                    sheet_open.set(true);
                });
                press_task.set(Some(task));
            },
            ontouchcancel: move |_| cancel_press(),
            onpointercancel: move |_| cancel_press(),
            ontouchend: move |e| {
                cancel_press();
                // preventDefault on touchend suppresses the synthesised click; without it the
                // click hits the sheet's backdrop and closes it instantly. Only touchend does
                // this - pointerup would not.
                if long_pressed() {
                    e.prevent_default();
                }
            },
            div {
            class: if locked { "flex-1 min-w-0 flex items-center gap-3 ps-4 pe-3 py-3 cursor-pointer" } else { "flex-1 min-w-0 flex items-center gap-3 px-4 py-3 cursor-pointer" },
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
                ProjectTile { name: display_name.clone(), id: project_id, emoji: display_emoji.clone() }
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
            button {
                r#type: "button",
                class: if locked { "btn btn-ghost btn-circle btn-sm me-4" } else { "sr-only focus:not-sr-only focus:me-4" },
                aria_label: if locked { tid!("project-actions") } else { format!("{}, {display_name}", tid!("project-actions")) },
                aria_haspopup: "dialog",
                aria_expanded: sheet_open(),
                onclick: move |_| {
                    haptic(Haptic::Light);
                    sheet_open.set(true);
                },
                "..."
            }
        }
        if sheet_open() {
            ProjectRowSheet {
                title: if locked { tid!("missing-encryption-key-title") } else { display_name.clone() },
                locked,
                status: project_status.clone(),
                on_close: move |_| sheet_open.set(false),
                on_edit: move |_| show_edit_modal.set(true),
                on_unlock: move |_| show_unlock_modal.set(true),
                on_status: apply_status,
                on_leave: move |_| show_leave_confirm.set(true),
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
