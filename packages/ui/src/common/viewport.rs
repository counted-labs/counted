//! Visual-viewport arithmetic, kept out of JavaScript so it can be tested.
//!
//! The listener that feeds these lives in `packages/mobile/src/main.rs`, next to the safe-area
//! bridge it is modelled on. Only the maths is here — in `ui` rather than beside that listener,
//! because `cargo test --package ui` is the one a CI job actually runs.

/// Below this, an obscured strip is not a keyboard.
///
/// The visual viewport shrinks for several things that are not the keyboard: a collapsing URL bar,
/// sub-pixel rounding, a frame captured mid-rotation. Treating any of those as a keyboard would add
/// dead space under an open sheet for no reason. Real iOS keyboards are ~250-350pt; 80 is clear of
/// the noise and far below the smallest of them.
///
/// It judges the keyboard itself (`layout_h - visual_h`), never what remains once the scroll is
/// subtracted: that remainder shrinks legitimately as the page pans, and must not read as noise.
pub const KEYBOARD_MIN_PX: f64 = 80.0;

/// How much of the layout viewport the on-screen keyboard is covering, in CSS px.
///
/// `layout_h` is the height of the box `position: fixed` lays out in, measured off a probe — never
/// `innerHeight`, which iOS 26 reports net of the scroll (see the listener in the mobile entry point).
///
/// `offset_top` is the visual viewport's own scroll within the layout viewport: when WKWebView
/// scrolls the page to reveal a focused input, the visible band moves down, and the part hidden at
/// the *bottom* shrinks by exactly that much. Leaving it out over-reports the keyboard on any page
/// that scrolled to focus something — which is every page with a form.
///
/// Returns 0 rather than a negative or a NaN for anything incoherent, so a viewport that reports
/// nonsense degrades to "no keyboard" instead of to a broken layout.
pub fn keyboard_inset(layout_h: f64, visual_h: f64, offset_top: f64) -> f64 {
    if !layout_h.is_finite() || !visual_h.is_finite() || !offset_top.is_finite() {
        return 0.0;
    }
    let keyboard = layout_h - visual_h;
    if keyboard < KEYBOARD_MIN_PX {
        return 0.0;
    }
    (keyboard - offset_top).max(0.0)
}

/// The `--kb` and `--vvo` writes, mirroring `css_inset_vars` in the mobile entry point.
///
/// One decimal, like the safe-area vars: the values feed a `max()` on a sheet's padding and a
/// `calc()` on its max-height, and more precision than that is noise in a layout measured in rem.
///
/// `--vvo` is the visual viewport's scroll within the layout viewport — the same `offset_top`
/// `keyboard_inset` subtracts. A sheet is `position: fixed`, so it is pinned to the *layout*
/// viewport while the visible band is the visual one; on a platform that does not resize its layout
/// viewport for the keyboard (Android is `adjustNothing`), scrolling to reveal a focused field moves
/// the band down and leaves the sheet's top edge that far above it. `keyboard_inset` already keeps
/// the *bottom* edge honest; `--vvo` is what a sheet's max-height subtracts to keep the top edge
/// honest too.
///
/// With a keyboard up it also re-scrolls the focused field. The WebView scrolled it into view when
/// it was focused, against the sheet body as it was then; the padding this writes shrinks that
/// body by the keyboard's height a bridge round-trip later, and a field near its bottom edge ends
/// up under the footer. Same script, so the new padding is laid out before the scroll is computed;
/// `nearest` touches only the ancestors that actually clip the field and is a no-op when nothing
/// does, which is what makes it safe on every page rather than just the sheets.
///
/// It keys on either variable, not on `kb` alone: when WebKit pans by the whole keyboard, `kb` is
/// 0 while `--vvo` has shrunk the sheet by that much, and the field ends up just as hidden.
pub fn css_viewport_vars(kb: f64, offset_top: f64) -> String {
    // Same degradation as `keyboard_inset`: nonsense from a viewport mid-teardown becomes "no
    // offset" rather than a `calc()` that drops the whole declaration.
    let vvo = if offset_top.is_finite() && offset_top > 0.0 { offset_top } else { 0.0 };
    let set = format!(
        "document.documentElement.style.setProperty('--kb','{kb:.1}px');\
         document.documentElement.style.setProperty('--vvo','{vvo:.1}px');"
    );
    if kb > 0.0 || vvo > 0.0 {
        set + "document.activeElement?.scrollIntoView({block:'nearest'});"
    } else {
        set
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The ordinary case: nothing focused, the visual viewport fills the layout viewport.
    #[test]
    fn a_closed_keyboard_obscures_nothing() {
        assert_eq!(keyboard_inset(844.0, 844.0, 0.0), 0.0);
    }

    /// A typical iPhone keyboard, page not scrolled.
    #[test]
    fn an_open_keyboard_is_the_difference() {
        assert_eq!(keyboard_inset(844.0, 508.0, 0.0), 336.0);
    }

    /// The bug this argument exists for. WKWebView scrolls the page to reveal a focused input, so
    /// the visible band starts lower; without subtracting that, a sheet would be padded by the
    /// keyboard *plus* the scroll and pushed off the top of the screen.
    #[test]
    fn a_scrolled_page_does_not_double_count_the_offset() {
        assert_eq!(keyboard_inset(844.0, 508.0, 100.0), 236.0);
    }

    /// The threshold judges the keyboard, not what the scroll leaves of it. Refocusing fields pans
    /// the visual viewport further each time; once the remainder fell under the threshold it read
    /// as "no keyboard", the padding dropped to `--sab`, and the operator bar sank under the keys.
    #[test]
    fn a_scrolled_page_keeps_the_strip_the_keyboard_still_covers() {
        assert_eq!(keyboard_inset(844.0, 508.0, 300.0), 36.0);
    }

    /// Measured on an iPhone 17 (iOS 26, 874px screen): fixed box 873.7, visual viewport 566,
    /// scrolled 24.7. The keyboard's top edge sat at 566 on screen and the sheet's bottom at 849, so
    /// the sheet owed 283. Fed `innerHeight` (849) instead, this returned 258.3 and the bar sank
    /// 24.7px behind the keys.
    #[test]
    fn the_measured_iphone_case_lands_the_sheet_on_the_keyboard() {
        assert!((keyboard_inset(873.7, 566.0, 24.7) - 283.0).abs() < 0.01);
    }

    #[test]
    fn a_page_scrolled_past_the_keyboard_owes_nothing() {
        assert_eq!(keyboard_inset(844.0, 508.0, 400.0), 0.0);
    }

    /// A URL bar collapsing, or a rotation caught mid-frame. Padding a sheet for this would be
    /// visible dead space with no keyboard on screen.
    #[test]
    fn a_strip_too_small_to_be_a_keyboard_is_ignored() {
        assert_eq!(keyboard_inset(844.0, 800.0, 0.0), 0.0);
        assert_eq!(keyboard_inset(844.0, 844.0 - KEYBOARD_MIN_PX + 1.0, 0.0), 0.0);
    }

    /// The threshold is inclusive at the boundary, so the constant means what it says.
    #[test]
    fn the_threshold_itself_counts() {
        assert_eq!(keyboard_inset(844.0, 844.0 - KEYBOARD_MIN_PX, 0.0), KEYBOARD_MIN_PX);
    }

    /// A window resized for the keyboard (an `adjustResize` Android build) shrinks both heights
    /// together, so the sheet owes nothing — the layout viewport already ends at the keyboard.
    #[test]
    fn a_platform_that_resizes_its_layout_viewport_owes_nothing() {
        assert_eq!(keyboard_inset(508.0, 508.0, 0.0), 0.0);
    }

    /// Over-scroll can make the visual viewport taller than the layout viewport. Negative padding
    /// would pull the sheet off the bottom of the screen.
    #[test]
    fn a_negative_difference_never_becomes_padding() {
        assert_eq!(keyboard_inset(844.0, 900.0, 0.0), 0.0);
    }

    /// visualViewport is absent or mid-teardown and the eval sends garbage.
    #[test]
    fn non_finite_input_is_not_a_keyboard() {
        assert_eq!(keyboard_inset(f64::NAN, 508.0, 0.0), 0.0);
        assert_eq!(keyboard_inset(844.0, f64::INFINITY, 0.0), 0.0);
        assert_eq!(keyboard_inset(844.0, 508.0, f64::NAN), 0.0);
    }

    #[test]
    fn the_css_write_carries_one_decimal_and_a_unit() {
        assert!(css_viewport_vars(336.0, 0.0).contains("'--kb','336.0px'"));
        assert!(css_viewport_vars(0.0, 0.0).contains("'--kb','0.0px'"));
        assert!(css_viewport_vars(336.0, 124.0).contains("'--vvo','124.0px'"));
        assert!(css_viewport_vars(0.0, 0.0).contains("'--vvo','0.0px'"));
    }

    /// The offset is written on every report, keyboard or not: a page that scrolled for a focused
    /// field and then dismissed the keyboard can still be offset for a frame.
    #[test]
    fn both_variables_are_written_every_time() {
        for (kb, offset) in [(0.0, 0.0), (336.0, 0.0), (0.0, 40.0), (236.0, 100.0)] {
            let js = css_viewport_vars(kb, offset);
            assert!(js.contains("'--kb'"), "kb missing from {js}");
            assert!(js.contains("'--vvo'"), "vvo missing from {js}");
        }
    }

    /// A negative or non-finite offset would otherwise render as `--vvo: NaNpx`, and a `calc()`
    /// reading it drops the whole max-height declaration.
    #[test]
    fn an_incoherent_offset_is_not_an_offset() {
        assert!(css_viewport_vars(336.0, -12.0).contains("'--vvo','0.0px'"));
        assert!(css_viewport_vars(336.0, f64::NAN).contains("'--vvo','0.0px'"));
        assert!(css_viewport_vars(336.0, f64::INFINITY).contains("'--vvo','0.0px'"));
    }

    /// The sheet body shrinks by the padding this writes, so the field focused a moment ago can
    /// now sit under the footer; the scroll runs after the write, in the same script, and only
    /// while a keyboard is up — a closing keyboard has nothing to reveal.
    #[test]
    fn an_open_keyboard_rescrolls_the_focused_field_after_the_write() {
        let js = css_viewport_vars(336.0, 0.0);
        let kb = js.find("setProperty('--kb'").unwrap();
        let vvo = js.find("setProperty('--vvo'").unwrap();
        let scroll = js.find("document.activeElement?.scrollIntoView({block:'nearest'})").unwrap();
        assert!(kb < scroll, "the padding must be laid out before the scroll is computed");
        assert!(vvo < scroll, "the max-height must be laid out before the scroll is computed");
        assert!(!css_viewport_vars(0.0, 0.0).contains("scrollIntoView"));
    }

    /// Measured on an iPhone 17 (iOS 26): focusing a field low in the add-project sheet made
    /// WebKit pan the visual viewport by the whole keyboard (offsetTop 308, visual height 566 of
    /// 874). `keyboard_inset` is then 0, yet `--vvo` has shrunk the sheet by 308 and the field sat
    /// under the footer, never re-scrolled.
    #[test]
    fn a_keyboard_fully_absorbed_by_the_pan_still_rescrolls() {
        assert_eq!(keyboard_inset(874.0, 566.0, 308.0), 0.0);
        assert!(css_viewport_vars(0.0, 308.0).contains("scrollIntoView"));
    }
}
