use dioxus::prelude::*;
use crate::tid;

use crate::common::{haptic, use_modal_focus, Haptic};
use crate::expenses::helpers::guest_invites::guest_names_line;
use crate::icons::{CloseIcon, LinkIcon, LockIcon, ShieldIcon, UsersIcon, ICON_HEADER, ICON_INLINE};

/// The participants who may never have received the project, with the two ways to send it.
#[component]
pub fn GuestsCard(names: Vec<String>, on_later: EventHandler<()>, on_send: EventHandler<()>) -> Element {
    rsx! {
        div { id: "guests-hint", role: "status", class: "alert alert-warning alert-soft text-sm text-base-content! flex flex-col gap-2",
            div { class: "flex items-start gap-2 w-full min-w-0",
                span { class: "shrink-0 flex pt-0.5 text-warning", UsersIcon { size: ICON_INLINE } }
                span { class: "flex flex-col min-w-0",
                    span { class: "font-semibold", {tid!("guests-hint-title", count: names.len() as i64)} }
                    span { class: "text-base-content/70 truncate", {guest_names_line(&names)} }
                }
            }
            div { class: "flex justify-end gap-2 w-full",
                button {
                    id: "guests-hint-later",
                    r#type: "button",
                    class: "btn btn-ghost btn-sm",
                    onclick: move |_| {
                        haptic(Haptic::Light);
                        on_later.call(());
                    },
                    {tid!("guests-hint-later")}
                }
                button {
                    id: "guests-hint-send",
                    r#type: "button",
                    class: "btn btn-primary btn-sm",
                    onclick: move |_| {
                        haptic(Haptic::Light);
                        on_send.call(());
                    },
                    {tid!("guests-hint-send")}
                }
            }
        }
    }
}

/// The encrypted invitation first, for friends with an account; the project link second, with what
/// sharing it gives away. `on_invite` is `None` signed out, where there are no friends to invite.
#[component]
pub fn SendToGuestsSheet(
    on_invite: Option<EventHandler<()>>,
    on_share_link: EventHandler<()>,
    on_close: EventHandler<()>,
) -> Element {
    use_modal_focus("guests-send-sheet");
    let pick = move |action: EventHandler<()>| {
        move |_| {
            haptic(Haptic::Light);
            on_close.call(());
            action.call(());
        }
    };

    rsx! {
        div {
            id: "guests-send-sheet",
            class: "modal modal-open modal-bottom sm:modal-middle",
            role: "dialog",
            aria_modal: "true",
            aria_labelledby: "guests-send-title",
            tabindex: "-1",
            div { class: "modal-box max-w-md p-0",
                div { class: "flex flex-col gap-4 px-6 pt-5 pb-6",
                div { class: "flex items-center gap-2",
                    h2 { id: "guests-send-title", class: "flex-1 min-w-0 font-display font-bold text-lg", {tid!("guests-send-title")} }
                    button {
                        r#type: "button",
                        class: "btn btn-circle size-11 min-h-11 border-0 bg-base-200 text-base-content/80",
                        aria_label: tid!("close"),
                        onclick: move |_| on_close.call(()),
                        CloseIcon { size: ICON_HEADER }
                    }
                }
                if let Some(on_invite) = on_invite {
                    button {
                        id: "guests-send-friend",
                        r#type: "button",
                        class: "flex items-start gap-3 w-full text-left rounded-box border-2 border-primary bg-primary/10 p-4",
                        onclick: pick(on_invite),
                        span { class: "shrink-0 flex text-primary", LockIcon { size: ICON_HEADER } }
                        span { class: "flex flex-col gap-1 min-w-0",
                            span { class: "flex flex-wrap items-center gap-2 font-semibold",
                                {tid!("guests-send-friend")}
                                span { class: "badge badge-soft badge-success badge-sm", {tid!("guests-send-friend-badge")} }
                            }
                            span { class: "text-xs text-base-content/70 leading-snug", {tid!("guests-send-friend-hint")} }
                        }
                    }
                }
                button {
                    id: "guests-send-link",
                    r#type: "button",
                    class: "flex items-start gap-3 w-full text-left rounded-box bg-base-200 p-4",
                    onclick: pick(on_share_link),
                    span { class: "shrink-0 flex", LinkIcon { size: ICON_HEADER } }
                    span { class: "flex flex-col gap-1 min-w-0",
                        span { class: "font-semibold", {tid!("guests-send-link")} }
                        span { class: "text-xs text-base-content/70 leading-snug", {tid!("guests-send-link-hint")} }
                    }
                }
                p { class: "alert alert-warning alert-soft text-xs text-base-content! items-start",
                    span { class: "shrink-0 flex text-warning", ShieldIcon { size: ICON_INLINE } }
                    {tid!("guests-send-link-warning")}
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
        signed_in: bool,
        calls: Rc<RefCell<Vec<&'static str>>>,
    }

    #[component]
    fn Harness(props: HarnessProps) -> Element {
        crate::i18n::use_test_i18n();
        let log = |name: &'static str| {
            let calls = props.calls.clone();
            EventHandler::new(move |_: ()| calls.borrow_mut().push(name))
        };
        rsx! {
            SendToGuestsSheet {
                on_invite: props.signed_in.then(|| log("invite")),
                on_share_link: log("link"),
                on_close: log("close"),
            }
        }
    }

    fn shown(signed_in: bool) -> Vec<String> {
        let calls = Rc::new(RefCell::new(Vec::new()));
        let mut dom = VirtualDom::new_with_props(Harness, HarnessProps { signed_in, calls });
        texts(&dom.rebuild_to_vec())
    }

    #[test]
    fn the_friend_invitation_is_offered_only_signed_in() {
        assert!(shown(true).iter().any(|t| t == "Invite a Counted friend"));
        assert!(!shown(false).iter().any(|t| t == "Invite a Counted friend"));
        assert!(shown(false).iter().any(|t| t == "Share the project link"));
    }

    #[test]
    fn the_link_choice_says_what_sharing_it_gives_away() {
        assert!(shown(false).iter().any(|t| t.starts_with("Anyone with this link can read and edit")));
    }

    #[test]
    fn a_choice_closes_the_sheet_before_it_acts() {
        let calls = Rc::new(RefCell::new(Vec::new()));
        let mut dom = VirtualDom::new_with_props(Harness, HarnessProps { signed_in: true, calls: calls.clone() });
        for id in listener_ids(&dom.rebuild_to_vec(), "click") {
            calls.borrow_mut().clear();
            dom.runtime().handle_event("click", click(), id);
            if calls.borrow().contains(&"invite") {
                assert_eq!(*calls.borrow(), vec!["close", "invite"]);
                return;
            }
        }
        panic!("no click target reached on_invite");
    }
}
