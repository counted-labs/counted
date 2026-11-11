use dioxus::prelude::*;
use crate::tid;
use shared::{Account, IncomingFriendRequest};

use super::friend_sheets::{changed_friend_avatar, AddFriendSheet, FriendSheet, FriendsSheet, IncomingRow, OwnCodeSheet};
use super::friends_service::{notify_address, FriendRow, OutgoingRow};
use super::use_friends::use_friends;
use crate::common::{
    account_color_class, email_initials, haptic, AppHeader, AuthResolved, Avatar, CheckFriend, ConfirmModal, GroupedListSkeleton,
    Haptic, ListRow, PullToRefresh, RowTile, SizeClass,
};
use crate::expenses::project_actions_sheet::Row;
use crate::icons::{CheckIcon, CloseIcon, SendIcon, ShieldIcon, UserPlusIcon, UsersIcon, ICON_HEADER};
use crate::route::Route;

#[derive(Clone, Copy, PartialEq)]
enum Tab {
    Friends,
    Requests,
}

#[derive(Clone, PartialEq)]
enum Open {
    Friend(FriendRow),
    OwnCode,
    Outgoing(OutgoingRow),
    Incoming(IncomingFriendRequest),
}

/// Friends in two tabs (docs/plans/friends-settings-redesign.md, track B). A changed code sits above
/// the tabs, never behind one; every row opens on a tap, a long press being only a shortcut.
#[component]
pub fn FriendsPage() -> Element {
    let nav = use_navigator();
    let auth_ctx = use_context::<Signal<Option<Account>>>();
    let auth_resolved = use_context::<Signal<AuthResolved>>();
    let friends = use_friends();
    let mut tab = use_signal(|| Tab::Friends);
    let mut adding = use_signal(|| false);
    let mut open = use_signal(|| None::<Open>);
    let mut removing = use_signal(|| None::<FriendRow>);

    // A sheet shows the row it was opened on. If a reload changes that row (a new key served, the
    // friend removed elsewhere), the sheet closes rather than show or pin something stale.
    use_effect(move || {
        let current = friends.current();
        let stale = match &*open.peek() {
            Some(Open::Friend(row)) => current.as_ref().is_none_or(|l| !l.friends.contains(row)),
            Some(Open::OwnCode) => current.as_ref().is_none_or(|l| l.my_fingerprint.is_none()),
            Some(Open::Outgoing(out)) => current.as_ref().is_none_or(|l| !l.outgoing.contains(out)),
            Some(Open::Incoming(req)) => current.as_ref().is_none_or(|l| !l.incoming.contains(req)),
            None => false,
        };
        if stale {
            open.set(None);
        }
        if removing.peek().as_ref().is_some_and(|row| current.as_ref().is_none_or(|l| !l.friends.contains(row))) {
            removing.set(None);
        }
    });

    let mut check_friend = use_context::<Signal<CheckFriend>>();
    use_effect(move || {
        let Some(id) = check_friend().0 else { return };
        let Some(lists) = friends.current() else { return };
        if let Some(row) = lists.friends.into_iter().find(|f| f.id == id) {
            open.set(Some(Open::Friend(row)));
        }
        check_friend.set(CheckFriend(None));
    });

    let signed_in = auth_resolved().0 && auth_ctx().is_some();
    let lists = friends.current();
    let load_error = match &*friends.lists.read() {
        Some(Err(e)) => Some(crate::common::error_message(e)),
        _ => None,
    };
    let load_failed = load_error.is_some();
    let changed: Vec<FriendRow> = lists.iter().flat_map(|l| &l.friends).filter(|f| f.key_changed).cloned().collect();
    let settled: Vec<FriendRow> = lists.iter().flat_map(|l| &l.friends).filter(|f| !f.key_changed).cloned().collect();
    let incoming = lists.as_ref().map_or(0, |l| l.incoming.len());

    rsx! {
        div { class: "container app-container bg-base-100 p-4 pb-24 max-w-md mx-auto flex flex-col gap-4 overflow-auto",
            PullToRefresh { on_refresh: move |_| friends.reload(), busy: (friends.busy)() }
            AppHeader { title: tid!("friends-title"), back_button_route: Route::SettingsPage {},
                if signed_in {
                    button {
                        id: "friends-add-open",
                        r#type: "button",
                        class: "btn btn-circle size-11 min-h-11 border-0 bg-primary/10 text-secondary-content",
                        aria_label: tid!("friends-add-title"),
                        aria_haspopup: "dialog",
                        onclick: move |_| {
                            haptic(Haptic::Light);
                            adding.set(true);
                        },
                        UserPlusIcon { size: ICON_HEADER }
                    }
                }
            }

            if !auth_resolved().0 {
                GroupedListSkeleton { rows: 3 }
            } else if auth_ctx().is_none() {
                div { class: "flex flex-col shrink-0 rounded-box bg-base-100 p-4 gap-2",
                    h2 { class: "font-display font-bold text-lg", {tid!("settings-anonymous-title")} }
                    p { class: "text-sm text-base-content/70", {tid!("friends-anonymous-body")} }
                    div { class: "flex justify-end",
                        button {
                            r#type: "button",
                            class: "btn btn-primary",
                            onclick: move |_| { nav.push(Route::LoginPage {}); },
                            {tid!("login-submit")}
                        }
                    }
                }
            } else {
                if let Some(err) = (friends.error)().filter(|_| !adding()) {
                    div { role: "alert", class: "alert alert-error text-sm", "{err}" }
                }
                if let Some(err) = load_error {
                    div { role: "alert", class: "alert alert-error text-sm", "{err}" }
                }
                            if !changed.is_empty() {
                                section { id: "friends-to-check", class: "flex flex-col",
                                    h2 { class: "list-header", {tid!("friends-to-check")} }
                                    ul { class: "list-group", style: "--separator-inset: 4.25rem",
                                        for row in changed {
                                            {
                                                let opened = row.clone();
                                                rsx! {
                                                    ListRow {
                                                        key: "{row.id}",
                                                        title: row.email.clone(),
                                                        title_class: "font-medium break-all",
                                                        subtitle: tid!("friends-code-changed-row"),
                                                        subtitle_class: "text-error font-medium",
                                                        chevron: true,
                                                        long_press: format!("long-press-changed-{}", row.id),
                                                        onclick: move |_| open.set(Some(Open::Friend(opened.clone()))),
                                                        {changed_friend_avatar(&row)}
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }

                            div { role: "tablist", class: "tabs tabs-box bg-base-300 shadow-none grid grid-cols-2",
                                button {
                                    id: "friends-tab-friends",
                                    r#type: "button",
                                    role: "tab",
                                    aria_controls: "friends-panel",
                                    aria_selected: tab() == Tab::Friends,
                                    class: if tab() == Tab::Friends { "tab tab-active font-semibold" } else { "tab text-base-content/70" },
                                    onclick: move |_| {
                                        haptic(Haptic::Selection);
                                        tab.set(Tab::Friends);
                                    },
                                    {tid!("friends-title")}
                                }
                                button {
                                    id: "friends-tab-requests",
                                    r#type: "button",
                                    role: "tab",
                                    aria_controls: "friends-panel",
                                    aria_selected: tab() == Tab::Requests,
                                    class: if tab() == Tab::Requests { "tab tab-active font-semibold gap-1.5" } else { "tab text-base-content/70 gap-1.5" },
                                    onclick: move |_| {
                                        haptic(Haptic::Selection);
                                        tab.set(Tab::Requests);
                                    },
                                    {tid!("friends-incoming-title")}
                                    if incoming > 0 {
                                        span { class: "badge badge-xs badge-soft badge-primary font-semibold tabular-nums", "{incoming}" }
                                    }
                                }
                            }

                            div {
                                id: "friends-panel",
                                role: "tabpanel",
                                aria_labelledby: if tab() == Tab::Friends { "friends-tab-friends" } else { "friends-tab-requests" },
                                class: "flex flex-col gap-4",
                            match lists {
                            None if load_failed => rsx! {},
                            None => rsx! { GroupedListSkeleton { rows: 3 } },
                            Some(lists) => rsx! {
                            if tab() == Tab::Friends {
                                if lists.key_mismatch {
                                    div { id: "friends-key-mismatch", role: "alert", class: "alert alert-error alert-soft text-sm flex flex-col items-start gap-1",
                                        span { class: "font-semibold", {tid!("friends-code-hidden")} }
                                        span { {tid!("friends-code-hidden-body")} }
                                    }
                                } else if let Some(code) = lists.my_fingerprint.clone() {
                                    ul { class: "list-group",
                                        ListRow {
                                            id: "friends-own-code",
                                            title: tid!("friends-own-code"),
                                            subtitle: code,
                                            subtitle_class: "font-mono text-base-content/70 tracking-wide",
                                            chevron: true,
                                            onclick: move |_| open.set(Some(Open::OwnCode)),
                                            RowTile { tint: "bg-tile-green text-tile-green-content", ShieldIcon {} }
                                        }
                                    }
                                }
                                if lists.friends.is_empty() {
                                    div { id: "friends-empty", class: "flex flex-col items-center text-center gap-3 pt-10",
                                        span { class: "size-20 rounded-3xl grid place-items-center bg-primary/10 text-secondary-content", UsersIcon { size: 40 } }
                                        h2 { class: "font-display font-bold text-lg", {tid!("friends-empty-title")} }
                                        p { class: "text-sm text-base-content/70 max-w-64", {tid!("friends-empty-body")} }
                                        button {
                                            r#type: "button",
                                            class: "btn btn-primary",
                                            aria_haspopup: "dialog",
                                            onclick: move |_| {
                                                haptic(Haptic::Light);
                                                adding.set(true);
                                            },
                                            UserPlusIcon { size: ICON_HEADER }
                                            {tid!("friends-add-title")}
                                        }
                                    }
                                } else if !settled.is_empty() {
                                    section { class: "flex flex-col",
                                        h2 { class: "list-header flex justify-between", span { {tid!("friends-list-title")} } span { class: "tabular-nums", "{settled.len()}" } }
                                        ul { id: "friends-list", class: "list-group", style: "--separator-inset: 4.25rem",
                                            for row in settled {
                                                {
                                                    let opened = row.clone();
                                                    let (subtitle, subtitle_class) = match row.fingerprint.clone() {
                                                        Some(code) => (code, "font-mono text-base-content/70 tracking-wide"),
                                                        None => (tid!("friends-no-key"), "text-base-content/70"),
                                                    };
                                                    rsx! {
                                                        ListRow {
                                                            key: "{row.id}",
                                                            title: row.email.clone(),
                                                            title_class: "font-medium break-all",
                                                            subtitle,
                                                            subtitle_class,
                                                            chevron: true,
                                                            long_press: format!("long-press-friend-{}", row.id),
                                                            onclick: move |_| open.set(Some(Open::Friend(opened.clone()))),
                                                            Avatar { initials: email_initials(&row.email), color_class: account_color_class(row.account_id).to_string(), size: SizeClass::W10 }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            } else if lists.incoming.is_empty() && lists.outgoing.is_empty() {
                                div { id: "requests-empty", class: "flex flex-col items-center text-center gap-2 pt-12",
                                    h2 { class: "font-display font-bold text-lg", {tid!("friends-requests-empty-title")} }
                                    p { class: "text-sm text-base-content/70 max-w-64", {tid!("friends-requests-empty-body")} }
                                }
                            } else {
                                if !lists.incoming.is_empty() {
                                    section { class: "flex flex-col",
                                        h2 { class: "list-header", {tid!("friends-received")} }
                                        ul { id: "friends-incoming", class: "list-group", style: "--separator-inset: 4.25rem",
                                            for req in lists.incoming.clone() {
                                                IncomingRow { key: "{req.id}", req, friends, on_actions: move |req| open.set(Some(Open::Incoming(req))) }
                                            }
                                        }
                                    }
                                }
                                if !lists.outgoing.is_empty() {
                                    section { class: "flex flex-col",
                                        h2 { class: "list-header", {tid!("friends-outgoing-title")} }
                                        ul { id: "friends-outgoing", class: "list-group", style: "--separator-inset: 4.25rem",
                                            for out in lists.outgoing.clone() {
                                                {
                                                    let opened = out.clone();
                                                    let from_project = notify_address(&out.label).is_none();
                                                    rsx! {
                                                        ListRow {
                                                            key: "{out.id}",
                                                            title: out.label.clone(),
                                                            title_class: "font-medium break-all",
                                                            subtitle: if from_project { tid!("friends-from-project") } else { tid!("friends-pending") },
                                                            subtitle_class: "text-base-content/70",
                                                            chevron: true,
                                                            long_press: format!("long-press-outgoing-{}", out.id),
                                                            onclick: move |_| open.set(Some(Open::Outgoing(opened.clone()))),
                                                            Avatar { initials: email_initials(&out.label), size: SizeClass::W10, guest: true }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            },
                            }
                            }
            }

            if adding() {
                AddFriendSheet { friends, on_close: move |_| adding.set(false) }
            }

            match open() {
                Some(Open::Friend(row)) => rsx! {
                    FriendSheet {
                        row,
                        busy: (friends.busy)(),
                        on_close: move |_| open.set(None),
                        on_trust: move |row| friends.trust_new_key(row),
                        on_remove: move |row| {
                            open.set(None);
                            removing.set(Some(row));
                        },
                    }
                },
                Some(Open::OwnCode) => match friends.current().and_then(|l| l.my_fingerprint) {
                    Some(code) => rsx! { OwnCodeSheet { code, on_close: move |_| open.set(None) } },
                    None => rsx! {},
                },
                Some(Open::Outgoing(out)) => {
                    let address = notify_address(&out.label).map(str::to_string);
                    let id = out.id;
                    rsx! {
                        FriendsSheet { id: "outgoing-sheet", title: out.label.clone(), on_close: move |_| open.set(None),
                            p { class: "text-sm text-base-content/70 -mt-2", {tid!("friends-pending-sheet")} }
                            div { class: "bg-base-200 rounded-box overflow-hidden divide-y divide-base-300",
                            if let Some(address) = address {
                                Row {
                                    id: "row-action-notify",
                                    label: tid!("friends-notify"),
                                    onclick: move |_| {
                                        open.set(None);
                                        friends.notify(address.clone());
                                    },
                                    SendIcon { size: ICON_HEADER }
                                }
                            }
                            Row {
                                id: "row-action-withdraw",
                                label: tid!("friends-withdraw-row"),
                                danger: true,
                                onclick: move |_| {
                                    open.set(None);
                                    friends.decline_or_withdraw(id);
                                },
                                CloseIcon { size: ICON_HEADER }
                            }
                            }
                        }
                    }
                }
                Some(Open::Incoming(req)) => {
                    let id = req.id;
                    rsx! {
                        FriendsSheet { id: "incoming-sheet", title: req.requester_email.clone(), on_close: move |_| open.set(None),
                            div { class: "bg-base-200 rounded-box overflow-hidden divide-y divide-base-300",
                            Row {
                                id: "row-action-accept",
                                label: tid!("friends-accept"),
                                onclick: move |_| {
                                    open.set(None);
                                    friends.accept(id);
                                },
                                CheckIcon { size: ICON_HEADER }
                            }
                            Row {
                                id: "row-action-decline",
                                label: tid!("friends-decline"),
                                danger: true,
                                onclick: move |_| {
                                    open.set(None);
                                    friends.decline_or_withdraw(id);
                                },
                                CloseIcon { size: ICON_HEADER }
                            }
                            }
                        }
                    }
                }
                None => rsx! {},
            }

            if let Some(row) = removing() {
                ConfirmModal {
                    title: tid!("friends-remove-confirm-title"),
                    message: tid!("friends-remove-confirm-message", email: row.email.clone()),
                    confirm_label: tid!("friends-remove"),
                    action_sheet: true,
                    on_cancel: move |_| removing.set(None),
                    on_confirm: move |_| {
                        removing.set(None);
                        friends.remove(row.clone());
                    },
                }
            }
        }
    }
}
