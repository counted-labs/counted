use dioxus::prelude::*;
use crate::tid;
use shared::ProjectStatus;

use super::DropdownItem;

/// Every status change offered in a project's dropdown, as (current status, target status,
/// translation key, daisyUI variant). Both the list page and the details page render this same
/// set; they differ only in what they do with the chosen target.
const TRANSITIONS: &[(ProjectStatus, ProjectStatus, &str, &str)] = &[
    (ProjectStatus::Ongoing, ProjectStatus::Closed, "project-close", "warning"),
    (ProjectStatus::Ongoing, ProjectStatus::Archived, "project-archive", "neutral"),
    (ProjectStatus::Closed, ProjectStatus::Ongoing, "project-reopen", "success"),
    (ProjectStatus::Closed, ProjectStatus::Archived, "project-archive", "neutral"),
    (ProjectStatus::Archived, ProjectStatus::Ongoing, "project-unarchive", "success"),
];

#[derive(PartialEq, Props, Clone)]
pub struct ProjectStatusItemsProps {
    pub status: ProjectStatus,
    /// Called with the target status when an item is picked.
    pub on_apply: EventHandler<ProjectStatus>,
}

/// The status-change entries of a project dropdown. Renders nothing for a status with no
/// transitions. Meant to sit among other `DropdownItem`s inside a `DropdownButton`.
#[component]
pub fn ProjectStatusItems(props: ProjectStatusItemsProps) -> Element {
    rsx! {
        for (from , to , key , variant) in TRANSITIONS.iter() {
            if *from == props.status {
                DropdownItem {
                    variant: *variant,
                    label: tid!(key),
                    onclick: move |_| props.on_apply.call(to.clone()),
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::test_dom::listener_ids;

    /// `tid!` here is keyed off the table, not a literal, so the crate-wide scanner in `i18n` does
    /// not see these. Without this a typo would ship as a raw key in the dropdown.
    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn every_transition_label_is_a_known_message() {
        let known = crate::i18n::locale_ids(crate::i18n::FALLBACK);
        for (_, _, key, _) in TRANSITIONS {
            assert!(known.contains(*key), "{key} is not defined in the fallback locale");
        }
    }

    use dioxus::dioxus_core::VirtualDom;
    use dioxus_html::{set_event_converter, PlatformEventData, SerializedHtmlEventConverter,
        SerializedMouseData};
    use std::any::Any;
    use std::cell::RefCell;
    use std::rc::Rc;
    use std::sync::Once;

    #[derive(Props, Clone, PartialEq)]
    struct HarnessProps {
        status: ProjectStatus,
        applied: Rc<RefCell<Vec<ProjectStatus>>>,
    }

    #[component]
    fn Harness(props: HarnessProps) -> Element {
        // The items translate their labels, and `tid!` panics with no provider above it.
        crate::i18n::use_test_i18n();
        let applied = props.applied.clone();
        rsx! {
            ul {
                ProjectStatusItems {
                    status: props.status.clone(),
                    on_apply: move |s| applied.borrow_mut().push(s),
                }
            }
        }
    }

    fn click() -> Event<dyn Any> {
        static CONVERTER: Once = Once::new();
        CONVERTER.call_once(|| set_event_converter(Box::new(SerializedHtmlEventConverter)));
        let data = PlatformEventData::new(Box::new(SerializedMouseData::default()));
        Event::new(Rc::new(data) as Rc<dyn Any>, true)
    }

    /// Renders the menu for `status` and returns (item count, statuses applied by clicking
    /// item `click_index`).
    fn render(status: ProjectStatus, click_index: usize) -> (usize, Vec<ProjectStatus>) {
        let applied = Rc::new(RefCell::new(Vec::new()));
        let mut dom =
            VirtualDom::new_with_props(Harness, HarnessProps { status, applied: applied.clone() });
        let ids = listener_ids(&dom.rebuild_to_vec(), "click");
        if let Some(id) = ids.get(click_index) {
            dom.runtime().handle_event("click", click(), *id);
        }
        let out = applied.borrow().clone();
        (ids.len(), out)
    }

    #[test]
    fn ongoing_offers_close_then_archive() {
        let (count, applied) = render(ProjectStatus::Ongoing, 0);
        assert_eq!(count, 2, "Ongoing offers Clôturer and Archiver");
        assert_eq!(applied, vec![ProjectStatus::Closed]);

        let (_, applied) = render(ProjectStatus::Ongoing, 1);
        assert_eq!(applied, vec![ProjectStatus::Archived]);
    }

    #[test]
    fn closed_offers_reopen_then_archive() {
        let (count, applied) = render(ProjectStatus::Closed, 0);
        assert_eq!(count, 2, "Closed offers Réouvrir and Archiver");
        assert_eq!(applied, vec![ProjectStatus::Ongoing]);

        let (_, applied) = render(ProjectStatus::Closed, 1);
        assert_eq!(applied, vec![ProjectStatus::Archived]);
    }

    #[test]
    fn archived_offers_unarchive_only() {
        let (count, applied) = render(ProjectStatus::Archived, 0);
        assert_eq!(count, 1, "Archived offers only Désarchiver");
        assert_eq!(applied, vec![ProjectStatus::Ongoing]);
    }
}
