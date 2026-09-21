use dioxus::prelude::*;

use super::super::helpers::expense_form_helpers::{amount_field_text, is_expression};

/// A decimal text field bound to an `f64` that keeps showing the keystrokes as typed.
///
/// Dioxus treats `value` as volatile and rewrites it on every render, and an `f64` cannot hold a
/// trailing "12." or a "12,0" — binding the number directly erased the separator under a mobile
/// keyboard. The draft lives here so every amount field in the form gets the same fix; what is
/// displayed is decided by `amount_field_text`.
///
/// An expression (`250/3`) is evaluated by `parse_amount` on every keystroke, so `value` is already
/// 83.33 while the field still shows what was typed; blur only swaps the text for the number. A
/// plain draft is left alone on blur — a French `12,50` must not come back as `12.5`.
///
/// A caller that needs to write into the field from outside (the operator keys, which mobile
/// keyboards lack) passes its own `draft` signal.
#[derive(Props, Clone, PartialEq)]
pub struct AmountInputProps {
    pub value: f64,
    pub oninput: EventHandler<String>,
    #[props(default)]
    pub draft: Option<Signal<Option<String>>>,
    #[props(extends = GlobalAttributes, extends = input)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn AmountInput(props: AmountInputProps) -> Element {
    let own: Signal<Option<String>> = use_signal(|| None);
    let mut draft = props.draft.unwrap_or(own);
    let shown = amount_field_text(draft().as_deref(), props.value);

    rsx! {
        input {
            r#type: "text",
            inputmode: "decimal",
            value: shown,
            oninput: move |e| {
                let raw = e.value();
                draft.set(Some(raw.clone()));
                props.oninput.call(raw);
            },
            onblur: move |_| {
                if draft().as_deref().is_some_and(is_expression) {
                    draft.set(None);
                }
            },
            ..props.attributes,
        }
    }
}
