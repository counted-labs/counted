//! The projects page's money (J1, J3, P4): "On vous doit / Vous devez" with the people behind it,
//! and what each group row says under and beside its name.

use chrono::{Datelike, NaiveDate};
use dioxus::prelude::*;
use uuid::Uuid;

use crate::common::{amount, initials, month_abbrev, signed_amount, user_color_class, Avatar};
use crate::icons::{CheckIcon, ChevronRightIcon};
use crate::projects::home_balances::{Debt, ProjectGlance, Totals};
use crate::tid;

const OWED: &str = "text-owed";
const OWE: &str = "text-owe";

#[component]
pub fn BalanceCard(
    totals: Totals,
    currency: String,
    debts: Vec<Debt>,
    on_pay: EventHandler<Uuid>,
    on_remind: EventHandler<Debt>,
) -> Element {
    let mut open = use_signal(|| false);
    let approx = if totals.converted { "≈ " } else { "" };
    let toggle_label = if open() { tid!("home-details-hide") } else { tid!("home-details-show") };
    rsx! {
        section { id: "home-balances", class: "shrink-0 rounded-[20px] bg-base-100 shadow-soft overflow-hidden",
            div { class: "grid grid-cols-2",
                div { class: "flex flex-col items-center gap-0.5 text-center px-3 pt-4 pb-3.5",
                    span { class: "text-[13px] text-base-content/70", {tid!("home-owed-to-you")} }
                    span { id: "home-owed-to-you", class: "font-display font-bold text-[22px] tabular-nums {OWED}",
                        "{approx}{amount(totals.owed_to_me, &currency)}"
                    }
                }
                div { class: "flex flex-col items-center gap-0.5 text-center px-3 pt-4 pb-3.5 border-l border-base-200",
                    span { class: "text-[13px] text-base-content/70", {tid!("home-you-owe")} }
                    span { id: "home-you-owe", class: "font-display font-bold text-[22px] tabular-nums {OWE}",
                        "{approx}{amount(totals.i_owe, &currency)}"
                    }
                }
            }
            if totals.left_out {
                p { class: "text-xs text-center text-base-content/60 px-3 pb-2", {tid!("home-totals-partial")} }
            }
            if open() && !debts.is_empty() {
                ul { id: "home-debts", class: "border-t border-base-200 bg-well flex flex-col py-1",
                    for debt in debts.iter().cloned() {
                        DebtRow { key: "{debt.project_id}-{debt.person_id}", debt, on_pay, on_remind }
                    }
                }
            }
            if debts.is_empty() {
                if totals.settled() {
                    p { id: "home-all-settled", class: "border-t border-base-200 h-11 flex items-center justify-center gap-1.5 text-[13px] font-semibold text-secondary-content",
                        CheckIcon { size: 16 }
                        {tid!("home-all-settled")}
                    }
                }
            } else {
                button {
                    id: "home-details-toggle",
                    r#type: "button",
                    class: "w-full h-11 border-t border-base-200 flex items-center justify-center gap-1.5 text-[13px] font-semibold text-secondary-content",
                    aria_expanded: open(),
                    aria_controls: "home-debts",
                    onclick: move |_| open.toggle(),
                    {toggle_label}
                    span { class: if open() { "flex -rotate-90" } else { "flex rotate-90" }, ChevronRightIcon { size: 16 } }
                }
            }
        }
    }
}

#[component]
fn DebtRow(debt: Debt, on_pay: EventHandler<Uuid>, on_remind: EventHandler<Debt>) -> Element {
    let tone = if debt.owed_to_me { OWED } else { OWE };
    let project_id = debt.project_id;
    let remind = debt.clone();
    rsx! {
        li { class: "flex items-center gap-2.5 py-2 pl-4 pr-2.5",
            Avatar { initials: initials(&debt.person), color_class: user_color_class(debt.person_id).to_string() }
            span { class: "flex-1 min-w-0 flex flex-col",
                span { class: "text-sm truncate",
                    "{debt.person} · "
                    strong { class: "{tone} tabular-nums", {amount(debt.amount, &debt.currency)} }
                }
                span { class: "text-xs text-base-content/70 truncate", "{debt.project_name}" }
            }
            if debt.owed_to_me {
                button {
                    id: "home-remind-{project_id}-{debt.person_id}",
                    r#type: "button",
                    class: "btn btn-sm h-9 min-h-9 rounded-full border-0 bg-secondary text-secondary-content px-3.5",
                    onclick: move |_| on_remind.call(remind.clone()),
                    {tid!("home-remind")}
                }
            } else {
                button {
                    id: "home-pay-{project_id}-{debt.person_id}",
                    r#type: "button",
                    class: "btn btn-sm h-9 min-h-9 rounded-full border-0 btn-primary px-3.5",
                    onclick: move |_| on_pay.call(project_id),
                    {tid!("home-pay")}
                }
            }
        }
    }
}

/// "Aujourd'hui", "Hier", or "2 oct.".
pub fn activity_label(date: NaiveDate, today: NaiveDate) -> String {
    match (today - date).num_days() {
        0 => tid!("date-today"),
        1 => tid!("date-yesterday"),
        _ => format!("{} {}", date.day(), month_abbrev(date.month0() as usize)),
    }
}

/// The line under a group's name: last activity and member count, as far as this device knows them.
pub fn row_subtitle(glance: &ProjectGlance, today: NaiveDate) -> String {
    let mut parts = Vec::new();
    if let Some(date) = glance.last_activity {
        parts.push(activity_label(date, today));
    }
    parts.push(tid!("move-members", count: glance.members as i64));
    parts.join(" · ")
}

/// The right side of a group's row: my balance and what it means, or "À jour".
#[component]
pub fn RowBalance(balance: f64, currency: String) -> Element {
    if balance.abs() < 0.005 {
        return rsx! {
            span { class: "shrink-0 flex items-center gap-1 text-sm text-base-content/70",
                CheckIcon { size: 16 }
                {tid!("home-row-settled")}
            }
        };
    }
    let (tone, label) = if balance > 0.0 { (OWED, tid!("home-row-owed")) } else { (OWE, tid!("home-row-owe")) };
    rsx! {
        span { class: "shrink-0 flex flex-col items-end gap-px",
            span { class: "font-bold text-[15px] tabular-nums {tone}", {signed_amount(balance, &currency)} }
            span { class: "text-[11px] text-base-content/70", "{label}" }
        }
    }
}
