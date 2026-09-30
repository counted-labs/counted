//! The project page's one and only decryption pass.
//!
//! Every figure the page shows comes out of a single [`build`] call, held in a `use_memo` in
//! `expenses_page.rs`. Nothing below it decrypts. The page used to decrypt everything twice off
//! the same ciphertext — once for the stats, once for the rows. Guard:
//! `decrypt_count_is_linear`.

use shared::{
    balances::get_reimbursement_suggestions, Expense, ExpenseType, Payment, ProjectDto,
    ProjectStatus, User, UserBalance, UserSummary,
};
use std::collections::{HashMap, HashSet};

use crate::categories::{find_category, get_expense_category, parent_emoji};
use crate::decrypted::{
    decrypt_expense, decrypt_payment, decrypt_user, project_currency, DecryptedExpense,
    DecryptedPayment,
};
use crate::expenses::tabs::expenses_tab::inconsistent_expense_ids;

/// A decrypted expense plus the category emoji and parsed date every row needs.
///
/// Both are derived here and **never cached by expense id**: an edit replaces the ciphertext row
/// and the emoji must follow the new name, which an id-keyed cache would not.
///
/// A hand-picked category wins over the name so the row agrees with the charts — except when the
/// two agree, where the more specific leaf emoji wins: "Pizza" under Nourriture stays 🍕, moved to
/// Transport it becomes 🚕.
#[derive(Clone, Debug, PartialEq)]
pub struct RowExpense {
    pub expense: DecryptedExpense,
    pub emoji: &'static str,
    pub date: chrono::NaiveDate,
}

impl From<DecryptedExpense> for RowExpense {
    fn from(expense: DecryptedExpense) -> Self {
        let inferred = find_category(&expense.name);
        let parent = get_expense_category(&expense);
        let emoji = if parent == inferred.parent { inferred.emoji } else { parent_emoji(parent) };
        // `created_at` rather than dropping the row: an unparseable date is still an expense, and
        // vanishing unexplained is worse than being filed under the day it was entered.
        let date = chrono::NaiveDate::parse_from_str(&expense.date, "%Y-%m-%d")
            .unwrap_or_else(|_| expense.created_at.date());
        Self { expense, emoji, date }
    }
}

/// The page's still-encrypted inputs, resolved against the offline cache and **tagged with the
/// project they belong to**.
///
/// The tag is load-bearing: the router renders `ExpensesPage { project_id }` with no `key:`
/// (`router-macro/src/route.rs`), so a deep link between projects reuses the component and every
/// `use_signal` survives. Untagged, A's rows render under B and land in B's cache entry beside B's
/// `data_version`, after which the server answers "unchanged" forever. Every consumer filters on
/// it rather than trusting mount semantics.
#[derive(Clone, Debug, PartialEq)]
pub struct LiveData {
    pub project_id: uuid::Uuid,
    pub project: Option<ProjectDto>,
    pub users: Vec<User>,
    pub expenses: Vec<Expense>,
    pub payments: Vec<Payment>,
    /// The `data_version` these exact rows correspond to; `None` for cached rows or after a local
    /// patch. It is the cache's claim to be current, so clearing it is deliberate: the rows are
    /// right but their version is not, and claiming a stale one gets "unchanged" back forever.
    pub version: Option<i64>,
}

/// Everything the page renders from, decrypted exactly once.
#[derive(Clone, Debug)]
pub struct ProjectData {
    /// Which project this was built from. The store outlives any one page, so a consumer that does
    /// not check this can render the previous project's rows in the window before the new sync
    /// lands. `None` when nothing is loaded.
    pub project_id: Option<uuid::Uuid>,
    pub expenses: Vec<RowExpense>,
    pub payments: Vec<DecryptedPayment>,
    /// The still-encrypted user rows — the avatar group and the participant pickers want these.
    pub users: Vec<User>,
    pub user_names: HashMap<i32, String>,
    /// Payer ids per expense, for the "payée par …" subtitle.
    pub payers_by_expense: HashMap<i32, Vec<i32>>,
    /// Expenses whose payments no longer add up to their total.
    pub inconsistent: HashSet<i32>,
    pub summary: UserSummary,
    /// Sum of what was paid, counting `Expense` entries only — transfers and gains are excluded.
    pub global_total: f64,
    /// Ids of the `Expense`-typed entries, so callers can apply the same exclusion.
    pub real_expense_ids: HashSet<i32>,
    pub currency: String,
    pub project_status: ProjectStatus,
    /// False while the three body resources have produced neither live data nor a usable cache.
    /// The page shows its error/spinner instead of an empty project.
    pub loaded: bool,
    /// Identity stamp, fresh on every `build` — see the `PartialEq` impl below.
    pub revision: u64,
}

/// Spelled out because neither `UserSummary` nor `ProjectStatus` has a `Default`, and inventing
/// one would put a meaningless status in the shared crate.
impl Default for ProjectData {
    fn default() -> Self {
        Self {
            project_id: None,
            expenses: Vec::new(),
            payments: Vec::new(),
            users: Vec::new(),
            user_names: HashMap::new(),
            payers_by_expense: HashMap::new(),
            inconsistent: HashSet::new(),
            summary: UserSummary {
                reimbursement_suggestions: Vec::new(),
                summary: HashMap::new(),
            },
            global_total: 0.0,
            real_expense_ids: HashSet::new(),
            currency: "...".to_string(),
            project_status: ProjectStatus::Ongoing,
            loaded: false,
            revision: 0,
        }
    }
}

/// Identity, not field-by-field: every consumer should see a rebuild, and a deep compare would
/// cost more than it saves. Same `build` call is equal, any two builds are not.
///
/// **Keep `eq` reflexive.** It was once `{ false }`, so `x != x` held; `Memo::recompute` and
/// `ReadSignal`'s derived `PartialEq` both then reported "changed" on every read, and the
/// render/write cycle pinned the main thread — see `equality_is_reflexive`.
impl PartialEq for ProjectData {
    fn eq(&self, other: &Self) -> bool {
        self.revision == other.revision
    }
}

impl ProjectData {
    /// Transfers and gains excluded, matching `global_total`.
    pub fn user_total(&self, user_id: i32) -> f64 {
        self.payments
            .iter()
            .filter(|p| p.user_id == user_id && p.is_debt)
            .filter(|p| self.real_expense_ids.contains(&p.expense_id))
            .map(|p| p.amount)
            .sum()
    }

    pub fn payor_label(&self, expense_id: i32) -> String {
        let payer_ids = self.payers_by_expense.get(&expense_id).map_or(&[][..], |v| v);
        match payer_ids.len() {
            0 => "inconnu".to_string(),
            1 => self.user_names.get(&payer_ids[0]).cloned().unwrap_or_else(|| "inconnu".to_string()),
            n => format!("{n} personnes"),
        }
    }
}

// Monotonic, so no two `build` results compare equal. Thread-local rather than an atomic: the only
// rendering target is single-threaded wasm, and a shared counter would make one test's stamps
// depend on another's.
thread_local! {
    static REVISION: std::cell::Cell<u64> = const { std::cell::Cell::new(0) };
}

/// Never 0: that is `ProjectData::default()`'s stamp, so empty pages compare equal and an
/// unloaded memo does not churn.
fn next_revision() -> u64 {
    REVISION.with(|c| {
        let next = c.get().wrapping_add(1).max(1);
        c.set(next);
        next
    })
}

/// Rows whose ciphertext will not open are dropped — one corrupt row must not take the page down.
pub fn build(key: Option<[u8; 32]>, live: Option<&LiveData>) -> ProjectData {
    let (Some(k), Some(live)) = (key, live) else {
        return ProjectData::default();
    };
    let (users, expenses, payments) = (&live.users, &live.expenses, &live.payments);
    let project = live.project.as_ref();

    let decrypted: Vec<DecryptedExpense> =
        expenses.iter().filter_map(|e| decrypt_expense(&k, e).ok()).collect();
    // Only the payments of expenses the page lists: the rest used to move balances by rows nobody
    // could see.
    let listed: HashSet<i32> = decrypted.iter().map(|d| d.id).collect();
    let payments: Vec<DecryptedPayment> = payments
        .iter()
        .filter(|p| listed.contains(&p.expense_id))
        .filter_map(|p| decrypt_payment(&k, p).ok())
        .collect();
    let user_names: HashMap<i32, String> =
        users.iter().filter_map(|u| decrypt_user(&k, u).ok().map(|d| (d.id, d.name))).collect();

    // Borrowed before `decrypted` moves into the row wrappers, so no clone. The signature stays
    // `&[DecryptedExpense]` — 18 tests are written against it.
    let inconsistent = inconsistent_expense_ids(&decrypted, &payments);

    let real_expense_ids: HashSet<i32> = decrypted
        .iter()
        .filter(|d| d.expense_type == ExpenseType::Expense)
        .map(|d| d.id)
        .collect();

    let global_total: f64 = payments
        .iter()
        .filter(|p| !p.is_debt && real_expense_ids.contains(&p.expense_id))
        .map(|p| p.amount)
        .sum();

    let mut payers_by_expense: HashMap<i32, Vec<i32>> = HashMap::new();
    for p in payments.iter().filter(|p| !p.is_debt) {
        payers_by_expense.entry(p.expense_id).or_default().push(p.user_id);
    }

    let summary = summary_from_payments(&payments);

    let (currency, project_status) = project
        .map(|p| (project_currency(&k, p), p.status.clone()))
        .unwrap_or_else(|| ("...".to_string(), ProjectStatus::Ongoing));

    ProjectData {
        project_id: Some(live.project_id),
        expenses: decrypted.into_iter().map(RowExpense::from).collect(),
        payments,
        users: live.users.clone(),
        user_names,
        payers_by_expense,
        inconsistent,
        summary,
        global_total,
        real_expense_ids,
        currency,
        project_status,
        loaded: true,
        revision: next_revision(),
    }
}

/// Net balance per user (positive is owed) and the transfers that settle them. Every payment
/// counts, transfers and gains included: they move real money.
pub fn summary_from_payments(payments: &[DecryptedPayment]) -> UserSummary {
    let mut balances: HashMap<i32, f64> = HashMap::new();
    for dp in payments {
        let entry = balances.entry(dp.user_id).or_insert(0.0);
        if dp.is_debt {
            *entry -= dp.amount;
        } else {
            *entry += dp.amount;
        }
    }
    let balance_vec: Vec<UserBalance> =
        balances.iter().map(|(id, amt)| UserBalance { user_id: *id, amount: *amt }).collect();
    let suggestions = get_reimbursement_suggestions(balance_vec);
    UserSummary { reimbursement_suggestions: suggestions, summary: balances }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::test_fixtures::{
        large_project, make_expense, make_expense_with_category, make_payment, make_user, test_key,
    };
    use crate::crypto::count_decrypts;
    use uuid::Uuid;

    fn live(users: Vec<User>, expenses: Vec<Expense>, payments: Vec<Payment>) -> LiveData {
        LiveData {
            project_id: uuid::Uuid::nil(),
            project: None,
            users,
            expenses,
            payments,
            version: None,
        }
    }

    /// `large_project()` is a `&'static` fixture, so rows are cloned — outside `count_decrypts`,
    /// so the clone never lands in a decryption count.
    fn large_live() -> LiveData {
        let p = large_project();
        live(p.users.clone(), p.expenses.clone(), p.payments.clone())
    }

    fn built(l: &LiveData) -> ProjectData {
        build(Some(test_key()), Some(l))
    }

    /// The tag is what stops A's rows rendering under B — see [`LiveData`].
    ///
    /// The store is shared and outlives any one page, so it still holds A while B is mounting.
    /// `build` stamps the id it actually built from and `ProjectStore::data_for` compares it; a
    /// consumer that skipped that check would render A's expenses under B's header.
    #[test]
    fn data_belonging_to_another_project_is_refused() {
        let k = test_key();
        let a = Uuid::from_u128(1);
        let b = Uuid::from_u128(2);
        let held = LiveData {
            project_id: a,
            project: None,
            users: vec![],
            expenses: vec![expense(&k, 1, "Pizza", "2025-01-01")],
            payments: vec![],
            version: None,
        };

        let built = build(Some(k), Some(&held));
        assert!(built.loaded);
        assert_eq!(built.expenses.len(), 1);
        assert_eq!(built.project_id, Some(a), "the pass must name the project it decrypted");

        // What every consumer does before rendering, and what `data_for` encodes.
        assert!(built.project_id == Some(a));
        assert!(built.project_id != Some(b), "B must show its skeleton, never A's rows");
    }

    /// Nothing loaded is nobody's project — `data_for` must not match a page on it.
    #[test]
    fn an_unloaded_pass_names_no_project() {
        assert_eq!(ProjectData::default().project_id, None);
        assert_eq!(build(Some(test_key()), None).project_id, None);
    }

    /// A wrong key drops every row, so `loaded` is the only thing left saying data was expected —
    /// the page must not present that as an empty project.
    #[test]
    fn a_wrong_key_drops_every_row_without_panicking() {
        let k = test_key();
        let other = [0x11u8; 32];
        let l = live(
            vec![make_user(&k, 7, "Alice")],
            vec![expense(&k, 1, "Pizza", "2025-01-01")],
            vec![make_payment(&k, 1, 1, 7, false, 10.0)],
        );
        let data = build(Some(other), Some(&l));
        assert!(data.expenses.is_empty());
        assert!(data.payments.is_empty());
        assert!(data.user_names.is_empty());
        assert_eq!(data.global_total, 0.0);
        assert!(data.summary.summary.is_empty());
    }

    /// Balances follow the rows the page shows. An expense that will not open is not listed, so
    /// its payments moving the balances was money nobody could see or correct.
    #[test]
    fn an_unreadable_expense_moves_no_balance() {
        let k = test_key();
        let other = [0x11u8; 32];
        let l = live(
            vec![make_user(&k, 7, "Alice"), make_user(&k, 8, "Bob")],
            vec![expense(&k, 1, "Pizza", "2025-01-01"), expense(&other, 2, "Hidden", "2025-01-02")],
            vec![
                make_payment(&k, 1, 1, 7, false, 10.0),
                make_payment(&k, 2, 1, 8, true, 10.0),
                make_payment(&k, 3, 2, 7, false, 50.0),
                make_payment(&k, 4, 2, 8, true, 50.0),
            ],
        );
        let data = built(&l);
        assert_eq!(data.summary.summary.get(&7), Some(&10.0));
        assert_eq!(data.summary.summary.get(&8), Some(&-10.0));
    }

    /// One unreadable expense must not take the other rows, the totals or the balances with it.
    #[test]
    fn one_corrupt_row_does_not_take_the_page_down() {
        let k = test_key();
        let mut bad = expense(&k, 2, "Corrompu", "2025-01-01");
        bad.payload.ct = "not_valid_base64!!!".to_string();
        let l = live(
            vec![],
            vec![make_expense(&k, 1, "Pizza", 30.0, ExpenseType::Expense, "2025-01-01"), bad],
            vec![make_payment(&k, 1, 1, 7, false, 30.0)],
        );
        let data = build(Some(k), Some(&l));
        assert_eq!(data.expenses.len(), 1, "the readable row survives");
        assert_eq!(data.expenses[0].expense.id, 1);
        assert!((data.global_total - 30.0).abs() < 0.01);
        assert!(data.loaded);
    }

    /// Regression guard for the freeze in `c0e10e0` — see the `PartialEq` impl.
    #[test]
    fn equality_is_reflexive() {
        let k = test_key();
        let l = live(vec![], vec![expense(&k, 1, "Pizza", "2025-01-01")], vec![]);
        let data = build(Some(k), Some(&l));
        #[allow(clippy::eq_op)]
        {
            assert!(data == data, "a value must equal itself, or every consumer sees a change");
        }
        assert_eq!(ProjectData::default(), ProjectData::default(), "and so must two empty pages");
    }

    /// The other half: a rebuild means new data, even when the inputs happen to be identical.
    #[test]
    fn two_builds_are_never_equal() {
        let k = test_key();
        let l = live(vec![], vec![expense(&k, 1, "Pizza", "2025-01-01")], vec![]);
        assert_ne!(build(Some(k), Some(&l)), build(Some(k), Some(&l)));
    }

    #[test]
    fn without_a_key_nothing_is_attempted() {
        let l = large_live();
        let (data, n) = count_decrypts(|| build(None, Some(&l)));
        assert_eq!(n, 0, "without a key nothing may be decrypted");
        assert!(!data.loaded, "and the page must not render an empty project as if it loaded");
    }

    /// `large_project()` is 10 users + 1 000 expenses + 5 000 payments, so one decrypt per row is
    /// 6 010. Per-row decryption cost ~5.02M; decrypting in parent *and* tab cost 12 020.
    #[test]
    fn decrypt_count_is_linear() {
        let l = large_live();
        let (data, n) = count_decrypts(|| built(&l));
        assert_eq!(data.expenses.len(), 1_000, "every expense should survive the pass");
        assert_eq!(n, 6_010, "one decryption per row of data, not per row x payment: got {n}");
    }

    fn expense(k: &[u8; 32], id: i32, name: &str, date: &str) -> Expense {
        make_expense(k, id, name, 10.0, ExpenseType::Expense, date)
    }

    #[test]
    fn the_emoji_comes_from_the_name() {
        let k = test_key();
        let data =
            build(Some(k), Some(&live(vec![], vec![expense(&k, 1, "Pizza", "2025-01-01")], vec![])));
        assert_eq!(data.expenses[0].emoji, "🍕");
    }

    /// Why `RowExpense` derives the emoji rather than caching it by id.
    #[test]
    fn renaming_an_expense_updates_its_emoji() {
        let k = test_key();
        let before =
            build(Some(k), Some(&live(vec![], vec![expense(&k, 1, "Taxi", "2025-01-01")], vec![])));
        assert_eq!(before.expenses[0].emoji, "🚕");

        let after =
            build(Some(k), Some(&live(vec![], vec![expense(&k, 1, "Pizza", "2025-01-01")], vec![])));
        assert_eq!(after.expenses[0].emoji, "🍕", "a rename must re-derive the emoji");
    }

    /// The category the user picked wins over the name, so the row agrees with the charts.
    #[test]
    fn a_picked_category_overrides_the_name() {
        let k = test_key();
        let e = make_expense_with_category(&k, 1, "Pizza", "Transport");
        let data = build(Some(k), Some(&live(vec![], vec![e], vec![])));
        assert_eq!(data.expenses[0].emoji, "🚕");
    }

    /// When the pick agrees with what the name inferred, the leaf emoji is the better of the two.
    #[test]
    fn a_picked_category_matching_the_name_keeps_the_leaf_emoji() {
        let k = test_key();
        let e = make_expense_with_category(&k, 1, "Pizza", "Nourriture");
        let data = build(Some(k), Some(&live(vec![], vec![e], vec![])));
        assert_eq!(data.expenses[0].emoji, "🍕");
    }

    #[test]
    fn a_category_outside_the_chart_list_falls_back_to_the_name() {
        let k = test_key();
        let e = make_expense_with_category(&k, 1, "Pizza", "PasUneCategorie");
        let data = build(Some(k), Some(&live(vec![], vec![e], vec![])));
        assert_eq!(data.expenses[0].emoji, "🍕");
    }

    /// It used to be dropped from the list outright, with nothing to show the user why.
    #[test]
    fn an_unparseable_date_falls_back_to_created_at_instead_of_vanishing() {
        let k = test_key();
        let expenses = vec![expense(&k, 1, "Pizza", "pas-une-date")];
        let data = build(Some(k), Some(&live(vec![], expenses, vec![])));
        assert_eq!(data.expenses.len(), 1, "the row must still render");
        assert_eq!(data.expenses[0].date, data.expenses[0].expense.created_at.date());
    }

    #[test]
    fn transfers_and_gains_stay_out_of_the_totals() {
        let k = test_key();
        let expenses = vec![
            make_expense(&k, 1, "Pizza", 30.0, ExpenseType::Expense, "2025-01-01"),
            make_expense(&k, 2, "Remboursement", 50.0, ExpenseType::Transfer, "2025-01-01"),
        ];
        let payments = vec![
            make_payment(&k, 1, 1, 7, false, 30.0),
            make_payment(&k, 2, 2, 7, false, 50.0),
        ];
        let data = build(Some(k), Some(&live(vec![], expenses, payments)));
        assert_eq!(data.global_total, 30.0, "only Expense entries count toward the total");
        assert!(data.real_expense_ids.contains(&1));
        assert!(!data.real_expense_ids.contains(&2));
    }

    #[test]
    fn gains_stay_out_of_the_totals_too() {
        let k = test_key();
        let expenses = vec![
            make_expense(&k, 1, "Pizza", 25.0, ExpenseType::Expense, "2025-01-01"),
            make_expense(&k, 2, "Cagnotte", 12.5, ExpenseType::Gain, "2025-01-01"),
        ];
        let payments = vec![
            make_payment(&k, 1, 1, 1, false, 25.0),
            make_payment(&k, 2, 2, 1, false, 12.5),
        ];
        let data = build(Some(k), Some(&live(vec![], expenses, payments)));
        assert!((data.global_total - 25.0).abs() < 0.01, "got {}", data.global_total);
        assert!(!data.real_expense_ids.contains(&2));
    }

    #[test]
    fn user_total_excludes_transfers_too() {
        let k = test_key();
        let expenses = vec![
            make_expense(&k, 1, "Pizza", 30.0, ExpenseType::Expense, "2025-01-01"),
            make_expense(&k, 2, "Remboursement", 50.0, ExpenseType::Transfer, "2025-01-01"),
        ];
        let payments = vec![
            make_payment(&k, 1, 1, 7, true, 30.0),
            make_payment(&k, 2, 2, 7, true, 50.0),
        ];
        let data = build(Some(k), Some(&live(vec![], expenses, payments)));
        assert_eq!(data.user_total(7), 30.0);
    }

    /// "Mes dépenses" is what you consumed, not what you fronted. User 1 pays 200 for a 50/50
    /// split: their total is 100, not 200.
    #[test]
    fn user_total_is_the_debt_share_not_the_amount_paid() {
        let k = test_key();
        let expenses = vec![make_expense(&k, 1, "Pizza", 200.0, ExpenseType::Expense, "2025-01-01")];
        let payments = vec![
            make_payment(&k, 1, 1, 1, false, 200.0),
            make_payment(&k, 2, 1, 1, true, 100.0),
            make_payment(&k, 3, 1, 2, true, 100.0),
        ];
        let data = build(Some(k), Some(&live(vec![], expenses, payments)));
        assert!((data.user_total(1) - 100.0).abs() < 0.01, "got {}", data.user_total(1));
    }

    #[test]
    fn user_total_sums_across_expenses_and_is_zero_without_any() {
        let k = test_key();
        let expenses = vec![
            make_expense(&k, 1, "Pizza", 30.0, ExpenseType::Expense, "2025-01-01"),
            make_expense(&k, 2, "Taxi", 20.0, ExpenseType::Expense, "2025-01-02"),
        ];
        let payments = vec![
            make_payment(&k, 1, 1, 1, true, 30.0),
            make_payment(&k, 2, 2, 1, true, 20.0),
        ];
        let data = build(Some(k), Some(&live(vec![], expenses, payments)));
        assert!((data.user_total(1) - 50.0).abs() < 0.01);
        assert_eq!(data.user_total(99), 0.0, "a user with no debt rows owes nothing");
        assert_eq!(ProjectData::default().user_total(1), 0.0);
    }

    #[test]
    fn payor_label_reads_the_index() {
        let k = test_key();
        let users = vec![make_user(&k, 7, "Alice"), make_user(&k, 8, "Bob")];
        let one = vec![make_payment(&k, 1, 1, 7, false, 10.0), make_payment(&k, 2, 1, 8, true, 10.0)];
        let data = build(Some(k), Some(&live(users.clone(), vec![expense(&k, 1, "Pizza", "2025-01-01")], one)));
        assert_eq!(data.payor_label(1), "Alice");

        let two = vec![
            make_payment(&k, 1, 1, 7, false, 5.0),
            make_payment(&k, 2, 1, 8, false, 5.0),
        ];
        let data = build(Some(k), Some(&live(users, vec![expense(&k, 1, "Pizza", "2025-01-01")], two)));
        assert_eq!(data.payor_label(1), "2 personnes");
        assert_eq!(data.payor_label(99), "inconnu", "an expense nobody paid");
    }

    fn dp(user_id: i32, is_debt: bool, amount: f64) -> DecryptedPayment {
        DecryptedPayment {
            id: 0,
            expense_id: 1,
            user_id,
            is_debt,
            amount,
            created_at: chrono::NaiveDateTime::default(),
        }
    }

    #[test]
    fn summary_of_nothing_is_empty() {
        let s = summary_from_payments(&[]);
        assert!(s.summary.is_empty());
        assert!(s.reimbursement_suggestions.is_empty());
    }

    #[test]
    fn the_payer_is_owed_and_the_debtor_owes() {
        let s = summary_from_payments(&[dp(1, false, 30.0), dp(2, true, 30.0)]);
        assert!(s.summary[&1] > 0.0, "payer should have positive balance");
        assert!(s.summary[&2] < 0.0, "debtor should have negative balance");
        assert!((s.summary[&1] + s.summary[&2]).abs() < 0.01, "net must be zero");
    }

    #[test]
    fn a_split_expense_balances() {
        let s = summary_from_payments(&[dp(1, false, 30.0), dp(2, true, 15.0), dp(3, true, 15.0)]);
        assert!((s.summary[&1] - 30.0).abs() < 0.01);
        assert!((s.summary[&2] + 15.0).abs() < 0.01);
        assert!((s.summary[&3] + 15.0).abs() < 0.01);
        assert_eq!(s.reimbursement_suggestions.len(), 2);
    }

    #[test]
    fn the_suggestion_points_from_debtor_to_payer() {
        let s = summary_from_payments(&[dp(1, false, 100.0), dp(2, true, 100.0)]);
        assert_eq!(s.reimbursement_suggestions.len(), 1);
        let sug = &s.reimbursement_suggestions[0];
        assert_eq!(sug.user_id_debtor, 2);
        assert_eq!(sug.user_id_payer, 1);
        assert!((sug.amount - 100.0).abs() < 0.01);
    }

    /// A row that will not open is dropped by the pass, so it never reaches the summary — one
    /// corrupt payment must not take the whole project's balances down with it.
    #[test]
    fn a_corrupt_payment_is_dropped_not_surfaced() {
        let k = test_key();
        let mut bad = make_payment(&k, 2, 1, 99, true, 50.0);
        bad.payload.ct = "not_valid_base64!!!".to_string();
        let payments = vec![make_payment(&k, 1, 1, 1, false, 20.0), bad];
        let expenses = vec![expense(&k, 1, "Pizza", "2025-01-01")];
        let data = build(Some(k), Some(&live(vec![], expenses, payments)));
        assert_eq!(data.payments.len(), 1, "the corrupt row must be dropped");
        assert!(data.summary.summary.contains_key(&1));
        assert!(!data.summary.summary.contains_key(&99));
    }
}
