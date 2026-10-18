//! "Mes dépenses" (designs R1, R2, R5, R7): the personal project's own rows plus my share of every
//! group this device holds, each group counted unless switched off in the encrypted preferences.

use api::payments::payments_controller::sync_project_data;
use chrono::{Datelike, NaiveDate};
use shared::{Expense, ExpenseType, Payment, ProjectDto, ProjectPayload, User};
use std::collections::{HashMap, HashSet};
use uuid::Uuid;

use crate::charts::charts_page::{known_version, synced_rows};
use crate::common::fx_cache::{cross_rate, CachedFx};
use crate::common::{key_of, read_from_ls, LocalStorageProject, LocalStorageState};
use crate::crypto::decrypt_json;
use crate::decrypted::{pickable_users, DecryptedExpense};
use crate::expenses::helpers::budgets::{budget_lines, spent_by_category, BudgetLine};
use crate::expenses::helpers::project_data::{self, LiveData, ProjectData, RowExpense};
use crate::expenses::tabs::expenses_tab::my_debt_by_expense;

/// One group, decrypted as its own page would decrypt it.
#[derive(Clone, Debug, PartialEq)]
pub struct GroupLedger {
    pub project_id: Uuid,
    pub name: String,
    /// My participant there; `None` until I picked one, when no share is mine.
    pub me: Option<i32>,
    pub members: usize,
    pub data: ProjectData,
}

/// My share of one group expense.
#[derive(Clone, Debug, PartialEq)]
pub struct ShareRow {
    /// The group's expense, its amount in the group's currency.
    pub row: RowExpense,
    pub project_id: Uuid,
    pub group: String,
    pub currency: String,
    pub share: f64,
    /// The share in the personal project's currency; `None` when no rate is known.
    pub converted: Option<f64>,
    /// Who paid; `None` when I did.
    pub payer: Option<String>,
    pub counted: bool,
}

pub fn first_of_month(date: NaiveDate) -> NaiveDate {
    date.with_day(1).unwrap_or(date)
}

pub fn shift_month(month: NaiveDate, by: i32) -> NaiveDate {
    let index = month.year() * 12 + month.month0() as i32 + by;
    NaiveDate::from_ymd_opt(index.div_euclid(12), index.rem_euclid(12) as u32 + 1, 1).unwrap_or(month)
}

pub fn in_month(date: NaiveDate, month: NaiveDate) -> bool {
    date.year() == month.year() && date.month() == month.month()
}

/// The rate frozen on the expense when it was typed in `to`, the month's InforEuro table otherwise.
pub fn convert(share: f64, expense: &DecryptedExpense, from: &str, to: &str, fx: Option<&CachedFx>) -> Option<f64> {
    if from == to {
        return Some(share);
    }
    if let Some((typed, currency, _)) = expense.conversion() {
        if currency == to && expense.amount > 0.0 {
            return Some(share * typed / expense.amount);
        }
    }
    fx.and_then(|t| cross_rate(t, from, to)).map(|rate| share * rate)
}

/// Every expense of every group where I owe something. Transfers settle debts and gains are not
/// spending, so neither is a share.
pub fn share_rows(ledgers: &[GroupLedger], uncounted: &HashSet<Uuid>, currency: &str, fx: Option<&CachedFx>) -> Vec<ShareRow> {
    let mut out = Vec::new();
    for ledger in ledgers {
        let Some(me) = ledger.me else { continue };
        let debts = my_debt_by_expense(&ledger.data.payments, me);
        for row in ledger.data.expenses.iter().filter(|r| r.expense.expense_type == ExpenseType::Expense) {
            let id = row.expense.id;
            let Some(share) = debts.get(&id).copied().filter(|s| *s > 0.0) else { continue };
            let by_me = ledger.data.payers_by_expense.get(&id).is_some_and(|p| p[..] == [me]);
            out.push(ShareRow {
                row: row.clone(),
                project_id: ledger.project_id,
                group: ledger.name.clone(),
                currency: ledger.data.currency.clone(),
                share,
                converted: convert(share, &row.expense, &ledger.data.currency, currency, fx),
                payer: (!by_me).then(|| ledger.data.payor_label(id)),
                counted: !uncounted.contains(&ledger.project_id),
            });
        }
    }
    out
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct MonthTotals {
    pub personal: f64,
    pub groups: f64,
    pub income: f64,
    /// Counted shares left out of `groups` for want of a rate.
    pub unconverted: usize,
}

impl MonthTotals {
    pub fn spent(&self) -> f64 {
        self.personal + self.groups
    }

    pub fn balance(&self) -> f64 {
        self.income - self.spent()
    }
}

pub fn month_totals(own: &[RowExpense], shares: &[ShareRow], month: NaiveDate) -> MonthTotals {
    let mut totals = MonthTotals::default();
    for r in own.iter().filter(|r| in_month(r.date, month)) {
        match r.expense.expense_type {
            ExpenseType::Expense => totals.personal += r.expense.amount,
            ExpenseType::Gain => totals.income += r.expense.amount,
            ExpenseType::Transfer => {}
        }
    }
    for s in shares.iter().filter(|s| s.counted && in_month(s.row.date, month)) {
        match s.converted {
            Some(amount) => totals.groups += amount,
            None => totals.unconverted += 1,
        }
    }
    totals
}

/// The personal card on the projects page (designs J3, P4): the month as Mes dépenses adds it up,
/// and every ceiling taken together against what their categories spent.
#[derive(Clone, Debug, PartialEq)]
pub struct MonthGlance {
    pub spent: f64,
    pub budget: Option<BudgetLine>,
}

/// From this device's cache alone; `None` until the personal project has been opened here once.
pub fn month_glance(
    state: &LocalStorageState,
    personal: Uuid,
    uncounted: &HashSet<Uuid>,
    fx: Option<&CachedFx>,
    today: NaiveDate,
) -> Option<MonthGlance> {
    let p = state.projects.iter().find(|p| p.project_id == personal)?;
    let key = key_of(p)?;
    let data = project_data::build(
        Some(key),
        Some(&LiveData {
            project_id: personal,
            project: p.cached_project.clone(),
            users: p.cached_users.clone().unwrap_or_default(),
            expenses: p.cached_expenses.clone()?,
            payments: p.cached_payments.clone().unwrap_or_default(),
            version: None,
        }),
    );
    let month = first_of_month(today);
    let shares = share_rows(&cached_ledgers(state, personal), uncounted, &data.currency, fx);
    let lines = budget_lines(&data.budgets, &spent_by_category(&data.expenses, &shares, month));
    let budget = (!lines.is_empty()).then(|| BudgetLine {
        category: "",
        spent: lines.iter().map(|l| l.spent).sum(),
        limit: lines.iter().map(|l| l.limit).sum(),
    });
    Some(MonthGlance { spent: month_totals(&data.expenses, &shares, month).spent(), budget })
}

/// What a list of results adds up to: own expenses, and counted shares in the personal currency.
pub fn results_total(rows: &[&RowExpense], shares: &HashMap<i32, &ShareRow>) -> f64 {
    rows.iter()
        .map(|r| match shares.get(&r.expense.id) {
            Some(s) => s.converted.filter(|_| s.counted).unwrap_or(0.0),
            None if r.expense.expense_type == ExpenseType::Expense => r.expense.amount,
            None => 0.0,
        })
        .sum()
}

/// Every other project this device holds a key for, synced as the charts page syncs them. Offline,
/// a group falls back to its cached rows.
pub async fn load_ledgers(personal: Uuid) -> Vec<GroupLedger> {
    let ls = read_from_ls();
    let mut out = Vec::new();
    for p in ls.projects.iter().filter(|p| p.project_id != personal) {
        let (project, users, rows) = match sync_project_data(p.project_id, known_version(p)).await {
            Ok(sync) => (
                sync.project.clone().or_else(|| p.cached_project.clone()),
                sync.users.clone().or_else(|| p.cached_users.clone()),
                synced_rows(sync, p),
            ),
            Err(_) => (p.cached_project.clone(), p.cached_users.clone(), cached_rows(p)),
        };
        out.extend(ledger_of(p, project, users, rows));
    }
    sorted(out)
}

/// The same groups from this device's cache alone, for the projects page, which makes no request
/// per project.
pub fn cached_ledgers(state: &LocalStorageState, personal: Uuid) -> Vec<GroupLedger> {
    let ledgers = state
        .projects
        .iter()
        .filter(|p| p.project_id != personal)
        .filter_map(|p| ledger_of(p, p.cached_project.clone(), p.cached_users.clone(), cached_rows(p)))
        .collect();
    sorted(ledgers)
}

fn cached_rows(p: &LocalStorageProject) -> Option<(Vec<Expense>, Vec<Payment>)> {
    p.cached_expenses.clone().zip(p.cached_payments.clone())
}

fn sorted(mut ledgers: Vec<GroupLedger>) -> Vec<GroupLedger> {
    ledgers.sort_by_key(|l| l.name.to_lowercase());
    ledgers
}

fn ledger_of(
    p: &LocalStorageProject,
    project: Option<ProjectDto>,
    users: Option<Vec<User>>,
    rows: Option<(Vec<Expense>, Vec<Payment>)>,
) -> Option<GroupLedger> {
    let key = key_of(p)?;
    let (project, (expenses, payments)) = (project?, rows?);
    let payload = decrypt_json::<ProjectPayload>(&key, &project.payload).ok()?;
    if payload.is_personal() {
        return None;
    }
    let data = project_data::build(
        Some(key),
        Some(&LiveData {
            project_id: p.project_id,
            project: Some(project),
            users: users.unwrap_or_default(),
            expenses,
            payments,
            version: None,
        }),
    );
    Some(GroupLedger {
        project_id: p.project_id,
        name: payload.name,
        me: p.user_id,
        members: pickable_users(Some(&key), &data.users, &[]).len(),
        data,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::test_fixtures::{make_expense, make_payment, make_project, make_user, test_key};

    fn day(s: &str) -> NaiveDate {
        NaiveDate::parse_from_str(s, "%Y-%m-%d").unwrap()
    }

    fn ledger(name: &str, currency: &str, me: Option<i32>, expenses: Vec<Expense>, payments: Vec<Payment>) -> GroupLedger {
        let k = test_key();
        let project_id = Uuid::new_v4();
        let data = project_data::build(
            Some(k),
            Some(&LiveData {
                project_id,
                project: Some(make_project(&k, project_id, name, currency)),
                users: vec![make_user(&k, 7, "Moi"), make_user(&k, 8, "Clara")],
                expenses,
                payments,
                version: None,
            }),
        );
        GroupLedger { project_id, name: name.to_string(), me, members: 2, data }
    }

    /// Clara paid 128 for a dinner, my share 32; I paid 20 for a taxi, my share 10; a transfer and a
    /// gain owe nothing.
    fn corsica(me: Option<i32>) -> GroupLedger {
        let k = test_key();
        ledger(
            "Corsica",
            "EUR",
            me,
            vec![
                make_expense(&k, 1, "Dîner", 128.0, ExpenseType::Expense, "2026-10-11"),
                make_expense(&k, 2, "Taxi", 20.0, ExpenseType::Expense, "2026-09-30"),
                make_expense(&k, 3, "Remboursement", 50.0, ExpenseType::Transfer, "2026-10-11"),
                make_expense(&k, 4, "Cagnotte", 30.0, ExpenseType::Gain, "2026-10-11"),
            ],
            vec![
                make_payment(&k, 1, 1, 8, false, 128.0),
                make_payment(&k, 2, 1, 7, true, 32.0),
                make_payment(&k, 3, 1, 8, true, 96.0),
                make_payment(&k, 4, 2, 7, false, 20.0),
                make_payment(&k, 5, 2, 7, true, 10.0),
                make_payment(&k, 6, 2, 8, true, 10.0),
                make_payment(&k, 7, 3, 8, false, 50.0),
                make_payment(&k, 8, 3, 7, true, 50.0),
                make_payment(&k, 9, 4, 8, false, 30.0),
                make_payment(&k, 10, 4, 7, true, 30.0),
            ],
        )
    }

    #[test]
    fn my_share_of_each_group_expense_and_nothing_else() {
        let rows = share_rows(&[corsica(Some(7))], &HashSet::new(), "EUR", None);
        let got: Vec<(i32, f64, Option<String>)> = rows.iter().map(|s| (s.row.expense.id, s.share, s.payer.clone())).collect();
        assert_eq!(got, vec![(1, 32.0, Some("Clara".to_string())), (2, 10.0, None)]);
        assert!(rows.iter().all(|s| s.counted && s.converted == Some(s.share)));
    }

    #[test]
    fn no_identity_there_no_share() {
        assert!(share_rows(&[corsica(None)], &HashSet::new(), "EUR", None).is_empty());
    }

    #[test]
    fn a_switched_off_group_is_listed_but_not_counted() {
        let group = corsica(Some(7));
        let rows = share_rows(std::slice::from_ref(&group), &HashSet::from([group.project_id]), "EUR", None);
        assert_eq!(rows.len(), 2);
        assert!(rows.iter().all(|s| !s.counted));
        assert_eq!(month_totals(&[], &rows, day("2026-10-01")), MonthTotals::default());
    }

    #[test]
    fn the_month_adds_own_expenses_counted_shares_and_income() {
        let k = test_key();
        let own: Vec<RowExpense> = [
            make_expense(&k, 20, "Courses", 46.30, ExpenseType::Expense, "2026-10-11"),
            make_expense(&k, 21, "Salaire", 2400.0, ExpenseType::Gain, "2026-10-10"),
            make_expense(&k, 22, "Loyer", 800.0, ExpenseType::Expense, "2026-09-01"),
        ]
        .iter()
        .map(|e| RowExpense::from(crate::decrypted::decrypt_expense(&k, e).unwrap()))
        .collect();
        let shares = share_rows(&[corsica(Some(7))], &HashSet::new(), "EUR", None);
        let totals = month_totals(&own, &shares, day("2026-10-01"));
        assert!((totals.personal - 46.30).abs() < 1e-9);
        assert_eq!(totals.groups, 32.0);
        assert_eq!(totals.income, 2400.0);
        assert!((totals.balance() - (2400.0 - 78.30)).abs() < 1e-9);
        assert_eq!(month_totals(&own, &shares, day("2026-09-01")).spent(), 810.0);
    }

    #[test]
    fn the_home_card_adds_up_the_month_and_the_budgets_from_the_cache() {
        use crate::crypto::{encrypt_json, key_to_fragment};
        let k = test_key();
        let category_of = |e: &Expense| RowExpense::from(crate::decrypted::decrypt_expense(&k, e).unwrap()).category;
        let courses = make_expense(&k, 20, "Courses", 46.30, ExpenseType::Expense, "2026-10-11");
        let personal_id = Uuid::new_v4();
        let mut personal = make_project(&k, personal_id, "Mes dépenses", "EUR");
        let mut payload: ProjectPayload = decrypt_json(&k, &personal.payload).unwrap();
        payload.extra.insert("kind".into(), "personal".into());
        payload.set_budgets(&shared::Budgets {
            monthly: [(category_of(&courses).to_string(), 100.0)].into_iter().collect(),
            alert: true,
        });
        personal.payload = encrypt_json(&k, &payload).unwrap();
        let mine = LocalStorageProject {
            project_id: personal_id,
            encryption_key: Some(key_to_fragment(&k)),
            cached_project: Some(personal),
            cached_expenses: Some(vec![
                courses.clone(),
                make_expense(&k, 21, "Salaire", 2400.0, ExpenseType::Gain, "2026-10-10"),
            ]),
            ..Default::default()
        };
        let group = corsica(Some(7));
        let group_id = Uuid::new_v4();
        let dinner = make_expense(&k, 1, "Dîner", 128.0, ExpenseType::Expense, "2026-10-11");
        let trip = LocalStorageProject {
            project_id: group_id,
            encryption_key: Some(key_to_fragment(&k)),
            user_id: Some(7),
            cached_project: Some(make_project(&k, group_id, "Corsica", "EUR")),
            cached_users: Some(group.data.users.clone()),
            cached_expenses: Some(vec![dinner.clone()]),
            cached_payments: Some(vec![
                make_payment(&k, 1, 1, 8, false, 128.0),
                make_payment(&k, 2, 1, 7, true, 32.0),
                make_payment(&k, 3, 1, 8, true, 96.0),
            ]),
            ..Default::default()
        };
        let state = LocalStorageState { projects: vec![mine.clone(), trip], ..Default::default() };
        let today = day("2026-10-12");

        let glance = month_glance(&state, personal_id, &HashSet::new(), None, today).unwrap();
        assert!((glance.spent - 78.30).abs() < 1e-9);
        let budget = glance.budget.unwrap();
        assert_eq!(budget.limit, 100.0);
        let dinner_counts = if category_of(&dinner) == category_of(&courses) { 32.0 } else { 0.0 };
        assert!((budget.spent - (46.30 + dinner_counts)).abs() < 1e-9);

        let uncounted = month_glance(&state, personal_id, &HashSet::from([group_id]), None, today).unwrap();
        assert!((uncounted.spent - 46.30).abs() < 1e-9);

        let unopened = LocalStorageProject { cached_expenses: None, ..mine };
        let state = LocalStorageState { projects: vec![unopened], ..Default::default() };
        assert_eq!(month_glance(&state, personal_id, &HashSet::new(), None, today), None);
    }

    #[test]
    fn another_currency_converts_or_is_left_out_and_counted_as_such() {
        let k = test_key();
        let ny = ledger(
            "New York",
            "USD",
            Some(7),
            vec![make_expense(&k, 1, "Diner", 100.0, ExpenseType::Expense, "2026-10-02")],
            vec![make_payment(&k, 1, 1, 8, false, 100.0), make_payment(&k, 2, 1, 7, true, 50.0)],
        );
        let none = share_rows(std::slice::from_ref(&ny), &HashSet::new(), "EUR", None);
        assert_eq!(none[0].converted, None);
        assert_eq!(month_totals(&[], &none, day("2026-10-01")).unconverted, 1);

        let table = CachedFx {
            rates: [("EUR".to_string(), 1.0), ("USD".to_string(), 1.25)].into_iter().collect(),
            ..Default::default()
        };
        let rows = share_rows(&[ny], &HashSet::new(), "EUR", Some(&table));
        assert_eq!(rows[0].converted, Some(40.0));
    }

    #[test]
    fn a_rate_frozen_on_the_expense_wins_over_the_table() {
        let mut expense = RowExpense::from(crate::decrypted::decrypt_expense(
            &test_key(),
            &make_expense(&test_key(), 1, "Hotel", 110.0, ExpenseType::Expense, "2026-10-02"),
        ).unwrap())
        .expense;
        expense.source_currency = Some("EUR".to_string());
        expense.source_amount = Some(100.0);
        expense.rate = Some(1.1);
        let table = CachedFx { rates: [("EUR".to_string(), 1.0), ("USD".to_string(), 2.0)].into_iter().collect(), ..Default::default() };
        assert_eq!(convert(55.0, &expense, "USD", "EUR", Some(&table)), Some(50.0));
        assert_eq!(convert(55.0, &expense, "USD", "GBP", None), None);
    }

    #[test]
    fn months_shift_across_years() {
        assert_eq!(shift_month(day("2026-01-01"), -1), day("2025-12-01"));
        assert_eq!(shift_month(day("2026-12-01"), 1), day("2027-01-01"));
        assert_eq!(first_of_month(day("2026-10-12")), day("2026-10-01"));
    }

    #[test]
    fn results_add_own_expenses_and_counted_shares_only() {
        let group = corsica(Some(7));
        let shares = share_rows(std::slice::from_ref(&group), &HashSet::from([group.project_id]), "EUR", None);
        let k = test_key();
        let own = RowExpense::from(crate::decrypted::decrypt_expense(&k, &make_expense(&k, 30, "Resto", 48.0, ExpenseType::Expense, "2026-10-11")).unwrap());
        let by_id: HashMap<i32, &ShareRow> = shares.iter().map(|s| (s.row.expense.id, s)).collect();
        let rows: Vec<&RowExpense> = std::iter::once(&own).chain(shares.iter().map(|s| &s.row)).collect();
        assert_eq!(results_total(&rows, &by_id), 48.0);
    }
}
