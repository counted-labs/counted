use dioxus::prelude::*;
use crate::tid;
use shared::{Account, Friend};
use uuid::Uuid;

use super::friend_checklist::FriendChecklist;
use super::friends_service::{invite_each, invitee_emails, my_private_key, Invitee};
use crate::common::{error_message, Flash};
use crate::route::Route;

#[derive(PartialEq, Props, Clone)]
pub struct InviteFriendsModalProps {
    pub project_id: Uuid,
    pub project_key: [u8; 32],
    pub on_close: EventHandler<()>,
    /// Friends ticked on open, with the participant each was created as: "Invite again" after a
    /// create or edit whose invitations did not go out.
    #[props(default)]
    pub preselected: Vec<Invitee>,
}

/// Picks friends to hand this project's key to. Every friend with a public key is a checkbox; the
/// key is boxed to each one on this device and the server only ever stores the boxes.
#[component]
pub fn InviteFriendsModal(props: InviteFriendsModalProps) -> Element {
    let auth_ctx = use_context::<Signal<Option<Account>>>();
    let account_key = use_context::<Signal<Option<[u8; 32]>>>();
    let mut flash = use_context::<Signal<Option<Flash>>>();
    let preselected = props.preselected.clone();
    let selected: Signal<Vec<Uuid>> =
        use_signal(|| preselected.iter().map(|i| i.friend.account_id).collect());
    let mut busy = use_signal(|| false);
    let mut error: Signal<Option<String>> = use_signal(|| None);

    let friends = use_resource(|| async {
        api::friends::friends_controller::get_friends().await.map(|v| v.friends)
    });

    let my_private = auth_ctx().zip(account_key()).and_then(|(a, k)| my_private_key(&a, &k));
    let project_id = props.project_id;
    let project_key = props.project_key;
    let on_close = props.on_close;

    let on_invite = move |_| {
        if my_private.is_none() {
            error.set(Some(tid!("friends-no-account-key")));
            return;
        }
        let chosen: Vec<Invitee> = friends
            .read()
            .as_ref()
            .and_then(|r| r.as_ref().ok())
            .map(|all| {
                all.iter()
                    .filter(|f| selected().contains(&f.account_id))
                    .map(|f| Invitee {
                        friend: f.clone(),
                        user_id: preselected
                            .iter()
                            .find(|i| i.friend.account_id == f.account_id)
                            .and_then(|i| i.user_id),
                    })
                    .collect()
            })
            .unwrap_or_default();
        if chosen.is_empty() {
            return;
        }
        spawn(async move {
            busy.set(true);
            error.set(None);
            let failed = invite_each(project_id, &chosen, my_private, &project_key).await;
            if failed.is_empty() {
                flash.set(Some(Flash::ok(tid!("invite-sent", count: chosen.len()))));
                on_close.call(());
            } else {
                error.set(Some(tid!("invite-failed", emails: invitee_emails(&failed))));
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
                            FriendChecklist { friends: list.clone(), selected, disabled: busy() }
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

/// The "All friends" picker of the participant block: the same checklist, before the project
/// exists, so it hands the choice back instead of inviting.
#[derive(PartialEq, Props, Clone)]
pub struct FriendPickerModalProps {
    pub friends: Vec<Friend>,
    pub on_pick: EventHandler<Vec<Friend>>,
    pub on_close: EventHandler<()>,
}

#[component]
pub fn FriendPickerModal(props: FriendPickerModalProps) -> Element {
    let selected: Signal<Vec<Uuid>> = use_signal(Vec::new);
    let on_close = props.on_close;
    let on_pick = props.on_pick;
    let friends = props.friends.clone();

    rsx! {
        dialog { open: true, class: "modal modal-open modal-bottom sm:modal-middle",
            div { class: "modal-box p-0 flex flex-col",
                div { class: "flex flex-col gap-0.5 px-6 pt-5 pb-4 border-b border-base-200 flex-shrink-0",
                    h2 { class: "text-lg font-bold font-display", {tid!("friend-picker-title")} }
                    p { class: "text-sm text-base-content/70", {tid!("invite-friends-hint")} }
                }
                div { class: "flex-1 overflow-y-auto px-6 py-4 flex flex-col gap-3",
                    if props.friends.is_empty() {
                        p { class: "text-sm text-base-content/70", {tid!("invite-friends-empty")} }
                    } else {
                        FriendChecklist { friends: props.friends.clone(), selected, disabled: false }
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
                        id: "friend-picker-confirm",
                        r#type: "button",
                        class: "btn btn-primary",
                        disabled: selected().is_empty(),
                        onclick: move |_| {
                            let chosen = friends.iter().filter(|f| selected().contains(&f.account_id)).cloned().collect();
                            on_pick.call(chosen);
                        },
                        {tid!("add")}
                    }
                }
            }
            div { class: "modal-backdrop", onclick: move |_| on_close.call(()) }
        }
    }
}
