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

fn initial(name: &str) -> String {
    name.trim().chars().next().map(|c| c.to_uppercase().collect()).unwrap_or_default()
}

/// A project's first letter on a tint fixed by its id, as the home list and the move and
/// counted-groups sheets show it.
#[component]
pub fn ProjectTile(name: String, id: Uuid) -> Element {
    rsx! {
        span { class: "size-[42px] shrink-0 rounded-[13px] font-display font-bold text-[17px] flex items-center justify-center {tint(id)}",
            aria_hidden: "true",
            "{initial(&name)}"
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_initial_is_the_first_letter_upper_cased() {
        assert_eq!(initial("corsica 2026"), "C");
        assert_eq!(initial("  été"), "É");
        assert_eq!(initial(""), "");
    }

    #[test]
    fn a_project_keeps_its_tint() {
        let id = Uuid::from_u128(7);
        assert_eq!(tint(id), tint(id));
        assert_ne!(tint(Uuid::from_u128(0)), tint(Uuid::from_u128(1)));
    }
}
