use dioxus::prelude::*;
use crate::tid;
use shared::Account;

use super::use_friends::use_friends;
use crate::common::DropdownButton;
use crate::icons::{BellIcon, ICON_HEADER};

/// The unread notifications — today only incoming friend requests — behind a bell with a count.
/// Renders nothing for an anonymous user.
#[component]
pub fn NotificationsBell() -> Element {
    let auth_ctx = use_context::<Signal<Option<Account>>>();
    let friends = use_friends();

    if auth_ctx().is_none() {
        return rsx! {};
    }

    let incoming = match &*friends.lists.read() {
        Some(Ok(lists)) => lists.incoming.clone(),
        _ => vec![],
    };
    let count = incoming.len();

    rsx! {
        DropdownButton {
            id: "notifications",
            label: tid!("notifications-label"),
            close_on_select: false,
            menu_class: "list bg-base-100 rounded-box w-72 shadow absolute right-0 top-full mt-1 z-50",
            trigger: rsx! {
                div { class: "indicator",
                    if count > 0 {
                        span { class: "indicator-item badge badge-primary badge-xs", "{count}" }
                    }
                    BellIcon { size: ICON_HEADER }
                }
            },
            li { class: "p-3 text-sm font-semibold", {tid!("notifications-title")} }
            if let Some(err) = (friends.error)() {
                li { class: "p-3 text-sm text-error", "{err}" }
            }
            if incoming.is_empty() {
                li { class: "p-3 text-sm text-base-content/70", {tid!("notifications-empty")} }
            } else {
                for req in incoming.into_iter() {
                    {
                        let id = req.id;
                        rsx! {
                            li { class: "list-row items-center",
                                div { class: "list-col-grow flex flex-col min-w-0",
                                    span { class: "font-medium truncate", "{req.requester_email}" }
                                    span { class: "text-xs text-base-content/70", {tid!("notifications-friend-request")} }
                                }
                                div { class: "flex gap-1",
                                    button {
                                        r#type: "button",
                                        class: "btn btn-primary btn-xs",
                                        disabled: (friends.busy)(),
                                        onclick: move |_| friends.accept(id),
                                        {tid!("friends-accept")}
                                    }
                                    button {
                                        r#type: "button",
                                        class: "btn btn-ghost btn-xs",
                                        disabled: (friends.busy)(),
                                        onclick: move |_| friends.decline_or_withdraw(id),
                                        {tid!("friends-decline")}
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
