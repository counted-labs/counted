use dioxus::prelude::*;

use crate::icons::{CloseIcon, ICON_HEADER};
use crate::tid;

pub const SHEET_TITLE: &str = "flex-1 min-w-0 break-words font-bold text-lg font-display sheet-bar-title";

/// The top bar of every sheet (docs/plans/sheet-consistency.md): close leading, the title or a
/// control after it. A sheet's confirm lives in its `SheetFooter`, never here. One markup for every
/// platform; `native-feel.css` draws the close per platform: an xmark on iOS 26, Android and the
/// web, its name as text on iOS 16-18 ("Cancel" on a task sheet, `task`, else "Close"). With no
/// `on_close` the sheet cannot be dismissed and the bar holds the title alone.
#[component]
pub fn SheetBar(
    #[props(default)] close_id: String,
    #[props(default)] on_close: Option<EventHandler<()>>,
    #[props(default)] task: bool,
    children: Element,
) -> Element {
    rsx! {
        div { class: if on_close.is_some() { "sheet-bar flex items-center gap-2 px-4 pt-3 pb-2 flex-shrink-0" } else { "sheet-bar flex items-center gap-2 px-5 pt-3 pb-2 flex-shrink-0" },
            if let Some(on_close) = on_close {
                button {
                    id: if !close_id.is_empty() { close_id.clone() },
                    r#type: "button",
                    class: "sheet-bar-close btn btn-circle size-11 min-h-11 border-0 bg-base-200 text-base-content/80 shrink-0",
                    onclick: move |_| on_close.call(()),
                    span { class: "sheet-bar-icon", aria_hidden: "true", CloseIcon { size: ICON_HEADER } }
                    span { class: "sheet-bar-text", if task { {tid!("cancel")} } else { {tid!("close")} } }
                }
            }
            div { class: "flex-1 min-w-0 flex items-center gap-2", {children} }
            if on_close.is_some() {
                span { class: "sheet-bar-spacer size-11 shrink-0", aria_hidden: "true" }
            }
        }
    }
}
