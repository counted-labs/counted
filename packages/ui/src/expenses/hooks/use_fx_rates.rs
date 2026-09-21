//! Supplies the InforEuro rate table to the expense form, from the local cache when it can.
//!
//! Lazy on purpose: `enabled` is false until the form is actually on a currency other than the
//! project's, so a single-currency project never fetches — no request to the server, and therefore
//! none from the server to InforEuro either.

use api::fx::fx_controller::get_fx_rates;
use dioxus::prelude::*;

use crate::common::fx_cache::{self, CachedFx};

/// `None` while loading or when no table could be obtained; the form falls back to asking for a
/// rate by hand, which is the only thing it can do offline anyway.
///
/// The cache is checked before the resource ever runs, so the common case makes no request. A
/// server error is not retried in a loop: the resource runs once per `enabled` flip, and the user
/// can still type a rate.
pub fn use_fx_rates(enabled: Memo<bool>) -> Memo<Option<CachedFx>> {
    let fetched: Resource<Option<CachedFx>> = use_resource(move || async move {
        if !enabled() {
            return None;
        }
        if let Some(cached) = fx_cache::read_fx() {
            if fx_cache::is_fresh(&cached, fx_cache::now_seconds()) {
                return Some(cached);
            }
        }
        match get_fx_rates().await {
            Ok(rates) => {
                let cached = CachedFx {
                    day: rates.day,
                    base: rates.base,
                    rates: rates.rates,
                    fetched_at: fx_cache::now_seconds(),
                };
                fx_cache::write_fx(&cached);
                Some(cached)
            }
            // A stored table that is merely stale still converts better than nothing, and its `day`
            // tells the user what they are looking at.
            Err(e) => {
                dioxus::logger::tracing::warn!("counted: fx rates unavailable: {e}");
                fx_cache::read_fx()
            }
        }
    });

    use_memo(move || fetched.read().clone().flatten())
}
