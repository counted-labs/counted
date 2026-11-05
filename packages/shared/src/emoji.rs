/// A project's or a category's emoji is written by any key holder and rendered on every member's
/// device, so it is checked on read as well as on write: a byte cap, and only code points that
/// build emoji. A grapheme count would need a segmentation crate and still let zalgo through.
pub const MAX_EMOJI_BYTES: usize = 32;

fn is_pictograph(c: char) -> bool {
    matches!(
        c as u32,
        0x1F000..=0x1FAFF
            | 0x2600..=0x27BF
            | 0x2300..=0x23FF
            | 0x2B00..=0x2BFF
            | 0x2190..=0x21FF
            | 0x25A0..=0x25FF
            | 0x2934..=0x2935
            | 0x3030
            | 0x303D
            | 0x3297
            | 0x3299
            | 0x00A9
            | 0x00AE
            | 0x203C
            | 0x2049
            | 0x2122
            | 0x2139
            | 0x24C2
    )
}

fn is_modifier(c: char) -> bool {
    matches!(c as u32, 0x200D | 0xFE0F | 0x20E3 | 0xE0020..=0xE007F) || matches!(c, '0'..='9' | '#' | '*')
}

pub fn is_allowed_emoji(text: &str) -> bool {
    !text.is_empty()
        && text.len() <= MAX_EMOJI_BYTES
        && text.chars().all(|c| is_pictograph(c) || is_modifier(c))
        && text.chars().any(|c| is_pictograph(c) || c == '\u{20E3}')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn everyday_emoji_are_allowed() {
        for e in ["🏝️", "🥕", "🅿️", "⛴️", "🛣️", "👩‍👩‍👧", "👍🏽", "🇫🇷", "1️⃣", "❤️", "☕"] {
            assert!(is_allowed_emoji(e), "{e}");
        }
    }

    #[test]
    fn text_controls_and_oversized_input_are_refused() {
        assert!(!is_allowed_emoji(""));
        assert!(!is_allowed_emoji("A"));
        assert!(!is_allowed_emoji("12"));
        assert!(!is_allowed_emoji("🍕\u{202E}"));
        assert!(!is_allowed_emoji("e\u{301}\u{301}\u{301}"));
        assert!(!is_allowed_emoji("🍕 "));
        assert!(!is_allowed_emoji(&"🍕".repeat(9)));
    }
}
