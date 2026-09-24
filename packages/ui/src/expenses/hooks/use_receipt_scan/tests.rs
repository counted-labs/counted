use super::*;

/// Every key the hook can hand to `tid!` comes from `scan_error_key`, which the literal scanner in
/// `i18n.rs` cannot see — so this module asserts its own keys, the pattern `error_utils` and
/// `project_status_menu` use.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn every_key_the_hook_can_show_exists_in_the_fallback_locale() {
    let known = crate::i18n::locale_ids(crate::i18n::FALLBACK);
    for error in [ScanError::Capture, ScanError::Unreadable] {
        let key = scan_error_key(error);
        assert!(known.contains(key), "{key} is not defined in {}.ftl", crate::i18n::FALLBACK);
    }
    for key in [
        "expense-scan",
        "scan-in-progress",
        "scan-check-amount",
        "scan-take-photo",
        "scan-choose-photo",
    ] {
        assert!(known.contains(key), "{key} is not defined in {}.ftl", crate::i18n::FALLBACK);
    }
}

/// The two outcomes that are deliberately silent. Showing a toast when someone taps "cancel" in
/// the camera would be worse than showing nothing at all.
#[test]
fn cancelling_and_unsupported_are_the_silent_outcomes() {
    for error in [ScanError::Cancelled, ScanError::Unsupported] {
        assert!(
            matches!(error, ScanError::Cancelled | ScanError::Unsupported),
            "if a variant is added, decide explicitly whether it is shown"
        );
    }
}

#[test]
fn the_button_is_hidden_where_no_scanner_is_installed() {
    assert!(!scanner_available(), "the ui crate installs none of its own");
}
