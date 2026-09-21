use dioxus::prelude::*;
use crate::tid;
use shared::Account;
use uuid::Uuid;

use super::friends_service::{accept_invitation, decline_invitation, load_invitations, my_private_key, OpenedInvitation};
use crate::common::{error_message, Flash, LocalStorageState};
use crate::route::Route;

#[derive(PartialEq, Props, Clone)]
pub struct InvitationsCardProps {
    /// Fired after an accepted key has been written to the store, before navigating.
    pub on_accepted: EventHandler<Uuid>,
}

/// The invitations waiting for this account, one card each, above the project list. Renders
/// nothing for an anonymous user, while loading, or when there are none.
#[component]
pub fn InvitationsCard(props: InvitationsCardProps) -> Element {
    let nav = use_navigator();
    let auth_ctx = use_context::<Signal<Option<Account>>>();
    let account_key = use_context::<Signal<Option<[u8; 32]>>>();
    let ls_ctx = use_context::<Signal<LocalStorageState>>();
    let mut flash = use_context::<Signal<Option<Flash>>>();
    let mut busy = use_signal(|| false);
    let mut version = use_signal(|| 0u64);

    let invitations = use_resource(move || {
        let _ = version();
        let private = auth_ctx().zip(account_key()).and_then(|(a, k)| my_private_key(&a, &k));
        let signed_in = auth_ctx().is_some();
        async move {
            if !signed_in {
                return Ok(vec![]);
            }
            load_invitations(private).await
        }
    });

    let on_accepted = props.on_accepted;
    let accept = move |inv: OpenedInvitation| {
        let Some(key) = inv.project_key else { return };
        spawn(async move {
            busy.set(true);
            match accept_invitation(ls_ctx, &inv.invitation, &key).await {
                Ok(()) => {
                    on_accepted.call(inv.invitation.project_id);
                    nav.push(Route::ExpensesPage { project_id: inv.invitation.project_id });
                }
                Err(e) => flash.set(Some(Flash::err(error_message(&e)))),
            }
            busy.set(false);
        });
    };

    let decline = move |id: Uuid| {
        spawn(async move {
            busy.set(true);
            match decline_invitation(id).await {
                Ok(()) => version += 1,
                Err(e) => flash.set(Some(Flash::err(error_message(&e)))),
            }
            busy.set(false);
        });
    };

    let list = match &*invitations.read() {
        Some(Ok(list)) if !list.is_empty() => list.clone(),
        _ => return rsx! {},
    };

    rsx! {
        div { class: "flex flex-col gap-2",
            for inv in list.into_iter() {
                {
                    let id = inv.invitation.id;
                    let openable = inv.project_key.is_some();
                    let title = match &inv.project_name {
                        Some(name) => tid!("invitation-to", name: name.clone()),
                        None if openable => tid!("invitation-to-unnamed"),
                        None => tid!("invitation-unreadable"),
                    };
                    let from = inv.invitation.from_email.clone();
                    let inv_for_accept = inv.clone();
                    rsx! {
                        div { class: "card bg-base-100 shadow-soft border border-primary/30",
                            div { class: "card-body p-4 gap-2",
                                span { class: "badge badge-soft badge-primary badge-sm", {tid!("invitation-badge")} }
                                h2 { class: "font-semibold", "{title}" }
                                p { class: "text-sm text-base-content/70", {tid!("invitation-from", email: from)} }
                                div { class: "card-actions justify-end",
                                    button {
                                        r#type: "button",
                                        class: "btn btn-ghost btn-sm",
                                        disabled: busy(),
                                        onclick: move |_| decline(id),
                                        {tid!("invitation-decline")}
                                    }
                                    if openable {
                                        button {
                                            r#type: "button",
                                            class: "btn btn-primary btn-sm",
                                            disabled: busy(),
                                            onclick: move |_| accept(inv_for_accept.clone()),
                                            {tid!("invitation-accept")}
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
