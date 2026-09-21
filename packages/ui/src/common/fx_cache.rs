//! The client's 24-hour cache of the InforEuro rate table.
//!
//! **A key of its own, not a field on `LocalStorageState`.** That store is rewritten wholesale on
//! every project sync — serialising every cached row of every project — and a rate table has
//! nothing to do with a project. Keeping it separate means a refresh here does not rewrite the hot
//! blob, and a project sync does not rewrite the rates.
//!
//! Only read when the expense form is on a currency other than the project's, so a single-currency
//! user never causes a fetch, of either kind.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::persist;

const KEY: &str = "counted_fx_rates";
const LABEL: &str = "fx rates";

/// How long a stored table is served without asking the server again. The table changes once a
/// month; a day is what keeps the month rollover from lagging noticeably on a device.
const MAX_AGE_SECONDS: i64 = 24 * 60 * 60;

/// `fetched_at` is a Unix timestamp in seconds, not a `NaiveDateTime`: it is only ever compared
/// against "now" for an age, and a plain integer survives a clock format change.
#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
#[serde(default)]
pub struct CachedFx {
    /// The first day of the month the rates are valid for. Shown to the user, so they can see
    /// which month they are converting at.
    pub day: String,
    pub base: String,
    pub rates: HashMap<String, f64>,
    pub fetched_at: i64,
}

pub fn read_fx() -> Option<CachedFx> {
    let cached: CachedFx = persist::read_json(KEY);
    (!cached.rates.is_empty()).then_some(cached)
}

pub fn write_fx(value: &CachedFx) {
    persist::write_json(KEY, LABEL, value);
}

/// Seconds since the Unix epoch. `chrono::Utc::now` works on wasm and native alike.
pub fn now_seconds() -> i64 {
    chrono::Utc::now().timestamp()
}

/// True when the table is recent enough to use without asking the server.
///
/// A `fetched_at` in the future — a device whose clock was wrong when it wrote, then corrected —
/// reads as stale rather than as fresh forever.
pub fn is_fresh(cached: &CachedFx, now: i64) -> bool {
    let age = now - cached.fetched_at;
    (0..=MAX_AGE_SECONDS).contains(&age)
}

/// The rate to convert `from` into `to`. `None` when either currency is missing from the table or
/// the quotient is not a usable rate.
pub fn cross_rate(cached: &CachedFx, from: &str, to: &str) -> Option<f64> {
    shared::cross_rate(&cached.rates, from, to)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn table() -> CachedFx {
        CachedFx {
            day: "2026-09-04".to_string(),
            base: "EUR".to_string(),
            rates: [("EUR", 1.0), ("USD", 1.1622), ("GBP", 0.85898)]
                .into_iter()
                .map(|(c, r)| (c.to_string(), r))
                .collect(),
            fetched_at: 1_000_000,
        }
    }

    #[test]
    fn fresh_within_the_window_stale_outside_it() {
        let t = table();
        assert!(is_fresh(&t, 1_000_000));
        assert!(is_fresh(&t, 1_000_000 + MAX_AGE_SECONDS));
        assert!(!is_fresh(&t, 1_000_000 + MAX_AGE_SECONDS + 1));
    }

    /// A clock that was wrong on write and right on read must not pin the cache as fresh forever.
    #[test]
    fn a_future_timestamp_reads_as_stale() {
        let t = table();
        assert!(!is_fresh(&t, 999_999));
    }

    #[test]
    fn cross_rate_delegates_and_rejects_unknowns() {
        let t = table();
        assert_eq!(cross_rate(&t, "EUR", "USD"), Some(1.1622));
        assert!(cross_rate(&t, "EUR", "JPY").is_none());
    }

    /// An old blob, or one with fields we no longer write, must not take the cache down — the
    /// worst case is a refetch.
    #[test]
    fn deserialises_a_partial_blob() {
        let c: CachedFx = serde_json::from_str(r#"{"day":"2026-09-04"}"#).unwrap();
        assert!(c.rates.is_empty());
        assert_eq!(c.fetched_at, 0);
    }
}
