use chrono::{Datelike, Duration, Months, NaiveDate};
use shared::ExpenseType;
use std::collections::{HashMap, HashSet};
use uuid::Uuid;

use crate::categories::{ProjectCategories, ShownCategory};
use crate::decrypted::{DecryptedExpense, DecryptedPayment};

pub const OTHER: &str = crate::categories::custom::OTHER;
pub const TOP_CATEGORIES: usize = 5;
pub const MAX_BUCKETS: usize = 62;

#[derive(Clone, PartialEq, Debug)]
pub struct ProjectInfo {
    pub id: Uuid,
    pub name: String,
    pub currency: String,
    pub my_user_id: Option<i32>,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Scope {
    Group,
    Me,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Bucket {
    Day,
    Week,
    Month,
}

/// Fixed per category, never per rank, so a category keeps its colour when the filter changes.
/// The first seven pass the dataviz palette validator on adjacent pairs; see docs/charts.md.
pub fn category_color(category: &str) -> &'static str {
    match category {
        "Nourriture" => "oklch(62% 0.15 162)",
        "Transport" => "oklch(68% 0.14 230)",
        "Hébergement" => "oklch(72% 0.16 80)",
        "Loisirs" => "oklch(64% 0.18 25)",
        "Shopping" => "oklch(70% 0.15 300)",
        "Services" => "oklch(50% 0.13 260)",
        "Fêtes & Cadeaux" => "oklch(60% 0.13 55)",
        _ => "oklch(80% 0.01 165)",
    }
}

fn is_expense(e: &DecryptedExpense) -> bool {
    e.expense_type == ExpenseType::Expense
}

fn parse_date(date: &str) -> Option<NaiveDate> {
    NaiveDate::parse_from_str(date, "%Y-%m-%d").ok()
}

fn sort_desc<K>(data: &mut [(K, f64)]) {
    data.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
}

pub fn total_spend(expenses: &[DecryptedExpense]) -> f64 {
    expenses.iter().filter(|e| is_expense(e)).map(|e| e.amount).sum()
}

pub fn expense_count(expenses: &[DecryptedExpense]) -> usize {
    expenses.iter().filter(|e| is_expense(e)).count()
}

pub fn per_person(total: f64, participants: usize) -> f64 {
    if participants == 0 {
        0.0
    } else {
        total / participants as f64
    }
}

pub fn filter_by_date_range(
    expenses: &[DecryptedExpense],
    from: Option<NaiveDate>,
    to: Option<NaiveDate>,
) -> Vec<DecryptedExpense> {
    if from.is_none() && to.is_none() {
        return expenses.to_vec();
    }
    expenses
        .iter()
        .filter(|e| {
            parse_date(&e.date)
                .map(|d| from.is_none_or(|f| d >= f) && to.is_none_or(|t| d <= t))
                .unwrap_or(true)
        })
        .cloned()
        .collect()
}

pub fn in_projects(expenses: &[DecryptedExpense], projects: &HashSet<Uuid>) -> Vec<DecryptedExpense> {
    expenses.iter().filter(|e| projects.contains(&e.project_id)).cloned().collect()
}

/// What `user_id` paid (`is_debt == false`) or owes (`true`) per expense.
pub fn sum_by_expense(payments: &[DecryptedPayment], user_id: i32, is_debt: bool) -> HashMap<i32, f64> {
    let mut by_expense: HashMap<i32, f64> = HashMap::new();
    for p in payments.iter().filter(|p| p.user_id == user_id && p.is_debt == is_debt) {
        *by_expense.entry(p.expense_id).or_default() += p.amount;
    }
    by_expense
}

/// Each expense restated as my share of it, so every aggregate works unchanged on it. Expenses I
/// owe nothing on are dropped rather than zeroed: a zero would still count and still list.
pub fn with_my_share(expenses: &[DecryptedExpense], share: &HashMap<i32, f64>) -> Vec<DecryptedExpense> {
    expenses
        .iter()
        .filter(|e| is_expense(e))
        .filter_map(|e| match share.get(&e.id) {
            Some(amount) if *amount > 0.0 => Some(DecryptedExpense { amount: *amount, ..e.clone() }),
            _ => None,
        })
        .collect()
}

pub fn sum_over(expenses: &[DecryptedExpense], by_expense: &HashMap<i32, f64>) -> f64 {
    expenses.iter().filter(|e| is_expense(e)).filter_map(|e| by_expense.get(&e.id)).sum()
}

/// Each project's own categories, by project. Across projects every category rolls up into its
/// built-in parent: two groups that each have a "Courses" are not two slices of the same name.
#[derive(Clone, PartialEq, Debug, Default)]
pub struct ChartCategories {
    pub projects: HashMap<Uuid, ProjectCategories>,
    pub roll_up: bool,
}

impl ChartCategories {
    pub fn of(&self, e: &DecryptedExpense) -> ShownCategory {
        let shown = self.unrolled(e);
        if self.roll_up { shown.rolled_up() } else { shown }
    }

    pub fn unrolled(&self, e: &DecryptedExpense) -> ShownCategory {
        match self.projects.get(&e.project_id) {
            Some(c) => c.resolve(e.category.as_deref(), &e.name),
            None => ProjectCategories::default().resolve(e.category.as_deref(), &e.name),
        }
    }

    pub fn other(&self) -> ShownCategory {
        match self.projects.values().next() {
            Some(c) if !self.roll_up && self.projects.len() == 1 => c.shown_builtin(OTHER),
            _ => ShownCategory::builtin(OTHER),
        }
    }
}

fn by_value(a: &(ShownCategory, f64), b: &(ShownCategory, f64)) -> std::cmp::Ordering {
    b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal).then_with(|| a.0.cmp(&b.0))
}

pub fn aggregate_by_category(expenses: &[DecryptedExpense], lens: &ChartCategories) -> Vec<(ShownCategory, f64)> {
    let mut map: HashMap<ShownCategory, f64> = HashMap::new();
    for e in expenses.iter().filter(|e| is_expense(e)) {
        *map.entry(lens.of(e)).or_default() += e.amount;
    }
    let mut result: Vec<(ShownCategory, f64)> = map.into_iter().collect();
    result.sort_by(by_value);
    result
}

/// The five largest categories, then everything else as one `other` slice, last. A custom
/// category follows its parent when both are shown.
pub fn fold_categories(data: &[(ShownCategory, f64)], other: &ShownCategory) -> Vec<(ShownCategory, f64)> {
    let mut sorted = data.to_vec();
    sorted.sort_by(by_value);
    let kept: Vec<(ShownCategory, f64)> =
        sorted.iter().take(TOP_CATEGORIES).filter(|(k, _)| k != other).cloned().collect();
    let rest: f64 = sorted.iter().filter(|(k, _)| !kept.iter().any(|(c, _)| c == k)).map(|(_, v)| v).sum();
    let mut out: Vec<(ShownCategory, f64)> = Vec::new();
    for (category, value) in kept.iter().filter(|(c, _)| !is_child(c, &kept)) {
        out.push((category.clone(), *value));
        if !category.is_custom() {
            out.extend(kept.iter().filter(|(c, _)| is_child(c, &kept) && c.parent == category.id).cloned());
        }
    }
    if rest > 0.0 {
        out.push((other.clone(), rest));
    }
    out
}

/// Whether the legend draws `category` indented under its parent.
pub fn is_child(category: &ShownCategory, slices: &[(ShownCategory, f64)]) -> bool {
    category.is_custom() && slices.iter().any(|(p, _)| !p.is_custom() && p.id == category.parent)
}

pub fn folded_category(expense: &DecryptedExpense, shown: &[(ShownCategory, f64)], lens: &ChartCategories) -> ShownCategory {
    let category = lens.of(expense);
    if shown.iter().any(|(c, _)| *c == category) {
        category
    } else {
        lens.other()
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct CategoryShare {
    pub category: ShownCategory,
    pub mine: f64,
    pub group: f64,
}

pub fn category_shares(group: &[DecryptedExpense], my_share: &HashMap<i32, f64>, lens: &ChartCategories) -> Vec<CategoryShare> {
    let shown = fold_categories(&aggregate_by_category(group, lens), &lens.other());
    let mut mine: HashMap<ShownCategory, f64> = HashMap::new();
    for e in group.iter().filter(|e| is_expense(e)) {
        if let Some(share) = my_share.get(&e.id) {
            *mine.entry(folded_category(e, &shown, lens)).or_default() += share;
        }
    }
    shown
        .into_iter()
        .map(|(category, group)| {
            let mine = mine.get(&category).copied().unwrap_or(0.0);
            CategoryShare { category, mine, group }
        })
        .collect()
}

/// Newest first.
pub fn expenses_in_category(
    expenses: &[DecryptedExpense],
    category: &ShownCategory,
    shown: &[(ShownCategory, f64)],
    lens: &ChartCategories,
) -> Vec<DecryptedExpense> {
    let mut out: Vec<DecryptedExpense> =
        expenses.iter().filter(|e| is_expense(e) && folded_category(e, shown, lens) == *category).cloned().collect();
    out.sort_by(|a, b| b.date.cmp(&a.date).then(b.id.cmp(&a.id)));
    out
}

#[derive(Clone, PartialEq, Debug)]
pub struct PersonShare {
    pub user_id: i32,
    pub paid: f64,
    pub share: f64,
}

/// From the payment rows, not `author_id`: the author is whoever typed the expense in.
pub fn paid_vs_share(expenses: &[DecryptedExpense], payments: &[DecryptedPayment]) -> Vec<PersonShare> {
    let ids: HashSet<i32> = expenses.iter().filter(|e| is_expense(e)).map(|e| e.id).collect();
    let mut by_user: HashMap<i32, (f64, f64)> = HashMap::new();
    for p in payments.iter().filter(|p| ids.contains(&p.expense_id)) {
        let entry = by_user.entry(p.user_id).or_default();
        if p.is_debt {
            entry.1 += p.amount;
        } else {
            entry.0 += p.amount;
        }
    }
    let mut rows: Vec<PersonShare> = by_user
        .into_iter()
        .filter(|(_, (paid, share))| *paid > 0.0 || *share > 0.0)
        .map(|(user_id, (paid, share))| PersonShare { user_id, paid, share })
        .collect();
    rows.sort_by(|a, b| {
        (b.paid - b.share).partial_cmp(&(a.paid - a.share)).unwrap_or(std::cmp::Ordering::Equal).then(a.user_id.cmp(&b.user_id))
    });
    rows
}

pub fn auto_bucket(from: NaiveDate, to: NaiveDate) -> Bucket {
    let days = (to - from).num_days() + 1;
    if days <= 31 {
        Bucket::Day
    } else if days <= 183 {
        Bucket::Week
    } else {
        Bucket::Month
    }
}

pub fn bucket_start(date: NaiveDate, bucket: Bucket) -> NaiveDate {
    match bucket {
        Bucket::Day => date,
        Bucket::Week => date.checked_sub_signed(Duration::days(date.weekday().num_days_from_monday() as i64)).unwrap_or(NaiveDate::MIN),
        Bucket::Month => date.with_day(1).unwrap_or(date),
    }
}

fn previous_bucket(start: NaiveDate, bucket: Bucket) -> Option<NaiveDate> {
    match bucket {
        Bucket::Day => start.checked_sub_signed(Duration::days(1)),
        Bucket::Week => start.checked_sub_signed(Duration::days(7)),
        Bucket::Month => start.checked_sub_months(Months::new(1)),
    }
}

/// Arithmetic, not by walking: a tampered date in year 1 must not spin a loop.
pub fn bucket_count(from: NaiveDate, to: NaiveDate, bucket: Bucket) -> usize {
    if from > to {
        return 0;
    }
    let n = match bucket {
        Bucket::Day => (to - from).num_days() + 1,
        Bucket::Week => (bucket_start(to, bucket) - bucket_start(from, bucket)).num_days() / 7 + 1,
        Bucket::Month => {
            (to.year() - from.year()) as i64 * 12 + to.month() as i64 - from.month() as i64 + 1
        }
    };
    n.max(0) as usize
}

/// Every bucket start covering `from..=to`, the latest `MAX_BUCKETS` of them.
pub fn bucket_starts(from: NaiveDate, to: NaiveDate, bucket: Bucket) -> Vec<NaiveDate> {
    if from > to {
        return vec![];
    }
    let first = bucket_start(from, bucket);
    let mut current = Some(bucket_start(to, bucket));
    let mut out = vec![];
    while let Some(d) = current {
        if d < first || out.len() == MAX_BUCKETS {
            break;
        }
        out.push(d);
        current = previous_bucket(d, bucket);
    }
    out.reverse();
    out
}

pub fn sum_by_bucket(expenses: &[DecryptedExpense], starts: &[NaiveDate], bucket: Bucket) -> Vec<f64> {
    let index: HashMap<NaiveDate, usize> = starts.iter().enumerate().map(|(i, d)| (*d, i)).collect();
    let mut out = vec![0.0; starts.len()];
    for e in expenses.iter().filter(|e| is_expense(e)) {
        if let Some(i) = parse_date(&e.date).and_then(|d| index.get(&bucket_start(d, bucket))) {
            out[*i] += e.amount;
        }
    }
    out
}

pub fn running_total(values: &[f64]) -> Vec<f64> {
    values
        .iter()
        .scan(0.0, |acc, v| {
            *acc += v;
            Some(*acc)
        })
        .collect()
}

pub fn average(values: &[f64]) -> f64 {
    if values.is_empty() {
        0.0
    } else {
        values.iter().sum::<f64>() / values.len() as f64
    }
}

/// My balance at the end of each bucket. Every expense type counts and history before the first
/// bucket carries in, so the last point equals the balance tab.
pub fn balance_by_bucket(
    expenses: &[DecryptedExpense],
    payments: &[DecryptedPayment],
    user_id: i32,
    starts: &[NaiveDate],
    bucket: Bucket,
) -> Vec<f64> {
    let dates: HashMap<i32, NaiveDate> =
        expenses.iter().filter_map(|e| parse_date(&e.date).map(|d| (e.id, bucket_start(d, bucket)))).collect();
    let deltas: Vec<(NaiveDate, f64)> = payments
        .iter()
        .filter(|p| p.user_id == user_id)
        .filter_map(|p| dates.get(&p.expense_id).map(|d| (*d, if p.is_debt { -p.amount } else { p.amount })))
        .collect();
    starts.iter().map(|start| deltas.iter().filter(|(d, _)| d <= start).map(|(_, v)| v).sum()).collect()
}

#[derive(Clone, PartialEq, Debug)]
pub struct CurrencyTotals {
    pub currency: String,
    pub rows: Vec<(String, f64)>,
}

/// Grouped per currency and never summed across: each project's amounts are in its own currency.
pub fn totals_by_project(expenses: &[DecryptedExpense], projects: &[ProjectInfo]) -> Vec<CurrencyTotals> {
    let mut by_project: HashMap<Uuid, f64> = HashMap::new();
    for e in expenses.iter().filter(|e| is_expense(e)) {
        *by_project.entry(e.project_id).or_default() += e.amount;
    }
    let mut groups: Vec<CurrencyTotals> = vec![];
    for p in projects {
        let Some(total) = by_project.get(&p.id).copied().filter(|t| *t > 0.0) else {
            continue;
        };
        match groups.iter_mut().find(|g| g.currency == p.currency) {
            Some(g) => g.rows.push((p.name.clone(), total)),
            None => groups.push(CurrencyTotals { currency: p.currency.clone(), rows: vec![(p.name.clone(), total)] }),
        }
    }
    for g in &mut groups {
        sort_desc(&mut g.rows);
    }
    groups.sort_by(|a, b| {
        let (ta, tb): (f64, f64) = (a.rows.iter().map(|r| r.1).sum(), b.rows.iter().map(|r| r.1).sum());
        tb.partial_cmp(&ta).unwrap_or(std::cmp::Ordering::Equal)
    });
    groups
}

/// Most used first.
pub fn currencies(projects: &[ProjectInfo]) -> Vec<String> {
    let mut counts: Vec<(String, usize)> = vec![];
    for p in projects {
        match counts.iter_mut().find(|(c, _)| *c == p.currency) {
            Some((_, n)) => *n += 1,
            None => counts.push((p.currency.clone(), 1)),
        }
    }
    counts.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
    counts.into_iter().map(|(c, _)| c).collect()
}

/// Whole units, sign handled by the caller ([`signed_money`]).
pub fn money(value: f64, currency: &str) -> String {
    let body = crate::common::whole_amount(value, currency);
    if value.round() < 0.0 {
        format!("−{body}")
    } else {
        body
    }
}

pub fn signed_money(value: f64, currency: &str) -> String {
    let body = money(value.abs(), currency);
    if value.round() > 0.0 {
        format!("+{body}")
    } else if value.round() < 0.0 {
        format!("−{body}")
    } else {
        body
    }
}

pub fn percent(part: f64, whole: f64) -> i64 {
    if whole <= 0.0 {
        0
    } else {
        (part / whole * 100.0).round() as i64
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDateTime;

    fn expense(id: i32, amount: f64, date: &str, category: &str) -> DecryptedExpense {
        DecryptedExpense {
            id,
            author_id: Some(1),
            project_id: Uuid::nil(),
            created_at: NaiveDateTime::default(),
            name: format!("e{id}"),
            description: None,
            amount,
            expense_type: ExpenseType::Expense,
            date: date.to_string(),
            category: Some(category.to_string()),
            source_currency: None,
            source_amount: None,
            rate: None,
            recurring_id: None,
            estimate: false,
        }
    }

    fn payment(expense_id: i32, user_id: i32, is_debt: bool, amount: f64) -> DecryptedPayment {
        DecryptedPayment { id: 0, expense_id, user_id, is_debt, amount, created_at: NaiveDateTime::default() }
    }

    fn day(d: &str) -> NaiveDate {
        parse_date(d).unwrap()
    }

    #[test]
    fn paid_vs_share_credits_the_payer_not_the_author() {
        let e = expense(1, 100.0, "2025-01-01", "Nourriture");
        let payments = [payment(1, 2, false, 100.0), payment(1, 1, true, 50.0), payment(1, 2, true, 50.0)];
        let rows = paid_vs_share(&[e], &payments);
        assert_eq!(rows[0], PersonShare { user_id: 2, paid: 100.0, share: 50.0 });
        assert_eq!(rows[1], PersonShare { user_id: 1, paid: 0.0, share: 50.0 });
    }

    #[test]
    fn paid_vs_share_ignores_transfers_and_other_ranges() {
        let mut transfer = expense(2, 30.0, "2025-01-01", "Autres");
        transfer.expense_type = ExpenseType::Transfer;
        let payments = [payment(2, 1, false, 30.0), payment(2, 2, true, 30.0), payment(99, 1, false, 5.0)];
        assert!(paid_vs_share(&[transfer], &payments).is_empty());
    }

    #[test]
    fn per_person_divides_by_participants() {
        assert_eq!(per_person(100.0, 4), 25.0);
        assert_eq!(per_person(100.0, 0), 0.0);
    }

    #[test]
    fn project_totals_never_add_currencies() {
        let mut eur = expense(1, 100.0, "2025-01-01", "Nourriture");
        eur.project_id = Uuid::from_u128(1);
        let mut usd = expense(2, 100.0, "2025-01-01", "Nourriture");
        usd.project_id = Uuid::from_u128(2);
        let projects = [
            ProjectInfo { id: Uuid::from_u128(1), name: "Flat".into(), currency: "EUR".into(), my_user_id: None },
            ProjectInfo { id: Uuid::from_u128(2), name: "NYC".into(), currency: "USD".into(), my_user_id: None },
        ];
        let groups = totals_by_project(&[eur, usd], &projects);
        assert_eq!(groups.len(), 2);
        assert!(groups.iter().all(|g| g.rows.len() == 1 && g.rows[0].1 == 100.0));
    }

    #[test]
    fn a_ten_day_trip_is_one_bar_per_day() {
        let trip: Vec<_> = (12..22).map(|d| expense(d, 10.0, &format!("2026-09-{d}"), "Nourriture")).collect();
        let (from, to) = (day("2026-09-12"), day("2026-09-21"));
        let bucket = auto_bucket(from, to);
        assert_eq!(bucket, Bucket::Day);
        let starts = bucket_starts(from, to, bucket);
        assert_eq!(sum_by_bucket(&trip, &starts, bucket), vec![10.0; 10]);
    }

    #[test]
    fn auto_bucket_thresholds() {
        let from = day("2026-01-01");
        assert_eq!(auto_bucket(from, day("2026-01-31")), Bucket::Day);
        assert_eq!(auto_bucket(from, day("2026-02-01")), Bucket::Week);
        assert_eq!(auto_bucket(from, day("2026-07-02")), Bucket::Week);
        assert_eq!(auto_bucket(from, day("2026-07-03")), Bucket::Month);
    }

    #[test]
    fn weeks_start_on_monday() {
        assert_eq!(bucket_start(day("2026-09-27"), Bucket::Week), day("2026-09-21"));
        assert_eq!(bucket_start(day("2026-09-21"), Bucket::Week), day("2026-09-21"));
    }

    #[test]
    fn empty_buckets_are_kept_as_zero() {
        let starts = bucket_starts(day("2026-01-01"), day("2026-03-31"), Bucket::Month);
        let spend = sum_by_bucket(&[expense(1, 5.0, "2026-03-02", "Nourriture")], &starts, Bucket::Month);
        assert_eq!(spend, vec![0.0, 0.0, 5.0]);
    }

    #[test]
    fn bucket_count_is_arithmetic_and_starts_are_capped() {
        let (from, to) = (day("0001-01-01"), day("2026-09-27"));
        assert!(bucket_count(from, to, Bucket::Day) > 700_000);
        assert_eq!(bucket_starts(from, to, Bucket::Day).len(), MAX_BUCKETS);
        assert_eq!(bucket_starts(from, to, Bucket::Day).last(), Some(&to));
        assert_eq!(bucket_count(day("2025-11-15"), day("2026-02-01"), Bucket::Month), 4);
        assert_eq!(bucket_count(to, from, Bucket::Day), 0);
    }

    #[test]
    fn balance_carries_history_in_and_counts_transfers() {
        let mut settle = expense(3, 50.0, "2026-09-20", "Autres");
        settle.expense_type = ExpenseType::Transfer;
        let expenses = [expense(1, 100.0, "2026-08-01", "Nourriture"), expense(2, 40.0, "2026-09-12", "Nourriture"), settle];
        let payments = [
            payment(1, 1, false, 100.0),
            payment(1, 1, true, 50.0),
            payment(2, 2, false, 40.0),
            payment(2, 1, true, 20.0),
            payment(3, 2, false, 50.0),
            payment(3, 1, true, 50.0),
        ];
        let starts = bucket_starts(day("2026-09-12"), day("2026-09-21"), Bucket::Day);
        let balance = balance_by_bucket(&expenses, &payments, 1, &starts, Bucket::Day);
        assert_eq!(balance.first(), Some(&30.0));
        assert_eq!(balance.last(), Some(&-20.0));
    }

    #[test]
    fn fold_keeps_five_then_other_last() {
        let data: Vec<(ShownCategory, f64)> = [
            ("Nourriture", 60.0),
            ("Autres", 50.0),
            ("Transport", 40.0),
            ("Hébergement", 30.0),
            ("Loisirs", 20.0),
            ("Shopping", 10.0),
            ("Services", 5.0),
        ]
        .into_iter()
        .map(|(c, v)| (ShownCategory::builtin(crate::categories::custom::builtin_id(c).unwrap()), v))
        .collect();
        let folded = fold_categories(&data, &ShownCategory::builtin(OTHER));
        let names: Vec<_> = folded.iter().map(|(k, _)| k.id.as_str()).collect();
        assert_eq!(names, ["Nourriture", "Transport", "Hébergement", "Loisirs", "Autres"]);
        assert_eq!(folded.last().unwrap().1, 65.0);
        assert_eq!(folded.iter().map(|(_, v)| v).sum::<f64>(), 215.0);
    }

    #[test]
    fn category_shares_split_mine_from_the_group() {
        let group = [expense(1, 100.0, "2026-01-01", "Nourriture"), expense(2, 60.0, "2026-01-01", "Transport")];
        let share = HashMap::from([(1, 25.0)]);
        let rows = category_shares(&group, &share, &ChartCategories::default());
        assert_eq!(rows[0], CategoryShare { category: ShownCategory::builtin("Nourriture"), mine: 25.0, group: 100.0 });
        assert_eq!(rows[1], CategoryShare { category: ShownCategory::builtin("Transport"), mine: 0.0, group: 60.0 });
    }

    fn lens(categories: serde_json::Value, roll_up: bool) -> ChartCategories {
        ChartCategories { projects: HashMap::from([(Uuid::nil(), ProjectCategories::from_value(Some(&categories)))]), roll_up }
    }

    #[test]
    fn a_custom_category_is_its_own_slice_under_its_parent_and_rolls_up_across_projects() {
        let list = serde_json::json!([{ "id": "c-0000000a", "name": "Courses bio", "parent": "Nourriture" }]);
        let expenses = [
            expense(1, 300.0, "2026-01-01", "Nourriture"),
            expense(2, 200.0, "2026-01-01", "Transport"),
            expense(3, 96.0, "2026-01-01", "c-0000000a"),
        ];
        let one = lens(list.clone(), false);
        let slices = fold_categories(&aggregate_by_category(&expenses, &one), &one.other());
        let ids: Vec<&str> = slices.iter().map(|(c, _)| c.id.as_str()).collect();
        assert_eq!(ids, ["Nourriture", "c-0000000a", "Transport"]);
        assert!(is_child(&slices[1].0, &slices));
        assert_eq!(slices[1].0.parent, "Nourriture");
        let all = lens(list, true);
        let rolled = aggregate_by_category(&expenses, &all);
        assert_eq!(rolled[0], (ShownCategory::builtin("Nourriture"), 396.0));
        assert_eq!(rolled.len(), 2);
    }

    #[test]
    fn a_renamed_other_is_still_the_folded_tail() {
        let one = lens(serde_json::json!([{ "id": "Autres", "name": "Divers" }]), false);
        let expenses: Vec<_> = ["Nourriture", "Transport", "Hébergement", "Loisirs", "Shopping", "Services", "Autres"]
            .iter()
            .enumerate()
            .map(|(i, c)| expense(i as i32, 100.0 - i as f64, "2026-01-01", c))
            .collect();
        let slices = fold_categories(&aggregate_by_category(&expenses, &one), &one.other());
        let last = slices.last().unwrap();
        assert_eq!((last.0.name.as_deref(), last.1), (Some("Divers"), 95.0 + 94.0));
        assert_eq!(slices.iter().filter(|(c, _)| c.id == "Autres").count(), 1);
    }

    #[test]
    fn my_share_replaces_the_amount_and_drops_zero() {
        let expenses = [expense(7, 200.0, "2025-01-01", "Nourriture"), expense(8, 50.0, "2025-01-01", "Nourriture")];
        let out = with_my_share(&expenses, &HashMap::from([(7, 100.0), (8, 0.0)]));
        assert_eq!(out.len(), 1);
        assert_eq!((out[0].id, out[0].amount), (7, 100.0));
    }

    #[test]
    fn drilldown_other_collects_the_folded_tail() {
        let expenses: Vec<_> = ["Nourriture", "Transport", "Hébergement", "Loisirs", "Shopping", "Services"]
            .iter()
            .enumerate()
            .map(|(i, c)| expense(i as i32, 100.0 - i as f64, "2026-01-01", c))
            .collect();
        let lens = ChartCategories::default();
        let shown = fold_categories(&aggregate_by_category(&expenses, &lens), &lens.other());
        let other = expenses_in_category(&expenses, &lens.other(), &shown, &lens);
        assert_eq!(other.len(), 1);
        assert_eq!(other[0].category.as_deref(), Some("Services"));
    }

    #[test]
    fn filter_bounds_are_inclusive() {
        let expenses = [
            expense(1, 1.0, "2024-12-31", "Autres"),
            expense(2, 1.0, "2025-01-01", "Autres"),
            expense(3, 1.0, "2025-12-31", "Autres"),
            expense(4, 1.0, "2026-01-01", "Autres"),
        ];
        let kept = filter_by_date_range(&expenses, Some(day("2025-01-01")), Some(day("2025-12-31")));
        assert_eq!(kept.iter().map(|e| e.id).collect::<Vec<_>>(), [2, 3]);
    }

    #[test]
    fn currencies_most_used_first() {
        let p = |c: &str| ProjectInfo { id: Uuid::nil(), name: String::new(), currency: c.into(), my_user_id: None };
        assert_eq!(currencies(&[p("USD"), p("EUR"), p("EUR")]), ["EUR", "USD"]);
    }

    #[test]
    fn money_formats() {
        assert_eq!(money(1728.4, "EUR"), "1\u{a0}728\u{a0}EUR");
        assert_eq!(money(640.0, "USD"), "640\u{a0}USD");
        assert_eq!(money(12.0, ""), "12");
        assert_eq!(money(-0.3, "EUR"), "0\u{a0}EUR");
        assert_eq!(signed_money(185.0, "EUR"), "+185\u{a0}EUR");
        assert_eq!(signed_money(-285.0, "EUR"), "−285\u{a0}EUR");
        assert_eq!(signed_money(0.2, "EUR"), "0\u{a0}EUR");
    }

    #[test]
    fn running_total_and_average() {
        assert_eq!(running_total(&[1.0, 2.0, 3.0]), [1.0, 3.0, 6.0]);
        assert_eq!(average(&[]), 0.0);
        assert_eq!(average(&[2.0, 4.0]), 3.0);
    }
}
