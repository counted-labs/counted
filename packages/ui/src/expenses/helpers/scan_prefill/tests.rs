use super::*;

fn fields(title: Option<&str>, amount: Option<f64>, confident: bool) -> ScanFields {
    ScanFields {
        title: title.map(str::to_string),
        amount,
        date: Some("2026-09-04".into()),
        amount_confident: confident,
    }
}

#[test]
fn the_merchant_becomes_the_expense_name() {
    let p = prefill_from_scan(&fields(Some("Carrefour Market"), Some(12.90), true));
    assert_eq!(p.name.as_deref(), Some("Carrefour Market"));
    assert_eq!(p.amount, Some(12.90));
    assert_eq!(p.date.as_deref(), Some("2026-09-04"));
}

/// The category must come from the shared keyword table, not a second copy of it — these ids are
/// persisted inside the encrypted payload.
#[test]
fn the_category_is_a_stable_chart_category_id() {
    let p = prefill_from_scan(&fields(Some("Pizza Roma"), Some(20.0), true));
    assert_eq!(p.category.as_deref(), Some("Nourriture"));
    assert!(crate::categories::CHART_CATEGORIES.contains(&p.category.as_deref().unwrap()));
}

/// The keyword table matches whole words, plurals and truncations, not derivations — "Pizzaiolo"
/// is not "pizza". Worth pinning so nobody assumes the scan path does its own smarter matching.
#[test]
fn a_derived_word_is_not_a_keyword_match() {
    let p = prefill_from_scan(&fields(Some("Pizzaiolo Roma"), Some(20.0), true));
    assert_eq!(p.category.as_deref(), Some("Autres"));
}

#[test]
fn an_unrecognised_merchant_still_yields_a_category() {
    let p = prefill_from_scan(&fields(Some("Zzzz Qqq"), Some(5.0), true));
    assert_eq!(p.category.as_deref(), Some("Autres"));
}

#[test]
fn no_title_means_no_name_and_no_category() {
    let p = prefill_from_scan(&fields(None, Some(5.0), true));
    assert_eq!(p.name, None);
    assert_eq!(p.category, None);
}

#[test]
fn a_blank_title_is_treated_as_absent() {
    let p = prefill_from_scan(&fields(Some("   "), Some(5.0), true));
    assert_eq!(p.name, None);
    assert_eq!(p.category, None);
}

// ─── the hint ────────────────────────────────────────────────────────────────

#[test]
fn a_confident_amount_needs_no_hint() {
    assert_eq!(prefill_from_scan(&fields(Some("X"), Some(12.90), true)).hint, None);
}

#[test]
fn an_unconfident_amount_asks_the_user_to_check_it() {
    let p = prefill_from_scan(&fields(Some("X"), Some(12.90), false));
    assert_eq!(p.hint, Some("scan-check-amount"));
}

#[test]
fn a_missing_amount_asks_the_user_to_check_it() {
    let p = prefill_from_scan(&fields(Some("X"), None, true));
    assert_eq!(p.hint, Some("scan-check-amount"));
}

/// Every hint is rendered through `tid!`, so it has to exist in the fallback locale.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn the_hint_key_exists_in_english() {
    let ids = crate::i18n::locale_ids(crate::i18n::FALLBACK);
    assert!(ids.contains("scan-check-amount"), "scan-check-amount missing from en.ftl");
}
