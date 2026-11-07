use dioxus::prelude::*;
use crate::tid;

use crate::icons::{BackArrowIcon, ICON_HEADER};

#[derive(PartialEq, Props, Clone)]
pub struct BackButtonArrowProps {
    #[props(default)]
    onclick: EventHandler<MouseEvent>,
}

#[component]
pub fn BackButtonArrow(props: BackButtonArrowProps) -> Element {
    rsx! {
        button {
            // Fixed, not a prop: there is one header per page, so it cannot collide. E2E navigates
            // back through this - `aria-label` is translated copy and changes freely.
            id: "back-button",
            r#type: "button",
            class: "btn btn-circle size-11 min-h-11 border-0 bg-base-300 glass-button",
            aria_label: tid!("back"),
            onclick: move |e| props.onclick.call(e),
            BackArrowIcon { size: ICON_HEADER }
        }
    }
}
