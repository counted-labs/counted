use dioxus::prelude::*;
use crate::tid;
use shared::Account;

use super::use_friends::Friends;
use crate::common::{haptic, Haptic};
use crate::expenses::project_actions_sheet::MenuSheet;
use crate::icons::{BellIcon, ICON_HEADER};

/// The unread notifications - today only incoming friend requests - behind a bell with a count.
/// Renders nothing for an anonymous user. Its sheet is [`NotificationsSheet`], rendered beside the
/// page header, never in it: the sticky bar is a stacking context under the dock (and once
/// scrolled a containing block), so a sheet inside it would sit under the dock.
#[component]
pub fn NotificationsBell(friends: Friends, open: Signal<bool>) -> Element {
    let auth_ctx = use_context::<Signal<Option<Account>>>();
    let mut open = open;

    if auth_ctx().is_none() {
        return rsx! {};
    }

    let count = match &*friends.lists.read() {
        Some(Ok(lists)) => lists.incoming.len(),
        _ => 0,
    };

    rsx! {
        button {
            id: "notifications",
            r#type: "button",
            class: "btn btn-ghost btn-circle size-11 min-h-11",
            aria_label: tid!("notifications-label"),
            aria_haspopup: "dialog",
            aria_expanded: open(),
            onclick: move |_| {
                haptic(Haptic::Light);
                open.set(true);
            },
            div { class: "indicator",
                if count > 0 {
                    span { class: "indicator-item badge badge-primary badge-xs", "{count}" }
                }
                BellIcon { size: ICON_HEADER }
            }
        }
    }
}

#[component]
pub fn NotificationsSheet(friends: Friends, open: Signal<bool>) -> Element {
    let mut open = open;
    let incoming = match &*friends.lists.read() {
        Some(Ok(lists)) => lists.incoming.clone(),
        _ => vec![],
    };

    rsx! {
        if open() {
            MenuSheet { id: "notifications-sheet", title: tid!("notifications-title"), on_close: move |_| open.set(false),
                if let Some(err) = (friends.error)() {
                    p { role: "alert", class: "text-sm text-error", "{err}" }
                }
                if incoming.is_empty() {
                    p { class: "text-sm text-base-content/70", {tid!("notifications-empty")} }
                } else {
                    ul { class: "list bg-base-200 rounded-box",
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
                                                class: "btn btn-primary btn-sm",
                                                disabled: (friends.busy)(),
                                                onclick: move |_| friends.accept(id),
                                                {tid!("friends-accept")}
                                            }
                                            button {
                                                r#type: "button",
                                                class: "btn btn-ghost btn-sm",
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
    }
}
