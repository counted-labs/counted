use dioxus::prelude::*;
use crate::tid;
use shared::{Expense, ExpenseType, ExpenseWithPayments, Payment, ProjectStatus};
use std::collections::{BTreeMap, HashMap, HashSet, VecDeque};
use std::sync::Arc;
use uuid::Uuid;

use crate::common::{
    format_date, haptic, is_pending_expense, Flash, Haptic, Mascot, MascotPose, ProjectKey,
    QueuedOp, ScanSource, SpeedDialAction, Toast,
};
use crate::decrypted::{payments_are_inconsistent, pickable_users, DecryptedExpense, DecryptedPayment};
use crate::expenses::helpers::delete_expense_action::{
    can_delete_expense, can_edit_expense, delete_expense_request, run_delete_expense,
};
use crate::categories::{ProjectCategories, ShownCategory};
use crate::expenses::expense_search::{ExpenseSearchField, SearchToggle};
use crate::expenses::move_expense::finish_move;
use crate::expenses::helpers::expense_query::{mark_segments_hit, ExpenseFilter, ExpenseQuery};
use crate::expenses::budget_panel::{sheet_categories, BudgetPanel, BudgetSheet};
use crate::expenses::helpers::budgets::{average_by_category, budget_lines, crossed, spent_by_category, Meter};
use crate::expenses::helpers::my_spending::{
    first_of_month, in_month, month_totals, results_total, share_rows, GroupLedger, ShareRow,
};
use crate::expenses::helpers::project_data::{LiveData, ProjectData, RowExpense};
use crate::expenses::hooks::use_fx_rates::use_fx_rates;
use crate::expenses::hooks::use_receipt_scan::use_receipt_scan;
use crate::expenses::my_spending::{MonthSummary, SearchPill};
use crate::expenses::project_states::ExpenseRowsSkeleton;
use crate::preferences::uncounted_projects;
use crate::expenses::{AddExpenseModal, EditExpenseModal, ExpenseRow};
use crate::icons::{CameraIcon, CloseIcon, PhotoIcon, PlusIcon, ICON_INLINE};
use crate::expenses::hooks::use_project_store::ProjectStore;
use crate::recurring::actions::stop_rule;
use crate::recurring::dialog::DeleteExpenseDialog;
use crate::recurring::recurring_page::RecurringStrip;
use crate::recurring::store::live_rule_of;
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
            recurring_id: None,
            estimate: false,
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
        // Nothing decrypted for it - a missing key, not a broken expense.
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
        // The credit row is mine and sits on the same expense - it must not be counted.
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
        // Absent, not zero - the row then renders no second line at all.
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
            None,
        )
    }

    fn scope(scope: ExpenseFilter) -> ExpenseQuery {
        ExpenseQuery { scope, ..Default::default() }
    }

    /// Everything the body does below the decryption pass, as the component does it.
    fn render(data: &ProjectData, filter: ExpenseFilter, uid: Option<i32>) -> usize {
        let groups = group_expenses(data, &scope(filter), uid);
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

    /// A filter tap re-runs the whole body and must open no ciphertext - it used to cost a full
    /// 6 010-op pass every time.
    #[test]
    fn a_filter_tap_decrypts_nothing() {
        let data = built();
        for filter in [ExpenseFilter::All, ExpenseFilter::MyPayments, ExpenseFilter::MyDebts] {
            let (_, n) = count_decrypts(|| render(&data, filter, Some(1)));
            assert_eq!(
                n, 0,
                "filter {:?} re-decrypted - the pass belongs above the body, not in it",
                expense_filter_label(filter)
            );
        }
    }

    /// The window only decides how much of an already-decrypted list reaches the DOM.
    #[test]
    fn growing_the_window_decrypts_nothing() {
        let data = built();
        let groups = group_expenses(&data, &ExpenseQuery::default(), Some(1));
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
        let groups = group_expenses(&data, &ExpenseQuery::default(), Some(1));
        let total: usize = groups.iter().map(|(_, g)| g.len()).sum();
        assert_eq!(total, 1_000, "the fixture is 1 000 expenses");

        let (_, rendered) = take_rows(&groups, PAGE);
        assert!(rendered >= PAGE, "a page must be filled");
        assert!(rendered < total, "and it must not be the whole project");
    }

    /// A day is never split across the boundary - a divider with half its rows under it would read
    /// as data loss.
    #[test]
    fn a_day_is_never_split_across_the_window_edge() {
        let data = built();
        let groups = group_expenses(&data, &ExpenseQuery::default(), Some(1));
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
        let groups = group_expenses(&data, &ExpenseQuery::default(), Some(1));
        let (page, rendered) = take_rows(&groups, 10_000);
        assert_eq!(rendered, 1_000);
        assert_eq!(page.len(), groups.len());
    }

    #[test]
    fn an_empty_list_yields_no_groups_and_no_rows() {
        let data = ProjectData::default();
        let groups = group_expenses(&data, &ExpenseQuery::default(), Some(1));
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

    // A save that resolves after a switch to another project must not land in that project's list.
    #[test]
    fn a_mutation_belongs_to_its_own_project_only() {
        let mut expense = exp(3, "Bière");
        let mine = uuid::Uuid::new_v4();
        expense.project_id = mine;
        let m = ExpenseMutation::Added(ExpenseWithPayments { expense, payments: vec![] });
        assert!(m.belongs_to(mine));
        assert!(!m.belongs_to(uuid::Uuid::new_v4()));
        assert!(ExpenseMutation::Deleted(3).belongs_to(uuid::Uuid::new_v4()));
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

        let before = project_data::build(Some(k), Some(&live_of(e.clone(), p.clone())), None);
        assert_eq!(before.expenses[0].emoji, "🚕");

        let m = ExpenseMutation::Edited(ExpenseWithPayments {
            expense: exp(1, "Pizza"),
            payments: vec![pay(2, 1, 7)],
        });
        assert!(apply_mutation(&mut e, &mut p, &m));

        let after = project_data::build(Some(k), Some(&live_of(e.clone(), p.clone())), None);
        assert_eq!(after.expenses[0].expense.name, "Pizza");
        assert_eq!(after.expenses[0].emoji, "🍕", "the emoji must follow the rename");
    }

    #[test]
    fn days_run_newest_first_and_rows_newest_first_within_a_day() {
        let data = built();
        let groups = group_expenses(&data, &ExpenseQuery::default(), Some(1));
        for w in groups.windows(2) {
            assert!(w[0].0 > w[1].0, "days must run newest first");
        }
        for (_, group) in &groups {
            for w in group.windows(2) {
                assert!(w[0].expense.id > w[1].expense.id, "rows must run newest first");
            }
        }
    }

    #[test]
    fn expenses_queued_offline_run_newest_first_above_the_sent_ones() {
        let live = live_of(vec![exp(4, "a"), exp(-1, "b"), exp(5, "c"), exp(-2, "d")], vec![]);
        let data = project_data::build(Some(test_key()), Some(&live), None);
        let groups = group_expenses(&data, &ExpenseQuery::default(), Some(1));
        let ids: Vec<i32> = groups.iter().flat_map(|(_, g)| g.iter().map(|r| r.expense.id)).collect();
        assert_eq!(ids, vec![-2, -1, 5, 4]);
    }

    #[test]
    fn a_server_id_at_the_i32_bounds_sorts_without_overflow() {
        let live = live_of(vec![exp(i32::MIN, "a"), exp(i32::MAX, "b"), exp(-1, "c")], vec![]);
        let data = project_data::build(Some(test_key()), Some(&live), None);
        let groups = group_expenses(&data, &ExpenseQuery::default(), Some(1));
        let ids: Vec<i32> = groups.iter().flat_map(|(_, g)| g.iter().map(|r| r.expense.id)).collect();
        assert_eq!(ids, vec![i32::MIN, -1, i32::MAX]);
    }

    mod hostile_payloads {
        use crate::common::hostile;
        use proptest::prelude::*;

        proptest! {
            #![proptest_config(ProptestConfig::with_cases(500))]

            #[test]
            fn any_ledger_renders_and_settles(expenses in hostile::expenses(), payments in hostile::payments(), users in hostile::users(), me in hostile::id()) {
                crate::expenses::tabs::expenses_tab::exercise_hostile(expenses, payments, users, me);
            }

            #[test]
            fn any_plaintext_under_the_key_is_refused_or_read(plain in any::<String>()) {
                crate::decrypted::decode_hostile(&plain);
            }
        }
    }
}

/// Any key holder writes the plaintext every member's page decodes and sums: whatever it holds,
/// the page renders, a panic freezing the wasm app for everyone in the project.
#[cfg(any(test, fuzzing))]
pub(crate) fn exercise_hostile(expenses: Vec<Expense>, payments: Vec<Payment>, users: Vec<shared::User>, me: i32) {
    use crate::common::test_fixtures::{make_project, TEST_KEY};
    use crate::expenses::helpers::project_data;

    let live = LiveData { project_id: Uuid::nil(), project: Some(make_project(&TEST_KEY, Uuid::nil(), "p", "EUR")), users, expenses, payments, version: None };
    let data = project_data::build(Some(TEST_KEY), Some(&live), None);
    let _ = group_expenses(&data, &ExpenseQuery::default(), Some(me));
    let _ = inconsistent_expense_ids(&data.expenses.iter().map(|r| r.expense.clone()).collect::<Vec<_>>(), &data.payments);
    let _ = my_debt_by_expense(&data.payments, me);
    assert!(data.global_total.is_finite());
    assert!(data.summary.summary.values().all(|b| b.is_finite()));
    assert!(data.summary.reimbursement_suggestions.iter().all(|s| s.amount.is_finite()));
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

/// Translation keys, not labels - the render site translates.
fn expense_filter_label(f: ExpenseFilter) -> &'static str {
    match f {
        ExpenseFilter::All => "filter-all",
        ExpenseFilter::MyPayments => "filter-my-payments",
        ExpenseFilter::MyDebts => "filter-my-debts",
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
/// show the whole claim - the ⚠ badge says the numbers are wrong, this must not hide half of one.
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

type Groups = Vec<(chrono::NaiveDate, Vec<RowExpense>)>;

/// Date descending, each day by id descending. Reads the page's single decryption pass and the
/// date already parsed on [`RowExpense`], so a filter tap costs no crypto and no parsing.
#[cfg(any(test, fuzzing))]
fn group_expenses(data: &ProjectData, query: &ExpenseQuery, stored_user_id: Option<i32>) -> Groups {
    group_rows(query.select(data, stored_user_id, ShownCategory::label))
}

fn group_rows(rows: Vec<&RowExpense>) -> Groups {
    let mut map: BTreeMap<chrono::NaiveDate, Vec<RowExpense>> = BTreeMap::new();
    for r in rows {
        map.entry(r.date).or_default().push(r.clone());
    }
    let mut groups: Groups = map.into_iter().rev().collect();
    for (_, group) in &mut groups {
        group.sort_by_key(|r| std::cmp::Reverse((is_pending_expense(r.expense.id), r.expense.id.unsigned_abs())));
    }
    groups
}

/// The day-groups covering the first `limit` rows, plus how many rows that came to. A day is never
/// split across the boundary: the divider and its rows arrive together or not at all.
fn take_rows(groups: &[(chrono::NaiveDate, Vec<RowExpense>)], limit: usize) -> (Groups, usize) {
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
/// row, rendering 2000 at once is the whole cost of opening the tab - see
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

impl ExpenseMutation {
    /// Whether this may patch `project_id`'s rows. Only a returned row names its project; a bare id
    /// matches nothing elsewhere, expense ids being unique across projects.
    pub(crate) fn belongs_to(&self, project_id: uuid::Uuid) -> bool {
        match self {
            ExpenseMutation::Added(c) | ExpenseMutation::Edited(c) => {
                c.expense.project_id == project_id
            }
            ExpenseMutation::Deleted(_) | ExpenseMutation::Queued => true,
        }
    }
}

/// False when the caller must refetch instead. A queued delete still applies: the row is gone for
/// this device and the op replays on reconnect - refetching offline falls back to the pre-delete
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

/// What an expense added on this page is measured against, as the page showed it.
#[derive(Clone)]
struct BudgetWatch {
    on: bool,
    month: chrono::NaiveDate,
    lines: Vec<crate::expenses::helpers::budgets::BudgetLine>,
    categories: ProjectCategories,
}

/// "Me prévenir à 80 %": a personal expense that takes its category past 80 % or past the ceiling
/// says so at once, on this device.
fn warn_on_budget(key: &[u8; 32], created: &ExpenseWithPayments, watch: &BudgetWatch, mut flash: Signal<Option<Flash>>) {
    if !watch.on {
        return;
    }
    let Ok(expense) = crate::decrypted::decrypt_expense(key, &created.expense) else { return };
    let row = RowExpense::new(expense, &watch.categories);
    if row.expense.expense_type != ExpenseType::Expense || !in_month(row.date, watch.month) {
        return;
    }
    let watched = [row.category.id.as_str(), row.category.parent];
    let crossing = watched.iter().find_map(|id| {
        let line = watch.lines.iter().find(|l| l.category.id == *id)?;
        Some((line, crossed(line.spent, row.expense.amount, line.limit)?))
    });
    let Some((line, meter)) = crossing else { return };
    let label = line.category.label();
    match meter {
        Meter::Near(pct) => flash.set(Some(Flash::ok(tid!("budget-alert-near", category: label, percent: pct as i64)))),
        Meter::Over(_) => flash.set(Some(Flash::err(tid!("budget-alert-over", category: label)))),
        Meter::Left(_) => {}
    }
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
    /// The groups whose shares "Mes dépenses" lists; empty on any other project.
    pub ledgers: ReadSignal<Arc<Vec<GroupLedger>>>,
    /// The first `ledgers` load: an empty list then is not an empty month yet.
    pub ledgers_loading: bool,
}

#[component]
pub fn ExpensesTab(props: ExpensesTabProps) -> Element {
    let key_ctx = use_context::<ProjectKey>().0;
    let nav = use_navigator();
    let is_online = use_context::<Signal<bool>>();
    let pending_ops = use_context::<Signal<VecDeque<QueuedOp>>>();
    let flash = use_context::<Signal<Option<Flash>>>();
    let store = use_context::<ProjectStore>();
    let mut query = store.query;
    // Set by the toggle only, so the field is not focused again when the tab remounts.
    let mut focus_search = use_signal(|| false);
    let mut show_add_expense = use_signal(|| false);
    let mut scan = use_receipt_scan(show_add_expense);
    let mut scan_menu = use_signal(|| false);
    // Delete carries the name so the confirmation can quote it - the expense may be gone by then.
    let mut confirming_delete: Signal<Option<(i32, String)>> = use_signal(|| None);
    let mut editing: Signal<Option<i32>> = use_signal(|| None);
    // How many rows are currently rendered. Grows as the sentinel scrolls into view.
    let mut shown = use_signal(|| PAGE);

    // An Arc clone, not a deep one: nothing below decrypts.
    let data = props.data.read().clone();
    let currency = data.currency.clone();
    let project_status = data.project_status.clone();
    let read_only = data.read_only;
    let personal = data.personal;
    let month = use_signal(|| first_of_month(crate::common::local_today()));
    let account = use_context::<Signal<Option<shared::Account>>>();
    let account_key = use_context::<Signal<Option<[u8; 32]>>>();
    let needs_fx = use_memo(move || {
        let d = props.data.read();
        d.personal && props.ledgers.read().iter().any(|l| l.data.currency != d.currency)
    });
    let fx = use_fx_rates(needs_fx);
    let uncounted = match (account.read().as_ref(), account_key()) {
        (Some(a), Some(k)) => uncounted_projects(a, &k),
        _ => Default::default(),
    };
    let shares: Vec<ShareRow> = if personal {
        share_rows(&props.ledgers.read(), &uncounted, &currency, fx.read().as_ref())
    } else {
        Vec::new()
    };
    let share_by_id: HashMap<i32, &ShareRow> = shares.iter().map(|s| (s.row.expense.id, s)).collect();
    let mut budget_tab = use_signal(|| false);
    let mut show_budget_sheet = use_signal(|| false);
    let budget_open = personal && budget_tab();
    let spent_by_cat = if personal { spent_by_category(&data.expenses, &shares, month()) } else { HashMap::new() };
    let lines = if personal { budget_lines(&data.budgets, &data.budgets_custom, &data.categories, &spent_by_cat) } else { Vec::new() };
    let budget_watch = BudgetWatch {
        on: personal && data.budgets.alert,
        month: month(),
        lines: lines.clone(),
        categories: data.categories.clone(),
    };

    // Only the debt filter shows the second amount.
    let scope = query.read().scope;
    let my_debts: HashMap<i32, f64> = match (scope, props.stored_user_id) {
        (ExpenseFilter::MyDebts, Some(uid)) => my_debt_by_expense(&data.payments, uid),
        _ => HashMap::new(),
    };

    let searching = query.read().searching;
    let words = query.read().words();
    let q: ExpenseQuery = query.read().clone();
    // "Mes dépenses" shows one month, its own rows and the counted shares; a search runs over every
    // month and every group, counted or not.
    let selected: Vec<&RowExpense> = if personal {
        let shown_month = month();
        let everywhere = !words.is_empty();
        q.search(data.expenses.iter().filter(|r| everywhere || in_month(r.date, shown_month)), ShownCategory::label)
            .into_iter()
            .chain(q.search(
                shares
                    .iter()
                    .filter(|s| everywhere || (s.counted && in_month(s.row.date, shown_month)))
                    .map(|s| &s.row),
                ShownCategory::label,
            ))
            .collect()
    } else {
        q.select(&data, props.stored_user_id, ShownCategory::label)
    };
    let search_summary = (!words.is_empty()).then(|| {
        let count = selected.len() as i64;
        if personal {
            let spent = results_total(&selected, &share_by_id);
            tid!("my-spending-search-summary", count: count, total: crate::common::amount(spent, &currency))
        } else {
            let spent: f64 = selected
                .iter()
                .filter(|r| r.expense.expense_type == ExpenseType::Expense)
                .map(|r| r.expense.amount)
                .sum();
            tid!("expenses-search-summary", count: count, total: crate::common::amount(spent, &currency))
        }
    });
    let totals = month_totals(&data.expenses, &shares, month());
    let all_groups = group_rows(selected);
    let total: usize = all_groups.iter().map(|(_, g)| g.len()).sum();
    let (groups, rendered) = take_rows(&all_groups, shown());

    let is_empty = all_groups.is_empty();
    let project_id = props.project_id;
    let rules = store.recurring_for(project_id);

    // Every query change resets the window - carrying a scrolled offset into a different list
    // would open it part-way down.
    let mut set_filter = move |f: ExpenseFilter| {
        query.write().scope = f;
        shown.set(PAGE);
    };

    rsx! {
        div { class: "flex flex-col gap-2",
            if personal && words.is_empty() {
                MonthSummary {
                    month,
                    totals: totals.clone(),
                    currency: currency.clone(),
                }
            }
            if personal {
                div { role: "tablist", class: "tabs tabs-box bg-base-300 shadow-none grid grid-flow-col auto-cols-fr",
                    button {
                        id: "tab-expenses",
                        role: "tab",
                        aria_selected: !budget_open,
                        class: if budget_open { "tab text-[13px] font-semibold text-base-content/70" } else { "tab tab-active text-[13px] font-semibold" },
                        onclick: move |_| budget_tab.set(false),
                        {tid!("tab-expenses")}
                    }
                    button {
                        id: "tab-budget",
                        role: "tab",
                        aria_selected: budget_open,
                        class: if budget_open { "tab tab-active text-[13px] font-semibold" } else { "tab text-[13px] font-semibold text-base-content/70" },
                        onclick: move |_| {
                            {
                                let mut q = query.write();
                                q.searching = false;
                                q.text.clear();
                            }
                            budget_tab.set(true);
                        },
                        {tid!("tab-budget")}
                    }
                }
            }
            if budget_open {
                BudgetPanel {
                    lines: lines.clone(),
                    currency: currency.clone(),
                    on_manage: move |_| show_budget_sheet.set(true),
                }
            }
            if (!personal || words.is_empty()) && !budget_open {
                RecurringStrip { project_id: props.project_id }
            }
            if personal && !searching && !budget_open {
                SearchPill {
                    onclick: move |_| {
                        query.write().searching = true;
                        focus_search.set(true);
                        shown.set(PAGE);
                    },
                }
            }
            // Chips stay on one row; longer locales scroll, never wrap.
            //
            // `text-sm!`: these are radios, so `main.css`'s unlayered iOS-zoom guard
            // `input { font-size: 16px !important }` outranks daisyUI's `btn-sm`. Tailwind's
            // `!important` lives in `@layer utilities` and layer order reverses for important
            // declarations, so the layered one wins. Radios never focus-auto-zoom anyway.
            //
            // The padding/negative-margin pair stops the chips looking sliced off: `overflow-x-auto`
            // computes `overflow-y` to `auto` too, so the scroller clips `main.css`'s three-stop
            // `shadow-soft` elevation (~18px below a chip, ~6px to the sides, and the top edge of
            // any ring above). The padding gives that room back, the negative margins take it out
            // of the layout again.
            if !personal {
            div { class: "flex items-start gap-2",
            SearchToggle {
                open: searching,
                onclick: move |_| {
                    let open = !query.read().searching;
                    {
                        let mut q = query.write();
                        q.searching = open;
                        q.text.clear();
                    }
                    focus_search.set(open);
                    shown.set(PAGE);
                },
            }
            div { role: "radiogroup", aria_label: tid!("expense-filter-group"), class: "flex flex-nowrap gap-2 overflow-x-auto [scrollbar-width:none] [&::-webkit-scrollbar]:hidden grow min-w-0 px-2 -mx-2 pt-1 -mt-1 pb-5 -mb-4",
                input {
                    class: "btn btn-sm whitespace-nowrap text-sm! font-medium border-0 shadow-soft bg-base-100 checked:bg-secondary checked:text-secondary-content",
                    r#type: "radio",
                    name: "expense-filter",
                    aria_label: tid!(expense_filter_label(ExpenseFilter::All)),
                    checked: scope == ExpenseFilter::All,
                    onchange: move |_| set_filter(ExpenseFilter::All),
                }
                input {
                    class: "btn btn-sm whitespace-nowrap text-sm! font-medium border-0 shadow-soft bg-base-100 checked:bg-secondary checked:text-secondary-content",
                    r#type: "radio",
                    name: "expense-filter",
                    aria_label: tid!(expense_filter_label(ExpenseFilter::MyPayments)),
                    checked: scope == ExpenseFilter::MyPayments,
                    onchange: move |_| set_filter(ExpenseFilter::MyPayments),
                }
                input {
                    class: "btn btn-sm whitespace-nowrap text-sm! font-medium border-0 shadow-soft bg-base-100 checked:bg-secondary checked:text-secondary-content",
                    r#type: "radio",
                    name: "expense-filter",
                    aria_label: tid!(expense_filter_label(ExpenseFilter::MyDebts)),
                    checked: scope == ExpenseFilter::MyDebts,
                    onchange: move |_| set_filter(ExpenseFilter::MyDebts),
                }
            }
            }
            }

            if searching && !budget_open {
                ExpenseSearchField {
                    query,
                    autofocus: focus_search(),
                    on_change: move |_| shown.set(PAGE),
                    closes: personal,
                }
            }

            if let Some(summary) = search_summary.filter(|_| !is_empty) {
                p { id: "expense-search-summary", role: "status", class: "text-[13px] text-base-content/70 px-1", "{summary}" }
            }

            if budget_open {
            } else if is_empty && !words.is_empty() {
                div { class: "flex flex-col items-center gap-3 py-12 text-base-content/70",
                    Mascot { pose: MascotPose::Searching }
                    span { class: "font-semibold", {tid!("expenses-no-match")} }
                    button {
                        id: "expense-no-match-reset",
                        class: "btn btn-sm",
                        onclick: move |_| {
                            query.write().text.clear();
                            shown.set(PAGE);
                        },
                        {tid!("expenses-search-clear")}
                    }
                }
            } else if is_empty && personal && props.ledgers_loading {
                ExpenseRowsSkeleton {}
            } else if is_empty {
                div { class: "flex flex-col items-center gap-2 py-12 text-base-content/70",
                    Mascot { pose: MascotPose::Empty }
                    if personal {
                        span { id: "my-spending-empty", class: "font-semibold", {tid!("my-spending-empty")} }
                        span { class: "text-sm text-center", {tid!("my-spending-empty-hint")} }
                    } else {
                        span { class: "font-semibold", {tid!("expenses-empty")} }
                        span { class: "text-sm text-center",
                            {tid!("expenses-empty-hint")}
                        }
                    }
                }
            } else {
                for (date , group) in groups {
                    div { key: "{date}", class: "flex flex-col",
                        h2 { class: "divider divider-start text-xs text-base-content/70 font-medium my-1",
                            "{format_date(date)}"
                        }
                        ul { class: "flex flex-col gap-2",
                            for row in group {
                                {
                                    let expense_id = row.expense.id;
                                    let name = row.expense.name.clone();
                                    let etype = row.expense.expense_type.clone();
                                    // Shown only when the name says nothing of why the row matched.
                                    let matched_category = (!words.is_empty() && !mark_segments_hit(&name, &words))
                                        .then(|| row.category.label())
                                        .filter(|label| mark_segments_hit(label, &words));
                                    if let Some(share) = share_by_id.get(&expense_id) {
                                        let group_project = share.project_id;
                                        let payer = share.payer.clone().unwrap_or_else(|| tid!("payer-you"));
                                        let (amount, row_currency) = match share.converted {
                                            Some(converted) => (converted, currency.clone()),
                                            None => (share.share, share.currency.clone()),
                                        };
                                        let note = if share.counted {
                                            tid!("my-spending-share-of", amount: crate::common::amount(row.expense.amount, &share.currency))
                                        } else {
                                            tid!("my-spending-uncounted")
                                        };
                                        rsx! {
                                            ExpenseRow {
                                                key: "{expense_id}",
                                                expense_id,
                                                name: name.clone(),
                                                subtitle: format!("{} {payer}", tid!("expense-paid-by")),
                                                group: Some(share.group.clone()),
                                                note: Some(note),
                                                highlight: words.clone(),
                                                matched_category,
                                                emoji: row.emoji.to_string(),
                                                amount,
                                                currency: row_currency,
                                                source: None,
                                                inconsistent: false,
                                                my_debt: None,
                                                estimate: row.expense.estimate,
                                                can_edit: false,
                                                can_delete: false,
                                                on_open: move |_| {
                                                    nav.push(Route::PaymentPage {
                                                        project_id: group_project,
                                                        expense_id,
                                                    });
                                                },
                                                on_edit: move |_| {},
                                                on_delete: move |_| {},
                                            }
                                        }
                                    } else {
                                    let (connector, payor) = if personal {
                                        let connector = if etype == ExpenseType::Gain { tid!("expense-received-by") } else { tid!(payor_connector(&etype)) };
                                        (connector, tid!("payer-you"))
                                    } else {
                                        (tid!(payor_connector(&etype)), data.payor_label(expense_id))
                                    };
                                    let subtitle = format!("{} {connector} {payor}", tid!(expense_type_label(&etype)));
                                    let delete_name = name.clone();
                                    let my_debt = my_debts.get(&expense_id).copied();
                                    let pending = is_pending_expense(expense_id);
                                    rsx! {
                                        ExpenseRow {
                                            key: "{expense_id}",
                                            expense_id,
                                            name: name.clone(),
                                            subtitle,
                                            highlight: words.clone(),
                                            matched_category,
                                            emoji: row.emoji.to_string(),
                                            amount: row.expense.amount,
                                            currency: currency.clone(),
                                            source: row.expense.conversion().map(|(a, c, _)| (a, c.to_string())),
                                            inconsistent: data.inconsistent.contains(&expense_id),
                                            my_debt,
                                            recurring: row.expense.recurring_id.is_some(),
                                            estimate: row.expense.estimate,
                                            income: row.expense.expense_type == ExpenseType::Gain,
                                            pending,
                                            can_edit: !pending && can_edit_expense(&project_status, read_only, &etype),
                                            can_delete: !pending && can_delete_expense(&project_status, read_only),
                                            on_open: move |_| {
                                                if pending {
                                                    return;
                                                }
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

            if !read_only && project_status != ProjectStatus::Archived {
                // Same offsets as `SpeedDialFab` - `safe-bottom-fab` is what clears the bottom dock
                // (and the OS navigation bar on mobile, where the class adds `var(--sab)`).
                // One FAB shell, not two buttons in a row: the gradient, the shadow and the pill
                // live here, and each action is a transparent target inside it. With no scanner
                // the shell collapses back to a single circle, so web and desktop keep exactly
                // the FAB they had - no second branch to hold in sync.
                //
                // The scan half opens a two-action dial, same shape as `SpeedDialFab`: wry offers
                // no single file input that reaches both the camera and the gallery on Android
                // (`ScanSource`), so the choice is made here. The scrim sits *outside* the
                // translated shell - `fixed` inside a `transform` is positioned against the
                // shell, not the viewport.
                if scan_menu() {
                    div {
                        class: "fixed inset-0 z-30 bg-base-content/30",
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
                                // shows through, and a white wash for press feedback - `brightness-*`
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

            if show_budget_sheet() {
                BudgetSheet {
                    project_id: props.project_id,
                    budgets: data.budgets.clone(),
                    custom: data.budgets_custom.clone(),
                    averages: sheet_categories(&average_by_category(&data.expenses, &shares, month()), &data.categories),
                    currency: currency.clone(),
                    on_close: move |_| show_budget_sheet.set(false),
                    on_saved: move |_| {
                        show_budget_sheet.set(false);
                        let mut sync = store.sync;
                        sync.restart();
                    },
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
                        if let (Some(c), Some(k)) = (created.as_ref(), key_ctx()) {
                            warn_on_budget(&k, c, &budget_watch, flash);
                        }
                        props.on_expenses_changed.call(match created {
                            Some(c) => ExpenseMutation::Added(c),
                            None => ExpenseMutation::Queued,
                        });
                    },
                    project_id: props.project_id,
                    users: pickable_users(key_ctx().as_ref(), &data.users, &[]),
                    stored_user_id: props.stored_user_id,
                    currency: currency.clone(),
                    initial_name: scan.prefill.read().as_ref().and_then(|p| p.name.clone()),
                    initial_amount: scan.prefill.read().as_ref().and_then(|p| p.amount),
                    initial_date: scan.prefill.read().as_ref().and_then(|p| p.date.clone()),
                    initial_category: scan.prefill.read().as_ref().and_then(|p| p.category.clone()),
                    amount_hint: scan.prefill.read().as_ref().and_then(|p| p.hint),
                    personal: data.personal,
                }
            }

            // A move started on the source's page lands here, once this project is the one loaded:
            // its own key, participants and currency are what the form needs.
            if let Some(draft) = store.pending_move.read().clone().filter(|d| {
                d.to_project == props.project_id && data.project_id == Some(props.project_id)
            }) {
                AddExpenseModal {
                    on_close: move |_| {
                        let mut pending_move = store.pending_move;
                        pending_move.set(None);
                    },
                    on_created: {
                        let draft = draft.clone();
                        move |created: Option<ExpenseWithPayments>| {
                            props.on_expenses_changed.call(match created {
                                Some(c) => ExpenseMutation::Added(c),
                                None => ExpenseMutation::Queued,
                            });
                            finish_move(&draft, is_online(), pending_ops, flash);
                            let mut pending_move = store.pending_move;
                            pending_move.set(None);
                        }
                    },
                    project_id: props.project_id,
                    users: pickable_users(key_ctx().as_ref(), &data.users, &[]),
                    stored_user_id: props.stored_user_id,
                    currency: currency.clone(),
                    initial_name: Some(draft.payload.name.clone()),
                    initial_amount: Some(draft.amount),
                    initial_currency: Some(draft.currency.clone()),
                    initial_date: Some(draft.payload.date.clone()),
                    initial_category: draft.category_for(&data.categories),
                    initial_expense_type: Some(draft.expense_type()),
                    carried: Some(draft.payload.clone()),
                    personal: draft.to_personal,
                    move_banner: Some(tid!("move-banner", from: draft.from_name.clone(), to: draft.to_name.clone())),
                }
            }

            // Swipe-right shortcut. The modal wants the *encrypted* expense and only its own
            // payments.
            if let Some(edit_id) = editing() {
                // Ciphertext read here and nowhere else: cloning both lists per render, for a
                // usually-closed modal, was ~5.6 MB of string copying on a large project. The
                // project filter is the usual guard - this component is reused across projects.
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
                DeleteExpenseDialog {
                    name: del_name.clone(),
                    date: data.expenses.iter().find(|r| r.expense.id == del_id).map(|r| r.expense.date.clone()).unwrap_or_default(),
                    occurrence: live_rule_of(&data, del_id, rules.as_ref()).is_some(),
                    on_cancel: move |_| confirming_delete.set(None),
                    on_delete: move |stop: bool| {
                        confirming_delete.set(None);
                        let Some(k) = key_ctx() else { return };
                        if stop {
                            if let Some(rule) = live_rule_of(&props.data.read(), del_id, store.recurring_for(project_id).as_ref()) {
                                stop_rule(k, project_id, &rule, props.stored_user_id, flash, move || {
                                    let mut sync = store.sync;
                                    sync.restart();
                                });
                            }
                        }
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
