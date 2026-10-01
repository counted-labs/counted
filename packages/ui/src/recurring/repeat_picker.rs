use chrono::NaiveDate;
use dioxus::prelude::*;

use super::labels::{anchor_label, ends_label, frequency_label, lands_on_month_end, repeat_summary, PRESETS};
use super::model::{Ends, Freq};
use super::schedule::last_date;
use super::view::{backfill_count, Repeat};
use crate::common::format_date;
use crate::icons::{CloseIcon, RepeatIcon, RightArrowIcon, ICON_HEADER, ICON_INLINE};
use crate::tid;

fn parse(date: &str) -> Option<NaiveDate> {
    NaiveDate::parse_from_str(date, "%Y-%m-%d").ok()
}

/// The Repeat line of the expense form. `foreign` is the frozen project-currency amount when the
/// expense is typed in another currency; `backfill` shows how many occurrences saving writes now.
#[component]
pub fn RepeatRow(
    repeat: Signal<Option<Repeat>>,
    date_str: ReadSignal<String>,
    online: bool,
    today: NaiveDate,
    foreign: Option<(f64, String)>,
    #[props(default = true)] backfill: bool,
) -> Element {
    let mut open = use_signal(|| false);
    let anchor = parse(&date_str()).unwrap_or(today);

    if !online {
        return rsx! {
            div { id: "repeat-row", class: "flex items-center gap-3 px-5 py-2 border-b border-base-200 text-base-content/70",
                span { class: "shrink-0 w-8 h-8 rounded-full bg-base-200 flex items-center justify-center",
                    RepeatIcon { size: ICON_INLINE }
                }
                div { class: "flex-1 min-w-0",
                    p { class: "text-sm font-medium", {tid!("repeat-label")} }
                    p { class: "text-xs", {tid!("repeat-offline")} }
                }
            }
        };
    }

    let current = repeat();
    let due_now = match (current, backfill) {
        (Some(r), true) => backfill_count(r, anchor, today),
        _ => 0,
    };

    rsx! {
        button {
            id: "repeat-row",
            r#type: "button",
            class: "flex items-center gap-3 px-5 py-2 border-b border-base-200 text-left w-full hover:bg-base-200/50",
            onclick: move |_| open.set(true),
            span {
                class: if current.is_some() { "shrink-0 w-8 h-8 rounded-full bg-primary/15 text-primary flex items-center justify-center" } else { "shrink-0 w-8 h-8 rounded-full bg-base-200 text-base-content/70 flex items-center justify-center" },
                RepeatIcon { size: ICON_INLINE }
            }
            div { class: "flex-1 min-w-0",
                match current {
                    None => rsx! {
                        p { class: "text-sm text-base-content/70", {tid!("repeat-none")} }
                    },
                    Some(r) => rsx! {
                        p { class: "text-sm font-semibold truncate", {repeat_summary(&r, anchor)} }
                        p { class: "text-xs text-base-content/70 truncate", {ends_label(&r.rule(anchor))} }
                        if let Some((amount, currency)) = foreign.clone() {
                            p { class: "text-xs text-base-content/70",
                                {tid!("repeat-foreign", amount: format!("{amount:.2}"), currency: currency)}
                            }
                        }
                    },
                }
            }
            RightArrowIcon { size: ICON_INLINE }
        }
        if due_now > 0 {
            div { id: "repeat-backfill", role: "status", class: "alert alert-info alert-soft text-sm py-2 mx-5 mt-2",
                {tid!("repeat-backfill", count: due_now as i64)}
            }
        }
        if open() {
            RepeatPicker {
                initial: current,
                anchor,
                on_done: move |picked| {
                    repeat.set(picked);
                    open.set(false);
                },
                on_close: move |_| open.set(false),
            }
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Choice {
    None,
    Preset(Freq, u32),
    Custom,
}

#[component]
pub fn RepeatPicker(
    initial: Option<Repeat>,
    anchor: NaiveDate,
    on_done: EventHandler<Option<Repeat>>,
    on_close: EventHandler<()>,
) -> Element {
    let start = initial.unwrap_or_else(Repeat::monthly);
    let initial_choice = match initial {
        None => Choice::None,
        Some(r) if PRESETS.contains(&(r.freq, r.interval)) => Choice::Preset(r.freq, r.interval),
        Some(_) => Choice::Custom,
    };
    let mut choice = use_signal(|| initial_choice);
    let mut custom_freq = use_signal(|| start.freq);
    let mut custom_interval = use_signal(|| start.interval.max(2));
    let mut ends = use_signal(|| start.ends);
    let mut until = use_signal(|| match start.ends {
        Ends::On(d) => d,
        _ => anchor.checked_add_months(chrono::Months::new(12)).unwrap_or(anchor),
    });
    let mut count = use_signal(|| match start.ends {
        Ends::After(n) => n,
        _ => 12,
    });
    let mut variable = use_signal(|| start.variable);

    let picked = move || -> Option<Repeat> {
        let (freq, interval) = match choice() {
            Choice::None => return None,
            Choice::Preset(f, i) => (f, i),
            Choice::Custom => (custom_freq(), custom_interval().max(1)),
        };
        Some(Repeat { freq, interval, ends: ends(), variable: variable() })
    };
    let preview = picked();

    rsx! {
        div { class: "modal modal-open modal-bottom sm:modal-middle", role: "dialog", aria_labelledby: "repeat-title",
            div { class: "modal-box max-w-md p-0 flex flex-col",
                div { class: "flex items-center gap-2 px-5 pt-4 pb-2",
                    h3 { id: "repeat-title", class: "flex-1 font-bold text-lg font-display", {tid!("repeat-label")} }
                    button {
                        r#type: "button",
                        class: "btn btn-ghost btn-circle h-11 w-11 min-h-11",
                        aria_label: tid!("close"),
                        onclick: move |_| on_close.call(()),
                        CloseIcon { size: ICON_HEADER }
                    }
                }
                div { class: "flex-1 overflow-y-auto flex flex-col pb-2",
                    ul { class: "flex flex-col px-3",
                        RepeatOption {
                            id: "repeat-opt-none",
                            selected: choice() == Choice::None,
                            label: tid!("repeat-none"),
                            onselect: move |_| choice.set(Choice::None),
                        }
                        for (freq , interval) in PRESETS {
                            RepeatOption {
                                id: "repeat-opt-{interval}-{freq:?}",
                                selected: choice() == Choice::Preset(freq, interval),
                                label: frequency_label(freq, interval),
                                hint: anchor_label(freq, anchor),
                                onselect: move |_| choice.set(Choice::Preset(freq, interval)),
                            }
                        }
                        RepeatOption {
                            id: "repeat-opt-custom",
                            selected: choice() == Choice::Custom,
                            label: tid!("repeat-custom"),
                            onselect: move |_| choice.set(Choice::Custom),
                        }
                    }
                    if choice() == Choice::Custom {
                        div { class: "flex items-center gap-2 px-5 pt-2",
                            span { class: "text-sm", {tid!("repeat-every")} }
                            input {
                                id: "repeat-interval",
                                class: "input input-sm w-20 tabular-nums",
                                r#type: "number",
                                inputmode: "numeric",
                                min: "1",
                                max: "99",
                                value: "{custom_interval}",
                                oninput: move |e| {
                                    if let Ok(n) = e.value().parse::<u32>() {
                                        custom_interval.set(n.clamp(1, 99));
                                    }
                                },
                            }
                            select {
                                id: "repeat-unit",
                                class: "select select-sm w-auto",
                                value: match custom_freq() { Freq::Weekly => "weekly", Freq::Monthly => "monthly", Freq::Yearly => "yearly" },
                                onchange: move |e| custom_freq.set(match e.value().as_str() {
                                    "weekly" => Freq::Weekly,
                                    "yearly" => Freq::Yearly,
                                    _ => Freq::Monthly,
                                }),
                                option { value: "weekly", {tid!("repeat-unit-weeks")} }
                                option { value: "monthly", {tid!("repeat-unit-months")} }
                                option { value: "yearly", {tid!("repeat-unit-years")} }
                            }
                        }
                    }
                    if let Some(p) = preview {
                        if lands_on_month_end(p.freq, anchor) {
                            p { class: "text-xs text-base-content/70 px-5 pt-2", {tid!("repeat-month-end")} }
                        }
                        p { class: "text-xs font-bold uppercase tracking-wide text-base-content/70 px-5 pt-4 pb-2", {tid!("repeat-ends")} }
                        div { role: "tablist", class: "tabs tabs-box mx-5",
                            EndsTab { label: tid!("repeat-ends-never"), active: matches!(ends(), Ends::Never), onselect: move |_| ends.set(Ends::Never) }
                            EndsTab { label: tid!("repeat-ends-on"), active: matches!(ends(), Ends::On(_)), onselect: move |_| ends.set(Ends::On(until())) }
                            EndsTab { label: tid!("repeat-ends-after"), active: matches!(ends(), Ends::After(_)), onselect: move |_| ends.set(Ends::After(count())) }
                        }
                        match ends() {
                            Ends::On(_) => rsx! {
                                div { class: "px-5 pt-3",
                                    input {
                                        id: "repeat-until",
                                        class: "input input-sm w-auto tabular-nums",
                                        r#type: "date",
                                        aria_label: tid!("repeat-ends-on"),
                                        min: "{anchor}",
                                        value: "{until}",
                                        oninput: move |e| {
                                            if let Some(d) = parse(&e.value()) {
                                                until.set(d);
                                                ends.set(Ends::On(d));
                                            }
                                        },
                                    }
                                }
                            },
                            Ends::After(n) => rsx! {
                                div { class: "flex items-center gap-3 px-5 pt-3 text-sm",
                                    div { class: "join",
                                        button {
                                            r#type: "button",
                                            class: "btn btn-sm join-item",
                                            aria_label: tid!("repeat-fewer"),
                                            onclick: move |_| { let v = count().saturating_sub(1).max(1); count.set(v); ends.set(Ends::After(v)); },
                                            "−"
                                        }
                                        span { id: "repeat-count", class: "btn btn-sm join-item pointer-events-none tabular-nums", "{n}" }
                                        button {
                                            r#type: "button",
                                            class: "btn btn-sm join-item",
                                            aria_label: tid!("repeat-more"),
                                            onclick: move |_| { let v = (count() + 1).min(999); count.set(v); ends.set(Ends::After(v)); },
                                            "+"
                                        }
                                    }
                                    if let Some(last) = last_date(&p.rule(anchor)) {
                                        span { class: "text-base-content/70", {tid!("repeat-last-on", date: format_date(last))} }
                                    }
                                }
                            },
                            Ends::Never => rsx! {},
                        }
                        label { class: "flex items-start gap-3 px-5 pt-4 cursor-pointer",
                            div { class: "flex-1",
                                p { class: "text-sm font-semibold", {tid!("repeat-variable")} }
                                p { class: "text-xs text-base-content/70", {tid!("repeat-variable-hint")} }
                            }
                            input {
                                id: "repeat-variable",
                                r#type: "checkbox",
                                class: "toggle toggle-primary mt-1",
                                checked: variable(),
                                onchange: move |e| variable.set(e.checked()),
                            }
                        }
                    }
                }
                div { class: "px-5 py-3 border-t border-base-200",
                    button {
                        id: "repeat-done",
                        r#type: "button",
                        class: "btn btn-primary w-full",
                        onclick: move |_| on_done.call(picked()),
                        {tid!("repeat-done")}
                    }
                }
            }
            div { class: "modal-backdrop", onclick: move |_| on_close.call(()) }
        }
    }
}

#[component]
fn RepeatOption(
    id: String,
    selected: bool,
    label: String,
    #[props(default)] hint: Option<String>,
    onselect: EventHandler<()>,
) -> Element {
    rsx! {
        li {
            button {
                id,
                r#type: "button",
                role: "radio",
                aria_checked: "{selected}",
                class: if selected { "flex items-center gap-3 w-full text-left px-2 py-1.5 rounded-field bg-base-200 font-semibold" } else { "flex items-center gap-3 w-full text-left px-2 py-1.5 rounded-field" },
                onclick: move |_| onselect.call(()),
                span {
                    class: if selected { "shrink-0 w-5 h-5 rounded-full border-2 border-primary flex items-center justify-center" } else { "shrink-0 w-5 h-5 rounded-full border-2 border-base-300" },
                    "aria-hidden": "true",
                    if selected {
                        span { class: "w-2.5 h-2.5 rounded-full bg-primary" }
                    }
                }
                span { class: "flex-1 min-w-0",
                    span { class: "block text-sm", "{label}" }
                    if let Some(h) = hint {
                        span { class: "block text-xs font-normal text-base-content/70", "{h}" }
                    }
                }
            }
        }
    }
}

#[component]
fn EndsTab(label: String, active: bool, onselect: EventHandler<()>) -> Element {
    rsx! {
        button {
            r#type: "button",
            role: "tab",
            aria_selected: "{active}",
            class: if active { "tab tab-active flex-1 text-sm font-semibold" } else { "tab flex-1 text-sm" },
            onclick: move |_| onselect.call(()),
            "{label}"
        }
    }
}
