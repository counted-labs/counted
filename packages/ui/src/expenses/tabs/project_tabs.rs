use dioxus::prelude::*;
use crate::tid;

#[derive(PartialEq, Clone, Copy)]
pub enum Tab {
    Expenses,
    Balance,
    Reimbursements,
}

const TABS: [Tab; 3] = [Tab::Expenses, Tab::Balance, Tab::Reimbursements];

/// The e2e suite selects the tabs by id — these strings are part of the contract.
fn tab_id(t: Tab) -> &'static str {
    match t {
        Tab::Expenses => "tab-expenses",
        Tab::Balance => "tab-balance",
        Tab::Reimbursements => "tab-reimbursements",
    }
}

/// Translation keys, not labels — the render site translates.
fn tab_label(t: Tab) -> &'static str {
    match t {
        Tab::Expenses => "tab-expenses",
        Tab::Balance => "tab-balance",
        Tab::Reimbursements => "tab-reimbursements",
    }
}

#[derive(PartialEq, Props, Clone)]
pub struct ProjectTabsProps {
    pub active: Tab,
    pub on_select: EventHandler<Tab>,
}

#[component]
pub fn ProjectTabs(props: ProjectTabsProps) -> Element {
    rsx! {
        div { role: "tablist", class: "tabs tabs-box justify-center shadow-soft",
            for tab in TABS {
                button {
                    id: tab_id(tab),
                    role: "tab",
                    aria_selected: props.active == tab,
                    class: if props.active == tab { "tab tab-active text-xs font-semibold" } else { "tab text-xs" },
                    onclick: move |_| props.on_select.call(tab),
                    {tid!(tab_label(tab))}
                }
            }
        }
    }
}
