use dioxus::prelude::*;
use shared::{Currency, CURRENCIES};

use crate::common::{haptic, use_modal_focus, Haptic};
use crate::icons::{CheckIcon, CloseIcon, ICON_ACTION};
use crate::tid;

pub fn currency_name(currency: &Currency) -> String {
    let id = format!("currency-{}", currency.code);
    tid!(&id)
}

/// Codes starting with the query lead, so Enter on "us" picks USD rather than AUD. The English
/// name matches too, for someone who knows a currency only by it.
pub fn matching_currencies(
    query: &str,
    local_name: impl Fn(&Currency) -> String,
) -> Vec<(&'static Currency, String)> {
    let q = query.trim().to_lowercase();
    let (by_code, by_name): (Vec<_>, Vec<_>) = CURRENCIES
        .iter()
        .map(|c| (c, local_name(c)))
        .filter(|(c, name)| {
            c.code.to_lowercase().contains(&q)
                || name.to_lowercase().contains(&q)
                || c.name.to_lowercase().contains(&q)
        })
        .partition(|(c, _)| c.code.to_lowercase().starts_with(&q));
    by_code.into_iter().chain(by_name).collect()
}

/// A finger raises the keyboard on a scripted focus, which would cover half the list.
fn focus_search_on_open() -> bool {
    #[cfg(target_arch = "wasm32")]
    return !crate::common::web_dom::coarse_pointer();
    #[cfg(not(target_arch = "wasm32"))]
    false
}

/// A native `<select>` sizes itself to its widest option and cannot be searched on a phone, so
/// the trigger shows the code alone and the names live in a searchable bottom sheet, stacked on
/// the form it sits in.
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

    rsx! {
        div { class: "shrink-0",
            button {
                id: "{id}",
                r#type: "button",
                class: "{button_class}",
                aria_label: "{label}",
                aria_haspopup: "dialog",
                aria_expanded: open(),
                value: "{value}",
                onclick: move |_| {
                    haptic(Haptic::Light);
                    open.set(true);
                },
                "{value}"
                span { aria_hidden: "true", class: "text-xs opacity-60", "▾" }
            }
            if open() {
                CurrencySheet {
                    id: id.clone(),
                    value: value.clone(),
                    title: label.clone(),
                    on_pick: move |code: String| {
                        open.set(false);
                        onchange.call(code);
                    },
                    on_close: move |_| open.set(false),
                }
            }
        }
    }
}

/// One sheet id for every picker: only one is ever open.
#[component]
fn CurrencySheet(
    id: String,
    value: String,
    title: String,
    on_pick: EventHandler<String>,
    on_close: EventHandler<()>,
) -> Element {
    use_modal_focus("currency-sheet");
    let mut query = use_signal(String::new);
    let matches = matching_currencies(&query(), currency_name);
    let first = matches.first().map(|(c, _)| c.code);
    let pick = move |code: &str| {
        haptic(Haptic::Selection);
        on_pick.call(code.to_string());
    };

    rsx! {
        div {
            id: "currency-sheet",
            "data-sheet-detents": "true",
            class: "modal modal-open modal-bottom sm:modal-middle",
            role: "dialog",
            aria_modal: "true",
            aria_labelledby: "currency-sheet-title",
            tabindex: "-1",
            onclick: move |e| e.prevent_default(),
            div { class: "modal-box max-w-md p-0 flex flex-col",
                div { class: "flex items-center gap-2 px-5 pt-4 pb-2",
                    h2 { id: "currency-sheet-title", class: "flex-1 min-w-0 truncate font-bold text-lg font-display", "{title}" }
                    button {
                        r#type: "button",
                        class: "btn btn-circle size-11 min-h-11 border-0 bg-base-200 text-base-content/80",
                        aria_label: tid!("close"),
                        onclick: move |_| on_close.call(()),
                        CloseIcon {}
                    }
                }
                div { class: "px-5 pb-2",
                    input {
                        id: "{id}-search",
                        r#type: "search",
                        class: "input w-full text-base",
                        autocomplete: "off",
                        placeholder: tid!("currency-search"),
                        aria_label: tid!("currency-search"),
                        value: "{query}",
                        onmounted: move |e| async move {
                            if focus_search_on_open() {
                                let _ = e.data().set_focus(true).await;
                            }
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
                }
                ul { class: "flex-1 overflow-y-auto px-3 pb-3", role: "listbox", aria_label: "{title}",
                    for (c, name) in matches {
                        li { key: "{c.code}",
                            button {
                                id: "{id}-{c.code}",
                                r#type: "button",
                                role: "option",
                                aria_selected: c.code == value,
                                class: "w-full flex gap-3 items-center text-left px-3 min-h-11 rounded-field row-press",
                                onclick: move |_| pick(c.code),
                                span { class: "w-11 shrink-0 font-semibold tabular-nums", "{c.code}" }
                                span { class: "flex-1 min-w-0 truncate text-base-content/70", "{name}" }
                                if c.code == value {
                                    span { class: "shrink-0 text-primary", aria_hidden: "true",
                                        CheckIcon { size: ICON_ACTION }
                                    }
                                }
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

    fn codes(query: &str) -> Vec<&'static str> {
        matching_currencies(query, |c| c.name.to_string()).iter().map(|(c, _)| c.code).collect()
    }

    #[test]
    fn the_local_and_the_english_name_both_match() {
        let french = |c: &Currency| if c.code == "HUF" { "Forint hongrois".to_string() } else { c.name.to_string() };
        let found = |q: &str| matching_currencies(q, french).iter().map(|(c, _)| c.code).collect::<Vec<_>>();
        assert_eq!(found("hongrois"), ["HUF"]);
        assert_eq!(found("hungarian"), ["HUF"]);
    }

    #[test]
    fn every_currency_has_a_message() {
        let ids = crate::i18n::locale_ids("en");
        let missing: Vec<_> =
            CURRENCIES.iter().map(|c| format!("currency-{}", c.code)).filter(|id| !ids.contains(id)).collect();
        assert!(missing.is_empty(), "en.ftl lacks {missing:?}");
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
