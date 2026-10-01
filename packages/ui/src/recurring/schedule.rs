use chrono::{Datelike, Days, Months, NaiveDate};
use sha2::{Digest, Sha256};
use uuid::Uuid;

use super::model::{Ends, Freq, Rule};

/// Above this the loops below stop: a weekly rule would need a century to reach it.
const MAX_INDEX: u32 = 10_000;

/// The `n`th date of the rule, always counted from the anchor so a rule on the 31st lands on the
/// last day of shorter months and returns to the 31st afterwards.
pub fn nth_date(rule: &Rule, n: u32) -> Option<NaiveDate> {
    let steps = n.checked_mul(rule.interval.max(1))?;
    match rule.freq {
        Freq::Weekly => rule.anchor.checked_add_days(Days::new(7 * steps as u64)),
        Freq::Monthly => rule.anchor.checked_add_months(Months::new(steps)),
        Freq::Yearly => rule.anchor.checked_add_months(Months::new(steps.checked_mul(12)?)),
    }
}

fn in_bounds(rule: &Rule, n: u32, date: NaiveDate) -> bool {
    match rule.ends {
        Ends::Never => true,
        Ends::On(last) => date <= last,
        Ends::After(count) => n < count,
    }
}

/// The occurrences from `next` up to `today`, at most `limit` of them.
pub fn due(rule: &Rule, next: u32, today: NaiveDate, limit: usize) -> Vec<(u32, NaiveDate)> {
    let mut out = Vec::new();
    let mut n = next;
    while out.len() < limit && n < MAX_INDEX {
        let Some(date) = nth_date(rule, n) else { break };
        if date > today || !in_bounds(rule, n, date) {
            break;
        }
        out.push((n, date));
        n += 1;
    }
    out
}

pub fn upcoming(rule: &Rule, next: u32, count: usize) -> Vec<NaiveDate> {
    (next..next.saturating_add(count as u32))
        .map_while(|n| nth_date(rule, n).filter(|d| in_bounds(rule, n, *d)))
        .collect()
}

pub fn is_finished(rule: &Rule, next: u32) -> bool {
    upcoming(rule, next, 1).is_empty()
}

/// Where a resumed rule picks up: the first occurrence on or after `today`. The ones missed while
/// paused are skipped, never backfilled.
pub fn resume_index(rule: &Rule, next: u32, today: NaiveDate) -> u32 {
    (next..MAX_INDEX)
        .find(|n| nth_date(rule, *n).is_none_or(|d| d >= today))
        .unwrap_or(next)
}

/// How many occurrences the rule has in all, when it ends.
pub fn total(rule: &Rule) -> Option<u32> {
    match rule.ends {
        Ends::Never => None,
        Ends::After(count) => Some(count),
        Ends::On(_) => Some(
            (0..MAX_INDEX)
                .take_while(|n| nth_date(rule, *n).is_some_and(|d| in_bounds(rule, *n, d)))
                .count() as u32,
        ),
    }
}

/// The date of the last occurrence, when the rule ends.
pub fn last_date(rule: &Rule) -> Option<NaiveDate> {
    total(rule).filter(|t| *t > 0).and_then(|t| nth_date(rule, t - 1))
}

pub fn per_month(rule: &Rule) -> f64 {
    let interval = rule.interval.max(1) as f64;
    match rule.freq {
        Freq::Weekly => 52.0 / 12.0 / interval,
        Freq::Monthly => 1.0 / interval,
        Freq::Yearly => 1.0 / 12.0 / interval,
    }
}

/// Deterministic per (rule, date), so the server's unique index refuses a second write of the same
/// occurrence. Keyed by the secret seed: without it the server cannot test candidate dates.
pub fn occurrence_id(seed: Uuid, date: NaiveDate) -> Uuid {
    let mut hasher = Sha256::new();
    hasher.update(seed.as_bytes());
    hasher.update(date.format("%Y-%m-%d").to_string().as_bytes());
    let digest = hasher.finalize();
    let mut bytes = [0u8; 16];
    bytes.copy_from_slice(&digest[..16]);
    bytes[6] = (bytes[6] & 0x0f) | 0x80;
    bytes[8] = (bytes[8] & 0x3f) | 0x80;
    Uuid::from_bytes(bytes)
}

pub fn weekday(date: NaiveDate) -> chrono::Weekday {
    date.weekday()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn d(y: i32, m: u32, day: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, day).unwrap()
    }

    fn rule(freq: Freq, interval: u32, anchor: NaiveDate, ends: Ends) -> Rule {
        Rule { freq, interval, anchor, ends }
    }

    #[test]
    fn the_31st_lands_on_the_last_day_and_comes_back() {
        let r = rule(Freq::Monthly, 1, d(2027, 1, 31), Ends::Never);
        assert_eq!(nth_date(&r, 1), Some(d(2027, 2, 28)));
        assert_eq!(nth_date(&r, 2), Some(d(2027, 3, 31)));
        assert_eq!(nth_date(&r, 3), Some(d(2027, 4, 30)));
    }

    #[test]
    fn a_leap_day_rule_lands_on_the_28th_in_other_years() {
        let r = rule(Freq::Yearly, 1, d(2028, 2, 29), Ends::Never);
        assert_eq!(nth_date(&r, 1), Some(d(2029, 2, 28)));
        assert_eq!(nth_date(&r, 4), Some(d(2032, 2, 29)));
    }

    #[test]
    fn intervals_multiply_the_step() {
        let every_two_weeks = rule(Freq::Weekly, 2, d(2026, 10, 6), Ends::Never);
        assert_eq!(nth_date(&every_two_weeks, 1), Some(d(2026, 10, 20)));
        let quarterly = rule(Freq::Monthly, 3, d(2026, 10, 1), Ends::Never);
        assert_eq!(nth_date(&quarterly, 1), Some(d(2027, 1, 1)));
    }

    #[test]
    fn due_catches_up_every_missed_occurrence_and_no_more() {
        let r = rule(Freq::Monthly, 1, d(2026, 8, 1), Ends::Never);
        let got = due(&r, 0, d(2026, 9, 30), 50);
        assert_eq!(got, vec![(0, d(2026, 8, 1)), (1, d(2026, 9, 1))]);
        assert!(due(&r, 2, d(2026, 9, 30), 50).is_empty());
        assert_eq!(due(&r, 2, d(2026, 10, 1), 50), vec![(2, d(2026, 10, 1))]);
    }

    #[test]
    fn due_is_capped_so_a_long_catch_up_takes_several_calls() {
        let r = rule(Freq::Weekly, 1, d(2024, 9, 30), Ends::Never);
        let first = due(&r, 0, d(2026, 9, 30), 50);
        assert_eq!(first.len(), 50);
        let second = due(&r, 50, d(2026, 9, 30), 50);
        assert_eq!(second[0].0, 50);
    }

    #[test]
    fn an_end_date_or_count_stops_the_rule() {
        let on = rule(Freq::Monthly, 1, d(2026, 1, 15), Ends::On(d(2026, 3, 15)));
        assert_eq!(due(&on, 0, d(2026, 12, 31), 50).len(), 3);
        assert_eq!(total(&on), Some(3));
        let after = rule(Freq::Monthly, 1, d(2026, 1, 15), Ends::After(4));
        assert_eq!(due(&after, 0, d(2026, 12, 31), 50).len(), 4);
        assert!(is_finished(&after, 4));
        assert!(!is_finished(&after, 3));
        assert_eq!(last_date(&after), Some(d(2026, 4, 15)));
    }

    #[test]
    fn upcoming_lists_the_next_dates_within_the_end() {
        let r = rule(Freq::Monthly, 1, d(2026, 10, 1), Ends::After(2));
        assert_eq!(upcoming(&r, 0, 3), vec![d(2026, 10, 1), d(2026, 11, 1)]);
    }

    #[test]
    fn resuming_skips_what_was_missed_while_paused() {
        let r = rule(Freq::Weekly, 2, d(2026, 6, 2), Ends::Never);
        let next = resume_index(&r, 2, d(2026, 9, 30));
        assert_eq!(nth_date(&r, next), Some(d(2026, 10, 6)));
        assert_eq!(resume_index(&r, next, d(2026, 9, 30)), next);
    }

    #[test]
    fn a_resume_on_the_due_date_keeps_that_date() {
        let r = rule(Freq::Monthly, 1, d(2026, 1, 1), Ends::Never);
        assert_eq!(resume_index(&r, 0, d(2026, 10, 1)), 9);
    }

    #[test]
    fn monthly_equivalents() {
        let a = d(2026, 1, 1);
        assert!((per_month(&rule(Freq::Weekly, 1, a, Ends::Never)) - 52.0 / 12.0).abs() < 1e-9);
        assert_eq!(per_month(&rule(Freq::Monthly, 2, a, Ends::Never)), 0.5);
        assert!((per_month(&rule(Freq::Yearly, 1, a, Ends::Never)) - 1.0 / 12.0).abs() < 1e-9);
    }

    #[test]
    fn the_occurrence_id_is_stable_per_date_and_secret_per_rule() {
        let seed = Uuid::from_u128(7);
        let a = occurrence_id(seed, d(2026, 10, 1));
        assert_eq!(a, occurrence_id(seed, d(2026, 10, 1)));
        assert_ne!(a, occurrence_id(seed, d(2026, 11, 1)));
        assert_ne!(a, occurrence_id(Uuid::from_u128(8), d(2026, 10, 1)));
        assert_eq!(a.get_version_num(), 8);
    }
}
