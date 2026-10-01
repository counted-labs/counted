use api::expenses::expenses_controller::get_expenses_by_project_id;
use api::payments::payments_controller::get_payments_by_project_id;
use api::projects::projects_controller::get_projects_by_ids;
use api::users::users_controller::get_users_by_project_id;
use chrono::{Datelike, Local, NaiveDate};
use dioxus::fullstack::Json;
use dioxus::prelude::*;
use shared::{BatchProject, ExpenseType, ProjectPayload};
use std::collections::{HashMap, HashSet};
use uuid::Uuid;

use super::cards::{
    BalanceCard, CategoryBreakdownCard, CategoryOverTimeCard, CategoryShareCard, PaidVsShareCard,
    PersonView, ProjectTotalsCard, RunningTotalCard, ScopeSwitch, SpendOverTimeCard, StatTiles,
};
use super::model::{
    aggregate_by_category, auto_bucket, average, balance_by_bucket, bucket_count, bucket_starts, category_shares, currencies, expense_count, filter_by_date_range, fold_categories,
    in_projects, paid_vs_share, per_person, running_total, sum_by_bucket, sum_by_expense, sum_over,
    total_spend, totals_by_project, with_my_share, Bucket, ProjectInfo, Scope, MAX_BUCKETS, OTHER,
};
use super::primitives::{Column, EmptyState, Segment, Segmented};
use super::svg::label_every;
use crate::categories::get_expense_category;
use crate::common::{
    format_date, initials, key_of, month_abbrev, pending, project_key, read_from_ls, user_color_class,
    AppHeader, DropdownButton, DropdownItem, PullToRefresh,
};
use crate::crypto::decrypt_json;
use crate::decrypted::{
    decrypt_expense, decrypt_payment, decrypt_user, DecryptedExpense, DecryptedPayment,
    DecryptedUser,
};
use crate::expenses::helpers::export::csv_field;
use crate::route::Route;
use crate::tid;

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
        let etype = match e.expense_type {
            ExpenseType::Expense => "expense",
            ExpenseType::Transfer => "transfer",
            ExpenseType::Gain => "gain",
        };
        out.push_str(&format!(
            "{},{},{},{:.2},{},{},{}\n",
            csv_field(&e.date),
            csv_field(&e.name),
            get_expense_category(e),
            e.amount,
            csv_field(author),
            etype,
            csv_field(e.description.as_deref().unwrap_or(""))
        ));
    }
    out
}

#[derive(Clone, PartialEq, Default)]
struct ChartData {
    expenses: Vec<DecryptedExpense>,
    payments: Vec<DecryptedPayment>,
    my_share: HashMap<i32, f64>,
    my_paid: HashMap<i32, f64>,
    /// Projects whose payments could not be read, or where I am nobody: their expenses are
    /// loaded but carry no share, so the headline says it is short.
    left_out: usize,
}

#[derive(PartialEq, Clone, Copy)]
enum Tab {
    Categories,
    People,
    Time,
}

/// As (value, translation key). Labels must stay short so five equal segments fit at 360px.
const PERIODS: [(&str, &str); 5] = [
    ("all", "period-all"),
    ("month", "period-month"),
    ("3months", "period-3months"),
    ("year", "period-year"),
    ("custom", "period-custom"),
];

/// `custom` alone leaves the dates alone: it means "whatever the two fields say".
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

fn earliest_date(expenses: &[DecryptedExpense]) -> Option<NaiveDate> {
    expenses.iter().filter_map(|e| NaiveDate::parse_from_str(&e.date, "%Y-%m-%d").ok()).min()
}

fn axis_label(start: NaiveDate, bucket: Bucket) -> String {
    match bucket {
        Bucket::Day | Bucket::Week => start.day().to_string(),
        Bucket::Month => month_abbrev(start.month0() as usize).chars().next().map(|c| c.to_uppercase().collect()).unwrap_or_default(),
    }
}

fn axis_labels(starts: &[NaiveDate], bucket: Bucket) -> Vec<String> {
    let every = label_every(starts.len(), 12);
    starts
        .iter()
        .enumerate()
        .map(|(i, d)| if i % every == 0 || i + 1 == starts.len() { axis_label(*d, bucket) } else { String::new() })
        .collect()
}

fn columns(values: &[f64], labels: &[String], marked: Option<usize>, currency: &str) -> Vec<Column> {
    values
        .iter()
        .zip(labels)
        .enumerate()
        .map(|(i, (v, l))| Column {
            label: l.clone(),
            value: *v,
            value_label: (marked == Some(i) && *v > 0.0).then(|| super::model::money(*v, currency)),
        })
        .collect()
}

fn peak(values: &[f64]) -> Option<usize> {
    values
        .iter()
        .enumerate()
        .max_by(|a, b| a.1.partial_cmp(b.1).unwrap_or(std::cmp::Ordering::Equal))
        .map(|(i, _)| i)
}

async fn load_projects() -> Vec<ProjectInfo> {
    let ls = read_from_ls();
    let ids: Vec<Uuid> = ls.projects.iter().map(|p| p.project_id).collect();
    if ids.is_empty() {
        return vec![];
    }
    let Ok(encrypted) = get_projects_by_ids(Json(BatchProject { ids })).await else {
        return vec![];
    };
    encrypted
        .into_iter()
        .filter_map(|p| {
            let local = ls.projects.iter().find(|lp| lp.project_id == p.id)?;
            let payload = decrypt_json::<ProjectPayload>(&key_of(local)?, &p.payload).ok()?;
            Some(ProjectInfo { id: p.id, name: payload.name, currency: payload.currency, my_user_id: local.user_id })
        })
        .collect()
}

async fn load_data(project_filter: Option<Uuid>) -> ChartData {
    let ls = read_from_ls();
    let mut data = ChartData::default();
    for p in ls.projects.iter().filter(|p| project_filter.is_none_or(|id| id == p.project_id)) {
        let Some(key) = key_of(p) else {
            continue;
        };
        let Ok(expenses) = get_expenses_by_project_id(p.project_id).await else {
            continue;
        };
        data.expenses.extend(expenses.iter().filter_map(|e| decrypt_expense(&key, e).ok()));
        let Ok(payments) = get_payments_by_project_id(p.project_id).await else {
            data.left_out += 1;
            continue;
        };
        let decrypted: Vec<DecryptedPayment> = payments.iter().filter_map(|p| decrypt_payment(&key, p).ok()).collect();
        match p.user_id {
            Some(user_id) => {
                data.my_share.extend(sum_by_expense(&decrypted, user_id, true));
                data.my_paid.extend(sum_by_expense(&decrypted, user_id, false));
            }
            None => data.left_out += 1,
        }
        data.payments.extend(decrypted);
    }
    data
}

async fn load_users(project: Option<Uuid>) -> Vec<DecryptedUser> {
    let Some(pid) = project else {
        return vec![];
    };
    let Some(key) = project_key(pid) else {
        return vec![];
    };
    let Ok(users) = get_users_by_project_id(pid).await else {
        return vec![];
    };
    users.into_iter().filter_map(|u| decrypt_user(&key, &u).ok()).collect()
}

#[component]
pub fn ChartsPage() -> Element {
    let mut active_tab = use_signal(|| Tab::Categories);
    let mut selected_project: Signal<Option<Uuid>> = use_signal(|| None);
    let mut date_from: Signal<Option<NaiveDate>> = use_signal(|| None);
    let mut date_to: Signal<Option<NaiveDate>> = use_signal(|| None);
    let mut preset: Signal<&'static str> = use_signal(|| "all");
    let mut scope = use_signal(|| Scope::Me);
    let mut chosen_currency: Signal<Option<String>> = use_signal(|| None);
    let mut chosen_bucket: Signal<Option<Bucket>> = use_signal(|| None);
    let mut trend_category: Signal<Option<&'static str>> = use_signal(|| None);

    let projects = use_resource(load_projects);
    let data = use_resource(move || load_data(selected_project()));
    let users = use_resource(move || load_users(selected_project()));

    // Reset to all-time on every reload: a project change or the first load.
    use_effect(move || {
        if let Some(d) = data.read().as_ref() {
            date_from.set(earliest_date(&d.expenses));
            date_to.set(Some(Local::now().date_naive()));
            preset.set("all");
            chosen_bucket.set(None);
        }
    });

    let on_export = move |_: ()| {
        let raw = data.read();
        let Some(d) = raw.as_ref() else {
            return;
        };
        let users = users.read().as_ref().cloned().unwrap_or_default();
        let csv = export_csv_string(&filter_by_date_range(&d.expenses, date_from(), date_to()), &users);
        crate::expenses::helpers::export::trigger_download(&csv, "depenses.csv", "text/csv;charset=utf-8");
    };

    let today = Local::now().date_naive();
    let projects_list = projects.read().as_ref().cloned().unwrap_or_default();
    let all_mode = selected_project().is_none();
    let selected_info = selected_project().and_then(|id| projects_list.iter().find(|p| p.id == id).cloned());
    let currency_list = currencies(&projects_list);
    let currency = match &selected_info {
        Some(p) => p.currency.clone(),
        None => chosen_currency().filter(|c| currency_list.contains(c)).or_else(|| currency_list.first().cloned()).unwrap_or_default(),
    };
    let scoped_ids: HashSet<Uuid> = match &selected_info {
        Some(p) => HashSet::from([p.id]),
        None => projects_list.iter().filter(|p| p.currency == currency).map(|p| p.id).collect(),
    };
    let my_user_id = selected_info.as_ref().and_then(|p| p.my_user_id);
    let effective_scope = if all_mode {
        Scope::Me
    } else if my_user_id.is_some() {
        scope()
    } else {
        Scope::Group
    };

    rsx! {
        div { class: "container app-container bg-base-100 overflow-auto p-4 pb-24 max-w-md w-full mx-auto flex flex-col gap-4",
            PullToRefresh {
                on_refresh: move |_| {
                    let (mut p, mut d, mut u) = (projects, data, users);
                    p.restart();
                    d.restart();
                    u.restart();
                },
                busy: pending(&projects) || pending(&data) || pending(&users),
            }
            AppHeader {
                title: tid!("nav-charts"),
                back_button_route: Route::ProjectsPage {},
                DropdownButton { label: tid!("export"),
                    DropdownItem { variant: "ghost", label: tid!("export-csv"), onclick: on_export }
                }
            }

            {
                let earliest = data.read().as_ref().and_then(|d| earliest_date(&d.expenses));
                let from_val = date_from().map(|d| d.to_string()).unwrap_or_default();
                let to_val = date_to().map(|d| d.to_string()).unwrap_or_default();
                let min_date = earliest.map(|d| d.to_string()).unwrap_or_default();
                let today_str = today.to_string();
                rsx! {
                    div { class: "rounded-[var(--radius-box)] bg-base-100 shadow-soft p-3 flex flex-col gap-3",
                        label { class: "select bg-base-200 w-full border-0 min-h-11",
                            span { class: "label", {tid!("charts-project")} }
                            select {
                                id: "charts-project",
                                onchange: move |e| selected_project.set(e.value().parse::<Uuid>().ok()),
                                option { value: "all", selected: all_mode, {tid!("charts-all-projects")} }
                                for p in projects_list.iter() {
                                    option { value: "{p.id}", selected: selected_project() == Some(p.id), "{p.name}" }
                                }
                            }
                        }
                        if all_mode && currency_list.len() > 1 {
                            Segmented {
                                aria_label: tid!("charts-currency"),
                                segments: currency_list.iter().map(|c| Segment { label: c.clone(), active: *c == currency, enabled: true }).collect::<Vec<_>>(),
                                on_select: {
                                    let list = currency_list.clone();
                                    move |i: usize| chosen_currency.set(list.get(i).cloned())
                                },
                            }
                        }
                        Segmented {
                            aria_label: tid!("charts-period"),
                            segments: PERIODS.into_iter().map(|(value, key)| Segment { label: tid!(key), active: *preset.read() == value, enabled: true }).collect::<Vec<_>>(),
                            on_select: move |i: usize| apply_period(PERIODS[i].0, date_from, date_to, preset, earliest),
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
                                            let d = NaiveDate::parse_from_str(&e.value(), "%Y-%m-%d").ok();
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
                                            date_to.set(NaiveDate::parse_from_str(&e.value(), "%Y-%m-%d").ok());
                                            preset.set("custom");
                                        },
                                    }
                                }
                            }
                        } else if let (Some(f), Some(t)) = (date_from(), date_to()) {
                            p { class: "text-xs text-base-content/70", "{format_date(f)} → {format_date(t)}" }
                        }
                    }
                }
            }

            if !all_mode && my_user_id.is_some() {
                ScopeSwitch { scope: effective_scope, on_change: move |s| scope.set(s) }
            }

            match &*data.read() {
                Some(d) => {
                    let group = in_projects(&filter_by_date_range(&d.expenses, date_from(), date_to()), &scoped_ids);
                    let group_total = total_spend(&group);
                    let mine = with_my_share(&group, &d.my_share);
                    let chart_expenses = if effective_scope == Scope::Me { mine.clone() } else { group.clone() };
                    let users = users.read().as_ref().cloned().unwrap_or_default();
                    let from = date_from().or_else(|| earliest_date(&group)).unwrap_or(today);
                    let to = date_to().unwrap_or(today).max(from);
                    let auto = auto_bucket(from, to);
                    let fits = [Bucket::Day, Bucket::Week, Bucket::Month].map(|b| bucket_count(from, to, b) <= MAX_BUCKETS);
                    let spend_bucket = chosen_bucket().filter(|b| bucket_count(from, to, *b) <= MAX_BUCKETS).unwrap_or(auto);
                    let range = format!("{} → {}", format_date(from), format_date(to));

                    rsx! {
                        StatTiles {
                            scope: effective_scope,
                            currency: currency.clone(),
                            group_total,
                            count: expense_count(&group),
                            per_person: per_person(group_total, users.len()),
                            my_share: total_spend(&mine),
                            my_paid: sum_over(&group, &d.my_paid),
                        }
                        if all_mode {
                            div { class: "flex flex-col gap-0.5 px-1",
                                p { class: "text-xs text-base-content/70", {tid!("charts-my-share-note")} }
                                if d.left_out > 0 {
                                    p { class: "text-xs text-base-content/70", {tid!("charts-my-share-skipped", count: d.left_out as i64)} }
                                }
                            }
                        }

                        div { role: "tablist", class: "tabs tabs-box shadow-soft",
                            for (tab , id , label) in [
                                (Tab::Categories, "charts-tab-categories", tid!("charts-tab-categories")),
                                (Tab::People, "charts-tab-people", if all_mode { tid!("charts-tab-projects") } else { tid!("charts-tab-people") }),
                                (Tab::Time, "charts-tab-time", tid!("charts-tab-trends")),
                            ] {
                                button {
                                    id,
                                    role: "tab",
                                    aria_selected: active_tab() == tab,
                                    aria_controls: "charts-panel",
                                    class: if active_tab() == tab { "tab tab-active flex-1 min-h-11 text-sm font-semibold whitespace-nowrap" } else { "tab flex-1 min-h-11 text-sm text-base-content/70 whitespace-nowrap" },
                                    onclick: move |_| active_tab.set(tab),
                                    "{label}"
                                }
                            }
                        }

                        div {
                            id: "charts-panel",
                            role: "tabpanel",
                            class: "flex flex-col gap-4",
                            aria_labelledby: match active_tab() {
                                Tab::Categories => "charts-tab-categories",
                                Tab::People => "charts-tab-people",
                                Tab::Time => "charts-tab-time",
                            },
                            if chart_expenses.is_empty() {
                                EmptyState {}
                            } else {
                                match active_tab() {
                                    Tab::Categories => {
                                        let slices = fold_categories(&aggregate_by_category(&chart_expenses));
                                        rsx! {
                                            if effective_scope == Scope::Me {
                                                CategoryShareCard { rows: category_shares(&group, &d.my_share), currency: currency.clone() }
                                            }
                                            CategoryBreakdownCard { slices, expenses: chart_expenses.clone(), currency: currency.clone() }
                                        }
                                    }
                                    Tab::People if all_mode => {
                                        let everything = with_my_share(&filter_by_date_range(&d.expenses, date_from(), date_to()), &d.my_share);
                                        rsx! {
                                            ProjectTotalsCard { groups: totals_by_project(&everything, &projects_list) }
                                        }
                                    }
                                    Tab::People => {
                                        let people: Vec<PersonView> = paid_vs_share(&group, &d.payments)
                                            .into_iter()
                                            .map(|r| {
                                                let name = users.iter().find(|u| u.id == r.user_id).map(|u| u.name.clone()).unwrap_or_default();
                                                PersonView {
                                                    initials: initials(&name),
                                                    name,
                                                    color_class: user_color_class(r.user_id).to_string(),
                                                    paid: r.paid,
                                                    share: r.share,
                                                    is_me: Some(r.user_id) == my_user_id,
                                                }
                                            })
                                            .collect();
                                        let starts = bucket_starts(from, to, auto);
                                        rsx! {
                                            PaidVsShareCard { people, currency: currency.clone() }
                                            if let Some(uid) = my_user_id {
                                                BalanceCard {
                                                    values: balance_by_bucket(&d.expenses, &d.payments, uid, &starts, auto),
                                                    labels: axis_labels(&starts, auto),
                                                    currency: currency.clone(),
                                                    range: range.clone(),
                                                }
                                            }
                                        }
                                    }
                                    Tab::Time => {
                                        let spend_starts = bucket_starts(from, to, spend_bucket);
                                        let spend = sum_by_bucket(&chart_expenses, &spend_starts, spend_bucket);
                                        let spend_labels = axis_labels(&spend_starts, spend_bucket);
                                        let categories: Vec<&'static str> = aggregate_by_category(&chart_expenses).into_iter().map(|(c, _)| c).collect();
                                        let category = trend_category()
                                            .filter(|c| categories.contains(c))
                                            .or_else(|| categories.iter().find(|c| **c != OTHER).copied())
                                            .unwrap_or(OTHER);
                                        let starts = bucket_starts(from, to, auto);
                                        let labels = axis_labels(&starts, auto);
                                        let in_category: Vec<DecryptedExpense> = chart_expenses.iter().filter(|e| get_expense_category(e) == category).cloned().collect();
                                        let category_values = sum_by_bucket(&in_category, &starts, auto);
                                        let totals = sum_by_bucket(&chart_expenses, &starts, auto);
                                        rsx! {
                                            SpendOverTimeCard {
                                                columns: columns(&spend, &spend_labels, peak(&spend), &currency),
                                                bucket: spend_bucket,
                                                fits,
                                                on_bucket: move |b| chosen_bucket.set(Some(b)),
                                                average: average(&spend),
                                                currency: currency.clone(),
                                                range: range.clone(),
                                            }
                                            CategoryOverTimeCard {
                                                categories,
                                                selected: category,
                                                on_select: move |c| trend_category.set(Some(c)),
                                                columns: columns(&category_values, &labels, category_values.len().checked_sub(1), &currency),
                                                bucket: auto,
                                                average: average(&category_values),
                                                currency: currency.clone(),
                                                range: range.clone(),
                                            }
                                            RunningTotalCard {
                                                values: running_total(&totals),
                                                labels,
                                                bucket: auto,
                                                average: average(&totals),
                                                currency: currency.clone(),
                                                since: format_date(from),
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                None => rsx! {
                    div { class: "flex justify-center py-8",
                        span { class: "loading loading-spinner loading-md", role: "status", aria_label: tid!("loading") }
                    }
                },
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::categories::get_expense_category;
    use crate::charts::model::aggregate_by_category;
    use chrono::NaiveDateTime;

    fn make_expense(name: &str, amount: f64, expense_type: ExpenseType, author_id: Option<i32>) -> DecryptedExpense {
        DecryptedExpense {
            id: 0,
            author_id,
            project_id: Uuid::nil(),
            created_at: NaiveDateTime::default(),
            name: name.to_string(),
            description: None,
            amount,
            expense_type,
            date: "2025-01-01".to_string(),
            category: None,
            source_currency: None,
            source_amount: None,
            rate: None,
            recurring_id: None,
            estimate: false,
        }
    }

    fn category_of(name: &str) -> &'static str {
        get_expense_category(&make_expense(name, 0.0, ExpenseType::Expense, Some(1)))
    }

    #[test]
    fn keyword_categories() {
        for (name, cat) in [
            ("restaurant", "Nourriture"),
            ("pizza", "Nourriture"),
            ("coffee", "Nourriture"),
            ("beer", "Nourriture"),
            ("grocery", "Nourriture"),
            ("uber", "Transport"),
            ("train", "Transport"),
            ("hotel", "Hébergement"),
            ("cinema", "Loisirs"),
            ("something totally unknown", "Autres"),
            ("PIZZA", "Nourriture"),
            ("Hotel du Nord", "Hébergement"),
        ] {
            assert_eq!(category_of(name), cat, "{name}");
        }
    }

    #[test]
    fn stored_category_takes_priority_unless_unknown() {
        let mut e = make_expense("pizza", 10.0, ExpenseType::Expense, Some(1));
        e.category = Some("Loisirs".to_string());
        assert_eq!(get_expense_category(&e), "Loisirs");
        e.category = Some("UnknownCategory".to_string());
        assert_eq!(get_expense_category(&e), "Nourriture");
    }

    #[test]
    fn category_totals_exclude_transfers_and_gains() {
        let expenses = vec![
            make_expense("restaurant", 30.0, ExpenseType::Expense, Some(1)),
            make_expense("petit restaurant", 20.0, ExpenseType::Expense, Some(1)),
            make_expense("transfer", 100.0, ExpenseType::Transfer, Some(1)),
            make_expense("refund", 50.0, ExpenseType::Gain, Some(1)),
        ];
        assert_eq!(aggregate_by_category(&expenses), vec![("Nourriture", 50.0)]);
    }

    #[test]
    fn axis_labels_thin_but_keep_the_last() {
        let from = NaiveDate::from_ymd_opt(2026, 9, 1).unwrap();
        let starts: Vec<NaiveDate> = (0..30).map(|i| from + chrono::Duration::days(i)).collect();
        let labels = axis_labels(&starts, Bucket::Day);
        assert_eq!(labels[0], "1");
        assert_eq!(labels[1], "");
        assert_eq!(labels[29], "30");
    }

    #[test]
    fn peak_marks_the_largest_column() {
        assert_eq!(peak(&[1.0, 5.0, 3.0]), Some(1));
        assert_eq!(peak(&[]), None);
    }

    #[test]
    fn export_csv_renders_removed_author_as_blank() {
        let csv = export_csv_string(&[make_expense("orphan", 10.0, ExpenseType::Expense, None)], &[]);
        let row = csv.lines().nth(1).unwrap();
        assert!(row.contains("orphan"), "{row}");
        assert!(row.contains(",,"), "author column must be blank: {row}");
    }

    #[test]
    fn export_csv_comma_in_name_is_quoted() {
        let csv = export_csv_string(&[make_expense("Bière, frites", 12.0, ExpenseType::Expense, Some(1))], &[]);
        assert!(csv.contains("\"Bière, frites\""));
    }

    #[test]
    fn export_csv_has_header() {
        assert!(export_csv_string(&[], &[]).starts_with("date,name,category,amount,author,type,description\n"));
    }

    #[test]
    fn export_csv_one_row() {
        let mut e = make_expense("Dîner", 45.5, ExpenseType::Expense, Some(1));
        e.date = "2025-06-15".to_string();
        let csv = export_csv_string(&[e], &[]);
        for part in ["2025-06-15", "Dîner", "45.50", "expense"] {
            assert!(csv.contains(part), "{part} in {csv}");
        }
    }
}
