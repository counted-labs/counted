use dioxus::prelude::*;
use crate::tid;

use crate::icons::{ArrowBackIcon, BackArrowIcon, ICON_HEADER};

#[derive(PartialEq, Props, Clone)]
pub struct BackButtonArrowProps {
    #[props(default)]
    onclick: EventHandler<MouseEvent>,
}

/// Both glyphs ship and native-feel.css shows one: the chevron on iOS and the web, Material's
/// `arrow_back` on Android.
#[component]
pub fn BackButtonArrow(props: BackButtonArrowProps) -> Element {
    rsx! {
        button {
            // Fixed, not a prop: there is one header per page, so it cannot collide. E2E navigates
            // back through this - `aria-label` is translated copy and changes freely.
            id: "back-button",
            r#type: "button",
            class: "nav-back btn btn-circle size-11 min-h-11 border-0 bg-base-300 glass-button",
            aria_label: tid!("back"),
            onclick: move |e| props.onclick.call(e),
            span { class: "back-glyph-chevron", BackArrowIcon { size: ICON_HEADER } }
            span { class: "back-glyph-arrow", ArrowBackIcon { size: 24 } }
        }
    }
}
