use dioxus::prelude::*;
use crate::tid;
use std::rc::Rc;

#[derive(PartialEq, Clone, Copy)]
pub enum Tab {
    Expenses,
    Balance,
    Reimbursements,
}

const TABS: [Tab; 3] = [Tab::Expenses, Tab::Balance, Tab::Reimbursements];

/// The e2e suite selects the tabs by id - these strings are part of the contract.
pub fn tab_id(t: Tab) -> &'static str {
    match t {
        Tab::Expenses => "tab-expenses",
        Tab::Balance => "tab-balance",
        Tab::Reimbursements => "tab-reimbursements",
    }
}

/// Translation keys, not labels - the render site translates.
fn tab_label(t: Tab) -> &'static str {
    match t {
        Tab::Expenses => "tab-expenses",
        Tab::Balance => "tab-balance",
        Tab::Reimbursements => "tab-reimbursements",
    }
}

pub const TABPANEL_ID: &str = "project-tabpanel";

/// Where an arrow, Home or End key moves the selection, wrapping as in the WAI-ARIA tabs pattern.
fn index_after_key(index: usize, key: &Key) -> Option<usize> {
    let last = TABS.len() - 1;
    match key {
        Key::ArrowRight => Some(if index == last { 0 } else { index + 1 }),
        Key::ArrowLeft => Some(if index == 0 { last } else { index - 1 }),
        Key::Home => Some(0),
        Key::End => Some(last),
        _ => None,
    }
}

#[derive(PartialEq, Props, Clone)]
pub struct ProjectTabsProps {
    pub active: Tab,
    pub on_select: EventHandler<Tab>,
}

#[component]
pub fn ProjectTabs(props: ProjectTabsProps) -> Element {
    let mut buttons: Signal<[Option<Rc<MountedData>>; 3]> = use_signal(|| [None, None, None]);

    rsx! {
        div { role: "tablist", class: "tabs tabs-box bg-base-300 shadow-none grid grid-flow-col auto-cols-fr",
            for (index, tab) in TABS.into_iter().enumerate() {
                button {
                    id: tab_id(tab),
                    role: "tab",
                    aria_selected: props.active == tab,
                    aria_controls: TABPANEL_ID,
                    tabindex: if props.active == tab { "0" } else { "-1" },
                    onmounted: move |e| buttons.write()[index] = Some(e.data()),
                    onkeydown: move |e| {
                        let Some(next) = index_after_key(index, &e.key()) else { return };
                        e.prevent_default();
                        props.on_select.call(TABS[next]);
                        if let Some(button) = buttons.peek()[next].clone() {
                            spawn(async move {
                                let _ = button.set_focus(true).await;
                            });
                        }
                    },
                    class: if props.active == tab { "tab tab-active text-[13px] font-semibold" } else { "tab text-[13px] font-semibold text-base-content/70" },
                    onclick: move |_| props.on_select.call(tab),
                    {tid!(tab_label(tab))}
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn arrows_wrap_and_home_end_jump() {
        assert_eq!(index_after_key(0, &Key::ArrowRight), Some(1));
        assert_eq!(index_after_key(2, &Key::ArrowRight), Some(0));
        assert_eq!(index_after_key(0, &Key::ArrowLeft), Some(2));
        assert_eq!(index_after_key(1, &Key::Home), Some(0));
        assert_eq!(index_after_key(0, &Key::End), Some(2));
        assert_eq!(index_after_key(1, &Key::Enter), None);
    }
}
