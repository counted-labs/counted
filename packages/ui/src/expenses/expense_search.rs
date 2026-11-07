use dioxus::prelude::*;

use crate::expenses::helpers::expense_query::{mark_segments, ExpenseQuery};
use crate::icons::{CloseIcon, SearchIcon, ICON_INLINE};
use crate::tid;

/// The open search field. `autofocus` only when the user just opened it: coming back from an
/// expense remounts it, and focusing then would raise the keyboard unasked. `closes`: no toggle
/// beside it, so its clear button is always there and also closes the field.
///
/// A UISearchBar's shape: a filled field with a grey magnifier, and while it has focus a Cancel
/// beside it that empties and blurs it (and closes it where nothing else can).
#[component]
pub fn ExpenseSearchField(
    query: Signal<ExpenseQuery>,
    autofocus: bool,
    on_change: EventHandler<()>,
    #[props(default)] closes: bool,
) -> Element {
    let text = query.read().text.clone();
    let mut focused = use_signal(|| false);
    let mut field: Signal<Option<std::rc::Rc<MountedData>>> = use_signal(|| None);
    let mut clear = move |close: bool| {
        {
            let mut q = query.write();
            q.text.clear();
            if close {
                q.searching = false;
            }
        }
        on_change.call(());
    };
    rsx! {
        div { class: "flex items-center gap-1",
            label { class: "search-field input flex-1 min-w-0 h-10 pe-1",
                span { class: "text-base-content/60", aria_hidden: "true", SearchIcon { size: 18 } }
                input {
                    id: "expense-search",
                    class: "grow min-w-0 [&::-webkit-search-cancel-button]:hidden",
                    r#type: "search",
                    enterkeyhint: "search",
                    autocomplete: "off",
                    aria_label: tid!("expenses-search"),
                    value: "{text}",
                    onmounted: move |e| async move {
                        field.set(Some(e.data()));
                        if autofocus {
                            let _ = e.data().set_focus(true).await;
                        }
                    },
                    onfocus: move |_| focused.set(true),
                    onblur: move |_| focused.set(false),
                    oninput: move |e| {
                        query.write().text = e.value();
                        on_change.call(());
                    },
                }
                if closes || !text.is_empty() {
                    button {
                        id: "expense-search-clear",
                        r#type: "button",
                        class: "btn btn-ghost btn-circle btn-sm text-base-content/60",
                        aria_label: tid!("expenses-search-clear"),
                        onmousedown: move |e| e.prevent_default(),
                        onclick: move |_| clear(closes),
                        CloseIcon { size: 14 }
                    }
                }
            }
            if focused() {
                button {
                    id: "expense-search-cancel",
                    r#type: "button",
                    class: "btn btn-ghost btn-sm shrink-0 px-2 text-primary text-base font-normal",
                    // Both buttons keep the field focused through the press: a blur would take Cancel away
                    // and shift the clear button out from under the click.
                    onmousedown: move |e| e.prevent_default(),
                    onclick: move |_| {
                        clear(closes);
                        if let Some(field) = field() {
                            spawn(async move {
                                let _ = field.set_focus(false).await;
                            });
                        }
                    },
                    {tid!("cancel")}
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
