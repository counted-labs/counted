use dioxus::prelude::*;
use crate::tid;

use crate::common::{haptic, Haptic};

/// Yes/no dialog for an action that cannot be undone. Same shell as `UserSelectionModal`.
#[derive(PartialEq, Props, Clone)]
pub struct ConfirmModalProps {
    pub title: String,
    pub message: String,
    pub confirm_label: String,
    pub on_confirm: EventHandler<()>,
    pub on_cancel: EventHandler<()>,
}

#[component]
pub fn ConfirmModal(props: ConfirmModalProps) -> Element {
    rsx! {
        div { class: "modal modal-open", role: "dialog",
            div { class: "modal-box max-w-sm p-0 flex flex-col",
                div { class: "px-6 pt-5 pb-4 border-b border-base-200",
                    h3 { class: "font-bold text-lg font-display", "{props.title}" }
                }
                div { class: "px-6 py-4",
                    p { class: "text-sm text-base-content/70", "{props.message}" }
                }
                div { class: "flex justify-end gap-2 px-6 py-4 border-t border-base-200",
                    // Fixed ids, not props: only ever one confirmation is on screen at a time, and
                    // the E2E suite selects on id (docs/e2e.md).
                    button {
                        id: "confirm-modal-cancel",
                        r#type: "button",
                        class: "btn",
                        onclick: move |_| props.on_cancel.call(()),
                        {tid!("cancel")}
                    }
                    button {
                        id: "confirm-modal-confirm",
                        r#type: "button",
                        class: "btn btn-error",
                        // Warning, not Success: this button is only ever the irreversible half.
                        onclick: move |_| {
                            haptic(Haptic::Warning);
                            props.on_confirm.call(())
                        },
                        "{props.confirm_label}"
                    }
                }
            }
            div { class: "modal-backdrop", onclick: move |_| props.on_cancel.call(()) }
        }
    }
}
