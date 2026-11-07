use dioxus::prelude::*;
use crate::tid;
use shared::{ExpenseType, Payment};
use std::collections::VecDeque;
use uuid::Uuid;

use crate::common::{
    format_date, format_date_str, initials, pending, read_from_ls, user_color_class, AppHeader,
    Avatar, Flash, ProjectKey, PullToRefresh, QueuedOp,
};
use crate::decrypted::DecryptedPayment;
use crate::expenses::helpers::delete_expense_action::{
    can_delete_expense, can_edit_expense, delete_expense_request, run_delete_expense,
};
use crate::expenses::helpers::expenses_page_helpers::sync_error;
use crate::expenses::hooks::use_project_store::ProjectStore;
use crate::expenses::tabs::expenses_tab::ExpenseMutation;
use crate::expenses::move_expense::{can_move, destinations, typed_amount, MoveDraft, MoveExpenseSheet};
use crate::expenses::EditExpenseModal;
use crate::icons::{RepeatIcon, RightArrowIcon, TransferIcon, ICON_HEADER, ICON_INLINE};
use crate::expenses::project_actions_sheet::MoreButton;
use super::expense_actions_sheet::ExpenseActionsSheet;
use crate::recurring::actions::stop_rule;
use crate::recurring::dialog::DeleteExpenseDialog;
use crate::recurring::store::live_rule_of;
use crate::recurring::view::next_date;
use crate::route::Route;

/// Translation keys, not labels - the render site translates.
fn payers_title(t: &ExpenseType) -> &'static str {
    match t {
        ExpenseType::Gain => "payers-title-contributors",
        ExpenseType::Transfer => "payers-title-sender",
        _ => "payers-title-paid-by",
    }
}

fn debtors_title(t: &ExpenseType, count: usize) -> &'static str {
    match t {
        ExpenseType::Gain => "debtors-title-beneficiaries",
        ExpenseType::Transfer if count == 1 => "debtors-title-recipient",
        ExpenseType::Transfer => "debtors-title-recipients",
        _ => "debtors-title-for",
    }
}

fn share_percent(amount: f64, total: f64) -> u32 {
    if total <= 0.0 {
        return 0;
    }
    (amount / total * 100.0).round().clamp(0.0, 100.0) as u32
}

fn summary_subtitle(kind: String, people: Option<String>, description: Option<&str>) -> String {
    let description = description.map(str::trim).filter(|d| !d.is_empty()).map(str::to_string);
    [Some(kind), people, description].into_iter().flatten().collect::<Vec<_>>().join(" · ")
}

/// One participant's line in the payers or debtors card. `share` adds the proportional bar.
#[component]
fn ParticipantRow(
    name: String,
    user_id: i32,
    amount: f64,
    currency: String,
    share: Option<u32>,
    is_me: bool,
    highlight: bool,
) -> Element {
    let color = user_color_class(user_id);
    rsx! {
        div { class: if highlight { "flex items-center gap-3 -mx-2 px-2 py-2 rounded-xl bg-primary/10" } else { "flex items-center gap-3 py-2" },
            Avatar { initials: initials(&name), color_class: color.to_string() }
            div { class: "flex-1 min-w-0 flex flex-col gap-1.5",
                div { class: "flex items-baseline gap-2",
                    span { class: if is_me { "flex-1 min-w-0 truncate font-semibold text-sm" } else { "flex-1 min-w-0 truncate font-medium text-sm" },
                        if is_me {
                            {tid!("participants-you-badge")}
                        } else {
                            "{name}"
                        }
                    }
                    if let Some(percent) = share {
                        span { class: "shrink-0 text-xs text-base-content/70",
                            {tid!("expense-share-percent", percent: percent as i64)}
                        }
                    }
                    span { class: "shrink-0 font-bold text-sm", {crate::common::amount(amount, &currency)} }
                }
                if let Some(percent) = share {
                    div { class: "h-1.5 rounded-full bg-base-content/10",
                        div { class: "h-1.5 rounded-full {color}", width: "{percent}%" }
                    }
                }
            }
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
    let mut show_move = use_signal(|| false);
    let mut show_actions = use_signal(|| false);

    let stored_user_id = read_from_ls()
        .projects
        .iter()
        .find(|p| p.project_id == project_id)
        .and_then(|p| p.user_id);

    let store = use_context::<ProjectStore>();
    let key_ctx = use_context::<ProjectKey>().0;
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
        div { class: "container app-container bg-base-100 overflow-auto p-4 max-w-md w-full mx-auto flex flex-col gap-4 pb-24",
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
                            let mut debtors: Vec<_> =
                                d.payments.iter().filter(of_expense).filter(|dp| dp.is_debt).collect();
                            let payers_sum: f64 = payers.iter().map(|dp| dp.amount).sum();
                            let debtors_sum: f64 = debtors.iter().map(|dp| dp.amount).sum();
                            let can_delete = can_delete_expense(&d.project_status, d.read_only);
                            let can_edit = can_edit_expense(&d.project_status, d.read_only, &expense_type);
                            let currency = d.currency.clone();
                            let name_of = |id: i32| {
                                d.user_names.get(&id).cloned().unwrap_or_else(|| "?".to_string())
                            };
                            // Two clones: one the message formats, one the confirm closure owns.
                            let confirm_name = exp_name.clone();
                            let delete_name = exp_name.clone();
                            let rules = store.recurring_for(project_id);
                            let origin_next: Option<Option<chrono::NaiveDate>> = expense
                                .recurring_id
                                .and_then(|id| rules.as_ref()?.rule(id))
                                .map(|r| next_date(&r.payload));
                            let live_rule = live_rule_of(&d, expense_id, rules.as_ref());
                            let expense_date = expense.date.clone();
                            let is_transfer = expense_type == ExpenseType::Transfer;
                            let kind = match expense_type {
                                ExpenseType::Transfer => tid!("expense-type-transfer"),
                                ExpenseType::Gain => tid!("expense-type-gain"),
                                ExpenseType::Expense => row.category.label(),
                            };
                            let people = (!is_transfer && !d.personal)
                                .then(|| tid!("expense-people", count: debtors.len() as i64));
                            let subtitle = summary_subtitle(kind, people, expense.description.as_deref());
                            let emoji = row.emoji.clone();
                            let category_parent = row.category.parent;
                            let total = expense.amount;
                            let added_by = expense
                                .author_id
                                .filter(|_| is_transfer)
                                .map(|id| (name_of(id), format_date(expense.created_at.date())));
                            let is_me = |id: i32| !is_transfer && stored_user_id == Some(id);
                            debtors.sort_by_key(|dp| !is_me(dp.user_id));
                            let debtors_heading = debtors_title(&expense_type, debtors.len());
                            let split_count = (!is_transfer && !d.personal && debtors.len() > 1).then_some(debtors.len());
                            let state = read_from_ls();
                            let targets = destinations(state.cached_projects_list.as_deref().unwrap_or_default(), &state, project_id);
                            let movable = can_delete && can_move(&expense_type) && targets.iter().any(|t| !t.current);
                            let source_name = targets.iter().find(|t| t.current).map(|t| t.name.clone()).unwrap_or_default();
                            let source_personal = targets.iter().any(|t| t.current && t.personal);
                            rsx! {
                                AppHeader {
                                    title: "{exp_name}",
                                    private_title: true,
                                    back_button_route: Route::ExpensesPage { project_id },
                                    sub_title: format_date_str(&expense.date),
                                    if can_edit || can_delete || movable {
                                        MoreButton { id: "expense-actions", label: tid!("expense-actions"), open: show_actions }
                                    }
                                }
                                if show_actions() {
                                    ExpenseActionsSheet {
                                        title: exp_name.clone(),
                                        can_edit,
                                        movable,
                                        can_delete,
                                        on_close: move |_| show_actions.set(false),
                                        on_edit: move |_| show_edit.set(true),
                                        on_move: move |_| show_move.set(true),
                                        on_delete: move |_| show_delete_confirm.set(true),
                                    }
                                }

                                div { id: "expense-summary", class: "card bg-base-100 shadow-soft",
                                    div { class: "card-body p-4 gap-3",
                                    p { id: "expense-full-name", class: "font-semibold leading-snug wrap-anywhere text-pretty", "{exp_name}" }
                                    div { class: "flex items-center gap-3",
                                        if is_transfer {
                                            span { class: "size-11 shrink-0 rounded-2xl bg-primary/10 text-primary flex items-center justify-center",
                                                TransferIcon { size: ICON_HEADER }
                                            }
                                        } else {
                                            span { class: "shrink-0 text-3xl", aria_hidden: "true", "{emoji}" }
                                        }
                                        div { class: "flex-1 min-w-0",
                                            p { class: "font-display font-bold text-2xl", {crate::common::amount(total, &currency)} }
                                            p { class: "text-sm text-base-content/70 truncate", "{subtitle}" }
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
                                            amount: crate::common::number(source_amount),
                                            from: source_currency.to_string(),
                                            rate: format!("{rate}"),
                                            to: currency.clone()
                                        )}
                                    }
                                }

                                if expense.estimate {
                                    div { id: "estimate-alert", role: "alert", class: "alert alert-warning alert-soft text-sm items-start",
                                        div { class: "flex flex-col gap-2",
                                            span {
                                                span { class: "font-semibold", {tid!("estimate-title")} }
                                                " "
                                                {tid!("estimate-body")}
                                            }
                                            if can_edit {
                                                button {
                                                    id: "estimate-confirm",
                                                    r#type: "button",
                                                    class: "btn btn-sm btn-secondary self-start",
                                                    onclick: move |_| show_edit.set(true),
                                                    {tid!("estimate-confirm")}
                                                }
                                            }
                                        }
                                    }
                                }

                                if let Some(rule_id) = expense.recurring_id {
                                    div { id: "occurrence-origin", class: "alert alert-soft text-sm items-start bg-primary/10 border-0",
                                        RepeatIcon { size: ICON_INLINE }
                                        div { class: "flex flex-col gap-1 min-w-0",
                                            match origin_next {
                                                Some(Some(date)) => rsx! { span { {tid!("occurrence-auto-next", date: format_date(date))} } },
                                                Some(None) => rsx! { span { {tid!("occurrence-auto")} } },
                                                None => rsx! { span { {tid!("occurrence-auto-stopped")} } },
                                            }
                                            if origin_next.is_some() {
                                                button {
                                                    id: "occurrence-manage",
                                                    r#type: "button",
                                                    class: "link link-primary font-semibold self-start text-sm",
                                                    onclick: move |_| { nav.push(Route::RecurringRulePage { project_id, rule_id }); },
                                                    {tid!("occurrence-manage")}
                                                }
                                            }
                                        }
                                    }
                                }

                                // Written by a tampered or outdated client: the server cannot reject
                                // amounts it cannot read, so the mismatch surfaces here instead.
                                if d.inconsistent.contains(&expense_id) {
                                    div { role: "alert", class: "alert alert-warning text-sm",
                                        {tid!("expense-inconsistent-detail", paid: crate::common::number(payers_sum), owed: crate::common::number(debtors_sum), total: crate::common::number(expense.amount))}
                                    }
                                }

                                div { class: "flex flex-col",
                                    div { class: "card bg-base-100 shadow-soft",
                                        div { class: "card-body p-4 gap-0",
                                            h2 { class: "font-semibold text-xs tracking-wide text-base-content/70 uppercase mb-1",
                                                {tid!(payers_title(&expense_type))}
                                            }
                                            for dp in payers {
                                                ParticipantRow {
                                                    name: name_of(dp.user_id),
                                                    user_id: dp.user_id,
                                                    amount: dp.amount,
                                                    currency: currency.clone(),
                                                    share: None,
                                                    is_me: is_me(dp.user_id),
                                                    highlight: is_me(dp.user_id) && !d.personal,
                                                }
                                            }
                                        }
                                    }
                                    div { class: "relative flex justify-center py-2",
                                        div { class: "absolute inset-y-0 left-1/2 w-0.5 -translate-x-1/2 bg-base-content/10" }
                                        span { id: "expense-split-connector", class: "relative flex items-center gap-1.5 rounded-full border border-base-content/10 bg-base-200 px-3 py-1 text-xs font-semibold text-base-content/70",
                                            span { class: "flex rotate-90", RightArrowIcon { size: 12 } }
                                            if let Some(count) = split_count {
                                                {tid!("expense-split-between", count: count as i64)}
                                            }
                                        }
                                    }
                                    div { class: "card bg-base-100 shadow-soft",
                                        div { class: "card-body p-4 gap-0",
                                            h2 { id: "expense-debtors-title", class: "font-semibold text-xs tracking-wide text-base-content/70 uppercase mb-1",
                                                {tid!(debtors_heading)}
                                            }
                                            div { id: "expense-debtors", class: "flex flex-col",
                                                for dp in debtors {
                                                    ParticipantRow {
                                                        name: name_of(dp.user_id),
                                                        user_id: dp.user_id,
                                                        amount: dp.amount,
                                                        currency: currency.clone(),
                                                        share: (!is_transfer && !d.personal).then(|| share_percent(dp.amount, total)),
                                                        is_me: is_me(dp.user_id),
                                                        highlight: is_me(dp.user_id) && !d.personal,
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }

                                if let Some((author, created)) = added_by {
                                    p { id: "expense-added-by", class: "px-1 text-xs text-base-content/70",
                                        {tid!("expense-added-by", name: author, date: created)}
                                    }
                                }

                                if let Some((expense, payments)) = edit_rows {
                                    EditExpenseModal {
                                        on_close: move |_| show_edit.set(false),
                                        on_edited: move |edited| {
                                            show_edit.set(false);
                                            on_expenses_changed.call(ExpenseMutation::Edited(edited));
                                        },
                                        expense,
                                        payments,
                                        users: d.users.clone(),
                                        project_id,
                                        stored_user_id,
                                        currency: currency.clone(),
                                    }
                                }

                                if show_move() {
                                    MoveExpenseSheet {
                                        destinations: targets.clone(),
                                        from_group: (!source_personal).then(|| source_name.clone()),
                                        on_close: move |_| show_move.set(false),
                                        on_pick: {
                                            let targets = targets.clone();
                                            let source_name = source_name.clone();
                                            let currency = currency.clone();
                                            move |to: Uuid| {
                                                show_move.set(false);
                                                let Some(target) = targets.iter().find(|t| t.project_id == to) else { return };
                                                // The ciphertext, not the decrypted row: it carries
                                                // the fields this build does not know.
                                                let payload = store.live.read().as_ref()
                                                    .filter(|l| l.project_id == project_id)
                                                    .and_then(|l| l.expenses.iter().find(|e| e.id == expense_id).cloned())
                                                    .and_then(|e| crate::crypto::decrypt_json::<shared::ExpensePayload>(&k, &e.payload).ok());
                                                let Some(payload) = payload else { return };
                                                let (amount, typed_currency) = typed_amount(&payload, &currency);
                                                let mut pending_move = store.pending_move;
                                                pending_move.set(Some(MoveDraft {
                                                    from_project: project_id,
                                                    from_name: source_name.clone(),
                                                    from_key: k,
                                                    from_user_id: stored_user_id,
                                                    expense_id,
                                                    to_project: to,
                                                    to_name: target.name.clone(),
                                                    to_personal: target.personal,
                                                    payload,
                                                    amount,
                                                    currency: typed_currency,
                                                    category_parent,
                                                }));
                                                // Replace, not push: the expense page left behind is about to stop existing.
                                                nav.replace(Route::ExpensesPage { project_id: to });
                                            }
                                        },
                                    }
                                }

                                // A delete cascades to the payments and there is no undo, so it is
                                // confirmed here exactly as it is behind the list's swipe shortcut.
                                if show_delete_confirm() {
                                    DeleteExpenseDialog {
                                        name: confirm_name.clone(),
                                        date: expense_date.clone(),
                                        occurrence: live_rule.is_some(),
                                        on_cancel: move |_| show_delete_confirm.set(false),
                                        on_delete: move |stop: bool| {
                                            show_delete_confirm.set(false);
                                            if let Some(rule) = live_rule.as_ref().filter(|_| stop) {
                                                stop_rule(k, project_id, rule, stored_user_id, flash, move || {
                                                    let mut sync = store.sync;
                                                    sync.restart();
                                                });
                                            }
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
                                                pending_ops,
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
                    None => rsx! { crate::common::ExpenseDetailSkeleton {} },
                },
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn share_percent_rounds_to_the_nearest_point() {
        assert_eq!(share_percent(30.0, 86.4), 35);
        assert_eq!(share_percent(16.4, 86.4), 19);
        assert_eq!(share_percent(12.5, 12.5), 100);
    }

    #[test]
    fn share_percent_survives_a_zero_or_inconsistent_total() {
        assert_eq!(share_percent(10.0, 0.0), 0);
        assert_eq!(share_percent(10.0, -5.0), 0);
        assert_eq!(share_percent(20.0, 10.0), 100);
    }

    #[test]
    fn summary_subtitle_appends_a_non_blank_description() {
        assert_eq!(summary_subtitle("Transfer".into(), None, Some(" Lydia ")), "Transfer · Lydia");
        assert_eq!(summary_subtitle("Food".into(), None, Some("  ")), "Food");
        assert_eq!(summary_subtitle("Food".into(), None, None), "Food");
    }

    #[test]
    fn summary_subtitle_counts_the_people_before_the_description() {
        assert_eq!(summary_subtitle("Food".into(), Some("4 people".into()), None), "Food · 4 people");
        assert_eq!(summary_subtitle("Food".into(), Some("4 people".into()), Some("Lidl")), "Food · 4 people · Lidl");
    }

    #[test]
    fn one_recipient_is_named_in_the_singular() {
        assert_eq!(debtors_title(&ExpenseType::Transfer, 1), "debtors-title-recipient");
        assert_eq!(debtors_title(&ExpenseType::Transfer, 2), "debtors-title-recipients");
        assert_eq!(debtors_title(&ExpenseType::Expense, 4), "debtors-title-for");
    }
}
