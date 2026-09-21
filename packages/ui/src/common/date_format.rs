use chrono::Datelike;
use crate::tid;

/// A date as a reader of the active language expects it: `24 août 2026`, `August 24, 2026`.
///
/// The field *order* lives in the `.ftl` (`date-long`), not here — only the locale file knows
/// whether the month comes first. `chrono`'s `%B` is English-only, and `fluent-rs` ships no CLDR
/// data, so its `DATETIME` builtin cannot do this either; the month names are ordinary messages.
pub fn format_date(date: chrono::NaiveDate) -> String {
    tid!(
        "date-long",
        day: date.day() as i64,
        month: month_name(date.month0() as usize),
        year: date.year() as i64
    )
}

/// The same, from the `%Y-%m-%d` string the encrypted payloads carry. Falls back to the raw
/// string: a date that fails to parse means a tampered or outdated writer, and showing it as-is
/// beats hiding it or panicking on the render path.
pub fn format_date_str(date: &str) -> String {
    chrono::NaiveDate::parse_from_str(date, "%Y-%m-%d")
        .map(format_date)
        .unwrap_or_else(|_| date.to_string())
}

/// `septembre 2026` / `September 2026`, from any `%Y-%m-%d` date in that month — what the rate
/// hint shows for a table valid for a calendar month. Same raw-string fallback as above.
pub fn format_month_str(date: &str) -> String {
    chrono::NaiveDate::parse_from_str(date, "%Y-%m-%d")
        .map(|d| format!("{} {}", month_name(d.month0() as usize), d.year()))
        .unwrap_or_else(|_| date.to_string())
}

/// Full month name, `month0` indexed (0 = January).
pub fn month_name(month0: usize) -> String {
    MONTH_KEYS[month0 % 12].translate()
}

/// Abbreviated month name for chart axes, `month0` indexed. Same table, shorter labels — before
/// this there were two independent month lists to keep in sync.
pub fn month_abbrev(month0: usize) -> String {
    MONTH_SHORT_KEYS[month0 % 12].translate()
}

/// `tid!` takes a literal, and a month is picked by index. Wrapping each key in a unit struct with
/// its own `tid!` call keeps every key a literal the `i18n` scanner can verify.
struct MonthKey(fn() -> String);

impl MonthKey {
    fn translate(&self) -> String {
        (self.0)()
    }
}

const MONTH_KEYS: [MonthKey; 12] = [
    MonthKey(|| tid!("month-1")),
    MonthKey(|| tid!("month-2")),
    MonthKey(|| tid!("month-3")),
    MonthKey(|| tid!("month-4")),
    MonthKey(|| tid!("month-5")),
    MonthKey(|| tid!("month-6")),
    MonthKey(|| tid!("month-7")),
    MonthKey(|| tid!("month-8")),
    MonthKey(|| tid!("month-9")),
    MonthKey(|| tid!("month-10")),
    MonthKey(|| tid!("month-11")),
    MonthKey(|| tid!("month-12")),
];

const MONTH_SHORT_KEYS: [MonthKey; 12] = [
    MonthKey(|| tid!("month-short-1")),
    MonthKey(|| tid!("month-short-2")),
    MonthKey(|| tid!("month-short-3")),
    MonthKey(|| tid!("month-short-4")),
    MonthKey(|| tid!("month-short-5")),
    MonthKey(|| tid!("month-short-6")),
    MonthKey(|| tid!("month-short-7")),
    MonthKey(|| tid!("month-short-8")),
    MonthKey(|| tid!("month-short-9")),
    MonthKey(|| tid!("month-short-10")),
    MonthKey(|| tid!("month-short-11")),
    MonthKey(|| tid!("month-short-12")),
];

#[cfg(test)]
mod tests {
    use chrono::NaiveDate;

    use super::*;

    /// Rendering needs an i18n context, which a plain unit test has no runtime for. What is
    /// testable without one is the parsing half — and that every locale defines all 24 month
    /// messages, which is what would actually break a date.
    #[test]
    fn unparseable_dates_are_shown_verbatim() {
        assert_eq!(format_date_str("not-a-date"), "not-a-date");
        assert_eq!(format_date_str(""), "");
        assert_eq!(format_date_str("2026-13-45"), "2026-13-45");
        assert_eq!(format_month_str("2026-13-01"), "2026-13-01");
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn every_locale_defines_all_twelve_months_in_both_forms() {
        let fallback = crate::i18n::locale_ids(crate::i18n::FALLBACK);
        for m in 1..=12 {
            for key in [format!("month-{m}"), format!("month-short-{m}")] {
                assert!(fallback.contains(&key), "{key} missing from the fallback locale");
            }
        }
        // A locale that translates *some* months would render a date half in English.
        for (code, _) in crate::i18n::SUPPORTED {
            let ids = crate::i18n::locale_ids(code);
            let full = (1..=12).filter(|m| ids.contains(&format!("month-{m}"))).count();
            let short = (1..=12).filter(|m| ids.contains(&format!("month-short-{m}"))).count();
            assert!(full == 0 || full == 12, "{code} defines {full}/12 month names");
            assert!(short == 0 || short == 12, "{code} defines {short}/12 short month names");
        }
    }

    #[test]
    fn month_helpers_are_in_range_for_every_month() {
        for m in 0..12 {
            let _ = MONTH_KEYS[m].0;
            let _ = MONTH_SHORT_KEYS[m].0;
        }
        assert_eq!(NaiveDate::from_ymd_opt(2026, 8, 24).unwrap().month0(), 7);
    }
}
