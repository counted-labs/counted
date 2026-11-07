use dioxus::prelude::*;
use crate::tid;
use shared::ProjectStatus;

use crate::common::{haptic, status_transitions, Haptic};
use crate::expenses::group_security::{GroupSecurity, GroupSecuritySection};
use crate::icons::{
    ArchiveIcon, CloseIcon, DownloadIcon, HistoryIcon, LockIcon, LogOutIcon, MoreIcon, PencilIcon,
    RepeatIcon, ShareIcon, SmartphoneIcon, UndoIcon, UserPlusIcon, WalletIcon, ICON_HEADER,
};

#[derive(PartialEq, Props, Clone)]
pub struct ProjectActionsSheetProps {
    pub title: String,
    pub status: ProjectStatus,
    pub read_only: bool,
    pub native_app_link: Option<String>,
    pub on_close: EventHandler<()>,
    /// `None` in the demo: a shared demo link would land in the recipient's real store.
    pub on_share: Option<EventHandler<()>>,
    pub on_invite: Option<EventHandler<()>>,
    pub on_history: EventHandler<()>,
    pub on_recurring: EventHandler<()>,
    pub on_edit: EventHandler<()>,
    pub on_status: EventHandler<ProjectStatus>,
    pub on_export_csv: EventHandler<()>,
    pub on_export_json: EventHandler<()>,
    pub on_leave: EventHandler<()>,
    /// R4, on a group when the account has a personal project: whether my share counts there.
    #[props(default)]
    pub counted: Option<bool>,
    #[props(default)]
    pub on_counted: Option<EventHandler<bool>>,
    /// R5, on the personal project: the groups it counts.
    #[props(default)]
    pub on_counted_groups: Option<EventHandler<()>>,
    /// The admin actions (docs/plans/key-lifecycle.md §8.7), in whichever state applies.
    #[props(default)]
    pub security: Option<GroupSecurity>,
}

/// The project page's actions, as a bottom sheet. Every entry closes the sheet, then calls out.
#[component]
pub fn ProjectActionsSheet(props: ProjectActionsSheetProps) -> Element {
    crate::common::use_modal_focus("project-actions-sheet");
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
            id: "project-actions-sheet",
            class: "modal modal-open modal-bottom sm:modal-middle",
            role: "dialog",
            aria_modal: "true",
            aria_labelledby: "project-actions-title",
            tabindex: "-1",
            div { class: "modal-box max-w-md p-0",
                div { class: "flex flex-col gap-4 px-6 pt-5 pb-6",
                div { class: "flex items-center gap-2",
                    h2 {
                        id: "project-actions-title",
                        class: "flex-1 min-w-0 truncate font-bold text-lg font-display",
                        "{props.title}"
                    }
                    button {
                        id: "project-actions-close",
                        r#type: "button",
                        class: "btn btn-circle size-11 min-h-11 border-0 bg-base-200 text-base-content/80",
                        aria_label: tid!("close"),
                        onclick: move |_| on_close.call(()),
                        CloseIcon {}
                    }
                }
                div { class: "flex gap-2",
                    if let Some(on_share) = props.on_share {
                        Tile {
                            id: "project-share-btn",
                            label: tid!("share-link"),
                            onclick: pick(on_share),
                            ShareIcon { size: 24 }
                        }
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
                if let (Some(counted), Some(on_counted)) = (props.counted, props.on_counted) {
                    div { class: "bg-base-200 rounded-box overflow-hidden",
                        label { class: "flex items-center gap-3 min-h-16 px-4 py-2 cursor-pointer",
                            WalletIcon { size: ICON_HEADER }
                            span { class: "flex-1 min-w-0",
                                span { class: "block font-medium", {tid!("counted-switch")} }
                                span { class: "block text-xs text-base-content/70 leading-snug",
                                    if counted { {tid!("counted-switch-on")} } else { {tid!("counted-switch-off")} }
                                }
                            }
                            input {
                                id: "counted-switch",
                                r#type: "checkbox",
                                role: "switch",
                                class: "toggle toggle-primary",
                                checked: counted,
                                onchange: move |e| on_counted.call(e.checked()),
                            }
                        }
                    }
                }
                if let Some(on_counted_groups) = props.on_counted_groups {
                    div { class: "bg-base-200 rounded-box overflow-hidden",
                        Row {
                            id: "counted-groups-item",
                            label: tid!("counted-groups"),
                            onclick: pick(on_counted_groups),
                            WalletIcon { size: ICON_HEADER }
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
                if let Some(security) = props.security.clone() {
                    GroupSecuritySection { security, on_close }
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

const ROW_CLASS: &str = "flex w-full items-center gap-3 min-h-13 px-4 text-left font-medium row-press";

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

pub(crate) const GROUP_CLASS: &str = "bg-base-200 rounded-box overflow-hidden divide-y divide-base-300";

/// The round ⋯ in a page header that opens a [`MenuSheet`]. The sheet goes beside the header,
/// never inside it: a scrolled header has a `backdrop-filter`, which traps a `position: fixed`
/// sheet in the bar.
#[component]
pub(crate) fn MoreButton(id: &'static str, label: String, open: Signal<bool>) -> Element {
    let mut open = open;
    rsx! {
        button {
            id,
            r#type: "button",
            class: "btn btn-circle size-11 min-h-11 border-0 bg-base-300 glass-button",
            aria_label: "{label}",
            aria_haspopup: "dialog",
            aria_expanded: open(),
            onclick: move |_| {
                haptic(Haptic::Light);
                open.set(true);
            },
            MoreIcon { size: 24 }
        }
    }
}

/// A titled bottom sheet of [`Row`] groups, the shell of the project sheet without its tiles.
#[component]
pub(crate) fn MenuSheet(id: &'static str, title: String, on_close: EventHandler<()>, children: Element) -> Element {
    crate::common::use_modal_focus(id);
    rsx! {
        div {
            id,
            class: "modal modal-open modal-bottom sm:modal-middle",
            role: "dialog",
            aria_modal: "true",
            aria_labelledby: "{id}-title",
            tabindex: "-1",
            div { class: "modal-box max-w-md p-0",
                div { class: "flex flex-col gap-4 px-6 pt-5 pb-6",
                    div { class: "flex items-center gap-2",
                        h2 {
                            id: "{id}-title",
                            class: "flex-1 min-w-0 truncate font-bold text-lg font-display",
                            "{title}"
                        }
                        button {
                            r#type: "button",
                            class: "btn btn-circle size-11 min-h-11 border-0 bg-base-200 text-base-content/80",
                            aria_label: tid!("close"),
                            onclick: move |_| on_close.call(()),
                            CloseIcon {}
                        }
                    }
                    {children}
                }
            }
            div { class: "modal-backdrop", onclick: move |_| on_close.call(()) }
        }
    }
}

#[component]
pub(crate) fn Row(
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
                on_share: Some(EventHandler::new(log("share"))),
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
