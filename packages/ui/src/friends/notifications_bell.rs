use dioxus::prelude::*;
use crate::tid;
use shared::Account;

use super::friend_sheets::{changed_friend_avatar, IncomingRow};
use super::use_friends::Friends;
use crate::common::{format_date, haptic, CheckFriend, Haptic, ListRow};
use crate::expenses::project_actions_sheet::MenuSheet;
use crate::icons::{BellIcon, ICON_HEADER};
use crate::route::Route;

/// What waits on the account: incoming friend requests and friends whose code changed, both worked
/// out on this device from the friends list. Renders nothing for an anonymous user. Its sheet is
/// [`NotificationsSheet`], rendered beside the page header, never in it: the sticky bar is a
/// stacking context under the dock (and once scrolled a containing block), so a sheet inside it
/// would sit under the dock.
#[component]
pub fn NotificationsBell(friends: Friends, open: Signal<bool>) -> Element {
    let auth_ctx = use_context::<Signal<Option<Account>>>();
    let mut open = open;

    if auth_ctx().is_none() {
        return rsx! {};
    }

    let count = friends.current().map(|l| l.incoming.len() + l.changed_count()).unwrap_or(0);

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
    let nav = use_navigator();
    let mut check_friend = use_context::<Signal<CheckFriend>>();
    let mut open = open;
    let (incoming, changed) = friends
        .current()
        .map(|l| (l.incoming.clone(), l.friends.into_iter().filter(|f| f.key_changed).collect::<Vec<_>>()))
        .unwrap_or_default();

    rsx! {
        if open() {
            MenuSheet { id: "notifications-sheet", title: tid!("notifications-title"), on_close: move |_| open.set(false),
                if let Some(err) = (friends.error)() {
                    p { role: "alert", class: "text-sm text-error", "{err}" }
                }
                if incoming.is_empty() && changed.is_empty() {
                    p { class: "text-sm text-base-content/70", {tid!("notifications-empty")} }
                }
                if !changed.is_empty() {
                    ul { class: "list-group", style: "--separator-inset: 4.25rem; background-color: var(--color-base-200)",
                        for row in changed {
                            ListRow {
                                key: "{row.id}",
                                title: row.email.clone(),
                                title_class: "font-medium break-all",
                                subtitle: tid!("friends-code-changed-row"),
                                subtitle_class: "text-error font-medium",
                                chevron: true,
                                onclick: move |_| {
                                    open.set(false);
                                    check_friend.set(CheckFriend(Some(row.id)));
                                    nav.push(Route::FriendsPage {});
                                },
                                {changed_friend_avatar(&row)}
                            }
                        }
                    }
                }
                if !incoming.is_empty() {
                    ul { class: "list-group", style: "--separator-inset: 4.25rem; background-color: var(--color-base-200)",
                        for req in incoming {
                            IncomingRow {
                                key: "{req.id}",
                                subtitle: tid!("notifications-friend-request-on", date: format_date(req.created_at.date())),
                                req,
                                friends,
                            }
                        }
                    }
                }
                button {
                    id: "notifications-manage-friends",
                    r#type: "button",
                    class: "btn btn-ghost text-secondary-content",
                    onclick: move |_| {
                        open.set(false);
                        nav.push(Route::FriendsPage {});
                    },
                    {tid!("notifications-manage-friends")}
                }
            }
        }
    }
}
