use dioxus::prelude::*;
use crate::tid;
use shared::ProjectStatus;

use crate::common::status_transitions;
use crate::icons::{
    ArchiveIcon, CloseIcon, DownloadIcon, HistoryIcon, LockIcon, LogOutIcon, PencilIcon, RepeatIcon,
    ShareIcon, SmartphoneIcon, UndoIcon, UserPlusIcon, ICON_HEADER,
};

#[derive(PartialEq, Props, Clone)]
pub struct ProjectActionsSheetProps {
    pub title: String,
    pub status: ProjectStatus,
    pub read_only: bool,
    pub native_app_link: Option<String>,
    pub on_close: EventHandler<()>,
    pub on_share: EventHandler<()>,
    pub on_invite: Option<EventHandler<()>>,
    pub on_history: EventHandler<()>,
    pub on_recurring: EventHandler<()>,
    pub on_edit: EventHandler<()>,
    pub on_status: EventHandler<ProjectStatus>,
    pub on_export_csv: EventHandler<()>,
    pub on_export_json: EventHandler<()>,
    pub on_leave: EventHandler<()>,
}

/// The project page's actions, as a bottom sheet. Every entry closes the sheet, then calls out.
#[component]
pub fn ProjectActionsSheet(props: ProjectActionsSheetProps) -> Element {
    let on_close = props.on_close;
    let on_status = props.on_status;
    let pick = move |action: EventHandler<()>| {
        move |_: ()| {
            on_close.call(());
            action.call(());
        }
    };
    let invite = props.on_invite.filter(|_| !props.read_only);

    rsx! {
        div {
            class: "modal modal-open modal-bottom sm:modal-middle",
            role: "dialog",
            aria_modal: "true",
            aria_labelledby: "project-actions-title",
            tabindex: "-1",
            onkeydown: move |e| {
                if e.key() == Key::Escape {
                    e.prevent_default();
                    on_close.call(());
                }
            },
            div { class: "modal-box max-w-md p-0",
                div { class: "flex flex-col gap-4 px-6 pt-5 pb-6",
                div { class: "flex items-center gap-2",
                    h3 {
                        id: "project-actions-title",
                        class: "flex-1 min-w-0 truncate font-bold text-lg font-display",
                        "{props.title}"
                    }
                    button {
                        id: "project-actions-close",
                        r#type: "button",
                        class: "btn btn-ghost btn-circle size-11",
                        aria_label: tid!("close"),
                        onclick: move |_| on_close.call(()),
                        CloseIcon {}
                    }
                }
                div { class: "flex gap-2",
                    Tile {
                        id: "project-share-btn",
                        label: tid!("share-link"),
                        onclick: pick(props.on_share),
                        ShareIcon { size: 24 }
                    }
                    if let Some(on_invite) = invite {
                        Tile {
                            id: "invite-friends-item",
                            label: tid!("project-sheet-invite"),
                            onclick: pick(on_invite),
                            UserPlusIcon { size: 24 }
                        }
                    }
                    Tile {
                        id: "project-history-btn",
                        label: tid!("project-history-title"),
                        onclick: pick(props.on_history),
                        HistoryIcon { size: 24 }
                    }
                    if !props.read_only {
                        Tile {
                            id: "recurring-menu-item",
                            label: tid!("project-sheet-recurring"),
                            onclick: pick(props.on_recurring),
                            RepeatIcon { size: 24 }
                        }
                    }
                }
                if !props.read_only {
                    div { class: "bg-base-200 rounded-box overflow-hidden divide-y divide-base-300",
                        Row {
                            id: "project-edit-item",
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
                div { class: "bg-base-200 rounded-box overflow-hidden divide-y divide-base-300",
                    Row {
                        label: tid!("export-csv"),
                        onclick: pick(props.on_export_csv),
                        DownloadIcon { size: ICON_HEADER }
                    }
                    Row {
                        label: tid!("export-json"),
                        onclick: pick(props.on_export_json),
                        DownloadIcon { size: ICON_HEADER }
                    }
                    if let Some(link) = props.native_app_link.clone() {
                        // Plain href so the OS resolves the scheme, and no click handler: closing
                        // the sheet would unmount the link before the browser follows it.
                        a { href: "{link}", class: ROW_CLASS,
                            SmartphoneIcon { size: ICON_HEADER }
                            {tid!("open-in-app")}
                        }
                    }
                }
                div { class: "bg-base-200 rounded-box overflow-hidden",
                    Row {
                        id: "project-leave-item",
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

const ROW_CLASS: &str = "flex w-full items-center gap-3 min-h-13 px-4 text-left font-medium";

#[component]
fn Tile(id: String, label: String, onclick: EventHandler<()>, children: Element) -> Element {
    rsx! {
        button {
            id,
            r#type: "button",
            class: "flex-1 min-w-0 h-20 rounded-box bg-primary/10 text-primary flex flex-col items-center justify-center gap-1",
            onclick: move |_| onclick.call(()),
            {children}
            span { class: "text-xs font-semibold truncate max-w-full px-1", "{label}" }
        }
    }
}

#[component]
fn Row(
    #[props(into, default)] id: Option<String>,
    label: String,
    #[props(default)] danger: bool,
    onclick: EventHandler<()>,
    children: Element,
) -> Element {
    rsx! {
        button {
            id,
            r#type: "button",
            class: "{ROW_CLASS}",
            class: if danger { "text-error" },
            onclick: move |_| onclick.call(()),
            {children}
            "{label}"
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
        read_only: bool,
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
            ProjectActionsSheet {
                title: "Trip",
                status: ProjectStatus::Ongoing,
                read_only: props.read_only,
                native_app_link: None,
                on_close: log("close"),
                on_share: log("share"),
                on_invite: Some(EventHandler::new(log("invite"))),
                on_history: log("history"),
                on_recurring: log("recurring"),
                on_edit: log("edit"),
                on_status: move |_| calls.borrow_mut().push("status"),
                on_export_csv: log("export-csv"),
                on_export_json: log("export-json"),
                on_leave: log("leave"),
            }
        }
    }

    #[test]
    fn read_only_hides_every_editing_action() {
        let calls = Rc::new(RefCell::new(Vec::new()));
        let mut dom = VirtualDom::new_with_props(Harness, HarnessProps { read_only: true, calls });
        let shown = texts(&dom.rebuild_to_vec());
        for hidden in [
            "Edit project",
            "Recurring",
            "Invite",
            "Close project",
            "Archive project",
        ] {
            assert!(!shown.iter().any(|t| t == hidden), "{hidden} is shown on a read-only project");
        }
        for kept in ["Leave project", "Export CSV", "Export JSON", "Share", "History"] {
            assert!(shown.iter().any(|t| t == kept), "{kept} is missing on a read-only project");
        }
    }

    #[test]
    fn an_action_closes_the_sheet_then_runs() {
        let calls = Rc::new(RefCell::new(Vec::new()));
        let mut dom = VirtualDom::new_with_props(
            Harness,
            HarnessProps { read_only: false, calls: calls.clone() },
        );
        for id in listener_ids(&dom.rebuild_to_vec(), "click") {
            calls.borrow_mut().clear();
            dom.runtime().handle_event("click", click(), id);
            if calls.borrow().contains(&"edit") {
                assert_eq!(*calls.borrow(), vec!["close", "edit"]);
                return;
            }
        }
        panic!("no click target reached on_edit");
    }
}
