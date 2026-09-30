use dioxus::prelude::*;
use crate::tid;
use shared::{ExpenseType, Payment};
use std::collections::VecDeque;
use uuid::Uuid;

use crate::common::{
    format_date_str, initials, pending, read_from_ls, user_color_class, AppHeader, Avatar,
    ConfirmModal, DropdownButton, DropdownItem, Flash, ProjectKey, PullToRefresh, QueuedOp,
};
use crate::decrypted::DecryptedPayment;
use crate::expenses::helpers::delete_expense_action::{
    can_delete_expense, can_edit_expense, delete_expense_request, run_delete_expense,
};
use crate::expenses::helpers::expenses_page_helpers::sync_error;
use crate::expenses::hooks::use_project_store::ProjectStore;
use crate::expenses::tabs::expenses_tab::ExpenseMutation;
use crate::expenses::EditExpenseModal;
use crate::route::Route;

/// Translation keys, not labels — the render site translates.
fn payers_title(t: &ExpenseType) -> &'static str {
    match t {
        ExpenseType::Gain => "payers-title-contributors",
        ExpenseType::Transfer => "payers-title-sender",
        _ => "payers-title-paid-by",
    }
}

fn debtors_title(t: &ExpenseType) -> &'static str {
    match t {
        ExpenseType::Gain => "debtors-title-beneficiaries",
        ExpenseType::Transfer => "debtors-title-recipients",
        _ => "debtors-title-debtors",
    }
}

/// One participant's line in the payers or debtors card.
#[component]
fn ParticipantRow(name: String, user_id: i32, amount: f64, currency: String) -> Element {
    rsx! {
        div { class: "flex items-center gap-3",
            Avatar {
                initials: initials(&name),
                color_class: user_color_class(user_id).to_string(),
            }
            span { class: "flex-1 min-w-0 truncate font-medium text-sm", "{name}" }
            span { class: "shrink-0 font-bold text-sm", "{amount:.2} {currency}" }
        }
    }
}

/// The detail view of one expense.
///
/// It fetches nothing: every figure comes out of the project store's single decryption pass, which
/// `AppLayout` already holds because the list one route up filled it. Opening an expense and going
/// back costs zero requests and zero AEAD opens.
#[component]
pub fn PaymentPage(project_id: Uuid, expense_id: i32) -> Element {
    let nav = use_navigator();
    let mut show_edit = use_signal(|| false);
    let mut show_delete_confirm = use_signal(|| false);

    let stored_user_id = read_from_ls()
        .projects
        .iter()
        .find(|p| p.project_id == project_id)
        .and_then(|p| p.user_id);

    let store = use_context::<ProjectStore>();
    let key_ctx = use_context::<ProjectKey>().0;
    let is_online = use_context::<Signal<bool>>();
    let pending_ops = use_context::<Signal<VecDeque<QueuedOp>>>();
    let flash = use_context::<Signal<Option<Flash>>>();
    let on_expenses_changed = store.on_expenses_changed;

    let body_err = sync_error(store.sync.read().as_ref().and_then(|r| r.as_ref()));
    let data = store.data_for(project_id).filter(|d| d.loaded);

    // Still-encrypted rows, cloned only while the edit modal is open.
    let edit_rows: Option<(shared::Expense, Vec<Payment>)> = show_edit()
        .then(|| {
            let guard = store.live.read();
            let l = guard.as_ref().filter(|l| l.project_id == project_id)?;
            let expense = l.expenses.iter().find(|e| e.id == expense_id)?.clone();
            let payments =
                l.payments.iter().filter(|p| p.expense_id == expense_id).cloned().collect();
            Some((expense, payments))
        })
        .flatten();

    rsx! {
        div { class: "container app-container bg-base-100 overflow-auto p-4 max-w-md w-full mx-auto flex flex-col gap-4",
            PullToRefresh {
                on_refresh: move |_| {
                    let mut sync = store.sync;
                    sync.restart();
                },
                busy: pending(&store.sync),
            }

            match (key_ctx(), data) {
                (None, _) if store.key_missing.cloned() => rsx! {
                    div { role: "alert", class: "alert alert-warning", {tid!("missing-access-key")} }
                },
                (Some(k), Some(d)) => {
                    match d.expenses.iter().find(|r| r.expense.id == expense_id) {
                        // Deleted while this page was open, or a link to a row that is gone.
                        None => rsx! {
                            div { role: "alert", class: "alert alert-warning",
                                {tid!("error-expense-not-found")}
                            }
                        },
                        Some(row) => {
                            let expense = &row.expense;
                            let expense_type = expense.expense_type.clone();
                            let exp_name = expense.name.clone();
                            let of_expense = |dp: &&DecryptedPayment| dp.expense_id == expense_id;
                            let payers: Vec<_> =
                                d.payments.iter().filter(of_expense).filter(|dp| !dp.is_debt).collect();
                            let debtors: Vec<_> =
                                d.payments.iter().filter(of_expense).filter(|dp| dp.is_debt).collect();
                            let payers_sum: f64 = payers.iter().map(|dp| dp.amount).sum();
                            let debtors_sum: f64 = debtors.iter().map(|dp| dp.amount).sum();
                            let can_delete = can_delete_expense(&d.project_status);
                            let can_edit = can_edit_expense(&d.project_status, &expense_type);
                            let currency = d.currency.clone();
                            let name_of = |id: i32| {
                                d.user_names.get(&id).cloned().unwrap_or_else(|| "?".to_string())
                            };
                            // Two clones: one the message formats, one the confirm closure owns.
                            let confirm_name = exp_name.clone();
                            let delete_name = exp_name.clone();
                            rsx! {
                                AppHeader {
                                    title: "{exp_name}",
                                    back_button_route: Route::ExpensesPage { project_id },
                                    sub_title: format_date_str(&expense.date),
                                    if can_edit || can_delete {
                                        DropdownButton {
                                            id: "expense-actions",
                                            label: tid!("expense-actions"),
                                            if can_edit {
                                                DropdownItem {
                                                    id: "edit-expense-item",
                                                    variant: "primary",
                                                    label: tid!("edit"),
                                                    onclick: move |_| show_edit.set(true),
                                                }
                                            }
                                            if can_delete {
                                                DropdownItem {
                                                    id: "delete-expense-item",
                                                    variant: "error",
                                                    label: tid!("delete"),
                                                    onclick: move |_| show_delete_confirm.set(true),
                                                }
                                            }
                                        }
                                    }
                                }

                                // The rate this expense was booked at, frozen at write time. Shown
                                // here and not on the list row: it is what makes the converted
                                // total auditable after the fact, when the market has moved on.
                                if let Some((source_amount, source_currency, rate)) = expense.conversion() {
                                    div {
                                        id: "expense-conversion",
                                        class: "text-sm text-base-content/70",
                                        {tid!(
                                            "expense-converted-from",
                                            amount: format!("{source_amount:.2}"),
                                            from: source_currency.to_string(),
                                            rate: format!("{rate}"),
                                            to: currency.clone()
                                        )}
                                    }
                                }

                                // Written by a tampered or outdated client: the server cannot reject
                                // amounts it cannot read, so the mismatch surfaces here instead.
                                if d.inconsistent.contains(&expense_id) {
                                    div { role: "alert", class: "alert alert-warning text-sm",
                                        {tid!("expense-inconsistent-detail", paid: format!("{payers_sum:.2}"), owed: format!("{debtors_sum:.2}"), total: format!("{:.2}", expense.amount))}
                                    }
                                }

                                div { class: "card bg-base-100 shadow-soft",
                                    div { class: "card-body p-4 gap-3",
                                        h2 { class: "font-semibold text-sm text-base-content/70 uppercase",
                                            {tid!(payers_title(&expense_type))}
                                        }
                                        for dp in payers {
                                            ParticipantRow {
                                                name: name_of(dp.user_id),
                                                user_id: dp.user_id,
                                                amount: dp.amount,
                                                currency: currency.clone(),
                                            }
                                        }
                                    }
                                }

                                div { class: "card bg-base-100 shadow-soft",
                                    div { class: "card-body p-4 gap-3",
                                        h2 { class: "font-semibold text-sm text-base-content/70 uppercase",
                                            {tid!(debtors_title(&expense_type))}
                                        }
                                        for dp in debtors {
                                            ParticipantRow {
                                                name: name_of(dp.user_id),
                                                user_id: dp.user_id,
                                                amount: dp.amount,
                                                currency: currency.clone(),
                                            }
                                        }
                                    }
                                }

                                if let Some((expense, payments)) = edit_rows {
                                    EditExpenseModal {
                                        on_close: move |_| show_edit.set(false),
                                        on_edited: move |edited| {
                                            show_edit.set(false);
                                            on_expenses_changed.call(match edited {
                                                Some(e) => ExpenseMutation::Edited(e),
                                                None => ExpenseMutation::Queued,
                                            });
                                        },
                                        expense,
                                        payments,
                                        users: d.users.clone(),
                                        project_id,
                                        stored_user_id,
                                        currency: currency.clone(),
                                    }
                                }

                                // A delete cascades to the payments and there is no undo, so it is
                                // confirmed here exactly as it is behind the list's swipe shortcut.
                                if show_delete_confirm() {
                                    ConfirmModal {
                                        title: tid!("expense-delete-title"),
                                        message: tid!("expense-delete-message", name: confirm_name.clone()),
                                        confirm_label: tid!("delete"),
                                        on_cancel: move |_| show_delete_confirm.set(false),
                                        on_confirm: move |_| {
                                            show_delete_confirm.set(false);
                                            let name = delete_name.clone();
                                            let req = delete_expense_request(
                                                &k,
                                                expense_id,
                                                project_id,
                                                tid!("history-expense-deleted", name: name.clone()),
                                                stored_user_id.unwrap_or(0),
                                            );
                                            run_delete_expense(
                                                req,
                                                name,
                                                is_online(),
                                                pending_ops,
                                                flash,
                                                move || {
                                                    // Patch the store before leaving, or the list
                                                    // one route up still shows the deleted row.
                                                    on_expenses_changed
                                                        .call(ExpenseMutation::Deleted(expense_id));
                                                    nav.push(Route::ExpensesPage { project_id });
                                                },
                                            );
                                        },
                                    }
                                }
                            }
                        }
                    }
                }
                _ => match body_err {
                    Some((msg, _)) => rsx! {
                        div { role: "alert", class: "alert alert-error", "{msg}" }
                    },
                    None => rsx! {
                        div { class: "flex justify-center py-8",
                            span {
                                class: "loading loading-spinner loading-md",
                                role: "status",
                                aria_label: tid!("loading"),
                            }
                        }
                    },
                },
            }
        }
    }
}
