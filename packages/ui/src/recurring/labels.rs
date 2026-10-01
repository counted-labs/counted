use chrono::{Datelike, NaiveDate, Weekday};

use super::model::{Ends, Freq, Rule};
use super::schedule::total;
use super::view::Repeat;
use crate::common::{format_date, month_name};
use crate::tid;

pub const PRESETS: [(Freq, u32); 5] =
    [(Freq::Weekly, 1), (Freq::Weekly, 2), (Freq::Monthly, 1), (Freq::Monthly, 3), (Freq::Yearly, 1)];

pub fn weekday_name(day: Weekday) -> String {
    match day {
        Weekday::Mon => tid!("weekday-1"),
        Weekday::Tue => tid!("weekday-2"),
        Weekday::Wed => tid!("weekday-3"),
        Weekday::Thu => tid!("weekday-4"),
        Weekday::Fri => tid!("weekday-5"),
        Weekday::Sat => tid!("weekday-6"),
        Weekday::Sun => tid!("weekday-7"),
    }
}

pub fn frequency_label(freq: Freq, interval: u32) -> String {
    let n = interval.max(1) as i64;
    match (freq, n) {
        (Freq::Weekly, 1) => tid!("repeat-weekly"),
        (Freq::Weekly, 2) => tid!("repeat-biweekly"),
        (Freq::Monthly, 1) => tid!("repeat-monthly"),
        (Freq::Monthly, 3) => tid!("repeat-quarterly"),
        (Freq::Yearly, 1) => tid!("repeat-yearly"),
        (Freq::Weekly, n) => tid!("repeat-every-weeks", count: n),
        (Freq::Monthly, n) => tid!("repeat-every-months", count: n),
        (Freq::Yearly, n) => tid!("repeat-every-years", count: n),
    }
}

/// "on Thursday", "on day 1", "on October 1": what the frequency is anchored to.
pub fn anchor_label(freq: Freq, anchor: NaiveDate) -> String {
    match freq {
        Freq::Weekly => tid!("repeat-on-weekday", weekday: weekday_name(anchor.weekday())),
        Freq::Monthly => tid!("repeat-on-day", day: anchor.day() as i64),
        Freq::Yearly => tid!(
            "repeat-on-day-month",
            day: anchor.day() as i64,
            month: month_name(anchor.month0() as usize)
        ),
    }
}

pub fn schedule_label(rule: &Rule) -> String {
    format!("{} {}", frequency_label(rule.freq, rule.interval), anchor_label(rule.freq, rule.anchor))
}

pub fn ends_label(rule: &Rule) -> String {
    match rule.ends {
        Ends::Never => tid!("repeat-no-end"),
        Ends::On(date) => tid!("repeat-until", date: format_date(date)),
        Ends::After(_) => tid!("repeat-occurrences", count: total(rule).unwrap_or(0) as i64),
    }
}

pub fn repeat_summary(repeat: &Repeat, anchor: NaiveDate) -> String {
    schedule_label(&repeat.rule(anchor))
}

/// Whether the month-end clamp can apply, so the picker explains it.
pub fn lands_on_month_end(freq: Freq, anchor: NaiveDate) -> bool {
    freq != Freq::Weekly && anchor.day() >= 29
}
