use dioxus::prelude::*;
use crate::tid;
use shared::Account;

use super::friends_service::{notify_address, FriendRow};
use super::use_friends::use_friends;
use crate::common::{haptic, AppHeader, AuthResolved, ConfirmModal, Haptic, PullToRefresh, RowActionsSheet};
use crate::expenses::project_actions_sheet::Row;
use crate::icons::{SendIcon, ShieldIcon, TrashIcon, ICON_HEADER, ICON_INLINE};
use crate::route::Route;

#[component]
pub fn FriendsPage() -> Element {
    let nav = use_navigator();
    let auth_ctx = use_context::<Signal<Option<Account>>>();
    let auth_resolved = use_context::<Signal<AuthResolved>>();
    let mut friends = use_friends();
    let mut removing = use_signal(|| None::<FriendRow>);
    let mut actions_for = use_signal(|| None::<FriendRow>);

    rsx! {
        div { class: "container app-container bg-base-100 p-4 pb-24 max-w-md mx-auto flex flex-col gap-4 overflow-auto",
            PullToRefresh { on_refresh: move |_| friends.reload(), busy: (friends.busy)() }
            AppHeader { title: tid!("friends-title"), back_button_route: Route::SettingsPage {} }

            if !auth_resolved().0 {
                div { class: "skeleton h-48 w-full", aria_hidden: "true" }
            } else if auth_ctx().is_none() {
                div { class: "card bg-base-100 shadow-soft",
                    div { class: "card-body gap-2",
                        h2 { class: "card-title text-base", {tid!("settings-anonymous-title")} }
                        p { class: "text-sm text-base-content/70", {tid!("friends-anonymous-body")} }
                        div { class: "card-actions justify-end",
                            button {
                                r#type: "button",
                                class: "btn btn-primary",
                                onclick: move |_| { nav.push(Route::LoginPage {}); },
                                {tid!("login-submit")}
                            }
                        }
                    }
                }
            } else {
                div { class: "card bg-base-100 shadow-soft",
                    form {
                        class: "card-body gap-3",
                        onsubmit: move |e| {
                            e.prevent_default();
                            friends.add_by_email();
                        },
                        h2 { class: "card-title text-base", {tid!("friends-add-title")} }
                        p { class: "text-sm text-base-content/70", {tid!("friends-add-hint")} }
                        div { class: "join w-full",
                            input {
                                id: "friend-email",
                                r#type: "email",
                                class: "input join-item flex-1",
                                placeholder: tid!("field-email-placeholder"),
                                autocomplete: "off",
                                required: true,
                                value: "{friends.email}",
                                oninput: move |e| friends.email.set(e.value()),
                            }
                            button {
                                id: "friend-add",
                                r#type: "submit",
                                class: "btn btn-primary join-item",
                                disabled: (friends.busy)(),
                                {tid!("friends-add-button")}
                            }
                        }
                        if let Some(err) = (friends.error)() {
                            div { role: "alert", class: "alert alert-error text-sm", "{err}" }
                        }
                    }
                }

                if let Some(sent) = (friends.sent_to)() {
                    div { id: "friend-request-sent", role: "status", class: "alert alert-success alert-soft flex flex-col items-start gap-2",
                        span { class: "flex flex-col min-w-0 max-w-full text-sm",
                            span { class: "font-semibold", {tid!("friends-request-sent")} }
                            bdi { class: "break-all", "{sent}" }
                        }
                        button {
                            id: "friend-notify",
                            r#type: "button",
                            class: "btn btn-primary btn-sm",
                            onclick: move |_| friends.notify(sent.clone()),
                            SendIcon { size: ICON_INLINE }
                            {tid!("friends-notify")}
                        }
                    }
                }

                match &*friends.lists.read() {
                    None => rsx! { div { class: "skeleton h-32 w-full", aria_hidden: "true" } },
                    Some(Err(e)) => rsx! {
                        div { role: "alert", class: "alert alert-error text-sm", {crate::common::error_message(e)} }
                    },
                    Some(Ok(lists)) => rsx! {
                        if lists.key_mismatch {
                            div { role: "alert", class: "alert alert-error", {tid!("friends-key-mismatch")} }
                        }
                        if !lists.incoming.is_empty() {
                            div { class: "card bg-base-100 shadow-soft",
                                div { class: "card-body gap-2",
                                    h2 { class: "card-title text-base", {tid!("friends-incoming-title")} }
                                    ul { class: "list",
                                        for req in lists.incoming.iter() {
                                            {
                                                let id = req.id;
                                                rsx! {
                                                    li { class: "list-row items-center",
                                                        span { class: "font-medium truncate", "{req.requester_email}" }
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

                        div { class: "card bg-base-100 shadow-soft",
                            div { class: "card-body gap-2",
                                h2 { class: "card-title text-base", {tid!("friends-list-title")} }
                                if let Some(code) = lists.my_fingerprint.as_ref() {
                                    div { class: "flex flex-col min-w-0 pb-2 border-b border-base-200",
                                        span { class: "font-medium truncate",
                                            {auth_ctx().map(|a| a.email).unwrap_or_default()}
                                        }
                                        span { class: "text-xs text-base-content/70 flex flex-wrap items-center gap-1",
                                            {tid!("friends-fingerprint")}
                                            kbd { class: "kbd kbd-xs", "{code}" }
                                        }
                                    }
                                }
                                if lists.friends.is_empty() {
                                    p { class: "text-sm text-base-content/70", {tid!("friends-list-empty")} }
                                } else {
                                    ul { class: "list",
                                        for f in lists.friends.iter() {
                                            {
                                                let row = f.clone();
                                                let pressed = f.clone();
                                                let long_press = format!("long-press-friend-{}", f.id);
                                                rsx! {
                                                    li { class: "list-row items-center", "data-long-press": long_press.clone(),
                                                        div { class: "list-col-grow flex flex-col min-w-0",
                                                            span { class: "font-medium truncate", "{f.email}" }
                                                            match &f.fingerprint {
                                                                _ if f.key_changed => {
                                                                    let trusted = row.clone();
                                                                    rsx! {
                                                                        span { class: "badge badge-soft badge-error badge-xs", {tid!("friends-key-changed")} }
                                                                        if let Some(code) = f.new_fingerprint() {
                                                                            span { class: "text-xs text-base-content/70 flex flex-wrap items-center gap-1",
                                                                                {tid!("friends-new-fingerprint")}
                                                                                kbd { class: "kbd kbd-xs", "{code}" }
                                                                            }
                                                                            button {
                                                                                r#type: "button",
                                                                                class: "btn btn-xs btn-outline mt-1 self-start",
                                                                                disabled: (friends.busy)(),
                                                                                onclick: move |_| friends.trust_new_key(trusted.clone()),
                                                                                {tid!("friends-code-matches")}
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                                Some(fp) => rsx! {
                                                                    span { class: "text-xs text-base-content/70 flex flex-wrap items-center gap-1",
                                                                        {tid!("friends-fingerprint")}
                                                                        kbd { class: "kbd kbd-xs", "{fp}" }
                                                                    }
                                                                },
                                                                None => rsx! {
                                                                    span { class: "badge badge-soft badge-warning badge-xs", {tid!("friends-no-key")} }
                                                                },
                                                            }
                                                        }
                                                        button {
                                                            r#type: "button",
                                                            class: "btn btn-square btn-soft btn-error btn-sm",
                                                            aria_label: tid!("friends-remove"),
                                                            disabled: (friends.busy)(),
                                                            onclick: move |_| removing.set(Some(row.clone())),
                                                            TrashIcon { size: ICON_INLINE }
                                                        }
                                                        // Last: daisyUI sizes `.list-row`'s columns by the position of
                                                        // `.list-col-grow` among the children.
                                                        button {
                                                            id: long_press.clone(),
                                                            r#type: "button",
                                                            hidden: true,
                                                            tabindex: "-1",
                                                            onclick: move |_| {
                                                                haptic(Haptic::Medium);
                                                                actions_for.set(Some(pressed.clone()));
                                                            },
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                                if lists.friends.iter().any(|f| f.key_changed) {
                                    div { role: "alert", class: "alert alert-error text-sm", {tid!("friends-key-changed-hint")} }
                                }
                                p { class: "text-xs text-base-content/70", {tid!("friends-fingerprint-hint")} }
                            }
                        }

                        if !lists.outgoing.is_empty() {
                            div { class: "card bg-base-100 shadow-soft",
                                div { class: "card-body gap-2",
                                    h2 { class: "card-title text-base", {tid!("friends-outgoing-title")} }
                                    p { class: "text-sm text-base-content/70", {tid!("friends-outgoing-hint")} }
                                    ul { class: "list",
                                        for req in lists.outgoing.iter() {
                                            {
                                                let id = req.id;
                                                let address = notify_address(&req.label).map(str::to_string);
                                                rsx! {
                                                    li { class: "list-row items-center",
                                                        bdi { class: "list-col-grow min-w-0 font-medium truncate", "{req.label}" }
                                                        if let Some(address) = address {
                                                            button {
                                                                r#type: "button",
                                                                class: "btn btn-ghost btn-square btn-xs",
                                                                aria_label: tid!("friends-notify-label", email: address.clone()),
                                                                onclick: move |_| friends.notify(address.clone()),
                                                                SendIcon { size: ICON_INLINE }
                                                            }
                                                        }
                                                        button {
                                                            r#type: "button",
                                                            class: "btn btn-ghost btn-xs",
                                                            disabled: (friends.busy)(),
                                                            onclick: move |_| friends.decline_or_withdraw(id),
                                                            {tid!("friends-withdraw")}
                                                        }
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

            if let Some(row) = actions_for() {
                {
                    let trusted = row.clone();
                    let can_trust = row.key_changed && row.new_fingerprint().is_some();
                    rsx! {
                        RowActionsSheet { title: row.email.clone(), on_close: move |_| actions_for.set(None),
                            if can_trust {
                                Row {
                                    id: "row-action-trust",
                                    label: tid!("friends-code-matches"),
                                    onclick: move |_| {
                                        actions_for.set(None);
                                        friends.trust_new_key(trusted.clone());
                                    },
                                    ShieldIcon { size: ICON_HEADER }
                                }
                            }
                            Row {
                                id: "row-action-remove",
                                label: tid!("friends-remove"),
                                danger: true,
                                onclick: move |_| {
                                    actions_for.set(None);
                                    removing.set(Some(row.clone()));
                                },
                                TrashIcon { size: ICON_HEADER }
                            }
                        }
                    }
                }
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
