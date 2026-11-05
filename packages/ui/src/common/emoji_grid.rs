use dioxus::html::geometry::PixelsVector2D;
use dioxus::prelude::*;

use crate::common::{haptic, Haptic};
use crate::tid;

/// Picked from a grid rather than typed: a one-character field opens the letter keyboard on
/// Android and has no emoji keyboard on desktop. Every entry passes `shared::emoji::is_allowed_emoji`.
pub const EMOJI_CHOICES: &[&str] = &[
    "🏝️", "✈️", "🏕️", "🚐", "⛰️", "🗺️", "🧳", "🏖️", "🚆", "⛵", "🏰", "🌋", "🎒", "🌍", "🗼", "🏔️",
    "🏠", "🏡", "🏢", "🛋️", "🛏️", "🔑", "🧹", "🧺", "🪴", "🛠️", "💡", "🔥", "💧", "⚡", "📡", "🧸",
    "🍽️", "🛒", "🥕", "🥖", "🧀", "🍷", "☕", "🍕", "🍣", "🥗", "🍔", "🍦", "🥐", "🍺", "🧃", "🍫",
    "🍜", "🥩", "🍎", "🥑", "🍰", "🍾", "🥂", "🍸", "🌮", "🥘", "🍝", "🍿", "🧁", "🍪", "🥡", "🍱",
    "🚕", "🚗", "🛵", "🚲", "🚌", "⛴️", "🚇", "⛽", "🅿️", "🛣️", "🚀", "🛴", "🏍️", "🚁", "🚢", "🚖",
    "🎉", "🎂", "🎁", "🎄", "🎈", "💍", "🎊", "🥳", "🎃", "💐", "🕯️", "🪅", "👶", "🎓", "💒", "🎆",
    "🎬", "🎵", "🎮", "🎿", "⚽", "🎾", "🏀", "🏊", "🚴", "🧗", "🎳", "🎨", "📚", "🎭", "🎟️", "🏛️",
    "🛍️", "👕", "👟", "💄", "📱", "💻", "🎧", "📷", "⌚", "🕶️", "👜", "🧴", "💊", "🏥", "💇", "💆",
    "🐶", "🐱", "🐾", "🐴", "🌸", "🌳", "🌊", "☀️", "❄️", "🌙", "⭐", "🌈", "🍀", "🐟", "🦄", "🐝",
    "💼", "📈", "🧾", "💳", "💶", "🏦", "📦", "✏️", "🔧", "🧪", "🩺", "⚖️", "🛡️", "📌", "❤️", "👍",
];

#[component]
pub fn EmojiGrid(
    id: String,
    selected: Option<String>,
    /// Offers "none" first, drawn as what shows instead (a project's initial).
    #[props(default)]
    none_preview: Option<String>,
    onpick: EventHandler<Option<String>>,
) -> Element {
    let none_selected = selected.is_none();
    let leading = usize::from(none_preview.is_some());
    let cells = EMOJI_CHOICES.len() + leading;
    let selected_cell = selected.as_deref().and_then(|s| EMOJI_CHOICES.iter().position(|e| *e == s)).map(|i| i + leading);
    rsx! {
        div {
            id: "{id}",
            role: "group",
            aria_label: tid!("emoji-picker-label"),
            class: "grid grid-cols-8 gap-1 max-h-44 overflow-y-auto overscroll-contain p-0.5",
            onmounted: move |e| async move {
                let Some(cell) = selected_cell else { return };
                let grid = e.data();
                let Ok(size) = grid.get_scroll_size().await else { return };
                let top = reveal_offset(cell, cells, size.height);
                // The grid's own scrollTo: scrollIntoView would also scroll the page and pan the open sheet.
                let _ = grid.scroll(PixelsVector2D::new(0.0, top), ScrollBehavior::Instant).await;
            },
            if let Some(preview) = none_preview {
                button {
                    r#type: "button",
                    class: if none_selected { "aspect-square rounded-xl text-xs font-semibold bg-primary/15 ring-2 ring-primary" } else { "aspect-square rounded-xl text-xs font-semibold bg-base-300 text-base-content/70" },
                    aria_pressed: none_selected,
                    aria_label: tid!("emoji-none"),
                    onclick: move |_| {
                        haptic(Haptic::Light);
                        onpick.call(None);
                    },
                    "{preview}"
                }
            }
            for emoji in EMOJI_CHOICES.iter().copied() {
                button {
                    key: "{emoji}",
                    r#type: "button",
                    class: if selected.as_deref() == Some(emoji) { "aspect-square rounded-xl text-xl leading-none bg-primary/15 ring-2 ring-primary" } else { "aspect-square rounded-xl text-xl leading-none hover:bg-base-200" },
                    aria_pressed: selected.as_deref() == Some(emoji),
                    onclick: move |_| {
                        haptic(Haptic::Light);
                        onpick.call(Some(emoji.to_string()));
                    },
                    "{emoji}"
                }
            }
        }
    }
}

const COLUMNS: usize = 8;

/// Leaves the row above the selected one in view, so it does not read as the grid's first row.
fn reveal_offset(cell: usize, cells: usize, scroll_height: f64) -> f64 {
    let pitch = scroll_height / cells.div_ceil(COLUMNS) as f64;
    (cell / COLUMNS).saturating_sub(1) as f64 * pitch
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;

    #[test]
    fn reveal_offset_scrolls_to_the_row_above_the_selected_one() {
        assert_eq!(reveal_offset(0, 161, 840.0), 0.0);
        assert_eq!(reveal_offset(12, 161, 840.0), 0.0);
        assert_eq!(reveal_offset(73, 161, 840.0), 320.0);
        assert_eq!(reveal_offset(160, 161, 840.0), 760.0);
    }

    #[test]
    fn every_choice_passes_the_read_check_and_appears_once() {
        let mut seen = BTreeSet::new();
        for e in EMOJI_CHOICES {
            assert!(shared::emoji::is_allowed_emoji(e), "{e}");
            assert!(seen.insert(*e), "{e} twice");
        }
    }
}
