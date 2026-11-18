use dioxus::prelude::*;

use crate::tid;

/// The bottom of a task sheet (docs/plans/sheet-consistency.md): pinned under the scrolling body,
/// so it rides above the keyboard. On a phone the primary (`children`) fills the width and Cancel
/// is hidden, since the bar's close, the backdrop, the drag and Android back already cancel; in the
/// centred dialog (640px and up) Cancel comes back before the primary. `secondary` (Delete, Back)
/// sits on the leading side.
#[component]
pub fn SheetFooter(
    #[props(default)] secondary: Option<Element>,
    #[props(default)] on_cancel: Option<EventHandler<()>>,
    children: Element,
) -> Element {
    rsx! {
        div { class: "sheet-footer flex items-center gap-2 px-5 py-3 border-t border-base-200 flex-shrink-0",
            {secondary.unwrap_or_else(|| rsx! {})}
            div { class: "flex-1 flex gap-2 sm:flex-none sm:ml-auto [&>.btn]:flex-1 sm:[&>.btn]:flex-none",
                if let Some(on_cancel) = on_cancel {
                    button {
                        r#type: "button",
                        class: "btn btn-ghost hidden sm:inline-flex",
                        onclick: move |_| on_cancel.call(()),
                        {tid!("cancel")}
                    }
                }
                {children}
            }
        }
    }
}
