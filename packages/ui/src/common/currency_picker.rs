use dioxus::prelude::*;
use shared::{Currency, CURRENCIES};

use crate::tid;

/// Codes starting with the query lead, so Enter on "us" picks USD rather than AUD.
pub fn matching_currencies(query: &str) -> Vec<&'static Currency> {
    let q = query.trim().to_lowercase();
    let (by_code, by_name): (Vec<_>, Vec<_>) = CURRENCIES
        .iter()
        .filter(|c| c.code.to_lowercase().contains(&q) || c.name.to_lowercase().contains(&q))
        .partition(|c| c.code.to_lowercase().starts_with(&q));
    by_code.into_iter().chain(by_name).collect()
}

/// A native `<select>` sizes itself to its widest option and cannot be searched on a phone, so
/// the trigger shows the code alone and the names live in the searchable list.
///
/// The list hangs from the right edge of the caller's nearest `relative` ancestor, not from the
/// trigger: beside a short amount the trigger sits at the far left and the list would leave the modal.
///
/// The clicks inside are `prevent_default`ed: the expense form wraps this in the amount's
/// `<label>`, whose activation would otherwise yank focus back to the amount.
#[component]
pub fn CurrencyPicker(
    id: String,
    value: String,
    label: String,
    button_class: String,
    onchange: EventHandler<String>,
) -> Element {
    let mut open = use_signal(|| false);
    let mut query = use_signal(String::new);
    let matches = matching_currencies(&query());
    let first = matches.first().map(|c| c.code);
    let mut pick = move |code: &str| {
        onchange.call(code.to_string());
        open.set(false);
        query.set(String::new());
    };

    rsx! {
        div {
            class: "shrink-0",
            onkeydown: move |e| {
                if e.key() == Key::Escape && open() {
                    e.prevent_default();
                    e.stop_propagation();
                    open.set(false);
                }
            },
            if open() {
                div {
                    class: "fixed inset-0 z-40",
                    onclick: move |e| {
                        e.prevent_default();
                        e.stop_propagation();
                        open.set(false);
                    },
                }
            }
            button {
                id: "{id}",
                r#type: "button",
                class: "{button_class}",
                aria_label: "{label}",
                aria_haspopup: "listbox",
                aria_expanded: open(),
                value: "{value}",
                onclick: move |_| open.toggle(),
                "{value}"
                span { aria_hidden: "true", class: "text-xs opacity-60", "▾" }
            }
            if open() {
                div {
                    class: "absolute right-0 top-full mt-1 z-50 w-64 bg-base-100 rounded-box shadow-lg border border-base-200 p-2 flex flex-col gap-1",
                    onclick: move |e| e.prevent_default(),
                    input {
                        id: "{id}-search",
                        r#type: "search",
                        class: "input w-full text-base",
                        autocomplete: "off",
                        placeholder: tid!("currency-search"),
                        aria_label: tid!("currency-search"),
                        value: "{query}",
                        onmounted: move |e| async move {
                            let _ = e.data().set_focus(true).await;
                        },
                        oninput: move |e| query.set(e.value()),
                        onkeydown: move |e| {
                            if e.key() == Key::Enter {
                                e.prevent_default();
                                if let Some(code) = first {
                                    pick(code);
                                }
                            }
                        },
                    }
                    ul { class: "max-h-60 overflow-y-auto", role: "listbox",
                        for c in matches {
                            li { key: "{c.code}",
                                button {
                                    id: "{id}-{c.code}",
                                    r#type: "button",
                                    role: "option",
                                    aria_selected: c.code == value,
                                    class: "w-full flex gap-2 items-baseline text-left px-2 py-1.5 rounded-field text-sm hover:bg-base-200",
                                    class: if c.code == value { "bg-base-200" },
                                    onclick: move |_| pick(c.code),
                                    span { class: "font-semibold tabular-nums", "{c.code}" }
                                    span { class: "truncate text-base-content/60", "{c.name}" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn codes(query: &str) -> Vec<&'static str> {
        matching_currencies(query).iter().map(|c| c.code).collect()
    }

    #[test]
    fn empty_query_lists_every_currency_eur_first() {
        let all = codes("  ");
        assert_eq!(all.len(), CURRENCIES.len());
        assert_eq!(all[0], "EUR");
    }

    #[test]
    fn a_code_prefix_outranks_a_code_infix() {
        let found = codes("us");
        assert_eq!(found[0], "USD");
        assert!(found.contains(&"AUD"));
    }

    #[test]
    fn names_match_case_insensitively() {
        assert_eq!(codes("SWISS"), ["CHF"]);
    }

    #[test]
    fn nothing_matches_gibberish() {
        assert!(codes("zzzz").is_empty());
    }
}
