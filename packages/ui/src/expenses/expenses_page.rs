use api::projects::projects_controller::update_project_by_id;
use dioxus::fullstack::Json;
use dioxus::prelude::*;
use crate::tid;
use shared::{EditableProject, ExpenseType, ExpenseWithPayments, ReimbursementSuggestion};
use uuid::Uuid;

use super::helpers::export;
use crate::common::{
    copy_text, is_mobile, leave_project_and_forget, pending, read_from_ls,
    remove_project, scheme_link_for, share_link_for, share_text, write_to_ls, AppHeader, AvatarGroup, ConfirmModal, Flash,
    LocalStorageState, ProjectKey, PullToRefresh, SizeClass, Toast, LEAVE_CONFIRM_MESSAGE,
    LEAVE_CONFIRM_TITLE,
};
use crate::expenses::helpers::expenses_page_helpers::{
    header_state, sync_error, transfer_preset, DownloadFn,
};
use crate::expenses::hooks::use_project_store::ProjectStore;
use crate::expenses::project_header::ProjectHeader;
use crate::friends::InviteFriendsModal;
use crate::projects::JoinProjectModal;
use crate::expenses::project_states::{
    ExpensesSkeleton, MissingKeyScreen, NoLocalDataScreen, Spinner,
};
use crate::expenses::project_stats::ProjectStats;
use crate::expenses::tabs::expenses_tab::ExpenseMutation;
use crate::expenses::tabs::project_tabs::{ProjectTabs, Tab};
use crate::expenses::{
    AddExpenseModal, BalanceTab, EditProjectModal, ExpensesTab, ReimbursementsTab,
    UserSelectionModal,
};
use crate::route::Route;

#[component]
pub fn ExpensesPage(project_id: Uuid) -> Element {
    let nav = use_navigator();
    let mut ls_ctx = use_context::<Signal<LocalStorageState>>();
    let mut flash = use_context::<Signal<Option<Flash>>>();
    let auth_ctx = use_context::<Signal<Option<shared::Account>>>();
    let mut copy_error: Signal<Option<String>> = use_signal(|| None);
    let mut active_tab = use_signal(|| Tab::Expenses);
    let mut show_transfer_modal = use_signal(|| false);
    let mut show_edit_modal = use_signal(|| false);
    let mut show_invite_modal = use_signal(|| false);
    let mut show_leave_confirm = use_signal(|| false);
    let mut show_unlock_modal = use_signal(|| false);
    let mut transfer = use_signal(|| None::<(String, f64, i32, i32)>);

    // "Ouvrir dans l'application" targets a phone *browser* — not the native app, not desktop.
    // The user agent is unknown to SSR, so deciding on the first render would break hydration;
    // gated post-mount like onboarding in app_layout.rs.
    let mut mounted = use_signal(|| false);
    use_effect(move || mounted.set(true));
    let offer_native_app = mounted() && cfg!(target_arch = "wasm32") && is_mobile();

    // Owned by AppLayout, so it survives navigating to an expense and back.
    let store = use_context::<ProjectStore>();
    let (mut sync, live, data, key_missing, unusable) =
        (store.sync, store.live, store.data, store.key_missing, store.unusable);
    let on_expenses_changed = store.on_expenses_changed;
    let key_ctx = use_context::<ProjectKey>().0;

    let stored_user_id = move || {
        ls_ctx().projects.iter().find(|p| p.project_id == project_id).and_then(|p| p.user_id)
    };

    // One small DTO, not `ls_ctx()`, which would deep-clone every cached expense and payment per
    // render. Only the header's offline branch needs it, and only for the name.
    let cached_project = ls_ctx
        .read()
        .projects
        .iter()
        .find(|p| p.project_id == project_id)
        .and_then(|p| p.cached_project.clone());

    // The only exit from a dangling share link: the id is gone server-side, so every resource
    // here keeps failing until the local entry is dropped.
    let forget_project = move |_| {
        let mut state = read_from_ls();
        remove_project(&mut state, project_id);
        write_to_ls(&state);
        ls_ctx.set(state);
        nav.push(Route::ProjectsPage {});
    };

    if key_ctx().is_none() {
        return rsx! {
            div { class: "container app-container bg-base-100 overflow-auto p-4 pb-36 max-w-md w-full mx-auto flex flex-col gap-4",
                PullToRefresh { on_refresh: move |_| sync.restart(), busy: pending(&sync) }
                AppHeader {
                    back_button_route: Route::ProjectsPage {},
                    title: "Counted",
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

    // Exports `live`, not the resource, so an export taken right after an edit contains it.
    let export_all = move |download: DownloadFn| {
        let snapshot = live.read();
        let Some(l) = snapshot.as_ref().filter(|l| l.project_id == project_id) else { return };
        let Some(proj) = l.project.as_ref() else { return };
        download(&key, proj, &l.users, &l.expenses, &l.payments);
    };

    // (message, server-unreachable) — the second drives the cache fallback.
    let body_err = sync_error(sync.read().as_ref().and_then(|r| r.as_ref()));
    let is_stale = body_err.as_ref().map(|(_, offline)| *offline).unwrap_or(false);
    let body_err_msg = body_err.as_ref().map(|(msg, _)| msg.clone());

    rsx! {
        div { class: "container app-container bg-base-100 overflow-auto p-4 pb-36 max-w-md w-full mx-auto flex flex-col gap-4",
            PullToRefresh { on_refresh: move |_| sync.restart(), busy: pending(&sync) }

            ProjectHeader {
                state: header_state(sync.read().as_ref().and_then(|r| r.as_ref()), cached_project.as_ref(), &key, project_id),
                native_app_link: offer_native_app.then(|| scheme_link_for(project_id, &key)),
                on_history: move |_| { nav.push(Route::ProjectHistoryPage { project_id }); },
                on_edit: move |_| show_edit_modal.set(true),
                on_leave: move |_| show_leave_confirm.set(true),
                on_status: move |status| {
                    spawn(async move {
                        let editable = EditableProject {
                            id: project_id,
                            payload: None,
                            status: Some(status),
                            history: None,
                        };
                        let _ = update_project_by_id(Json(editable)).await;
                        sync.restart();
                    });
                },
                on_export_json: move |_| export_all(export::download_json),
                on_export_csv: move |_| export_all(export::download_csv),
                // `share_text` is false without a sheet (desktop browsers) or when it could not
                // open, and true on dismiss — so the clipboard is only the fallback, never a second
                // prompt after a shown sheet.
                on_share: move |_| {
                    let url = share_link_for(project_id, &key);
                    spawn(async move {
                        if share_text(&url).await {
                            return;
                        }
                        if copy_text(&url).await {
                            flash.set(Some(Flash::ok(tid!("link-copied"))));
                        } else {
                            copy_error.set(Some(tid!("copy-link-failed")));
                        }
                    });
                },
                on_invite: auth_ctx().is_some().then(|| EventHandler::new(move |_| show_invite_modal.set(true))),
                on_forget: forget_project,
            }

            if show_invite_modal() {
                InviteFriendsModal {
                    project_id,
                    project_key: key,
                    on_close: move |_| show_invite_modal.set(false),
                }
            }

            // The store's decryption pass, but only once it is this project's — it still holds the
            // previous one until the new sync lands.
            match store.data_for(project_id).filter(|d| d.loaded) {
                Some(d) => {
                    let summary = d.summary.clone();
                    let user_list = d.users.clone();
                    // A stored identity the roster no longer contains — removed by another member
                    // — counts as none, so the picker reopens instead of every write being refused.
                    let uid = stored_user_id().filter(|id| user_list.iter().any(|u| u.id == *id));
                    let currency = d.currency.clone();
                    let project_status = d.project_status.clone();
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

                        if uid.is_none() {
                            UserSelectionModal { users: user_list.clone(), project_id }
                        }

                        div { class: "flex justify-center",
                            AvatarGroup {
                                users: user_list.clone(),
                                encryption_key: Some(key),
                                size: SizeClass::W10,
                            }
                        }

                        ProjectStats {
                            global_total: d.global_total,
                            user_total: uid.map(|id| d.user_total(id)),
                            currency: currency.clone(),
                        }

                        ProjectTabs {
                            active: active_tab(),
                            on_select: move |t| active_tab.set(t),
                        }

                        match active_tab() {
                            Tab::Expenses => rsx! {
                                ExpensesTab {
                                    data,
                                    live,
                                    stored_user_id: uid,
                                    project_id,
                                    on_expenses_changed: move |m| on_expenses_changed.call(m),
                                }
                            },
                            Tab::Balance => rsx! {
                                BalanceTab {
                                    summary: summary.clone(),
                                    users: user_list.clone(),
                                    currency: currency.clone(),
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

                        // Transfer modal opened from a reimbursement suggestion
                        if show_transfer_modal() {
                            if let Some((name, amount, payer_id, debtor_id)) = transfer() {
                                AddExpenseModal {
                                    on_close: move |_| {
                                        show_transfer_modal.set(false);
                                        transfer.set(None);
                                    },
                                    on_created: move |created: Option<ExpenseWithPayments>| {
                                        show_transfer_modal.set(false);
                                        transfer.set(None);
                                        on_expenses_changed.call(match created {
                                            Some(c) => ExpenseMutation::Added(c),
                                            None => ExpenseMutation::Queued,
                                        });
                                    },
                                    project_id,
                                    users: user_list.clone(),
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
                                }
                            }
                        }
                    }
                }
                None => {
                    if is_stale {
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

            if let Some(msg) = copy_error() {
                Toast {
                    msg,
                    onclose: move |_| copy_error.set(None),
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
                                // Membership survives on the server, so keep the local entry —
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
