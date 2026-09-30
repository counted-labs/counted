use api::account_projects::account_projects_controller::upsert_account_project;
use api::auth::auth_controller::me;
use dioxus::fullstack::Json;
use dioxus::prelude::*;
use crate::tid;
use shared::{Account as AccountData, UpsertAccountProject, User};
use uuid::Uuid;

use crate::common::{
    identity_locked, is_claim_proof_error, is_identity_taken_error, is_offline_error, update_ls,
    upsert_project, AcceptedInvite, LocalStorageState, ProjectKey,
};
use crate::crypto::{claim_label, claim_token};
use crate::decrypted::{decrypt_user, DecryptedUser};
use crate::icons::{CloseIcon, LockIcon, ICON_ACTION, ICON_HEADER};
use crate::participants::Avatar;
use crate::payment_methods::refill_project_copies;

#[derive(PartialEq, Props, Clone)]
pub struct UserSelectionModalProps {
    pub users: Vec<User>,
    pub project_id: Uuid,
    /// Set when opened on purpose ("Switch" in the edit modal): the modal can be dismissed, and is
    /// closed after a successful pick. Without it the modal is the only way into the project.
    #[props(default)]
    pub on_close: Option<EventHandler<()>>,
}

/// Every key `error_key` can hold. They reach `tid!` as a value, not a literal, so the crate-wide
/// scanner in `i18n` cannot see them — this is the guard that replaces it.
#[cfg(all(test, not(target_arch = "wasm32")))]
const ERROR_KEYS: &[&str] =
    &["user-selection-required", "error-identity-taken", "error-claim-proof-invalid", "error-generic"];

#[cfg(all(test, not(target_arch = "wasm32")))]
mod key_guard {
    #[test]
    fn every_error_key_is_a_known_message() {
        let known = crate::i18n::locale_ids(crate::i18n::FALLBACK);
        for key in super::ERROR_KEYS {
            assert!(known.contains(*key), "{key} is not defined in the fallback locale");
        }
    }
}

#[component]
pub fn UserSelectionModal(props: UserSelectionModalProps) -> Element {
    let ls_ctx = use_context::<Signal<LocalStorageState>>();
    let key_ctx = use_context::<ProjectKey>().0;
    let account_key_ctx = use_context::<Signal<Option<[u8; 32]>>>();
    let auth_ctx = use_context::<Signal<Option<AccountData>>>();
    let is_online = use_context::<Signal<bool>>();
    let mut accepted_ctx = use_context::<Signal<Option<AcceptedInvite>>>();
    let mut error_key: Signal<Option<&'static str>> = use_signal(|| None);
    let mut saving = use_signal(|| false);

    let project_id = props.project_id;
    let on_close = props.on_close;

    // This device's current identity, so its own claim is never shown as locked against it.
    let my_user_id = ls_ctx()
        .projects
        .iter()
        .find(|p| p.project_id == project_id)
        .and_then(|p| p.user_id);

    // Decrypted once per render rather than per participant per branch: the claim label needs the
    // project key too, and `decrypt_user` reads both in one pass.
    let participants: Vec<DecryptedUser> = match key_ctx() {
        Some(k) => props.users.iter().filter_map(|u| decrypt_user(&k, u).ok()).collect(),
        None => vec![],
    };

    // Reached by accepting an invitation: who sent it, and the participant they created for this
    // person, when it is still free.
    let accepted = accepted_ctx().filter(|a| a.project_id == project_id);
    let suggested = accepted.as_ref().and_then(|a| a.user_id).filter(|id| {
        participants.iter().any(|u| u.id == *id && !identity_locked(u, my_user_id))
    });
    let mut selected_user_id: Signal<Option<i32>> = use_signal(|| suggested.or(my_user_id));

    let mut finish = move || {
        accepted_ctx.set(None);
        if let Some(close) = on_close {
            close.call(());
        }
    };

    let on_confirm = move |_| {
        let Some(user_id) = selected_user_id() else {
            error_key.set(Some("user-selection-required"));
            return;
        };

        let Some(account) = auth_ctx() else {
            // No account, no claim: an anonymous identity lives only in localStorage, so there is
            // nothing to ask the server for and nothing it could refuse.
            update_ls(ls_ctx, |state| upsert_project(state, project_id, Some(user_id)));
            finish();
            return;
        };

        // Offline, the claim cannot be checked — and this modal is the only way past itself, so
        // refusing to record anything would lock a signed-in user out of their own project until
        // they get signal. Record it locally; `ensure_membership` pushes it on the next open and
        // clears it there if the server refuses.
        if !is_online() {
            update_ls(ls_ctx, |state| upsert_project(state, project_id, Some(user_id)));
            finish();
            return;
        }

        let label = match (account_key_ctx(), key_ctx()) {
            (Some(ak), Some(pk)) => claim_label(&account, &ak, &pk),
            _ => None,
        };
        let token = key_ctx().map(|pk| claim_token(&pk).to_vec());

        saving.set(true);
        let mut finish = finish;
        spawn(async move {
            // The server writes first, and localStorage only on success. The other order left a
            // device convinced it was a participant the account had been refused — invisibly, since
            // the call was fire-and-forget.
            //
            // No key: `ensure_membership` escrows it on every project open, and this modal only ever
            // narrows *which participant* the device is.
            let result = upsert_account_project(Json(UpsertAccountProject {
                project_id,
                user_id: Some(user_id),
                key: None,
                claim_label: label,
                claim_token: token,
            }))
            .await;

            saving.set(false);
            match result {
                Ok(()) => {
                    update_ls(ls_ctx, |state| upsert_project(state, project_id, Some(user_id)));
                    finish();
                    // The shared payment methods follow the identity, from a fresh `me()` rather
                    // than the `account` this session booted with — see `refill_project_copies`.
                    if let Some(ak) = account_key_ctx() {
                        if let Ok(Some(fresh)) = me().await {
                            let _ = refill_project_copies(&fresh, &ak).await;
                        }
                    }
                }
                Err(e) if is_identity_taken_error(&e) => {
                    // Raced against another account between the render and the click — the list is
                    // a snapshot, the index is the truth.
                    selected_user_id.set(None);
                    error_key.set(Some("error-identity-taken"));
                }
                // This device's key does not match the project's verifier: nothing to pick, the
                // fix is the share link.
                Err(e) if is_claim_proof_error(&e) => error_key.set(Some("error-claim-proof-invalid")),
                // The request never reached the server, so nothing was refused — same situation as
                // the offline branch above, and the same answer. `is_online` can still be true here:
                // it only flips on native connectivity events, and on web it is always true.
                Err(e) if is_offline_error(&e) => {
                    update_ls(ls_ctx, |state| upsert_project(state, project_id, Some(user_id)));
                    finish();
                }
                Err(_) => error_key.set(Some("error-generic")),
            }
        });
    };

    let invited_hint = accepted.as_ref().and_then(|a| {
        a.project_name.clone().map(|project| (a.from_email.clone(), project))
    });
    let sender = accepted.as_ref().map(|a| a.from_email.clone());
    let confirm_name = participants
        .iter()
        .find(|u| Some(u.id) == selected_user_id())
        .map(|u| u.name.clone());

    rsx! {
        dialog { open: true, class: "modal modal-open modal-bottom sm:modal-middle",
            div { class: "modal-box p-0 flex flex-col",
                div { class: "flex items-start justify-between gap-2 px-6 pt-5 pb-4 border-b border-base-200 flex-shrink-0",
                    div { class: "flex flex-col gap-0.5",
                        h1 { class: "text-lg font-bold font-display", {tid!("user-selection-title")} }
                        p { id: "user-selection-hint", class: "text-sm text-base-content/70",
                            match invited_hint {
                                Some((email, project)) => rsx! { {tid!("user-selection-invited-hint", email: email, project: project)} },
                                None => rsx! { {tid!("user-selection-hint")} },
                            }
                        }
                    }
                    if let Some(close) = on_close {
                        button {
                            r#type: "button",
                            class: "btn btn-ghost btn-circle h-11 w-11 min-h-11 -mr-3 -mt-1",
                            aria_label: tid!("close"),
                            onclick: move |_| close.call(()),
                            CloseIcon { size: ICON_HEADER }
                        }
                    }
                }
                div { class: "flex-1 overflow-y-auto px-6 py-4",
                if let Some(key) = error_key() {
                    div { role: "alert", class: "alert alert-error text-sm mb-3", {tid!(key)} }
                }

                fieldset { class: "flex flex-col gap-2 min-w-0",
                    legend { class: "sr-only", {tid!("add-project-participants")} }
                    for user in participants.iter() {
                        {
                            let user_id = user.id;
                            let user_name = user.name.clone();
                            let is_selected = selected_user_id() == Some(user_id);
                            let is_suggested = suggested == Some(user_id);
                            let locked = identity_locked(user, my_user_id);
                            // Rendered inline, not in a hover tooltip: this is a mobile-first app
                            // and there is no hover on touch. `title` still helps a desktop pointer.
                            let claimed_by = match user.claim_name.clone() {
                                Some(name) => tid!("identity-claimed-by", name: name),
                                None => tid!("identity-claimed"),
                            };
                            let claimed_title = claimed_by.clone();
                            rsx! {
                                if locked {
                                    div {
                                        class: "flex items-center gap-3 min-h-14 px-3.5 rounded-box border border-dashed border-base-300 bg-base-200",
                                        title: claimed_title,
                                        aria_disabled: "true",
                                        LockIcon { size: ICON_ACTION }
                                        Avatar { name: user_name.clone(), guest: false }
                                        span { class: "flex flex-col min-w-0 flex-1",
                                            span { class: "font-semibold truncate", "{user_name}" }
                                            span { class: "text-sm text-base-content/70 truncate", "{claimed_by}" }
                                        }
                                    }
                                } else {
                                    label {
                                        class: if is_selected { "flex items-center gap-3 min-h-14 px-3 rounded-box border-2 border-primary bg-primary/5 cursor-pointer" } else { "flex items-center gap-3 min-h-14 px-3.5 rounded-box border border-base-200 cursor-pointer" },
                                        input {
                                            id: "user-selection-{user_id}",
                                            r#type: "radio",
                                            name: "user-selection",
                                            class: "radio radio-primary radio-sm",
                                            checked: is_selected,
                                            onchange: move |_| {
                                                selected_user_id.set(Some(user_id));
                                                error_key.set(None);
                                            },
                                        }
                                        Avatar { name: user_name.clone(), guest: false }
                                        span { class: "flex flex-col min-w-0 flex-1",
                                            span { class: "font-semibold truncate", "{user_name}" }
                                            if is_suggested {
                                                if let Some(email) = sender.clone() {
                                                    span { class: "text-sm text-base-content/70 truncate", {tid!("user-selection-suggested-sub", email: email)} }
                                                }
                                            }
                                        }
                                        if is_suggested {
                                            span { class: "badge badge-soft badge-primary badge-sm", {tid!("user-selection-suggested")} }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                p { class: "text-xs text-base-content/70 mt-3", {tid!("user-selection-missing")} }

                } // end scrollable body
                div { class: "flex justify-end px-6 py-4 border-t border-base-200 flex-shrink-0",
                    button {
                        id: "user-selection-confirm",
                        r#type: "button",
                        class: "btn btn-primary",
                        disabled: saving(),
                        onclick: on_confirm,
                        if saving() {
                            span { class: "loading loading-spinner loading-sm", role: "status", aria_label: tid!("loading") }
                        }
                        match confirm_name {
                            Some(name) => rsx! { {tid!("user-selection-confirm-as", name: name)} },
                            None => rsx! { {tid!("confirm")} },
                        }
                    }
                }
            }
            if let Some(close) = on_close {
                div { class: "modal-backdrop", onclick: move |_| close.call(()) }
            }
        }
    }
}
