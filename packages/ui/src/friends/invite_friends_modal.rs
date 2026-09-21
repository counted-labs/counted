use dioxus::prelude::*;
use crate::tid;
use shared::{Account, Friend};
use uuid::Uuid;

use super::friends_service::{invite_friends, my_private_key};
use crate::common::{error_message, Flash};
use crate::route::Route;

#[derive(PartialEq, Props, Clone)]
pub struct InviteFriendsModalProps {
    pub project_id: Uuid,
    pub project_key: [u8; 32],
    pub on_close: EventHandler<()>,
}

/// Picks friends to hand this project's key to. Every friend with a public key is a checkbox; the
/// key is boxed to each one on this device and the server only ever stores the boxes.
#[component]
pub fn InviteFriendsModal(props: InviteFriendsModalProps) -> Element {
    let auth_ctx = use_context::<Signal<Option<Account>>>();
    let account_key = use_context::<Signal<Option<[u8; 32]>>>();
    let mut flash = use_context::<Signal<Option<Flash>>>();
    let mut selected: Signal<Vec<Uuid>> = use_signal(Vec::new);
    let mut busy = use_signal(|| false);
    let mut error: Signal<Option<String>> = use_signal(|| None);

    let friends = use_resource(|| async {
        api::friends::friends_controller::get_friends().await.map(|v| v.friends)
    });

    let my_private = auth_ctx().zip(account_key()).and_then(|(a, k)| my_private_key(&a, &k));
    let project_id = props.project_id;
    let project_key = props.project_key;
    let on_close = props.on_close;

    let mut toggle = move |id: Uuid| {
        let mut list = selected.write();
        match list.iter().position(|x| *x == id) {
            Some(i) => {
                list.remove(i);
            }
            None => list.push(id),
        }
    };

    let on_invite = move |_| {
        let Some(private) = my_private else {
            error.set(Some(tid!("friends-no-account-key")));
            return;
        };
        let chosen: Vec<Friend> = friends
            .read()
            .as_ref()
            .and_then(|r| r.as_ref().ok())
            .map(|all| all.iter().filter(|f| selected().contains(&f.account_id)).cloned().collect())
            .unwrap_or_default();
        if chosen.is_empty() {
            return;
        }
        spawn(async move {
            busy.set(true);
            error.set(None);
            match invite_friends(project_id, &chosen, &private, &project_key).await {
                Ok(n) => {
                    flash.set(Some(Flash::ok(tid!("invite-sent", count: n))));
                    on_close.call(());
                }
                Err(e) => error.set(Some(error_message(&e))),
            }
            busy.set(false);
        });
    };

    rsx! {
        dialog { open: true, class: "modal modal-open modal-bottom sm:modal-middle",
            div { class: "modal-box p-0 flex flex-col",
                div { class: "flex flex-col gap-0.5 px-6 pt-5 pb-4 border-b border-base-200 flex-shrink-0",
                    h1 { class: "text-lg font-bold font-display", {tid!("invite-friends-title")} }
                    p { class: "text-sm text-base-content/70", {tid!("invite-friends-hint")} }
                }
                div { class: "flex-1 overflow-y-auto px-6 py-4 flex flex-col gap-3",
                    if let Some(err) = error() {
                        div { role: "alert", class: "alert alert-error text-sm", "{err}" }
                    }
                    match &*friends.read() {
                        None => rsx! { div { class: "skeleton h-24 w-full", aria_hidden: "true" } },
                        Some(Err(e)) => rsx! {
                            div { role: "alert", class: "alert alert-error text-sm", {error_message(e)} }
                        },
                        Some(Ok(list)) if list.is_empty() => rsx! {
                            p { class: "text-sm text-base-content/70", {tid!("invite-friends-empty")} }
                            Link { class: "link link-primary text-sm", to: Route::FriendsPage {}, {tid!("friends-title")} }
                        },
                        Some(Ok(list)) => rsx! {
                            ul { class: "list",
                                for f in list.iter() {
                                    {
                                        let id = f.account_id;
                                        let has_key = f.public_key.is_some();
                                        rsx! {
                                            li { class: "list-row items-center",
                                                label { class: "flex items-center gap-3 cursor-pointer min-w-0",
                                                    input {
                                                        r#type: "checkbox",
                                                        class: "checkbox checkbox-primary checkbox-sm",
                                                        disabled: !has_key || busy(),
                                                        checked: selected().contains(&id),
                                                        onchange: move |_| toggle(id),
                                                    }
                                                    span { class: "truncate", "{f.email}" }
                                                }
                                                if !has_key {
                                                    span { class: "badge badge-soft badge-warning badge-xs", {tid!("friends-no-key")} }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        },
                    }
                }
                div { class: "flex justify-end gap-2 px-6 py-4 border-t border-base-200 flex-shrink-0",
                    button {
                        r#type: "button",
                        class: "btn btn-ghost",
                        onclick: move |_| on_close.call(()),
                        {tid!("cancel")}
                    }
                    button {
                        id: "invite-friends-confirm",
                        r#type: "button",
                        class: "btn btn-primary",
                        disabled: busy() || selected().is_empty(),
                        onclick: on_invite,
                        if busy() {
                            span { class: "loading loading-spinner loading-sm", role: "status", aria_label: tid!("loading") }
                        }
                        {tid!("invite-friends-button")}
                    }
                }
            }
            div { class: "modal-backdrop", onclick: move |_| on_close.call(()) }
        }
    }
}
