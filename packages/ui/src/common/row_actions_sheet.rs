use dioxus::prelude::*;

use crate::common::{SheetBar, SHEET_TITLE};

/// A list row's actions as a bottom sheet: what a long press on the row opens (boot.js), and what
/// its visually hidden actions button opens for a screen reader or a keyboard. The children are
/// `project_actions_sheet::Row`s, each closing the sheet before it acts.
#[component]
pub fn RowActionsSheet(title: String, on_close: EventHandler<()>, children: Element) -> Element {
    crate::common::use_modal_focus("row-actions-sheet");
    rsx! {
        div {
            id: "row-actions-sheet",
            class: "modal modal-open modal-bottom sm:modal-middle",
            role: "dialog",
            aria_modal: "true",
            aria_labelledby: "row-actions-title",
            tabindex: "-1",
            div { class: "modal-box max-w-md p-0 flex flex-col",
                SheetBar { close_id: "row-actions-close", on_close,
                    h2 { id: "row-actions-title", class: SHEET_TITLE, "{title}" }
                }
                div { class: "flex-1 overflow-y-auto px-5 pt-1 pb-6",
                    div { class: "bg-base-200 rounded-box overflow-hidden divide-y divide-base-300", {children} }
                }
            }
            div { class: "modal-backdrop", onclick: move |_| on_close.call(()) }
        }
    }
}

/// The two hidden ways into a row's actions sheet. `long_press_id` is the button boot.js clicks on
/// a long press (a Medium haptic, as a committed gesture); the other is the row's actions entry
/// for TalkBack, VoiceOver and the keyboard, out of sight until it has focus (a Light haptic, as a
/// tap on a trigger).
#[component]
pub fn RowActionsEntry(long_press_id: String, label: String, open: bool, on_open: EventHandler<()>) -> Element {
    rsx! {
        button {
            id: long_press_id,
            r#type: "button",
            hidden: true,
            tabindex: "-1",
            onclick: move |_| {
                crate::common::haptic(crate::common::Haptic::Medium);
                on_open.call(());
            },
        }
        button {
            r#type: "button",
            class: "sr-only focus:not-sr-only btn btn-ghost btn-sm",
            aria_label: label,
            aria_haspopup: "dialog",
            aria_expanded: open,
            onclick: move |_| {
                crate::common::haptic(crate::common::Haptic::Light);
                on_open.call(());
            },
            "..."
        }
    }
}
