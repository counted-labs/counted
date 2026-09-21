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
pub const KEYBOARD_MIN_PX: f64 = 80.0;

/// How much of the layout viewport the on-screen keyboard is covering, in CSS px.
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
    let obscured = layout_h - visual_h - offset_top;
    if obscured < KEYBOARD_MIN_PX {
        0.0
    } else {
        obscured
    }
}

/// The `--kb` write, mirroring `css_inset_vars` in the mobile entry point.
///
/// One decimal, like the safe-area vars: the value feeds a `max()` on a sheet's padding, and more
/// precision than that is noise in a layout measured in rem.
///
/// With a keyboard up it also re-scrolls the focused field. The WebView scrolled it into view when
/// it was focused, against the sheet body as it was then; the padding this writes shrinks that
/// body by the keyboard's height a bridge round-trip later, and a field near its bottom edge ends
/// up under the footer. Same script, so the new padding is laid out before the scroll is computed;
/// `nearest` touches only the ancestors that actually clip the field and is a no-op when nothing
/// does, which is what makes it safe on every page rather than just the sheets.
pub fn css_keyboard_var(px: f64) -> String {
    let set = format!("document.documentElement.style.setProperty('--kb','{px:.1}px');");
    if px > 0.0 {
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
        assert!(css_keyboard_var(336.0).contains("'--kb','336.0px'"));
        assert!(css_keyboard_var(0.0).contains("'--kb','0.0px'"));
    }

    /// The sheet body shrinks by the padding this writes, so the field focused a moment ago can
    /// now sit under the footer; the scroll runs after the write, in the same script, and only
    /// while a keyboard is up — a closing keyboard has nothing to reveal.
    #[test]
    fn an_open_keyboard_rescrolls_the_focused_field_after_the_write() {
        let js = css_keyboard_var(336.0);
        let set = js.find("setProperty('--kb'").unwrap();
        let scroll = js.find("document.activeElement?.scrollIntoView({block:'nearest'})").unwrap();
        assert!(set < scroll, "the padding must be laid out before the scroll is computed");
        assert!(!css_keyboard_var(0.0).contains("scrollIntoView"));
    }
}
