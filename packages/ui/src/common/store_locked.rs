use dioxus::prelude::*;

use crate::icons::KeyIcon;
use crate::tid;

/// Shown instead of the app while the phone's store cannot be opened (docs/plans/security-3.3.0.md
/// §4): `lost` when the platform says the key is gone, otherwise locked (try again after unlocking
/// the phone). Reset is a second step, since it gives up what is not saved to an account.
#[component]
pub fn StoreLocked(lost: bool, on_retry: EventHandler<()>, on_reset: EventHandler<()>) -> Element {
    let mut confirming = use_signal(|| false);
    rsx! {
        div { class: "container app-container bg-base-100 min-h-dvh max-w-md mx-auto p-4 flex flex-col justify-center",
            div { id: "store-locked", role: "alert", class: "rounded-box border border-warning/50 bg-base-100 p-4 flex flex-col gap-2.5",
                div { class: "flex items-center gap-2.5",
                    span { class: "text-warning shrink-0", KeyIcon { size: 22 } }
                    h1 { class: "font-display font-bold text-[1.0625rem] leading-snug",
                        if lost { {tid!("store-lost-title")} } else { {tid!("store-locked-title")} }
                    }
                }
                p { class: "text-sm text-base-content/80 leading-snug",
                    if lost { {tid!("store-lost-body")} } else { {tid!("store-locked-body")} }
                }
                if confirming() {
                    p { id: "store-reset-warning", class: "text-sm text-warning leading-snug", {tid!("store-reset-warning")} }
                }
                div { class: "flex flex-col gap-2",
                    if !lost {
                        button {
                            id: "store-retry",
                            r#type: "button",
                            class: "btn btn-primary",
                            onclick: move |_| on_retry.call(()),
                            {tid!("store-retry")}
                        }
                    }
                    if confirming() {
                        button {
                            id: "store-reset-confirm",
                            r#type: "button",
                            class: "btn btn-error",
                            onclick: move |_| on_reset.call(()),
                            {tid!("store-reset-confirm")}
                        }
                    } else {
                        button {
                            id: "store-reset",
                            r#type: "button",
                            class: if lost { "btn btn-primary" } else { "btn btn-ghost" },
                            onclick: move |_| confirming.set(true),
                            {tid!("store-reset")}
                        }
                    }
                }
            }
        }
    }
}
