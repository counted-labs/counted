use dioxus::prelude::*;
use crate::tid;

use crate::common::{ConfirmModal, Entry, SheetBar, SHEET_TITLE, ListRow};

/// The queued changes this build will not send as things stand (SPEC §15.9.5 Q1, Q5), each one
/// its user may discard (Q3) behind a confirmation.
#[component]
pub fn UnsentSheet(unsent: Vec<(Entry, String)>, on_discard: EventHandler<Entry>, on_close: EventHandler<()>) -> Element {
    crate::common::use_modal_focus("unsent-sheet");
    let mut asked: Signal<Option<Entry>> = use_signal(|| None);

    rsx! {
        div {
            id: "unsent-sheet",
            class: "modal modal-open modal-bottom sm:modal-middle",
            role: "dialog",
            aria_modal: "true",
            aria_labelledby: "unsent-title",
            aria_describedby: "unsent-body",
            tabindex: "-1",
            div { class: "modal-box max-w-md p-0 flex flex-col",
                SheetBar { close_id: "unsent-close", on_close,
                    h2 { id: "unsent-title", class: SHEET_TITLE, {tid!("unsent-title")} }
                }
                div { class: "flex-1 overflow-y-auto flex flex-col gap-3 px-5 pt-1 pb-4",
                    p { id: "unsent-body", class: "text-sm text-base-content/70", {tid!("unsent-body")} }
                    ul { class: "list-group",
                        for (i, (entry, name)) in unsent.iter().cloned().enumerate() {
                            ListRow {
                                key: "{i}",
                                id: format!("unsent-{i}"),
                                long_press: format!("unsent-{i}-press"),
                                title: if name.is_empty() { tid!("unsent-unnamed") } else { name },
                                onclick: move |_| asked.set(Some(entry.clone())),
                            }
                        }
                    }
                }
            }
            div { class: "modal-backdrop", onclick: move |_| on_close.call(()) }
        }
        if let Some(entry) = asked() {
            ConfirmModal {
                title: tid!("unsent-discard-title"),
                message: tid!("unsent-discard-message"),
                confirm_label: tid!("discard"),
                action_sheet: true,
                on_cancel: move |_| asked.set(None),
                on_confirm: move |_| {
                    asked.set(None);
                    on_discard.call(entry.clone());
                },
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
    fn Harness(unsent: Vec<(Entry, String)>) -> Element {
        crate::i18n::use_test_i18n();
        rsx! {
            UnsentSheet { unsent, on_discard: |_| {}, on_close: |_| {} }
        }
    }

    #[test]
    fn each_unsent_change_is_listed_by_name_and_an_unnamed_one_says_so() {
        let entry = |n: i64| Entry::read(serde_json::json!({ "op": { "type": "mystery", "n": n } }));
        let unsent = vec![(entry(1), "Hôtel".to_string()), (entry(2), String::new())];
        let mut dom = VirtualDom::new_with_props(Harness, HarnessProps { unsent });
        let shown = texts(&dom.rebuild_to_vec());
        assert!(shown.iter().any(|t| t == "Hôtel"), "{shown:?}");
        assert!(shown.iter().any(|t| t == "Unnamed change"), "{shown:?}");
    }
}
