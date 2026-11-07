use dioxus::prelude::*;

use crate::expenses::project_actions_sheet::{MenuSheet, Row, GROUP_CLASS};
use crate::icons::{PencilIcon, RightArrowIcon, TrashIcon, ICON_HEADER};
use crate::tid;

/// The expense page's ⋯ actions. Every entry closes the sheet, then calls out.
#[component]
pub fn ExpenseActionsSheet(
    title: String,
    can_edit: bool,
    movable: bool,
    can_delete: bool,
    on_close: EventHandler<()>,
    on_edit: EventHandler<()>,
    on_move: EventHandler<()>,
    on_delete: EventHandler<()>,
) -> Element {
    let pick = move |action: EventHandler<()>| {
        move |_: ()| {
            on_close.call(());
            action.call(());
        }
    };
    rsx! {
        MenuSheet { id: "expense-actions-sheet", title, on_close,
            if can_edit || movable {
                div { class: GROUP_CLASS,
                    if can_edit {
                        Row {
                            id: "edit-expense-item",
                            label: tid!("edit"),
                            onclick: pick(on_edit),
                            PencilIcon { size: ICON_HEADER }
                        }
                    }
                    if movable {
                        Row {
                            id: "move-expense-item",
                            label: tid!("move-expense"),
                            onclick: pick(on_move),
                            RightArrowIcon { size: ICON_HEADER }
                        }
                    }
                }
            }
            if can_delete {
                div { class: GROUP_CLASS,
                    Row {
                        id: "delete-expense-item",
                        label: tid!("delete"),
                        danger: true,
                        onclick: pick(on_delete),
                        TrashIcon { size: ICON_HEADER }
                    }
                }
            }
        }
    }
}
