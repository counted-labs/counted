//! The personal project's Budget tab (P2, R3) and the sheet that sets the ceilings (P3).

use dioxus::prelude::*;
use shared::{Budgets, EditableProject, ProjectPayload};
use std::collections::{BTreeMap, HashMap};
use uuid::Uuid;

use crate::categories::{category_label, parent_emoji, CHART_CATEGORIES};
use crate::common::{error_message, update_project_fresh, ProjectKey};
use crate::crypto::{decrypt_json, encrypt_padded_json};
use crate::expenses::helpers::budgets::{BudgetLine, Meter};
use crate::expenses::helpers::expense_form_helpers::parse_amount;
use crate::icons::{CloseIcon, WalletIcon, ICON_HEADER};
use crate::tid;

pub(crate) fn money(amount: f64, currency: &str) -> String {
    crate::common::amount(amount, currency)
}

pub(crate) fn ceiling(amount: f64) -> String {
    let plain = if amount.fract() == 0.0 { format!("{amount:.0}") } else { format!("{amount:.2}") };
    if crate::common::comma_language() { plain.replace('.', ",") } else { plain }
}

#[component]
pub fn BudgetPanel(lines: Vec<BudgetLine>, currency: String, on_manage: EventHandler<()>) -> Element {
    if lines.is_empty() {
        return rsx! {
            section { id: "budget-empty", class: "rounded-[20px] bg-base-100 shadow-soft p-4 flex flex-col items-center gap-2.5 text-center",
                span { class: "size-14 rounded-full bg-primary/10 text-primary flex items-center justify-center", WalletIcon { size: 26 } }
                span { class: "font-semibold", {tid!("budget-empty")} }
                span { class: "text-sm text-base-content/70 max-w-72", {tid!("budget-empty-hint")} }
                button {
                    id: "budget-define",
                    r#type: "button",
                    class: "btn btn-primary rounded-full mt-1.5",
                    onclick: move |_| on_manage.call(()),
                    {tid!("budget-define")}
                }
            }
        };
    }
    rsx! {
        section { id: "budget-panel", class: "rounded-[20px] bg-base-100 shadow-soft px-4 py-1.5 flex flex-col",
            div { class: "flex justify-between items-baseline pt-2",
                span { class: "font-semibold", {tid!("budget-title")} }
                button {
                    id: "budget-manage",
                    r#type: "button",
                    class: "link link-primary no-underline text-[13px] font-semibold",
                    onclick: move |_| on_manage.call(()),
                    {tid!("budget-manage")}
                }
            }
            ul { class: "flex flex-col divide-y divide-base-200",
                for line in lines {
                    {
                        let meter = line.meter();
                        let width = (line.ratio() * 100.0).clamp(0.0, 100.0);
                        let bar = match meter {
                            Meter::Left(_) => "bg-success",
                            Meter::Near(_) => "bg-warning",
                            Meter::Over(_) => "bg-error",
                        };
                        rsx! {
                            li { key: "{line.category}", class: "budget-line flex items-center gap-3 py-2.5",
                                span { class: "size-10 shrink-0 rounded-full flex items-center justify-center text-xl", style: "background: color-mix(in oklab, {crate::charts::category_color(&line.category)} 18%, white);", aria_hidden: "true",
                                    "{parent_emoji(line.category)}"
                                }
                                span { class: "flex-1 min-w-0 flex flex-col gap-1.5",
                                    span { class: "flex justify-between gap-2 text-sm",
                                        span { class: "font-medium truncate", "{category_label(line.category)}" }
                                        span { class: "tabular-nums whitespace-nowrap",
                                            span { aria_hidden: "true",
                                                strong { "{money(line.spent, &currency)}" }
                                                span { class: "text-base-content/70", " / {crate::common::round_amount(line.limit, &currency)}" }
                                            }
                                            span { class: "sr-only",
                                                {tid!("charts-of-total", amount: money(line.spent, &currency), total: crate::common::round_amount(line.limit, &currency))}
                                            }
                                        }
                                    }
                                    span { class: "block h-1.5 rounded-full bg-base-200 overflow-hidden",
                                        span { class: "block h-full rounded-full {bar}", style: "width: {width}%;" }
                                    }
                                    span { class: "budget-meter text-xs text-base-content/70",
                                        match meter {
                                            Meter::Left(left) => rsx! { {tid!("budget-left", amount: money(left, &currency))} },
                                            Meter::Near(pct) => rsx! { span { class: "text-warning font-semibold", "{pct} %" } },
                                            Meter::Over(by) => rsx! { span { class: "text-error font-semibold", {tid!("budget-over", amount: money(by, &currency))} } },
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// The ceilings as typed, read back into budgets. `Err` names the first category that is not an amount.
fn parsed(values: &BTreeMap<String, String>, alert: bool) -> Result<Budgets, &'static str> {
    let mut monthly = BTreeMap::new();
    for category in CHART_CATEGORIES {
        let raw = values.get(*category).map(|v| v.trim()).unwrap_or("");
        if raw.is_empty() {
            continue;
        }
        match parse_amount(raw).filter(|v| *v > 0.0) {
            Some(v) => {
                monthly.insert(category.to_string(), v);
            }
            None => return Err(category),
        }
    }
    Ok(Budgets { monthly, alert })
}

#[component]
pub fn BudgetSheet(
    project_id: Uuid,
    budgets: Budgets,
    /// Every chart category with its average of the three months before, most spent first.
    averages: Vec<(&'static str, f64)>,
    currency: String,
    on_close: EventHandler<()>,
    on_saved: EventHandler<()>,
) -> Element {
    crate::common::use_modal_focus("budget-sheet");
    let key_ctx = use_context::<ProjectKey>().0;
    let is_online = use_context::<Signal<bool>>();
    let initial = budgets.clone();
    let mut values = use_signal(move || {
        initial.monthly.iter().map(|(c, v)| (c.clone(), ceiling(*v))).collect::<BTreeMap<String, String>>()
    });
    let mut alert = use_signal(|| budgets.monthly.is_empty() || budgets.alert);
    let mut error_msg: Signal<Option<String>> = use_signal(|| None);
    let mut saving = use_signal(|| false);

    let on_save = move |_| {
        if !is_online() {
            error_msg.set(Some(tid!("budget-offline")));
            return;
        }
        let Some(key) = key_ctx() else { return };
        let next = match parsed(&values.read(), alert()) {
            Ok(b) => b,
            Err(category) => {
                error_msg.set(Some(tid!("budget-invalid", category: category_label(category))));
                return;
            }
        };
        saving.set(true);
        error_msg.set(None);
        spawn(async move {
            let edit = |stored: &shared::ProjectDto| {
                let mut payload: ProjectPayload = decrypt_json(&key, &stored.payload)?;
                payload.set_budgets(&next);
                Ok(EditableProject {
                    id: project_id,
                    payload: Some(encrypt_padded_json(&key, &payload)?),
                    status: None,
                    history: None,
                    expected_iv: None,
                    notify_closed: false,
                })
            };
            match update_project_fresh(project_id, edit).await {
                Ok(_) => on_saved.call(()),
                Err(e) => {
                    error_msg.set(Some(error_message(&e)));
                    saving.set(false);
                }
            }
        });
    };

    rsx! {
        div {
            id: "budget-sheet",
            class: "modal modal-open modal-bottom sm:modal-middle",
            role: "dialog",
            aria_modal: "true",
            aria_labelledby: "budget-sheet-title",
            tabindex: "-1",
            div { class: "modal-box max-w-md flex flex-col gap-4",
                div { class: "flex items-start gap-3",
                    div { class: "flex-1 flex flex-col gap-1",
                        h2 { id: "budget-sheet-title", class: "font-display font-bold text-xl", {tid!("budget-sheet-title")} }
                        p { class: "text-sm text-base-content/70", {tid!("budget-sheet-intro")} }
                    }
                    button {
                        r#type: "button",
                        class: "btn btn-circle size-11 min-h-11 border-0 bg-base-200 text-base-content/80",
                        aria_label: tid!("close"),
                        onclick: move |_| on_close.call(()),
                        CloseIcon { size: ICON_HEADER }
                    }
                }
                if let Some(err) = error_msg() {
                    div { role: "alert", class: "alert alert-error text-sm", "{err}" }
                }
                ul { class: "flex flex-col gap-3",
                    for (category , average) in averages {
                        {
                            let index = CHART_CATEGORIES.iter().position(|c| *c == category).unwrap_or(0);
                            let label = category_label(category);
                            let value = values.read().get(category).cloned().unwrap_or_default();
                            rsx! {
                                li { key: "{category}", class: "flex items-center gap-3",
                                    span { class: "size-10 shrink-0 rounded-full flex items-center justify-center text-xl", style: "background: color-mix(in oklab, {crate::charts::category_color(&category)} 18%, white);", aria_hidden: "true",
                                        "{parent_emoji(category)}"
                                    }
                                    span { class: "flex-1 min-w-0 flex flex-col",
                                        span { class: "font-medium truncate", "{label}" }
                                        span { class: "text-xs text-base-content/70", {tid!("budget-average", amount: format!("{average:.0} {currency}"))} }
                                    }
                                    label { class: "input w-28 h-11 rounded-xl gap-1",
                                        input {
                                            id: "budget-{index}",
                                            class: "grow min-w-0 text-right",
                                            r#type: "text",
                                            inputmode: "decimal",
                                            aria_label: tid!("budget-for", category: label.clone()),
                                            value: "{value}",
                                            oninput: move |e| {
                                                values.write().insert(category.to_string(), e.value());
                                            },
                                        }
                                        span { class: "text-base-content/60 text-sm", "{currency}" }
                                    }
                                }
                            }
                        }
                    }
                }
                label { class: "flex items-center gap-3 px-3.5 py-3 rounded-box bg-primary/5 text-sm cursor-pointer",
                    span { class: "flex-1",
                        {tid!("budget-alert")}
                        span { class: "block text-xs text-base-content/70", {tid!("budget-alert-hint")} }
                    }
                    input {
                        id: "budget-alert",
                        r#type: "checkbox",
                        role: "switch",
                        class: "toggle toggle-primary",
                        checked: alert(),
                        onchange: move |e| alert.set(e.checked()),
                    }
                }
                div { class: "flex gap-2.5",
                    button {
                        r#type: "button",
                        class: "btn flex-1 rounded-full",
                        onclick: move |_| on_close.call(()),
                        {tid!("cancel")}
                    }
                    button {
                        id: "budget-save",
                        r#type: "button",
                        class: "btn btn-primary flex-[2] rounded-full",
                        disabled: saving(),
                        onclick: on_save,
                        if saving() {
                            span { class: "loading loading-spinner loading-sm" }
                        }
                        {tid!("save")}
                    }
                }
            }
            div { class: "modal-backdrop", onclick: move |_| on_close.call(()) }
        }
    }
}

/// Every chart category, most spent first over the three months before, so the ones that matter
/// sit at the top of the sheet.
pub fn sheet_categories(averages: &HashMap<&'static str, f64>) -> Vec<(&'static str, f64)> {
    let mut all: Vec<(&'static str, f64)> =
        CHART_CATEGORIES.iter().map(|c| (*c, averages.get(c).copied().unwrap_or(0.0))).collect();
    all.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    all
}

#[cfg(test)]
mod tests {
    use super::*;

    fn typed(pairs: &[(&str, &str)]) -> BTreeMap<String, String> {
        pairs.iter().map(|(c, v)| (c.to_string(), v.to_string())).collect()
    }

    #[test]
    fn typed_ceilings_become_budgets_and_blanks_are_dropped() {
        let got = parsed(&typed(&[("Nourriture", "400"), ("Loisirs", "120,50"), ("Transport", "  ")]), true).unwrap();
        assert_eq!(got.monthly.get("Nourriture"), Some(&400.0));
        assert_eq!(got.monthly.get("Loisirs"), Some(&120.5));
        assert_eq!(got.monthly.len(), 2);
        assert!(got.alert);
    }

    #[test]
    fn a_non_amount_names_its_category() {
        assert_eq!(parsed(&typed(&[("Shopping", "beaucoup")]), false), Err("Shopping"));
        assert_eq!(parsed(&typed(&[("Shopping", "0")]), false), Err("Shopping"));
    }

    #[test]
    fn the_sheet_lists_every_category_most_spent_first() {
        let cats = sheet_categories(&HashMap::from([("Transport", 94.0), ("Nourriture", 365.0)]));
        assert_eq!(cats.len(), CHART_CATEGORIES.len());
        assert_eq!(&cats[..2], &[("Nourriture", 365.0), ("Transport", 94.0)]);
    }
}
