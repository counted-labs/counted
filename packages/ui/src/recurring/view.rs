use chrono::NaiveDate;
use shared::ExpenseType;
use uuid::Uuid;

use super::model::{DecryptedRule, Ends, Freq, RecurringPayload, Rule, Side, Template};
use super::schedule::{is_finished, nth_date, per_month, total, upcoming};
use super::split::side_amounts;
use crate::expenses::helpers::expense_modal_helpers::{Conversion, ValidatedExpense};

/// Today's rate may differ this much from the one a rule uses before it is flagged.
pub const DRIFT_THRESHOLD: f64 = 0.05;

/// What the Repeat picker edits; the anchor comes from the form's date.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Repeat {
    pub freq: Freq,
    pub interval: u32,
    pub ends: Ends,
    pub variable: bool,
}

impl Repeat {
    pub fn monthly() -> Self {
        Self { freq: Freq::Monthly, interval: 1, ends: Ends::Never, variable: false }
    }

    pub fn rule(&self, anchor: NaiveDate) -> Rule {
        Rule { freq: self.freq, interval: self.interval.max(1), anchor, ends: self.ends }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum RuleState {
    Active,
    Paused,
    Finished,
}

pub fn state(p: &RecurringPayload) -> RuleState {
    if is_finished(&p.rule, p.next) {
        RuleState::Finished
    } else if p.paused {
        RuleState::Paused
    } else {
        RuleState::Active
    }
}

pub fn next_date(p: &RecurringPayload) -> Option<NaiveDate> {
    upcoming(&p.rule, p.next, 1).first().copied()
}

/// Active first by next date, then paused, then finished.
pub fn sorted(rules: &[DecryptedRule]) -> Vec<DecryptedRule> {
    let mut out = rules.to_vec();
    out.sort_by_key(|r| (state(&r.payload), next_date(&r.payload), r.payload.template.name.clone()));
    out
}

/// "2 of 4" for a rule with a fixed count: how many were written, out of how many.
pub fn progress(p: &RecurringPayload) -> Option<(u32, u32)> {
    total(&p.rule).map(|t| (p.next.min(t), t))
}

fn counts(r: &DecryptedRule) -> bool {
    state(&r.payload) == RuleState::Active && r.payload.template.expense_type() == ExpenseType::Expense
}

pub fn monthly_total(rules: &[DecryptedRule]) -> f64 {
    rules
        .iter()
        .filter(|r| counts(r))
        .map(|r| r.payload.template.amount * per_month(&r.payload.rule))
        .sum()
}

pub fn my_monthly_share(rules: &[DecryptedRule], user_id: i32) -> f64 {
    rules
        .iter()
        .filter(|r| counts(r))
        .map(|r| {
            let t = &r.payload.template;
            let mine: f64 = side_amounts(&t.debtors, t.amount, 0)
                .iter()
                .filter(|(id, _)| *id == user_id)
                .map(|(_, a)| a)
                .sum();
            mine * per_month(&r.payload.rule)
        })
        .sum()
}

/// The relative move of today's rate against the one the rule last settled on, when it is beyond
/// [`DRIFT_THRESHOLD`].
pub fn rate_drift(t: &Template, today_rate: f64) -> Option<f64> {
    let base = t.rate_ack.or(t.rate).filter(|r| shared::is_valid_rate(*r))?;
    let drift = today_rate / base - 1.0;
    (drift.abs() > DRIFT_THRESHOLD).then_some(drift)
}

/// The template a form describes. The amount is frozen in the project currency.
pub fn template_from_form(
    form: &ValidatedExpense,
    expense_type: &ExpenseType,
    category: Option<String>,
    conversion: Option<&Conversion>,
    payers: Side,
    debtors: Side,
    variable: bool,
) -> Template {
    Template {
        name: form.name.clone(),
        expense_type: expense_type.as_str().to_string(),
        category,
        amount: form.total,
        payers,
        debtors,
        source_currency: conversion.map(|c| c.source_currency.clone()),
        source_amount: conversion.map(|c| c.source_amount),
        rate: conversion.map(|c| c.rate),
        rate_ack: None,
        variable,
    }
}

pub fn new_payload(template: Template, repeat: Repeat, anchor: NaiveDate) -> RecurringPayload {
    RecurringPayload { template, rule: repeat.rule(anchor), next: 0, paused: false, seed: Uuid::new_v4(), author_id: None }
}

/// A rule edited from "next on `next_on`". An unchanged schedule keeps its cursor; a changed one is
/// re-anchored there, with a fixed count reduced to what was left. The seed is kept either way, so
/// a date already written can never be written again.
pub fn edited_payload(
    old: &RecurringPayload,
    template: Template,
    repeat: Repeat,
    next_on: NaiveDate,
) -> RecurringPayload {
    let unchanged = old.rule.freq == repeat.freq
        && old.rule.interval == repeat.interval.max(1)
        && old.rule.ends == repeat.ends
        && next_date(old) == Some(next_on);
    if unchanged {
        return RecurringPayload { template, ..old.clone() };
    }
    RecurringPayload { template, rule: repeat.rule(next_on), next: 0, ..old.clone() }
}

/// The Repeat an existing rule shows in the edit form: a fixed count counts what is left.
pub fn repeat_of(p: &RecurringPayload) -> Repeat {
    let ends = match p.rule.ends {
        Ends::After(n) => Ends::After(n.saturating_sub(p.next).max(1)),
        other => other,
    };
    Repeat { freq: p.rule.freq, interval: p.rule.interval, ends, variable: p.template.variable }
}

/// How many occurrences saving a rule anchored on `anchor` writes at once.
pub fn backfill_count(repeat: Repeat, anchor: NaiveDate, today: NaiveDate) -> usize {
    super::schedule::due(&repeat.rule(anchor), 0, today, 10_000).len()
}

pub fn date_of(rule: &Rule, n: u32) -> Option<NaiveDate> {
    nth_date(rule, n)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDateTime;

    fn d(y: i32, m: u32, day: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, day).unwrap()
    }

    fn t(amount: f64, rate: Option<f64>) -> Template {
        Template {
            name: "x".to_string(),
            expense_type: "expense".to_string(),
            category: None,
            amount,
            payers: Side { exact: true, entries: vec![(1, amount)] },
            debtors: Side { exact: false, entries: vec![(1, 1.0), (2, 1.0)] },
            source_currency: rate.map(|_| "CHF".to_string()),
            source_amount: rate.map(|r| amount / r),
            rate,
            rate_ack: None,
            variable: false,
        }
    }

    fn rule(name: &str, freq: Freq, amount: f64, next: u32, paused: bool, ends: Ends) -> DecryptedRule {
        let mut template = t(amount, None);
        template.name = name.to_string();
        DecryptedRule {
            id: Uuid::new_v4(),
            version: 0,
            author_id: Some(1),
            created_at: NaiveDateTime::default(),
            payload: RecurringPayload {
                template,
                rule: Rule { freq, interval: 1, anchor: d(2026, 10, 1), ends },
                next,
                paused,
                seed: Uuid::nil(),
                author_id: None,
            },
        }
    }

    #[test]
    fn active_rules_come_first_by_next_date_then_paused_then_finished() {
        let rules = vec![
            rule("done", Freq::Monthly, 1.0, 2, false, Ends::After(2)),
            rule("paused", Freq::Monthly, 1.0, 0, true, Ends::Never),
            rule("later", Freq::Monthly, 1.0, 1, false, Ends::Never),
            rule("sooner", Freq::Weekly, 1.0, 1, false, Ends::Never),
        ];
        let names: Vec<String> = sorted(&rules).iter().map(|r| r.payload.template.name.clone()).collect();
        assert_eq!(names, vec!["sooner", "later", "paused", "done"]);
    }

    #[test]
    fn the_monthly_total_leaves_out_paused_and_finished_rules() {
        let rules = vec![
            rule("rent", Freq::Monthly, 1200.0, 0, false, Ends::Never),
            rule("insurance", Freq::Yearly, 120.0, 0, false, Ends::Never),
            rule("paused", Freq::Monthly, 50.0, 0, true, Ends::Never),
            rule("done", Freq::Monthly, 50.0, 1, false, Ends::After(1)),
        ];
        assert!((monthly_total(&rules) - 1210.0).abs() < 1e-9);
        assert!((my_monthly_share(&rules, 2) - 605.0).abs() < 1e-9);
    }

    #[test]
    fn drift_is_flagged_only_beyond_five_percent_of_the_kept_rate() {
        let tpl = t(192.6, Some(1.07));
        assert!(rate_drift(&tpl, 1.04).is_none());
        let drift = rate_drift(&tpl, 1.005).unwrap();
        assert!((drift + 0.0607).abs() < 1e-3);
        let kept = Template { rate_ack: Some(1.005), ..tpl };
        assert!(rate_drift(&kept, 1.005).is_none(), "keeping the rate silences the warning");
        assert!(rate_drift(&t(10.0, None), 2.0).is_none());
    }

    #[test]
    fn an_unchanged_schedule_keeps_its_cursor() {
        let old = rule("rent", Freq::Monthly, 1450.0, 2, false, Ends::Never).payload;
        let edited = edited_payload(&old, t(1480.0, None), repeat_of(&old), d(2026, 12, 1));
        assert_eq!(edited.next, 2);
        assert_eq!(edited.rule, old.rule);
        assert_eq!(edited.template.amount, 1480.0);
    }

    #[test]
    fn a_changed_schedule_is_reanchored_keeping_the_seed_and_what_is_left() {
        let old = rule("sofa", Freq::Monthly, 225.0, 2, false, Ends::After(4)).payload;
        let mut repeat = repeat_of(&old);
        assert_eq!(repeat.ends, Ends::After(2));
        repeat.freq = Freq::Weekly;
        let edited = edited_payload(&old, old.template.clone(), repeat, d(2026, 12, 3));
        assert_eq!(edited.next, 0);
        assert_eq!(edited.rule.anchor, d(2026, 12, 3));
        assert_eq!(edited.rule.ends, Ends::After(2));
        assert_eq!(edited.seed, old.seed);
    }

    #[test]
    fn progress_counts_written_occurrences() {
        let p = rule("sofa", Freq::Monthly, 225.0, 2, false, Ends::After(4)).payload;
        assert_eq!(progress(&p), Some((2, 4)));
        assert_eq!(progress(&rule("x", Freq::Monthly, 1.0, 0, false, Ends::Never).payload), None);
    }

    #[test]
    fn a_start_in_the_past_backfills_every_date_up_to_today() {
        let r = Repeat::monthly();
        assert_eq!(backfill_count(r, d(2026, 8, 1), d(2026, 9, 30)), 2);
        assert_eq!(backfill_count(r, d(2026, 10, 1), d(2026, 9, 30)), 0);
    }
}
