use dioxus::prelude::*;
use crate::tid;
use shared::{ExpenseType, ProjectDto, ProjectStatus, ReimbursementSuggestion, User};
use std::collections::VecDeque;
use uuid::Uuid;

use super::helpers::export;
use crate::common::persist::is_demo;
use crate::common::{
    copy_text, is_mobile, key_offer_shown, leave_project_and_forget, pending, read_from_ls, InviteRetry,
    remove_project, scheme_link_for, share_link_for, share_text, update_ls, update_project_fresh, write_queue, write_to_ls, AppHeader, AvatarGroup, ConfirmModal, Flash,
    LocalStorageState, ProjectKey, PullToRefresh, QueuedOp, SizeClass, LEAVE_CONFIRM_MESSAGE,
    LEAVE_CONFIRM_TITLE,
};
use crate::common::offline_queue::drop_project_ops;
use crate::expenses::rotation_sheets::ProjectRotation;
use crate::expenses::helpers::my_spending::load_ledgers;
use crate::expenses::helpers::guest_invites::{dismiss_guests_hint, guests_hint_dismissed, unclaimed_guests};
use crate::expenses::guests_card::{GuestsCard, SendToGuestsSheet};
use crate::expenses::my_spending::{CountedGroup, CountedGroupsSheet};
use crate::preferences::{push_counted, uncounted_projects};
use crate::projects::personal_cards::is_personal_dto;
use std::sync::Arc;
use crate::expenses::helpers::expenses_page_helpers::{
    header_state, sync_error, transfer_preset, went_rotated, DownloadFn,
};
use crate::expenses::hooks::use_project_store::ProjectStore;
use crate::expenses::group_security::{recreate_prefill, security_access, AdminAccess, GroupSecurity};
use crate::expenses::project_header::ProjectHeader;
use crate::friends::friends_service::{invitee_emails, Invitee};
use crate::friends::InviteFriendsModal;
use crate::icons::{CloseIcon, ICON_INLINE};
use crate::projects::{AddProjectModal, GroupPrefill, JoinProjectModal};
use crate::decrypted::{pickable_users, status_edit};
use crate::expenses::project_states::{
    ExpensesSkeleton, MissingKeyScreen, NoLocalDataScreen, Spinner,
};
use crate::expenses::project_stats::ProjectStats;
use crate::expenses::tabs::project_tabs::{tab_id, ProjectTabs, Tab, TABPANEL_ID};
use crate::expenses::{
    AddExpenseModal, BalanceTab, EditProjectModal, ExpensesTab, QueuedAdd, ReimbursementsTab,
    UserSelectionModal,
};
use crate::route::Route;

/// Set by "Payer" on the projects page: the project whose page opens on its Remboursements tab.
pub static OPEN_REIMBURSEMENTS: GlobalSignal<Option<Uuid>> = Signal::global(|| None);

/// A key change being confirmed, with the key and roster it started from: both outlive the switch,
/// which replaces the page's key and reloads its rows while the new-link sheet is still open.
#[derive(Clone, PartialEq)]
struct RotationTarget {
    k1: [u8; 32],
    users: Vec<User>,
    removee: Option<i32>,
}

#[component]
pub fn ExpensesPage(project_id: Uuid) -> Element {
    let nav = use_navigator();
    let mut ls_ctx = use_context::<Signal<LocalStorageState>>();
    let mut flash = use_context::<Signal<Option<Flash>>>();
    let auth_ctx = use_context::<Signal<Option<shared::Account>>>();
    let mut stored_tab = use_context::<ProjectStore>().tab;
    let active_tab = move || stored_tab().filter(|(id, _)| *id == project_id).map_or(Tab::Expenses, |(_, t)| t);
    let mut select_tab = move |t: Tab| stored_tab.set(Some((project_id, t)));
    use_effect(move || {
        if *OPEN_REIMBURSEMENTS.peek() == Some(project_id) {
            *OPEN_REIMBURSEMENTS.write() = None;
            select_tab(Tab::Reimbursements);
        }
    });
    let mut show_transfer_modal = use_signal(|| false);
    let mut show_edit_modal = use_signal(|| false);
    let mut recreate: Signal<Option<GroupPrefill>> = use_signal(|| None);
    let mut show_invite_modal = use_signal(|| false);
    let mut show_guests_sheet = use_signal(|| false);
    let mut show_switch = use_signal(|| false);
    let mut invite_retry = use_context::<Signal<Option<InviteRetry>>>();
    let mut retry_invitees: Signal<Vec<Invitee>> = use_signal(Vec::new);
    let mut show_leave_confirm = use_signal(|| false);
    let mut show_unlock_modal = use_signal(|| false);
    let mut rotation: Signal<Option<RotationTarget>> = use_signal(|| None);
    let mut pending_ops = use_context::<Signal<VecDeque<QueuedOp>>>();
    let mut transfer = use_signal(|| None::<(String, f64, i32, i32)>);

    // "Ouvrir dans l'application" targets a phone *browser* - not the native app, not desktop.
    // The user agent is unknown to SSR, so deciding on the first render would break hydration;
    // gated post-mount like onboarding in app_layout.rs.
    let mut mounted = use_signal(|| false);
    use_effect(move || mounted.set(true));
    let demo = mounted() && is_demo();
    let offer_native_app = mounted() && cfg!(target_arch = "wasm32") && is_mobile() && !demo;

    // Owned by AppLayout, so it survives navigating to an expense and back.
    let store = use_context::<ProjectStore>();
    // One participant and no one to settle with: no sharing, no balance tabs, no avatars.
    let personal = store.data_for(project_id).is_some_and(|d| d.personal);
    let (mut sync, live, data, key_missing, unusable) =
        (store.sync, store.live, store.data, store.key_missing, store.unusable);
    let on_expenses_changed = store.on_expenses_changed;
    let mut key_ctx = use_context::<ProjectKey>().0;
    let account_key_ctx = use_context::<Signal<Option<[u8; 32]>>>();
    let mut show_counted_groups = use_signal(|| false);

    // "Mes dépenses" counts my share of every other project this device holds.
    let personal_id = use_memo(move || {
        let d = store.data.read();
        d.personal.then_some(d.project_id).flatten()
    });
    let mut ledgers_res = use_resource(move || async move {
        match personal_id() {
            Some(id) => Some(Arc::new(load_ledgers(id).await)),
            None => None,
        }
    });
    let ledgers = use_memo(move || ledgers_res.read().clone().flatten().unwrap_or_default());
    // The page mounts on the previous project's data, so the first answer is `None` and a rerun
    // keeps it until the personal project's ledgers land.
    let ledgers_loading = !matches!(*ledgers_res.read(), Some(Some(_)));
    let uncounted = match (auth_ctx.read().as_ref(), account_key_ctx()) {
        (Some(a), Some(k)) => Some(uncounted_projects(a, &k)),
        _ => None,
    };
    // R4 only makes sense once there is a personal project to count in.
    let has_personal = {
        let state = ls_ctx.read();
        state.projects.iter().any(|p| p.cached_project.as_ref().is_some_and(|dto| is_personal_dto(dto, &state)))
    };
    let counted = uncounted.as_ref().filter(|_| !personal && has_personal).map(|u| !u.contains(&project_id));
    let set_counted = move |id: Uuid, on: bool| {
        if let Some(k) = account_key_ctx() {
            push_counted(auth_ctx, k, id, on, flash);
        }
    };

    let stored_user_id = move || {
        ls_ctx.read().projects.iter().find(|p| p.project_id == project_id).and_then(|p| p.user_id)
    };

    // One small DTO, not `ls_ctx()`, which would deep-clone every cached expense and payment per
    // render. Only the header's offline branch needs it, and only for the name.
    let cached_project = ls_ctx
        .read()
        .projects
        .iter()
        .find(|p| p.project_id == project_id)
        .and_then(|p| p.cached_project.clone());

    // The only exit from a dangling share link (the id is gone server-side, so every resource here
    // keeps failing until the local entry is dropped) and from a key that changed (T10), whose
    // queued ops go with it (D15).
    let forget_project = move |_| {
        let mut state = read_from_ls();
        remove_project(&mut state, project_id);
        write_to_ls(&state);
        ls_ctx.set(state);
        drop_project_ops(&mut pending_ops.write(), project_id);
        write_queue(&pending_ops.read());
        nav.push(Route::ProjectsPage {});
    };

    if key_ctx().is_none() {
        return rsx! {
            div { class: "container app-container bg-base-100 overflow-auto p-4 pb-36 max-w-md w-full mx-auto flex flex-col gap-4",
                PullToRefresh { on_refresh: move |_| sync.restart(), busy: pending(&sync) }
                AppHeader {
                    back_button_route: Route::ProjectsPage {},
                    title: "Counted",
                    brand: true,
                    sticky: true,
                }
                if key_missing() {
                    MissingKeyScreen { on_unlock: move |_| show_unlock_modal.set(true) }
                } else {
                    Spinner { padding_class: "py-8" }
                }
                if show_unlock_modal() {
                    JoinProjectModal {
                        expect_project_id: project_id,
                        on_close: move |_| show_unlock_modal.set(false),
                    }
                }
            }
        };
    }

    let key = key_ctx().unwrap();
    let key_opens = store.data_for(project_id).is_some_and(|d| d.loaded && !d.project_unreadable);

    // Exports `live`, not the resource, so an export taken right after an edit contains it.
    let export_all = move |download: DownloadFn| {
        let snapshot = live.read();
        let Some(l) = snapshot.as_ref().filter(|l| l.project_id == project_id) else { return };
        let Some(proj) = l.project.as_ref() else { return };
        download(&key, proj, &l.users, &l.expenses, &l.payments);
    };

    let security = {
        let snapshot = live.read();
        let stored_admin = ls_ctx.read().projects.iter().any(|p| p.project_id == project_id && p.admin);
        snapshot.as_ref().filter(|l| l.project_id == project_id && !demo && !personal).and_then(|l| {
            let project = l.project.as_ref()?;
            let access = security_access(project, &l.users, &key, auth_ctx.read().is_some(), stored_admin, stored_user_id())?;
            let prefill = recreate_prefill(&key, project, &l.users, stored_user_id());
            let users = l.users.clone();
            Some(GroupSecurity {
                access,
                on_rotate: Some(EventHandler::new(move |_| {
                    rotation.set(Some(RotationTarget { k1: key, users: users.clone(), removee: None }));
                })),
                on_remove: Some(EventHandler::new(move |_| show_edit_modal.set(true))),
                on_hide: EventHandler::new(move |_| show_edit_modal.set(true)),
                on_recreate: EventHandler::new(move |_| recreate.set(Some(prefill.clone()))),
                on_sign_in: EventHandler::new(move |_| { nav.push(Route::LoginPage {}); }),
                on_register: EventHandler::new(move |_| { nav.push(Route::RegisterPage {}); }),
            })
        })
    };

    let can_rotate = security.as_ref().is_some_and(|s| s.access == AdminAccess::Admin);

    // (message, server-unreachable) - the second drives the cache fallback.
    let body_err = sync_error(sync.read().as_ref().and_then(|r| r.as_ref()));
    let is_stale = body_err.as_ref().map(|(_, offline)| *offline).unwrap_or(false);
    let body_err_msg = body_err.as_ref().map(|(msg, _)| msg.clone());

    // `share_text` is false without a sheet (desktop browsers) or when it could not open, and true
    // on dismiss - so the clipboard is only the fallback, never a second prompt after a shown sheet.
    // Only a key proven on this project's row: a server can file another account-key blob where
    // the escrowed key belongs, and this would put it in a link.
    let share_link = (!demo && !personal && key_opens).then(|| EventHandler::new(move |_| {
        let url = share_link_for(project_id, &key);
        spawn(async move {
            if share_text(&url).await {
                return;
            }
            if copy_text(&url).await {
                flash.set(Some(Flash::ok(tid!("link-copied"))));
            } else {
                flash.set(Some(Flash::err(tid!("copy-link-failed"))));
            }
        });
    }));
    let invite = (auth_ctx().is_some() && !personal).then(|| EventHandler::new(move |_| show_invite_modal.set(true)));
    let dismiss_guests = move || update_ls(ls_ctx, |state| dismiss_guests_hint(state, project_id));

    rsx! {
        div { class: "container app-container bg-base-100 overflow-auto p-4 pb-36 max-w-md w-full mx-auto flex flex-col gap-4",
            // Each tab keeps its own scroll offset (page_transition.rs).
            "data-scroll-key": tab_id(active_tab()),
            PullToRefresh {
                on_refresh: move |_| {
                    sync.restart();
                    if personal {
                        ledgers_res.restart();
                    }
                },
                busy: pending(&sync),
            }

            ProjectHeader {
                state: header_state(sync.read().as_ref().and_then(|r| r.as_ref()), cached_project.as_ref(), &key, project_id),
                native_app_link: offer_native_app.then(|| scheme_link_for(project_id, &key)),
                on_history: move |_| { nav.push(Route::ProjectHistoryPage { project_id }); },
                on_recurring: move |_| { nav.push(Route::RecurringPage { project_id }); },
                on_edit: move |_| show_edit_modal.set(true),
                on_leave: move |_| show_leave_confirm.set(true),
                on_status: move |status: ProjectStatus| {
                    spawn(async move {
                        let edit = |stored: &ProjectDto| status_edit(&key, stored, &status);
                        let _ = update_project_fresh(project_id, edit).await;
                        sync.restart();
                    });
                },
                on_export_json: move |_| export_all(export::download_json),
                on_export_csv: move |_| export_all(export::download_csv),
                badge: personal.then(|| tid!("personal-badge")),
                on_share: share_link,
                on_invite: invite,
                on_forget: forget_project,
                on_open_link: EventHandler::new(move |_| show_unlock_modal.set(true)),
                counted,
                on_counted: counted.is_some().then(|| EventHandler::new(move |on: bool| set_counted(project_id, on))),
                on_counted_groups: personal.then(|| EventHandler::new(move |_| show_counted_groups.set(true))),
                security,
            }

            // The rotated screen's "Open a link": a new key that passes §8.4 replaces the held one.
            if show_unlock_modal() {
                JoinProjectModal {
                    expect_project_id: project_id,
                    on_close: move |_| {
                        show_unlock_modal.set(false);
                        let held = crate::common::local_storage::project_key(project_id);
                        if held.is_some() && *key_ctx.peek() != held {
                            key_ctx.set(held);
                            sync.restart();
                        }
                    },
                }
            }

            if let Some(prefill) = recreate() {
                AddProjectModal { on_close: move |_| recreate.set(None), prefill }
            }

            if let Some(target) = rotation() {
                ProjectRotation {
                    project_id,
                    k1: target.k1,
                    users: target.users,
                    removee: target.removee,
                    my_user_id: stored_user_id(),
                    on_close: move |_| rotation.set(None),
                    on_switched: move |k2: [u8; 32]| {
                        key_ctx.set(Some(k2));
                        sync.restart();
                    },
                }
            }

            if show_counted_groups() {
                CountedGroupsSheet {
                    groups: ledgers.read().iter().filter(|l| l.me.is_some()).map(CountedGroup::from).collect::<Vec<_>>(),
                    uncounted: uncounted.clone().unwrap_or_default(),
                    currency: store.data_for(project_id).map(|d| d.currency.clone()).unwrap_or_default(),
                    on_toggle: move |(id, on): (Uuid, bool)| set_counted(id, on),
                    on_close: move |_| show_counted_groups.set(false),
                }
            }

            // What the create or edit modal could not send. It closed anyway, so the offer to send
            // them again lives here.
            if let Some(retry) = invite_retry().filter(|r| r.project_id == project_id) {
                div { role: "alert", id: "invite-retry", class: "alert alert-warning alert-soft text-sm items-start",
                    div { class: "flex flex-col gap-2 flex-1",
                        if !retry.unsent.failed.is_empty() {
                            span { {tid!("invite-failed", emails: invitee_emails(&retry.unsent.failed))} }
                        }
                        if !retry.unsent.waiting.is_empty() {
                            span { id: "invite-waiting", {tid!("invite-waiting", emails: invitee_emails(&retry.unsent.waiting))} }
                        }
                        if !retry.unsent.failed.is_empty() {
                            button {
                                id: "invite-retry-again",
                                r#type: "button",
                                class: "btn btn-sm self-start",
                                onclick: move |_| {
                                    retry_invitees.set(retry.unsent.failed.clone());
                                    invite_retry.set(None);
                                    show_invite_modal.set(true);
                                },
                                {tid!("invite-again")}
                            }
                        }
                    }
                    button {
                        r#type: "button",
                        class: "btn btn-ghost btn-circle btn-sm",
                        aria_label: tid!("close"),
                        onclick: move |_| invite_retry.set(None),
                        CloseIcon { size: ICON_INLINE }
                    }
                }
            }

            if show_invite_modal() {
                InviteFriendsModal {
                    project_id,
                    project_key: key,
                    preselected: retry_invitees(),
                    on_close: move |_| {
                        show_invite_modal.set(false);
                        retry_invitees.set(Vec::new());
                    },
                }
            }

            // The store's decryption pass, but only once it is this project's - it still holds the
            // previous one until the new sync lands.
            match store.data_for(project_id).filter(|d| d.loaded) {
                Some(d) if d.project_unreadable => rsx! {
                    MissingKeyScreen {}
                },
                Some(d) => {
                    let unreadable = d.unreadable_expenses > 0;
                    let summary = d.summary.clone();
                    let user_list = d.users.clone();
                    let pickable = pickable_users(Some(&key), &user_list, &[]);
                    // A stored identity the roster no longer contains - removed by another member -
                    // counts as none, so the picker reopens instead of every write being refused.
                    let uid = stored_user_id().filter(|id| pickable.iter().any(|u| u.id == *id));
                    let currency = d.currency.clone();
                    let project_status = d.project_status.clone();
                    let read_only = d.read_only;
                    let guests = match (share_link, uid) {
                        (Some(_), Some(me)) if !read_only && !guests_hint_dismissed(&ls_ctx.read(), project_id) => {
                            unclaimed_guests(&key, &pickable, me)
                        }
                        _ => Vec::new(),
                    };
                    // The edit modal must never open on another project's row.
                    let project_dto = live
                        .read()
                        .as_ref()
                        .filter(|l| l.project_id == project_id)
                        .and_then(|l| l.project.clone());
                    rsx! {

                        if is_stale {
                            div { role: "alert",
                                class: "alert alert-warning alert-soft text-sm",
                                {tid!("projects-offline-banner")}
                            }
                        }

                        if unreadable {
                            div { id: "unreadable-banner", role: "alert",
                                class: "alert alert-warning alert-soft text-sm",
                                {tid!("expenses-unreadable")}
                            }
                        }

                        if read_only {
                            div { id: "demo-banner", role: "status",
                                class: "alert alert-info alert-soft text-sm flex flex-wrap justify-between gap-2",
                                span { {tid!("demo-banner")} }
                                Link { class: "btn btn-primary btn-sm", to: Route::ProjectsPage {},
                                    {tid!("demo-start-own")}
                                }
                            }
                        } else if uid.is_none() && key_offer_shown().is_none() {
                            UserSelectionModal { users: pickable.clone(), project_id }
                        } else if show_switch() {
                            UserSelectionModal {
                                users: pickable.clone(),
                                project_id,
                                on_close: move |_| show_switch.set(false),
                            }
                        }

                        if !guests.is_empty() {
                            GuestsCard {
                                names: guests,
                                on_later: move |_| dismiss_guests(),
                                on_send: move |_| show_guests_sheet.set(true),
                            }
                        }

                        if show_guests_sheet() {
                            if let Some(share_link) = share_link {
                                SendToGuestsSheet {
                                    on_invite: invite.map(|open| EventHandler::new(move |_| {
                                        dismiss_guests();
                                        open.call(());
                                    })),
                                    on_share_link: move |_| {
                                        dismiss_guests();
                                        share_link.call(());
                                    },
                                    on_close: move |_| show_guests_sheet.set(false),
                                }
                            }
                        }

                        if !personal {
                            div { class: "flex justify-center",
                                AvatarGroup {
                                    users: pickable.clone(),
                                    encryption_key: Some(key),
                                    size: SizeClass::W10,
                                }
                            }
                        }

                        if !personal {
                            ProjectStats {
                                global_total: d.global_total,
                                user_total: uid.map(|id| d.user_total(id)),
                                currency: currency.clone(),
                            }
                        }

                        if !personal {
                            ProjectTabs {
                                active: active_tab(),
                                on_select: select_tab,
                            }
                        }

                        div {
                            id: TABPANEL_ID,
                            class: "flex flex-col gap-4",
                            role: if !personal { "tabpanel" },
                            aria_labelledby: if !personal { tab_id(active_tab()) },
                        match if personal { Tab::Expenses } else { active_tab() } {
                            Tab::Expenses => rsx! {
                                ExpensesTab {
                                    data,
                                    live,
                                    stored_user_id: uid,
                                    project_id,
                                    on_expenses_changed: move |m| on_expenses_changed.call(m),
                                    ledgers,
                                    ledgers_loading,
                                }
                            },
                            Tab::Balance => rsx! {
                                BalanceTab {
                                    summary: summary.clone(),
                                    users: user_list.clone(),
                                    currency: currency.clone(),
                                    stored_user_id: uid,
                                }
                            },
                            Tab::Reimbursements => {
                                let users_for_cb = user_list.clone();
                                rsx! {
                                    ReimbursementsTab {
                                        suggestions: summary.reimbursement_suggestions.clone(),
                                        users: user_list.clone(),
                                        currency: currency.clone(),
                                        project_status: project_status.clone(),
                                        read_only,
                                        stored_user_id: uid,
                                        on_reimburse: move |s: ReimbursementSuggestion| {
                                            if let Some(preset) = transfer_preset(&key, &users_for_cb, &s) {
                                                transfer.set(Some(preset));
                                                show_transfer_modal.set(true);
                                            }
                                        },
                                    }
                                }
                            }
                        }

                        }

                        // Transfer modal opened from a reimbursement suggestion
                        if show_transfer_modal() {
                            if let Some((name, amount, payer_id, debtor_id)) = transfer() {
                                AddExpenseModal {
                                    on_close: move |_| {
                                        show_transfer_modal.set(false);
                                        transfer.set(None);
                                    },
                                    on_created: move |_: Option<QueuedAdd>| {
                                        show_transfer_modal.set(false);
                                        transfer.set(None);
                                        flash.set(Some(Flash::ok(tid!("settlement-recorded"))));
                                    },
                                    project_id,
                                    users: pickable_users(Some(&key), &user_list, &[payer_id, debtor_id]),
                                    stored_user_id: uid,
                                    currency: currency.clone(),
                                    initial_name: Some(name),
                                    initial_amount: Some(amount),
                                    initial_expense_type: Some(ExpenseType::Transfer),
                                    initial_payer_id: Some(payer_id),
                                    initial_debtor_id: Some(debtor_id),
                                    restrict_to_transfer: true,
                                }
                            }
                        }

                        if show_edit_modal() {
                            if let Some(p) = project_dto {
                                EditProjectModal {
                                    project: p,
                                    users: user_list.clone(),
                                    on_close: move |_| show_edit_modal.set(false),
                                    on_saved: move |_| {
                                        show_edit_modal.set(false);
                                        sync.restart();
                                    },
                                    on_switch: EventHandler::new(move |_| {
                                        show_edit_modal.set(false);
                                        show_switch.set(true);
                                    }),
                                    on_remove_member: can_rotate.then(|| {
                                        let users = user_list.clone();
                                        EventHandler::new(move |user_id: i32| {
                                            show_edit_modal.set(false);
                                            rotation.set(Some(RotationTarget { k1: key, users: users.clone(), removee: Some(user_id) }));
                                        })
                                    }),
                                }
                            }
                        }
                    }
                }
                None => {
                    // The header's card is the whole page: nothing of the project shows (T10).
                    if went_rotated(sync.read().as_ref().and_then(|r| r.as_ref())) {
                        rsx! {}
                    } else if is_stale {
                        rsx! {
                            NoLocalDataScreen {}
                        }
                    } else if let Some(msg) = body_err_msg.clone() {
                        rsx! {
                            div { role: "alert", class: "alert alert-error", "{msg}" }
                        }
                    } else if unusable() {
                        // The header already carries the message; nothing arrives on its own from
                        // here, so all this needs to offer is the way out of the dead end.
                        rsx! {
                            div { class: "flex justify-center py-4",
                                button {
                                    class: "btn btn-primary btn-sm",
                                    onclick: move |_| sync.restart(),
                                    {tid!("retry")}
                                }
                            }
                        }
                    } else {
                        rsx! {
                            ExpensesSkeleton {}
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
                        show_leave_confirm.set(false);
                        spawn(async move {
                            match leave_project_and_forget(ls_ctx, project_id).await {
                                // `leave_project_and_forget` already refreshed `ls_ctx`.
                                Ok(_) => {
                                    nav.push(Route::ProjectsPage {});
                                }
                                // Membership survives on the server, so keep the local entry -
                                // dropping it leaves a member nobody can remove.
                                Err(e) => flash.set(Some(Flash::err(e))),
                            }
                        });
                    },
                }
            }
        }
    }
}
