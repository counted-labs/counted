use api::account_projects::account_projects_controller::upsert_account_project;
use api::projects::projects_controller::update_project_by_id;
use api::users::users_controller::{add_user, delete_user};
use dioxus::fullstack::Json;
use dioxus::prelude::*;
use crate::tid;
use shared::{Account, CreatableUser, CreatableUserBatch, EditableProject, HistoryContext, HistoryPayload, ProjectDto, ProjectPayload, UpsertAccountProject, User, UserPayload};
use std::collections::VecDeque;
use uuid::Uuid;

use crate::common::{
    error_message, identity_locked, is_claim_proof_error, is_identity_taken_error, update_ls,
    upsert_project, write_queue, Flash, LocalStorageState, OpKind, ProjectKey, QueuedOp,
};
use std::collections::HashSet;
use crate::crypto::{
    claim_label, claim_token, decrypt_json, decrypt_user, encrypt_json, DecryptedUser,
};
use crate::icons::{LockIcon, TrashIcon, UserIcon, ICON_INLINE};

#[derive(PartialEq, Props, Clone)]
pub struct EditProjectModalProps {
    pub project: ProjectDto,
    pub users: Vec<User>,
    pub on_close: EventHandler<()>,
    pub on_saved: EventHandler<()>,
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
    let mut new_user_input = use_signal(String::new);
    let mut new_users: Signal<Vec<String>> = use_signal(Vec::new);
    // None means "use localStorage"; Some(name) means user explicitly picked
    let mut selected_me: Signal<Option<String>> = use_signal(|| None);
    let mut loading = use_signal(|| false);
    let mut error_msg: Signal<Option<String>> = use_signal(|| None);

    let original_user_ids: Vec<i32> = props.users.iter().map(|u| u.id).collect();

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
        let current_new = new_users();
        let selected = selected_me();
        let orig_ids = original_user_ids.clone();
        let currency = initial_currency.clone();

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
            // Update "me" from existing users only (new users have no server ID yet).
            let me_is_new = selected.as_ref().map(|name| {
                current_existing.iter().all(|u| u.name != name.as_str())
                    && current_new.iter().any(|n| n == name)
            }).unwrap_or(false);
            if let Some(ref me_name) = selected {
                if !me_is_new {
                    if let Some(uid) = current_existing.iter().find(|u| u.name == me_name.as_str()).map(|u| u.id) {
                        update_ls(ls_ctx, |state| upsert_project(state, project_id, Some(uid)));
                    }
                }
            }
            // Warn about anything that can't be applied offline.
            let existing_ids: HashSet<i32> = current_existing.iter().map(|u| u.id).collect();
            let has_deletions = orig_ids.iter().any(|id| !existing_ids.contains(id));
            let mut deferred: Vec<String> = Vec::new();
            if !current_new.is_empty() { deferred.push(tid!("edit-project-deferred-new-members")); }
            if has_deletions { deferred.push(tid!("edit-project-deferred-removals")); }
            if me_is_new { deferred.push(tid!("edit-project-deferred-me")); }
            if !deferred.is_empty() {
                flash.set(Some(Flash::ok(
                    tid!("edit-project-offline-deferred", items: deferred.join(", ")),
                )));
            }
            on_saved.call(());
            return;
        }

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
                for n in &current_new {
                    let payload = match encrypt_json(&key, &UserPayload { name: n.clone() }) {
                        Ok(v) => v,
                        Err(e) => { error_msg.set(Some(e)); loading.set(false); return; }
                    };
                    creatables.push(CreatableUser { payload, project_id, invited_email: None });
                }
                match add_user(Json(CreatableUserBatch::Multiple(creatables))).await {
                    Ok(users) => created_users = users,
                    Err(e) => {
                        error_msg.set(Some(error_message(&e)));
                        loading.set(false);
                        return;
                    }
                }
            }

            let kept_ids: std::collections::HashSet<i32> =
                current_existing.iter().map(|u| u.id).collect();
            for uid in &orig_ids {
                if !kept_ids.contains(uid) {
                    if let Err(e) = delete_user(project_id, *uid).await {
                        error_msg.set(Some(error_message(&e)));
                        loading.set(false);
                        return;
                    }
                }
            }

            if let Some(me_name) = selected {
                let user_id = current_existing
                    .iter()
                    .find(|u| u.name == me_name)
                    .map(|u| u.id)
                    .or_else(|| {
                        created_users
                            .iter()
                            .find(|u| {
                                decrypt_json::<UserPayload>(&key, &u.payload)
                                    .map(|p| p.name == me_name)
                                    .unwrap_or(false)
                            })
                            .map(|u| u.id)
                    });
                if let Some(uid) = user_id {
                    // Changing identity has to reach the server: the row is updated in place, so
                    // the participant left behind is released in the same statement.
                    //
                    // Nothing here aborts the save. By this point the project, the new participants
                    // and the removals are already committed server-side, so returning early would
                    // report a failure for work that succeeded and leave the modal open — inviting a
                    // re-submit that adds every new participant a second time. The identity is the
                    // last and least of what this form does; it reports itself through the flash and
                    // lets the rest land.
                    if let Some(account) = auth_ctx() {
                        let label = account_key_ctx()
                            .and_then(|ak| claim_label(&account, &ak, &key));
                        match upsert_account_project(Json(UpsertAccountProject {
                            project_id,
                            user_id: Some(uid),
                            key: None,
                            claim_label: label,
                            claim_token: Some(claim_token(&key).to_vec()),
                        }))
                        .await
                        {
                            Ok(()) => {
                                update_ls(ls_ctx, |s| upsert_project(s, project_id, Some(uid)));
                            }
                            // Refused: another account holds that participant. The local identity is
                            // deliberately left alone — the previous one is still the true one.
                            Err(e) if is_identity_taken_error(&e) => {
                                flash.set(Some(Flash::err(tid!("error-identity-taken"))));
                            }
                            // Refused for want of the key: recording it locally would only have
                            // `ensure_membership` refused again on every open.
                            Err(e) if is_claim_proof_error(&e) => {
                                flash.set(Some(Flash::err(tid!("error-claim-proof-invalid"))));
                            }
                            // Never reached the server, so nothing was refused. Record it locally;
                            // `ensure_membership` pushes it on the next open and clears it there if
                            // the server turns it down.
                            _ => {
                                update_ls(ls_ctx, |s| upsert_project(s, project_id, Some(uid)));
                            }
                        }
                    } else {
                        update_ls(ls_ctx, |state| upsert_project(state, project_id, Some(uid)));
                    }
                }
            }

            on_saved.call(());
        });
    };

    let stored_user_id = move || {
        ls_ctx().projects.iter().find(|p| p.project_id == project_id).and_then(|p| p.user_id)
    };

    let is_me = move |name: &str| -> bool {
        if let Some(ref sel) = selected_me() {
            sel == name
        } else {
            let stored_uid = stored_user_id();
            existing_users().iter().any(|u| u.name == name && Some(u.id) == stored_uid)
        }
    };

    rsx! {
        div { class: "modal modal-open modal-bottom sm:modal-middle", role: "dialog",
            div { class: "modal-box max-w-md p-0 flex flex-col",
                div { class: "flex items-center justify-between px-6 pt-5 pb-4 border-b border-base-200 flex-shrink-0",
                    h3 { class: "font-bold text-lg font-display", {tid!("edit-project-title")} }
                    button {
                        r#type: "button",
                        class: "btn btn-ghost btn-circle h-11 w-11 min-h-11 text-lg",
                        aria_label: tid!("close"),
                        onclick: move |_| on_close_x.call(()),
                        "✕"
                    }
                }
                div { class: "flex-1 overflow-y-auto px-6 py-4",
                div { class: "flex flex-col gap-3",
                    fieldset { class: "fieldset",
                        label { class: "fieldset-legend", r#for: "edit-project-name", {tid!("field-name")} }
                        input {
                            id: "edit-project-name",
                            class: "input w-full",
                            r#type: "text",
                            enterkeyhint: "next",
                            value: "{name}",
                            oninput: move |e| name.set(e.value()),
                        }

                        label { class: "fieldset-legend", r#for: "edit-project-description", {tid!("field-description")} }
                        input {
                            id: "edit-project-description",
                            class: "input w-full",
                            r#type: "text",
                            enterkeyhint: "next",
                            value: "{description}",
                            oninput: move |e| description.set(e.value()),
                        }

                        // Read-only, and it has to stay that way: every expense stores its amount
                        // already converted into this currency, so changing it here would relabel a
                        // whole ledger without re-pricing a single row.
                        label { class: "fieldset-legend", r#for: "edit-project-currency", {tid!("project-currency")} }
                        input {
                            id: "edit-project-currency",
                            class: "input w-full",
                            r#type: "text",
                            disabled: true,
                            value: "{displayed_currency}",
                        }
                        p { class: "label text-xs", {tid!("project-currency-locked")} }
                    }

                    fieldset { class: "fieldset bg-base-200 border-base-300 rounded-box border p-4",
                        legend { class: "fieldset-legend", {tid!("add-project-participants")} }

                        div { class: "flex gap-2 mb-3",
                            label { class: "input flex-1",
                                UserIcon {}
                                input {
                                    r#type: "text",
                                    aria_label: tid!("add-project-participant-name"),
                                    placeholder: tid!("add-project-participant-placeholder"),
                                    enterkeyhint: "done",
                                    autocapitalize: "words",
                                    autocomplete: "off",
                                    value: "{new_user_input}",
                                    oninput: move |e| new_user_input.set(e.value()),
                                    // Enter adds the participant instead of submitting. Nothing
                                    // blurs, so the keyboard stays up and the view does not reflow.
                                    onkeydown: move |e| {
                                        if e.key() == Key::Enter {
                                            e.prevent_default();
                                            let input = new_user_input().trim().to_string();
                                            if !input.is_empty() {
                                                new_users.write().push(input);
                                                new_user_input.set(String::new());
                                            }
                                        }
                                    },
                                }
                            }
                            button {
                                r#type: "button",
                                class: "btn btn-neutral",
                                onclick: move |_| {
                                    let input = new_user_input().trim().to_string();
                                    if !input.is_empty() {
                                        new_users.write().push(input);
                                        new_user_input.set(String::new());
                                    }
                                },
                                {tid!("add")}
                            }
                        }

                        ul { class: "flex flex-col gap-1 max-h-48 overflow-y-auto",
                            for user in existing_users().into_iter() {
                                {
                                    let user_id = user.id;
                                    let user_name = user.name.clone();
                                    let me = is_me(&user_name);
                                    let locked = identity_locked(&user, stored_user_id());
                                    let friend_name = user.name.clone();
                                    rsx! {
                                        ParticipantRow {
                                            name: user.name.clone(),
                                            is_new: false,
                                            is_me: me,
                                            is_locked: locked,
                                            claim_name: user.claim_name.clone(),
                                            on_remove: move |_| {
                                                existing_users.write().retain(|u| u.id != user_id);
                                                if selected_me() == Some(user_name.clone()) {
                                                    selected_me.set(None);
                                                }
                                            },
                                            on_claim: move |_| selected_me.set(Some(user.name.clone())),
                                            on_add_friend: (can_add_friend && locked).then(|| {
                                                EventHandler::new(move |_| add_friend((user_id, friend_name.clone())))
                                            }),
                                        }
                                    }
                                }
                            }
                            for i in 0..new_users().len() {
                                {
                                    let new_name = new_users().get(i).cloned().unwrap_or_default();
                                    let me = selected_me() == Some(new_name.clone());
                                    rsx! {
                                        ParticipantRow {
                                            name: new_name.clone(),
                                            is_new: true,
                                            is_me: me,
                                            // Not saved yet, so nobody can be holding it.
                                            is_locked: false,
                                            claim_name: None,
                                            on_remove: move |_| {
                                                if selected_me()
                                                    == Some(
                                                        new_users().get(i).cloned().unwrap_or_default(),
                                                    )
                                                {
                                                    selected_me.set(None);
                                                }
                                                new_users.write().remove(i);
                                            },
                                            on_claim: move |_| selected_me.set(Some(new_name.clone())),
                                            on_add_friend: None,
                                        }
                                    }
                                }
                            }
                        }
                    }

                    if let Some(msg) = error_msg() {
                        div { role: "alert", class: "alert alert-error text-sm", "{msg}" }
                    }
                } // end scrollable body
                } // end flex flex-col gap-3
                div { class: "flex justify-end gap-2 px-6 py-4 border-t border-base-200 flex-shrink-0",
                        button {
                            r#type: "button",
                            class: "btn btn-ghost",
                            onclick: move |_| on_close_cancel.call(()),
                            {tid!("cancel")}
                        }
                        button {
                            r#type: "button",
                            class: "btn btn-primary",
                            disabled: loading(),
                            onclick: on_submit,
                            if loading() {
                                span { class: "loading loading-spinner loading-sm", role: "status", aria_label: tid!("loading") }
                            }
                            {tid!("save")}
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
struct ParticipantRowProps {
    name: String,
    /// Not yet saved to the server: shown in italics with a "nouveau" badge.
    is_new: bool,
    is_me: bool,
    /// Another account holds this participant as its identity: "C'est moi !" is replaced by a
    /// padlock naming the holder. Never true for the row this device already is.
    is_locked: bool,
    /// The holder's account display name, when the claim carried one.
    claim_name: Option<String>,
    on_remove: EventHandler<()>,
    on_claim: EventHandler<()>,
    /// Sends the holder a friend request. Offered only on a locked row, to a signed-in device.
    on_add_friend: Option<EventHandler<()>>,
}

#[component]
fn ParticipantRow(props: ParticipantRowProps) -> Element {
    let claimed_by = match props.claim_name.clone() {
        Some(name) => tid!("identity-claimed-by", name: name),
        None => tid!("identity-claimed"),
    };

    rsx! {
        li { class: "flex items-center gap-2 py-1",
            button {
                r#type: "button",
                class: "btn btn-square btn-sm btn-soft",
                aria_label: tid!("add-project-remove-participant"),
                onclick: move |_| props.on_remove.call(()),
                TrashIcon { size: ICON_INLINE }
            }
            span {
                class: if props.is_new { "flex-1 text-sm italic" } else { "flex-1 text-sm" },
                "{props.name}"
            }
            if props.is_new {
                span { class: "badge badge-soft badge-info badge-xs", {tid!("edit-project-new-badge")} }
            }
            if props.is_me {
                div { class: "badge badge-soft badge-accent", {tid!("add-project-me-badge")} }
            } else if props.is_locked {
                // Inline rather than a hover tooltip — no hover on touch. `title` is the desktop
                // affordance only.
                div {
                    class: "badge badge-soft badge-ghost gap-1",
                    title: "{claimed_by}",
                    LockIcon { size: ICON_INLINE }
                    span { class: "text-xs", "{claimed_by}" }
                }
                if let Some(on_add_friend) = props.on_add_friend {
                    button {
                        r#type: "button",
                        class: "btn btn-outline btn-xs",
                        onclick: move |_| on_add_friend.call(()),
                        {tid!("friends-add-from-project")}
                    }
                }
            } else {
                button {
                    r#type: "button",
                    class: "btn btn-outline btn-xs",
                    onclick: move |_| props.on_claim.call(()),
                    {tid!("add-project-thats-me")}
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypto::{decrypt_json, decrypt_user, encrypt_json};
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
            invited_email: None,
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
        assert!(cu.invited_email.is_none());
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
