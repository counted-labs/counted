use dioxus::prelude::*;

use crate::icons::{CheckIcon, CloseIcon, ICON_HEADER};
use crate::tid;

/// The top bar of a form sheet: close leading, confirm trailing, the title or a control between.
/// One markup for every platform; `native-feel.css` draws it per platform: iOS 26 and the web an
/// xmark and a tinted checkmark, iOS 16-18 the text buttons "Cancel" and a bold confirm, Android
/// the xmark and a text confirm (the M3 full-screen dialog bar). Each button carries both its icon
/// and its text, so the text is the accessible name whichever is shown. The confirm submits
/// `form` by id when given, else calls `on_confirm`; with neither, the bar has no confirm and a
/// spacer keeps the title centred. The form's own bottom button stays.
#[component]
pub fn FormSheetBar(
    close_id: &'static str,
    #[props(default)] confirm_id: &'static str,
    #[props(default)] confirm_label: String,
    on_close: EventHandler<()>,
    #[props(default)] form: Option<&'static str>,
    #[props(default)] on_confirm: Option<EventHandler<()>>,
    #[props(default)] disabled: bool,
    children: Element,
) -> Element {
    rsx! {
        div { class: "sheet-bar flex items-center gap-2",
            button {
                id: close_id,
                r#type: "button",
                class: "sheet-bar-close btn btn-circle size-11 min-h-11 border-0 bg-base-200 text-base-content/80 shrink-0",
                onclick: move |_| on_close.call(()),
                span { class: "sheet-bar-icon", aria_hidden: "true", CloseIcon { size: ICON_HEADER } }
                span { class: "sheet-bar-text", {tid!("cancel")} }
            }
            div { class: "flex-1 min-w-0 flex items-center", {children} }
            if form.is_none() && on_confirm.is_none() {
                span { class: "size-11 shrink-0", aria_hidden: "true" }
            } else {
            button {
                id: confirm_id,
                r#type: if form.is_some() { "submit" } else { "button" },
                form,
                class: "sheet-bar-confirm btn btn-circle btn-primary size-11 min-h-11 border-0 shrink-0",
                disabled,
                onclick: move |_| {
                    if let Some(on_confirm) = on_confirm {
                        on_confirm.call(());
                    }
                },
                span { class: "sheet-bar-icon", aria_hidden: "true", CheckIcon { size: ICON_HEADER } }
                span { class: "sheet-bar-text", "{confirm_label}" }
            }
            }
        }
    }
}
