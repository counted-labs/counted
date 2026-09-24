use dioxus::prelude::*;
use crate::tid;
use shared::{Expense, ExpenseType, ExpenseWithPayments, Payment, ProjectStatus};
use std::collections::{BTreeMap, HashMap, HashSet, VecDeque};
use std::sync::Arc;
use uuid::Uuid;

use crate::common::{
    format_date, haptic, ConfirmModal, EmptyMagnifyingGlassIllustration, Flash, Haptic, ProjectKey,
    QueuedOp, ScanSource, SpeedDialAction, Toast,
};
use crate::crypto::{payments_are_inconsistent, DecryptedExpense, DecryptedPayment};
use crate::expenses::helpers::delete_expense_action::{
    can_delete_expense, can_edit_expense, delete_expense_request, run_delete_expense,
};
use crate::expenses::helpers::project_data::{LiveData, ProjectData, RowExpense};
use crate::expenses::hooks::use_receipt_scan::use_receipt_scan;
use crate::expenses::{AddExpenseModal, EditExpenseModal, ExpenseRow};
use crate::icons::{CameraIcon, CloseIcon, PhotoIcon, PlusIcon, ICON_INLINE};
use crate::route::Route;

#[cfg(test)]
mod tests {
    use super::*;

    fn de(id: i32, amount: f64) -> DecryptedExpense {
        DecryptedExpense {
            id,
            author_id: Some(1),
            project_id: Uuid::nil(),
            created_at: chrono::NaiveDateTime::default(),
            name: format!("expense {id}"),
            description: None,
            amount,
            expense_type: ExpenseType::Expense,
            date: "2025-01-01".to_string(),
            category: None,
            source_currency: None,
            source_amount: None,
            rate: None,
        }
    }

    fn dp(expense_id: i32, is_debt: bool, amount: f64) -> DecryptedPayment {
        DecryptedPayment {
            id: 0,
            expense_id,
            user_id: 0,
            is_debt,
            amount,
            created_at: chrono::NaiveDateTime::default(),
        }
    }

    #[test]
    fn a_balanced_project_flags_nothing() {
        let expenses = [de(1, 100.0), de(2, 30.0)];
        let payments = [
            dp(1, false, 100.0),
            dp(1, true, 50.0),
            dp(1, true, 50.0),
            dp(2, false, 30.0),
            dp(2, true, 30.0),
        ];
        assert!(inconsistent_expense_ids(&expenses, &payments).is_empty());
    }

    #[test]
    fn only_the_broken_expense_is_flagged() {
        let expenses = [de(1, 100.0), de(2, 30.0)];
        let payments = [
            // expense 1: 120 paid, 140 owed on a 100 expense
            dp(1, false, 60.0),
            dp(1, false, 60.0),
            dp(1, true, 70.0),
            dp(1, true, 70.0),
            // expense 2 is fine
            dp(2, false, 30.0),
            dp(2, true, 30.0),
        ];
        let flagged = inconsistent_expense_ids(&expenses, &payments);
        assert_eq!(flagged, HashSet::from([1]));
    }

    #[test]
    fn payments_never_leak_across_expenses() {
        // Both expenses balance only when each is judged on its own rows; pooled, neither would.
        let expenses = [de(1, 10.0), de(2, 90.0)];
        let payments = [
            dp(1, false, 10.0),
            dp(1, true, 10.0),
            dp(2, false, 90.0),
            dp(2, true, 90.0),
        ];
        assert!(inconsistent_expense_ids(&expenses, &payments).is_empty());
    }

    #[test]
    fn an_expense_without_payments_is_not_flagged() {
        // Nothing decrypted for it — a missing key, not a broken expense.
        let expenses = [de(1, 100.0)];
        assert!(inconsistent_expense_ids(&expenses, &[]).is_empty());
    }

    #[test]
    fn payments_of_an_unknown_expense_are_ignored() {
        let expenses = [de(1, 100.0)];
        let payments = [dp(1, false, 100.0), dp(1, true, 100.0), dp(99, false, 5.0)];
        assert!(inconsistent_expense_ids(&expenses, &payments).is_empty());
    }

    #[test]
    fn payor_connector_expense_is_payee_par() {
        assert_eq!(payor_connector(&ExpenseType::Expense), "expense-paid-by");
    }

    #[test]
    fn payor_connector_gain_is_contribue_par() {
        assert_eq!(payor_connector(&ExpenseType::Gain), "expense-contributed-by");
    }

    #[test]
    fn payor_connector_transfer_is_envoye_par() {
        assert_eq!(payor_connector(&ExpenseType::Transfer), "expense-sent-by");
    }

    #[test]
    fn expense_filter_labels() {
        assert_eq!(expense_filter_label(ExpenseFilter::All), "filter-all");
        assert_eq!(expense_filter_label(ExpenseFilter::MyPayments), "filter-my-payments");
        assert_eq!(expense_filter_label(ExpenseFilter::MyDebts), "filter-my-debts");
    }

    #[test]
    fn wants_debt_all_does_not_filter() {
        assert_eq!(wants_debt(ExpenseFilter::All), None);
    }

    #[test]
    fn wants_debt_my_payments_keeps_credits() {
        assert_eq!(wants_debt(ExpenseFilter::MyPayments), Some(false));
    }

    #[test]
    fn wants_debt_my_debts_keeps_debts() {
        assert_eq!(wants_debt(ExpenseFilter::MyDebts), Some(true));
    }

    fn dpu(expense_id: i32, user_id: i32, is_debt: bool, amount: f64) -> DecryptedPayment {
        DecryptedPayment { user_id, ..dp(expense_id, is_debt, amount) }
    }

    #[test]
    fn my_share_is_reported_per_expense() {
        let payments = [
            dpu(1, 7, false, 100.0),
            dpu(1, 7, true, 40.0),
            dpu(1, 8, true, 60.0),
            dpu(2, 8, false, 30.0),
            dpu(2, 7, true, 30.0),
        ];
        assert_eq!(my_debt_by_expense(&payments, 7), HashMap::from([(1, 40.0), (2, 30.0)]));
    }

    #[test]
    fn what_i_paid_is_not_what_i_owe() {
        // The credit row is mine and sits on the same expense — it must not be counted.
        let payments = [dpu(1, 7, false, 100.0), dpu(1, 7, true, 25.0)];
        assert_eq!(my_debt_by_expense(&payments, 7), HashMap::from([(1, 25.0)]));
    }

    #[test]
    fn other_peoples_debts_are_never_mine() {
        let payments = [dpu(1, 8, true, 60.0), dpu(2, 9, true, 10.0)];
        assert!(my_debt_by_expense(&payments, 7).is_empty());
    }

    /// Duplicated debt rows are inconsistent data, not a reason to under-report: the row shows
    /// the whole claim and the ⚠ badge says the expense no longer adds up.
    #[test]
    fn several_debt_rows_of_mine_add_up() {
        let payments = [dpu(1, 7, true, 30.0), dpu(1, 7, true, 20.0)];
        assert_eq!(my_debt_by_expense(&payments, 7), HashMap::from([(1, 50.0)]));
    }

    #[test]
    fn an_expense_i_owe_nothing_on_is_absent() {
        // Absent, not zero — the row then renders no second line at all.
        let payments = [dpu(1, 7, false, 100.0)];
        assert!(!my_debt_by_expense(&payments, 7).contains_key(&1));
    }

    // `project_data.rs` guards the decryption pass itself. What these prove is the other half:
    // filtering, grouping and the row loop cost no crypto at all.

    use crate::common::test_fixtures::{large_project, test_key};
    use crate::crypto::count_decrypts;
    use crate::expenses::helpers::project_data;

    fn live_of(expenses: Vec<Expense>, payments: Vec<Payment>) -> LiveData {
        LiveData {
            project_id: Uuid::nil(),
            project: None,
            users: vec![],
            expenses,
            payments,
            version: None,
        }
    }

    fn built() -> ProjectData {
        let p = large_project();
        project_data::build(
            Some(test_key()),
            Some(&LiveData {
                project_id: Uuid::nil(),
                project: None,
                users: p.users.clone(),
                expenses: p.expenses.clone(),
                payments: p.payments.clone(),
                version: None,
            }),
        )
    }

    /// Everything the body does below the decryption pass, as the component does it.
    fn render(data: &ProjectData, filter: ExpenseFilter, uid: Option<i32>) -> usize {
        let groups = group_expenses(data, filter, uid);
        let (page, rendered) = take_rows(&groups, PAGE);
        if let (ExpenseFilter::MyDebts, Some(u)) = (filter, uid) {
            let _ = my_debt_by_expense(&data.payments, u);
        }
        for (_, group) in &page {
            for row in group {
                let _ = data.payor_label(row.expense.id);
                let _ = data.inconsistent.contains(&row.expense.id);
                let _ = row.emoji;
            }
        }
        rendered
    }

    /// A filter tap re-runs the whole body and must open no ciphertext — it used to cost a full
    /// 6 010-op pass every time.
    #[test]
    fn a_filter_tap_decrypts_nothing() {
        let data = built();
        for filter in [ExpenseFilter::All, ExpenseFilter::MyPayments, ExpenseFilter::MyDebts] {
            let (_, n) = count_decrypts(|| render(&data, filter, Some(1)));
            assert_eq!(
                n, 0,
                "filter {:?} re-decrypted — the pass belongs above the body, not in it",
                expense_filter_label(filter)
            );
        }
    }

    /// The window only decides how much of an already-decrypted list reaches the DOM.
    #[test]
    fn growing_the_window_decrypts_nothing() {
        let data = built();
        let groups = group_expenses(&data, ExpenseFilter::All, Some(1));
        let (_, n) = count_decrypts(|| {
            for page in 1..=5 {
                let _ = take_rows(&groups, PAGE * page);
            }
        });
        assert_eq!(n, 0);
    }

    #[test]
    fn the_list_starts_at_one_page_not_the_whole_project() {
        let data = built();
        let groups = group_expenses(&data, ExpenseFilter::All, Some(1));
        let total: usize = groups.iter().map(|(_, g)| g.len()).sum();
        assert_eq!(total, 1_000, "the fixture is 1 000 expenses");

        let (_, rendered) = take_rows(&groups, PAGE);
        assert!(rendered >= PAGE, "a page must be filled");
        assert!(rendered < total, "and it must not be the whole project");
    }

    /// A day is never split across the boundary — a divider with half its rows under it would read
    /// as data loss.
    #[test]
    fn a_day_is_never_split_across_the_window_edge() {
        let data = built();
        let groups = group_expenses(&data, ExpenseFilter::All, Some(1));
        let (page, rendered) = take_rows(&groups, PAGE);
        let counted: usize = page.iter().map(|(_, g)| g.len()).sum();
        assert_eq!(counted, rendered);
        for (date, group) in &page {
            let whole = groups.iter().find(|(d, _)| d == date).unwrap();
            assert_eq!(group.len(), whole.1.len(), "day {date} arrived partially");
        }
    }

    #[test]
    fn a_big_enough_window_shows_everything() {
        let data = built();
        let groups = group_expenses(&data, ExpenseFilter::All, Some(1));
        let (page, rendered) = take_rows(&groups, 10_000);
        assert_eq!(rendered, 1_000);
        assert_eq!(page.len(), groups.len());
    }

    #[test]
    fn an_empty_list_yields_no_groups_and_no_rows() {
        let data = ProjectData::default();
        let groups = group_expenses(&data, ExpenseFilter::All, Some(1));
        let (page, rendered) = take_rows(&groups, PAGE);
        assert!(page.is_empty());
        assert_eq!(rendered, 0);
    }

    // A mutation must leave the local lists exactly as the refetch it replaces would have.

    use crate::common::test_fixtures::{make_expense, make_payment};
    use shared::ExpenseType;

    fn exp(id: i32, name: &str) -> Expense {
        make_expense(&test_key(), id, name, 10.0, ExpenseType::Expense, "2025-01-01")
    }

    fn pay(id: i32, expense_id: i32, user_id: i32) -> Payment {
        make_payment(&test_key(), id, expense_id, user_id, false, 10.0)
    }

    fn two_expenses() -> (Vec<Expense>, Vec<Payment>) {
        (
            vec![exp(1, "Pizza"), exp(2, "Taxi")],
            vec![pay(1, 1, 7), pay(2, 1, 8), pay(3, 2, 7)],
        )
    }

    #[test]
    fn an_add_appends_the_expense_and_its_payments() {
        let (mut e, mut p) = two_expenses();
        let m = ExpenseMutation::Added(ExpenseWithPayments {
            expense: exp(3, "Bière"),
            payments: vec![pay(4, 3, 7), pay(5, 3, 8)],
        });
        assert!(apply_mutation(&mut e, &mut p, &m));
        assert_eq!(e.len(), 3);
        assert_eq!(p.len(), 5);
        assert_eq!(p.iter().filter(|p| p.expense_id == 3).count(), 2);
    }

    /// The server deletes and re-inserts an expense's payments on edit, so the new rows have new
    /// ids. Merging by payment id would leave the old ones behind and double the expense.
    #[test]
    fn an_edit_replaces_the_expense_and_all_of_its_payments() {
        let (mut e, mut p) = two_expenses();
        let m = ExpenseMutation::Edited(ExpenseWithPayments {
            expense: exp(1, "Pizza margherita"),
            payments: vec![pay(99, 1, 7)],
        });
        assert!(apply_mutation(&mut e, &mut p, &m));
        assert_eq!(e.len(), 2, "an edit must not add a row");
        assert_eq!(e.iter().filter(|x| x.id == 1).count(), 1);
        assert_eq!(p.iter().filter(|x| x.expense_id == 1).count(), 1, "old rows must be gone");
        assert_eq!(p.iter().filter(|x| x.expense_id == 2).count(), 1, "other expenses untouched");
    }

    #[test]
    fn a_delete_takes_the_payments_with_it() {
        let (mut e, mut p) = two_expenses();
        assert!(apply_mutation(&mut e, &mut p, &ExpenseMutation::Deleted(1)));
        assert_eq!(e.len(), 1);
        assert!(e.iter().all(|x| x.id != 1));
        assert!(p.iter().all(|x| x.expense_id != 1), "the cascade must be mirrored locally");
        assert_eq!(p.len(), 1);
    }

    /// Applied locally, not refetched: an offline refetch falls back to the pre-delete cache and
    /// used to make the row reappear.
    #[test]
    fn a_delete_applies_without_needing_the_server() {
        let (mut e, mut p) = two_expenses();
        assert!(
            apply_mutation(&mut e, &mut p, &ExpenseMutation::Deleted(2)),
            "a delete never needs a refetch"
        );
        assert_eq!(e.len(), 1);
    }

    #[test]
    fn a_queued_op_asks_for_a_refetch_and_changes_nothing() {
        let (mut e, mut p) = two_expenses();
        assert!(!apply_mutation(&mut e, &mut p, &ExpenseMutation::Queued));
        assert_eq!(e.len(), 2);
        assert_eq!(p.len(), 3);
    }

    /// The lists are out of step with the server; inventing the row would hide that.
    #[test]
    fn editing_a_row_this_device_never_had_asks_for_a_refetch() {
        let (mut e, mut p) = two_expenses();
        let m = ExpenseMutation::Edited(ExpenseWithPayments {
            expense: exp(404, "Fantôme"),
            payments: vec![pay(9, 404, 7)],
        });
        assert!(!apply_mutation(&mut e, &mut p, &m));
        assert_eq!(e.len(), 2, "and nothing may be added on the way out");
        assert_eq!(p.len(), 3);
    }

    /// The point of the patch: visible without a refetch, carrying the *new* name and emoji.
    #[test]
    fn a_patched_edit_is_visible_in_the_next_pass_with_its_new_emoji() {
        let k = test_key();
        let mut e = vec![exp(1, "Taxi")];
        let mut p = vec![pay(1, 1, 7)];

        let before = project_data::build(Some(k), Some(&live_of(e.clone(), p.clone())));
        assert_eq!(before.expenses[0].emoji, "🚕");

        let m = ExpenseMutation::Edited(ExpenseWithPayments {
            expense: exp(1, "Pizza"),
            payments: vec![pay(2, 1, 7)],
        });
        assert!(apply_mutation(&mut e, &mut p, &m));

        let after = project_data::build(Some(k), Some(&live_of(e.clone(), p.clone())));
        assert_eq!(after.expenses[0].expense.name, "Pizza");
        assert_eq!(after.expenses[0].emoji, "🍕", "the emoji must follow the rename");
    }

    #[test]
    fn days_run_newest_first_and_rows_newest_first_within_a_day() {
        let data = built();
        let groups = group_expenses(&data, ExpenseFilter::All, Some(1));
        for w in groups.windows(2) {
            assert!(w[0].0 > w[1].0, "days must run newest first");
        }
        for (_, group) in &groups {
            for w in group.windows(2) {
                assert!(w[0].expense.id > w[1].expense.id, "rows must run newest first");
            }
        }
    }
}

/// Translation keys, not labels: the tests below assert on these without a Dioxus runtime, and
/// the render site translates.
fn expense_type_label(t: &ExpenseType) -> &'static str {
    match t {
        ExpenseType::Expense => "expense-type-expense",
        ExpenseType::Transfer => "expense-type-transfer",
        ExpenseType::Gain => "expense-type-gain",
    }
}

fn payor_connector(t: &ExpenseType) -> &'static str {
    match t {
        ExpenseType::Expense => "expense-paid-by",
        ExpenseType::Transfer => "expense-sent-by",
        ExpenseType::Gain => "expense-contributed-by",
    }
}

#[derive(Clone, Copy, PartialEq)]
enum ExpenseFilter {
    All,
    MyPayments,
    MyDebts,
}

/// Translation keys, not labels — the render site translates.
fn expense_filter_label(f: ExpenseFilter) -> &'static str {
    match f {
        ExpenseFilter::All => "filter-all",
        ExpenseFilter::MyPayments => "filter-my-payments",
        ExpenseFilter::MyDebts => "filter-my-debts",
    }
}

/// `None` = no user filtering. `Some(is_debt)` = keep expenses where the stored user has a
/// payment row with that `is_debt` value.
fn wants_debt(f: ExpenseFilter) -> Option<bool> {
    match f {
        ExpenseFilter::All => None,
        ExpenseFilter::MyPayments => Some(false),
        ExpenseFilter::MyDebts => Some(true),
    }
}

/// Expenses whose payments no longer add up. Grouped by `expense_id` first, so an expense is only
/// ever judged on its own rows.
pub(crate) fn inconsistent_expense_ids(
    expenses: &[DecryptedExpense],
    payments: &[DecryptedPayment],
) -> HashSet<i32> {
    let mut by_expense: HashMap<i32, Vec<DecryptedPayment>> = HashMap::new();
    for dp in payments {
        by_expense.entry(dp.expense_id).or_default().push(dp.clone());
    }
    expenses
        .iter()
        .filter(|e| {
            payments_are_inconsistent(e.amount, by_expense.get(&e.id).map_or(&[][..], |v| v))
        })
        .map(|e| e.id)
        .collect()
}

/// What `user_id` owes per expense; expenses they owe nothing on are absent.
///
/// Summed, not first-taken: an inconsistent expense can carry several debt rows, and the row must
/// show the whole claim — the ⚠ badge says the numbers are wrong, this must not hide half of one.
pub(crate) fn my_debt_by_expense(
    payments: &[DecryptedPayment],
    user_id: i32,
) -> HashMap<i32, f64> {
    let mut by_expense: HashMap<i32, f64> = HashMap::new();
    for p in payments.iter().filter(|p| p.user_id == user_id && p.is_debt) {
        *by_expense.entry(p.expense_id).or_default() += p.amount;
    }
    by_expense
}

/// Date descending, each day by id descending. Reads the page's single decryption pass and the
/// date already parsed on [`RowExpense`], so a filter tap costs no crypto and no parsing.
fn group_expenses(
    data: &ProjectData,
    filter: ExpenseFilter,
    stored_user_id: Option<i32>,
) -> Vec<(chrono::NaiveDate, Vec<RowExpense>)> {
    let filtered: Vec<&RowExpense> = match (wants_debt(filter), stored_user_id) {
        (Some(want_debt), Some(uid)) => {
            let ids: HashSet<i32> = data
                .payments
                .iter()
                .filter(|p| p.user_id == uid && p.is_debt == want_debt)
                .map(|p| p.expense_id)
                .collect();
            data.expenses.iter().filter(|r| ids.contains(&r.expense.id)).collect()
        }
        _ => data.expenses.iter().collect(),
    };

    let mut map: BTreeMap<chrono::NaiveDate, Vec<RowExpense>> = BTreeMap::new();
    for r in filtered {
        map.entry(r.date).or_default().push(r.clone());
    }
    let mut groups: Vec<_> = map.into_iter().collect();
    groups.sort_by_key(|(date, _)| std::cmp::Reverse(*date));
    for (_, group) in &mut groups {
        group.sort_by_key(|r| std::cmp::Reverse(r.expense.id));
    }
    groups
}

/// The day-groups covering the first `limit` rows, plus how many rows that came to. A day is never
/// split across the boundary: the divider and its rows arrive together or not at all.
fn take_rows(
    groups: &[(chrono::NaiveDate, Vec<RowExpense>)],
    limit: usize,
) -> (Vec<(chrono::NaiveDate, Vec<RowExpense>)>, usize) {
    let mut out = Vec::new();
    let mut taken = 0;
    for (date, group) in groups {
        if taken >= limit {
            break;
        }
        taken += group.len();
        out.push((*date, group.clone()));
    }
    (out, taken)
}

/// Initial rows, and rows added per scroll to the bottom. At ~22 DOM nodes and 6 listeners per
/// row, rendering 2000 at once is the whole cost of opening the tab — see
/// `docs/plans/expenses-tab-performance.md`.
const PAGE: usize = 100;

/// Carries enough for the parent to patch its lists in place. A bare "something changed" plus a
/// refetch cost two full project downloads (~2 MB at 2000 expenses) to move one row.
#[derive(Clone, PartialEq)]
pub enum ExpenseMutation {
    Added(ExpenseWithPayments),
    Edited(ExpenseWithPayments),
    Deleted(i32),
    /// Queued while offline: no server row to patch with, so the parent refetches when it can.
    /// The op replays on reconnect and the version bump covers it.
    Queued,
}

/// False when the caller must refetch instead. A queued delete still applies: the row is gone for
/// this device and the op replays on reconnect — refetching offline falls back to the pre-delete
/// cache and used to make the row come *back*.
pub(crate) fn apply_mutation(
    expenses: &mut Vec<Expense>,
    payments: &mut Vec<Payment>,
    mutation: &ExpenseMutation,
) -> bool {
    match mutation {
        ExpenseMutation::Added(c) => {
            // Idempotent by id: a duplicate `Added` (resubmit race, replayed queue entry) would
            // render two rows with the same `key:`, corrupting Dioxus's diffing.
            expenses.retain(|e| e.id != c.expense.id);
            payments.retain(|p| p.expense_id != c.expense.id);
            expenses.push(c.expense.clone());
            payments.extend(c.payments.iter().cloned());
        }
        ExpenseMutation::Edited(c) => {
            let id = c.expense.id;
            match expenses.iter_mut().find(|e| e.id == id) {
                Some(slot) => *slot = c.expense.clone(),
                // Not a row to invent: the lists are out of step and only a refetch settles it.
                None => return false,
            }
            // The server deleted and re-inserted this expense's payments, so the old ones are gone
            // whatever their ids were.
            payments.retain(|p| p.expense_id != id);
            payments.extend(c.payments.iter().cloned());
        }
        ExpenseMutation::Deleted(id) => {
            expenses.retain(|e| e.id != *id);
            payments.retain(|p| p.expense_id != *id);
        }
        ExpenseMutation::Queued => return false,
    }
    true
}

#[derive(PartialEq, Props, Clone)]
pub struct ExpensesTabProps {
    /// The page's single decryption pass. A signal, so passing it costs an id comparison, not a
    /// deep compare of every encrypted row.
    pub data: ReadSignal<Arc<ProjectData>>,
    /// The still-encrypted rows, read only when the edit modal is actually open. Carries the
    /// project it belongs to; see [`LiveData`].
    pub live: ReadSignal<Option<LiveData>>,
    pub stored_user_id: Option<i32>,
    pub project_id: Uuid,
    /// Fired after any create, edit or delete, carrying the rows the server wrote.
    pub on_expenses_changed: EventHandler<ExpenseMutation>,
}

#[component]
pub fn ExpensesTab(props: ExpensesTabProps) -> Element {
    let key_ctx = use_context::<ProjectKey>().0;
    let nav = use_navigator();
    let is_online = use_context::<Signal<bool>>();
    let pending_ops = use_context::<Signal<VecDeque<QueuedOp>>>();
    let flash = use_context::<Signal<Option<Flash>>>();
    let mut filter = use_signal(|| ExpenseFilter::All);
    let mut show_add_expense = use_signal(|| false);
    let mut scan = use_receipt_scan(show_add_expense);
    let mut scan_menu = use_signal(|| false);
    // Delete carries the name so the confirmation can quote it — the expense may be gone by then.
    let mut confirming_delete: Signal<Option<(i32, String)>> = use_signal(|| None);
    let mut editing: Signal<Option<i32>> = use_signal(|| None);
    // How many rows are currently rendered. Grows as the sentinel scrolls into view.
    let mut shown = use_signal(|| PAGE);

    // An Arc clone, not a deep one: nothing below decrypts.
    let data = props.data.read().clone();
    let currency = data.currency.clone();
    let project_status = data.project_status.clone();

    // Only the debt filter shows the second amount.
    let my_debts: HashMap<i32, f64> = match (filter(), props.stored_user_id) {
        (ExpenseFilter::MyDebts, Some(uid)) => my_debt_by_expense(&data.payments, uid),
        _ => HashMap::new(),
    };

    let all_groups = group_expenses(&data, filter(), props.stored_user_id);
    let total: usize = all_groups.iter().map(|(_, g)| g.len()).sum();
    let (groups, rendered) = take_rows(&all_groups, shown());

    let is_empty = all_groups.is_empty();
    let project_id = props.project_id;

    // Every filter chip resets the window — carrying a scrolled offset into a different list
    // would open it part-way down.
    let mut set_filter = move |f: ExpenseFilter| {
        filter.set(f);
        shown.set(PAGE);
    };

    rsx! {
        div { class: "flex flex-col gap-2",
            // Chips stay on one row; longer locales scroll, never wrap.
            //
            // `text-sm!`: these are radios, so `main.css`'s unlayered iOS-zoom guard
            // `input { font-size: 16px !important }` outranks daisyUI's `btn-sm`. Tailwind's
            // `!important` lives in `@layer utilities` and layer order reverses for important
            // declarations, so the layered one wins. Radios never focus-auto-zoom anyway.
            //
            // The padding/negative-margin pair stops the chips looking sliced off: `overflow-x-auto`
            // computes `overflow-y` to `auto` too, so the scroller clips `main.css`'s three-stop
            // `.filter .btn` elevation (~18px below a chip, ~6px to the sides). The padding gives
            // that room back, the negative margins take it out of the layout again.
            div { class: "filter flex-nowrap overflow-x-auto px-2 -mx-2 pb-5 -mb-4",
                input {
                    class: "btn btn-sm whitespace-nowrap text-sm! filter-reset",
                    r#type: "radio",
                    name: "expense-filter",
                    aria_label: tid!(expense_filter_label(ExpenseFilter::All)),
                    checked: filter() == ExpenseFilter::All,
                    onchange: move |_| set_filter(ExpenseFilter::All),
                }
                input {
                    class: "btn btn-sm whitespace-nowrap text-sm!",
                    r#type: "radio",
                    name: "expense-filter",
                    aria_label: tid!(expense_filter_label(ExpenseFilter::MyPayments)),
                    checked: filter() == ExpenseFilter::MyPayments,
                    onchange: move |_| set_filter(ExpenseFilter::MyPayments),
                }
                input {
                    class: "btn btn-sm whitespace-nowrap text-sm!",
                    r#type: "radio",
                    name: "expense-filter",
                    aria_label: tid!(expense_filter_label(ExpenseFilter::MyDebts)),
                    checked: filter() == ExpenseFilter::MyDebts,
                    onchange: move |_| set_filter(ExpenseFilter::MyDebts),
                }
            }

            if is_empty {
                div { class: "flex flex-col items-center gap-2 py-12 text-base-content/70",
                    EmptyMagnifyingGlassIllustration {}
                    span { class: "font-semibold", {tid!("expenses-empty")} }
                    span { class: "text-sm text-center",
                        {tid!("expenses-empty-hint")}
                    }
                }
            } else {
                for (date , group) in groups {
                    div { key: "{date}", class: "flex flex-col",
                        div { class: "divider divider-start text-xs text-base-content/70 font-medium my-1",
                            "{format_date(date)}"
                        }
                        ul { class: "flex flex-col gap-1",
                            for row in group {
                                {
                                    let expense_id = row.expense.id;
                                    let name = row.expense.name.clone();
                                    let etype = row.expense.expense_type.clone();
                                    let payor = data.payor_label(expense_id);
                                    let subtitle = format!(
                                        "{} {} {payor}",
                                        tid!(expense_type_label(&etype)),
                                        tid!(payor_connector(&etype)),
                                    );
                                    let delete_name = name.clone();
                                    let my_debt = my_debts.get(&expense_id).copied();
                                    rsx! {
                                        ExpenseRow {
                                            key: "{expense_id}",
                                            expense_id,
                                            name: name.clone(),
                                            subtitle,
                                            emoji: row.emoji.to_string(),
                                            amount: row.expense.amount,
                                            currency: currency.clone(),
                                            source: row.expense.conversion().map(|(a, c, _)| (a, c.to_string())),
                                            inconsistent: data.inconsistent.contains(&expense_id),
                                            my_debt,
                                            can_edit: can_edit_expense(&project_status, &etype),
                                            can_delete: can_delete_expense(&project_status),
                                            on_open: move |_| {
                                                nav.push(Route::PaymentPage {
                                                    project_id,
                                                    expense_id,
                                                });
                                            },
                                            on_edit: move |_| editing.set(Some(expense_id)),
                                            on_delete: move |_| {
                                                confirming_delete.set(Some((expense_id, delete_name.clone())));
                                            },
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                // `onvisible` is dioxus-html's IntersectionObserver event, implemented in the
                // shared interpreter: one path for web and wry, and it never touches
                // `new Function`, which the nginx CSP blocks (DOCUMENTATION §8). No rootMargin, so
                // the sentinel's own height buys the pre-fetch margin.
                //
                // The button is not dead: `onvisible` cannot fire when the list already fits the
                // viewport, and a keyboard user never scrolls the sentinel into view.
                if rendered < total {
                    div {
                        class: "h-24 flex flex-col items-center justify-center gap-2",
                        onvisible: move |e| {
                            // It fires on the way out too.
                            if e.data().is_intersecting().unwrap_or(false) {
                                shown += PAGE;
                            }
                        },
                        button {
                            class: "btn btn-ghost btn-sm",
                            onclick: move |_| shown += PAGE,
                            {tid!("expenses-show-more", count: (total - rendered) as i64)}
                        }
                    }
                }
            }

            if project_status != ProjectStatus::Archived {
                // Same offsets as `SpeedDialFab` — `safe-bottom-fab` is what clears the bottom dock
                // (and the OS navigation bar on mobile, where the class adds `var(--sab)`).
                // One FAB shell, not two buttons in a row: the gradient, the shadow and the pill
                // live here, and each action is a transparent target inside it. With no scanner
                // the shell collapses back to a single circle, so web and desktop keep exactly
                // the FAB they had — no second branch to hold in sync.
                //
                // The scan half opens a two-action dial, same shape as `SpeedDialFab`: wry offers
                // no single file input that reaches both the camera and the gallery on Android
                // (`ScanSource`), so the choice is made here. The scrim sits *outside* the
                // translated shell — `fixed` inside a `transform` is positioned against the
                // shell, not the viewport.
                if scan_menu() {
                    div {
                        class: "fixed inset-0 z-30 bg-base-content/10",
                        onclick: move |e| {
                            e.stop_propagation();
                            scan_menu.set(false);
                        },
                    }
                }
                div { class: "fixed safe-bottom-fab left-1/2 -translate-x-1/2 z-40 flex flex-col items-end gap-3",
                    if scan_menu() {
                        div {
                            class: "flex flex-col items-end gap-3",
                            onclick: move |e| {
                                e.stop_propagation();
                                scan_menu.set(false);
                            },
                            SpeedDialAction {
                                id: "scan-camera-btn",
                                label: tid!("scan-take-photo"),
                                icon: rsx! { CameraIcon { size: ICON_INLINE } },
                                onclick: move |_| scan.start.call(ScanSource::Camera),
                            }
                            SpeedDialAction {
                                id: "scan-library-btn",
                                label: tid!("scan-choose-photo"),
                                icon: rsx! { PhotoIcon { size: ICON_INLINE } },
                                onclick: move |_| scan.start.call(ScanSource::Library),
                            }
                        }
                    }
                    div { class: "self-center flex items-center rounded-full text-primary-content bg-gradient-brand shadow-soft",
                        // Mobile only: nothing installs a scanner on web, desktop or the server
                        // binary, so `scanner_available()` is false there and this never renders.
                        if scan.available() {
                            button {
                                id: "scan-expense-btn",
                                r#type: "button",
                                // `bg-transparent` over daisyUI's `--btn-bg` so the shell's gradient
                                // shows through, and a white wash for press feedback — `brightness-*`
                                // would tint the whole shell, not the half being touched.
                                class: "btn btn-circle btn-lg border-0 bg-transparent shadow-none text-primary-content [&>svg]:size-6 hover:bg-white/15 active:bg-white/25 disabled:bg-transparent disabled:text-primary-content",
                                "aria-label": if scan.busy() { tid!("scan-in-progress") } else { tid!("expense-scan") },
                                "aria-expanded": scan_menu(),
                                disabled: scan.busy(),
                                onclick: move |e| {
                                    e.stop_propagation();
                                    haptic(Haptic::Light);
                                    scan_menu.set(!scan_menu());
                                },
                                if scan.busy() {
                                    span { class: "loading loading-spinner loading-md" }
                                } else if scan_menu() {
                                    CloseIcon { size: ICON_INLINE }
                                } else {
                                    CameraIcon { size: ICON_INLINE }
                                }
                            }
                            // Two targets in one pill read as one button without it.
                            div { class: "w-px h-6 bg-primary-content/25" }
                        }
                        button {
                            id: "add-expense-btn",
                            r#type: "button",
                            class: "btn btn-circle btn-lg border-0 bg-transparent shadow-none text-primary-content [&>svg]:size-6 hover:bg-white/15 active:bg-white/25",
                            "aria-label": tid!("expense-add"),
                            onclick: move |_| {
                                show_add_expense.set(true);
                            },
                            PlusIcon { size: ICON_INLINE }
                        }
                    }
                }
            }

            if let Some(key) = scan.error.read().as_ref() {
                Toast {
                    msg: tid!(*key),
                    onclose: move |_| scan.error.set(None),
                }
            }

            if show_add_expense() {
                // Cleared on both exits, so the next plain-FAB open starts empty.
                AddExpenseModal {
                    on_close: move |_| {
                        show_add_expense.set(false);
                        scan.prefill.set(None);
                    },
                    on_created: move |created: Option<ExpenseWithPayments>| {
                        show_add_expense.set(false);
                        scan.prefill.set(None);
                        props.on_expenses_changed.call(match created {
                            Some(c) => ExpenseMutation::Added(c),
                            None => ExpenseMutation::Queued,
                        });
                    },
                    project_id: props.project_id,
                    users: data.users.clone(),
                    stored_user_id: props.stored_user_id,
                    currency: currency.clone(),
                    initial_name: scan.prefill.read().as_ref().and_then(|p| p.name.clone()),
                    initial_amount: scan.prefill.read().as_ref().and_then(|p| p.amount),
                    initial_date: scan.prefill.read().as_ref().and_then(|p| p.date.clone()),
                    initial_category: scan.prefill.read().as_ref().and_then(|p| p.category.clone()),
                    amount_hint: scan.prefill.read().as_ref().and_then(|p| p.hint),
                }
            }

            // Swipe-right shortcut. The modal wants the *encrypted* expense and only its own
            // payments.
            if let Some(edit_id) = editing() {
                // Ciphertext read here and nowhere else: cloning both lists per render, for a
                // usually-closed modal, was ~5.6 MB of string copying on a large project. The
                // project filter is the usual guard — this component is reused across projects.
                if let Some((exp, pays)) = props.live.read().as_ref()
                    .filter(|l| l.project_id == props.project_id)
                    .and_then(|l| {
                        let e = l.expenses.iter().find(|e| e.id == edit_id).cloned()?;
                        let p = l.payments.iter().filter(|p| p.expense_id == edit_id).cloned().collect();
                        Some((e, p))
                    })
                {
                    EditExpenseModal {
                        on_close: move |_| editing.set(None),
                        on_edited: move |edited: Option<ExpenseWithPayments>| {
                            editing.set(None);
                            props.on_expenses_changed.call(match edited {
                                Some(e) => ExpenseMutation::Edited(e),
                                None => ExpenseMutation::Queued,
                            });
                        },
                        expense: exp,
                        payments: pays,
                        users: data.users.clone(),
                        project_id: props.project_id,
                        stored_user_id: props.stored_user_id,
                        currency: currency.clone(),
                    }
                }
            }

            // Swipe-left shortcut. Always confirmed: the delete cascades to the payments, cannot
            // be undone, and the gesture is cheap enough to trigger by accident.
            if let Some((del_id, del_name)) = confirming_delete() {
                ConfirmModal {
                    title: tid!("expense-delete-title"),
                    message: tid!("expense-delete-message", name: del_name.clone()),
                    confirm_label: tid!("delete"),
                    on_cancel: move |_| confirming_delete.set(None),
                    on_confirm: move |_| {
                        confirming_delete.set(None);
                        let Some(k) = key_ctx() else { return };
                        let req = delete_expense_request(
                            &k,
                            del_id,
                            props.project_id,
                            tid!("history-expense-deleted", name: del_name.clone()),
                            props.stored_user_id.unwrap_or(0),
                        );
                        let on_changed = props.on_expenses_changed;
                        // A queued delete still removes the row locally: the expense is gone as far
                        // as this device is concerned, and the op replays on reconnect.
                        run_delete_expense(
                            req,
                            del_name.clone(),
                            is_online(),
                            pending_ops,
                            flash,
                            move || on_changed.call(ExpenseMutation::Deleted(del_id)),
                        );
                    },
                }
            }
        }
    }
}
