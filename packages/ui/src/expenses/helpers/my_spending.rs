//! "Mes dépenses" (designs R1, R2, R5, R7): the personal project's own rows plus my share of every
//! group this device holds, each group counted unless switched off in the encrypted preferences.

use api::payments::payments_controller::sync_projects;
use dioxus::fullstack::{Json, ServerFnError};
use chrono::{Datelike, NaiveDate};
use shared::{
    BatchSync, Expense, ExpenseType, KnownProject, Payment, ProjectDto, ProjectPayload, ProjectSync, User,
    NO_CACHED_VERSION,
};
use std::collections::{HashMap, HashSet};
use uuid::Uuid;

use crate::charts::charts_page::{known_version, synced_rows};
use crate::common::fx_cache::{cross_rate, CachedFx};
use crate::common::{
    is_rate_limited, key_of, read_from_ls, set_project_cache, sleep, write_to_ls, LocalStorageProject, RATE_LIMIT_RETRY_MS,
    LocalStorageState,
};
use crate::crypto::decrypt_json;
use crate::decrypted::{pickable_users, DecryptedExpense};
use crate::expenses::helpers::project_data::{self, LiveData, ProjectData, RowExpense};
use crate::expenses::tabs::expenses_tab::my_debt_by_expense;

/// One group, decrypted as its own page would decrypt it.
#[derive(Clone, Debug, PartialEq)]
pub struct GroupLedger {
    pub project_id: Uuid,
    pub name: String,
    pub emoji: Option<String>,
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

/// The personal card on the projects page (designs J3, P4): the month as Mes dépenses adds it up.
/// No budget: the ceilings are per category and one bar cannot stand for several.
#[derive(Clone, Debug, PartialEq)]
pub struct MonthGlance {
    pub spent: f64,
}

/// From this device's cache alone; `None` until the personal project has been opened here once.
/// `ledgers` are the groups the projects page already decrypted, not rebuilt here on every write.
pub fn month_glance(
    state: &LocalStorageState,
    personal: Uuid,
    ledgers: &[GroupLedger],
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
        Some(personal),
    );
    let month = first_of_month(today);
    let mut shares = share_rows(ledgers, uncounted, &data.currency, fx);
    shares.retain(|s| s.project_id != personal);
    Some(MonthGlance { spent: month_totals(&data.expenses, &shares, month).spent() })
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

/// Every project in `ids` in one request, written back to the cache so the next call claims those
/// versions and is answered "unchanged". Also returns the state the claims were made from: an
/// "unchanged" answer means its cached rows, which a full web quota may since have dropped.
pub async fn sync_all(ids: Vec<Uuid>) -> Result<(LocalStorageState, Vec<ProjectSync>), ServerFnError> {
    let before = read_from_ls();
    if ids.is_empty() {
        return Ok((before, vec![]));
    }
    let projects = ids
        .into_iter()
        .map(|id| KnownProject {
            id,
            known_version: before.projects.iter().find(|p| p.project_id == id).map_or(NO_CACHED_VERSION, known_version),
            token: crate::common::project_token::tokens(&[id]).pop().flatten(),
        })
        .collect();
    let request = BatchSync { projects };
    let syncs = match sync_projects(Json(request.clone())).await {
        Err(e) if is_rate_limited(&e) => {
            sleep(RATE_LIMIT_RETRY_MS).await;
            sync_projects(Json(request)).await?
        }
        result => result?,
    };
    let mut state = read_from_ls();
    for sync in &syncs {
        cache_sync(&mut state, sync);
    }
    write_to_ls(&state);
    Ok((before, syncs))
}

fn cache_sync(state: &mut LocalStorageState, sync: &ProjectSync) {
    let Some(project) = sync.project.clone() else { return };
    let id = project.id;
    let Some(entry) = state.projects.iter_mut().find(|p| p.project_id == id) else { return };
    let users = sync.users.clone().or_else(|| entry.cached_users.clone()).unwrap_or_default();
    match (sync.expenses.clone(), sync.payments.clone()) {
        (Some(expenses), Some(payments)) => {
            set_project_cache(state, id, project, users, expenses, payments, Some(sync.data_version))
        }
        _ => {
            entry.cached_project = Some(project);
            entry.cached_users = Some(users);
        }
    }
}

/// Every other project this device holds a key for, in one request. Offline, a group falls back to
/// its cached rows.
pub async fn load_ledgers(personal: Uuid) -> Vec<GroupLedger> {
    let ids = read_from_ls().projects.iter().map(|p| p.project_id).filter(|id| *id != personal).collect();
    match sync_all(ids).await {
        Ok((before, syncs)) => synced_ledgers(&before, &syncs, personal),
        Err(_) => cached_ledgers(&read_from_ls(), personal),
    }
}

/// The groups as `sync_all` answered them, each falling back to `state`'s cache where the server
/// sent no rows or no longer has the project. A project whose key changed falls back to nothing:
/// its cache is what a removed member must no longer see (D14).
pub fn synced_ledgers(state: &LocalStorageState, syncs: &[ProjectSync], personal: Uuid) -> Vec<GroupLedger> {
    let ledgers = state
        .projects
        .iter()
        .filter(|p| p.project_id != personal && !syncs.iter().any(|s| s.rotated == Some(p.project_id)))
        .filter_map(|p| match syncs.iter().find(|s| s.project.as_ref().is_some_and(|d| d.id == p.project_id)) {
            Some(sync) => ledger_of(
                p,
                sync.project.clone(),
                sync.users.clone().or_else(|| p.cached_users.clone()),
                synced_rows(sync.clone(), p),
            ),
            None => ledger_of(p, p.cached_project.clone(), p.cached_users.clone(), cached_rows(p)),
        })
        .collect();
    sorted(ledgers)
}

/// The same groups from this device's cache alone, for a first render before the sync answers.
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
        None,
    );
    Some(GroupLedger {
        project_id: p.project_id,
        emoji: payload.emoji(),
        name: payload.name,
        me: p.user_id,
        members: pickable_users(Some(&key), &data.users, &[]).len(),
        data,
    })
}

#[cfg(any(test, fuzzing))]
#[allow(clippy::too_many_arguments)]
pub(crate) fn exercise_hostile(
    group: Vec<Expense>,
    payments: Vec<Payment>,
    users: Vec<User>,
    own: Vec<Expense>,
    me: Option<i32>,
    budgets: shared::Budgets,
    rates: HashMap<String, f64>,
    project: ProjectPayload,
) {
    use crate::common::test_fixtures::{make_project, TEST_KEY};
    use crate::crypto::encrypt_json;
    use crate::expenses::helpers::budgets::{average_by_category, budget_lines, crossed, spent_by_category};

    let k = TEST_KEY;
    let (group_id, personal) = (Uuid::from_u128(1), Uuid::from_u128(2));
    let group_project = ProjectDto {
        payload: encrypt_json(&k, &ProjectPayload { currency: "USD".into(), ..project.clone() }).unwrap(),
        ..make_project(&k, group_id, "g", "USD")
    };
    let group_data = project_data::build(
        Some(k),
        Some(&LiveData { project_id: group_id, project: Some(group_project), users, expenses: group, payments, version: None }),
        None,
    );
    let mut payload = project;
    payload.set_budgets(&budgets);
    let project = ProjectDto { payload: encrypt_json(&k, &payload).unwrap(), ..make_project(&k, personal, "", "") };
    let personal_data = project_data::build(
        Some(k),
        Some(&LiveData { project_id: personal, project: Some(project), users: vec![], expenses: own, payments: vec![], version: None }),
        Some(personal),
    );
    let ledgers = [GroupLedger { project_id: group_id, name: "g".into(), emoji: None, me, members: 2, data: group_data }];
    let fx = CachedFx { day: "2026-11-01".into(), base: "EUR".into(), rates, fetched_at: 0 };
    let shares = share_rows(&ledgers, &HashSet::new(), "EUR", Some(&fx));
    let month = first_of_month(NaiveDate::from_ymd_opt(2026, 11, 1).unwrap());
    let totals = month_totals(&personal_data.expenses, &shares, month);
    assert!(totals.spent().is_finite() && totals.balance().is_finite());
    let spent = spent_by_category(&personal_data.expenses, &shares, month);
    let _ = average_by_category(&personal_data.expenses, &shares, month);
    for line in budget_lines(&personal_data.budgets, &personal_data.budgets_custom, &personal_data.categories, &spent) {
        let _ = (line.meter(), crossed(line.spent, 1.0, line.limit));
    }
    let _ = crate::expenses::budget_panel::sheet_categories(&spent, &personal_data.categories);
    let by_id: HashMap<i32, &ShareRow> = shares.iter().map(|s| (s.row.expense.id, s)).collect();
    let rows: Vec<&RowExpense> = personal_data.expenses.iter().collect();
    assert!(results_total(&rows, &by_id).is_finite());
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
            None,
        );
        GroupLedger { project_id, name: name.to_string(), emoji: None, me, members: 2, data }
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
    fn the_home_card_adds_up_the_month_from_the_cache_whatever_the_ceilings() {
        use crate::crypto::{encrypt_json, key_to_fragment};
        let k = test_key();
        let category_of = |e: &Expense| RowExpense::from(crate::decrypted::decrypt_expense(&k, e).unwrap()).category;
        let courses = make_expense(&k, 20, "Courses", 46.30, ExpenseType::Expense, "2026-10-11");
        let personal_id = Uuid::new_v4();
        let mut personal = make_project(&k, personal_id, "Mes dépenses", "EUR");
        let mut payload: ProjectPayload = decrypt_json(&k, &personal.payload).unwrap();
        payload.extra.insert("kind".into(), "personal".into());
        payload.set_budgets(&shared::Budgets {
            monthly: [(category_of(&courses).id, 100.0)].into_iter().collect(),
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

        let glance = month_glance(&state, personal_id, &cached_ledgers(&state, personal_id), &HashSet::new(), None, today).unwrap();
        assert!((glance.spent - 78.30).abs() < 1e-9);

        let uncounted = month_glance(&state, personal_id, &cached_ledgers(&state, personal_id), &HashSet::from([group_id]), None, today).unwrap();
        assert!((uncounted.spent - 46.30).abs() < 1e-9);

        let unopened = LocalStorageProject { cached_expenses: None, ..mine };
        let state = LocalStorageState { projects: vec![unopened], ..Default::default() };
        assert_eq!(month_glance(&state, personal_id, &cached_ledgers(&state, personal_id), &HashSet::new(), None, today), None);
    }

    /// The projects page passes every ledger it holds, the personal project's own included.
    #[test]
    fn the_personal_project_is_not_counted_again_as_a_group() {
        use crate::crypto::key_to_fragment;
        let k = test_key();
        let personal_id = Uuid::new_v4();
        let personal = LocalStorageProject {
            project_id: personal_id,
            encryption_key: Some(key_to_fragment(&k)),
            user_id: Some(7),
            cached_project: Some(make_project(&k, personal_id, "Mes dépenses", "EUR")),
            cached_users: Some(corsica(Some(7)).data.users.clone()),
            cached_expenses: Some(vec![make_expense(&k, 1, "Courses", 128.0, ExpenseType::Expense, "2026-10-11")]),
            cached_payments: Some(vec![
                make_payment(&k, 1, 1, 8, false, 128.0),
                make_payment(&k, 2, 1, 7, true, 32.0),
                make_payment(&k, 3, 1, 8, true, 96.0),
            ]),
            ..Default::default()
        };
        let state = LocalStorageState { projects: vec![personal], ..Default::default() };
        let page_ledgers = cached_ledgers(&state, Uuid::nil());
        assert_eq!(page_ledgers.len(), 1);
        let spent = |ledgers: &[GroupLedger]| {
            month_glance(&state, personal_id, ledgers, &HashSet::new(), None, day("2026-10-12")).unwrap().spent
        };

        assert_eq!(spent(&page_ledgers), spent(&[]));
    }

    #[test]
    fn a_ledger_carries_its_project_emoji() {
        use crate::crypto::{encrypt_json, key_to_fragment};
        let k = test_key();
        let id = Uuid::new_v4();
        let mut project = make_project(&k, id, "Ski", "EUR");
        let mut payload: ProjectPayload = decrypt_json(&k, &project.payload).unwrap();
        payload.set_emoji(Some("🎿"));
        project.payload = encrypt_json(&k, &payload).unwrap();
        let entry = LocalStorageProject {
            project_id: id,
            encryption_key: Some(key_to_fragment(&k)),
            cached_project: Some(project),
            cached_expenses: Some(vec![]),
            cached_payments: Some(vec![]),
            ..Default::default()
        };
        let state = LocalStorageState { projects: vec![entry], ..Default::default() };
        let ledgers = cached_ledgers(&state, Uuid::nil());
        assert_eq!(ledgers[0].emoji.as_deref(), Some("🎿"));
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

    fn held(id: Uuid) -> LocalStorageState {
        LocalStorageState {
            projects: vec![LocalStorageProject { project_id: id, ..Default::default() }],
            ..Default::default()
        }
    }

    fn answer(id: Uuid, version: i64, expenses: Option<Vec<Expense>>) -> ProjectSync {
        let k = test_key();
        ProjectSync {
            data_version: version,
            project: Some(make_project(&k, id, "Corse", "EUR")),
            users: Some(vec![make_user(&k, 1, "Ana")]),
            payments: expenses.as_ref().map(|_| vec![]),
            expenses,
            recurring: Some(vec![]),
            server_date: None,
            server_time: None,
            rotated: None,
        }
    }

    fn cafe() -> Vec<Expense> {
        vec![make_expense(&test_key(), 1, "Café", 3.0, ExpenseType::Expense, "2026-10-01")]
    }

    #[test]
    fn a_rotated_project_shows_no_ledger_from_its_old_cache() {
        let id = Uuid::new_v4();
        let mut state = held(id);
        crate::common::upsert_project_key(&mut state, id, crate::crypto::key_to_fragment(&test_key()));
        cache_sync(&mut state, &answer(id, 7, Some(cafe())));
        assert_eq!(synced_ledgers(&state, &[], Uuid::nil()).len(), 1, "a project the answer left out keeps its cache");
        let rotated = ProjectSync { data_version: -1, project: None, users: None, expenses: None, payments: None, recurring: None, server_date: None, server_time: None, rotated: Some(id) };
        assert!(synced_ledgers(&state, &[rotated], Uuid::nil()).is_empty());
    }

    #[test]
    fn a_full_answer_is_cached_at_its_version() {
        let id = Uuid::new_v4();
        let mut state = held(id);
        cache_sync(&mut state, &answer(id, 7, Some(cafe())));
        assert_eq!(known_version(&state.projects[0]), 7);
        assert_eq!(state.projects[0].cached_expenses.as_ref().map(Vec::len), Some(1));
    }

    #[test]
    fn an_unchanged_answer_keeps_the_cached_rows_and_their_version() {
        let id = Uuid::new_v4();
        let mut state = held(id);
        cache_sync(&mut state, &answer(id, 7, Some(cafe())));
        cache_sync(&mut state, &answer(id, 7, None));
        assert_eq!(known_version(&state.projects[0]), 7);
        assert_eq!(state.projects[0].cached_expenses.as_ref().map(Vec::len), Some(1));
    }

    #[test]
    fn a_project_this_device_does_not_hold_is_not_cached() {
        let mut state = held(Uuid::new_v4());
        cache_sync(&mut state, &answer(Uuid::new_v4(), 7, Some(cafe())));
        assert_eq!(state.projects.len(), 1);
        assert!(state.projects[0].cached_project.is_none());
    }
    mod hostile_payloads {
        use crate::common::hostile;
        use proptest::prelude::*;
        use shared::Budgets;
        use std::collections::HashMap;

        fn budgets() -> impl Strategy<Value = Budgets> {
            let category = prop_oneof![prop::sample::select(crate::categories::CHART_CATEGORIES.to_vec()).prop_map(String::from), any::<String>()];
            (prop::collection::btree_map(category, hostile::amount(), 0..6), any::<bool>()).prop_map(|(monthly, alert)| Budgets { monthly, alert })
        }

        fn rates() -> impl Strategy<Value = HashMap<String, f64>> {
            prop::collection::hash_map(prop_oneof![Just("EUR".to_string()), Just("USD".to_string()), any::<String>()], hostile::amount(), 0..4)
        }

        proptest! {
            #![proptest_config(ProptestConfig::with_cases(300))]

            #[test]
            fn any_group_and_personal_ledger_adds_up(group in hostile::expenses(), payments in hostile::payments(), users in hostile::users(), own in hostile::expenses(), me in proptest::option::of(hostile::id()), budgets in budgets(), rates in rates(), project in hostile::project_payload()) {
                crate::expenses::helpers::my_spending::exercise_hostile(group, payments, users, own, me, budgets, rates, project);
            }
        }
    }
}
