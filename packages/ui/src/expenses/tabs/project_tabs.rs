use dioxus::prelude::*;
use crate::common::{haptic, Haptic};
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
    let active = props.active;
    let active_index = TABS.iter().position(|t| *t == active).unwrap_or(0);
    let segment_count = TABS.len();
    let on_select = props.on_select;
    let select = move |tab: Tab| {
        if tab != active {
            haptic(Haptic::Selection);
        }
        on_select.call(tab);
    };

    rsx! {
        // `segmented` (native-feel.css): a thumb under the selected segment slides to it by
        // `transform`, from the index below; daisyUI's own active fill steps aside for it.
        div { role: "tablist", class: "segmented tabs tabs-box bg-base-300 shadow-none grid grid-flow-col auto-cols-fr",
            style: "--segment-index: {active_index}; --segment-count: {segment_count}",
            span { class: "segmented-thumb", aria_hidden: "true" }
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
                        select(TABS[next]);
                        if let Some(button) = buttons.peek()[next].clone() {
                            spawn(async move {
                                let _ = button.set_focus(true).await;
                            });
                        }
                    },
                    class: if props.active == tab { "tab tab-active text-[0.8125rem] font-semibold" } else { "tab text-[0.8125rem] font-semibold text-base-content/70" },
                    onclick: move |_| select(tab),
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
