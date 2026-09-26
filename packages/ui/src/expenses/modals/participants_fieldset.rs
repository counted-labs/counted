use dioxus::prelude::*;
use crate::tid;
use shared::sums_to_total;

use super::super::helpers::expense_form_helpers::{parse_amount, redistribute, UserEntry};
use super::amount_input::AmountInput;
use crate::common::{initials, user_color_class, Avatar, SizeClass};

/// Same filter as `active_amounts` in `expense_modal_helpers`, so what is displayed is exactly
/// what submitting would validate.
fn checked_amounts(entries: &[UserEntry]) -> Vec<f64> {
    entries.iter().filter(|e| e.checked).map(|e| e.amount).collect()
}

/// One participants block — payers or debtors. Both were copy-pasted in `ExpenseForm` before,
/// which is why every handler existed twice.
#[derive(Props, Clone, PartialEq)]
pub struct ParticipantsFieldsetProps {
    pub id: &'static str,
    pub legend: String,
    pub currency: String,
    pub total: Signal<f64>,
    pub entries: Signal<Vec<UserEntry>>,
    pub share_mode: Signal<bool>,
}

#[component]
pub fn ParticipantsFieldset(props: ParticipantsFieldsetProps) -> Element {
    let total = props.total;
    let mut entries = props.entries;
    let mut share_mode = props.share_mode;

    rsx! {
        // No border — the collapse in `ExpenseForm` draws the box and repeats the legend in its
        // title. Still a fieldset/legend pair so the grouping stays in the accessibility tree.
        fieldset { id: props.id, class: "min-w-0",
            legend { class: "sr-only", "{props.legend}" }
            div { class: "flex items-center justify-between mb-1",
                label { class: "flex items-center gap-3 min-h-11 cursor-pointer",
                    input {
                        r#type: "checkbox",
                        class: "checkbox checkbox-sm",
                        checked: entries().iter().all(|e| e.checked),
                        oninput: move |_| {
                            let t = total();
                            let sm = share_mode();
                            let new_checked = !entries().iter().all(|e| e.checked);
                            let mut e = entries.write();
                            for entry in e.iter_mut() {
                                entry.checked = new_checked;
                                if !new_checked {
                                    entry.amount = 0.0;
                                    entry.shares = 0.0;
                                } else if sm && entry.shares == 0.0 {
                                    entry.shares = 1.0;
                                }
                            }
                            if new_checked {
                                redistribute(t, &mut e, sm);
                            }
                        },
                    }
                    span { class: "text-sm", {tid!("participants-select-all")} }
                }
                // Deliberately unlike the type selector at the top of the sheet: outlined rather
                // than filled, half the width, smaller type. Two segmented controls drawn the same
                // way read as one repeated control, and neither says what it governs.
                div { class: "join",
                    for (shares , key) in [(false, "split-amounts"), (true, "participants-by-shares")] {
                        button {
                            r#type: "button",
                            class: if share_mode() == shares { "join-item btn btn-xs btn-neutral" } else { "join-item btn btn-xs btn-outline" },
                            aria_pressed: "{share_mode() == shares}",
                            onclick: move |_| {
                                if share_mode() == shares {
                                    return;
                                }
                                let t = total();
                                share_mode.set(shares);
                                let mut e = entries.write();
                                if shares {
                                    for entry in e.iter_mut() {
                                        if entry.checked && entry.shares == 0.0 {
                                            entry.shares = 1.0;
                                        }
                                    }
                                }
                                redistribute(t, &mut e, shares);
                            },
                            {tid!(key)}
                        }
                    }
                }
            }
            div { class: "flex flex-col",
                for i in 0..entries().len() {
                    {
                        let name = entries().get(i).map(|e| e.display_name.clone()).unwrap_or_default();
                        let user_id = entries().get(i).map(|e| e.user.id).unwrap_or(0);
                        let checked = entries().get(i).map(|e| e.checked).unwrap_or(false);
                        let amount_val = entries().get(i).map(|e| e.amount).unwrap_or(0.0);
                        let shares_val = entries().get(i).map(|e| e.shares).unwrap_or(0.0);
                        let sm = share_mode();
                        rsx! {
                            div { class: "flex items-center gap-3 min-h-11",
                                // The label wraps the checkbox, the avatar and the name — the whole
                                // left of the row is one target instead of a 20px box with inert
                                // text beside it. The amount field stays outside it: inside, a tap
                                // meant for the field would toggle the row.
                                label { class: "flex items-center gap-3 flex-1 min-w-0 min-h-11 cursor-pointer",
                                    input {
                                        r#type: "checkbox",
                                        class: "checkbox checkbox-sm",
                                        aria_label: "{name}",
                                        checked,
                                        oninput: move |_| {
                                            let t = total();
                                            let sm = share_mode();
                                            let mut e = entries.write();
                                            if let Some(entry) = e.get_mut(i) {
                                                entry.checked = !entry.checked;
                                                if sm {
                                                    if entry.checked && entry.shares == 0.0 {
                                                        entry.shares = 1.0;
                                                    } else if !entry.checked {
                                                        entry.shares = 0.0;
                                                    }
                                                }
                                            }
                                            redistribute(t, &mut e, sm);
                                        },
                                    }
                                    Avatar {
                                        initials: initials(&name),
                                        color_class: user_color_class(user_id).to_string(),
                                        size: SizeClass::W6,
                                    }
                                    span { class: "flex-1 text-sm truncate", "{name}" }
                                }
                                if sm {
                                    span { class: "text-xs text-base-content/70 w-16 text-right tabular-nums", "{amount_val:.2}" }
                                    AmountInput {
                                        class: "input input-sm w-16 text-right tabular-nums",
                                        aria_label: tid!("participants-shares-for", name: name.clone()),
                                        value: shares_val,
                                        oninput: move |raw: String| {
                                            let v = parse_amount(&raw).unwrap_or(0.0);
                                            let t = total();
                                            let mut e = entries.write();
                                            if let Some(entry) = e.get_mut(i) {
                                                entry.shares = v;
                                                entry.checked = v > 0.0;
                                            }
                                            redistribute(t, &mut e, true);
                                        },
                                    }
                                } else {
                                    AmountInput {
                                        class: "input input-sm w-28 text-right tabular-nums",
                                        aria_label: tid!("participants-amount-for", name: name.clone()),
                                        value: amount_val,
                                        oninput: move |raw: String| {
                                            let Some(v) = parse_amount(&raw) else { return };
                                            let mut e = entries.write();
                                            if let Some(entry) = e.get_mut(i) {
                                                entry.amount = v;
                                                entry.checked = v > 0.0;
                                            }
                                        },
                                    }
                                }
                            }
                        }
                    }
                }
            }
            // Every path but a hand-typed amount goes through `redistribute`, which is exact — so
            // this only lights up when a row is edited by hand.
            {
                let amounts = checked_amounts(&entries());
                let sum: f64 = amounts.iter().sum();
                let balanced = sums_to_total(total(), amounts);
                let diff = sum - total();
                rsx! {
                    div {
                        class: if balanced { "flex justify-between text-xs mt-2 tabular-nums text-base-content/70" } else { "flex justify-between text-xs mt-2 tabular-nums text-error font-medium" },
                        span { "{sum:.2} / {total():.2} {props.currency}" }
                        if !balanced {
                            if diff < 0.0 {
                                span { {tid!("participants-remaining", amount: format!("{:.2}", -diff))} }
                            } else {
                                span { {tid!("participants-over-by", amount: format!("{diff:.2}"))} }
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
    use super::super::amount_input::FocusedAmount;
    use super::super::amount_operator_bar::AmountOperatorBar;
    use dioxus::dioxus_core::ElementId;
    use crate::common::test_dom::{click, focus, form_input, listener_ids, texts, value_writes};
    use shared::{EncryptedPair, User};
    use std::cell::RefCell;
    use std::rc::Rc;

    /// (checked, amount, shares) per row, mirrored out of the signal on every render.
    type Rows = Vec<(bool, f64, f64)>;
    type Spy = Rc<RefCell<Rows>>;

    #[derive(Props, Clone, PartialEq)]
    struct HarnessProps {
        spy: Spy,
        share_mode: bool,
    }

    /// Owns the signals the fieldset mutates and copies them into `spy` on every render.
    /// Reading the signal here subscribes the harness, so the mirror tracks every write.
    #[component]
    fn Harness(props: HarnessProps) -> Element {
        // The component translates its labels, and `tid!` panics with no provider above it.
        crate::i18n::use_test_i18n();
        let entries = use_signal(|| {
            (1..=2)
                .map(|id| UserEntry {
                    user: User { id, payload: EncryptedPair::default(), ..Default::default() },
                    display_name: format!("U{id}"),
                    checked: false,
                    amount: 0.0,
                    shares: 0.0,
                })
                .collect::<Vec<_>>()
        });
        *props.spy.borrow_mut() =
            entries().iter().map(|e| (e.checked, e.amount, e.shares)).collect();

        rsx! {
            ParticipantsFieldset {
                id: "test",
                legend: "who-paid".to_string(),
                currency: "EUR".to_string(),
                total: use_signal(|| 30.0),
                entries,
                share_mode: use_signal({
                    let sm = props.share_mode;
                    move || sm
                }),
            }
        }
    }

    /// Single template, so this is plain document order. The mode control is two buttons, so it is
    /// a `click` listener and does not shift these.
    const SELECT_ALL: usize = 0;
    const CHECKBOX_0: usize = 1;
    const FIELD_0: usize = 2;
    const CHECKBOX_1: usize = 3;
    const FIELD_1: usize = 4;

    /// The `click` listeners: the two mode segments.
    const AMOUNTS_MODE: usize = 0;
    const SHARES_MODE: usize = 1;

    fn harness(share_mode: bool) -> (VirtualDom, Spy, Vec<ElementId>) {
        let (dom, spy, ids, _) = harness_with_clicks(share_mode);
        (dom, spy, ids)
    }

    /// Both listener sets have to come from the same rebuild, and a rebuild can only be taken once.
    fn harness_with_clicks(share_mode: bool) -> (VirtualDom, Spy, Vec<ElementId>, Vec<ElementId>) {
        let spy: Spy = Rc::new(RefCell::new(Vec::new()));
        let mut dom =
            VirtualDom::new_with_props(Harness, HarnessProps { spy: spy.clone(), share_mode });
        let m = dom.rebuild_to_vec();
        let ids = listener_ids(&m, "input");
        let clicks = listener_ids(&m, "click");
        assert_eq!(ids.len(), 5, "template changed — re-index the constants above");
        assert_eq!(clicks.len(), 2, "the two mode segments");
        (dom, spy, ids, clicks)
    }

    #[test]
    fn typing_an_amount_checks_the_row_without_waiting_for_a_blur() {
        let (mut dom, spy, ids) = harness(false);

        dom.runtime().handle_event("input", form_input("5"), ids[FIELD_0]);
        dom.render_immediate_to_vec();
        assert_eq!(spy.borrow()[0], (true, 5.0, 0.0), "a positive amount checks the row");

        dom.runtime().handle_event("input", form_input("0"), ids[FIELD_0]);
        dom.render_immediate_to_vec();
        assert_eq!(spy.borrow()[0], (false, 0.0, 0.0), "back to zero unchecks it");
    }

    #[test]
    fn an_amount_accepts_the_decimal_comma() {
        // type="number" used to hand us an empty string here, so FR mobile keyboards could not
        // enter decimals at all.
        let (mut dom, spy, ids) = harness(false);

        dom.runtime().handle_event("input", form_input("12,50"), ids[FIELD_0]);
        dom.render_immediate_to_vec();

        assert_eq!(spy.borrow()[0], (true, 12.50, 0.0));
    }

    /// `value` is volatile in Dioxus — rewritten on every render — and an `f64` formats "12." as
    /// "12", which erased the decimal key under a mobile keyboard.
    #[test]
    fn a_trailing_separator_survives_the_render() {
        for share_mode in [false, true] {
            let (mut dom, _spy, ids) = harness(share_mode);
            for keystroke in ["2.", "2,", "2,0", "2.50"] {
                dom.runtime().handle_event("input", form_input(keystroke), ids[FIELD_0]);
                let m = dom.render_immediate_to_vec();
                for written in value_writes(&m, ids[FIELD_0]) {
                    assert_eq!(written, keystroke, "share_mode={share_mode}: rewrote the field");
                }
            }
        }
    }

    /// A value the split moved from outside must still replace what was typed.
    #[test]
    fn a_redistributed_amount_replaces_the_typed_draft() {
        let (mut dom, spy, ids) = harness(false);

        dom.runtime().handle_event("input", form_input("7."), ids[FIELD_0]);
        dom.render_immediate_to_vec();
        dom.runtime().handle_event("input", form_input("on"), ids[CHECKBOX_1]);
        let m = dom.render_immediate_to_vec();

        assert_eq!(spy.borrow()[0], (true, 15.0, 0.0));
        assert_eq!(value_writes(&m, ids[FIELD_0]).last().map(String::as_str), Some("15"));
    }

    #[test]
    fn an_unparseable_amount_leaves_the_row_untouched() {
        let (mut dom, spy, ids) = harness(false);

        dom.runtime().handle_event("input", form_input("8"), ids[FIELD_0]);
        dom.render_immediate_to_vec();
        // Bailing out on an empty value is what stops Dioxus from patching `value` back over
        // the keystroke in progress.
        dom.runtime().handle_event("input", form_input(""), ids[FIELD_0]);
        dom.render_immediate_to_vec();

        assert_eq!(spy.borrow()[0], (true, 8.0, 0.0));
    }

    #[test]
    fn typing_shares_redistributes_the_total_immediately() {
        let (mut dom, spy, ids) = harness(true);

        dom.runtime().handle_event("input", form_input("2"), ids[FIELD_0]);
        dom.render_immediate_to_vec();
        dom.runtime().handle_event("input", form_input("1"), ids[FIELD_1]);
        dom.render_immediate_to_vec();

        // 30 split 2:1 — recomputed on the keystroke, not on the blur.
        assert_eq!(spy.borrow()[0], (true, 20.0, 2.0));
        assert_eq!(spy.borrow()[1], (true, 10.0, 1.0));
    }

    #[test]
    fn toggling_a_row_redistributes_the_total() {
        let (mut dom, spy, ids) = harness(false);

        dom.runtime().handle_event("input", form_input("on"), ids[CHECKBOX_0]);
        dom.render_immediate_to_vec();

        assert_eq!(spy.borrow()[0], (true, 30.0, 0.0), "the sole checked row takes the total");
        assert_eq!(spy.borrow()[1], (false, 0.0, 0.0));
    }

    #[test]
    fn switching_to_share_mode_gives_checked_rows_one_share_each() {
        let (mut dom, spy, ids, clicks) = harness_with_clicks(false);

        dom.runtime().handle_event("input", form_input("on"), ids[CHECKBOX_0]);
        dom.render_immediate_to_vec();
        dom.runtime().handle_event("click", click(), clicks[SHARES_MODE]);
        dom.render_immediate_to_vec();

        assert_eq!(spy.borrow()[0], (true, 30.0, 1.0));
    }

    /// Tapping the segment already in force must not re-run the split — hand-typed amounts would
    /// be wiped by a tap that changes nothing.
    #[test]
    fn tapping_the_current_mode_changes_nothing() {
        let (mut dom, spy, ids, clicks) = harness_with_clicks(false);

        dom.runtime().handle_event("input", form_input("7"), ids[FIELD_0]);
        dom.render_immediate_to_vec();
        let before = spy.borrow().clone();

        dom.runtime().handle_event("click", click(), clicks[AMOUNTS_MODE]);
        dom.render_immediate_to_vec();

        assert_eq!(*spy.borrow(), before, "the amounts segment was already on");
    }

    /// Concatenated so an assertion doesn't depend on how the text splits across nodes.
    fn rendered(m: &dioxus::dioxus_core::Mutations) -> String {
        texts(m).join(" ")
    }

    #[test]
    fn the_counter_shows_what_is_left_to_allocate() {
        let (mut dom, _spy, ids) = harness(false);

        let out = rendered(&dom.render_immediate_to_vec());
        let _ = out; // first render is covered by the balanced case below

        dom.runtime().handle_event("input", form_input("60"), ids[FIELD_0]);
        let out = rendered(&dom.render_immediate_to_vec());
        assert!(out.contains("60.00 / 30.00"), "counter missing from: {out}");
        // `use_test_i18n` pins the harness to English, so asserting on English copy is stable.
        // Match the word, not the amount: Fluent wraps interpolated values in bidi isolates.
        assert!(out.contains("over"), "over-allocation not flagged: {out}");
    }

    #[test]
    fn the_counter_flags_a_short_side() {
        let (mut dom, _spy, ids) = harness(false);

        dom.runtime().handle_event("input", form_input("10"), ids[FIELD_0]);
        let out = rendered(&dom.render_immediate_to_vec());
        assert!(out.contains("10.00 / 30.00"), "counter missing from: {out}");
        assert!(out.contains("left"), "shortfall not flagged: {out}");
        assert!(out.contains("20.00"), "the shortfall amount is not shown: {out}");
    }

    #[test]
    fn the_counter_is_quiet_when_the_split_balances() {
        let (mut dom, _spy, ids) = harness(false);

        // Checking a row runs `redistribute`, which allocates the whole total.
        dom.runtime().handle_event("input", form_input("on"), ids[CHECKBOX_0]);
        let out = rendered(&dom.render_immediate_to_vec());
        assert!(out.contains("30.00 / 30.00"), "counter missing from: {out}");
        assert!(!out.contains("left") && !out.contains("over"), "balanced side must not flag: {out}");
    }

    #[test]
    fn the_counter_is_quiet_in_share_mode() {
        let (mut dom, _spy, ids) = harness(true);

        dom.runtime().handle_event("input", form_input("2"), ids[FIELD_0]);
        dom.render_immediate_to_vec();
        dom.runtime().handle_event("input", form_input("1"), ids[FIELD_1]);
        let out = rendered(&dom.render_immediate_to_vec());
        assert!(!out.contains("left") && !out.contains("over"), "shares split exactly: {out}");
    }

    #[test]
    fn select_all_rebalances_the_counter() {
        let (mut dom, _spy, ids) = harness(false);

        // Leave the side unbalanced by hand first…
        dom.runtime().handle_event("input", form_input("7"), ids[FIELD_0]);
        let out = rendered(&dom.render_immediate_to_vec());
        assert!(out.contains("left") && out.contains("23.00"), "expected a shortfall first: {out}");

        // …then let "Tout sélectionner" redistribute it.
        dom.runtime().handle_event("input", form_input("on"), ids[SELECT_ALL]);
        let out = rendered(&dom.render_immediate_to_vec());
        assert!(!out.contains("left") && !out.contains("over"), "select-all must rebalance: {out}");
    }

    /// The row follows the expression on the keystroke; blur only rewrites the text.
    #[test]
    fn an_expression_in_a_row_applies_on_the_keystroke_and_blur_only_settles_the_text() {
        let (mut dom, spy, ids) = harness(false);

        dom.runtime().handle_event("input", form_input("10+2.5"), ids[FIELD_0]);
        let m = dom.render_immediate_to_vec();
        assert_eq!(spy.borrow()[0], (true, 12.5, 0.0), "applied before any blur");
        for written in value_writes(&m, ids[FIELD_0]) {
            assert_eq!(written, "10+2.5");
        }

        let before = spy.borrow().clone();
        dom.runtime().handle_event("blur", focus(), ids[FIELD_0]);
        let m = dom.render_immediate_to_vec();
        assert_eq!(*spy.borrow(), before, "blur may settle the text, never the numbers");
        assert_eq!(value_writes(&m, ids[FIELD_0]), vec!["12.5".to_string()]);
    }

    /// `ExpenseForm` provides the context and renders the bar below the footer; this mirrors that
    /// arrangement so a row's field can be driven by the operator keys.
    #[component]
    fn BarHarness(props: HarnessProps) -> Element {
        // The bar is a sibling of `Harness`, so it needs its own provider above it.
        crate::i18n::use_test_i18n();
        use_context_provider::<FocusedAmount>(|| Signal::new(None));
        rsx! {
            Harness { spy: props.spy, share_mode: props.share_mode }
            AmountOperatorBar {}
        }
    }

    /// The bar's five keys, in template order.
    const DIVIDE: usize = 3;
    const EQUALS: usize = 4;

    fn bar_harness() -> (VirtualDom, Spy, Vec<ElementId>) {
        let spy: Spy = Rc::new(RefCell::new(Vec::new()));
        let mut dom = VirtualDom::new_with_props(
            BarHarness,
            HarnessProps { spy: spy.clone(), share_mode: false },
        );
        let m = dom.rebuild_to_vec();
        let ids = listener_ids(&m, "input");
        assert_eq!(ids.len(), 5, "template changed — re-index the constants above");
        assert_eq!(
            listener_ids(&m, "click").len(),
            2,
            "the two mode segments, and no bar until a field is focused"
        );
        (dom, spy, ids)
    }

    /// The keys the bar registered on its last render.
    fn keys(m: &dioxus::dioxus_core::Mutations) -> Vec<ElementId> {
        let ids = listener_ids(m, "click");
        assert_eq!(ids.len(), 5, "the bar carries +, −, ×, ÷ and =");
        ids
    }

    #[test]
    fn the_operator_bar_writes_into_the_focused_row() {
        let (mut dom, spy, ids) = bar_harness();

        dom.runtime().handle_event("input", form_input("20"), ids[FIELD_0]);
        dom.render_immediate_to_vec();
        dom.runtime().handle_event("focus", focus(), ids[FIELD_0]);
        let keys = keys(&dom.render_immediate_to_vec());

        dom.runtime().handle_event("click", click(), keys[DIVIDE]);
        let m = dom.render_immediate_to_vec();
        assert_eq!(value_writes(&m, ids[FIELD_0]).last().map(String::as_str), Some("20/"));

        // The keystroke the user would now type, with the field's whole value as the browser sends it.
        dom.runtime().handle_event("input", form_input("20/2"), ids[FIELD_0]);
        let m = dom.render_immediate_to_vec();
        assert_eq!(spy.borrow()[0], (true, 10.0, 0.0), "the row follows the expression");
        assert!(rendered(&m).contains("20/2"), "the expression shows in the bubble: {:?}", texts(&m));

        // "=" is what blur does, without dropping focus.
        dom.runtime().handle_event("click", click(), keys[EQUALS]);
        let m = dom.render_immediate_to_vec();
        assert_eq!(value_writes(&m, ids[FIELD_0]).last().map(String::as_str), Some("10"));
        assert_eq!(spy.borrow()[0], (true, 10.0, 0.0), "settling the text moves no number");
    }

    #[test]
    fn the_bar_follows_focus_from_one_row_to_the_next() {
        let (mut dom, _spy, ids) = bar_harness();

        dom.runtime().handle_event("input", form_input("20"), ids[FIELD_0]);
        dom.render_immediate_to_vec();
        dom.runtime().handle_event("focus", focus(), ids[FIELD_0]);
        let keys = keys(&dom.render_immediate_to_vec());

        // Moving to the other row blurs before it focuses — the bar stays on screen (so its keys
        // keep their ids) and must end up pointed at the new field.
        dom.runtime().handle_event("input", form_input("10"), ids[FIELD_1]);
        dom.render_immediate_to_vec();
        dom.runtime().handle_event("blur", focus(), ids[FIELD_0]);
        dom.runtime().handle_event("focus", focus(), ids[FIELD_1]);
        let m = dom.render_immediate_to_vec();
        assert!(listener_ids(&m, "click").is_empty(), "the bar must not flicker between fields");

        dom.runtime().handle_event("click", click(), keys[DIVIDE]);
        let m = dom.render_immediate_to_vec();
        assert_eq!(value_writes(&m, ids[FIELD_1]).last().map(String::as_str), Some("10/"));
        assert!(value_writes(&m, ids[FIELD_0]).is_empty(), "the row left behind is untouched");
    }

    /// Focusing again has to build the keys from scratch — which is only true if the blur took the
    /// bar off screen.
    #[test]
    fn blurring_the_last_field_takes_the_bar_away() {
        let (mut dom, _spy, ids) = bar_harness();

        dom.runtime().handle_event("focus", focus(), ids[FIELD_0]);
        keys(&dom.render_immediate_to_vec());

        dom.runtime().handle_event("blur", focus(), ids[FIELD_0]);
        dom.render_immediate_to_vec();
        dom.runtime().handle_event("focus", focus(), ids[FIELD_0]);
        keys(&dom.render_immediate_to_vec());
    }
}
