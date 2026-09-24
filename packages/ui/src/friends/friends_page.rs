use dioxus::prelude::*;
use crate::tid;
use shared::Account;

use super::friends_service::FriendRow;
use super::use_friends::use_friends;
use crate::common::{AppHeader, AuthResolved, ConfirmModal, PullToRefresh};
use crate::icons::{TrashIcon, ICON_INLINE};
use crate::route::Route;

#[component]
pub fn FriendsPage() -> Element {
    let nav = use_navigator();
    let auth_ctx = use_context::<Signal<Option<Account>>>();
    let auth_resolved = use_context::<Signal<AuthResolved>>();
    let mut friends = use_friends();
    let mut removing = use_signal(|| None::<FriendRow>);

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

                match &*friends.lists.read() {
                    None => rsx! { div { class: "skeleton h-32 w-full", aria_hidden: "true" } },
                    Some(Err(e)) => rsx! {
                        div { role: "alert", class: "alert alert-error text-sm", {crate::common::error_message(e)} }
                    },
                    Some(Ok(lists)) => rsx! {
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
                                if lists.friends.is_empty() {
                                    p { class: "text-sm text-base-content/70", {tid!("friends-list-empty")} }
                                } else {
                                    ul { class: "list",
                                        for f in lists.friends.iter() {
                                            {
                                                let row = f.clone();
                                                rsx! {
                                                    li { class: "list-row items-center",
                                                        div { class: "list-col-grow flex flex-col min-w-0",
                                                            span { class: "font-medium truncate", "{f.email}" }
                                                            match &f.fingerprint {
                                                                Some(fp) => rsx! {
                                                                    span { class: "text-xs text-base-content/70",
                                                                        {tid!("friends-fingerprint")}
                                                                        " "
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
                                                    }
                                                }
                                            }
                                        }
                                    }
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
                                                rsx! {
                                                    li { class: "list-row items-center",
                                                        span { class: "font-medium truncate", "{req.label}" }
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

            if let Some(row) = removing() {
                ConfirmModal {
                    title: tid!("friends-remove-confirm-title"),
                    message: tid!("friends-remove-confirm-message", email: row.email.clone()),
                    confirm_label: tid!("friends-remove"),
                    on_cancel: move |_| removing.set(None),
                    on_confirm: move |_| {
                        removing.set(None);
                        friends.remove(row.id);
                    },
                }
            }
        }
    }
}
