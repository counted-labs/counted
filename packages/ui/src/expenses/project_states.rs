//! The project page when it has no data to show: no key, no cache, or nothing loaded yet.

use dioxus::prelude::*;
use crate::tid;

/// The share link was opened without its `#fragment` and this device never stored the key.
///
/// `on_unlock` is optional so the screen stays usable where there is nothing to open — the caller
/// owns the modal, this component only says when it should appear.
#[derive(PartialEq, Props, Clone)]
pub struct MissingKeyScreenProps {
    #[props(default)]
    pub on_unlock: Option<EventHandler<()>>,
}

#[component]
pub fn MissingKeyScreen(props: MissingKeyScreenProps) -> Element {
    rsx! {
        div { class: "flex flex-col items-center gap-3 py-8 text-center",
            p { class: "font-semibold", {tid!("missing-encryption-key-title")} }
            p { class: "text-sm text-base-content/70",
                {tid!("missing-encryption-key-hint")}
            }
            if let Some(on_unlock) = props.on_unlock {
                button {
                    id: "unlock-project-btn",
                    r#type: "button",
                    class: "btn btn-primary btn-sm",
                    onclick: move |_| on_unlock.call(()),
                    {tid!("project-unlock")}
                }
            }
        }
    }
}

/// Offline on a project this device has never loaded — nothing cached to fall back to.
#[component]
pub fn NoLocalDataScreen() -> Element {
    rsx! {
        div { class: "flex flex-col items-center gap-4 py-12 text-base-content/70",
            p { class: "font-semibold", {tid!("projects-no-local-data")} }
            p { class: "text-sm text-center",
                {tid!("project-no-local-data-hint")}
            }
        }
    }
}

#[derive(PartialEq, Props, Clone)]
pub struct SpinnerProps {
    #[props(default = "py-4".to_string())]
    pub padding_class: String,
}

#[component]
pub fn Spinner(props: SpinnerProps) -> Element {
    rsx! {
        div { class: "flex justify-center {props.padding_class}",
            span { class: "loading loading-spinner loading-md", role: "status", aria_label: tid!("loading") }
        }
    }
}

#[component]
pub fn ExpensesSkeleton() -> Element {
    rsx! {
        div { class: "flex flex-col gap-2", aria_hidden: "true",
            div { class: "flex justify-end",
                div { class: "skeleton h-8 w-8 rounded-full" }
            }
            for _ in 0..5u8 {
                div { class: "flex items-center gap-3 p-3 bg-base-100 rounded-box",
                    div { class: "skeleton h-10 w-10 rounded-full shrink-0" }
                    div { class: "flex-1 flex flex-col gap-2",
                        div { class: "skeleton h-4 w-3/4" }
                        div { class: "skeleton h-3 w-1/2" }
                    }
                    div { class: "skeleton h-4 w-16 shrink-0" }
                }
            }
        }
    }
}
