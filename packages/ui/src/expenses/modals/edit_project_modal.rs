use api::friends::invitations_controller::get_sent_invitations;
use api::projects::projects_controller::update_project_by_id;
use api::users::users_controller::{add_user, delete_user};
use dioxus::fullstack::Json;
use dioxus::prelude::*;
use crate::tid;
use shared::{Account, CreatableUser, CreatableUserBatch, EditableProject, HistoryContext, HistoryPayload, ProjectDto, ProjectPayload, User, UserPayload};
use std::collections::VecDeque;
use uuid::Uuid;

use crate::common::{
    error_message, identity_locked, is_user_gone_error, write_queue, Flash, InviteRetry,
    LocalStorageState, OpKind, ProjectKey, QueuedOp,
};
use std::collections::HashSet;
use crate::crypto::{decrypt_json, encrypt_json};
use crate::decrypted::{decrypt_user, DecryptedUser};
use crate::friends::friends_service::{invite_each, my_private_key, Invitee};
use crate::icons::{CloseIcon, LockIcon, ICON_HEADER, ICON_INLINE};
use crate::participants::participants_service::{invite_count, same_name, DraftParticipant};
use crate::participants::{use_friend_list, Avatar, ParticipantsEditor};
use crate::expenses::hooks::use_project_store::ProjectStore;

#[derive(PartialEq, Props, Clone)]
pub struct EditProjectModalProps {
    pub project: ProjectDto,
    pub users: Vec<User>,
    pub on_close: EventHandler<()>,
    pub on_saved: EventHandler<()>,
    /// "Switch" on the You card: closes this and opens "Which participant are you?". The project
    /// list has no picker to open, so it passes none and the card shows no button.
    #[props(default)]
    pub on_switch: Option<EventHandler<()>>,
}

#[component]
pub fn EditProjectModal(props: EditProjectModalProps) -> Element {
    let project_id = props.project.id;
    let ls_ctx = use_context::<Signal<LocalStorageState>>();
    let key_ctx = use_context::<ProjectKey>().0;
    let auth_ctx = use_context::<Signal<Option<Account>>>();
    let account_key_ctx = use_context::<Signal<Option<[u8; 32]>>>();
    let is_online = use_context::<Signal<bool>>();
    let mut pending_ops = use_context::<Signal<VecDeque<QueuedOp>>>();
    let mut flash = use_context::<Signal<Option<Flash>>>();
    let mut invite_retry = use_context::<Signal<Option<InviteRetry>>>();
    let friend_list = use_friend_list();

    let key_snap = key_ctx();

    let initial_project = key_snap
        .as_ref()
        .and_then(|k| decrypt_json::<ProjectPayload>(k, &props.project.payload).ok());
    let initial_name = initial_project.as_ref().map(|p| p.name.clone()).unwrap_or_default();
    let initial_description = initial_project
        .as_ref()
        .and_then(|p| p.description.clone())
        .unwrap_or_default();
    let initial_currency = initial_project
        .as_ref()
        .map(|p| p.currency.clone())
        .unwrap_or_default();
    // The submit closure below moves `initial_currency` (it writes the value straight back), so the
    // read-only field needs its own copy.
    let displayed_currency = initial_currency.clone();
    let initial_users: Vec<DecryptedUser> = key_snap
        .as_ref()
        .map(|k| props.users.iter().filter_map(|u| decrypt_user(k, u).ok()).collect())
        .unwrap_or_default();

    let mut name = use_signal(|| initial_name.clone());
    let mut description = use_signal(|| initial_description.clone());
    // Existing users stored as decrypted so display and comparison work directly
    let mut existing_users: Signal<Vec<DecryptedUser>> = use_signal(|| initial_users.clone());
    let mut drafts: Signal<Vec<DraftParticipant>> = use_signal(Vec::new);
    let mut loading = use_signal(|| false);
    let mut error_msg: Signal<Option<String>> = use_signal(|| None);

    let original_user_ids: Vec<i32> = props.users.iter().map(|u| u.id).collect();
    let rules = use_context::<ProjectStore>()
        .recurring_for(project_id)
        .map(|r| r.rules)
        .unwrap_or_default();

    // The caller's own pending invitations naming a participant: those rows read "Invited".
    let sent = use_resource(move || {
        let signed_in = auth_ctx.read().is_some();
        async move {
            if !signed_in {
                return Vec::new();
            }
            get_sent_invitations(project_id).await.unwrap_or_default()
        }
    });

    // "Add as friend" on a locked row. Only a signed-in device with its account key can send one:
    // the request carries a label encrypted under that key.
    let can_add_friend = auth_ctx().is_some() && account_key_ctx().is_some();
    let add_friend = move |(user_id, participant_name): (i32, String)| {
        let Some(account_key) = account_key_ctx() else { return };
        spawn(async move {
            match crate::friends::friends_service::add_from_project(project_id, user_id, &participant_name, &account_key).await {
                Ok(()) => flash.set(Some(Flash::ok(tid!("friends-request-sent")))),
                Err(e) => flash.set(Some(Flash::err(error_message(&e)))),
            }
        });
    };

    let on_close_x = props.on_close;
    let on_close_cancel = props.on_close;
    let on_close_backdrop = props.on_close;
    let on_saved = props.on_saved;
    let on_switch = props.on_switch;

    let on_submit = move |_| {
        let key = match key_snap {
            Some(k) => k,
            None => {
                error_msg.set(Some(tid!("missing-encryption-key")));
                return;
            }
        };

        let name_val = name().trim().to_string();
        if name_val.is_empty() {
            error_msg.set(Some(tid!("expense-name-required")));
            return;
        }
        let desc_val = description();
        let desc: Option<String> =
            if desc_val.trim().is_empty() { None } else { Some(desc_val.trim().to_string()) };

        let current_existing = existing_users();
        let current_new = drafts();
        let orig_ids = original_user_ids.clone();
        let currency = initial_currency.clone();

        let kept: HashSet<i32> = current_existing.iter().map(|u| u.id).collect();
        for uid in orig_ids.iter().filter(|id| !kept.contains(id)) {
            let blocking: Vec<String> = rules
                .iter()
                .filter(|r| r.payload.template.involves(*uid))
                .map(|r| r.payload.template.name.clone())
                .collect();
            if !blocking.is_empty() {
                let who = initial_users.iter().find(|u| u.id == *uid).map(|u| u.name.clone()).unwrap_or_default();
                error_msg.set(Some(tid!("recurring-blocks-removal", name: who, rules: blocking.join(", "))));
                return;
            }
        }

        let actor_user_id = ls_ctx()
            .projects
            .iter()
            .find(|p| p.project_id == project_id)
            .and_then(|p| p.user_id)
            .unwrap_or(0);

        // History summaries are encrypted and stored, so they keep the language of whoever wrote
        // them. That is the honest record — the entry says what this person did — and it avoids a
        // payload migration for every summary already written.
        let mut diff: Vec<String> = Vec::new();
        if name_val != initial_name {
            diff.push(tid!("history-name-changed", from: initial_name.clone(), to: name_val.clone()));
        }
        let new_desc_str = desc.as_deref().unwrap_or("");
        if new_desc_str != initial_description.as_str() {
            if initial_description.is_empty() {
                diff.push(tid!("history-description-added", value: new_desc_str.to_string()));
            } else if new_desc_str.is_empty() {
                diff.push(tid!("history-description-removed", value: initial_description.clone()));
            } else {
                diff.push(tid!(
                    "history-description-changed",
                    from: initial_description.clone(),
                    to: new_desc_str.to_string()
                ));
            }
        }
        let summary = if diff.is_empty() {
            tid!("history-project-edited", name: name_val.clone())
        } else {
            diff.join(" • ")
        };

        let payload = match encrypt_json(&key, &ProjectPayload {
            name: name_val.clone(),
            currency,
            description: desc,
        }) {
            Ok(v) => v,
            Err(e) => { error_msg.set(Some(e)); return; }
        };

        let history = encrypt_json(&key, &HistoryPayload { summary })
            .ok()
            .map(|p| HistoryContext { actor_user_id, payload: p });

        if !is_online() {
            pending_ops.write().push_back(QueuedOp {
                id: Uuid::new_v4(),
                label: name_val,
                op: OpKind::UpdateProject(EditableProject {
                    id: project_id,
                    payload: Some(payload),
                    status: None,
                    history,
                }),
            });
            write_queue(&pending_ops.read());
            // Warn about anything that can't be applied offline.
            let existing_ids: HashSet<i32> = current_existing.iter().map(|u| u.id).collect();
            let has_deletions = orig_ids.iter().any(|id| !existing_ids.contains(id));
            let mut deferred: Vec<String> = Vec::new();
            if !current_new.is_empty() { deferred.push(tid!("edit-project-deferred-new-members")); }
            if has_deletions { deferred.push(tid!("edit-project-deferred-removals")); }
            if !deferred.is_empty() {
                flash.set(Some(Flash::ok(
                    tid!("edit-project-offline-deferred", items: deferred.join(", ")),
                )));
            }
            on_saved.call(());
            return;
        }

        let account = auth_ctx();
        let account_key = account_key_ctx();
        loading.set(true);
        error_msg.set(None);

        spawn(async move {
            if let Err(e) = update_project_by_id(Json(EditableProject {
                id: project_id,
                payload: Some(payload),
                status: None,
                history,
            }))
            .await
            {
                error_msg.set(Some(error_message(&e)));
                loading.set(false);
                return;
            }

            let mut created_users: Vec<User> = Vec::new();
            if !current_new.is_empty() {
                let mut creatables: Vec<CreatableUser> = Vec::new();
                for d in &current_new {
                    let payload = match encrypt_json(&key, &UserPayload { name: d.name.clone() }) {
                        Ok(v) => v,
                        Err(e) => { error_msg.set(Some(e)); loading.set(false); return; }
                    };
                    creatables.push(CreatableUser { payload, project_id });
                }
                match add_user(Json(CreatableUserBatch::Multiple(creatables))).await {
                    // Moved into the existing list at once: a removal below can still fail (409
                    // for a participant with payments, the ordinary case), and a re-submit used to
                    // insert every new participant a second time.
                    Ok(users) => {
                        existing_users
                            .write()
                            .extend(users.iter().filter_map(|u| decrypt_user(&key, u).ok()));
                        drafts.write().clear();
                        created_users = users;
                    }
                    Err(e) => {
                        error_msg.set(Some(error_message(&e)));
                        loading.set(false);
                        return;
                    }
                }
            }

            // Sent before the removals, which can fail and keep the modal open: the new
            // participants already exist, so their invitations must not wait on a re-submit.
            let invitees: Vec<Invitee> = current_new
                .iter()
                .filter_map(|d| {
                    let friend = d.friend.clone()?;
                    let user_id = created_users
                        .iter()
                        .find(|u| {
                            decrypt_json::<UserPayload>(&key, &u.payload)
                                .map(|p| same_name(&p.name, &d.name))
                                .unwrap_or(false)
                        })
                        .map(|u| u.id);
                    Some(Invitee { friend, user_id })
                })
                .collect();
            if !invitees.is_empty() {
                let my_private = account
                    .as_ref()
                    .zip(account_key)
                    .and_then(|(a, ak)| my_private_key(a, &ak));
                let failed = invite_each(project_id, &invitees, my_private, &key).await;
                if !failed.is_empty() {
                    invite_retry.set(Some(InviteRetry { project_id, invitees: failed }));
                }
            }

            let kept_ids: std::collections::HashSet<i32> =
                current_existing.iter().map(|u| u.id).collect();
            for uid in &orig_ids {
                if !kept_ids.contains(uid) {
                    // Already gone is done: a re-submit after a failed removal re-sends the ones
                    // that succeeded, since `orig_ids` comes from the props.
                    if let Err(e) = delete_user(project_id, *uid).await.or_else(|e| {
                        if is_user_gone_error(&e) { Ok(()) } else { Err(e) }
                    }) {
                        error_msg.set(Some(error_message(&e)));
                        loading.set(false);
                        return;
                    }
                }
            }

            on_saved.call(());
        });
    };

    let stored_user_id = move || {
        ls_ctx().projects.iter().find(|p| p.project_id == project_id).and_then(|p| p.user_id)
    };
    let my_id = stored_user_id();
    let me = existing_users().into_iter().find(|u| Some(u.id) == my_id);
    let others: Vec<DecryptedUser> =
        existing_users().into_iter().filter(|u| Some(u.id) != my_id).collect();
    let saved_names: Vec<String> = existing_users().iter().map(|u| u.name.clone()).collect();
    let sent_list = sent.read().clone().unwrap_or_default();
    let invited_email = |user_id: i32| -> Option<Option<String>> {
        let s = sent_list.iter().find(|s| s.user_id == user_id)?;
        Some(
            friend_list
                .friends
                .iter()
                .find(|f| f.account_id == s.to_account_id)
                .map(|f| f.email.clone()),
        )
    };
    let invites = invite_count(&drafts.read());
    let other_count = others.len() + drafts.read().len();

    rsx! {
        div { class: "modal modal-open modal-bottom sm:modal-middle", role: "dialog",
            div { class: "modal-box max-w-md p-0 flex flex-col",
                div { class: "flex items-center justify-between px-6 pt-5 pb-4 border-b border-base-200 flex-shrink-0",
                    h3 { class: "font-bold text-lg font-display", {tid!("edit-project-title")} }
                    button {
                        r#type: "button",
                        class: "btn btn-ghost btn-circle h-11 w-11 min-h-11",
                        aria_label: tid!("close"),
                        onclick: move |_| on_close_x.call(()),
                        CloseIcon { size: ICON_HEADER }
                    }
                }
                div { class: "flex-1 overflow-y-auto px-6 py-4",
                div { class: "flex flex-col gap-5",
                    div { class: "flex flex-col gap-3",
                        div {
                            label { class: "text-xs font-semibold text-base-content/70 block mb-1.5", r#for: "edit-project-name", {tid!("add-project-name-label")} }
                            input {
                                id: "edit-project-name",
                                class: "input w-full",
                                r#type: "text",
                                enterkeyhint: "next",
                                value: "{name}",
                                oninput: move |e| name.set(e.value()),
                            }
                        }
                        div { class: "grid grid-cols-[minmax(0,1fr)_7rem] gap-2",
                            div {
                                label { class: "text-xs font-semibold text-base-content/70 block mb-1.5", r#for: "edit-project-description", {tid!("field-description")} }
                                input {
                                    id: "edit-project-description",
                                    class: "input w-full",
                                    r#type: "text",
                                    enterkeyhint: "next",
                                    value: "{description}",
                                    oninput: move |e| description.set(e.value()),
                                }
                            }
                            // Read-only, and it has to stay that way: every expense stores its
                            // amount already converted into this currency, so changing it here would
                            // relabel a whole ledger without re-pricing a single row.
                            div {
                                span { class: "text-xs font-semibold text-base-content/70 block mb-1.5", {tid!("project-currency")} }
                                div {
                                    id: "edit-project-currency",
                                    class: "input w-full bg-base-200 text-base-content/70",
                                    title: tid!("project-currency-locked"),
                                    LockIcon { size: ICON_INLINE }
                                    "{displayed_currency}"
                                }
                            }
                        }
                    }

                    div { class: "flex flex-col gap-2",
                        h4 { class: "text-xs font-bold uppercase tracking-wide text-base-content/70", {tid!("participants-you-badge")} }
                        div { class: "flex items-center gap-2.5 min-h-13 border border-base-200 rounded-box pl-3 pr-1.5 py-1",
                            match me.clone() {
                                Some(u) => rsx! {
                                    span { class: "w-9 h-9 rounded-full bg-secondary text-secondary-content flex items-center justify-center text-sm font-semibold flex-shrink-0", aria_hidden: "true",
                                        {crate::participants::participants_service::initial(&u.name)}
                                    }
                                    span { class: "flex flex-col min-w-0 flex-1 gap-0.5",
                                        span { class: "flex items-center gap-1.5 min-w-0",
                                            span { class: "font-semibold truncate", "{u.name}" }
                                            span { class: "badge badge-secondary badge-sm", {tid!("participants-you-badge")} }
                                        }
                                        span { class: "text-sm text-base-content/70 leading-snug", {tid!("edit-project-you-are", name: u.name.clone())} }
                                    }
                                    if let Some(on_switch) = on_switch {
                                        button {
                                            id: "edit-project-switch",
                                            r#type: "button",
                                            class: "btn btn-ghost text-primary",
                                            onclick: move |_| on_switch.call(()),
                                            {tid!("edit-project-switch")}
                                        }
                                    }
                                },
                                None => rsx! {
                                    span { class: "text-sm text-base-content/70 flex-1", {tid!("edit-project-no-identity")} }
                                    if let Some(on_switch) = on_switch {
                                        button {
                                            id: "edit-project-switch",
                                            r#type: "button",
                                            class: "btn btn-ghost text-primary",
                                            onclick: move |_| on_switch.call(()),
                                            {tid!("edit-project-choose")}
                                        }
                                    }
                                },
                            }
                        }
                    }

                    div { class: "flex flex-col gap-2",
                        div { class: "flex items-center justify-between",
                            h4 { class: "text-xs font-bold uppercase tracking-wide text-base-content/70", {tid!("participants-others")} }
                            span { class: "badge badge-ghost badge-sm", "{other_count}" }
                        }
                        if !others.is_empty() {
                            ul { class: "flex flex-col",
                                for user in others.into_iter() {
                                    {
                                        let user_id = user.id;
                                        let locked = identity_locked(&user, my_id);
                                        let invited = if locked { None } else { invited_email(user_id) };
                                        let friend_name = user.name.clone();
                                        rsx! {
                                            SavedRow {
                                                key: "{user_id}",
                                                id: format!("edit-project-participant-{user_id}"),
                                                name: user.name.clone(),
                                                locked,
                                                claim_name: user.claim_name.clone(),
                                                invited,
                                                on_remove: move |_| {
                                                    existing_users.write().retain(|u| u.id != user_id);
                                                },
                                                on_add_friend: (can_add_friend && locked).then(|| {
                                                    EventHandler::new(move |_| add_friend((user_id, friend_name.clone())))
                                                }),
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        ParticipantsEditor {
                            drafts,
                            taken: saved_names,
                            friends: friend_list.friends.clone(),
                            signed_in: friend_list.signed_in,
                            id_prefix: "edit-project",
                            show_empty: false,
                            mark_new: true,
                        }
                    }

                    if let Some(msg) = error_msg() {
                        div { role: "alert", class: "alert alert-error text-sm", "{msg}" }
                    }
                } // end flex flex-col gap-5
                } // end scrollable body
                div { class: "flex justify-end gap-2 px-6 py-4 border-t border-base-200 flex-shrink-0",
                        button {
                            r#type: "button",
                            class: "btn btn-ghost",
                            onclick: move |_| on_close_cancel.call(()),
                            {tid!("cancel")}
                        }
                        button {
                            id: "edit-project-save",
                            r#type: "button",
                            class: "btn btn-primary",
                            disabled: loading(),
                            onclick: on_submit,
                            if loading() {
                                span { class: "loading loading-spinner loading-sm", role: "status", aria_label: tid!("loading") }
                            }
                            if invites > 0 {
                                {tid!("edit-project-save-invite", count: invites)}
                            } else {
                                {tid!("save")}
                            }
                        }
                    }
                }
            div {
                class: "modal-backdrop",
                onclick: move |_| on_close_backdrop.call(()),
            }
        }
    }
}

#[derive(PartialEq, Props, Clone)]
struct SavedRowProps {
    id: String,
    name: String,
    /// Another account holds this participant as its identity: a padlock naming the holder.
    locked: bool,
    /// The holder's account display name, when the claim carried one.
    claim_name: Option<String>,
    /// `Some` while an invitation this account sent for this participant is pending; the inner
    /// value is the friend's email when they are still in the friends list.
    invited: Option<Option<String>>,
    on_remove: EventHandler<()>,
    /// Sends the holder a friend request. Offered only on a locked row, to a signed-in device.
    on_add_friend: Option<EventHandler<()>>,
}

#[component]
fn SavedRow(props: SavedRowProps) -> Element {
    let claimed_by = match props.claim_name.clone() {
        Some(name) => tid!("identity-claimed-by", name: name),
        None => tid!("identity-claimed"),
    };
    let name = props.name.clone();

    rsx! {
        li { id: "{props.id}", class: "flex items-center gap-2.5 min-h-13",
            Avatar { name: props.name.clone(), guest: !props.locked && props.invited.is_none() }
            span { class: "flex flex-col min-w-0 flex-1 gap-0.5",
                span { class: "flex items-center gap-1.5 min-w-0",
                    span { class: "font-semibold truncate", "{props.name}" }
                    if props.invited.is_some() {
                        span { class: "badge badge-soft badge-info badge-sm", {tid!("participants-invited-badge")} }
                    }
                }
                if props.locked {
                    // Inline rather than a hover tooltip — no hover on touch. `title` is the desktop
                    // affordance only.
                    span { class: "text-sm text-base-content/70 flex items-center gap-1 min-w-0", title: "{claimed_by}",
                        LockIcon { size: 13 }
                        span { class: "truncate", "{claimed_by}" }
                    }
                } else if let Some(invited) = props.invited.clone() {
                    span { class: "text-sm text-base-content/70 truncate",
                        match invited {
                            Some(email) => rsx! { {tid!("participants-invited-sub", email: email)} },
                            None => rsx! { {tid!("participants-invited-pending")} },
                        }
                    }
                } else {
                    span { class: "text-sm text-base-content/70 truncate", {tid!("participants-unlinked")} }
                }
            }
            if let Some(on_add_friend) = props.on_add_friend {
                button {
                    r#type: "button",
                    class: "btn btn-outline btn-xs",
                    onclick: move |_| on_add_friend.call(()),
                    {tid!("friends-add-from-project")}
                }
            }
            button {
                id: "{props.id}-remove",
                r#type: "button",
                class: "btn btn-ghost btn-circle h-11 w-11 min-h-11",
                aria_label: tid!("participants-remove", name: name),
                onclick: move |_| props.on_remove.call(()),
                CloseIcon { size: ICON_INLINE }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypto::{decrypt_json, encrypt_json};
    use crate::decrypted::decrypt_user;
    use chrono::NaiveDateTime;
    use uuid::Uuid;
    use shared::{ProjectPayload, User, UserPayload};

    fn make_editable_project(
        key: &[u8; 32],
        id: Uuid,
        name: &str,
        description: Option<&str>,
    ) -> Result<EditableProject, String> {
        Ok(EditableProject {
            id,
            payload: Some(encrypt_json(key, &ProjectPayload {
                name: name.to_string(),
                currency: "EUR".to_string(),
                description: description.map(|d| d.to_string()),
            })?),
            status: None,
            history: None,
        })
    }

    fn make_creatable_user(
        key: &[u8; 32],
        name: &str,
        project_id: Uuid,
    ) -> Result<CreatableUser, String> {
        Ok(CreatableUser {
            payload: encrypt_json(key, &UserPayload { name: name.to_string() })?,
            project_id,
        })
    }

    use crate::common::test_fixtures::{make_user, test_key};

    #[test]
    fn editable_project_name_roundtrip() {
        let key = test_key();
        let ep = make_editable_project(&key, Uuid::nil(), "Road trip", None).unwrap();
        let pp: ProjectPayload = decrypt_json(&key, ep.payload.as_ref().unwrap()).unwrap();
        assert_eq!(pp.name, "Road trip");
        assert!(pp.description.is_none());
        assert!(ep.status.is_none());
    }

    #[test]
    fn editable_project_with_description() {
        let key = test_key();
        let ep = make_editable_project(&key, Uuid::nil(), "Coloc", Some("Appart Lyon")).unwrap();
        let pp: ProjectPayload = decrypt_json(&key, ep.payload.as_ref().unwrap()).unwrap();
        assert_eq!(pp.description.unwrap(), "Appart Lyon");
    }

    #[test]
    fn editable_project_clears_description() {
        let key = test_key();
        let ep = make_editable_project(&key, Uuid::nil(), "Trip", None).unwrap();
        let pp: ProjectPayload = decrypt_json(&key, ep.payload.as_ref().unwrap()).unwrap();
        assert!(pp.description.is_none());
    }

    #[test]
    fn editable_project_wrong_key_fails() {
        let key = test_key();
        let ep = make_editable_project(&key, Uuid::nil(), "Secret", None).unwrap();
        let mut bad = key;
        bad[0] ^= 0xFF;
        assert!(decrypt_json::<ProjectPayload>(&bad, ep.payload.as_ref().unwrap()).is_err());
    }

    #[test]
    fn editable_project_id_preserved() {
        let key = test_key();
        let id = Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000").unwrap();
        let ep = make_editable_project(&key, id, "x", None).unwrap();
        assert_eq!(ep.id, id);
    }

    #[test]
    fn new_user_name_roundtrip() {
        let key = test_key();
        let cu = make_creatable_user(&key, "Claire", Uuid::nil()).unwrap();
        let up: UserPayload = decrypt_json(&key, &cu.payload).unwrap();
        assert_eq!(up.name, "Claire");
    }

    #[test]
    fn new_user_wrong_key_fails() {
        let key = test_key();
        let cu = make_creatable_user(&key, "Dave", Uuid::nil()).unwrap();
        let mut bad = key;
        bad[2] ^= 0x12;
        assert!(decrypt_json::<UserPayload>(&bad, &cu.payload).is_err());
    }

    #[test]
    fn existing_users_decrypted_for_display() {
        let key = test_key();
        let raw_users = [make_user(&key, 1, "Alice"),
            make_user(&key, 2, "Bob")];
        let decrypted: Vec<DecryptedUser> =
            raw_users.iter().filter_map(|u| decrypt_user(&key, u).ok()).collect();
        assert_eq!(decrypted.len(), 2);
        assert_eq!(decrypted[0].name, "Alice");
        assert_eq!(decrypted[1].name, "Bob");
    }

    #[test]
    fn existing_users_wrong_key_yields_empty() {
        let key = test_key();
        let raw_users = [make_user(&key, 1, "Alice")];
        let mut bad = key;
        bad[0] ^= 0xFF;
        let decrypted: Vec<DecryptedUser> =
            raw_users.iter().filter_map(|u| decrypt_user(&bad, u).ok()).collect();
        assert!(decrypted.is_empty());
    }

    #[test]
    fn me_lookup_in_created_users() {
        let key = test_key();
        let created = [make_user(&key, 10, "Alice"),
            make_user(&key, 11, "Bob")];
        let found = created.iter().find(|u| {
            decrypt_json::<UserPayload>(&key, &u.payload)
                .map(|p| p.name == "Bob")
                .unwrap_or(false)
        });
        assert_eq!(found.unwrap().id, 11);
    }

    #[test]
    fn me_lookup_not_found_returns_none() {
        let key = test_key();
        let created = [make_user(&key, 10, "Alice")];
        let found = created.iter().find(|u| {
            decrypt_json::<UserPayload>(&key, &u.payload)
                .map(|p| p.name == "Unknown")
                .unwrap_or(false)
        });
        assert!(found.is_none());
    }

    #[test]
    fn user_with_created_at() {
        let key = test_key();
        let u = User {
            id: 99,
            payload: encrypt_json(&key, &UserPayload { name: "Eve".to_string() }).unwrap(),
            created_at: Some(NaiveDateTime::default()),
            ..Default::default()
        };
        let du = decrypt_user(&key, &u).unwrap();
        assert_eq!(du.id, 99);
        assert_eq!(du.name, "Eve");
    }
}
