//! "Mes dépenses" on the personal project's page: the month and what it adds up to (R1, R2), the
//! search pill (R1) and the groups it counts (R5).

use chrono::{Datelike, NaiveDate};
use dioxus::prelude::*;
use std::collections::HashSet;
use uuid::Uuid;

use crate::common::{month_title, ProjectTile};
use crate::expenses::helpers::my_spending::{shift_month, GroupLedger, MonthTotals};

/// A group as the R5 sheet lists it.
#[derive(Clone, Debug, PartialEq)]
pub struct CountedGroup {
    pub project_id: Uuid,
    pub name: String,
    pub emoji: Option<String>,
    pub members: usize,
    pub currency: String,
}

impl From<&GroupLedger> for CountedGroup {
    fn from(l: &GroupLedger) -> Self {
        Self { project_id: l.project_id, name: l.name.clone(), emoji: l.emoji.clone(), members: l.members, currency: l.data.currency.clone() }
    }
}
use crate::icons::{BackArrowIcon, ChevronRightIcon, CloseIcon, LockIcon, SearchIcon, ICON_HEADER};
use crate::tid;

fn month_label(month: NaiveDate) -> String {
    format!("{} {}", month_title(month.month0() as usize), month.year())
}

#[component]
pub fn MonthSummary(month: Signal<NaiveDate>, totals: MonthTotals, currency: String) -> Element {
    let spent = totals.spent();
    let personal_pct = if spent > 0.0 { totals.personal / spent * 100.0 } else { 0.0 };
    let balance = totals.balance();
    rsx! {
        div { class: "flex items-center justify-between",
            button {
                id: "month-prev",
                r#type: "button",
                class: "btn btn-ghost btn-sm w-11",
                aria_label: tid!("my-spending-prev-month"),
                onclick: move |_| month.set(shift_month(month(), -1)),
                BackArrowIcon {}
            }
            span { id: "month-label", class: "font-semibold", "{month_label(month())}" }
            button {
                id: "month-next",
                r#type: "button",
                class: "btn btn-ghost btn-sm w-11",
                aria_label: tid!("my-spending-next-month"),
                onclick: move |_| month.set(shift_month(month(), 1)),
                ChevronRightIcon {}
            }
        }
        section { id: "month-summary", class: "rounded-[20px] bg-base-100 shadow-soft p-4 flex flex-col",
            span { class: "text-[0.8125rem] text-base-content/70", {tid!("my-spending-spent")} }
            span { id: "month-spent", class: "font-display font-bold text-[1.75rem] tabular-nums mt-0.5 mb-2.5", {crate::common::amount(spent, &currency)} }
            span { class: "flex h-2.5 rounded-full overflow-hidden gap-0.5 bg-base-200",
                if totals.personal > 0.0 {
                    span { class: "bg-primary", style: "width: {personal_pct}%;" }
                }
                if totals.groups > 0.0 {
                    span { class: "bg-accent/60 flex-1" }
                }
            }
            div { class: "flex gap-4 mt-2.5 text-xs text-base-content/80",
                span { class: "flex items-center gap-1.5",
                    span { class: "size-2.5 rounded-[3px] bg-primary" }
                    {tid!("personal-badge")}
                    strong { id: "month-personal", class: "tabular-nums", {crate::common::amount(totals.personal, &currency)} }
                }
                span { class: "flex items-center gap-1.5",
                    span { class: "size-2.5 rounded-[3px] bg-accent/60" }
                    {tid!("my-spending-groups")}
                    strong { id: "month-groups", class: "tabular-nums", {crate::common::amount(totals.groups, &currency)} }
                }
            }
            if totals.income > 0.0 {
                div { class: "flex justify-between items-baseline gap-2 mt-3 pt-2.5 border-t border-base-200 text-[0.8125rem] text-base-content/80",
                    span {
                        {tid!("my-spending-income")}
                        " "
                        strong { id: "month-income", class: "text-base-content tabular-nums", {crate::common::amount(totals.income, &currency)} }
                    }
                    span {
                        {tid!("my-spending-balance")}
                        " "
                        strong {
                            id: "month-balance",
                            class: "tabular-nums",
                            class: if balance >= 0.0 { "text-success" } else { "text-error" },
                            {crate::common::signed_amount(balance, &currency)}
                        }
                    }
                }
            }
            if totals.unconverted > 0 {
                p { class: "mt-2 text-xs text-base-content/60", {tid!("my-spending-unconverted", count: totals.unconverted as i64)} }
            }
        }
    }
}

#[component]
pub fn SearchPill(onclick: EventHandler<()>) -> Element {
    rsx! {
        button {
            id: "my-spending-search",
            r#type: "button",
            class: "h-10 rounded-full border border-base-300 bg-base-100 flex items-center gap-2 px-3.5 text-base-content/70 text-left",
            onclick: move |_| onclick.call(()),
            SearchIcon { size: 18 }
            {tid!("expenses-search-open")}
        }
    }
}

/// R5: every group this device holds, each with its switch.
#[component]
pub fn CountedGroupsSheet(
    groups: Vec<CountedGroup>,
    uncounted: HashSet<Uuid>,
    currency: String,
    on_toggle: EventHandler<(Uuid, bool)>,
    on_close: EventHandler<()>,
) -> Element {
    crate::common::use_modal_focus("counted-groups-sheet");
    rsx! {
        div {
            id: "counted-groups-sheet",
            class: "modal modal-open modal-bottom sm:modal-middle",
            role: "dialog",
            aria_modal: "true",
            aria_labelledby: "counted-groups-title",
            tabindex: "-1",
            div { class: "modal-box max-w-md flex flex-col gap-4",
                div { class: "flex items-center gap-2",
                    h2 { id: "counted-groups-title", class: "flex-1 min-w-0 truncate font-display font-bold text-lg", {tid!("counted-groups")} }
                    button {
                        r#type: "button",
                        class: "btn btn-circle size-11 min-h-11 border-0 bg-base-200 text-base-content/80",
                        aria_label: tid!("close"),
                        onclick: move |_| on_close.call(()),
                        CloseIcon { size: ICON_HEADER }
                    }
                }
                p { class: "-mt-2 text-sm text-base-content/70", {tid!("counted-groups-intro")} }
                if groups.is_empty() {
                    p { class: "text-sm text-base-content/70", {tid!("counted-groups-empty")} }
                }
                ul { class: "flex flex-col divide-y divide-base-200",
                    for ledger in groups {
                        {
                            let id = ledger.project_id;
                            let counted = !uncounted.contains(&id);
                            let detail = if ledger.currency == currency {
                                format!("{} · {}", tid!("move-members", count: ledger.members as i64), ledger.currency)
                            } else {
                                format!(
                                    "{} · {}",
                                    tid!("move-members", count: ledger.members as i64),
                                    tid!("counted-groups-converted", from: ledger.currency.clone(), to: currency.clone()),
                                )
                            };
                            rsx! {
                                li { key: "{id}",
                                    label { class: "flex items-center gap-3 py-2.5 cursor-pointer",
                                        ProjectTile { name: ledger.name.clone(), id, emoji: ledger.emoji.clone() }
                                        span { class: "flex-1 min-w-0 flex flex-col gap-0.5",
                                            span { class: "font-semibold truncate", "{ledger.name}" }
                                            span { class: "text-xs text-base-content/70", "{detail}" }
                                        }
                                        input {
                                            id: "counted-group-{id}",
                                            r#type: "checkbox",
                                            role: "switch",
                                            class: "toggle toggle-primary",
                                            checked: counted,
                                            onchange: move |e| on_toggle.call((id, e.checked())),
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                p { class: "flex gap-2 items-start text-xs text-base-content/70",
                    span { class: "shrink-0 flex", LockIcon { size: 15 } }
                    {tid!("counted-groups-encrypted")}
                }
                button {
                    id: "counted-groups-done",
                    r#type: "button",
                    class: "btn btn-primary w-full",
                    onclick: move |_| on_close.call(()),
                    {tid!("counted-groups-done")}
                }
            }
            div { class: "modal-backdrop", onclick: move |_| on_close.call(()) }
        }
    }
}
