use dioxus::prelude::*;
use crate::tid;
use shared::ProjectStatus;

use crate::common::status_transitions;
use crate::expenses::project_actions_sheet::Row;
use crate::icons::{ArchiveIcon, CloseIcon, LockIcon, LogOutIcon, PencilIcon, UndoIcon, ICON_HEADER};

#[derive(PartialEq, Props, Clone)]
pub struct ProjectRowSheetProps {
    pub title: String,
    /// No key on this device: Unlock takes Edit's place, and the status changes are hidden since
    /// each writes an encrypted history entry.
    pub locked: bool,
    pub status: ProjectStatus,
    pub on_close: EventHandler<()>,
    pub on_edit: EventHandler<()>,
    pub on_unlock: EventHandler<()>,
    pub on_status: EventHandler<ProjectStatus>,
    pub on_leave: EventHandler<()>,
}

/// A projects-list row's actions, as a bottom sheet. Every entry closes the sheet, then calls out.
#[component]
pub fn ProjectRowSheet(props: ProjectRowSheetProps) -> Element {
    crate::common::use_modal_focus("project-row-sheet");
    let on_close = props.on_close;
    let on_status = props.on_status;
    let pick = move |action: EventHandler<()>| {
        move |_: ()| {
            on_close.call(());
            action.call(());
        }
    };

    rsx! {
        div {
            id: "project-row-sheet",
            class: "modal modal-open modal-bottom sm:modal-middle",
            role: "dialog",
            aria_modal: "true",
            aria_labelledby: "project-row-sheet-title",
            tabindex: "-1",
            div { class: "modal-box max-w-md p-0",
                div { class: "flex flex-col gap-4 px-6 pt-5 pb-6",
                    div { class: "flex items-center gap-2",
                        h2 {
                            id: "project-row-sheet-title",
                            class: "flex-1 min-w-0 truncate font-bold text-lg font-display",
                            "{props.title}"
                        }
                        button {
                            id: "project-row-sheet-close",
                            r#type: "button",
                            class: "btn btn-circle size-11 min-h-11 border-0 bg-base-200 text-base-content/80",
                            aria_label: tid!("close"),
                            onclick: move |_| on_close.call(()),
                            CloseIcon {}
                        }
                    }
                    div { class: "bg-base-200 rounded-box overflow-hidden divide-y divide-base-300",
                        if props.locked {
                            Row {
                                id: "project-row-unlock",
                                label: tid!("project-unlock"),
                                onclick: pick(props.on_unlock),
                                LockIcon { size: ICON_HEADER }
                            }
                        } else {
                            Row {
                                id: "project-row-edit",
                                label: tid!("project-sheet-edit"),
                                onclick: pick(props.on_edit),
                                PencilIcon { size: ICON_HEADER }
                            }
                            for (to , key) in status_transitions(&props.status) {
                                Row {
                                    label: tid!(key),
                                    onclick: {
                                        let to = to.clone();
                                        move |_: ()| {
                                            on_close.call(());
                                            on_status.call(to.clone());
                                        }
                                    },
                                    {
                                        match to {
                                            ProjectStatus::Closed => rsx! { LockIcon { size: ICON_HEADER } },
                                            ProjectStatus::Archived => rsx! { ArchiveIcon { size: ICON_HEADER } },
                                            ProjectStatus::Ongoing => rsx! { UndoIcon { size: ICON_HEADER } },
                                        }
                                    }
                                }
                            }
                        }
                    }
                    div { class: "bg-base-200 rounded-box overflow-hidden",
                        Row {
                            id: "project-row-leave",
                            label: tid!("project-sheet-leave"),
                            danger: true,
                            onclick: pick(props.on_leave),
                            LogOutIcon { size: ICON_HEADER }
                        }
                    }
                }
            }
            div { class: "modal-backdrop", onclick: move |_| on_close.call(()) }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::test_dom::{click, listener_ids, texts};
    use dioxus::dioxus_core::VirtualDom;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[derive(Props, Clone, PartialEq)]
    struct HarnessProps {
        locked: bool,
        calls: Rc<RefCell<Vec<&'static str>>>,
    }

    #[component]
    fn Harness(props: HarnessProps) -> Element {
        crate::i18n::use_test_i18n();
        let log = |name: &'static str| {
            let calls = props.calls.clone();
            move |_: ()| calls.borrow_mut().push(name)
        };
        let calls = props.calls.clone();
        rsx! {
            ProjectRowSheet {
                title: "Trip",
                locked: props.locked,
                status: ProjectStatus::Ongoing,
                on_close: log("close"),
                on_edit: log("edit"),
                on_unlock: log("unlock"),
                on_status: move |_| calls.borrow_mut().push("status"),
                on_leave: log("leave"),
            }
        }
    }

    fn render(locked: bool) -> Vec<String> {
        let calls = Rc::new(RefCell::new(Vec::new()));
        let mut dom = VirtualDom::new_with_props(Harness, HarnessProps { locked, calls });
        texts(&dom.rebuild_to_vec())
    }

    #[test]
    fn a_locked_row_offers_unlock_and_leave_only() {
        let shown = render(true);
        for hidden in ["Edit project", "Close project", "Archive project"] {
            assert!(!shown.iter().any(|t| t == hidden), "{hidden} is shown on a locked row");
        }
        for kept in ["Unlock", "Leave project"] {
            assert!(shown.iter().any(|t| t == kept), "{kept} is missing on a locked row");
        }
    }

    #[test]
    fn an_unlocked_row_offers_edit_the_status_changes_and_leave() {
        let shown = render(false);
        assert!(!shown.iter().any(|t| t == "Unlock"), "Unlock is shown on an unlocked row");
        for kept in ["Edit project", "Close project", "Archive project", "Leave project"] {
            assert!(shown.iter().any(|t| t == kept), "{kept} is missing on an unlocked row");
        }
    }

    #[test]
    fn an_action_closes_the_sheet_then_runs() {
        let calls = Rc::new(RefCell::new(Vec::new()));
        let mut dom =
            VirtualDom::new_with_props(Harness, HarnessProps { locked: false, calls: calls.clone() });
        for id in listener_ids(&dom.rebuild_to_vec(), "click") {
            calls.borrow_mut().clear();
            dom.runtime().handle_event("click", click(), id);
            if calls.borrow().contains(&"leave") {
                assert_eq!(*calls.borrow(), vec!["close", "leave"]);
                return;
            }
        }
        panic!("no click target reached on_leave");
    }
}
