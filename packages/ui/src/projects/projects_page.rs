use api::account_projects::account_projects_controller::{
    batch_upsert_account_projects, get_account_projects,
};
use api::auth::auth_controller::me;
use api::projects::projects_controller::{get_projects_by_ids, update_project_by_id};
use api::users::users_controller::get_users_by_project_id;
use dioxus::core::Task;
use dioxus::fullstack::Json;
use dioxus::prelude::*;
use crate::tid;
use shared::{Account, BatchProject, EditableProject, ProjectDto, ProjectPayload, ProjectStatus};
use uuid::Uuid;

use std::collections::{HashSet, VecDeque};

use crate::common::{
    apply_pull, clear_user_id, copy_missing, error_message, haptic, is_offline_error, key_of,
    leave_project_and_forget, left_elsewhere, mark_synced, pending,
    read_from_ls, remove_project, set_cached_projects_list, sleep, to_push, update_ls, write_queue,
    write_to_ls,
    AvatarGroup, ConfirmModal, DropdownButton, DropdownItem, EmptyMagnifyingGlassIllustration,
    Flash, Haptic, LocalStorageState, OpKind, ProjectKey, ProjectStatusItems, PullToRefresh,
    QueuedOp, SizeClass, SpeedDialAction, SpeedDialFab, LEAVE_CONFIRM_MESSAGE, LEAVE_CONFIRM_TITLE,
};
use crate::crypto::decrypt_json;
use crate::expenses::EditProjectModal;
use crate::icons::{ImportIcon, LinkIcon, LockIcon, PlusIcon, ICON_INLINE};
use crate::payment_methods::refill_project_copies;
use crate::projects::{AddProjectModal, ImportTricountModal, JoinProjectModal};
use crate::friends::{InvitationsCard, NotificationsBell};
use crate::route::Route;

/// `status` is plaintext in the cache, so archived ids are known without the server; with no cache
/// everything is fetched. While hidden, un-archiving elsewhere goes unnoticed until toggled on.
fn ids_to_fetch(state: &LocalStorageState, want_archived: bool) -> Vec<Uuid> {
    if want_archived {
        return state.projects.iter().map(|p| p.project_id).collect();
    }
    let archived: HashSet<Uuid> = state
        .cached_projects_list
        .iter()
        .flatten()
        .filter(|p| p.status == ProjectStatus::Archived)
        .map(|p| p.id)
        .collect();
    state
        .projects
        .iter()
        .map(|p| p.project_id)
        .filter(|id| !archived.contains(id))
        .collect()
}

/// Whether anything has been archived. The filter is noise until something has been, so the page
/// renders no control at all while this is false — which is most devices, most of the time.
///
/// `Closed` does not count: it is a distinct status that stays in the list, and only `Archived`
/// hides a row.
fn has_archived(state: &LocalStorageState) -> bool {
    state.cached_projects_list.iter().flatten().any(|p| p.status == ProjectStatus::Archived)
}

/// Archived rows were never requested, so replacing outright erases the only record that they are
/// archived. Carried over while still a member; anything else missing was genuinely deleted.
fn merge_cached_list(
    previous: Option<&Vec<ProjectDto>>,
    fetched: Vec<ProjectDto>,
    member_ids: &[Uuid],
    want_archived: bool,
) -> Vec<ProjectDto> {
    if want_archived {
        return fetched;
    }
    let returned: HashSet<Uuid> = fetched.iter().map(|p| p.id).collect();
    let carried = previous.into_iter().flatten().filter(|p| {
        p.status == ProjectStatus::Archived
            && !returned.contains(&p.id)
            && member_ids.contains(&p.id)
    });
    fetched.iter().cloned().chain(carried.cloned()).collect()
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

    // Reconciliation, both directions. Reads `account_key_ctx` as well as `auth_ctx` so it re-runs
    // when either resolves — on a fresh login both are set before this page renders, but a session
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
            // created while logged out never reaches `account_projects`, so no other device — and
            // no escrow — ever learns about it.
            let push = to_push(&read_from_ls(), &server, account_key.as_ref(), Some(&account));
            let mut rejected: Vec<Uuid> = vec![];
            let mut accepted: Vec<Uuid> = vec![];
            if !push.is_empty() {
                if let Ok(result) = batch_upsert_account_projects(Json(push)).await {
                    rejected = result.identity_rejected;
                    accepted = result.accepted;
                }
            }

            // A project claimed here that the account's shared payment methods never reached —
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
                // refused again on every load — the server has no second way to say no.
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
        });
    });

    let cached_list: Option<Vec<ProjectDto>> = ls_ctx().cached_projects_list.clone();

    let resource_version = use_context::<Signal<u64>>();

    let mut projects = use_resource(move || {
        let ls_ctx = ls_ctx;
        async move {
            let _v = resource_version();
            // Read, so flipping the toggle re-runs this and pulls the archived rows in.
            let want_archived = show_archived();
            let state = ls_ctx();
            let member_ids: Vec<Uuid> = state.projects.iter().map(|p| p.project_id).collect();
            let ids = ids_to_fetch(&state, want_archived);
            if ids.is_empty() {
                return Ok(vec![]);
            }
            let result = get_projects_by_ids(Json(BatchProject { ids })).await;
            if let Ok(ref list) = result {
                // read_from_ls() and no `ls_ctx.set()` — deliberately *not* `update_ls`: this
                // closure reads `ls_ctx()` above, so refreshing the signal here would re-run the
                // resource forever. Safe as a disk-only write only because the cached list is read
                // back through `read_from_ls` (`has_archived` below) or the signal's own seed, and
                // because every other writer now bases on disk too.
                let mut state = read_from_ls();
                let merged = merge_cached_list(
                    state.cached_projects_list.as_ref(),
                    list.clone(),
                    &member_ids,
                    want_archived,
                );
                set_cached_projects_list(&mut state, merged);
                write_to_ls(&state);
            }
            result
        }
    });

    let project_count = move || match &*projects.read() {
        Some(Ok(list)) => list.iter().filter(|item| item.status != ProjectStatus::Archived).count(),
        _ => 0,
    };

    // read_from_ls(), not `cached_list`: archiving while online goes through `update_project_by_id`
    // and the resource writes the merged cache with `write_to_ls` alone — `ls_ctx` is never set on
    // that path, so it stays stale for the rest of the session and the filter would not appear
    // until a reload. The resource result is read below, so this re-runs on the render that
    // completed the write.
    let archived_exist = has_archived(&read_from_ls());

    let choose_archived = move |value: bool| {
        // Not `update_ls`: the resource already re-runs off `show_archived`, and refreshing the
        // signal here would restart it twice. Safe as a disk-only write because `show_archived` is
        // read back with `read_from_ls`, never off the signal. Copied locally so the closure stays
        // Fn + Copy and both tabs can hold it.
        let mut show_archived = show_archived;
        show_archived.set(value);
        let mut state = read_from_ls();
        state.show_archived = value;
        write_to_ls(&state);
    };

    rsx! {
        div { class: "container app-container bg-base-100 overflow-auto p-4 pb-24 max-w-md w-full mx-auto flex flex-col gap-4",
            PullToRefresh { on_refresh: move |_| projects.restart(), busy: pending(&projects) }
            div { class: "navbar px-0",
                div { class: "navbar-start" }
                div { class: "navbar-center flex flex-col",
                    h1 { class: "text-2xl font-extrabold font-display text-gradient-brand", "Counted" }
                }
                div { class: "navbar-end", NotificationsBell {} }
            }

            div {
                class: "stats shadow-soft min-h-25",
                style: "grid-template-columns: 1fr;",
                div { class: "stat",
                    div { class: "stat-title", {tid!("projects-count-label")} }
                    div { class: "stat-value", "{project_count()}" }
                }
            }

            InvitationsCard { on_accepted: move |_| projects.restart() }

            // `show_archived` is an *include* flag, so the second tab is "All", not "Archived" —
            // labelling it the latter would promise a list that never renders.
            if archived_exist {
                div { role: "tablist", class: "tabs tabs-box shadow-soft",
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
                                .filter(|p| show_archived() || p.status != ProjectStatus::Archived)
                                .cloned()
                                .collect();
                            rsx! {
                                div { role: "alert",
                                    class: "alert alert-warning alert-soft text-sm",
                                    {tid!("projects-offline-banner")}
                                }
                                if filtered.is_empty() {
                                    div { class: "flex flex-col items-center gap-2 py-12 text-base-content/70",
                                        EmptyMagnifyingGlassIllustration {}
                                        span { class: "font-semibold", {tid!("projects-empty")} }
                                    }
                                } else {
                                    div { class: "flex flex-col gap-3",
                                        for project in filtered {
                                            ProjectCard { project: project.clone(), on_change: move |_| projects.restart() }
                                        }
                                    }
                                }
                            }
                        }
                        None => rsx! {
                            div { class: "flex flex-col items-center gap-4 py-12 text-base-content/70",
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
                    let filtered: Vec<ProjectDto> = list
                        .iter()
                        .filter(|p| show_archived() || p.status != ProjectStatus::Archived)
                        .cloned()
                        .collect();
                    if filtered.is_empty() {
                        rsx! {
                            div { class: "flex flex-col items-center gap-2 py-12 text-base-content/70",
                                EmptyMagnifyingGlassIllustration {}
                                span { class: "font-semibold", {tid!("projects-empty")} }
                                span { class: "text-sm text-center", {tid!("projects-empty-hint")} }
                            }
                        }
                    } else {
                        rsx! {
                            div { class: "flex flex-col gap-3",
                                for project in filtered {
                                    ProjectCard { project: project.clone(), on_change: move |_| projects.restart() }
                                }
                            }
                        }
                    }
                }
            }

            // FAB — fixed above the bottom dock, expands into import/join/create
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

#[derive(PartialEq, Props, Clone)]
struct ProjectCardProps {
    project: ProjectDto,
    on_change: EventHandler<()>,
}

/// How long a finger has to stay down before the card's actions menu opens.
const LONG_PRESS_MS: u32 = 500;

#[component]
fn ProjectCard(props: ProjectCardProps) -> Element {
    let nav = use_navigator();
    let project = props.project.clone();
    let project_id = project.id;
    let project_status = project.status.clone();
    let mut show_edit_modal = use_signal(|| false);

    let is_online = use_context::<Signal<bool>>();
    let mut pending_ops = use_context::<Signal<VecDeque<QueuedOp>>>();
    let ls_ctx = use_context::<Signal<LocalStorageState>>();
    let mut flash = use_context::<Signal<Option<Flash>>>();
    let mut show_leave_confirm = use_signal(|| false);
    let mut show_unlock_modal = use_signal(|| false);

    // `ls_ctx()`, not `read_from_ls()`: the signal is authoritative now that every writer goes
    // through `update_ls`, and reading it subscribes — so a key arriving (escrow unwrapped by the
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
    // case, which is what used to render as a blank card — so the card renders a locked state
    // instead, and offers the one action that fixes it.
    let locked = decrypted_project.is_none();
    let display_name = decrypted_project.as_ref().map(|p| p.name.clone()).unwrap_or_default();
    let display_desc = decrypted_project.as_ref().and_then(|p| p.description.clone());
    let display_currency =
        decrypted_project.as_ref().map(|p| p.currency.clone()).unwrap_or_default();

    // Not fetched while locked: the names come back as ciphertext this device cannot open, so the
    // stack rendered one differently-coloured `?` per member — four identities the app cannot name.
    // The hook cannot live behind an `if` (hook order), so the closure returns early instead.
    //
    // `use_reactive!` on `locked` rather than reading `ls_ctx()` inside: the closure needs to re-run
    // when *this* project is unlocked, and subscribing to the whole store would refetch every card's
    // participants on any project's cache write — the regression `request-count.spec.ts` guards.
    let users = use_resource(use_reactive!(|locked| async move {
        if locked {
            return Ok(vec![]);
        }
        get_users_by_project_id(project_id).await
    }));

    // Offline this queues for replay and updates the cached list optimistically, so the card
    // reflects the new status until the queue drains.
    let queue_label = display_name.clone();
    let apply_status = move |status: ProjectStatus| {
        let editable =
            EditableProject { id: project_id, payload: None, status: Some(status.clone()), history: None };
        let on_change = props.on_change;
        if !is_online() {
            pending_ops.write().push_back(QueuedOp {
                id: Uuid::new_v4(),
                label: queue_label.clone(),
                op: OpKind::UpdateProject(editable),
            });
            write_queue(&pending_ops.read());
            update_ls(ls_ctx, |state| {
                if let Some(ref mut list) = state.cached_projects_list {
                    if let Some(p) = list.iter_mut().find(|p| p.id == project_id) {
                        p.status = status;
                    }
                }
            });
            on_change.call(());
            return;
        }
        spawn(async move {
            let _ = update_project_by_id(Json(editable)).await;
            on_change.call(());
        });
    };

    rsx! {
        div {
            id: "project-card-{project_id}",
            class: "card card-clickable shadow-soft",
            role: "button",
            tabindex: "0",
            // No `ontouchmove`: on mobile every dispatched event costs a *synchronous* XHR to
            // wry's custom protocol, so a ~60Hz listener blocks the main thread through every
            // scroll. The two cancel events below fire once per gesture, when the UA claims it for
            // panning — so scrolling off a card never opens the menu.
            ontouchstart: move |_| {
                long_pressed.set(false);
                let task = spawn(async move {
                    sleep(LONG_PRESS_MS).await;
                    // Inside the task, not on touchstart: the press has to actually complete. This
                    // is the only signal that the hold worked — the menu appears in the same frame.
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
                // touchend does this — pointerup would not.
                if long_pressed() {
                    e.prevent_default();
                }
            },
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
            div { class: "card-body p-4 gap-2",
                div { class: "flex items-start justify-between gap-2",
                    div { class: "flex flex-col gap-1 min-w-0",
                        if locked {
                            h2 { class: "card-title text-base truncate text-base-content/70 italic",
                                {tid!("missing-encryption-key-title")}
                            }
                            p { class: "text-xs text-base-content/70",
                                {tid!("project-locked-hint")}
                            }
                        } else {
                            h2 { class: "card-title text-base truncate", "{display_name}" }
                            if let Some(desc) = &display_desc {
                                if !desc.is_empty() {
                                    p { class: "text-xs text-base-content/70 truncate",
                                        "{desc}"
                                    }
                                }
                            }
                        }
                    }
                    div { class: "flex items-center gap-1 shrink-0",
                        if locked {
                            // A badge, not the ghost lock below: that one means "encrypted" and is
                            // on every card. This one means "you cannot open this", and has to read
                            // as a different thing.
                            span { class: "badge badge-warning badge-soft badge-sm",
                                LockIcon { size: ICON_INLINE }
                            }
                        } else {
                            span { class: "text-xs font-mono text-base-content/70", "{display_currency}" }
                            span { class: "text-base-content/20", LockIcon { size: ICON_INLINE } }
                        }
                        DropdownButton {
                            open: menu_open,
                            label: tid!("project-actions"),
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
                }
                div { class: "flex items-center justify-between",
                    StatusBadge { status: project.status.clone() }
                    match &*users.read() {
                        Some(Ok(user_list)) if !user_list.is_empty() => {
                            rsx! {
                                AvatarGroup {
                                    users: user_list.clone(),
                                    encryption_key: stored_key,
                                    size: SizeClass::W8,
                                }
                            }
                        }
                        _ => rsx! {},
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
            {
                let user_list = users
                    .read()
                    .as_ref()
                    .and_then(|r| r.as_ref().ok())
                    .cloned()
                    .unwrap_or_default();
                rsx! {
                    EditProjectModal {
                        project: project.clone(),
                        users: user_list,
                        on_close: move |_| show_edit_modal.set(false),
                        on_saved: move |_| {
                            show_edit_modal.set(false);
                            props.on_change.call(());
                        },
                    }
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
                            // Membership survives on the server, so keep the local entry —
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

#[derive(PartialEq, Props, Clone)]
struct StatusBadgeProps {
    status: ProjectStatus,
}

#[component]
fn StatusBadge(props: StatusBadgeProps) -> Element {
    let (dot_class, label) = match props.status {
        ProjectStatus::Ongoing => ("status-success", tid!("status-ongoing")),
        ProjectStatus::Closed => ("status-warning", tid!("status-closed")),
        ProjectStatus::Archived => ("status-neutral", tid!("status-archived")),
    };
    rsx! {
        div { class: "flex gap-1 items-center text-xs",
            div { class: "status {dot_class}" }
            span { "{label}" }
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

    #[test]
    fn without_a_cache_everything_is_fetched() {
        let s = state(&[1, 2, 3], None);
        assert_eq!(ids_to_fetch(&s, false).len(), 3);
    }

    #[test]
    fn archived_projects_are_left_out_of_the_batch() {
        let s = state(
            &[1, 2],
            Some(vec![project(1, ProjectStatus::Ongoing), project(2, ProjectStatus::Archived)]),
        );
        assert_eq!(ids_to_fetch(&s, false), vec![id(1)]);
    }

    #[test]
    fn showing_archived_fetches_them_again() {
        let s = state(
            &[1, 2],
            Some(vec![project(1, ProjectStatus::Ongoing), project(2, ProjectStatus::Archived)]),
        );
        assert_eq!(ids_to_fetch(&s, true), vec![id(1), id(2)]);
    }

    #[test]
    fn a_closed_project_is_not_archived_and_is_still_fetched() {
        let s = state(&[1], Some(vec![project(1, ProjectStatus::Closed)]));
        assert_eq!(ids_to_fetch(&s, false), vec![id(1)]);
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

    #[test]
    fn archived_rows_survive_a_response_that_omits_them() {
        let previous = vec![project(1, ProjectStatus::Ongoing), project(2, ProjectStatus::Archived)];
        let merged = merge_cached_list(
            Some(&previous),
            vec![project(1, ProjectStatus::Ongoing)],
            &[id(1), id(2)],
            false,
        );
        let archived: Vec<Uuid> = merged
            .iter()
            .filter(|p| p.status == ProjectStatus::Archived)
            .map(|p| p.id)
            .collect();
        assert_eq!(archived, vec![id(2)], "the archived row must be carried over");
        assert_eq!(merged.len(), 2);
    }

    #[test]
    fn an_archived_project_the_device_left_is_forgotten() {
        let previous = vec![project(2, ProjectStatus::Archived)];
        let merged = merge_cached_list(Some(&previous), vec![], &[], false);
        assert!(merged.is_empty(), "leaving a project must still forget it");
    }

    #[test]
    fn a_missing_non_archived_project_is_dropped() {
        // Requested and not returned: deleted server-side.
        let previous = vec![project(1, ProjectStatus::Ongoing)];
        let merged = merge_cached_list(Some(&previous), vec![], &[id(1)], false);
        assert!(merged.is_empty());
    }

    #[test]
    fn showing_archived_replaces_the_cache_wholesale() {
        let previous = vec![project(9, ProjectStatus::Archived)];
        let merged =
            merge_cached_list(Some(&previous), vec![project(1, ProjectStatus::Ongoing)], &[id(9)], true);
        assert_eq!(merged.len(), 1);
        assert_eq!(merged[0].id, id(1));
    }
}
