use dioxus::prelude::*;
use std::rc::Rc;

use super::super::helpers::expense_form_helpers::{amount_field_text, is_expression};

/// The draft of the amount field that currently has focus — what `AmountOperatorBar` writes into.
/// `None` when no amount field is focused, which is also what hides the bar.
pub type FocusedAmount = Signal<Option<Signal<Option<String>>>>;

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
/// On a coarse pointer an expression is covered by an overlay showing the running result, so the
/// field reads `83.33` while `250/3` sits in the operator bar's bubble. The input's own value is
/// still the expression — caret, backspace and paste are the browser's, untouched.
#[derive(Props, Clone, PartialEq)]
pub struct AmountInputProps {
    pub value: f64,
    pub oninput: EventHandler<String>,
    /// Goes on the wrapper, not the input: the wrapper is the box the overlay is positioned
    /// against. Everything in `attributes` (`id`, `aria_label`…) stays on the input.
    #[props(default)]
    pub class: String,
    #[props(extends = GlobalAttributes, extends = input)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn AmountInput(props: AmountInputProps) -> Element {
    let mut draft: Signal<Option<String>> = use_signal(|| None);
    let mut focused = use_signal(|| false);
    let mut mounted: Signal<Option<Rc<MountedData>>> = use_signal(|| None);
    // The participants tests mount the fieldset with no form above it, so there may be no bar.
    let bar = try_use_context::<FocusedAmount>();

    let shown = amount_field_text(draft().as_deref(), props.value);
    let calc = draft().as_deref().is_some_and(is_expression);

    // The bar appears below the scroll body once a field takes focus, shrinking it from the bottom
    // after the WebView's own scroll-into-view already ran — without this a row tapped near the
    // bottom ends up behind the bar.
    use_effect(move || {
        if !focused() {
            return;
        }
        let Some(node) = mounted.peek().clone() else { return };
        spawn(async move {
            let _ = node
                .scroll_to_with_options(ScrollToOptions {
                    behavior: ScrollBehavior::Instant,
                    vertical: ScrollLogicalPosition::Nearest,
                    horizontal: ScrollLogicalPosition::Nearest,
                })
                .await;
        });
    });

    rsx! {
        span { class: "relative flex items-center {props.class}",
            input {
                r#type: "text",
                inputmode: "decimal",
                value: shown.clone(),
                class: "w-full",
                // Chromium's UA sheet sets `text-align: start` on an input, so a `text-right` on
                // the wrapper would not otherwise reach it.
                text_align: "inherit",
                class: if calc { "pointer-coarse:text-transparent pointer-coarse:caret-transparent" },
                onmounted: move |e| mounted.set(Some(e.data())),
                onfocus: move |_| {
                    // Materialised so the bar always has a left operand to append to.
                    if draft.peek().is_none() {
                        draft.set(Some(shown.clone()));
                    }
                    focused.set(true);
                    if let Some(mut bar) = bar {
                        bar.set(Some(draft));
                    }
                },
                oninput: move |e| {
                    let raw = e.value();
                    draft.set(Some(raw.clone()));
                    props.oninput.call(raw);
                },
                onblur: move |_| {
                    if draft().as_deref().is_some_and(is_expression) {
                        draft.set(None);
                    }
                    focused.set(false);
                    // Moving between two amount fields blurs before it focuses, so this nets to
                    // the new field in the same render.
                    if let Some(mut bar) = bar {
                        if bar.peek().is_some_and(|d| d == draft) {
                            bar.set(None);
                        }
                    }
                },
                ..props.attributes,
            }
            if calc {
                // `p-[inherit]` is what makes one overlay fit both call sites: no padding inside
                // the total's `label.input`, daisyUI's `padding-inline` on a participant row.
                span {
                    class: "absolute inset-0 p-[inherit] hidden pointer-coarse:flex items-center pointer-events-none",
                    aria_hidden: "true",
                    span { class: "block w-full", "{props.value}" }
                }
            }
        }
    }
}
