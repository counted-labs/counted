use dioxus::prelude::*;
use crate::tid;

use crate::common::{answer_key_offer, haptic, key_offer_shown, use_modal_focus, Haptic, LocalStorageState, SheetBar, SheetFooter, SHEET_TITLE};

/// Mounted once in `AppLayout`, after the page, so it covers any sheet the link was opened from.
#[component]
pub fn KeyOfferSheet() -> Element {
    let ls_ctx = use_context::<Signal<LocalStorageState>>();
    let Some(name) = key_offer_shown() else { return rsx! {} };
    rsx! {
        KeyOfferDialog {
            name,
            on_use: move |_| answer_key_offer(ls_ctx, true),
            on_keep: move |_| answer_key_offer(ls_ctx, false),
        }
    }
}

#[derive(PartialEq, Props, Clone)]
pub struct KeyOfferDialogProps {
    pub name: String,
    pub on_use: EventHandler<()>,
    pub on_keep: EventHandler<()>,
}

#[component]
pub fn KeyOfferDialog(props: KeyOfferDialogProps) -> Element {
    use_modal_focus("key-offer-sheet");
    let on_use = props.on_use;
    let on_keep = props.on_keep;

    rsx! {
        div {
            id: "key-offer-sheet",
            class: "modal modal-open modal-bottom sm:modal-middle",
            role: "dialog",
            aria_modal: "true",
            aria_labelledby: "key-offer-title",
            aria_describedby: "key-offer-body",
            tabindex: "-1",
            div { class: "modal-box max-w-md p-0 flex flex-col",
                SheetBar { close_id: "key-offer-close", task: true, on_close: on_keep,
                    h2 { id: "key-offer-title", class: SHEET_TITLE, {tid!("key-offer-title", name: props.name.clone())} }
                }
                div { id: "key-offer-body", class: "flex-1 overflow-y-auto flex flex-col gap-2 px-5 pt-1 pb-4 text-sm text-base-content/70",
                    p { {tid!("key-offer-body")} }
                    p { {tid!("key-offer-choices")} }
                }
                SheetFooter {
                    button {
                        id: "key-offer-keep",
                        r#type: "button",
                        class: "btn",
                        onclick: move |_| {
                            haptic(Haptic::Light);
                            on_keep.call(())
                        },
                        {tid!("cancel")}
                    }
                    button {
                        id: "key-offer-use",
                        r#type: "button",
                        class: "btn btn-primary",
                        onclick: move |_| {
                            haptic(Haptic::Light);
                            on_use.call(())
                        },
                        {tid!("key-offer-use")}
                    }
                }
            }
            div { class: "modal-backdrop", onclick: move |_| on_keep.call(()) }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::test_dom::texts;
    use dioxus::dioxus_core::VirtualDom;

    #[component]
    fn Harness() -> Element {
        crate::i18n::use_test_i18n();
        rsx! {
            KeyOfferDialog { name: "Ski trip".to_string(), on_use: |_| {}, on_keep: |_| {} }
        }
    }

    #[test]
    fn the_sheet_names_the_project_and_offers_both_choices() {
        let mut dom = VirtualDom::new(Harness);
        let shown = texts(&dom.rebuild_to_vec()).join(" ");
        assert!(shown.contains("Ski trip"), "{shown}");
        assert!(shown.contains("Use this key"), "{shown}");
        assert!(shown.contains("Cancel"), "{shown}");
    }
}
