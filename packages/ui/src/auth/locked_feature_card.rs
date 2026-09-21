use dioxus::prelude::*;
use crate::tid;

use crate::icons::{LockIcon, ICON_INLINE};

/// An account-only feature shown to an anonymous user in the slot it would occupy once signed in.
/// Nothing in it is interactive — the one call to action is the account card above it.
#[component]
pub fn LockedFeatureCard(icon: Element, title: String, body: String) -> Element {
    rsx! {
        div { class: "card bg-base-100 shadow-soft border border-dashed border-base-300",
            div { class: "card-body gap-2",
                div { class: "flex items-center justify-between gap-2",
                    h2 { class: "card-title text-base text-base-content/60", {icon} {title} }
                    span { class: "badge badge-ghost badge-sm gap-1 shrink-0",
                        LockIcon { size: ICON_INLINE }
                        {tid!("settings-locked-badge")}
                    }
                }
                p { class: "text-sm text-base-content/70", {body} }
            }
        }
    }
}
