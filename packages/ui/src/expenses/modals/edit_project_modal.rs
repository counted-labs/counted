use api::friends::invitations_controller::get_sent_invitations;
use dioxus::prelude::*;
use crate::tid;
use shared::{Account, EditableProject, HistoryAction, HistoryEntity, HistoryPayload, ProjectDto, ProjectPayload, User, UserPayload};
use crate::entity_ids::{add_participants, decrypt_row, new_participant, sealed_history};
use uuid::Uuid;

use crate::common::{
    error_message, haptic, identity_locked, project_initial, update_ls, update_project_fresh, enqueue, Queue,
    Avatar, DiscardConfirm, EmojiGrid, Flash, Haptic, SheetBar, SheetFooter, SHEET_TITLE, InviteRetry, LocalStorageState, OpKind, ProjectEdit, ProjectKey, ProjectTile,
    QueuedOp, SizeClass,
};
use std::collections::HashSet;
use crate::crypto::{decrypt_json, encrypt_json, encrypt_padded_json};
use crate::decrypted::{decrypt_user, DecryptedUser};
use crate::friends::friends_service::{invite_each, sender, Invitee};
use crate::icons::{CloseIcon, LockIcon, MoreIcon, UserPlusIcon, ICON_INLINE};
use crate::participants::participants_service::{invite_count, same_name, DraftParticipant};
use crate::participants::{use_friend_list, ParticipantsEditor};
use crate::participants::participants_service::initial;
use crate::participants::participant_write::{placement, reloaded, save_participants, Change, Saved};
use crate::expenses::hooks::use_project_store::ProjectStore;
use crate::expenses::group_security::{
    grant_admin, me_admin, member_actions, revoke_admin, step_down_admin, MemberAction, MemberSheet,
};

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
    /// "Remove from the project" in a member's sheet, by user id: the removal that changes the key.
    /// `None` where this device cannot rotate: the sheet then only hides the participant.
    #[props(default)]
    pub on_remove_member: Option<EventHandler<i32>>,
}

#[component]
pub fn EditProjectModal(props: EditProjectModalProps) -> Element {
    crate::common::use_modal_focus("edit-project-modal");
    let project_id = props.project.id;
    let ls_ctx = use_context::<Signal<LocalStorageState>>();
    let key_ctx = use_context::<ProjectKey>().0;
    let auth_ctx = use_context::<Signal<Option<Account>>>();
    let account_key_ctx = use_context::<Signal<Option<[u8; 32]>>>();
    let is_online = use_context::<Signal<bool>>();
    let pending_ops = use_context::<Signal<Queue>>();
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
    let initial_emoji = initial_project.as_ref().and_then(|p| p.emoji());
    let displayed_currency = initial_project
        .as_ref()
        .map(|p| p.currency.clone())
        .unwrap_or_default();
    let initial_users: Vec<DecryptedUser> = key_snap
        .as_ref()
        .map(|k| props.users.iter().filter_map(|u| decrypt_user(k, u).ok()).collect())
        .unwrap_or_default();

    let mut name = use_signal(|| initial_name.clone());
    let mut description = use_signal(|| initial_description.clone());
    let mut emoji = use_signal(|| initial_emoji.clone());
    let mut emoji_open = use_signal(|| false);
    // Existing users stored as decrypted so display and comparison work directly
    let mut existing_users: Signal<Vec<DecryptedUser>> =
        use_signal(|| initial_users.iter().filter(|u| !u.removed).cloned().collect());
    // Participants are never deleted: a removed one keeps its name on past expenses, and can come back.
    let mut removed_users: Signal<Vec<DecryptedUser>> =
        use_signal(|| initial_users.iter().filter(|u| u.removed).cloned().collect());
    let mut drafts: Signal<Vec<DraftParticipant>> = use_signal(Vec::new);
    let initial_rows = props.users.clone();
    let mut served_users: Signal<Vec<User>> = use_signal(move || initial_rows);
    let mut member_sheet: Signal<Option<i32>> = use_signal(|| None);
    let mut categories_now = use_signal(|| {
        initial_project.as_ref().map(crate::categories::ProjectCategories::from_payload).unwrap_or_default()
    });
    let mut managing_categories = use_signal(|| false);
    let mut stepped_down = use_signal(|| false);
    let mut loading = use_signal(|| false);
    let mut error_msg: Signal<Option<String>> = use_signal(|| None);
    let mut notice: Signal<Option<String>> = use_signal(|| None);
    let mut baseline = use_signal(|| Opened { name: initial_name.clone(), description: initial_description.clone(), emoji: initial_emoji.clone() });
    let mut history_written = use_signal(|| false);

    let original_user_ids: Vec<i32> =
        initial_users.iter().filter(|u| !u.removed).map(|u| u.id).collect();
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
            crate::common::project_token::scope(project_id);
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

    let on_close = props.on_close;
    let named = |users: Signal<Vec<DecryptedUser>>| -> Vec<(i32, String)> {
        users.read().iter().map(|u| (u.id, u.name.clone())).collect()
    };
    let typed = (name(), description(), emoji(), named(existing_users), named(removed_users), drafts());
    let opened = use_hook(|| typed.clone());
    let dirty = typed != opened;
    let mut discarding = use_signal(|| false);
    let mut request_close = move || {
        if dirty {
            discarding.set(true);
        } else {
            on_close.call(());
        }
    };
    let on_saved = props.on_saved;
    let on_switch = props.on_switch;
    let on_remove_member = props.on_remove_member;

    let on_submit = EventHandler::new(move |_: ()| {
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
        let base = baseline.peek().clone();
        let change = project_change(&base, &name_val, desc.clone(), &emoji());

        let current_existing = existing_users();
        let current_removed = removed_users();
        let current_new = drafts();
        let orig_ids = original_user_ids.clone();
        // `u.removed` is what the server holds, the list each one sits in is what it should.
        let changes: Vec<Change> = current_removed
            .iter()
            .map(|u| (u, true))
            .chain(current_existing.iter().map(|u| (u, false)))
            .filter(|(u, removed)| u.removed != *removed)
            .map(|(u, removed)| (u.id, removed))
            .collect();

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

        let actor_user_id = ls_ctx.read()
            .projects
            .iter()
            .find(|p| p.project_id == project_id)
            .and_then(|p| p.user_id);

        // History summaries are encrypted and stored, so they keep the language of whoever wrote
        // them. That is the honest record - the entry says what this person did - and it avoids a
        // payload migration for every summary already written.
        let mut diff: Vec<String> = Vec::new();
        if name_val != base.name {
            diff.push(tid!("history-name-changed", from: base.name.clone(), to: name_val.clone()));
        }
        let new_desc_str = desc.as_deref().unwrap_or("");
        if new_desc_str != base.description.as_str() {
            if base.description.is_empty() {
                diff.push(tid!("history-description-added", value: new_desc_str.to_string()));
            } else if new_desc_str.is_empty() {
                diff.push(tid!("history-description-removed", value: base.description.clone()));
            } else {
                diff.push(tid!(
                    "history-description-changed",
                    from: base.description.clone(),
                    to: new_desc_str.to_string()
                ));
            }
        }
        let summary = if diff.is_empty() {
            tid!("history-project-edited", name: name_val.clone())
        } else {
            diff.join(" • ")
        };

        let history = sealed_history(
            &key,
            HistoryPayload {
                summary,
                actor_user_id,
                action: Some(HistoryAction::Updated),
                entity: Some(HistoryEntity::Project),
                entity_id: None,
                extra: Default::default(),
            },
        )
            .ok();

        if !is_online() {
            if let Some(change) = &change {
                let sealed = match change.name_description.as_ref().map(|nd| encrypt_json(&key, nd)).transpose() {
                    Ok(v) => v,
                    Err(e) => { error_msg.set(Some(e)); loading.set(false); return; }
                };
                let sealed_emoji = match change.emoji.as_ref().map(|e| encrypt_json(&key, e)).transpose() {
                    Ok(v) => v,
                    Err(e) => { error_msg.set(Some(e)); loading.set(false); return; }
                };
                let queued = enqueue(pending_ops, QueuedOp::new(
                    Uuid::new_v4(),
                    &name_val,
                    OpKind::EditProject(ProjectEdit {
                        project_id,
                        name_description: sealed,
                        status: None,
                        history,
                        emoji: sealed_emoji,
                    }),
                ));
                if !queued {
                    return;
                }
            }
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

        let write = project_write(change, !current_new.is_empty() || !changes.is_empty(), *history_written.peek());
        notice.set(None);

        spawn(async move {
            if write != ProjectWrite::Nothing {
                let edit = |stored: &ProjectDto| {
                    let payload = match &write {
                        ProjectWrite::Lay(change) => Some(edited_project_payload(&key, stored, change)?),
                        _ => None,
                    };
                    Ok(EditableProject {
                        id: project_id,
                        payload,
                        status: None,
                        history: history.clone(),
                        expected_iv: None,
                        notify_closed: false,
                        expected_rev: None,
                    })
                };
                if let Err(e) = update_project_fresh(project_id, edit).await {
                    error_msg.set(Some(error_message(&e)));
                    loading.set(false);
                    return;
                }
                history_written.set(true);
                if let ProjectWrite::Lay(change) = &write {
                    baseline.set(base.after(change));
                }
            }

            let mut created_users: Vec<User> = Vec::new();
            if !current_new.is_empty() {
                let creatables = match current_new.iter().map(|d| new_participant(&key, project_id, d.name.clone())).collect::<Result<Vec<_>, _>>() {
                    Ok(v) => v,
                    Err(e) => { error_msg.set(Some(e)); loading.set(false); return; }
                };
                match add_participants(&key, creatables).await {
                    // Moved into the existing list at once: a removal below can still fail (409
                    // for a participant with payments, the ordinary case), and a re-submit used to
                    // insert every new participant a second time.
                    Ok(users) => {
                        existing_users
                            .write()
                            .extend(users.iter().filter_map(|u| decrypt_user(&key, u).ok()));
                        drafts.write().clear();
                        served_users.write().extend(users.iter().cloned());
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
                            decrypt_row::<UserPayload>(&key, &u.payload)
                                .map(|p| same_name(&p.name, &d.name))
                                .unwrap_or(false)
                        })
                        .map(|u| u.id);
                    Some(Invitee { friend, user_id })
                })
                .collect();
            if !invitees.is_empty() {
                let from = account.as_ref().zip(account_key).and_then(|(a, ak)| sender(a, &ak, &ls_ctx.peek()));
                let unsent = invite_each(project_id, &invitees, from, &key).await;
                if !unsent.is_empty() {
                    invite_retry.set(Some(InviteRetry { project_id, unsent }));
                }
            }

            // Removing and restoring both rewrite the payload's `removed`. Re-sending one that already
            // took is harmless, which is what makes a re-submit after a failure safe.
            // A batch without `rev` skips participants already gone, as before; one with it answers
            // 404 for them, and a row changed elsewhere reloads the form instead.
            let served = served_users.peek().clone();
            match save_participants(&key, project_id, &served, &changes).await {
                Ok(Saved::Written) => on_saved.call(()),
                Ok(Saved::Reloaded(fresh)) => {
                    let placed = placement(&existing_users.peek(), &removed_users.peek());
                    match reloaded(&key, &served, &fresh, |id| placed.get(&id).copied()) {
                        Some((kept, removed)) => {
                            existing_users.set(kept);
                            removed_users.set(removed);
                            served_users.set(fresh);
                            notice.set(Some(tid!("participants-reloaded")));
                        }
                        None => error_msg.set(Some(tid!("participants-reload-unreadable"))),
                    }
                    loading.set(false);
                }
                Err(e) => {
                    error_msg.set(Some(error_message(&e)));
                    loading.set(false);
                }
            }
        });
    });

    let stored_user_id = move || {
        ls_ctx.read().projects.iter().find(|p| p.project_id == project_id).and_then(|p| p.user_id)
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
    // The admin role (docs/plans/key-lifecycle.md §8.7), shown once every app understands it.
    let roles_shown = props.project.key_lifecycle && !props.project.read_only;
    let stored_admin = ls_ctx.read().projects.iter().any(|p| p.project_id == project_id && p.admin);
    let i_am_admin =
        roles_shown && auth_ctx.read().is_some() && !stepped_down() && me_admin(stored_admin, &props.users, my_id);
    let admin_count = existing_users().iter().filter(|u| u.claimed && u.admin).count();
    let mut hide = move |user_id: i32| {
        let gone = existing_users.read().iter().find(|u| u.id == user_id).cloned();
        existing_users.write().retain(|u| u.id != user_id);
        if let Some(u) = gone {
            removed_users.write().push(u);
        }
    };
    let mut set_role = move |user_id: i32, admin: bool, done: String| {
        if let Some(u) = existing_users.write().iter_mut().find(|u| u.id == user_id) {
            u.admin = admin;
        }
        flash.set(Some(Flash::ok(done)));
    };
    let mut on_member_action = move |(user_id, member_name, action): (i32, String, MemberAction)| match action {
        MemberAction::MakeAdmin => {
            spawn(async move {
                match grant_admin(project_id, user_id).await {
                    Ok(()) => set_role(user_id, true, tid!("member-made-admin", name: member_name)),
                    Err(e) => flash.set(Some(Flash::err(error_message(&e)))),
                }
            });
        }
        MemberAction::RevokeAdmin => {
            spawn(async move {
                match revoke_admin(project_id, user_id).await {
                    Ok(()) => set_role(user_id, false, tid!("member-revoked-admin", name: member_name)),
                    Err(e) => flash.set(Some(Flash::err(error_message(&e)))),
                }
            });
        }
        MemberAction::StepDown => {
            spawn(async move {
                match step_down_admin(project_id).await {
                    Ok(()) => {
                        stepped_down.set(true);
                        update_ls(ls_ctx, |state| {
                            if let Some(entry) = state.projects.iter_mut().find(|p| p.project_id == project_id) {
                                entry.admin = false;
                            }
                        });
                        set_role(user_id, false, tid!("member-stepped-down"));
                    }
                    Err(e) => flash.set(Some(Flash::err(error_message(&e)))),
                }
            });
        }
        MemberAction::Remove => match on_remove_member {
            Some(on_remove_member) => on_remove_member.call(user_id),
            None => hide(user_id),
        },
    };
    let sheet = member_sheet().and_then(|user_id| {
        let user = existing_users.read().iter().find(|u| u.id == user_id).cloned()?;
        let is_me = Some(user_id) == my_id;
        let subtitle = if is_me {
            tid!("participants-you-badge")
        } else if user.claimed {
            tid!("member-sheet-account")
        } else {
            tid!("participants-unlinked")
        };
        let actions = member_actions(i_am_admin, is_me, user.claimed, user.admin);
        Some((user_id, user.name, subtitle, !user.claimed, actions))
    });
    let name_required = tid!("expense-name-required");
    let name_error = error_msg().filter(|e| *e == name_required);

    rsx! {
        div {
            id: "edit-project-modal",
            class: "modal modal-open modal-bottom sm:modal-middle",
            role: "dialog",
            "data-sheet-dirty": if dirty { "true" },
            aria_modal: "true",
            aria_labelledby: "edit-project-modal-title",
            tabindex: "-1",
            div { class: "modal-box max-w-md p-0 flex flex-col",
                SheetBar {
                    close_id: "edit-project-close",
                    task: true,
                    on_close: move |_| request_close(),
                    h2 { id: "edit-project-modal-title", class: SHEET_TITLE, {tid!("edit-project-title")} }
                }
                div { class: "flex-1 overflow-y-auto px-5 pt-1 pb-4",
                div { class: "flex flex-col gap-5",
                    div { class: "flex flex-col gap-3",
                        div { class: "flex items-end gap-3",
                            div { class: "flex flex-col items-center shrink-0",
                                span { class: "text-xs font-semibold text-base-content/70 block mb-1", {tid!("emoji-picker-label")} }
                                button {
                                    id: "edit-project-emoji",
                                    r#type: "button",
                                    class: if emoji_open() { "size-11 flex items-center justify-center rounded-2xl ring-2 ring-primary" } else { "size-11 flex items-center justify-center rounded-2xl" },
                                    aria_label: format!("{}, {}", tid!("emoji-picker-label"), emoji().unwrap_or_else(|| tid!("emoji-none"))),
                                    aria_expanded: emoji_open(),
                                    aria_controls: "edit-project-emoji-grid",
                                    onclick: move |_| {
                                        haptic(Haptic::Light);
                                        emoji_open.toggle();
                                    },
                                    ProjectTile { name: name(), id: project_id, emoji: emoji() }
                                }
                            }
                            div { class: "flex-1 min-w-0",
                                label { class: "text-xs font-semibold text-base-content/70 block mb-1.5", r#for: "edit-project-name", {tid!("add-project-name-label")} }
                                input {
                                    id: "edit-project-name",
                                    class: "input w-full",
                                    r#type: "text",
                                    enterkeyhint: "next",
                                    aria_invalid: if error_msg().is_some() && name().trim().is_empty() { "true" },
                                    aria_describedby: if name_error.is_some() { "edit-project-name-error" } else if error_msg().is_some() { "edit-project-error" } else { "" },
                                    value: "{name}",
                                    oninput: move |e| name.set(e.value()),
                                }
                                if let Some(err) = name_error.clone() {
                                    p { id: "edit-project-name-error", role: "alert", class: "text-xs text-error mt-1", "{err}" }
                                }
                            }
                        }
                        if emoji_open() {
                            EmojiGrid {
                                id: "edit-project-emoji-grid",
                                selected: emoji(),
                                none_preview: Some(project_initial(&name())),
                                onpick: move |picked| {
                                    emoji.set(picked);
                                    emoji_open.set(false);
                                },
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

                    button {
                        id: "edit-project-categories",
                        r#type: "button",
                        class: "flex items-center gap-3 px-3 py-2.5 min-h-13 rounded-box bg-base-200 text-left",
                        aria_haspopup: "dialog",
                        onclick: move |_| {
                            crate::common::haptic(crate::common::Haptic::Light);
                            managing_categories.set(true);
                        },
                        span { class: "text-xl", aria_hidden: "true", "🏷️" }
                        span { class: "flex-1 min-w-0 flex flex-col",
                            span { class: "font-semibold", {tid!("categories-title")} }
                            span { class: "text-xs text-base-content/70 truncate",
                                {crate::expenses::category_manager::summary_text(&categories_now.read())}
                            }
                        }
                        crate::icons::ChevronRightIcon { size: ICON_INLINE }
                    }

                    div { class: "flex flex-col gap-2",
                        h4 { class: "text-xs font-bold uppercase tracking-wide text-base-content/70", {tid!("participants-you-badge")} }
                        div { class: "flex items-center gap-2 min-h-13 border border-base-200 rounded-box pl-3 pr-1 py-1.5",
                            match me.clone() {
                                Some(u) => rsx! {
                                    Avatar { initials: initial(&u.name), size: SizeClass::W9 }
                                    span { class: "flex flex-col min-w-0 flex-1 gap-0.5",
                                        span { class: "flex flex-wrap items-center gap-x-1 gap-y-0.5 min-w-0",
                                            span { class: "font-semibold truncate max-w-full", "{u.name}" }
                                            span { class: "badge badge-secondary badge-sm", {tid!("participants-you-badge")} }
                                            if i_am_admin {
                                                button {
                                                    id: "edit-project-you-actions",
                                                    r#type: "button",
                                                    class: "badge badge-primary badge-sm gap-0.5 pr-1.5 cursor-pointer relative after:absolute after:-inset-3",
                                                    aria_label: format!("{}, {}", tid!("participants-admin-badge"), tid!("member-actions", name: u.name.clone())),
                                                    aria_haspopup: "dialog",
                                                    onclick: move |_| {
                                                        crate::common::haptic(crate::common::Haptic::Light);
                                                        member_sheet.set(Some(u.id));
                                                    },
                                                    {tid!("participants-admin-badge")}
                                                    MoreIcon { size: 12 }
                                                }
                                            }
                                        }
                                        span { class: "text-[0.8125rem] text-base-content/70 leading-snug", {tid!("edit-project-you-are", name: u.name.clone())} }
                                    }
                                    if let Some(on_switch) = on_switch {
                                        button {
                                            id: "edit-project-switch",
                                            r#type: "button",
                                            class: "btn btn-ghost text-primary px-2 shrink-0",
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
                            span { class: "flex items-center gap-1.5",
                                if roles_shown && admin_count > 0 {
                                    span { id: "edit-project-admin-count", class: "badge badge-soft badge-primary badge-sm", {tid!("edit-project-admins", count: admin_count)} }
                                }
                                span { class: "badge badge-ghost badge-sm", "{other_count}" }
                            }
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
                                                admin: roles_shown && user.admin,
                                                on_actions: i_am_admin.then(|| EventHandler::new(move |_| member_sheet.set(Some(user_id)))),
                                                claim_name: user.claim_name.clone(),
                                                invited,
                                                on_remove: move |_| hide(user_id),
                                                on_add_friend: (can_add_friend && locked).then(|| {
                                                    EventHandler::new(move |_| add_friend((user_id, friend_name.clone())))
                                                }),
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        if !removed_users().is_empty() {
                            h4 { class: "text-xs font-bold uppercase tracking-wide text-base-content/70 mt-2", {tid!("participants-removed")} }
                            ul { class: "flex flex-col",
                                for user in removed_users() {
                                    {
                                        let user_id = user.id;
                                        rsx! {
                                            li {
                                                key: "{user_id}",
                                                id: "edit-project-removed-{user_id}",
                                                class: "flex items-center gap-2.5 min-h-13",
                                                Avatar { initials: initial(&user.name), size: SizeClass::W9, guest: true }
                                                span { class: "flex-1 min-w-0 truncate text-base-content/70", "{user.name}" }
                                                button {
                                                    r#type: "button",
                                                    class: "btn btn-ghost btn-sm text-primary",
                                                    onclick: move |_| {
                                                        let back = removed_users.read().iter().find(|u| u.id == user_id).cloned();
                                                        if let Some(u) = back {
                                                            removed_users.write().retain(|u| u.id != user_id);
                                                            existing_users.write().push(u);
                                                        }
                                                    },
                                                    {tid!("participant-restore")}
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        if roles_shown && props.project.has_admin {
                            p { id: "edit-project-succession", class: "text-xs text-base-content/70 leading-snug", {tid!("edit-project-succession")} }
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

                    if let Some(msg) = notice() {
                        div { id: "edit-project-reloaded", role: "status", class: "alert alert-warning text-sm", "{msg}" }
                    }
                    if let Some(msg) = error_msg().filter(|_| name_error.is_none()) {
                        div { id: "edit-project-error", role: "alert", class: "alert alert-error text-sm", "{msg}" }
                    }
                } // end flex flex-col gap-5
                } // end scrollable body
                SheetFooter {
                        on_cancel: move |_| request_close(),
                        button {
                            id: "edit-project-save",
                            r#type: "button",
                            class: "btn btn-primary",
                            disabled: loading(),
                            onclick: move |_| on_submit.call(()),
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
                onclick: move |_| request_close(),
            }
        }
        if let Some((user_id, member_name, subtitle, guest, actions)) = sheet {
            MemberSheet {
                name: member_name.clone(),
                subtitle,
                guest,
                actions,
                remove_rotates: on_remove_member.is_some(),
                on_action: move |action| on_member_action((user_id, member_name.clone(), action)),
                on_close: move |_| member_sheet.set(None),
            }
        }
        if managing_categories() {
            crate::expenses::category_manager::CategoryManager {
                project_id,
                initial: categories_now(),
                on_close: move |_| managing_categories.set(false),
                on_changed: move |next| categories_now.set(next),
            }
        }
        if discarding() {
            DiscardConfirm {
                on_keep: move |_| discarding.set(false),
                on_discard: move |_| {
                    discarding.set(false);
                    on_close.call(());
                },
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
    #[props(default)]
    admin: bool,
    /// The member sheet (artboard T3), for an admin.
    #[props(default)]
    on_actions: Option<EventHandler<()>>,
}

#[component]
fn SavedRow(props: SavedRowProps) -> Element {
    let claimed_by = match props.claim_name.clone() {
        Some(name) => tid!("identity-claimed-by", name: name),
        None => tid!("identity-claimed"),
    };
    let name = props.name.clone();
    let on_actions = props.on_actions;

    rsx! {
        li {
            id: "{props.id}",
            class: "flex items-center gap-2.5 min-h-13",
            "data-long-press": on_actions.map(|_| format!("{}-long-press", props.id)),
            if let Some(on_actions) = on_actions {
                button {
                    id: "{props.id}-long-press",
                    r#type: "button",
                    hidden: true,
                    tabindex: "-1",
                    onclick: move |_| {
                        crate::common::haptic(crate::common::Haptic::Medium);
                        on_actions.call(());
                    },
                }
            }
            Avatar { initials: initial(&props.name), size: SizeClass::W9, guest: !props.locked && props.invited.is_none() }
            span { class: "flex flex-col min-w-0 flex-1 gap-0.5",
                span { class: "flex items-center gap-1.5 min-w-0",
                    span { class: "font-semibold truncate", "{props.name}" }
                    if props.invited.is_some() {
                        span { class: "badge badge-soft badge-info badge-sm", {tid!("participants-invited-badge")} }
                    }
                    if props.admin {
                        span { class: "badge badge-primary badge-sm", {tid!("participants-admin-badge")} }
                    }
                }
                if props.locked {
                    // Inline rather than a hover tooltip - no hover on touch. `title` is the desktop
                    // affordance only.
                    span { class: "text-[0.8125rem] text-base-content/70 leading-snug flex items-start gap-1 min-w-0", title: "{claimed_by}",
                        span { class: "shrink-0 mt-0.5", LockIcon { size: 13 } }
                        span { class: "min-w-0 break-words", "{claimed_by}" }
                    }
                } else if let Some(invited) = props.invited.clone() {
                    span { class: "text-[0.8125rem] text-base-content/70 truncate",
                        match invited {
                            Some(email) => rsx! { {tid!("participants-invited-sub", email: email)} },
                            None => rsx! { {tid!("participants-invited-pending")} },
                        }
                    }
                } else {
                    span { class: "text-[0.8125rem] text-base-content/70 truncate", {tid!("participants-unlinked")} }
                }
            }
            div { class: "flex items-center shrink-0",
                if let Some(on_add_friend) = props.on_add_friend {
                    button {
                        r#type: "button",
                        class: "btn btn-ghost h-11 w-11 min-h-11 p-0 shrink-0 text-primary sm:w-auto sm:px-3",
                        aria_label: tid!("friends-add-from-project"),
                        title: tid!("friends-add-from-project"),
                        onclick: move |_| on_add_friend.call(()),
                        UserPlusIcon { size: ICON_INLINE }
                        span { class: "hidden sm:inline", {tid!("friends-add-from-project")} }
                    }
                }
                if let Some(on_actions) = props.on_actions {
                    button {
                        id: "{props.id}-actions",
                        r#type: "button",
                        class: "btn btn-ghost btn-circle h-11 w-11 min-h-11",
                        aria_label: tid!("member-actions", name: props.name.clone()),
                        aria_haspopup: "dialog",
                        onclick: move |_| {
                            crate::common::haptic(crate::common::Haptic::Light);
                            on_actions.call(());
                        },
                        MoreIcon { size: ICON_INLINE }
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
}

/// `Some` only when the user changed it, so a save leaves alone an emoji another member set since
/// this sheet opened.
fn emoji_change(initial: &Option<String>, current: &Option<String>) -> Option<Option<String>> {
    (initial != current).then(|| current.clone())
}

/// What a save writes on the project row: name with description, and the emoji, each only when the
/// user changed it since the sheet opened. Another device's change to a part left alone is kept; when
/// both changed the same part, the last save wins until the queued project writes ask.
#[derive(Clone, Debug, PartialEq)]
struct ProjectChange {
    name_description: Option<(String, Option<String>)>,
    emoji: Option<Option<String>>,
}

/// The project fields as this opening of the sheet first showed them, then as its last write
/// landed. Held in a signal: the props refresh under an open sheet, and comparing with them would
/// take another device's change for this user's.
#[derive(Clone, Debug, PartialEq)]
struct Opened {
    name: String,
    description: String,
    emoji: Option<String>,
}

impl Opened {
    fn after(&self, change: &ProjectChange) -> Opened {
        let mut next = self.clone();
        if let Some((name, description)) = &change.name_description {
            next.name = name.clone();
            next.description = description.clone().unwrap_or_default();
        }
        if let Some(emoji) = &change.emoji {
            next.emoji = emoji.clone();
        }
        next
    }
}

/// `None` when the save changes nothing on the project row.
fn project_change(opened: &Opened, name: &str, description: Option<String>, emoji: &Option<String>) -> Option<ProjectChange> {
    let touched = name != opened.name || description.as_deref().unwrap_or("") != opened.description;
    let name_description = touched.then(|| (name.to_string(), description));
    let emoji = emoji_change(&opened.emoji, emoji);
    (name_description.is_some() || emoji.is_some()).then_some(ProjectChange { name_description, emoji })
}

#[derive(Clone, Debug, PartialEq)]
enum ProjectWrite {
    Nothing,
    Lay(ProjectChange),
    /// The row as stored, written again only to carry the history entry of a save that changed
    /// participants alone, as every save did before: once per save, never again on a resubmit.
    Touch,
}

fn project_write(change: Option<ProjectChange>, participants_changed: bool, history_written: bool) -> ProjectWrite {
    match change {
        Some(change) => ProjectWrite::Lay(change),
        None if participants_changed && !history_written => ProjectWrite::Touch,
        None => ProjectWrite::Nothing,
    }
}

/// `stored` with the parts of `change` laid over it. Everything else - currency, status and fields
/// from newer builds - is kept as stored.
fn edited_project_payload(key: &[u8; 32], stored: &ProjectDto, change: &ProjectChange) -> Result<shared::EncryptedPair, String> {
    let mut payload: ProjectPayload = decrypt_json(key, &stored.payload)?;
    if let Some((name, description)) = &change.name_description {
        payload.name = name.clone();
        payload.description = description.clone();
    }
    if let Some(emoji) = &change.emoji {
        payload.set_emoji(emoji.as_deref());
    }
    encrypt_padded_json(key, &payload)
}

#[cfg(test)]
mod tests {
    use shared::time::DateTime;
    use super::*;
    use crate::crypto::{decrypt_json, encrypt_json};
    use crate::decrypted::decrypt_user;
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
                status: None,
                extra: Default::default(),
            })?),
            status: None,
            history: None,
            expected_iv: None,
            notify_closed: false,
            expected_rev: None,
        })
    }

    #[test]
    fn editing_keeps_what_the_form_does_not_own() {
        let key = test_key();
        let stored: ProjectPayload = serde_json::from_value(serde_json::json!({
            "name": "Coloc", "currency": "CHF", "description": "old", "status": "closed",
            "finance": { "budgets": [] },
        }))
        .unwrap();
        let project = ProjectDto {
            payload: encrypt_json(&key, &stored).unwrap(),
            ..crate::common::test_fixtures::make_project(&key, Uuid::nil(), "x", "EUR")
        };

        let change = project_change(&Opened { name: "Coloc".into(), description: "old".into(), emoji: None }, "Flat", None, &None).unwrap();
        let pair = edited_project_payload(&key, &project, &change).unwrap();

        let edited: serde_json::Value = decrypt_json(&key, &pair).unwrap();
        assert_eq!(
            edited,
            serde_json::json!({
                "name": "Flat", "currency": "CHF", "status": "closed", "finance": { "budgets": [] },
            })
        );
    }

    #[test]
    fn saving_the_emoji_keeps_every_other_field() {
        let key = test_key();
        let stored: ProjectPayload = serde_json::from_value(serde_json::json!({
            "name": "Coloc", "currency": "CHF", "emoji": "🏠", "budgets": { "monthly": { "Nourriture": 300.0 } },
        }))
        .unwrap();
        let project = ProjectDto {
            payload: encrypt_json(&key, &stored).unwrap(),
            ..crate::common::test_fixtures::make_project(&key, Uuid::nil(), "x", "EUR")
        };
        let edited = |emoji: Option<Option<&str>>| -> serde_json::Value {
            let change = ProjectChange { name_description: None, emoji: emoji.map(|e| e.map(str::to_string)) };
            decrypt_json(&key, &edited_project_payload(&key, &project, &change).unwrap()).unwrap()
        };

        assert_eq!(
            edited(Some(Some("🏡"))),
            serde_json::json!({
                "name": "Coloc", "currency": "CHF", "emoji": "🏡", "budgets": { "monthly": { "Nourriture": 300.0 } },
            })
        );
        assert_eq!(edited(Some(None)).get("emoji"), None, "back to the initial");
        assert_eq!(edited(None)["emoji"], "🏠", "untouched, it stays as stored");
    }

    #[test]
    fn a_save_that_did_not_touch_the_name_keeps_the_one_written_elsewhere() {
        let key = test_key();
        let fresh = ProjectDto {
            payload: encrypt_json(&key, &serde_json::json!({ "name": "Vacances 2027", "currency": "EUR", "description": "Lac" })).unwrap(),
            ..crate::common::test_fixtures::make_project(&key, Uuid::nil(), "x", "EUR")
        };
        let change = project_change(&Opened { name: "Vacances".into(), description: "".into(), emoji: None }, "Vacances", None, &Some("🏝️".to_string())).unwrap();
        let pair = edited_project_payload(&key, &fresh, &change).unwrap();
        let edited: serde_json::Value = decrypt_json(&key, &pair).unwrap();
        assert_eq!((edited["name"].as_str(), edited["description"].as_str(), edited["emoji"].as_str()), (Some("Vacances 2027"), Some("Lac"), Some("🏝️")));
    }

    #[test]
    fn a_save_that_changes_nothing_on_the_project_row_writes_none() {
        let opened = &Opened { name: "Vacances".into(), description: "Lac".into(), emoji: Some("🏝️".into()) };
        assert_eq!(project_change(opened, "Vacances", Some("Lac".into()), &Some("🏝️".to_string())), None);
        let renamed = project_change(opened, "Été", Some("Lac".into()), &Some("🏝️".to_string())).unwrap();
        assert_eq!((renamed.name_description, renamed.emoji), (Some(("Été".to_string(), Some("Lac".to_string()))), None));
        let cleared = project_change(opened, "Vacances", None, &Some("🏝️".to_string())).unwrap();
        assert_eq!(cleared.name_description, Some(("Vacances".to_string(), None)), "a cleared description is a change");
    }

    /// The props moved to another device's rename while the sheet stayed open: the field still
    /// shows what this user saw, which is no change of theirs.
    #[test]
    fn the_opening_values_are_the_baseline_not_the_refreshed_row() {
        let opened = &Opened { name: "Vacances".into(), description: "".into(), emoji: None };
        assert_eq!(project_change(opened, "Vacances", None, &None), None);
    }

    #[test]
    fn going_back_to_the_opening_value_after_a_landed_write_is_a_change() {
        let opened = Opened { name: "Vacances".into(), description: String::new(), emoji: None };
        let first = project_change(&opened, "X", None, &None).unwrap();
        let landed = opened.after(&first);
        assert_eq!(landed.name, "X");
        let back = project_change(&landed, "Vacances", None, &None).unwrap();
        assert_eq!(back.name_description, Some(("Vacances".to_string(), None)));
        let emoji = Opened { emoji: Some("🏝️".into()), ..landed.clone() }.after(&ProjectChange { name_description: None, emoji: Some(None) });
        assert_eq!((emoji.name.as_str(), emoji.emoji), ("X", None));
    }

    #[test]
    fn a_save_of_participants_alone_carries_its_history_once_and_nothing_writes_nothing() {
        let change = ProjectChange { name_description: Some(("X".into(), None)), emoji: None };
        assert_eq!(project_write(Some(change.clone()), true, true), ProjectWrite::Lay(change));
        assert_eq!(project_write(None, true, false), ProjectWrite::Touch);
        assert_eq!(project_write(None, true, true), ProjectWrite::Nothing, "a resubmit after a participant failure");
        assert_eq!(project_write(None, false, false), ProjectWrite::Nothing);
    }

    #[test]
    fn only_a_changed_emoji_is_written() {
        let island = Some("🏝️".to_string());
        assert_eq!(emoji_change(&island, &island), None);
        assert_eq!(emoji_change(&None, &island), Some(island.clone()));
        assert_eq!(emoji_change(&island, &None), Some(None));
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
        let cu = crate::entity_ids::new_participant(&key, Uuid::nil(), "Claire".into()).unwrap();
        let up: UserPayload = decrypt_json(&key, &cu.payload).unwrap();
        assert_eq!(up.name, "Claire");
    }

    #[test]
    fn new_user_wrong_key_fails() {
        let key = test_key();
        let cu = crate::entity_ids::new_participant(&key, Uuid::nil(), "Dave".into()).unwrap();
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
            payload: encrypt_json(&key, &UserPayload { name: "Eve".to_string(), removed: false, extra: Default::default() }).unwrap(),
            created_at: Some(DateTime::default()),
            ..Default::default()
        };
        let du = decrypt_user(&key, &u).unwrap();
        assert_eq!(du.id, 99);
        assert_eq!(du.name, "Eve");
    }
}
