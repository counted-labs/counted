//! The personal project's monthly budgets (designs P2, P3, R3): what each category's share of the
//! month comes to, measured against its ceiling. Everything here runs on the device.

use chrono::NaiveDate;
use shared::{Budgets, ExpenseType};
use std::collections::HashMap;

use crate::categories::CHART_CATEGORIES;
use crate::expenses::helpers::my_spending::{in_month, shift_month, ShareRow};
use crate::expenses::helpers::project_data::RowExpense;

/// My spending per category in `month`: own expenses, plus counted shares in the personal currency.
/// A share with no rate is left out, as it is from the month's total.
pub fn spent_by_category(own: &[RowExpense], shares: &[ShareRow], month: NaiveDate) -> HashMap<&'static str, f64> {
    let mut spent: HashMap<&'static str, f64> = HashMap::new();
    for r in own.iter().filter(|r| r.expense.expense_type == ExpenseType::Expense && in_month(r.date, month)) {
        *spent.entry(r.category).or_default() += r.expense.amount;
    }
    for s in shares.iter().filter(|s| s.counted && in_month(s.row.date, month)) {
        if let Some(amount) = s.converted {
            *spent.entry(s.row.category).or_default() += amount;
        }
    }
    spent
}

/// The average of the three months before `month`, shown next to each ceiling while it is set.
pub fn average_by_category(own: &[RowExpense], shares: &[ShareRow], month: NaiveDate) -> HashMap<&'static str, f64> {
    let mut sum: HashMap<&'static str, f64> = HashMap::new();
    for back in 1..=3 {
        for (category, amount) in spent_by_category(own, shares, shift_month(month, -back)) {
            *sum.entry(category).or_default() += amount;
        }
    }
    sum.into_iter().map(|(c, total)| (c, total / 3.0)).collect()
}

#[derive(Clone, Debug, PartialEq)]
pub struct BudgetLine {
    pub category: &'static str,
    pub spent: f64,
    pub limit: f64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Meter {
    Left(f64),
    /// From 80 % up to the ceiling itself, as a whole percentage.
    Near(u32),
    Over(f64),
}

impl BudgetLine {
    pub fn ratio(&self) -> f64 {
        self.spent / self.limit
    }

    pub fn meter(&self) -> Meter {
        let ratio = self.ratio();
        if ratio > 1.0 {
            Meter::Over(self.spent - self.limit)
        } else if ratio >= ALERT_RATIO {
            Meter::Near((ratio * 100.0).round() as u32)
        } else {
            Meter::Left(self.limit - self.spent)
        }
    }
}

pub const ALERT_RATIO: f64 = 0.8;

/// One line per category with a ceiling, the largest ceilings first.
pub fn budget_lines(budgets: &Budgets, spent: &HashMap<&'static str, f64>) -> Vec<BudgetLine> {
    let mut lines: Vec<BudgetLine> = CHART_CATEGORIES
        .iter()
        .filter_map(|c| {
            let limit = budgets.monthly.get(*c).copied().filter(|l| *l > 0.0)?;
            Some(BudgetLine { category: c, spent: spent.get(c).copied().unwrap_or(0.0), limit })
        })
        .collect();
    lines.sort_by(|a, b| b.limit.partial_cmp(&a.limit).unwrap_or(std::cmp::Ordering::Equal));
    lines
}

/// The threshold an added amount crosses, if any: the ceiling outranks the 80 % mark.
pub fn crossed(spent: f64, added: f64, limit: f64) -> Option<Meter> {
    if limit <= 0.0 || added <= 0.0 {
        return None;
    }
    let after = BudgetLine { category: "", spent: spent + added, limit };
    match after.meter() {
        Meter::Over(by) if spent <= limit => Some(Meter::Over(by)),
        Meter::Near(pct) if spent < limit * ALERT_RATIO => Some(Meter::Near(pct)),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::test_fixtures::{make_expense, test_key};
    use crate::decrypted::decrypt_expense;

    fn day(s: &str) -> NaiveDate {
        NaiveDate::parse_from_str(s, "%Y-%m-%d").unwrap()
    }

    fn own(id: i32, name: &str, amount: f64, kind: ExpenseType, date: &str) -> RowExpense {
        let k = test_key();
        RowExpense::from(decrypt_expense(&k, &make_expense(&k, id, name, amount, kind, date)).unwrap())
    }

    fn share(id: i32, name: &str, converted: Option<f64>, counted: bool, date: &str) -> ShareRow {
        ShareRow {
            row: own(id, name, 100.0, ExpenseType::Expense, date),
            project_id: uuid::Uuid::nil(),
            group: "Corsica".to_string(),
            currency: "EUR".to_string(),
            share: 25.0,
            converted,
            payer: None,
            counted,
        }
    }

    #[test]
    fn a_category_adds_own_expenses_and_counted_converted_shares() {
        let rows = [
            own(1, "Courses", 40.0, ExpenseType::Expense, "2026-10-02"),
            own(2, "Salaire", 2000.0, ExpenseType::Gain, "2026-10-01"),
            own(3, "Courses", 60.0, ExpenseType::Expense, "2026-09-30"),
        ];
        let shares = [
            share(10, "Courses", Some(25.0), true, "2026-10-05"),
            share(11, "Courses", Some(30.0), false, "2026-10-05"),
            share(12, "Courses", None, true, "2026-10-05"),
        ];
        let spent = spent_by_category(&rows, &shares, day("2026-10-01"));
        assert_eq!(spent.get("Nourriture"), Some(&65.0));
        assert_eq!(spent.len(), 1, "a gain is not spending");
    }

    #[test]
    fn the_average_covers_the_three_months_before() {
        let rows = [
            own(1, "Taxi", 30.0, ExpenseType::Expense, "2026-09-10"),
            own(2, "Taxi", 60.0, ExpenseType::Expense, "2026-07-10"),
            own(3, "Taxi", 300.0, ExpenseType::Expense, "2026-06-10"),
            own(4, "Taxi", 900.0, ExpenseType::Expense, "2026-10-10"),
        ];
        let avg = average_by_category(&rows, &[], day("2026-10-01"));
        assert_eq!(avg.get("Transport"), Some(&30.0));
    }

    #[test]
    fn the_meter_leaves_warns_then_overflows() {
        let line = |spent| BudgetLine { category: "Nourriture", spent, limit: 400.0 };
        assert_eq!(line(312.0).meter(), Meter::Left(88.0));
        assert_eq!(line(380.0).meter(), Meter::Near(95));
        assert_eq!(line(400.0).meter(), Meter::Near(100));
        assert_eq!(line(422.0).meter(), Meter::Over(22.0));
    }

    #[test]
    fn lines_follow_the_set_ceilings_largest_first() {
        let budgets = Budgets {
            monthly: [("Loisirs", 120.0), ("Hébergement", 550.0), ("Nourriture", 400.0), ("Transport", 0.0), ("Inconnue", 50.0)]
                .into_iter()
                .map(|(c, l)| (c.to_string(), l))
                .collect(),
            alert: true,
        };
        let spent = HashMap::from([("Loisirs", 142.0)]);
        let lines = budget_lines(&budgets, &spent);
        let got: Vec<(&str, f64)> = lines.iter().map(|l| (l.category, l.spent)).collect();
        assert_eq!(got, vec![("Hébergement", 0.0), ("Nourriture", 0.0), ("Loisirs", 142.0)]);
    }

    #[test]
    fn only_the_crossing_itself_warns() {
        assert_eq!(crossed(300.0, 30.0, 400.0), Some(Meter::Near(83)));
        assert_eq!(crossed(330.0, 10.0, 400.0), None, "already past 80 %");
        assert_eq!(crossed(390.0, 20.0, 400.0), Some(Meter::Over(10.0)));
        assert_eq!(crossed(300.0, 200.0, 400.0), Some(Meter::Over(100.0)), "both at once: the ceiling wins");
        assert_eq!(crossed(410.0, 5.0, 400.0), None, "already over");
        assert_eq!(crossed(0.0, 50.0, 0.0), None);
    }
}
