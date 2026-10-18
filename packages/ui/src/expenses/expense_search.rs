use dioxus::prelude::*;

use crate::expenses::helpers::expense_query::{mark_segments, ExpenseQuery};
use crate::icons::{CloseIcon, SearchIcon, ICON_INLINE};
use crate::tid;

/// The open search field. `autofocus` only when the user just opened it: coming back from an
/// expense remounts it, and focusing then would raise the keyboard unasked. `closes`: no toggle
/// beside it, so its clear button is always there and also closes the field.
#[component]
pub fn ExpenseSearchField(
    query: Signal<ExpenseQuery>,
    autofocus: bool,
    on_change: EventHandler<()>,
    #[props(default)] closes: bool,
) -> Element {
    let text = query.read().text.clone();
    rsx! {
        label { class: "input w-full h-11 rounded-full bg-base-100 border-primary ring-3 ring-primary/15 pe-1.5",
            SearchIcon { size: 18 }
            input {
                id: "expense-search",
                class: "grow min-w-0 [&::-webkit-search-cancel-button]:hidden",
                r#type: "search",
                enterkeyhint: "search",
                autocomplete: "off",
                aria_label: tid!("expenses-search"),
                value: "{text}",
                onmounted: move |e| async move {
                    if autofocus {
                        let _ = e.data().set_focus(true).await;
                    }
                },
                oninput: move |e| {
                    query.write().text = e.value();
                    on_change.call(());
                },
            }
            if closes || !text.is_empty() {
                button {
                    id: "expense-search-clear",
                    r#type: "button",
                    class: "btn btn-circle btn-sm size-9 border-0 bg-base-200",
                    aria_label: tid!("expenses-search-clear"),
                    onclick: move |_| {
                        {
                            let mut q = query.write();
                            q.text.clear();
                            if closes {
                                q.searching = false;
                            }
                        }
                        on_change.call(());
                    },
                    CloseIcon { size: 14 }
                }
            }
        }
    }
}

/// `text` with the parts the search matched in `<mark>`.
#[component]
pub fn Marked(text: String, words: Vec<String>) -> Element {
    rsx! {
        for (part , marked) in mark_segments(&text, &words) {
            if marked {
                mark { class: "bg-warning/35 text-inherit rounded-[3px] px-px", "{part}" }
            } else {
                "{part}"
            }
        }
    }
}

#[component]
pub fn SearchToggle(open: bool, onclick: EventHandler<()>) -> Element {
    rsx! {
        button {
            id: "expense-search-toggle",
            r#type: "button",
            class: if open { "btn btn-sm btn-square shrink-0 btn-primary btn-soft" } else { "btn btn-sm btn-square shrink-0" },
            aria_label: tid!("expenses-search-open"),
            aria_pressed: open,
            onclick: move |_| onclick.call(()),
            SearchIcon { size: ICON_INLINE }
        }
    }
}
