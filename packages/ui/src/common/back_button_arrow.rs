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
    // HIG navigation bars label the back control; a bare chevron is a web convention. Mobile only —
    // web and desktop keep the circular icon button, which suits a pointer and a wider header.
    // The button widens rather than the icon shrinking, so the hit target only grows.
    let mobile = cfg!(any(target_os = "android", target_os = "ios"));

    rsx! {
        button {
            // Fixed, not a prop: there is one header per page, so it cannot collide. E2E navigates
            // back through this — `aria-label` is translated copy and changes freely.
            id: "back-button",
            r#type: "button",
            class: if mobile { "btn btn-ghost gap-1 px-2" } else { "btn btn-ghost btn-circle" },
            aria_label: tid!("back"),
            onclick: move |e| props.onclick.call(e),
            BackArrowIcon { size: ICON_HEADER }
            if mobile {
                span { class: "text-base font-normal normal-case", {tid!("back")} }
            }
        }
    }
}
