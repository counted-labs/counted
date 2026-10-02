use dioxus::prelude::*;
use std::rc::Rc;

use super::super::helpers::expense_form_helpers::{amount_field_text, is_expression, keeps_draft_on_blur};

/// The draft of the amount field that currently has focus — what `AmountOperatorBar` writes into.
/// `None` when no amount field is focused, which is also what hides the bar.
pub type FocusedAmount = Signal<Option<Signal<Option<String>>>>;

/// `focus_on_mount`'s retry budget: 1s total, comfortably past daisyUI's 0.3s modal transition,
/// and abandoned rather than looping forever if something else holds the focus.
const FOCUS_TRIES: usize = 20;
const FOCUS_RETRY_MS: u32 = 50;

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
/// An empty field is 0: the caller is handed "0" while the field stays empty until blur. Focus
/// selects the whole text, so the first keystroke replaces the 0 instead of making "05".
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
    /// Takes the focus on mount. A prop rather than an `onmounted` the caller passes, because this
    /// component already owns that handler for its own scroll-into-view. **Not** named `autofocus`:
    /// that is a real `input` attribute, and `extends = input` below would swallow it.
    #[props(default)]
    pub focus_on_mount: bool,
    /// Sizes the input to its text in `ch`: `field-sizing: content` is ignored by older iOS WebKit.
    #[props(default)]
    pub fit: bool,
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
    let result = props.value.to_string();
    let fit_chars = if calc { shown.chars().count().max(result.chars().count()) } else { shown.chars().count().max(1) };
    let fit_width = format!("width: calc({fit_chars}ch + 0.25ch); max-width: 100%");

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
                class: "w-full outline-none",
                // Chromium's UA sheet sets `text-align: start` on an input, so a `text-right` on
                // the wrapper would not otherwise reach it.
                text_align: "inherit",
                style: if props.fit { "{fit_width}" },
                class: if calc { "pointer-coarse:text-transparent pointer-coarse:caret-transparent" },
                onmounted: move |e| {
                    let node = e.data();
                    mounted.set(Some(node.clone()));
                    if props.focus_on_mount {
                        // daisyUI animates the modal's `visibility` (0.3s, `allow-discrete`), and
                        // `HTMLElement.focus()` on a still-hidden element is *silently* ignored —
                        // `set_focus` reports `Ok(())` and nothing moves, which is exactly how this
                        // read as "autofocus does not work". Retry until `onfocus` confirms it
                        // landed, rather than hard-coding daisyUI's duration.
                        spawn(async move {
                            for _ in 0..FOCUS_TRIES {
                                // The user (or a test) got to another field first: retrying
                                // would yank them back mid-typing.
                                if crate::common::a_field_has_focus().await {
                                    return;
                                }
                                let _ = node.set_focus(true).await;
                                if *focused.peek() {
                                    return;
                                }
                                crate::common::sleep(FOCUS_RETRY_MS).await;
                            }
                        });
                    }
                },
                onfocus: move |_| {
                    // Materialised so the bar always has a left operand to append to.
                    if draft.peek().is_none() {
                        draft.set(Some(shown.clone()));
                    }
                    focused.set(true);
                    if let Some(mut bar) = bar {
                        bar.set(Some(draft));
                    }
                    spawn(crate::common::select_focused_input());
                },
                oninput: move |e| {
                    let raw = e.value();
                    draft.set(Some(raw.clone()));
                    props.oninput.call(if raw.trim().is_empty() { "0".to_string() } else { raw });
                },
                onblur: move |_| {
                    if draft().as_deref().is_some_and(|d| !keeps_draft_on_blur(d)) {
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
                    span { class: "block w-full", "{result}" }
                }
            }
        }
    }
}
