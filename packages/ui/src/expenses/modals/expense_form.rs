use dioxus::prelude::*;
use crate::tid;
use shared::{sums_to_total, ExpenseType};

use super::super::helpers::expense_form_helpers::{
    debtors_label, effective_rate, parse_amount, participants_summary, payers_label, project_total,
    redistribute, UserEntry,
};
use super::amount_input::{AmountInput, FocusedAmount};
use super::amount_operator_bar::AmountOperatorBar;
use super::participants_fieldset::ParticipantsFieldset;
use crate::categories::{category_label, infer_chart_category, parent_emoji, CHART_CATEGORIES as CATEGORIES};
use crate::common::{format_month_str, haptic, CurrencyPicker, Haptic};
use crate::icons::{CloseIcon, ICON_HEADER};
use crate::recurring::repeat_picker::RepeatRow;
use crate::recurring::view::Repeat;

/// The three type options, as (wire value, translation key).
const TYPES: [(&str, &str); 3] = [
    ("Expense", "expense-type-expense"),
    ("Gain", "expense-type-gain"),
    ("Transfer", "expense-type-transfer"),
];

fn type_value(t: &ExpenseType) -> &'static str {
    match t {
        ExpenseType::Expense => "Expense",
        ExpenseType::Gain => "Gain",
        ExpenseType::Transfer => "Transfer",
    }
}

/// `date_str`'s shape, matching `AddExpenseModal::today_iso`. The chips both set and highlight
/// against these, so a stale one would light up the wrong chip rather than write a wrong date.
fn today_iso() -> String {
    chrono::Utc::now().naive_utc().date().format("%Y-%m-%d").to_string()
}

fn yesterday_iso() -> String {
    (chrono::Utc::now().naive_utc().date() - chrono::Duration::days(1))
        .format("%Y-%m-%d")
        .to_string()
}

fn type_from_value(v: &str) -> ExpenseType {
    match v {
        "Gain" => ExpenseType::Gain,
        "Transfer" => ExpenseType::Transfer,
        _ => ExpenseType::Expense,
    }
}

/// Settled when the checked amounts add up to the total, quantised as the fieldset's counter and
/// `validate_expense_form` do.
fn side_balances(entries: &[UserEntry], total: f64) -> bool {
    sums_to_total(total, entries.iter().filter(|e| e.checked).map(|e| e.amount))
}

/// Recomputes the project-currency total and re-splits both sides.
///
/// The single place the three inputs that can change it — the amount, the currency and the rate —
/// converge, so they cannot drift apart. A `None` total means the form cannot convert yet (a blank
/// rate with no rate table, or one still being typed); the split is left alone rather than shown
/// wrong, exactly as the amount field already behaves mid-keystroke.
#[allow(clippy::too_many_arguments)]
fn apply_total(
    mut total_amount: Signal<f64>,
    mut payers: Signal<Vec<UserEntry>>,
    mut debtors: Signal<Vec<UserEntry>>,
    payers_share_mode: Signal<bool>,
    debtors_share_mode: Signal<bool>,
    source_amount: f64,
    foreign: bool,
    rate: Option<f64>,
) {
    let Some(total) = project_total(source_amount, foreign, rate) else { return };
    total_amount.set(total);
    redistribute(total, &mut payers.write(), payers_share_mode());
    redistribute(total, &mut debtors.write(), debtors_share_mode());
}

#[derive(Props, Clone, PartialEq)]
pub struct ExpenseFormProps {
    // Translated strings, not keys: each caller picks a different message per mode.
    pub title: String,
    pub submit_label: String,
    pub loading_label: String,
    pub currency: String,
    pub show_type_selector: bool,
    pub expense_name: Signal<String>,
    pub date_str: Signal<String>,
    pub total_amount: Signal<f64>,
    pub expense_type: Signal<ExpenseType>,
    pub payers: Signal<Vec<UserEntry>>,
    pub debtors: Signal<Vec<UserEntry>>,
    pub category: Signal<Option<String>>,
    pub payers_share_mode: Signal<bool>,
    pub debtors_share_mode: Signal<bool>,
    pub error_msg: Signal<Option<String>>,
    pub loading: Signal<bool>,
    /// The currency the amount is typed in. Starts as `currency`; when it differs, the rate field
    /// appears and `total_amount` holds the *converted* value while `source_amount` holds what was
    /// typed.
    pub expense_currency: Signal<String>,
    /// The amount as typed, in `expense_currency`. Equal to `total_amount` in the common case.
    pub source_amount: Signal<f64>,
    /// The rate field's raw contents. Blank means "use the InforEuro rate".
    pub rate_input: Signal<String>,
    /// The InforEuro rate from `expense_currency` into `currency`, and the first day of the month
    /// it is valid for. `None` when the table could not be obtained — the user must then type a
    /// rate.
    /// Resolved by the parent so this component stays free of fetching and caching.
    pub auto_rate: Option<f64>,
    pub rate_day: Option<String>,
    pub on_submit: EventHandler<FormEvent>,
    pub on_close: EventHandler<()>,
    /// A message key rendered above the amount field, or `None`. A receipt scan sets it when the
    /// total came from the fallback rather than a keyword anchor — the one place a low-confidence
    /// read is surfaced.
    pub amount_hint: Option<&'static str>,
    /// The Repeat row, shown only when set.
    #[props(default)]
    pub repeat: Option<RepeatSlot>,
    /// False for a recurring rule, whose date is its next occurrence rather than today or yesterday.
    #[props(default = true)]
    pub date_chips: bool,
    #[props(default)]
    pub banner: Option<String>,
    /// On an occurrence of a rule: whether saving also changes the next ones, and the hint naming
    /// the date that change applies from.
    #[props(default)]
    pub apply_to_next: Option<(Signal<bool>, String)>,
}

#[derive(Clone, Copy, PartialEq)]
pub struct RepeatSlot {
    pub value: Signal<Option<Repeat>>,
    pub today: chrono::NaiveDate,
    pub backfill: bool,
}

#[component]
pub fn ExpenseForm(props: ExpenseFormProps) -> Element {
    let mut expense_name = props.expense_name;
    let mut date_str = props.date_str;
    let mut total_amount = props.total_amount;
    let mut expense_type = props.expense_type;
    let mut category = props.category;
    let mut payers = props.payers;
    let mut debtors = props.debtors;
    let payers_share_mode = props.payers_share_mode;
    let debtors_share_mode = props.debtors_share_mode;
    let error_msg = props.error_msg;
    let loading = props.loading;
    let mut expense_currency = props.expense_currency;
    let mut source_amount = props.source_amount;
    let mut rate_input = props.rate_input;
    let is_online = try_use_context::<Signal<bool>>();

    let project_currency = props.currency.clone();
    let foreign = expense_currency() != project_currency;
    let rate = effective_rate(&rate_input(), props.auto_rate);

    // The rate table arrives asynchronously, so `auto_rate` can flip from `None` to `Some` *after* the
    // user picked the currency and typed the amount. Without this the handlers below would have
    // bailed out (no rate yet), leaving `total_amount` on the pre-conversion figure while the
    // preview showed the converted one — and submit would then write that stale total next to
    // correct conversion metadata, which is precisely the inconsistency the payload must never
    // carry. Reconciles only on a real change, so editing one participant's share is left alone.
    let auto_rate = props.auto_rate;
    let reconcile_currency = props.currency.clone();
    use_effect(move || {
        let foreign = expense_currency() != reconcile_currency;
        let rate = effective_rate(&rate_input(), auto_rate);
        let Some(total) = project_total(source_amount(), foreign, rate) else { return };
        if shared::to_cents(total) == shared::to_cents(*total_amount.peek()) {
            return;
        }
        total_amount.set(total);
        redistribute(total, &mut payers.write(), *payers_share_mode.peek());
        redistribute(total, &mut debtors.write(), *debtors_share_mode.peek());
    });
    // Everything the three handlers need to re-split, gathered once so they cannot disagree.
    let resplit = move |source: f64, foreign: bool, rate: Option<f64>| {
        apply_total(
            total_amount,
            payers,
            debtors,
            payers_share_mode,
            debtors_share_mode,
            source,
            foreign,
            rate,
        );
    };

    let on_close_x = props.on_close;
    let on_close_cancel = props.on_close;
    let on_close_backdrop = props.on_close;
    let on_close_esc = props.on_close;

    // Debtors is the side that actually gets edited; payers is almost always right as seeded.
    let mut payers_open = use_signal(|| false);
    let mut debtors_open = use_signal(|| true);

    // Every `AmountInput` below publishes its draft here while focused; `AmountOperatorBar` reads it.
    use_context_provider::<FocusedAmount>(|| Signal::new(None));

    // A side that stops adding up forces itself open — collapsing must never hide the shortfall.
    let payers_expanded = payers_open() || !side_balances(&payers(), total_amount());
    let debtors_expanded = debtors_open() || !side_balances(&debtors(), total_amount());

    // `shrink-0` is load-bearing: `.collapse` is `overflow: hidden`, giving a flex item an
    // automatic minimum size of zero (CSS flexbox §4.5). Without it these shrink away inside the
    // bounded scroll body instead of making it scroll, clipping their own rows.
    let collapse_class = |expanded: bool| {
        // `w-auto`: daisyUI's `.collapse` is `width: 100%`, which a margin does not reduce — with
        // `mx-5` the box became a 100%-wide element with 2.5rem of margin around it and pushed
        // everything inside it off the right edge of the sheet. The utility is layered above
        // daisyUI's component rule, and the box then stretches to the scroller minus its margins.
        if expanded {
            "collapse collapse-arrow bg-base-200/60 rounded-box shrink-0 w-auto mx-5 mt-3 collapse-open"
        } else {
            "collapse collapse-arrow bg-base-200/60 rounded-box shrink-0 w-auto mx-5 mt-3 collapse-close"
        }
    };

    rsx! {
        div {
            class: "modal modal-open modal-bottom sm:modal-middle",
            role: "dialog",
            aria_modal: "true",
            aria_labelledby: "expense-form-title",
            tabindex: "-1",
            onkeydown: move |e| {
                if e.key() == Key::Escape {
                    e.prevent_default();
                    on_close_esc.call(());
                }
            },
            div { class: "modal-box max-w-md p-0 flex flex-col",
                div { class: "flex items-center gap-2 px-4 pt-3 pb-2 flex-shrink-0",
                    h3 { id: "expense-form-title", class: "sr-only", "{props.title}" }
                    // The type is a mode switch — it renames both participant sections and decides
                    // whether the category field exists at all — so it leads, above everything it
                    // governs. Styled as the app's other tab bars rather than as buttons: the
                    // loudest control on the sheet should not be the one that is changed least.
                    if props.show_type_selector {
                        div { role: "tablist", class: "tabs tabs-box shadow-soft flex-1 flex-nowrap",
                            for (value , label) in TYPES {
                                // `min-w-0`: overrides daisyUI's `min-width: fit-content` on radio
                                // tabs, so the three share the row beside the ✕.
                                //
                                // `text-sm!`: these are radios, so `main.css`'s unlayered iOS-zoom
                                // guard `input { font-size: 16px !important }` outranks `btn-sm`.
                                // Tailwind's `!important` lives in `@layer utilities` and layer
                                // order reverses for important declarations, so the layered one
                                // wins. Radios never focus-auto-zoom anyway.
                                input {
                                    class: if type_value(&expense_type()) == value {
                                        "tab tab-active flex-1 min-w-0 whitespace-nowrap text-sm! font-semibold"
                                    } else {
                                        "tab flex-1 min-w-0 whitespace-nowrap text-sm! text-base-content/70"
                                    },
                                    r#type: "radio",
                                    name: "expense-type",
                                    aria_label: tid!(label),
                                    checked: type_value(&expense_type()) == value,
                                    oninput: move |_| {
                                        haptic(Haptic::Light);
                                        expense_type.set(type_from_value(value));
                                    },
                                }
                            }
                        }
                    } else {
                        span { class: "flex-1 font-bold text-lg font-display truncate", "{props.title}" }
                    }
                    button {
                        id: "expense-form-close",
                        r#type: "button",
                        class: "btn btn-ghost btn-circle h-11 w-11 min-h-11",
                        aria_label: tid!("close"),
                        onclick: move |_| on_close_x.call(()),
                        CloseIcon { size: ICON_HEADER }
                    }
                }

                form { class: "flex flex-col flex-1 overflow-hidden", onsubmit: props.on_submit,
                div { class: "flex-1 overflow-y-auto flex flex-col",
                if let Some(err) = error_msg() {
                    div { id: "expense-form-error", role: "alert", class: "alert alert-error text-sm mx-5 mb-2", "{err}" }
                }

                    if let Some(banner) = props.banner.clone() {
                        div { id: "expense-form-banner", class: "alert alert-info alert-soft text-sm py-2 mx-5 mb-2", "{banner}" }
                    }

                    if let Some(hint) = props.amount_hint {
                        div {
                            id: "amount-hint",
                            class: "alert alert-info alert-soft text-sm py-2 mx-5 mb-2",
                            role: "status",
                            {tid!(hint)}
                        }
                    }

                    // The amount leads and takes the focus: it is the one figure the user always
                    // knows, and its `inputmode="decimal"` makes the numeric keyboard the only one
                    // the common case needs — the name, still being decided, follows it. No `input`
                    // box: the rule below is the field's whole edge, so the currency picker stops
                    // reading as a second field sharing a frame.
                    label { class: "relative flex items-baseline gap-2 px-5 pt-3 pb-4 border-b border-base-200 focus-within:border-primary transition-colors",
                            span { class: "sr-only", {tid!("field-amount")} }
                            AmountInput {
                                id: "expense-amount",
                                class: "min-w-0 text-4xl leading-tight font-bold tracking-tight font-display tabular-nums caret-primary",
                                fit: true,
                                aria_required: "true",
                                enterkeyhint: "next",
                                focus_on_mount: true,
                                value: source_amount(),
                                oninput: move |raw: String| {
                                    let Some(v) = parse_amount(&raw) else { return };
                                    source_amount.set(v);
                                    resplit(v, foreign, rate);
                                },
                            }
                            CurrencyPicker {
                                id: "expense-currency",
                                value: expense_currency(),
                                label: tid!("expense-currency"),
                                button_class: "flex items-center gap-1 text-base font-medium text-base-content/70 bg-transparent border-0 focus:outline-none cursor-pointer",
                                onchange: move |picked: String| {
                                    let now_foreign = picked != project_currency;
                                    expense_currency.set(picked);
                                    // The rate that applied to the old currency means nothing for
                                    // the new one — clearing it falls back to the InforEuro rate.
                                    rate_input.set(String::new());
                                    resplit(source_amount(), now_foreign, props.auto_rate);
                                },
                            }
                        }

                    div { class: "flex items-center gap-2 px-5 py-1 border-b border-base-200",
                        if expense_type() == ExpenseType::Expense {
                            // The chip shows the emoji alone; the transparent native select on top
                            // takes the tap and opens the full labelled list. A closed `<select>`
                            // is sized to its widest option, so shown directly it truncated.
                            label { class: "relative shrink-0 flex items-center gap-1 h-9 px-2.5 rounded-full bg-base-200 text-lg has-[select:focus-visible]:outline-2 has-[select:focus-visible]:outline-primary",
                            span { aria_hidden: "true",
                                {parent_emoji(category().as_deref().unwrap_or(infer_chart_category(&expense_name())))}
                            }
                            span { aria_hidden: "true", class: "text-xs opacity-60", "▾" }
                            select {
                                id: "expense-category",
                                class: "absolute inset-0 w-full opacity-0 cursor-pointer",
                                aria_label: tid!("expense-category"),
                                value: category().unwrap_or_default(),
                                oninput: move |e| {
                                    let v = e.value();
                                    category.set(if v.is_empty() { None } else { Some(v) });
                                },
                                // The empty option previews what the name files under, so
                                // "automatic" stops being a guess.
                                {
                                    let inferred = infer_chart_category(&expense_name());
                                    rsx! {
                                        option { value: "", {tid!("expense-category-auto", emoji: parent_emoji(inferred))} }
                                    }
                                }
                                for cat in CATEGORIES.iter().copied() {
                                    option {
                                        value: "{cat}",
                                        selected: category().as_deref() == Some(cat),
                                        "{parent_emoji(cat)} {category_label(cat)}"
                                    }
                                }
                            }
                            }
                        }
                        label { class: "sr-only", r#for: "expense-name", {tid!("field-name")} }
                        input {
                            id: "expense-name",
                            class: "input input-ghost grow min-w-0 px-1",
                            r#type: "text",
                            enterkeyhint: "next",
                            aria_required: "true",
                            aria_describedby: if error_msg().is_some() { "expense-form-error" },
                            placeholder: tid!("expense-name-placeholder"),
                            value: "{expense_name}",
                            oninput: move |e| expense_name.set(e.value()),
                        }
                    }

                    // Nearly every expense is entered the day it happened or the day after, and
                    // both used to cost opening the native calendar. The field itself stays — it is
                    // the third chip, visible rather than hidden behind the other two.
                    div { class: "flex items-center gap-2 px-5 py-2 border-b border-base-200",
                        if props.date_chips {
                            label { class: "sr-only", r#for: "expense-date", {tid!("field-date")} }
                            button {
                                r#type: "button",
                                class: if date_str() == today_iso() { "btn btn-sm btn-secondary" } else { "btn btn-sm btn-outline" },
                                onclick: move |_| date_str.set(today_iso()),
                                {tid!("date-today")}
                            }
                            button {
                                r#type: "button",
                                class: if date_str() == yesterday_iso() { "btn btn-sm btn-secondary" } else { "btn btn-sm btn-outline" },
                                onclick: move |_| date_str.set(yesterday_iso()),
                                {tid!("date-yesterday")}
                            }
                        } else {
                            label { class: "text-sm text-base-content/70", r#for: "expense-date", {tid!("recurring-next-on-label")} }
                        }
                        input {
                            id: "expense-date",
                            class: "input input-sm w-auto min-w-0 rounded-full tabular-nums",
                            r#type: "date",
                            value: "{date_str}",
                            oninput: move |e| date_str.set(e.value()),
                        }
                    }

                    if let Some(slot) = props.repeat {
                        RepeatRow {
                            repeat: slot.value,
                            date_str,
                            online: is_online.map(|s| s()).unwrap_or(true),
                            today: slot.today,
                            foreign: foreign.then(|| (total_amount(), props.currency.clone())),
                            backfill: slot.backfill,
                        }
                    }

                    if foreign {
                        fieldset { class: "fieldset",
                            label { class: "fieldset-legend", r#for: "expense-rate", {tid!("expense-rate")} }
                            input {
                                id: "expense-rate",
                                class: "input w-full tabular-nums",
                                r#type: "text",
                                inputmode: "decimal",
                                aria_describedby: "expense-rate-hint",
                                placeholder: match props.auto_rate {
                                    Some(r) => format!("{r}"),
                                    None => String::new(),
                                },
                                value: "{rate_input}",
                                oninput: move |e| {
                                    let raw = e.value();
                                    let next = effective_rate(&raw, props.auto_rate);
                                    rate_input.set(raw);
                                    resplit(source_amount(), true, next);
                                },
                            }
                            p {
                                id: "expense-rate-hint",
                                class: "text-xs text-base-content/70",
                                match (props.auto_rate, props.rate_day.as_deref()) {
                                    (Some(r), Some(day)) => tid!(
                                        "expense-rate-hint",
                                        month: format_month_str(day),
                                        from: expense_currency(),
                                        to: props.currency.clone(),
                                        rate: format!("{r:.3}")
                                    ),
                                    _ => tid!("expense-rate-unavailable"),
                                }
                            }
                            if let Some(total) = project_total(source_amount(), true, rate) {
                                p {
                                    id: "expense-rate-preview",
                                    class: "label text-xs font-semibold tabular-nums",
                                    "≈ {total:.2} {props.currency}"
                                }
                            }
                        }
                    }

                    div { class: collapse_class(payers_expanded),
                        button {
                            r#type: "button",
                            // Layout utilities only: daisyUI's padding reserves the 3rem the
                            // absolute arrow needs, and its min-height is what the arrow's fixed
                            // `top` measures against.
                            class: "collapse-title flex items-center justify-between gap-3 text-left",
                            aria_expanded: "{payers_expanded}",
                            onclick: move |_| payers_open.toggle(),
                            span { class: "text-sm font-medium shrink-0", {tid!(payers_label(&expense_type()))} }
                            span { class: "text-sm text-base-content/70 truncate", {participants_summary(&payers()).label()} }
                        }
                        div { class: "collapse-content",
                            ParticipantsFieldset {
                                id: "expense-payers",
                                legend: tid!(payers_label(&expense_type())),
                                currency: props.currency.clone(),
                                total: total_amount,
                                entries: payers,
                                share_mode: payers_share_mode,
                            }
                        }
                    }

                    div { class: collapse_class(debtors_expanded),
                        button {
                            r#type: "button",
                            // Layout utilities only: daisyUI's padding reserves the 3rem the
                            // absolute arrow needs, and its min-height is what the arrow's fixed
                            // `top` measures against.
                            class: "collapse-title flex items-center justify-between gap-3 text-left",
                            aria_expanded: "{debtors_expanded}",
                            onclick: move |_| debtors_open.toggle(),
                            span { class: "text-sm font-medium shrink-0", {tid!(debtors_label(&expense_type()))} }
                            span { class: "text-sm text-base-content/70 truncate", {participants_summary(&debtors()).label()} }
                        }
                        div { class: "collapse-content",
                            ParticipantsFieldset {
                                id: "expense-debtors",
                                legend: tid!(debtors_label(&expense_type())),
                                currency: props.currency.clone(),
                                total: total_amount,
                                entries: debtors,
                                share_mode: debtors_share_mode,
                            }
                        }
                    }

                    if let Some((mut apply_next, hint)) = props.apply_to_next.clone() {
                        fieldset { id: "apply-to", class: "flex flex-col gap-1 px-5 pt-4",
                            legend { class: "text-xs font-bold uppercase tracking-wide text-base-content/70 pb-1", {tid!("apply-to")} }
                            label { class: "flex items-center gap-3 py-1 cursor-pointer",
                                input {
                                    id: "apply-this-only",
                                    r#type: "radio",
                                    name: "apply-to",
                                    class: "radio radio-primary radio-sm",
                                    checked: !apply_next(),
                                    onchange: move |_| apply_next.set(false),
                                }
                                span { class: "text-sm", {tid!("apply-this-only")} }
                            }
                            label { class: "flex items-center gap-3 py-1 cursor-pointer",
                                input {
                                    id: "apply-and-next",
                                    r#type: "radio",
                                    name: "apply-to",
                                    class: "radio radio-primary radio-sm",
                                    checked: apply_next(),
                                    onchange: move |_| apply_next.set(true),
                                }
                                span { class: "flex flex-col",
                                    span { class: "text-sm", {tid!("apply-and-next")} }
                                    span { class: "text-xs text-base-content/70", "{hint}" }
                                }
                            }
                        }
                    }

                } // end scrollable body
                    div { class: "modal-action m-0 flex justify-end gap-2 px-5 py-3 border-t border-base-200 flex-shrink-0",
                        // On a phone the ✕, the backdrop and the swipe down all cancel already, so
                        // the second button only competes with the one action worth a full target.
                        button {
                            r#type: "button",
                            class: "btn btn-ghost hidden sm:inline-flex",
                            onclick: move |_| on_close_cancel.call(()),
                            {tid!("cancel")}
                        }
                        button {
                            id: "expense-form-submit",
                            r#type: "submit",
                            class: "btn btn-primary flex-1 sm:flex-none",
                            disabled: loading(),
                            if loading() {
                                span { class: "loading loading-spinner loading-sm" }
                                "{props.loading_label}"
                            } else {
                                "{props.submit_label}"
                            }
                        }
                    }

                    AmountOperatorBar {}
                }
            }

            div {
                class: "modal-backdrop",
                onclick: move |_| on_close_backdrop.call(()),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dioxus::dioxus_core::ElementId;
    use crate::common::test_dom::{click, focus, form_input, listener_ids, texts, value_writes};
    use crate::expenses::helpers::expense_form_helpers::init_entries;
    use shared::{EncryptedPair, User};
    use std::cell::RefCell;
    use std::rc::Rc;

    /// (checked, amount) per row for payers and debtors, mirrored out on every render.
    type Rows = Vec<(bool, f64)>;
    type Spy = Rc<RefCell<(Rows, Rows)>>;

    #[derive(Props, Clone, PartialEq)]
    struct HarnessProps {
        spy: Spy,
        /// The currency the amount is typed in. Equal to the project's ("EUR") in every test but
        /// the conversion ones, which is what keeps the rate field out of the default template.
        #[props(default = "EUR".to_string())]
        expense_currency: String,
        #[props(default)]
        auto_rate: Option<f64>,
    }

    /// Owns the signals `ExpenseForm` mutates, mirroring them into `spy` per render — reading them
    /// here subscribes the harness. Both sides start checked so assertions never depend on which
    /// fieldset the VirtualDom creates first.
    #[component]
    fn Harness(props: HarnessProps) -> Element {
        // The component translates its labels, and `tid!` panics with no provider above it.
        crate::i18n::use_test_i18n();
        let all_checked = || {
            let users: Vec<User> = (1..=2)
                .map(|id| User { id, payload: EncryptedPair::default(), ..Default::default() })
                .collect();
            let names = vec!["Alice".to_string(), "Bob".to_string()];
            let mut entries = init_entries(&users, &names, None, 0.0, false);
            for e in entries.iter_mut() {
                e.checked = true;
                e.shares = 1.0;
            }
            entries
        };
        let payers = use_signal(all_checked);
        let debtors = use_signal(all_checked);

        let rows = |e: &Vec<UserEntry>| -> Rows { e.iter().map(|e| (e.checked, e.amount)).collect() };
        *props.spy.borrow_mut() = (rows(&payers()), rows(&debtors()));

        let init_currency = props.expense_currency.clone();

        rsx! {
            ExpenseForm {
                title: "t",
                submit_label: "s",
                loading_label: "l",
                currency: "EUR".to_string(),
                show_type_selector: false,
                expense_name: use_signal(String::new),
                date_str: use_signal(String::new),
                total_amount: use_signal(|| 0.0),
                expense_type: use_signal(|| ExpenseType::Expense),
                payers,
                debtors,
                category: use_signal(|| None),
                payers_share_mode: use_signal(|| false),
                debtors_share_mode: use_signal(|| false),
                error_msg: use_signal(|| None),
                loading: use_signal(|| false),
                expense_currency: use_signal(move || init_currency),
                source_amount: use_signal(|| 0.0),
                rate_input: use_signal(String::new),
                auto_rate: props.auto_rate,
                rate_day: props.auto_rate.map(|_| "2026-09-01".to_string()),
                on_submit: move |_| {},
                on_close: move |_| {},
            }
        }
    }

    /// The amount field is an `AmountInput` child component, whose listener is registered after
    /// everything in the form's own template — last in both the default and the foreign template.
    /// These indices are mutation order, not template order; reading the source and counting
    /// downwards gives the wrong answer here — measure it. The count assertion in `harness_with`
    /// is what catches it moving.
    fn total(ids: &[ElementId]) -> ElementId {
        *ids.last().unwrap()
    }

    /// The rate input, in the foreign-currency template only: inside an `if foreign` block, so
    /// after every static listener and before the amount component's.
    const RATE: usize = 12;

    fn harness() -> (VirtualDom, Spy, Vec<ElementId>) {
        harness_with(HarnessProps {
            spy: Rc::new(RefCell::new((Vec::new(), Vec::new()))),
            expense_currency: "EUR".to_string(),
            auto_rate: None,
        })
    }

    fn harness_with(props: HarnessProps) -> (VirtualDom, Spy, Vec<ElementId>) {
        let spy = props.spy.clone();
        let foreign = props.expense_currency != "EUR";
        let mut dom = VirtualDom::new_with_props(Harness, props);
        let ids = listener_ids(&dom.rebuild_to_vec(), "input");
        let expected = if foreign { 15 } else { 14 };
        assert_eq!(ids.len(), expected, "form template changed — re-index RATE above");
        (dom, spy, ids)
    }

    /// Foreign currency, InforEuro rate available: the amount is typed in USD and both sides split the
    /// **converted** total, which is what keeps `sum == total` in the project currency.
    #[test]
    fn a_foreign_amount_splits_the_converted_total() {
        let (mut dom, spy, ids) = harness_with(HarnessProps {
            spy: Rc::new(RefCell::new((Vec::new(), Vec::new()))),
            expense_currency: "USD".to_string(),
            auto_rate: Some(0.5),
        });

        dom.runtime().handle_event("input", form_input("30"), total(&ids));
        dom.render_immediate_to_vec();

        let (payers, debtors) = spy.borrow().clone();
        assert_eq!(payers, vec![(true, 7.5), (true, 7.5)], "30 USD at 0.5 is 15 EUR, split two ways");
        assert_eq!(debtors, vec![(true, 7.5), (true, 7.5)]);
    }

    /// A typed rate overrides the InforEuro one and re-splits immediately — no blur, same as the amount.
    #[test]
    fn typing_a_rate_resplits_the_existing_amount() {
        let (mut dom, spy, ids) = harness_with(HarnessProps {
            spy: Rc::new(RefCell::new((Vec::new(), Vec::new()))),
            expense_currency: "USD".to_string(),
            auto_rate: Some(0.5),
        });

        dom.runtime().handle_event("input", form_input("30"), total(&ids));
        dom.render_immediate_to_vec();
        dom.runtime().handle_event("input", form_input("2"), ids[RATE]);
        dom.render_immediate_to_vec();

        let (payers, _) = spy.borrow().clone();
        assert_eq!(payers, vec![(true, 30.0), (true, 30.0)], "30 USD at 2.0 is 60 EUR");
    }

    /// No rate anywhere: the split is left alone rather than shown wrong. Submit is what reports
    /// `RateUnavailable`.
    #[test]
    fn a_foreign_amount_with_no_rate_leaves_the_split_alone() {
        let (mut dom, spy, ids) = harness_with(HarnessProps {
            spy: Rc::new(RefCell::new((Vec::new(), Vec::new()))),
            expense_currency: "USD".to_string(),
            auto_rate: None,
        });

        dom.runtime().handle_event("input", form_input("30"), total(&ids));
        dom.render_immediate_to_vec();

        let (payers, _) = spy.borrow().clone();
        assert_eq!(payers, vec![(true, 0.0), (true, 0.0)]);
    }

    #[test]
    fn typing_the_total_splits_it_on_both_sides_without_waiting_for_a_blur() {
        let (mut dom, spy, ids) = harness();

        dom.runtime().handle_event("input", form_input("30"), total(&ids));
        dom.render_immediate_to_vec();

        let (payers, debtors) = spy.borrow().clone();
        assert_eq!(payers, vec![(true, 15.0), (true, 15.0)]);
        assert_eq!(debtors, vec![(true, 15.0), (true, 15.0)]);
    }

    #[test]
    fn the_total_opens_showing_zero() {
        let mut dom = VirtualDom::new_with_props(
            Harness,
            HarnessProps {
                spy: Rc::new(RefCell::new((Vec::new(), Vec::new()))),
                expense_currency: "EUR".to_string(),
                auto_rate: None,
            },
        );
        let m = dom.rebuild_to_vec();
        let field = total(&listener_ids(&m, "input"));

        assert_eq!(value_writes(&m, field), vec!["0".to_string()]);
    }

    /// An empty field is 0: the split follows at once, and the "0" only lands on blur so the
    /// keystroke in progress is not clobbered.
    #[test]
    fn clearing_the_total_splits_zero_and_blur_shows_it() {
        let (mut dom, spy, ids) = harness();
        let field = total(&ids);

        dom.runtime().handle_event("input", form_input("30"), field);
        dom.render_immediate_to_vec();
        dom.runtime().handle_event("input", form_input(""), field);
        let m = dom.render_immediate_to_vec();

        assert_eq!(spy.borrow().0, vec![(true, 0.0), (true, 0.0)]);
        assert_eq!(spy.borrow().1, vec![(true, 0.0), (true, 0.0)]);
        assert!(!value_writes(&m, field).contains(&"0".to_string()));

        dom.runtime().handle_event("blur", focus(), field);
        let m = dom.render_immediate_to_vec();
        assert_eq!(value_writes(&m, field), vec!["0".to_string()]);
    }

    #[test]
    fn the_total_accepts_the_decimal_comma() {
        let (mut dom, spy, ids) = harness();

        dom.runtime().handle_event("input", form_input("12,50"), total(&ids));
        dom.render_immediate_to_vec();

        assert_eq!(spy.borrow().0, vec![(true, 6.25), (true, 6.25)]);
    }

    /// `value` is volatile in Dioxus — rewritten on every render — and an `f64` formats "12." as
    /// "12", which is how a mobile keyboard's decimal key got erased under the user's finger.
    #[test]
    fn a_trailing_separator_survives_the_render() {
        let (mut dom, spy, ids) = harness();

        for keystroke in ["12.", "12,", "12,0", "12.50"] {
            dom.runtime().handle_event("input", form_input(keystroke), total(&ids));
            let m = dom.render_immediate_to_vec();
            for written in value_writes(&m, total(&ids)) {
                assert_eq!(written, keystroke, "the render rewrote the field under the keystroke");
            }
        }
        assert_eq!(spy.borrow().0, vec![(true, 6.25), (true, 6.25)]);
    }

    /// Both sides follow whatever the total becomes, so a form never edited by hand always passes
    /// `validate_expense_form`.
    #[test]
    fn every_total_keystroke_leaves_both_sides_adding_up() {
        let (mut dom, spy, ids) = harness();

        for keystroke in ["1", "12", "12.5", "125", "100,005", "99999.99"] {
            dom.runtime().handle_event("input", form_input(keystroke), total(&ids));
            dom.render_immediate_to_vec();

            let total = crate::expenses::helpers::expense_form_helpers::parse_amount(keystroke)
                .expect("keystroke parses");
            let (payers, debtors) = spy.borrow().clone();
            assert!(
                shared::sums_to_total(total, payers.iter().map(|(_, a)| *a)),
                "payers do not add up after typing {keystroke}: {payers:?}"
            );
            assert!(
                shared::sums_to_total(total, debtors.iter().map(|(_, a)| *a)),
                "debtors do not add up after typing {keystroke}: {debtors:?}"
            );
        }
    }
    /// A mobile keyboard can submit without ever blurring, so an expression is evaluated on the
    /// keystroke like any other amount; blur only swaps the text on screen for the number.
    #[test]
    fn an_expression_splits_on_the_keystroke_and_blur_only_settles_the_text() {
        let (mut dom, spy, ids) = harness();
        let field = total(&ids);

        dom.runtime().handle_event("input", form_input("250/3"), field);
        let m = dom.render_immediate_to_vec();
        let (payers, debtors) = spy.borrow().clone();
        assert_eq!(payers, vec![(true, 41.67), (true, 41.66)], "split before any blur");
        assert_eq!(debtors, vec![(true, 41.67), (true, 41.66)]);
        for written in value_writes(&m, field) {
            assert_eq!(written, "250/3", "the expression stays on screen while focused");
        }

        dom.runtime().handle_event("blur", focus(), field);
        let m = dom.render_immediate_to_vec();
        assert_eq!(*spy.borrow(), (payers, debtors), "blur may settle the text, never the numbers");
        assert_eq!(value_writes(&m, field), vec!["83.33".to_string()]);
    }

    /// The bar is below the footer and targets whatever amount field has focus — here, the total.
    /// Which row it targets when a participant is focused is covered in `participants_fieldset`.
    #[test]
    fn the_operator_bar_appears_on_focus_and_drives_the_total() {
        let (mut dom, spy, ids) = harness();
        let field = total(&ids);

        dom.runtime().handle_event("input", form_input("320"), field);
        let m = dom.render_immediate_to_vec();
        assert!(listener_ids(&m, "click").is_empty(), "no bar until a field is focused");

        dom.runtime().handle_event("focus", focus(), field);
        let m = dom.render_immediate_to_vec();
        let keys = listener_ids(&m, "click");
        assert_eq!(keys.len(), 5, "the bar carries +, −, ×, ÷ and =");

        // "−", then the keystroke the user types after it — the browser sends the whole value.
        dom.runtime().handle_event("click", click(), keys[1]);
        let m = dom.render_immediate_to_vec();
        assert_eq!(value_writes(&m, field).last().map(String::as_str), Some("320-"));

        dom.runtime().handle_event("input", form_input("320-25"), field);
        let m = dom.render_immediate_to_vec();
        let (payers, debtors) = spy.borrow().clone();
        assert_eq!(payers, vec![(true, 147.5), (true, 147.5)], "295 split two ways");
        assert_eq!(debtors, vec![(true, 147.5), (true, 147.5)]);
        assert!(
            texts(&m).iter().any(|t| t == "320-25"),
            "the expression shows in the bubble: {:?}",
            texts(&m)
        );

        // "=" settles the text without dropping focus, so the bar stays.
        dom.runtime().handle_event("click", click(), keys[4]);
        let m = dom.render_immediate_to_vec();
        assert_eq!(value_writes(&m, field).last().map(String::as_str), Some("295"));
        assert_eq!(*spy.borrow(), (payers, debtors), "settling the text moves no number");
    }

    /// A plain draft is left alone on blur: "12,50" must not come back as "12.5".
    #[test]
    fn blur_leaves_a_plain_draft_as_typed() {
        let (mut dom, _, ids) = harness();
        let field = total(&ids);

        dom.runtime().handle_event("input", form_input("12,50"), field);
        dom.render_immediate_to_vec();
        dom.runtime().handle_event("blur", focus(), field);
        let m = dom.render_immediate_to_vec();
        assert!(value_writes(&m, field).is_empty(), "nothing to settle, nothing rewritten");
    }
}
