use dioxus::prelude::*;
use crate::tid;

use crate::auth::erase_device::EraseCheck;
use crate::common::{haptic, Haptic};

#[derive(PartialEq, Props, Clone)]
pub struct EraseDeviceSheetProps {
    pub check: EraseCheck,
    pub erasing: bool,
    pub on_close: EventHandler<()>,
    pub on_confirm: EventHandler<()>,
}

#[component]
pub fn EraseDeviceSheet(props: EraseDeviceSheetProps) -> Element {
    crate::common::use_modal_focus("erase-device-sheet");
    let on_close = props.on_close;
    let on_confirm = props.on_confirm;
    let at_risk = match &props.check {
        EraseCheck::Ready(names) => Some(names.clone()),
        _ => None,
    };

    rsx! {
        div {
            id: "erase-device-sheet",
            class: "modal modal-open modal-bottom sm:modal-middle",
            role: "dialog",
            aria_modal: "true",
            aria_labelledby: "erase-device-title",
            aria_describedby: "erase-device-body",
            tabindex: "-1",
            div { class: "modal-box max-w-md p-0 flex flex-col",
                div { class: "flex flex-col gap-3 px-6 pt-5 pb-4",
                    h2 { id: "erase-device-title", class: "font-bold text-lg font-display", {tid!("erase-device-title")} }
                    p { id: "erase-device-body", class: "text-sm text-base-content/70", {tid!("erase-device-body")} }
                    match &props.check {
                        EraseCheck::Checking => rsx! {
                            p { id: "erase-device-checking", class: "flex items-center gap-2 text-sm text-base-content/70",
                                span { class: "loading loading-spinner loading-sm", aria_hidden: "true" }
                                {tid!("erase-device-checking")}
                            }
                        },
                        EraseCheck::Pending(count) => rsx! {
                            div { id: "erase-device-pending", role: "alert", class: "alert alert-warning text-sm",
                                {tid!("erase-device-pending", count: *count as i64)}
                            }
                        },
                        EraseCheck::Unreachable => rsx! {
                            div { id: "erase-device-unreachable", role: "alert", class: "alert alert-warning text-sm",
                                {tid!("erase-device-unreachable")}
                            }
                        },
                        EraseCheck::Ready(names) if !names.is_empty() => rsx! {
                            div { id: "erase-device-at-risk", role: "alert", class: "alert alert-warning text-sm flex-col items-start",
                                p { {tid!("erase-device-at-risk")} }
                                ul { class: "list-disc pl-5",
                                    for name in names.iter() {
                                        li { class: "break-all", "{name}" }
                                    }
                                }
                            }
                        },
                        EraseCheck::Ready(_) => rsx! {},
                    }
                }
                div { class: "flex justify-end gap-2 px-6 py-4 border-t border-base-200",
                    button {
                        id: "erase-device-cancel",
                        r#type: "button",
                        class: "btn",
                        onclick: move |_| on_close.call(()),
                        {tid!("cancel")}
                    }
                    if let Some(names) = at_risk {
                        button {
                            id: "erase-device-confirm",
                            r#type: "button",
                            class: "btn btn-error",
                            disabled: props.erasing,
                            onclick: move |_| {
                                haptic(Haptic::Warning);
                                on_confirm.call(())
                            },
                            if props.erasing {
                                {tid!("erase-device-erasing")}
                            } else if names.is_empty() {
                                {tid!("erase-device-confirm")}
                            } else {
                                {tid!("erase-device-confirm-anyway")}
                            }
                        }
                    }
                }
            }
            div { class: "modal-backdrop", onclick: move |_| on_close.call(()) }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::test_dom::texts;
    use dioxus::dioxus_core::VirtualDom;

    #[component]
    fn Harness(check: EraseCheck) -> Element {
        crate::i18n::use_test_i18n();
        rsx! {
            EraseDeviceSheet { check, erasing: false, on_close: |_| {}, on_confirm: |_| {} }
        }
    }

    fn render(check: EraseCheck) -> Vec<String> {
        let mut dom = VirtualDom::new_with_props(Harness, HarnessProps { check });
        texts(&dom.rebuild_to_vec())
    }

    #[test]
    fn the_erase_button_appears_only_once_the_check_is_done() {
        for check in [EraseCheck::Checking, EraseCheck::Pending(2), EraseCheck::Unreachable] {
            let shown = render(check.clone());
            assert!(!shown.iter().any(|t| t == "Erase and sign out" || t == "Erase anyway"), "{check:?}: {shown:?}");
        }
        assert!(render(EraseCheck::Ready(vec![])).iter().any(|t| t == "Erase and sign out"));
    }

    #[test]
    fn projects_that_would_be_lost_are_named_and_the_button_says_so() {
        let shown = render(EraseCheck::Ready(vec!["Ski trip".into()]));
        assert!(shown.iter().any(|t| t == "Ski trip"), "{shown:?}");
        assert!(shown.iter().any(|t| t == "Erase anyway"), "{shown:?}");
    }
}
