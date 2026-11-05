use dioxus::prelude::*;
use uuid::Uuid;

const TINTS: [&str; 5] = [
    "bg-tile-green text-tile-green-content",
    "bg-tile-blue text-tile-blue-content",
    "bg-tile-amber text-tile-amber-content",
    "bg-tile-violet text-tile-violet-content",
    "bg-tile-red text-tile-red-content",
];

fn tint(id: Uuid) -> &'static str {
    TINTS[(id.as_u128() % TINTS.len() as u128) as usize]
}

pub fn project_initial(name: &str) -> String {
    name.trim().chars().next().map(|c| c.to_uppercase().collect()).unwrap_or_else(|| "Aa".to_string())
}

/// The emoji when it passes the read check, else the initial: the tile renders what any key
/// holder wrote.
pub fn tile_text(name: &str, emoji: Option<&str>) -> (String, bool) {
    match emoji.filter(|e| shared::emoji::is_allowed_emoji(e)) {
        Some(e) => (e.to_string(), true),
        None => (project_initial(name), false),
    }
}

pub fn tile_class(text_is_emoji: bool) -> &'static str {
    if text_is_emoji {
        "size-[42px] shrink-0 rounded-[13px] text-[22px] leading-none flex items-center justify-center"
    } else {
        "size-[42px] shrink-0 rounded-[13px] font-display font-bold text-[17px] flex items-center justify-center"
    }
}

/// A project's emoji, or its first letter, on a tint fixed by its id, as the home list and the
/// move and counted-groups sheets show it.
#[component]
pub fn ProjectTile(name: String, id: Uuid, #[props(default)] emoji: Option<String>) -> Element {
    let (text, is_emoji) = tile_text(&name, emoji.as_deref());
    rsx! {
        span { class: "{tile_class(is_emoji)} {tint(id)}",
            aria_hidden: "true",
            "{text}"
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_initial_is_the_first_letter_upper_cased() {
        assert_eq!(project_initial("corsica 2026"), "C");
        assert_eq!(project_initial("  été"), "É");
        assert_eq!(project_initial("  "), "Aa", "a name not typed yet");
    }

    #[test]
    fn a_project_keeps_its_tint() {
        let id = Uuid::from_u128(7);
        assert_eq!(tint(id), tint(id));
        assert_ne!(tint(Uuid::from_u128(0)), tint(Uuid::from_u128(1)));
    }

    #[test]
    fn the_emoji_takes_the_initial_s_place() {
        assert_eq!(tile_text("corse", Some("🏝️")), ("🏝️".to_string(), true));
        assert_eq!(tile_text("corse", None), ("C".to_string(), false));
    }

    #[test]
    fn a_refused_emoji_falls_back_to_the_initial() {
        for hostile in ["A", "", "🍕\u{202E}", "🍕 ", "<b>", &"🍕".repeat(9)] {
            assert_eq!(tile_text("anniversaire", Some(hostile)), ("A".to_string(), false), "{hostile}");
        }
    }
}
