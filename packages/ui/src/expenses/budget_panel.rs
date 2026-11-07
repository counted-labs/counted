//! The personal project's Budget tab (P2, R3) and the sheet that sets the ceilings (P3).

use dioxus::prelude::*;
use shared::{Budgets, EditableProject, ProjectPayload};
use std::collections::{BTreeMap, HashMap};
use uuid::Uuid;

use crate::categories::{ProjectCategories, ShownCategory, CHART_CATEGORIES};
use crate::common::{error_message, update_project_fresh, whole_amount, ProjectKey};
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
                    class: "link link-primary no-underline text-[0.8125rem] font-semibold",
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
                            li { key: "{line.category.id}", class: "budget-line flex items-center gap-3 py-2.5",
                                span { class: "size-10 shrink-0 rounded-full flex items-center justify-center text-xl", style: "background: color-mix(in oklab, {line.category.color()} 18%, white);", aria_hidden: "true",
                                    "{line.category.emoji}"
                                }
                                span { class: "flex-1 min-w-0 flex flex-col gap-1.5",
                                    span { class: "flex justify-between gap-2 text-sm",
                                        span { class: "font-medium truncate", dir: "auto", {line.category.label()} }
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

/// The ceilings as typed for the categories the sheet shows, by id. `Err` names the first one that
/// is not an amount.
fn parsed(values: &BTreeMap<String, String>, shown: &[ShownCategory]) -> Result<BTreeMap<String, f64>, ShownCategory> {
    let mut typed = BTreeMap::new();
    for category in shown {
        let raw = values.get(&category.id).map(|v| v.trim()).unwrap_or("");
        if raw.is_empty() {
            continue;
        }
        match parse_amount(raw).filter(|v| *v > 0.0) {
            Some(v) => {
                typed.insert(category.id.clone(), v);
            }
            None => return Err(category.clone()),
        }
    }
    Ok(typed)
}

/// The stored payload with the ceilings of the shown categories replaced: a built-in's in
/// `budgets.monthly`, a custom one's in `budgetsCustom`. Every key the sheet did not show is kept.
fn with_ceilings(payload: &mut ProjectPayload, shown: &[ShownCategory], typed: &BTreeMap<String, f64>, alert: bool) {
    let mut monthly = payload.budgets().monthly;
    for category in shown.iter().filter(|c| !c.is_custom()) {
        monthly.remove(&category.id);
        if let Some(v) = typed.get(&category.id) {
            monthly.insert(category.id.clone(), *v);
        }
    }
    payload.set_budgets(&Budgets { monthly, alert });
    let custom: Vec<String> = shown.iter().filter(|c| c.is_custom()).map(|c| c.id.clone()).collect();
    payload.merge_budgets_custom(&custom, typed);
}

#[component]
pub fn BudgetSheet(
    project_id: Uuid,
    budgets: Budgets,
    custom: BTreeMap<String, f64>,
    /// Every shown category with its average of the three months before, most spent first.
    averages: Vec<(ShownCategory, f64)>,
    currency: String,
    on_close: EventHandler<()>,
    on_saved: EventHandler<()>,
) -> Element {
    crate::common::use_modal_focus("budget-sheet");
    let key_ctx = use_context::<ProjectKey>().0;
    let is_online = use_context::<Signal<bool>>();
    let initial = budgets.clone();
    let mut values = use_signal(move || {
        initial.monthly.iter().chain(custom.iter()).map(|(c, v)| (c.clone(), ceiling(*v))).collect::<BTreeMap<String, String>>()
    });
    let shown: Vec<ShownCategory> = averages.iter().map(|(c, _)| c.clone()).collect();
    let mut alert = use_signal(|| budgets.monthly.is_empty() || budgets.alert);
    let mut error_msg: Signal<Option<String>> = use_signal(|| None);
    let mut saving = use_signal(|| false);

    let on_save = move |_| {
        if !is_online() {
            error_msg.set(Some(tid!("budget-offline")));
            return;
        }
        let Some(key) = key_ctx() else { return };
        let typed = match parsed(&values.read(), &shown) {
            Ok(t) => t,
            Err(category) => {
                error_msg.set(Some(tid!("budget-invalid", category: category.label())));
                return;
            }
        };
        let shown = shown.clone();
        let alert = alert();
        saving.set(true);
        error_msg.set(None);
        spawn(async move {
            let edit = |stored: &shared::ProjectDto| {
                let mut payload: ProjectPayload = decrypt_json(&key, &stored.payload)?;
                with_ceilings(&mut payload, &shown, &typed, alert);
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
                            let input_id = match CHART_CATEGORIES.iter().position(|c| *c == category.id) {
                                Some(index) => format!("budget-{index}"),
                                None => format!("budget-{}", category.id),
                            };
                            let label = category.label();
                            let value = values.read().get(&category.id).cloned().unwrap_or_default();
                            let id = category.id.clone();
                            rsx! {
                                li { key: "{category.id}", class: "flex items-center gap-3",
                                    span { class: "size-10 shrink-0 rounded-full flex items-center justify-center text-xl", style: "background: color-mix(in oklab, {category.color()} 18%, white);", aria_hidden: "true",
                                        "{category.emoji}"
                                    }
                                    span { class: "flex-1 min-w-0 flex flex-col",
                                        span { class: "font-medium truncate", dir: "auto", "{label}" }
                                        span { class: "text-xs text-base-content/70", {tid!("budget-average", amount: whole_amount(average, &currency))} }
                                    }
                                    label { class: "input w-28 h-11 rounded-xl gap-1",
                                        input {
                                            id: "{input_id}",
                                            class: "grow min-w-0 text-right",
                                            r#type: "text",
                                            inputmode: "decimal",
                                            aria_label: tid!("budget-for", category: label.clone()),
                                            value: "{value}",
                                            oninput: move |e| {
                                                values.write().insert(id.clone(), e.value());
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

/// Every shown category, most spent first over the three months before, so the ones that matter
/// sit at the top of the sheet.
pub fn sheet_categories(averages: &HashMap<String, f64>, categories: &ProjectCategories) -> Vec<(ShownCategory, f64)> {
    let mut all: Vec<(ShownCategory, f64)> = categories
        .builtin_shown()
        .into_iter()
        .chain(categories.custom_shown())
        .map(|c| {
            let average = averages.get(&c.id).copied().unwrap_or(0.0);
            (c, average)
        })
        .collect();
    all.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    all
}

#[cfg(test)]
mod tests {
    use super::*;

    fn typed(pairs: &[(&str, &str)]) -> BTreeMap<String, String> {
        pairs.iter().map(|(c, v)| (c.to_string(), v.to_string())).collect()
    }

    fn builtins() -> Vec<ShownCategory> {
        ProjectCategories::default().builtin_shown()
    }

    #[test]
    fn typed_ceilings_become_budgets_and_blanks_are_dropped() {
        let got = parsed(&typed(&[("Nourriture", "400"), ("Loisirs", "120,50"), ("Transport", "  ")]), &builtins()).unwrap();
        assert_eq!(got.get("Nourriture"), Some(&400.0));
        assert_eq!(got.get("Loisirs"), Some(&120.5));
        assert_eq!(got.len(), 2);
    }

    #[test]
    fn a_non_amount_names_its_category() {
        let shopping = ShownCategory::builtin("Shopping");
        assert_eq!(parsed(&typed(&[("Shopping", "beaucoup")]), &builtins()), Err(shopping.clone()));
        assert_eq!(parsed(&typed(&[("Shopping", "0")]), &builtins()), Err(shopping));
    }

    #[test]
    fn the_sheet_lists_every_category_most_spent_first() {
        let cats = sheet_categories(&HashMap::from([("Transport".to_string(), 94.0), ("Nourriture".to_string(), 365.0)]), &ProjectCategories::default());
        assert_eq!(cats.len(), CHART_CATEGORIES.len());
        assert_eq!(&cats[..2], &[(ShownCategory::builtin("Nourriture"), 365.0), (ShownCategory::builtin("Transport"), 94.0)]);
    }

    #[test]
    fn saving_keeps_every_ceiling_the_sheet_did_not_show() {
        let list = serde_json::json!([{ "id": "c-0000000a", "name": "Bio", "parent": "Nourriture" }, { "id": "Loisirs", "hidden": true }]);
        let categories = ProjectCategories::from_value(Some(&list));
        let shown: Vec<ShownCategory> = sheet_categories(&HashMap::new(), &categories).into_iter().map(|(c, _)| c).collect();
        let mut payload: ProjectPayload = serde_json::from_value(serde_json::json!({
            "name": "M", "currency": "EUR",
            "budgets": { "monthly": { "Loisirs": 50.0, "Future": 7.0, "Transport": 80.0 }, "alert": false },
            "budgetsCustom": { "c-0000000a": 10.0, "c-000000ff": 20.0 },
        }))
        .unwrap();
        let typed = parsed(&typed(&[("Nourriture", "300"), ("c-0000000a", "40")]), &shown).unwrap();
        with_ceilings(&mut payload, &shown, &typed, true);
        let monthly = payload.budgets().monthly;
        assert_eq!(monthly.get("Nourriture"), Some(&300.0));
        assert_eq!((monthly.get("Loisirs"), monthly.get("Future"), monthly.get("Transport")), (Some(&50.0), Some(&7.0), None));
        assert_eq!(payload.budgets_custom(), BTreeMap::from([("c-0000000a".to_string(), 40.0), ("c-000000ff".to_string(), 20.0)]));
        assert!(payload.budgets().alert);
    }
}
