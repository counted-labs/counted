use dioxus::prelude::*;
use crate::tid;

use crate::auth::erase_device::{EraseCheck, Unconfirmed};
use crate::common::{haptic, Haptic, SheetBar, SheetFooter, SHEET_TITLE};

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
    let ready = match &props.check {
        EraseCheck::Ready { at_risk, unconfirmed } => Some(!at_risk.is_empty() || unconfirmed.count > 0),
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
                SheetBar { close_id: "erase-device-close", task: true, on_close,
                    h2 { id: "erase-device-title", class: SHEET_TITLE, {tid!("erase-device-title")} }
                }
                div { class: "flex-1 overflow-y-auto flex flex-col gap-3 px-5 pt-1 pb-4",
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
                        EraseCheck::Unsent(count) => rsx! {
                            div { id: "erase-device-pending", role: "alert", class: "alert alert-warning text-sm",
                                {tid!("erase-device-pending-unsent", count: *count as i64)}
                            }
                        },
                        EraseCheck::Confirming => rsx! {
                            div { id: "erase-device-confirming", role: "alert", class: "alert alert-warning text-sm",
                                {tid!("project-still-confirming")}
                            }
                        },
                        EraseCheck::Unreachable => rsx! {
                            div { id: "erase-device-unreachable", role: "alert", class: "alert alert-warning text-sm",
                                {tid!("erase-device-unreachable")}
                            }
                        },
                        EraseCheck::Ready { at_risk, unconfirmed } => rsx! {
                            if unconfirmed.count > 0 {
                                UnconfirmedWarning { unconfirmed: unconfirmed.clone() }
                            }
                            if !at_risk.is_empty() {
                                div { id: "erase-device-at-risk", role: "alert", class: "alert alert-warning text-sm flex-col items-start",
                                    p { {tid!("erase-device-at-risk")} }
                                    ul { class: "list-disc pl-5",
                                        for name in at_risk.iter() {
                                            li { class: "break-all", "{name}" }
                                        }
                                    }
                                }
                            }
                        },
                    }
                }
                if let Some(loses) = ready {
                    SheetFooter {
                        on_cancel: on_close,
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
                            } else if loses {
                                {tid!("erase-device-confirm-anyway")}
                            } else {
                                {tid!("erase-device-confirm")}
                            }
                        }
                    }
                }
            }
            div { class: "modal-backdrop", onclick: move |_| on_close.call(()) }
        }
    }
}

#[component]
fn UnconfirmedWarning(unconfirmed: Unconfirmed) -> Element {
    rsx! {
        div { id: "erase-device-unconfirmed", role: "alert", class: "alert alert-warning text-sm flex-col items-start",
            p { {tid!("erase-device-unconfirmed")} }
            if unconfirmed.other_account {
                p { {tid!("erase-device-unconfirmed-other-account")} }
            }
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
        for check in [EraseCheck::Checking, EraseCheck::Pending(2), EraseCheck::Unsent(1), EraseCheck::Confirming, EraseCheck::Unreachable] {
            let shown = render(check.clone());
            assert!(!shown.iter().any(|t| t == "Erase and sign out" || t == "Erase anyway"), "{check:?}: {shown:?}");
        }
        assert!(render(ready(vec![], 0, false)).iter().any(|t| t == "Erase and sign out"));
    }

    fn ready(at_risk: Vec<String>, count: usize, other_account: bool) -> EraseCheck {
        EraseCheck::Ready { at_risk, unconfirmed: Unconfirmed { count, other_account } }
    }

    #[test]
    fn a_creation_that_may_still_commit_says_so_and_offers_no_erase() {
        let shown = render(EraseCheck::Confirming);
        assert!(shown.iter().any(|t| t.contains("still being confirmed")), "{shown:?}");
    }

    #[test]
    fn an_unconfirmed_creation_is_named_as_a_loss_and_can_be_erased_anyway() {
        let shown = render(ready(vec![], 1, false));
        assert!(shown.iter().any(|t| t.contains("not been confirmed by the server")), "{shown:?}");
        assert!(!shown.iter().any(|t| t.contains("another account")), "{shown:?}");
        assert!(shown.iter().any(|t| t == "Erase anyway"), "{shown:?}");

        let shown = render(ready(vec![], 1, true));
        assert!(shown.iter().any(|t| t.contains("another account")), "{shown:?}");
    }

    #[test]
    fn projects_that_would_be_lost_are_named_and_the_button_says_so() {
        let shown = render(ready(vec!["Ski trip".into()], 0, false));
        assert!(shown.iter().any(|t| t == "Ski trip"), "{shown:?}");
        assert!(shown.iter().any(|t| t == "Erase anyway"), "{shown:?}");
    }
}
