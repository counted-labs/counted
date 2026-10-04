use chrono::{Datelike, NaiveDate};
use dioxus::prelude::*;

use super::model::{
    category_color, expenses_in_category, money, percent, signed_money, Bucket, CategoryShare,
    CurrencyTotals, Scope,
};
use super::primitives::{
    ChartCard, Column, ColumnChart, LineChart, Note, Segment, Segmented, Swatch,
};
use super::svg::donut_path;
use crate::categories::category_label;
use crate::common::{month_abbrev, Avatar};
use crate::decrypted::DecryptedExpense;
use crate::route::Route;
use crate::tid;

#[component]
pub fn ScopeSwitch(scope: Scope, on_change: EventHandler<Scope>) -> Element {
    rsx! {
        Segmented {
            aria_label: tid!("charts-scope"),
            segments: vec![
                Segment { label: tid!("charts-scope-group"), active: scope == Scope::Group, enabled: true },
                Segment { label: tid!("charts-scope-me"), active: scope == Scope::Me, enabled: true },
            ],
            on_select: move |i: usize| on_change.call(if i == 0 { Scope::Group } else { Scope::Me }),
        }
    }
}

#[component]
pub fn StatTiles(
    scope: Scope,
    currency: String,
    group_total: f64,
    count: usize,
    per_person: f64,
    my_share: f64,
    my_paid: f64,
) -> Element {
    let diff = my_paid - my_share;
    rsx! {
        div { class: "flex shadow-soft rounded-[var(--radius-box)] bg-base-100 divide-x divide-base-200",
            match scope {
                Scope::Group => rsx! {
                    div { class: "flex-1 min-w-0 py-3 px-4 flex flex-col gap-0.5",
                        span { class: "text-xs text-base-content/70", {tid!("charts-total-spent")} }
                        span { class: "text-2xl font-extrabold font-display text-gradient-brand leading-tight", {money(group_total, &currency)} }
                        span { class: "text-xs text-base-content/70", {tid!("charts-expense-count", count: count as i64)} }
                    }
                    div { class: "flex-1 min-w-0 py-3 px-4 flex flex-col gap-0.5",
                        span { class: "text-xs text-base-content/70", {tid!("charts-avg-per-person")} }
                        span { class: "text-2xl font-extrabold font-display text-primary leading-tight", {money(per_person, &currency)} }
                    }
                },
                Scope::Me => rsx! {
                    div { class: "flex-1 min-w-0 py-3 px-4 flex flex-col gap-0.5",
                        span { class: "text-xs text-base-content/70", {tid!("charts-my-share")} }
                        span { class: "text-2xl font-extrabold font-display text-gradient-brand leading-tight", {money(my_share, &currency)} }
                        span { class: "text-xs text-base-content/70",
                            {tid!("charts-share-of-total", pct: percent(my_share, group_total), total: money(group_total, &currency))}
                        }
                    }
                    div { class: "flex-1 min-w-0 py-3 px-4 flex flex-col gap-0.5",
                        span { class: "text-xs text-base-content/70", {tid!("charts-i-paid")} }
                        span { class: "text-2xl font-extrabold font-display text-primary leading-tight", {money(my_paid, &currency)} }
                        if diff.round() > 0.0 {
                            span { class: "text-xs font-semibold text-success", {tid!("charts-paid-more", amount: money(diff, &currency))} }
                        } else if diff.round() < 0.0 {
                            span { class: "text-xs font-semibold text-error", {tid!("charts-paid-less", amount: money(-diff, &currency))} }
                        } else {
                            span { class: "text-xs text-base-content/70", {tid!("charts-paid-even")} }
                        }
                    }
                },
            }
        }
    }
}

#[component]
pub fn CategoryShareCard(rows: Vec<CategoryShare>, currency: String) -> Element {
    let max = rows.iter().map(|r| r.group).fold(0.0, f64::max);
    rsx! {
        ChartCard { title: tid!("charts-part-title"), description: tid!("charts-part-desc"),
            div { class: "flex flex-col gap-2.5",
                for row in rows {
                    {
                        let color = category_color(row.category);
                        let track = if max > 0.0 { row.group / max * 100.0 } else { 0.0 };
                        let fill = if row.group > 0.0 { row.mine / row.group * 100.0 } else { 0.0 };
                        rsx! {
                            div { class: "grid grid-cols-[5.25rem_1fr_auto] items-center gap-2 text-xs",
                                span { class: "flex items-center gap-1.5 min-w-0",
                                    Swatch { color }
                                    span { class: "truncate", {category_label(row.category)} }
                                }
                                span { class: "h-2 rounded-full bg-base-300 relative", style: "width: {track:.1}%;",
                                    span { class: "absolute inset-y-0 left-0 rounded-full", style: "width: {fill:.1}%; background: {color};" }
                                }
                                span { class: "text-right whitespace-nowrap tabular-nums",
                                    b { {money(row.mine, &currency)} }
                                    span { class: "text-base-content/70", {format!(" / {}", money(row.group, &currency))} }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

const DRILL_PREVIEW: usize = 3;

fn short_date(date: &str) -> String {
    NaiveDate::parse_from_str(date, "%Y-%m-%d")
        .map(|d| format!("{} {}", d.day(), month_abbrev(d.month0() as usize)))
        .unwrap_or_else(|_| date.to_string())
}

#[component]
pub fn CategoryBreakdownCard(
    slices: Vec<(&'static str, f64)>,
    expenses: Vec<DecryptedExpense>,
    currency: String,
) -> Element {
    let mut selected: Signal<Option<&'static str>> = use_signal(|| None);
    let mut expanded = use_signal(|| false);
    let total: f64 = slices.iter().map(|(_, v)| v).sum();
    let mut cumulative = 0.0;
    let arcs: Vec<(&'static str, String)> = slices
        .iter()
        .map(|(category, value)| {
            let start = cumulative;
            cumulative += value / total.max(f64::MIN_POSITIVE);
            (*category, donut_path(90.0, 84.0, 54.0, start, cumulative))
        })
        .collect();
    let chosen = selected().filter(|c| slices.iter().any(|(s, _)| s == c));
    let drill = chosen.map(|c| expenses_in_category(&expenses, c, &slices)).unwrap_or_default();
    let shown = if expanded() { drill.len() } else { drill.len().min(DRILL_PREVIEW) };
    let mut toggle = move |category: &'static str| {
        expanded.set(false);
        selected.set(if selected() == Some(category) { None } else { Some(category) });
    };

    rsx! {
        ChartCard { title: tid!("charts-breakdown-title"), description: tid!("charts-breakdown-desc"),
            svg { view_box: "0 0 180 180", class: "w-[180px] max-w-full h-auto mx-auto", role: "img", "aria-label": tid!("charts-breakdown-title"),
                for (category , d) in arcs {
                    path {
                        d,
                        fill: category_color(category),
                        stroke: "var(--color-base-100)",
                        stroke_width: "2",
                        opacity: if chosen.is_some() && chosen != Some(category) { "0.3" } else { "1" },
                        class: "cursor-pointer transition-opacity",
                        onclick: move |_| toggle(category),
                    }
                }
                match chosen.and_then(|c| slices.iter().find(|(s, _)| *s == c)) {
                    Some((category, value)) => rsx! {
                        text { x: "90", y: "76", text_anchor: "middle", style: "font-size:10px;fill:var(--color-base-content);fill-opacity:.5", {category_label(category)} }
                        text { x: "90", y: "98", text_anchor: "middle", class: "font-display", style: "font-size:22px;font-weight:800;fill:var(--color-primary)", "{percent(*value, total)}%" }
                        text { x: "90", y: "114", text_anchor: "middle", style: "font-size:10px;fill:var(--color-base-content);fill-opacity:.5",
                            {tid!("charts-of-total", amount: money(*value, &currency), total: money(total, &currency))}
                        }
                    },
                    None => rsx! {
                        text { x: "90", y: "82", text_anchor: "middle", style: "font-size:10px;fill:var(--color-base-content);fill-opacity:.5", {tid!("charts-total")} }
                        text { x: "90", y: "104", text_anchor: "middle", class: "font-display", style: "font-size:20px;font-weight:800;fill:var(--color-primary)", {money(total, &currency)} }
                    },
                }
            }
            div { class: "flex flex-col",
                for (category , value) in slices.iter().copied() {
                    button {
                        r#type: "button",
                        aria_pressed: chosen == Some(category),
                        class: if chosen == Some(category) { "grid grid-cols-[auto_1fr_auto] items-center gap-2 px-2 py-[7px] rounded-field text-xs text-left bg-base-200 font-semibold" } else { "grid grid-cols-[auto_1fr_auto] items-center gap-2 px-2 py-[7px] rounded-field text-xs text-left hover:bg-base-200" },
                        onclick: move |_| toggle(category),
                        Swatch { color: category_color(category) }
                        span { class: "min-w-0 truncate",
                            {category_label(category)}
                            span { class: "font-normal text-base-content/70", {format!(" {}%", percent(value, total))} }
                        }
                        span { class: "tabular-nums", {money(value, &currency)} }
                    }
                }
            }
            if let Some(category) = chosen {
                div { class: "border-t border-base-300 pt-2.5 flex flex-col gap-1.5",
                    span { class: "text-xs text-base-content/70",
                        {format!("{} · {}", category_label(category), tid!("charts-expense-count", count: drill.len() as i64))}
                    }
                    for e in drill.iter().take(shown) {
                        div { class: "grid grid-cols-[3.25rem_1fr_auto] gap-2 text-xs",
                            span { class: "text-base-content/70 tabular-nums", {short_date(&e.date)} }
                            Link {
                                class: "min-w-0 truncate link link-hover",
                                to: Route::PaymentPage { project_id: e.project_id, expense_id: e.id },
                                "{e.name}"
                            }
                            span { class: "tabular-nums", {money(e.amount, &currency)} }
                        }
                    }
                    if drill.len() > DRILL_PREVIEW {
                        button {
                            r#type: "button",
                            class: "self-start text-xs font-semibold text-primary",
                            onclick: move |_| expanded.set(!expanded()),
                            if expanded() { {tid!("charts-show-less")} } else { {tid!("charts-show-all", count: drill.len() as i64)} }
                        }
                    }
                }
            }
        }
    }
}

pub fn bucket_segments(active: Bucket, fits: [bool; 3]) -> Vec<Segment> {
    [(Bucket::Day, tid!("bucket-day")), (Bucket::Week, tid!("bucket-week")), (Bucket::Month, tid!("bucket-month"))]
        .into_iter()
        .zip(fits)
        .map(|((b, label), enabled)| Segment { label, active: b == active, enabled })
        .collect()
}

#[component]
pub fn SpendOverTimeCard(
    columns: Vec<Column>,
    bucket: Bucket,
    fits: [bool; 3],
    on_bucket: EventHandler<Bucket>,
    average: f64,
    currency: String,
    range: String,
) -> Element {
    rsx! {
        ChartCard { title: tid!("charts-spend-title"),
            Segmented {
                aria_label: tid!("charts-group-by"),
                segments: bucket_segments(bucket, fits),
                on_select: move |i: usize| on_bucket.call([Bucket::Day, Bucket::Week, Bucket::Month][i]),
            }
            ColumnChart {
                columns,
                color: "var(--color-primary)",
                average: Some((average, money(average, &currency))),
                aria_label: tid!("charts-spend-title"),
            }
            Note { text: range }
        }
    }
}

#[component]
pub fn CategoryOverTimeCard(
    categories: Vec<&'static str>,
    selected: &'static str,
    on_select: EventHandler<&'static str>,
    columns: Vec<Column>,
    bucket: Bucket,
    average: f64,
    currency: String,
    range: String,
) -> Element {
    let label = category_label(selected);
    let title = match bucket {
        Bucket::Day => tid!("charts-cat-title-day", category: label),
        Bucket::Week => tid!("charts-cat-title-week", category: label),
        Bucket::Month => tid!("charts-cat-title-month", category: label),
    };
    rsx! {
        ChartCard { title: title.clone(), description: tid!("charts-cat-desc"),
            div { class: "flex flex-wrap gap-1",
                for category in categories {
                    button {
                        r#type: "button",
                        aria_pressed: category == selected,
                        class: if category == selected { "flex items-center gap-1.5 text-xs font-semibold rounded-field bg-base-200 px-2 py-1" } else { "flex items-center gap-1.5 text-xs text-base-content/70 rounded-field px-2 py-1 hover:bg-base-200" },
                        onclick: move |_| on_select.call(category),
                        Swatch { color: category_color(category) }
                        {category_label(category)}
                    }
                }
            }
            ColumnChart {
                columns,
                color: category_color(selected).to_string(),
                average: Some((average, money(average, &currency))),
                aria_label: title,
            }
            Note { text: range }
        }
    }
}

#[component]
pub fn RunningTotalCard(
    values: Vec<f64>,
    labels: Vec<String>,
    bucket: Bucket,
    average: f64,
    currency: String,
    since: String,
) -> Element {
    let total = values.last().copied().unwrap_or(0.0);
    let avg = money(average, &currency);
    let per_bucket = match bucket {
        Bucket::Day => tid!("charts-avg-per-day", amount: avg),
        Bucket::Week => tid!("charts-avg-per-week", amount: avg),
        Bucket::Month => tid!("charts-avg-per-month", amount: avg),
    };
    rsx! {
        ChartCard { title: tid!("charts-running-title"), description: tid!("charts-running-desc", date: since),
            div { class: "flex justify-between items-baseline gap-2 text-xs",
                span { class: "text-[22px] font-extrabold font-display text-gradient-brand leading-tight", {money(total, &currency)} }
                span { class: "text-base-content/70 tabular-nums text-right", "{per_bucket}" }
            }
            LineChart {
                values,
                labels,
                end_label: money(total, &currency),
                signed: false,
                aria_label: tid!("charts-running-title"),
            }
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct PersonView {
    pub name: String,
    pub initials: String,
    pub color_class: String,
    pub paid: f64,
    pub share: f64,
    pub is_me: bool,
}

#[component]
pub fn PaidVsShareCard(people: Vec<PersonView>, currency: String) -> Element {
    let max = people.iter().flat_map(|p| [p.paid, p.share]).fold(0.0, f64::max).max(f64::MIN_POSITIVE);
    rsx! {
        ChartCard { title: tid!("charts-people-title"), description: tid!("charts-people-desc"),
            div { class: "flex flex-wrap gap-x-3 gap-y-1 text-xs text-base-content/70",
                span { class: "inline-flex items-center gap-1.5", Swatch { color: "var(--color-primary)" } {tid!("charts-paid")} }
                span { class: "inline-flex items-center gap-1.5", Swatch { color: "color-mix(in oklab, var(--color-base-content) 30%, transparent)" } {tid!("charts-fair-share")} }
            }
            div { class: "flex flex-col gap-3",
                for p in people {
                    {
                        let net = p.paid - p.share;
                        rsx! {
                            div { class: "grid grid-cols-[2rem_1fr_auto] items-center gap-2.5",
                                Avatar { initials: p.initials.clone(), color_class: p.color_class.clone() }
                                div { class: "flex flex-col gap-[3px] min-w-0",
                                    span { class: "flex justify-between gap-1.5 text-xs font-semibold min-w-0",
                                        span { class: "truncate",
                                            "{p.name}"
                                            if p.is_me {
                                                span { class: "font-normal text-base-content/70", {format!(" {}", tid!("charts-you"))} }
                                            }
                                        }
                                        span { class: "font-normal text-base-content/70 tabular-nums whitespace-nowrap",
                                            {format!("{} · {}", money(p.paid, &currency), money(p.share, &currency))}
                                        }
                                    }
                                    span { class: "h-1.5 rounded-r-[3px] bg-primary", style: "width: {p.paid / max * 100.0:.1}%;" }
                                    span { class: "h-1.5 rounded-r-[3px] bg-base-content/30", style: "width: {p.share / max * 100.0:.1}%;" }
                                }
                                span { class: "text-right text-xs leading-tight whitespace-nowrap",
                                    b { class: if net.round() >= 0.0 { "block text-sm text-success" } else { "block text-sm text-error" },
                                        {signed_money(net, &currency)}
                                    }
                                    span { class: "text-base-content/70",
                                        if net.round() >= 0.0 { {tid!("charts-net-more")} } else { {tid!("charts-net-less")} }
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

#[component]
pub fn BalanceCard(values: Vec<f64>, labels: Vec<String>, currency: String, range: String) -> Element {
    let end = values.last().copied().unwrap_or(0.0);
    rsx! {
        ChartCard { title: tid!("charts-balance-title"), description: tid!("charts-balance-desc"),
            LineChart {
                values,
                labels,
                end_label: signed_money(end, &currency),
                signed: true,
                aria_label: tid!("charts-balance-title"),
            }
            div { class: "flex flex-wrap gap-x-3 gap-y-1 text-xs text-base-content/70",
                span { class: "inline-flex items-center gap-1.5", Swatch { color: "var(--color-success)" } {tid!("charts-owed")} }
                span { class: "inline-flex items-center gap-1.5", Swatch { color: "var(--color-error)" } {tid!("charts-owe")} }
            }
            Note { text: range }
        }
    }
}

#[component]
pub fn ProjectTotalsCard(groups: Vec<CurrencyTotals>) -> Element {
    rsx! {
        ChartCard { title: tid!("charts-projects-title"), description: tid!("charts-projects-desc"),
            for (i , group) in groups.into_iter().enumerate() {
                {
                    let max = group.rows.iter().map(|r| r.1).fold(0.0, f64::max).max(f64::MIN_POSITIVE);
                    rsx! {
                        div { class: if i == 0 { "flex flex-col gap-2" } else { "flex flex-col gap-2 border-t border-base-300 pt-2.5" },
                            span { class: "text-xs font-semibold text-base-content/70", "{group.currency}" }
                            for (name , total) in group.rows.iter() {
                                div { class: "grid grid-cols-[6.5rem_1fr] items-center gap-2 text-[11px]",
                                    span { class: "truncate font-medium", "{name}" }
                                    span { class: "flex items-center gap-1.5 min-w-0",
                                        span { class: "h-4 rounded-r bg-primary shrink-0", style: "width: max(2px, {total / max * 72.0:.1}%);" }
                                        span { class: "tabular-nums whitespace-nowrap text-xs font-bold", {money(*total, &group.currency)} }
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
