use dioxus::prelude::*;

use crate::tid;

use super::super::helpers::expense_form_helpers::{is_expression, with_operator};
use super::amount_input::FocusedAmount;

/// The operator keys mobile numeric keyboards lack, as a bar that sits on the keyboard for
/// whichever amount field has focus — the total or any participant row.
///
/// Rendered as the last row of `.modal-box`, below the footer: on phones the box's
/// `padding-bottom: max(--sab, --kb)` is what puts it directly above the keyboard, so nothing here
/// positions itself. `onpointerdown` preventDefault keeps the field focused across the tap, which
/// is also what keeps the keyboard up and this bar on screen.
#[component]
pub fn AmountOperatorBar() -> Element {
    let bar = use_context::<FocusedAmount>();

    let Some(draft) = bar() else {
        return rsx! {};
    };
    let expression = draft().filter(|d| is_expression(d));

    let tap = move |op: char| {
        let Some(mut draft) = bar.peek().to_owned() else { return };
        let shown = draft.peek().clone().unwrap_or_default();
        if let Some(next) = with_operator(&shown, op) {
            draft.set(Some(next));
        }
    };

    rsx! {
        div {
            class: "hidden pointer-coarse:flex gap-2 px-6 py-2 border-t border-base-200 shrink-0 relative",
            if let Some(expr) = expression.clone() {
                span {
                    class: "absolute -top-3 left-1/2 -translate-x-1/2 badge badge-neutral tabular-nums pointer-events-none",
                    "{expr}"
                }
            }
            button {
                r#type: "button",
                class: "btn btn-sm btn-ghost flex-1 text-lg",
                aria_label: tid!("amount-op-add"),
                onpointerdown: move |e| e.prevent_default(),
                onclick: move |_| tap('+'),
                "+"
            }
            button {
                r#type: "button",
                class: "btn btn-sm btn-ghost flex-1 text-lg",
                aria_label: tid!("amount-op-subtract"),
                onpointerdown: move |e| e.prevent_default(),
                onclick: move |_| tap('-'),
                "−"
            }
            button {
                r#type: "button",
                class: "btn btn-sm btn-ghost flex-1 text-lg",
                aria_label: tid!("amount-op-multiply"),
                onpointerdown: move |e| e.prevent_default(),
                onclick: move |_| tap('*'),
                "×"
            }
            button {
                r#type: "button",
                class: "btn btn-sm btn-ghost flex-1 text-lg",
                aria_label: tid!("amount-op-divide"),
                onpointerdown: move |e| e.prevent_default(),
                onclick: move |_| tap('/'),
                "÷"
            }
            button {
                r#type: "button",
                class: "btn btn-sm btn-ghost flex-1 text-lg",
                aria_label: tid!("amount-op-equals"),
                disabled: expression.is_none(),
                onpointerdown: move |e| e.prevent_default(),
                // Exactly what blur does, without dropping focus: the draft goes, the field shows
                // the number the split already used.
                onclick: move |_| {
                    if let Some(mut draft) = bar.peek().to_owned() {
                        draft.set(None);
                    }
                },
                "="
            }
            // The one key here that must NOT preventDefault: letting the tap's default action run
            // is what blurs the field, which is what lowers the keyboard — and blurring also
            // clears `FocusedAmount`, so this bar goes with it. It exists because suppressing
            // iOS's own accessory view (packages/mobile/src/main.rs) took away the ✓ that was the
            // numeric keypad's only dismiss control; the keypad has no return key of its own.
            button {
                r#type: "button",
                class: "btn btn-sm btn-ghost flex-1",
                aria_label: tid!("amount-op-done"),
                svg {
                    xmlns: "http://www.w3.org/2000/svg",
                    "aria-hidden": "true",
                    class: "h-5 w-5 stroke-current",
                    fill: "none",
                    view_box: "0 0 24 24",
                    path {
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        stroke_width: "2",
                        d: "M19 9l-7 7-7-7",
                    }
                }
            }
        }
    }
}
