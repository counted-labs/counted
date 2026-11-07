use dioxus::prelude::*;

pub use crate::common::DialogAction;
use crate::common::{format_date_str, ConfirmDialog, ConfirmModal};
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
                action_sheet: true,
                on_cancel: move |_| on_cancel.call(()),
                on_confirm: move |_| on_delete.call(false),
            }
        };
    }
    rsx! {
        ConfirmDialog {
            id: "choice-dialog",
            title: tid!("expense-delete-title"),
            message: tid!("occurrence-delete-message", name: name.clone(), date: format_date_str(&date)),
            action_sheet: true,
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

/// A confirmation with several outcomes on a recurring rule: the app's one confirm.
#[component]
pub fn ChoiceDialog(
    title: String,
    message: String,
    actions: Vec<DialogAction>,
    on_cancel: EventHandler<()>,
) -> Element {
    rsx! {
        ConfirmDialog { id: "choice-dialog", title, message, actions, on_cancel }
    }
}
