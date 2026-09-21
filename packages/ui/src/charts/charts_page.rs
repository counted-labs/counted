use api::expenses::expenses_controller::get_expenses_by_project_id;
use api::payments::payments_controller::get_payments_by_project_id;
use api::projects::projects_controller::get_projects_by_ids;
use api::users::users_controller::get_users_by_project_id;
use chrono::{Datelike, Local, NaiveDate};
use dioxus::fullstack::Json;
use dioxus::prelude::*;
use crate::tid;
use shared::{BatchProject, ExpenseType, ProjectPayload};
use std::collections::{HashMap, HashSet};
use uuid::Uuid;

use crate::categories::{category_label, CHART_CATEGORIES as CATEGORIES};
use crate::common::{
    format_date, format_date_str, month_abbrev, key_of, pending, project_key, read_from_ls, AppHeader, DropdownButton,
    DropdownItem, PullToRefresh,
};
use crate::crypto::{
    decrypt_expense, decrypt_json, decrypt_payment, decrypt_user, DecryptedExpense,
    DecryptedPayment, DecryptedUser,
};
use crate::expenses::helpers::export::csv_field;
use crate::expenses::tabs::expenses_tab::my_debt_by_expense;
use crate::route::Route;

/// Lives in `categories` so `expenses::helpers::project_data` can derive the row emoji without
/// importing a page module.
pub use crate::categories::get_expense_category;

pub fn aggregate_by_category(expenses: &[DecryptedExpense]) -> Vec<(&'static str, f64)> {
    let mut map: HashMap<&'static str, f64> = HashMap::new();
    for e in expenses {
        if e.expense_type == ExpenseType::Expense {
            *map.entry(get_expense_category(e)).or_default() += e.amount;
        }
    }
    let mut result: Vec<(&'static str, f64)> = map.into_iter().collect();
    result.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    result
}

pub fn aggregate_by_month(expenses: &[DecryptedExpense], n_months: usize) -> Vec<(String, f64)> {
    let mut map: HashMap<String, f64> = HashMap::new();
    for e in expenses {
        if e.expense_type != ExpenseType::Expense {
            continue;
        }
        let Ok(date) = NaiveDate::parse_from_str(&e.date, "%Y-%m-%d") else {
            continue;
        };
        let key = format!("{}-{:02}", date.year(), date.month());
        *map.entry(key).or_default() += e.amount;
    }
    let mut result: Vec<(String, f64)> = map.into_iter().collect();
    result.sort_by(|a, b| a.0.cmp(&b.0));
    if result.len() > n_months {
        result = result.into_iter().rev().take(n_months).rev().collect();
    }
    result
}

/// Expenses whose author has been removed are left out: there is no participant to attribute them
/// to, and inventing a bucket would imply one. `total_spend` is computed independently, so the
/// headline figures stay whole even when the per-author breakdown does not sum to them.
pub fn aggregate_by_author(expenses: &[DecryptedExpense]) -> Vec<(i32, f64)> {
    let mut map: HashMap<i32, f64> = HashMap::new();
    for e in expenses {
        if e.expense_type == ExpenseType::Expense {
            if let Some(author_id) = e.author_id {
                *map.entry(author_id).or_default() += e.amount;
            }
        }
    }
    let mut result: Vec<(i32, f64)> = map.into_iter().collect();
    result.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    result
}

pub fn normalize<K: Clone>(data: &[(K, f64)]) -> Vec<(K, f64)> {
    let max = data.iter().map(|(_, v)| *v).fold(f64::NEG_INFINITY, f64::max);
    if max <= 0.0 {
        return data.iter().map(|(k, _)| (k.clone(), 0.0)).collect();
    }
    data.iter().map(|(k, v)| (k.clone(), v / max)).collect()
}

fn donut_slice_path(start: f64, end: f64) -> String {
    use std::f64::consts::PI;
    const CX: f64 = 120.0;
    const CY: f64 = 120.0;
    const RO: f64 = 108.0;
    const RI: f64 = 62.0;
    let frac = end - start;
    let gap = if frac > 0.04 { 0.003 } else { 0.0 };
    let s = start + gap;
    let e = end - gap;
    if frac >= 0.999 {
        return format!(
            "M {:.2} {CY} A {RO} {RO} 0 1 1 {:.2} {CY} A {RO} {RO} 0 1 1 {:.2} {CY} Z \
             M {:.2} {CY} A {RI} {RI} 0 1 0 {:.2} {CY} A {RI} {RI} 0 1 0 {:.2} {CY} Z",
            CX - RO, CX + RO, CX - RO,
            CX - RI, CX + RI, CX - RI,
        );
    }
    let a0 = s * 2.0 * PI - PI / 2.0;
    let a1 = e * 2.0 * PI - PI / 2.0;
    let large = if (e - s) > 0.5 { 1 } else { 0 };
    let (x0o, y0o) = (CX + RO * a0.cos(), CY + RO * a0.sin());
    let (x1o, y1o) = (CX + RO * a1.cos(), CY + RO * a1.sin());
    let (x0i, y0i) = (CX + RI * a0.cos(), CY + RI * a0.sin());
    let (x1i, y1i) = (CX + RI * a1.cos(), CY + RI * a1.sin());
    format!(
        "M {x0o:.2} {y0o:.2} A {RO} {RO} 0 {large} 1 {x1o:.2} {y1o:.2} \
         L {x1i:.2} {y1i:.2} A {RI} {RI} 0 {large} 0 {x0i:.2} {y0i:.2} Z"
    )
}

/// `"2025-06"` → `(5, "25")`: the `month0` index and the two-digit year.
///
/// Split out from `short_month_label` because that one translates, and translating needs a Dioxus
/// runtime no unit test has. This half is where the parsing bugs would live.
fn split_ym(ym: &str) -> Option<(usize, &str)> {
    let (year, month) = ym.split_once('-')?;
    if year.len() < 2 {
        return None;
    }
    Some((month.parse::<usize>().unwrap_or(1).saturating_sub(1).min(11), &year[year.len() - 2..]))
}

fn short_month_label(ym: &str) -> String {
    let Some((month0, year_short)) = split_ym(ym) else {
        return ym.to_string();
    };
    format!("{} {}", month_abbrev(month0), year_short)
}

pub fn total_spend(expenses: &[DecryptedExpense]) -> f64 {
    expenses.iter().filter(|e| e.expense_type == ExpenseType::Expense).map(|e| e.amount).sum()
}

pub fn avg_per_person(expenses: &[DecryptedExpense]) -> f64 {
    let authors: HashSet<i32> = expenses
        .iter()
        .filter(|e| e.expense_type == ExpenseType::Expense)
        .filter_map(|e| e.author_id)
        .collect();
    if authors.is_empty() {
        return 0.0;
    }
    total_spend(expenses) / authors.len() as f64
}

/// No decimals, and "-0" renders as "0".
pub fn fmt_amount(value: f64) -> String {
    let s = format!("{value:.0}");
    if s == "-0" {
        "0".to_string()
    } else {
        s
    }
}

pub fn monthly_deltas(data: &[(String, f64)]) -> Vec<Option<f64>> {
    let mut result = Vec::with_capacity(data.len());
    result.push(None);
    for window in data.windows(2) {
        let prev = window[0].1;
        let curr = window[1].1;
        result.push(if prev > 0.0 { Some((curr - prev) / prev * 100.0) } else { None });
    }
    result
}

pub fn filter_by_date_range(
    expenses: &[DecryptedExpense],
    from: Option<NaiveDate>,
    to: Option<NaiveDate>,
) -> Vec<DecryptedExpense> {
    if from.is_none() && to.is_none() {
        return expenses.to_vec();
    }
    expenses
        .iter()
        .filter(|e| {
            let d = NaiveDate::parse_from_str(&e.date, "%Y-%m-%d").ok();
            d.map(|d| from.map(|f| d >= f).unwrap_or(true) && to.map(|t| d <= t).unwrap_or(true))
                .unwrap_or(true)
        })
        .cloned()
        .collect()
}

/// Falls back to `#id` for an author no longer in the project.
fn user_label(users: &[DecryptedUser], user_id: i32) -> String {
    users
        .iter()
        .find(|u| u.id == user_id)
        .map(|u| u.name.clone())
        .unwrap_or_else(|| format!("#{}", user_id))
}

pub fn export_csv_string(expenses: &[DecryptedExpense], users: &[DecryptedUser]) -> String {
    // Machine-readable header and type values, deliberately not translated: a CSV whose columns
    // rename themselves per UI language cannot be reimported or diffed against an older export.
    let mut out = "date,name,category,amount,author,type,description\n".to_string();
    for e in expenses {
        let author = e
            .author_id
            .and_then(|id| users.iter().find(|u| u.id == id))
            .map(|u| u.name.as_str())
            .unwrap_or("");
        let cat = get_expense_category(e);
        let etype = match e.expense_type {
            ExpenseType::Expense => "expense",
            ExpenseType::Transfer => "transfer",
            ExpenseType::Gain => "gain",
        };
        let desc = csv_field(e.description.as_deref().unwrap_or(""));
        out.push_str(&format!(
            "{},{},{},{:.2},{},{},{}\n",
            csv_field(&e.date),
            csv_field(&e.name),
            cat,
            e.amount,
            csv_field(author),
            etype,
            desc
        ));
    }
    out
}

pub fn aggregate_net_balance(
    expenses: &[DecryptedExpense],
    payments: &[DecryptedPayment],
) -> Vec<(i32, f64)> {
    let expense_ids: HashSet<i32> = expenses.iter().map(|e| e.id).collect();
    let mut balances: HashMap<i32, f64> = HashMap::new();
    for p in payments {
        if !expense_ids.contains(&p.expense_id) {
            continue;
        }
        let delta = if !p.is_debt { p.amount } else { -p.amount };
        *balances.entry(p.user_id).or_default() += delta;
    }
    let mut result: Vec<(i32, f64)> = balances.into_iter().collect();
    result.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    result
}

/// Each expense restated as my share of it, so every aggregate below works unchanged on the
/// cross-project view. Expenses I owe nothing on are dropped rather than zeroed:
/// `charts-expense-count` counts what it is given, and a zero slice would still list itself in the
/// drilldown table.
///
/// `id` and `project_id` survive the rewrite — the drilldown links to `PaymentPage` with both.
pub fn with_my_share(
    expenses: &[DecryptedExpense],
    share: &HashMap<i32, f64>,
) -> Vec<DecryptedExpense> {
    expenses
        .iter()
        .filter(|e| e.expense_type == ExpenseType::Expense)
        .filter_map(|e| match share.get(&e.id) {
            Some(amount) if *amount > 0.0 => Some(DecryptedExpense { amount: *amount, ..e.clone() }),
            _ => None,
        })
        .collect()
}

const PIE_COLORS: [&str; 8] = [
    "oklch(62% 0.15 162)",
    "oklch(68% 0.14 230)",
    "oklch(72% 0.16 80)",
    "oklch(64% 0.18 25)",
    "oklch(70% 0.15 300)",
    "oklch(72% 0.13 190)",
    "oklch(68% 0.13 140)",
    "oklch(65% 0.12 45)",
];

#[derive(PartialEq, Clone)]
enum Tab {
    Categories,
    ByPerson,
    Trends,
}

/// As (value, translation key). Labels must stay short so five equal segments fit at 360px —
/// keep that in mind when translating `period-*`.
const PERIODS: [(&str, &str); 5] = [
    ("all", "period-all"),
    ("month", "period-month"),
    ("3months", "period-3months"),
    ("year", "period-year"),
    ("custom", "period-custom"),
];

/// `custom` alone leaves the dates alone — it means "whatever the two fields say", and a
/// hand-edited date field sets it itself.
fn apply_period(
    value: &'static str,
    mut date_from: Signal<Option<NaiveDate>>,
    mut date_to: Signal<Option<NaiveDate>>,
    mut preset: Signal<&'static str>,
    earliest: Option<NaiveDate>,
) {
    let now = Local::now().date_naive();
    match value {
        "all" => {
            date_from.set(earliest);
            date_to.set(Some(now));
        }
        "month" => {
            date_from.set(NaiveDate::from_ymd_opt(now.year(), now.month(), 1));
            date_to.set(Some(now));
        }
        "3months" => {
            date_from.set(Some(now - chrono::Duration::days(90)));
            date_to.set(Some(now));
        }
        "year" => {
            date_from.set(NaiveDate::from_ymd_opt(now.year(), 1, 1));
            date_to.set(Some(now));
        }
        _ => {}
    }
    preset.set(value);
}

#[component]
pub fn ChartsPage() -> Element {
    let mut active_tab = use_signal(|| Tab::Categories);
    let mut selected_project: Signal<Option<Uuid>> = use_signal(|| None);
    let mut date_from: Signal<Option<NaiveDate>> = use_signal(|| None);
    let mut date_to: Signal<Option<NaiveDate>> = use_signal(|| None);
    let mut preset: Signal<&'static str> = use_signal(|| "all");

    let project_names = use_resource(move || async move {
        let ls = read_from_ls();
        let ids: Vec<Uuid> = ls.projects.iter().map(|p| p.project_id).collect();
        if ids.is_empty() {
            return vec![];
        }
        let Ok(encrypted) = get_projects_by_ids(Json(BatchProject { ids })).await else {
            return vec![];
        };
        let mut result: Vec<(Uuid, String)> = vec![];
        for p in encrypted {
            let Some(key) = ls.projects.iter().find(|lp| lp.project_id == p.id).and_then(key_of)
            else {
                continue;
            };
            if let Ok(payload) = decrypt_json::<ProjectPayload>(&key, &p.payload) {
                result.push((p.id, payload.name));
            }
        }
        result
    });

    let decrypted_expenses = use_resource(move || async move {
        let project_filter = selected_project();
        let ls = read_from_ls();
        let mut all: Vec<DecryptedExpense> = vec![];
        for p in &ls.projects {
            if let Some(pid) = project_filter {
                if p.project_id != pid {
                    continue;
                }
            }
            let Some(key) = key_of(p) else {
                continue;
            };
            let Ok(expenses) = get_expenses_by_project_id(p.project_id).await else {
                continue;
            };
            for e in expenses {
                if let Ok(d) = decrypt_expense(&key, &e) {
                    all.push(d);
                }
            }
        }
        all
    });

    let decrypted_users = use_resource(move || async move {
        let Some(pid) = selected_project() else {
            return vec![];
        };
        let Some(key) = project_key(pid) else {
            return vec![];
        };
        let Ok(users) = get_users_by_project_id(pid).await else {
            return vec![];
        };
        users.into_iter().filter_map(|u| decrypt_user(&key, &u).ok()).collect()
    });

    // Same loop as `decrypted_expenses`, and it also builds what I owe per expense. The share is
    // folded per project, where that project's participant id is in hand: one flat set of ids
    // would work only because `users.id` is a global SERIAL, and nothing should lean on that.
    // `expenses.id` being global is a schema fact, so the map itself needs no project qualifier.
    let decrypted_payments = use_resource(move || async move {
        let project_filter = selected_project();
        let ls = read_from_ls();
        let mut all: Vec<DecryptedPayment> = vec![];
        let mut my_share: HashMap<i32, f64> = HashMap::new();
        let mut left_out = 0usize;
        for p in &ls.projects {
            if let Some(pid) = project_filter {
                if p.project_id != pid {
                    continue;
                }
            }
            let Some(key) = key_of(p) else {
                continue;
            };
            // Counted, not skipped: the expenses are already loaded, so a silent `continue` would
            // leave them with a share of zero and under-report the headline.
            let Ok(payments) = get_payments_by_project_id(p.project_id).await else {
                left_out += 1;
                continue;
            };
            let decrypted: Vec<DecryptedPayment> =
                payments.into_iter().filter_map(|p| decrypt_payment(&key, &p).ok()).collect();
            match p.user_id {
                Some(user_id) => my_share.extend(my_debt_by_expense(&decrypted, user_id)),
                None => left_out += 1,
            }
            all.extend(decrypted);
        }
        (all, my_share, left_out)
    });

    // Reset to all-time on every expense reload — a project change or the first load.
    use_effect(move || {
        let raw = decrypted_expenses.read();
        let Some(expenses) = raw.as_ref() else {
            return;
        };
        let earliest = expenses
            .iter()
            .filter_map(|e| NaiveDate::parse_from_str(&e.date, "%Y-%m-%d").ok())
            .min();
        date_from.set(earliest);
        date_to.set(Some(Local::now().date_naive()));
        preset.set("all");
    });

    let today = Local::now().date_naive();

    let on_export = move |_: ()| {
        let raw = decrypted_expenses.read();
        let Some(expenses) = raw.as_ref() else {
            return;
        };
        let users = decrypted_users.read().as_ref().cloned().unwrap_or_default();
        let filtered = filter_by_date_range(expenses, date_from(), date_to());
        let csv = export_csv_string(&filtered, &users);
        crate::expenses::helpers::export::trigger_download(
            &csv,
            "depenses.csv",
            "text/csv;charset=utf-8",
        );
    };

    rsx! {
        div { class: "container app-container bg-base-100 overflow-auto p-4 pb-24 max-w-md w-full mx-auto flex flex-col gap-4",
            PullToRefresh {
                on_refresh: move |_| {
                    let (mut names, mut expenses, mut users, mut payments) =
                        (project_names, decrypted_expenses, decrypted_users, decrypted_payments);
                    names.restart();
                    expenses.restart();
                    users.restart();
                    payments.restart();
                },
                busy: pending(&project_names)
                    || pending(&decrypted_expenses)
                    || pending(&decrypted_users)
                    || pending(&decrypted_payments),
            }
            AppHeader {
                title: tid!("nav-charts"),
                back_button_route: Route::ProjectsPage {},
                DropdownButton {
label: tid!("export"),
                    DropdownItem {
                        variant: "ghost",
                        label: tid!("export-csv"),
                        onclick: on_export,
                    }
                }
            }

            // One raised surface, flat controls inside: a `shadow-soft` on each control reads as
            // five unrelated things floating.
            {
                let raw = decrypted_expenses.read();
                let all_expenses = raw.as_ref().map(|v| v.as_slice()).unwrap_or(&[]);
                let earliest = all_expenses
                    .iter()
                    .filter_map(|e| NaiveDate::parse_from_str(&e.date, "%Y-%m-%d").ok())
                    .min();
                let from_val = date_from().map(|d| d.to_string()).unwrap_or_default();
                let to_val = date_to().map(|d| d.to_string()).unwrap_or_default();
                let min_date = earliest.map(|d| d.to_string()).unwrap_or_default();
                let today_str = today.to_string();
                rsx! {
                    div { class: "rounded-[var(--radius-box)] bg-base-100 shadow-soft p-3 flex flex-col gap-3",
                        ProjectSelector {
                            projects: project_names.read().as_ref().cloned().unwrap_or_default(),
                            selected: selected_project(),
                            on_change: move |id: Option<Uuid>| selected_project.set(id),
                        }

                        // `button`, not `input type=radio`: main.css's unlayered
                        // `input { font-size: 16px !important }` outranks `text-sm`, and five
                        // segments at 16px overflow 360px. `btn-primary` comes from the signal, not
                        // DaisyUI's `:checked` — under `--depth: 0` that shift is barely visible.
                        div { role: "radiogroup", aria_label: tid!("charts-period"), class: "join w-full",
                            for (value , label) in PERIODS {
                                button {
                                    r#type: "button",
                                    role: "radio",
                                    aria_checked: *preset.read() == value,
                                    class: if *preset.read() == value { "join-item btn btn-primary flex-1 min-h-11 px-1 text-sm whitespace-nowrap" } else { "join-item btn flex-1 min-h-11 px-1 text-sm whitespace-nowrap" },
                                    onclick: move |_| apply_period(value, date_from, date_to, preset, earliest),
                                    {tid!(label)}
                                }
                            }
                        }

                        if *preset.read() == "custom" {
                            div { class: "flex gap-2 items-end",
                                fieldset { class: "fieldset flex-1 min-w-0",
                                    label { class: "fieldset-legend text-xs", r#for: "charts-date-from", {tid!("charts-date-from")} }
                                    input {
                                        id: "charts-date-from",
                                        class: "input bg-base-200 border-0 w-full min-h-11",
                                        r#type: "date",
                                        value: "{from_val}",
                                        min: "{min_date}",
                                        max: if to_val.is_empty() { today_str.clone() } else { to_val.clone() },
                                        oninput: move |e| {
                                            let v = e.value();
                                            let d = NaiveDate::parse_from_str(&v, "%Y-%m-%d").ok();
                                            date_from.set(d);
                                            if let (Some(f), Some(t)) = (d, date_to()) {
                                                if f > t {
                                                    date_to.set(None);
                                                }
                                            }
                                            preset.set("custom");
                                        },
                                    }
                                }
                                fieldset { class: "fieldset flex-1 min-w-0",
                                    label { class: "fieldset-legend text-xs", r#for: "charts-date-to", {tid!("charts-date-to")} }
                                    input {
                                        id: "charts-date-to",
                                        class: "input bg-base-200 border-0 w-full min-h-11",
                                        r#type: "date",
                                        value: "{to_val}",
                                        min: if from_val.is_empty() { min_date.clone() } else { from_val.clone() },
                                        max: "{today_str}",
                                        oninput: move |e| {
                                            let v = e.value();
                                            date_to.set(NaiveDate::parse_from_str(&v, "%Y-%m-%d").ok());
                                            preset.set("custom");
                                        },
                                    }
                                }
                            }
                        } else if let (Some(f), Some(t)) = (date_from(), date_to()) {
                            // The segments name a period; this names the range it resolved to.
                            p { class: "text-xs text-base-content/70",
                                "{format_date(f)} → {format_date(t)}"
                            }
                        }
                    }
                }
            }

            // All-projects mode reads the payments, and a pending resource unwraps to an empty
            // vec — without this guard the page would paint a 0 headline and an empty donut before
            // filling in. Single-project mode does not depend on them, so it is not made to wait.
            match &*decrypted_expenses.read() {
                Some(expenses) if !(selected_project().is_none() && decrypted_payments.read().is_none()) => {
                    let filtered = filter_by_date_range(expenses, date_from(), date_to());
                    let users = decrypted_users.read().as_ref().cloned().unwrap_or_default();
                    let (payments, my_share, left_out) =
                        decrypted_payments.read().as_ref().cloned().unwrap_or_default();

                    let personal = selected_project().is_none();
                    let group_total = total_spend(&filtered);
                    let charts_expenses =
                        if personal { with_my_share(&filtered, &my_share) } else { filtered.clone() };

                    rsx! {
                        StatsCards {
                            expenses: charts_expenses.clone(),
                            personal,
                            group_total,
                        }

                        if personal {
                            div { class: "flex flex-col gap-0.5 px-1",
                                p { class: "text-xs text-base-content/70", {tid!("charts-my-share-note")} }
                                if left_out > 0 {
                                    p { class: "text-xs text-base-content/70",
                                        {tid!("charts-my-share-skipped", count: left_out as i64)}
                                    }
                                }
                            }
                        }

                        // One panel element serves all three tabs, so each points `aria-controls`
                        // at it and the panel names the active tab back.
                        div { role: "tablist", class: "tabs tabs-box shadow-soft",
                            button {
                                id: "charts-tab-categories",
                                role: "tab",
                                aria_selected: active_tab() == Tab::Categories,
                                aria_controls: "charts-panel",
                                class: if active_tab() == Tab::Categories { "tab tab-active flex-1 min-h-11 text-sm font-semibold whitespace-nowrap" } else { "tab flex-1 min-h-11 text-sm text-base-content/70 whitespace-nowrap" },
                                onclick: move |_| active_tab.set(Tab::Categories),
                                {tid!("charts-tab-categories")}
                            }
                            button {
                                id: "charts-tab-by-person",
                                role: "tab",
                                aria_selected: active_tab() == Tab::ByPerson,
                                aria_controls: "charts-panel",
                                class: if active_tab() == Tab::ByPerson { "tab tab-active flex-1 min-h-11 text-sm font-semibold whitespace-nowrap" } else { "tab flex-1 min-h-11 text-sm text-base-content/70 whitespace-nowrap" },
                                onclick: move |_| active_tab.set(Tab::ByPerson),
                                {tid!("charts-tab-per-person")}
                            }
                            button {
                                id: "charts-tab-trends",
                                role: "tab",
                                aria_selected: active_tab() == Tab::Trends,
                                aria_controls: "charts-panel",
                                class: if active_tab() == Tab::Trends { "tab tab-active flex-1 min-h-11 text-sm font-semibold whitespace-nowrap" } else { "tab flex-1 min-h-11 text-sm text-base-content/70 whitespace-nowrap" },
                                onclick: move |_| active_tab.set(Tab::Trends),
                                {tid!("charts-tab-trends")}
                            }
                        }

                        div {
                            id: "charts-panel",
                            role: "tabpanel",
                            class: "flex flex-col gap-4",
                            aria_labelledby: match active_tab() {
                                Tab::Categories => "charts-tab-categories",
                                Tab::ByPerson => "charts-tab-by-person",
                                Tab::Trends => "charts-tab-trends",
                            },
                            match active_tab() {
                            Tab::Categories => rsx! {
                                div { class: "card bg-base-100 shadow-soft",
                                    div { class: "card-body p-4",
                                        h2 { class: "card-title text-sm justify-center pb-2", {tid!("charts-by-category")} }
                                        CategoryChart { expenses: charts_expenses, users: users.clone() }
                                    }
                                }
                            },
                            Tab::ByPerson => rsx! {
                                div { class: "card bg-base-100 shadow-soft",
                                    div { class: "card-body p-4",
                                        h2 { class: "card-title text-sm justify-center pb-2", {tid!("charts-per-person")} }
                                        ByPersonChart {
                                            expenses: filtered,
                                            users,
                                            has_project: selected_project().is_some(),
                                        }
                                    }
                                }
                            },
                            Tab::Trends => rsx! {
                                div { class: "card bg-base-100 shadow-soft",
                                    div { class: "card-body p-4",
                                        h2 { class: "card-title text-sm justify-center pb-2", {tid!("charts-categories-by-month")} }
                                        CategoryTrendChart { expenses: charts_expenses }
                                    }
                                }
                                div { class: "card bg-base-100 shadow-soft",
                                    div { class: "card-body p-4",
                                        h2 { class: "card-title text-sm justify-center pb-2", {tid!("charts-payments-per-person-by-month")} }
                                        PersonTrendChart {
                                            expenses: filtered,
                                            payments,
                                            users,
                                            has_project: selected_project().is_some(),
                                        }
                                    }
                                }
                            },
                            }
                        }
                    }
                }
                _ => rsx! {
                    div { class: "flex justify-center py-8",
                        span { class: "loading loading-spinner loading-md", role: "status", aria_label: tid!("loading") }
                    }
                },
            }
        }
    }
}

#[derive(PartialEq, Props, Clone)]
struct StatsCardsProps {
    expenses: Vec<DecryptedExpense>,
    /// All-projects mode: `expenses` are my share, so the tiles name that and the second one shows
    /// what the group spent instead of an average over authors, which no longer means anything.
    personal: bool,
    group_total: f64,
}

#[component]
fn StatsCards(props: StatsCardsProps) -> Element {
    let total = fmt_amount(total_spend(&props.expenses));
    let count = props.expenses.iter().filter(|e| e.expense_type == ExpenseType::Expense).count();
    // Both keys spelled out rather than picked from a variable: the i18n guard only scans literal
    // `tid!` keys, and a table here would need its own assertion test to stay safe.
    let second_value = if props.personal {
        fmt_amount(props.group_total)
    } else {
        fmt_amount(avg_per_person(&props.expenses))
    };
    rsx! {
        div { class: "flex shadow-soft rounded-[var(--radius-box)] bg-base-100 overflow-visible divide-x divide-base-200",
            div { class: "flex-1 py-3 px-4 flex flex-col gap-0.5",
                div { class: "text-xs text-base-content/70",
                    if props.personal { {tid!("stats-my-expenses")} } else { {tid!("charts-total-spent")} }
                }
                div { class: "text-2xl font-extrabold font-display text-gradient-brand leading-tight", "{total}" }
                div { class: "text-xs text-base-content/70", {tid!("charts-expense-count", count: count as i64)} }
            }
            div { class: "flex-1 py-3 px-4 flex flex-col gap-0.5",
                div { class: "text-xs text-base-content/70",
                    if props.personal { {tid!("stats-total-expenses")} } else { {tid!("charts-avg-per-person")} }
                }
                div { class: "text-2xl font-extrabold font-display text-primary leading-tight", "{second_value}" }
            }
        }
    }
}

#[derive(PartialEq, Props, Clone)]
struct ProjectSelectorProps {
    projects: Vec<(Uuid, String)>,
    selected: Option<Uuid>,
    on_change: EventHandler<Option<Uuid>>,
}

#[component]
fn ProjectSelector(props: ProjectSelectorProps) -> Element {
    rsx! {
        label { class: "select bg-base-200 w-full border-0 min-h-11",
            span { class: "label", {tid!("charts-project")} }
            select {
                id: "charts-project",
                onchange: move |e| {
                    let val = e.value();
                    if val == "all" {
                        props.on_change.call(None);
                    } else if let Ok(id) = val.parse::<Uuid>() {
                        props.on_change.call(Some(id));
                    }
                },
                option { value: "all", selected: props.selected.is_none(), {tid!("charts-all-projects")} }
                for (id , name) in &props.projects {
                    option { value: "{id}", selected: props.selected == Some(*id), "{name}" }
                }
            }
        }
    }
}

#[derive(PartialEq, Props, Clone)]
struct LegendChipProps {
    color: String,
    label: String,
    /// Reduced-emphasis suffix — the donut passes `(20% · 7321)`, the trend legends have no room.
    detail: Option<String>,
    selected: bool,
    onclick: EventHandler<()>,
}

/// The one legend toggle — donut and both trend charts share it rather than styling their own.
#[component]
fn LegendChip(props: LegendChipProps) -> Element {
    rsx! {
        button {
            r#type: "button",
            aria_pressed: props.selected,
            class: if props.selected { "flex items-center gap-1 text-xs font-semibold rounded-field bg-base-200 px-1.5 py-1" } else { "flex items-center gap-1 text-xs text-base-content/70 rounded-field px-1.5 py-1 hover:bg-base-200" },
            onclick: move |_| props.onclick.call(()),
            span {
                class: "w-2.5 h-2.5 rounded-sm inline-block shrink-0",
                style: "background: {props.color};",
            }
            "{props.label}"
            if let Some(detail) = props.detail.clone() {
                span { class: "text-base-content/70 font-normal", "{detail}" }
            }
        }
    }
}

#[derive(PartialEq, Props, Clone)]
struct CategoryChartProps {
    expenses: Vec<DecryptedExpense>,
    users: Vec<DecryptedUser>,
}

#[component]
fn CategoryChart(props: CategoryChartProps) -> Element {
    let mut selected_cat: Signal<Option<&'static str>> = use_signal(|| None);
    let mut selected_user: Signal<Option<i32>> = use_signal(|| None);

    let chart_expenses: Vec<DecryptedExpense> = match selected_user() {
        None => props.expenses.clone(),
        Some(uid) => {
            props.expenses.iter().filter(|e| e.author_id == Some(uid)).cloned().collect()
        }
    };

    let data = aggregate_by_category(&chart_expenses);
    if data.is_empty() {
        return rsx! {
            EmptyState {}
        };
    }
    let total: f64 = data.iter().map(|(_, v)| *v).sum();
    if total <= 0.0 {
        return rsx! {
            EmptyState {}
        };
    }
    let mut cum = 0.0f64;
    let slices: Vec<(&'static str, f64, f64, f64, f64, &'static str)> = data
        .iter()
        .enumerate()
        .map(|(i, (label, amount))| {
            let frac = amount / total;
            let start = cum;
            cum += frac;
            (*label, *amount, frac * 100.0, start, cum, PIE_COLORS[i % PIE_COLORS.len()])
        })
        .collect();

    let drilldown_expenses: Vec<DecryptedExpense> = selected_cat()
        .map(|cat| {
            chart_expenses.iter().filter(|e| get_expense_category(e) == cat).cloned().collect()
        })
        .unwrap_or_default();

    rsx! {
        div { class: "flex flex-col items-center gap-4",
            if !props.users.is_empty() {
                label { class: "select bg-base-200 border-0 w-full min-h-11",
                span { class: "label", {tid!("charts-person")} }
                select {
                    id: "charts-person",
                    onchange: move |e| {
                        let val = e.value();
                        if val == "all" {
                            selected_user.set(None);
                        } else if let Ok(id) = val.parse::<i32>() {
                            selected_user.set(Some(id));
                        }
                        selected_cat.set(None);
                    },
                    option { value: "all", selected: selected_user().is_none(), {tid!("charts-whole-project")} }
                    for user in &props.users {
                        option {
                            value: "{user.id}",
                            selected: selected_user() == Some(user.id),
                            "{user.name}"
                        }
                    }
                }
                }
            }
            svg {
                view_box: "0 0 240 240",
                width: "240",
                height: "240",
                for (label , _amount , _percent , start , end , color) in &slices {
                    {
                        let is_selected = selected_cat() == Some(*label);
                        let has_selection = selected_cat().is_some();
                        let opacity = if is_selected { "1" } else if has_selection { "0.3" } else { "1" };
                        let label = *label;
                        rsx! {
                            path {
                                d: donut_slice_path(*start, *end),
                                fill: "{color}",
                                fill_rule: "evenodd",
                                opacity: "{opacity}",
                                style: "cursor: pointer; transition: opacity 0.15s;",
                                onclick: move |_| {
                                    if selected_cat() == Some(label) {
                                        selected_cat.set(None);
                                    } else {
                                        selected_cat.set(Some(label));
                                    }
                                },
                            }
                        }
                    }
                }
                if let Some(cat) = selected_cat() {
                    if let Some((_,  amount, percent, ..)) = slices.iter().find(|(l, ..)| *l == cat) {
                        text {
                            x: "120", y: "110",
                            style: "text-anchor: middle; dominant-baseline: middle; font-size: 10px; fill: var(--color-base-content); opacity: 0.55;",
                            {category_label(cat)}
                        }
                        text {
                            x: "120", y: "128",
                            style: "text-anchor: middle; dominant-baseline: middle; font-size: 18px; font-weight: 700; fill: var(--color-primary);",
                            "{percent:.0}%"
                        }
                        text {
                            x: "120", y: "146",
                            style: "text-anchor: middle; dominant-baseline: middle; font-size: 10px; fill: var(--color-base-content); opacity: 0.4;",
                            "{amount:.0}"
                        }
                    }
                } else {
                    text {
                        x: "120", y: "112",
                        style: "text-anchor: middle; dominant-baseline: middle; font-size: 10px; fill: var(--color-base-content); opacity: 0.5;",
                        {tid!("charts-total")}
                    }
                    text {
                        x: "120", y: "132",
                        style: "text-anchor: middle; dominant-baseline: middle; font-size: 20px; font-weight: 700; fill: var(--color-primary);",
                        "{total:.0}"
                    }
                }
            }
            div { class: "flex flex-wrap gap-1 justify-center",
                for (label , amount , percent , _ , _ , color) in &slices {
                    LegendChip {
                        color: color.to_string(),
                        label: label.to_string(),
                        detail: Some(format!("({percent:.0}% · {amount:.0})")),
                        selected: selected_cat() == Some(label),
                        onclick: {
                            let label = *label;
                            move |_| {
                                if selected_cat() == Some(label) {
                                    selected_cat.set(None);
                                } else {
                                    selected_cat.set(Some(label));
                                }
                            }
                        },
                    }
                }
            }

            if let Some(cat) = selected_cat() {
                div { class: "w-full",
                    div { class: "flex items-center justify-between mb-2",
                        span { class: "text-xs font-medium", {category_label(cat)} }
                        button {
                            class: "btn btn-xs btn-ghost",
                            aria_label: tid!("charts-clear-category-filter"),
                            onclick: move |_| selected_cat.set(None),
                            "✕"
                        }
                    }
                    if drilldown_expenses.is_empty() {
                        p { class: "text-xs text-base-content/70", {tid!("charts-no-expenses")} }
                    } else {
                        div { class: "overflow-x-auto",
                            table { class: "table table-xs w-full",
                                thead {
                                    tr {
                                        th { scope: "col", {tid!("field-date")} }
                                        th { scope: "col", {tid!("field-name")} }
                                        th { scope: "col", class: "text-right", {tid!("field-amount")} }
                                    }
                                }
                                tbody {
                                    for e in &drilldown_expenses {
                                        tr {
                                            td { "{format_date_str(&e.date)}" }
                                            td {
                                                Link {
                                                    class: "link link-hover",
                                                    to: Route::PaymentPage {
                                                        project_id: e.project_id,
                                                        expense_id: e.id,
                                                    },
                                                    "{e.name}"
                                                }
                                            }
                                            td { class: "text-right", "{e.amount:.2}" }
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

#[derive(PartialEq, Props, Clone)]
struct ByPersonChartProps {
    expenses: Vec<DecryptedExpense>,
    users: Vec<DecryptedUser>,
    has_project: bool,
}

#[component]
fn ByPersonChart(props: ByPersonChartProps) -> Element {
    if !props.has_project {
        return rsx! {
            div { class: "alert alert-info text-sm",
                {tid!("charts-pick-a-project")}
            }
        };
    }
    let data = aggregate_by_author(&props.expenses);
    if data.is_empty() {
        return rsx! {
            EmptyState {}
        };
    }
    let max = data.iter().map(|(_, v)| *v).fold(0.0_f64, f64::max);
    let users = props.users.clone();

    rsx! {
        div { class: "flex flex-col gap-1 w-full",
            div { class: "flex gap-1 items-end overflow-x-auto px-1", style: "height: 140px;",
                for (_author_id , amount) in &data {
                    {
                        let bar_h = if max > 0.0 { (amount / max * 112.0).max(2.0) } else { 2.0 };
                        rsx! {
                            div { class: "flex flex-col items-center justify-end gap-0.5 flex-1 min-w-7",
                                span { class: "text-[10px] leading-none text-base-content/70", "{amount:.0}€" }
                                div {
                                    class: "w-full rounded-t-lg bg-primary/30",
                                    style: "height: {bar_h:.0}px;",
                                }
                            }
                        }
                    }
                }
            }
            div { class: "flex gap-1 overflow-x-auto px-1",
                for (author_id , _) in &data {
                    {
                        let name = user_label(&users, *author_id);
                        rsx! {
                            span { class: "flex-1 min-w-7 text-center text-[11px] text-base-content/70 truncate",
                                "{name}"
                            }
                        }
                    }
                }
            }
        }
    }
}

const PERSON_COLORS: [&str; 7] = [
    "var(--color-primary)",
    "var(--color-secondary)",
    "var(--color-accent)",
    "var(--color-info)",
    "var(--color-success)",
    "var(--color-warning)",
    "var(--color-error)",
];

fn aggregate_category_by_month(
    expenses: &[DecryptedExpense],
    n: usize,
) -> Vec<(String, Vec<(&'static str, f64)>)> {
    aggregate_by_month(expenses, n)
        .into_iter()
        .map(|(month_key, _)| {
            let mut cat_map: HashMap<&'static str, f64> = HashMap::new();
            for e in expenses.iter().filter(|e| {
                e.expense_type == ExpenseType::Expense && e.date.starts_with(&month_key)
            }) {
                *cat_map.entry(get_expense_category(e)).or_default() += e.amount;
            }
            let cats = CATEGORIES
                .iter()
                .filter_map(|&c| cat_map.get(c).copied().filter(|&v| v > 0.0).map(|v| (c, v)))
                .collect();
            (month_key, cats)
        })
        .collect()
}

fn aggregate_person_by_month(
    expenses: &[DecryptedExpense],
    payments: &[DecryptedPayment],
    n: usize,
) -> (Vec<String>, Vec<i32>, Vec<Vec<f64>>) {
    let month_keys: Vec<String> =
        aggregate_by_month(expenses, n).into_iter().map(|(k, _)| k).collect();
    let expense_month: HashMap<i32, &str> = expenses
        .iter()
        .filter(|e| e.expense_type == ExpenseType::Expense)
        .map(|e| (e.id, e.date[..7].as_ref()))
        .collect();
    let mut payer_ids: Vec<i32> = payments
        .iter()
        .filter(|p| !p.is_debt && expense_month.contains_key(&p.expense_id))
        .map(|p| p.user_id)
        .collect::<HashSet<_>>()
        .into_iter()
        .collect();
    payer_ids.sort();
    let amounts: Vec<Vec<f64>> = month_keys
        .iter()
        .map(|month_key| {
            payer_ids
                .iter()
                .map(|&uid| {
                    payments
                        .iter()
                        .filter(|p| {
                            !p.is_debt
                                && p.user_id == uid
                                && expense_month
                                    .get(&p.expense_id)
                                    .map(|&m| m == month_key)
                                    .unwrap_or(false)
                        })
                        .map(|p| p.amount)
                        .sum()
                })
                .collect()
        })
        .collect();
    (month_keys, payer_ids, amounts)
}

#[derive(PartialEq, Props, Clone)]
struct CategoryTrendProps {
    expenses: Vec<DecryptedExpense>,
}

#[component]
fn CategoryTrendChart(props: CategoryTrendProps) -> Element {
    let mut selected_cat: Signal<Option<&'static str>> = use_signal(|| None);
    let data = aggregate_category_by_month(&props.expenses, 12);
    let max: f64 = data
        .iter()
        .map(|(_, cats)| cats.iter().map(|(_, v)| v).sum::<f64>())
        .fold(0.0_f64, f64::max);
    if max <= 0.0 {
        return rsx! {
            EmptyState {}
        };
    }
    let month_totals: Vec<(String, f64)> = data
        .iter()
        .map(|(k, cats)| (k.clone(), cats.iter().map(|(_, v)| v).sum()))
        .collect();
    let deltas = monthly_deltas(&month_totals);
    let active_cats: Vec<(&'static str, &str)> = CATEGORIES
        .iter()
        .enumerate()
        .filter_map(|(i, &c)| {
            if data.iter().any(|(_, cats)| cats.iter().any(|(cat, _)| *cat == c)) {
                Some((c, PIE_COLORS[i % PIE_COLORS.len()]))
            } else {
                None
            }
        })
        .collect();

    rsx! {
        div { class: "flex flex-col gap-2 w-full",
            div {
                class: "flex gap-1 items-end overflow-x-auto px-1",
                style: "height: 164px;",
                for (idx , (_month_key , cats)) in data.iter().enumerate() {
                    {
                        let total: f64 = cats.iter().map(|(_, v)| v).sum();
                        let bar_h = (total / max * 112.0).max(if total > 0.0 { 2.0 } else { 0.0 });
                        rsx! {
                            div { class: "flex flex-col items-center justify-end gap-0.5 flex-1 min-w-7",
                                if let Some(d) = deltas.get(idx).copied().flatten() {
                                    span {
                                        class: if d >= 0.0 { "text-[9px] leading-none text-error" } else { "text-[9px] leading-none text-success" },
                                        if d >= 0.0 { "+{d:.0}%" } else { "{d:.0}%" }
                                    }
                                }
                                if total > 0.0 {
                                    span { class: "text-[10px] leading-none text-base-content/70", "{total:.0}€" }
                                }
                                div {
                                    class: "w-full rounded-t-lg overflow-hidden flex flex-col-reverse",
                                    style: "height: {bar_h:.0}px;",
                                    for (cat , amt) in cats {
                                        {
                                            let color = CATEGORIES
                                                .iter()
                                                .position(|&c| c == *cat)
                                                .map(|i| PIE_COLORS[i % PIE_COLORS.len()])
                                                .unwrap_or("#ccc");
                                            let seg_h = amt / total * bar_h;
                                            let opacity = match selected_cat() {
                                                None => 1.0,
                                                Some(s) if s == *cat => 1.0,
                                                _ => 0.15,
                                            };
                                            rsx! {
                                                div { style: "height: {seg_h:.1}px; background: {color}; flex-shrink: 0; opacity: {opacity};" }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            div { class: "flex gap-1 px-1",
                for (month_key , _) in &data {
                    span { class: "flex-1 min-w-7 text-center text-[11px] text-base-content/70 leading-tight",
                        "{short_month_label(month_key)}"
                    }
                }
            }
            div { class: "flex flex-wrap gap-1 justify-center pt-1",
                for (cat , color) in &active_cats {
                    {
                        let cat = *cat;
                        rsx! {
                            LegendChip {
                                color: color.to_string(),
                                label: cat.to_string(),
                                detail: None,
                                selected: selected_cat() == Some(cat),
                                onclick: move |_| {
                                    if selected_cat() == Some(cat) {
                                        selected_cat.set(None);
                                    } else {
                                        selected_cat.set(Some(cat));
                                    }
                                },
                            }
                        }
                    }
                }
            }
        }
    }
}

#[derive(PartialEq, Props, Clone)]
struct PersonTrendProps {
    expenses: Vec<DecryptedExpense>,
    payments: Vec<DecryptedPayment>,
    users: Vec<DecryptedUser>,
    has_project: bool,
}

#[component]
fn PersonTrendChart(props: PersonTrendProps) -> Element {
    let mut selected_person: Signal<Option<i32>> = use_signal(|| None);
    if !props.has_project {
        return rsx! {
            div { class: "alert alert-info text-sm",
                {tid!("charts-pick-a-project")}
            }
        };
    }
    let (month_keys, author_ids, amounts) =
        aggregate_person_by_month(&props.expenses, &props.payments, 12);
    let max: f64 = amounts.iter().map(|row| row.iter().sum::<f64>()).fold(0.0_f64, f64::max);
    if max <= 0.0 {
        return rsx! {
            EmptyState {}
        };
    }
    let users = props.users.clone();

    rsx! {
        div { class: "flex flex-col gap-2 w-full",
            div {
                class: "flex gap-1 items-end overflow-x-auto px-1",
                style: "height: 140px;",
                for (m_idx , _month_key) in month_keys.iter().enumerate() {
                    {
                        let row = &amounts[m_idx];
                        let total: f64 = row.iter().sum();
                        let bar_h = (total / max * 112.0).max(if total > 0.0 { 2.0 } else { 0.0 });
                        rsx! {
                            div { class: "flex flex-col items-center justify-end gap-0.5 flex-1 min-w-7",
                                if total > 0.0 {
                                    span { class: "text-[10px] leading-none text-base-content/70", "{total:.0}€" }
                                }
                                div {
                                    class: "w-full rounded-t-lg overflow-hidden flex flex-col-reverse",
                                    style: "height: {bar_h:.0}px;",
                                    for (a_idx , amt) in row.iter().enumerate() {
                                        if *amt > 0.0 {
                                            {
                                                let color = PERSON_COLORS[a_idx % PERSON_COLORS.len()];
                                                let seg_h = amt / total * bar_h;
                                                let person_id = author_ids.get(a_idx).copied();
                                                let opacity = match (selected_person(), person_id) {
                                                    (None, _) => "1",
                                                    (Some(s), Some(id)) if s == id => "1",
                                                    _ => "0.15",
                                                };
                                                rsx! {
                                                    div { style: "height: {seg_h:.1}px; background: {color}; flex-shrink: 0; opacity: {opacity};" }
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
            div { class: "flex gap-1 px-1",
                for month_key in &month_keys {
                    span { class: "flex-1 min-w-7 text-center text-[11px] text-base-content/70 leading-tight",
                        "{short_month_label(month_key)}"
                    }
                }
            }
            div { class: "flex flex-wrap gap-1 justify-center pt-1",
                for (a_idx , author_id) in author_ids.iter().enumerate() {
                    {
                        let color = PERSON_COLORS[a_idx % PERSON_COLORS.len()];
                        let name = user_label(&users, *author_id);
                        let id = *author_id;
                        rsx! {
                            LegendChip {
                                color: color.to_string(),
                                label: name,
                                detail: None,
                                selected: selected_person() == Some(id),
                                onclick: move |_| {
                                    if selected_person() == Some(id) {
                                        selected_person.set(None);
                                    } else {
                                        selected_person.set(Some(id));
                                    }
                                },
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn EmptyState() -> Element {
    rsx! {
        div { class: "flex flex-col items-center gap-2 py-8 text-base-content/70",
            span { class: "text-2xl", "📊" }
            span { class: "text-sm", {tid!("charts-nothing-to-show")} }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDateTime;

    fn make_expense(
        name: &str,
        amount: f64,
        expense_type: ExpenseType,
        date: &str,
        author_id: Option<i32>,
    ) -> DecryptedExpense {
        DecryptedExpense {
            id: 0,
            author_id,
            project_id: Uuid::nil(),
            created_at: NaiveDateTime::default(),
            name: name.to_string(),
            description: None,
            amount,
            expense_type,
            date: date.to_string(),
            category: None,
            source_currency: None,
            source_amount: None,
            rate: None,
        }
    }

    fn make_expense_with_category(name: &str, amount: f64, cat: &str) -> DecryptedExpense {
        DecryptedExpense {
            id: 0,
            author_id: Some(1),
            project_id: Uuid::nil(),
            created_at: NaiveDateTime::default(),
            name: name.to_string(),
            description: None,
            amount,
            expense_type: ExpenseType::Expense,
            date: "2025-01-01".to_string(),
            category: Some(cat.to_string()),
            source_currency: None,
            source_amount: None,
            rate: None,
        }
    }

    fn make_payment(
        id: i32,
        expense_id: i32,
        user_id: i32,
        is_debt: bool,
        amount: f64,
    ) -> DecryptedPayment {
        DecryptedPayment {
            id,
            expense_id,
            user_id,
            is_debt,
            amount,
            created_at: NaiveDateTime::default(),
        }
    }

    #[test]
    fn category_restaurant() {
        let e = make_expense("restaurant", 0.0, ExpenseType::Expense, "2025-01-01", Some(1));
        assert_eq!(get_expense_category(&e), "Nourriture");
    }

    #[test]
    fn category_pizza() {
        let e = make_expense("pizza", 0.0, ExpenseType::Expense, "2025-01-01", Some(1));
        assert_eq!(get_expense_category(&e), "Nourriture");
    }

    #[test]
    fn category_coffee() {
        let e = make_expense("coffee", 0.0, ExpenseType::Expense, "2025-01-01", Some(1));
        assert_eq!(get_expense_category(&e), "Nourriture");
    }

    #[test]
    fn category_beer() {
        let e = make_expense("beer", 0.0, ExpenseType::Expense, "2025-01-01", Some(1));
        assert_eq!(get_expense_category(&e), "Nourriture");
    }

    #[test]
    fn category_grocery() {
        let e = make_expense("grocery", 0.0, ExpenseType::Expense, "2025-01-01", Some(1));
        assert_eq!(get_expense_category(&e), "Nourriture");
    }

    #[test]
    fn category_uber() {
        let e = make_expense("uber", 0.0, ExpenseType::Expense, "2025-01-01", Some(1));
        assert_eq!(get_expense_category(&e), "Transport");
    }

    #[test]
    fn category_train() {
        let e = make_expense("train", 0.0, ExpenseType::Expense, "2025-01-01", Some(1));
        assert_eq!(get_expense_category(&e), "Transport");
    }

    #[test]
    fn category_hotel() {
        let e = make_expense("hotel", 0.0, ExpenseType::Expense, "2025-01-01", Some(1));
        assert_eq!(get_expense_category(&e), "Hébergement");
    }

    #[test]
    fn category_cinema() {
        let e = make_expense("cinema", 0.0, ExpenseType::Expense, "2025-01-01", Some(1));
        assert_eq!(get_expense_category(&e), "Loisirs");
    }

    #[test]
    fn category_fallback() {
        let e =
            make_expense("something totally unknown", 0.0, ExpenseType::Expense, "2025-01-01", Some(1));
        assert_eq!(get_expense_category(&e), "Autres");
    }

    #[test]
    fn category_case_insensitive() {
        let e1 = make_expense("PIZZA", 0.0, ExpenseType::Expense, "2025-01-01", Some(1));
        let e2 = make_expense("Hotel du Nord", 0.0, ExpenseType::Expense, "2025-01-01", Some(1));
        assert_eq!(get_expense_category(&e1), "Nourriture");
        assert_eq!(get_expense_category(&e2), "Hébergement");
    }

    #[test]
    fn category_stored_takes_priority_over_keyword() {
        let e = make_expense_with_category("pizza", 10.0, "Loisirs");
        assert_eq!(get_expense_category(&e), "Loisirs");
    }

    #[test]
    fn category_stored_invalid_falls_back_to_keyword() {
        let mut e = make_expense("pizza", 10.0, ExpenseType::Expense, "2025-01-01", Some(1));
        e.category = Some("UnknownCategory".to_string());
        assert_eq!(get_expense_category(&e), "Nourriture");
    }

    #[test]
    fn aggregate_category_same_category_summed() {
        let expenses = vec![
            make_expense("restaurant du coin", 30.0, ExpenseType::Expense, "2025-01-01", Some(1)),
            make_expense("petit restaurant", 20.0, ExpenseType::Expense, "2025-01-02", Some(1)),
        ];
        let result = aggregate_by_category(&expenses);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].0, "Nourriture");
        assert!((result[0].1 - 50.0).abs() < 0.001);
    }

    #[test]
    fn aggregate_category_excludes_transfer() {
        let expenses = vec![
            make_expense("restaurant", 30.0, ExpenseType::Expense, "2025-01-01", Some(1)),
            make_expense("transfer", 100.0, ExpenseType::Transfer, "2025-01-01", Some(1)),
        ];
        let result = aggregate_by_category(&expenses);
        assert_eq!(result.len(), 1);
        assert!((result[0].1 - 30.0).abs() < 0.001);
    }

    #[test]
    fn aggregate_category_excludes_gain() {
        let expenses = vec![make_expense("refund", 50.0, ExpenseType::Gain, "2025-01-01", Some(1))];
        let result = aggregate_by_category(&expenses);
        assert!(result.is_empty());
    }

    #[test]
    fn aggregate_category_sorted_descending() {
        // Grouping is by parent, so these three need distinct ones.
        let expenses = vec![
            make_expense("train", 15.0, ExpenseType::Expense, "2025-01-01", Some(1)),
            make_expense("restaurant", 30.0, ExpenseType::Expense, "2025-01-01", Some(1)),
            make_expense("hotel", 10.0, ExpenseType::Expense, "2025-01-01", Some(1)),
        ];
        let result = aggregate_by_category(&expenses);
        assert_eq!(result.len(), 3);
        assert!(result[0].1 >= result[1].1);
        assert!(result[1].1 >= result[2].1);
    }

    #[test]
    fn aggregate_category_empty() {
        assert!(aggregate_by_category(&[]).is_empty());
    }

    #[test]
    fn aggregate_month_groups_by_month() {
        let expenses = vec![
            make_expense("dinner", 30.0, ExpenseType::Expense, "2025-01-15", Some(1)),
            make_expense("lunch", 20.0, ExpenseType::Expense, "2025-01-20", Some(1)),
            make_expense("taxi", 15.0, ExpenseType::Expense, "2025-02-10", Some(1)),
        ];
        let result = aggregate_by_month(&expenses, 12);
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].0, "2025-01");
        assert!((result[0].1 - 50.0).abs() < 0.001);
        assert_eq!(result[1].0, "2025-02");
        assert!((result[1].1 - 15.0).abs() < 0.001);
    }

    #[test]
    fn aggregate_month_sorted_ascending() {
        let expenses = vec![
            make_expense("b", 10.0, ExpenseType::Expense, "2025-03-01", Some(1)),
            make_expense("a", 10.0, ExpenseType::Expense, "2025-01-01", Some(1)),
            make_expense("c", 10.0, ExpenseType::Expense, "2025-02-01", Some(1)),
        ];
        let result = aggregate_by_month(&expenses, 12);
        assert_eq!(result[0].0, "2025-01");
        assert_eq!(result[1].0, "2025-02");
        assert_eq!(result[2].0, "2025-03");
    }

    #[test]
    fn aggregate_month_excludes_transfer() {
        let expenses = vec![
            make_expense("dinner", 30.0, ExpenseType::Expense, "2025-01-01", Some(1)),
            make_expense("transfer", 100.0, ExpenseType::Transfer, "2025-01-01", Some(1)),
        ];
        let result = aggregate_by_month(&expenses, 12);
        assert_eq!(result.len(), 1);
        assert!((result[0].1 - 30.0).abs() < 0.001);
    }

    #[test]
    fn aggregate_month_respects_n_months() {
        let expenses: Vec<_> = (1..=15u32)
            .map(|m| {
                make_expense(
                    "dinner",
                    10.0,
                    ExpenseType::Expense,
                    &format!("2024-{:02}-01", m.min(12)),
                    Some(1),
                )
            })
            .collect();
        let result = aggregate_by_month(&expenses, 6);
        assert!(result.len() <= 6);
    }

    #[test]
    fn aggregate_month_skips_invalid_date() {
        let expenses = vec![
            make_expense("a", 10.0, ExpenseType::Expense, "not-a-date", Some(1)),
            make_expense("b", 20.0, ExpenseType::Expense, "2025-01-01", Some(1)),
        ];
        let result = aggregate_by_month(&expenses, 12);
        assert_eq!(result.len(), 1);
    }

    #[test]
    fn aggregate_author_groups_and_sorts() {
        let expenses = vec![
            make_expense("dinner", 30.0, ExpenseType::Expense, "2025-01-01", Some(1)),
            make_expense("lunch", 20.0, ExpenseType::Expense, "2025-01-02", Some(1)),
            make_expense("taxi", 15.0, ExpenseType::Expense, "2025-01-01", Some(2)),
        ];
        let result = aggregate_by_author(&expenses);
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].0, 1);
        assert!((result[0].1 - 50.0).abs() < 0.001);
    }

    #[test]
    fn aggregate_author_excludes_transfer() {
        let expenses = vec![
            make_expense("dinner", 30.0, ExpenseType::Expense, "2025-01-01", Some(1)),
            make_expense("transfer", 100.0, ExpenseType::Transfer, "2025-01-01", Some(2)),
        ];
        let result = aggregate_by_author(&expenses);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].0, 1);
    }

    #[test]
    fn aggregate_author_empty() {
        assert!(aggregate_by_author(&[]).is_empty());
    }

    #[test]
    fn aggregate_author_skips_removed_author() {
        let expenses = vec![
            make_expense("dinner", 30.0, ExpenseType::Expense, "2025-01-01", Some(1)),
            make_expense("orphan", 70.0, ExpenseType::Expense, "2025-01-02", None),
        ];
        let result = aggregate_by_author(&expenses);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].0, 1);
        assert!((result[0].1 - 30.0).abs() < 0.001);
        // The total is computed independently, so the removed author's spend is still counted there.
        assert!((total_spend(&expenses) - 100.0).abs() < 0.001);
    }

    #[test]
    fn avg_per_person_skips_removed_author() {
        let expenses = vec![
            make_expense("dinner", 60.0, ExpenseType::Expense, "2025-01-01", Some(1)),
            make_expense("orphan", 40.0, ExpenseType::Expense, "2025-01-02", None),
        ];
        // One identifiable author, 100 total spend.
        assert!((avg_per_person(&expenses) - 100.0).abs() < 0.001);
    }

    #[test]
    fn export_csv_renders_removed_author_as_blank() {
        let expenses = vec![make_expense("orphan", 10.0, ExpenseType::Expense, "2025-01-01", None)];
        let csv = export_csv_string(&expenses, &[]);
        let row = csv.lines().nth(1).unwrap();
        assert!(row.contains("orphan"), "{row}");
        assert!(row.contains(",,"), "author column must be blank: {row}");
    }

    #[test]
    fn normalize_basic() {
        let data = vec![("A", 100.0_f64), ("B", 50.0)];
        let result = normalize(&data);
        assert!((result[0].1 - 1.0).abs() < 0.001);
        assert!((result[1].1 - 0.5).abs() < 0.001);
    }

    #[test]
    fn normalize_all_zeros_no_panic() {
        let data = vec![("A", 0.0_f64), ("B", 0.0)];
        let result = normalize(&data);
        assert_eq!(result[0].1, 0.0);
        assert_eq!(result[1].1, 0.0);
    }

    #[test]
    fn normalize_single_item() {
        let data = vec![("X", 42.0_f64)];
        let result = normalize(&data);
        assert!((result[0].1 - 1.0).abs() < 0.001);
    }

    #[test]
    fn normalize_preserves_order() {
        let data = vec![("A", 10.0_f64), ("B", 30.0), ("C", 20.0)];
        let result = normalize(&data);
        assert_eq!(result[0].0, "A");
        assert_eq!(result[1].0, "B");
        assert_eq!(result[2].0, "C");
    }

    #[test]
    fn total_spend_sums_only_expenses() {
        let expenses = vec![
            make_expense("a", 50.0, ExpenseType::Expense, "2025-01-01", Some(1)),
            make_expense("b", 30.0, ExpenseType::Transfer, "2025-01-01", Some(1)),
            make_expense("c", 20.0, ExpenseType::Gain, "2025-01-01", Some(1)),
        ];
        assert!((total_spend(&expenses) - 50.0).abs() < 0.001);
    }

    #[test]
    fn total_spend_empty() {
        assert_eq!(total_spend(&[]), 0.0);
    }

    #[test]
    fn fmt_amount_never_shows_negative_zero() {
        assert_eq!(fmt_amount(-0.0), "0");
        assert_eq!(fmt_amount(-0.4), "0");
        assert_eq!(fmt_amount(0.0), "0");
        assert_eq!(fmt_amount(-12.0), "-12");
        assert_eq!(fmt_amount(12.4), "12");
    }

    #[test]
    fn avg_per_person_single_author() {
        let expenses = vec![
            make_expense("a", 60.0, ExpenseType::Expense, "2025-01-01", Some(1)),
            make_expense("b", 40.0, ExpenseType::Expense, "2025-01-01", Some(1)),
        ];
        assert!((avg_per_person(&expenses) - 100.0).abs() < 0.001);
    }

    #[test]
    fn avg_per_person_two_authors() {
        let expenses = vec![
            make_expense("a", 60.0, ExpenseType::Expense, "2025-01-01", Some(1)),
            make_expense("b", 40.0, ExpenseType::Expense, "2025-01-01", Some(2)),
        ];
        assert!((avg_per_person(&expenses) - 50.0).abs() < 0.001);
    }

    #[test]
    fn avg_per_person_empty() {
        assert_eq!(avg_per_person(&[]), 0.0);
    }

    #[test]
    fn monthly_deltas_first_is_none() {
        let data = vec![("2025-01".to_string(), 100.0), ("2025-02".to_string(), 150.0)];
        let d = monthly_deltas(&data);
        assert!(d[0].is_none());
    }

    #[test]
    fn monthly_deltas_increase() {
        let data = vec![("2025-01".to_string(), 100.0), ("2025-02".to_string(), 150.0)];
        let d = monthly_deltas(&data);
        assert!((d[1].unwrap() - 50.0).abs() < 0.001);
    }

    #[test]
    fn monthly_deltas_decrease() {
        let data = vec![("2025-01".to_string(), 200.0), ("2025-02".to_string(), 100.0)];
        let d = monthly_deltas(&data);
        assert!((d[1].unwrap() - (-50.0)).abs() < 0.001);
    }

    #[test]
    fn monthly_deltas_zero_prev_is_none() {
        let data = vec![("2025-01".to_string(), 0.0), ("2025-02".to_string(), 100.0)];
        let d = monthly_deltas(&data);
        assert!(d[1].is_none());
    }

    #[test]
    fn monthly_deltas_length_matches() {
        let data = vec![
            ("2025-01".to_string(), 10.0),
            ("2025-02".to_string(), 20.0),
            ("2025-03".to_string(), 15.0),
        ];
        assert_eq!(monthly_deltas(&data).len(), 3);
    }

    #[test]
    fn filter_no_bounds_returns_all() {
        let expenses = vec![
            make_expense("a", 10.0, ExpenseType::Expense, "2025-01-01", Some(1)),
            make_expense("b", 20.0, ExpenseType::Expense, "2025-06-15", Some(1)),
        ];
        assert_eq!(filter_by_date_range(&expenses, None, None).len(), 2);
    }

    #[test]
    fn filter_from_bound() {
        let expenses = vec![
            make_expense("old", 10.0, ExpenseType::Expense, "2024-12-31", Some(1)),
            make_expense("new", 20.0, ExpenseType::Expense, "2025-01-01", Some(1)),
        ];
        let from = NaiveDate::from_ymd_opt(2025, 1, 1);
        let result = filter_by_date_range(&expenses, from, None);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].name, "new");
    }

    #[test]
    fn filter_to_bound() {
        let expenses = vec![
            make_expense("early", 10.0, ExpenseType::Expense, "2025-01-01", Some(1)),
            make_expense("late", 20.0, ExpenseType::Expense, "2025-06-01", Some(1)),
        ];
        let to = NaiveDate::from_ymd_opt(2025, 3, 31);
        let result = filter_by_date_range(&expenses, None, to);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].name, "early");
    }

    #[test]
    fn filter_both_bounds_inclusive() {
        let expenses = vec![
            make_expense("before", 10.0, ExpenseType::Expense, "2024-12-31", Some(1)),
            make_expense("start", 10.0, ExpenseType::Expense, "2025-01-01", Some(1)),
            make_expense("mid", 10.0, ExpenseType::Expense, "2025-06-15", Some(1)),
            make_expense("end", 10.0, ExpenseType::Expense, "2025-12-31", Some(1)),
            make_expense("after", 10.0, ExpenseType::Expense, "2026-01-01", Some(1)),
        ];
        let from = NaiveDate::from_ymd_opt(2025, 1, 1);
        let to = NaiveDate::from_ymd_opt(2025, 12, 31);
        let result = filter_by_date_range(&expenses, from, to);
        assert_eq!(result.len(), 3);
    }

    #[test]
    fn net_balance_creditor_positive() {
        let expenses = vec![make_expense("x", 100.0, ExpenseType::Expense, "2025-01-01", Some(1))];
        let expenses2 = expenses.clone();
        let mut e = expenses2[0].clone();
        e.id = 1;
        let payments = vec![
            make_payment(1, 1, 1, false, 100.0), // user 1 paid
            make_payment(2, 1, 2, true, 50.0),   // user 2 owes
            make_payment(3, 1, 3, true, 50.0),   // user 3 owes
        ];
        let expenses_with_id = vec![{
            let mut ex = make_expense("x", 100.0, ExpenseType::Expense, "2025-01-01", Some(1));
            ex.id = 1;
            ex
        }];
        let result = aggregate_net_balance(&expenses_with_id, &payments);
        let u1 = result.iter().find(|(id, _)| *id == 1).map(|(_, b)| *b).unwrap();
        let u2 = result.iter().find(|(id, _)| *id == 2).map(|(_, b)| *b).unwrap();
        assert!(u1 > 0.0); // creditor
        assert!(u2 < 0.0); // debtor
    }

    #[test]
    fn net_balance_filters_out_unrelated_payments() {
        let expenses = vec![{
            let mut e = make_expense("x", 100.0, ExpenseType::Expense, "2025-01-01", Some(1));
            e.id = 5;
            e
        }];
        let payments = vec![
            make_payment(1, 5, 1, false, 100.0), // belongs to expense 5 ✓
            make_payment(2, 99, 2, true, 50.0),  // expense 99 not in list ✗
        ];
        let result = aggregate_net_balance(&expenses, &payments);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].0, 1);
    }

    fn with_id(id: i32, amount: f64, expense_type: ExpenseType) -> DecryptedExpense {
        let mut e = make_expense("x", amount, expense_type, "2025-01-01", Some(1));
        e.id = id;
        e
    }

    #[test]
    fn my_share_replaces_the_amount() {
        let expenses = [with_id(7, 200.0, ExpenseType::Expense)];
        let share = HashMap::from([(7, 100.0)]);
        let out = with_my_share(&expenses, &share);
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].amount, 100.0);
    }

    /// Dropped, not zeroed: a 0 slice would still be counted by `charts-expense-count` and listed
    /// in the drilldown table.
    #[test]
    fn an_expense_i_owe_nothing_on_is_dropped() {
        let expenses = [with_id(7, 200.0, ExpenseType::Expense)];
        assert!(with_my_share(&expenses, &HashMap::new()).is_empty());
        assert!(with_my_share(&expenses, &HashMap::from([(7, 0.0)])).is_empty());
    }

    #[test]
    fn my_share_keeps_only_real_expenses() {
        let expenses = [
            with_id(7, 200.0, ExpenseType::Expense),
            with_id(8, 50.0, ExpenseType::Transfer),
            with_id(9, 30.0, ExpenseType::Gain),
        ];
        let share = HashMap::from([(7, 100.0), (8, 50.0), (9, 30.0)]);
        let out = with_my_share(&expenses, &share);
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].id, 7);
    }

    /// The donut's drilldown links to `PaymentPage { project_id, expense_id }` — losing either in
    /// the rewrite would break every row of it.
    #[test]
    fn my_share_preserves_the_identity_the_drilldown_link_needs() {
        let mut e = with_id(7, 200.0, ExpenseType::Expense);
        e.project_id = Uuid::from_u128(42);
        e.name = "dinner".to_string();
        let out = with_my_share(&[e], &HashMap::from([(7, 100.0)]));
        assert_eq!(out[0].id, 7);
        assert_eq!(out[0].project_id, Uuid::from_u128(42));
        assert_eq!(out[0].name, "dinner");
    }

    /// The whole point of the cross-project view: the headline is what one person consumed.
    #[test]
    fn my_share_totals_what_i_consumed_not_what_the_group_spent() {
        let expenses =
            [with_id(7, 200.0, ExpenseType::Expense), with_id(8, 100.0, ExpenseType::Expense)];
        assert_eq!(total_spend(&expenses), 300.0);
        let share = HashMap::from([(7, 100.0), (8, 25.0)]);
        assert_eq!(total_spend(&with_my_share(&expenses, &share)), 125.0);
    }

    #[test]
    fn net_balance_empty_payments() {
        let expenses = vec![make_expense("x", 100.0, ExpenseType::Expense, "2025-01-01", Some(1))];
        assert!(aggregate_net_balance(&expenses, &[]).is_empty());
    }

    // The label is `month_abbrev(month0) + " " + year`, and the month names are locale data now.
    // What is left to get wrong is the split — an off-by-one here mislabels every chart bar.
    #[test]
    fn split_ym_typical() {
        assert_eq!(split_ym("2025-06"), Some((5, "25")));
        assert_eq!(split_ym("2024-01"), Some((0, "24")));
        assert_eq!(split_ym("2024-12"), Some((11, "24")));
    }

    #[test]
    fn split_ym_rejects_anything_that_is_not_a_year_month() {
        assert_eq!(split_ym("badformat"), None);
        assert_eq!(split_ym("2025"), None);
        // A one-digit year used to panic on `&parts[0][2..]`.
        assert_eq!(split_ym("1-06"), None);
    }

    /// Out-of-range months are clamped rather than panicking: the value comes out of an encrypted
    /// payload, so a tampered writer must not take down the charts page.
    #[test]
    fn split_ym_clamps_an_impossible_month() {
        assert_eq!(split_ym("2025-99"), Some((11, "25")));
        assert_eq!(split_ym("2025-00"), Some((0, "25")));
        assert_eq!(split_ym("2025-xx"), Some((0, "25")));
    }

    #[test]
    fn short_month_label_returns_unparseable_input_verbatim() {
        assert_eq!(short_month_label("badformat"), "badformat");
        assert_eq!(short_month_label("2025"), "2025");
    }

    #[test]
    fn avg_per_person_only_transfers_returns_zero() {
        let expenses = vec![
            make_expense("Transfer", 100.0, ExpenseType::Transfer, "2025-01-01", Some(1)),
            make_expense("Transfer", 50.0, ExpenseType::Transfer, "2025-02-01", Some(2)),
        ];
        assert_eq!(avg_per_person(&expenses), 0.0);
    }

    // csv_field's own escaping cases are covered in expenses_page::export.

    #[test]
    fn export_csv_comma_in_name_is_quoted() {
        let expenses =
            vec![make_expense("Bière, frites", 12.0, ExpenseType::Expense, "2025-03-01", Some(1))];
        let csv = export_csv_string(&expenses, &[]);
        assert!(csv.contains("\"Bière, frites\""));
    }

    #[test]
    fn export_csv_has_header() {
        let csv = export_csv_string(&[], &[]);
        assert!(csv.starts_with("date,name,category,amount,author,type,description\n"));
    }

    #[test]
    fn export_csv_one_row() {
        let expenses = vec![make_expense("Dîner", 45.5, ExpenseType::Expense, "2025-06-15", Some(1))];
        let csv = export_csv_string(&expenses, &[]);
        assert!(csv.contains("2025-06-15"));
        assert!(csv.contains("Dîner"));
        assert!(csv.contains("45.50"));
        assert!(csv.contains("expense"));
    }

    #[test]
    fn export_csv_handles_missing_description() {
        let expenses = vec![make_expense("x", 10.0, ExpenseType::Expense, "2025-01-01", Some(1))];
        let csv = export_csv_string(&expenses, &[]);
        let lines: Vec<&str> = csv.lines().collect();
        let data_line = lines[1];
        assert!(data_line.ends_with(',') || !data_line.contains("None"));
    }
}
