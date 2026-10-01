use dioxus::prelude::*;

use crate::common::{format_date_str, haptic, ConfirmModal, Haptic};
use crate::tid;

/// Deleting an expense. An occurrence of a live rule also offers to stop the rule, and says the
/// deleted date will not come back; `on_delete` carries whether to stop it.
#[component]
pub fn DeleteExpenseDialog(
    name: String,
    date: String,
    occurrence: bool,
    on_cancel: EventHandler<()>,
    on_delete: EventHandler<bool>,
) -> Element {
    if !occurrence {
        return rsx! {
            ConfirmModal {
                title: tid!("expense-delete-title"),
                message: tid!("expense-delete-message", name: name.clone()),
                confirm_label: tid!("delete"),
                on_cancel: move |_| on_cancel.call(()),
                on_confirm: move |_| on_delete.call(false),
            }
        };
    }
    rsx! {
        ChoiceDialog {
            title: tid!("expense-delete-title"),
            message: tid!("occurrence-delete-message", name: name.clone(), date: format_date_str(&date)),
            actions: vec![
                DialogAction {
                    id: "occurrence-delete-one",
                    label: tid!("occurrence-delete-one"),
                    class: "btn-error",
                    on_click: EventHandler::new(move |_| on_delete.call(false)),
                },
                DialogAction {
                    id: "occurrence-delete-stop",
                    label: tid!("occurrence-delete-stop"),
                    class: "btn-outline btn-error",
                    on_click: EventHandler::new(move |_| on_delete.call(true)),
                },
            ],
            on_cancel: move |_| on_cancel.call(()),
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct DialogAction {
    pub id: &'static str,
    pub label: String,
    /// daisyUI button classes, e.g. `btn-error` for the irreversible one.
    pub class: &'static str,
    pub on_click: EventHandler<()>,
}

/// A confirmation with several outcomes, stacked full width, Cancel last.
#[component]
pub fn ChoiceDialog(
    title: String,
    message: String,
    actions: Vec<DialogAction>,
    on_cancel: EventHandler<()>,
) -> Element {
    rsx! {
        div { class: "modal modal-open", role: "dialog", aria_labelledby: "choice-dialog-title",
            div { class: "modal-box max-w-sm flex flex-col gap-3",
                h3 { id: "choice-dialog-title", class: "font-bold text-lg font-display", "{title}" }
                p { class: "text-sm text-base-content/70", "{message}" }
                div { class: "flex flex-col gap-2 pt-1",
                    for action in actions {
                        button {
                            id: action.id,
                            r#type: "button",
                            class: "btn w-full {action.class}",
                            onclick: move |_| {
                                haptic(Haptic::Light);
                                action.on_click.call(())
                            },
                            "{action.label}"
                        }
                    }
                    button {
                        id: "choice-dialog-cancel",
                        r#type: "button",
                        class: "btn btn-ghost w-full",
                        onclick: move |_| on_cancel.call(()),
                        {tid!("cancel")}
                    }
                }
            }
            div { class: "modal-backdrop", onclick: move |_| on_cancel.call(()) }
        }
    }
}
