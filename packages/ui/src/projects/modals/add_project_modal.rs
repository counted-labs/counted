use api::account_projects::account_projects_controller::upsert_account_project;
use api::projects::projects_controller::add_project;
use api::users::users_controller::add_user;
use dioxus::fullstack::Json;
use dioxus::prelude::*;
use crate::tid;
use shared::{
    Account as AccountData, CreatableProject, CreatableUser, CreatableUserBatch, ProjectPayload,
    UpsertAccountProject, UserPayload,
};

use crate::common::creator_proof::{keep_creator_secret, new_creator_proof};
use crate::common::{
    error_message, haptic, project_initial, tile_class, tile_text, update_ls, CurrencyPicker, DiscardConfirm, EmojiGrid, FormSheetBar, Haptic,
    upsert_project, upsert_project_key, InviteRetry, LocalStorageState,
};
use crate::crypto::{
    claim_token, claim_verifier, decrypt_json, encrypt_padded_json, generate_key, key_to_fragment,
    wrap_key, WrapUse,
};
use crate::friends::friends_service::{invite_each, sender, Invitee};
use crate::participants::participants_service::{
    can_create, invite_count, same_name, taken_names, DraftParticipant,
};
use crate::participants::{use_friend_list, use_my_display_name, ParticipantsEditor};
use crate::route::Route;
use uuid::Uuid;

/// "Recreate the group" (D8): the same name and participants in a new group, so a new key and link.
#[derive(Clone, PartialEq, Debug)]
pub struct GroupPrefill {
    pub name: String,
    pub me: String,
    pub others: Vec<String>,
}

#[derive(Props, Clone, PartialEq)]
pub struct AddProjectModalProps {
    pub on_close: EventHandler<()>,
    #[props(default)]
    pub prefill: Option<GroupPrefill>,
}

#[component]
pub fn AddProjectModal(props: AddProjectModalProps) -> Element {
    crate::common::use_modal_focus("add-project-modal");
    let prefill = props.prefill.clone();
    let mut project_name = use_signal(|| prefill.as_ref().map(|p| p.name.clone()).unwrap_or_default());
    let mut description = use_signal(String::new);
    let mut emoji: Signal<Option<String>> = use_signal(|| None);
    let mut emoji_open = use_signal(|| false);
    // The project's currency is fixed at creation - every amount in it is stored converted into
    // this one, and there is no migration that could re-price an existing ledger, so
    // `EditProjectModal` shows it read-only.
    let mut currency = use_signal(|| "EUR".to_string());
    // You are never a row: the creator is always the first participant of the batch.
    let account_name = use_my_display_name();
    let mut me_name = use_signal(|| {
        prefill.as_ref().map(|p| p.me.clone()).filter(|me| !me.is_empty()).or(account_name.clone()).unwrap_or_default()
    });
    let mut me_touched = use_signal(|| false);
    let drafts: Signal<Vec<DraftParticipant>> = use_signal(|| {
        let others = prefill.as_ref().map(|p| p.others.clone()).unwrap_or_default();
        others.into_iter().map(|name| DraftParticipant { name, friend: None }).collect()
    });
    let mut error_msg: Signal<Option<String>> = use_signal(|| None);
    let mut loading = use_signal(|| false);
    // Set once the project row exists, so a retry after a failed participants step finishes this
    // project instead of creating a second one.
    let mut created: Signal<Option<(Uuid, [u8; 32])>> = use_signal(|| None);

    let friend_list = use_friend_list();
    let nav = use_navigator();
    let auth_ctx = use_context::<Signal<Option<AccountData>>>();
    let account_key_ctx = use_context::<Signal<Option<[u8; 32]>>>();
    let ls_ctx = use_context::<Signal<LocalStorageState>>();
    let is_online = use_context::<Signal<bool>>();
    let mut invite_retry = use_context::<Signal<Option<InviteRetry>>>();
    // The server makes the creator admin; the demo seeds without an account.
    let creates_admin = auth_ctx.read().as_ref().is_some_and(|a| a.key_lifecycle) && !crate::common::persist::is_demo();

    // `me()` can land after the modal opened: fill the name then, unless it was already typed.
    use_effect(use_reactive!(|account_name| {
        if !me_touched() && me_name.peek().is_empty() {
            if let Some(name) = account_name {
                me_name.set(name);
            }
        }
    }));

    let on_submit = move |e: FormEvent| {
        e.prevent_default();

        if !is_online() {
            error_msg.set(Some(tid!("add-project-offline")));
            return;
        }

        let name_val = project_name().trim().to_string();
        if name_val.is_empty() {
            error_msg.set(Some(tid!("add-project-name-required")));
            return;
        }
        let me_val = me_name().trim().to_string();
        let others = drafts();
        if !can_create(&me_val, &others) {
            return;
        }
        // Renaming yourself after adding someone can still produce a clash.
        if let Some(clash) = others.iter().find(|d| same_name(&d.name, &me_val)) {
            error_msg.set(Some(tid!("participants-duplicate", name: clash.name.clone())));
            return;
        }

        loading.set(true);
        error_msg.set(None);

        let desc_val = description().trim().to_string();
        let emoji_val = emoji();
        let currency_val = currency();
        let account = auth_ctx();
        let account_key = account_key_ctx();
        let resumed = created();
        let key = resumed.map(|(_, k)| k).unwrap_or_else(generate_key);

        spawn(async move {
            // Encrypt all project fields as a single JSON blob
            let payload = match encrypt_padded_json(&key, &new_project_payload(&name_val, &currency_val, &desc_val, emoji_val.as_deref())) {
                Ok(v) => v,
                Err(e) => { error_msg.set(Some(e)); loading.set(false); return; }
            };

            // Step A - create project
            // Born with its verifier, so no one holding only the UUID can ever seed one.
            let project_id = match resumed {
                Some((id, _)) => id,
                None => {
                    let claim_verifier = Some(claim_verifier(&key).to_vec());
                    let demo = crate::common::persist::is_demo();
                    let (creator_secret, creator_verifier) = new_creator_proof(account.is_none() && !demo);
                    let project =
                        match add_project(Json(CreatableProject { payload, claim_verifier, demo, creator_verifier })).await {
                            Ok(p) => p,
                            Err(e) => {
                                error_msg.set(Some(error_message(&e)));
                                loading.set(false);
                                return;
                            }
                        };
                    // The key is stored the moment the project exists: it used to wait for the
                    // participants, and a failure there left a project nobody could ever decrypt.
                    update_ls(ls_ctx, |state| {
                        upsert_project(state, project.id, None);
                        upsert_project_key(state, project.id, key_to_fragment(&key));
                        keep_creator_secret(state, project.id, creator_secret);
                    });
                    created.set(Some((project.id, key)));
                    project.id
                }
            };

            // Step B - create encrypted users, you first
            let names: Vec<String> =
                std::iter::once(me_val.clone()).chain(others.iter().map(|d| d.name.clone())).collect();
            let mut creatables: Vec<CreatableUser> = Vec::new();
            for user_name in &names {
                let payload = match encrypt_padded_json(&key, &UserPayload { name: user_name.clone(), removed: false }) {
                    Ok(v) => v,
                    Err(e) => { error_msg.set(Some(e)); loading.set(false); return; }
                };
                creatables.push(CreatableUser { payload, project_id });
            }

            crate::common::project_token::scope_key(Some(&key));
            let created_users = match add_user(Json(CreatableUserBatch::Multiple(creatables))).await
            {
                Ok(u) => u,
                Err(e) => {
                    error_msg.set(Some(error_message(&e)));
                    loading.set(false);
                    return;
                }
            };

            // Step C - the returned order is not guaranteed, so ids are matched back by name; the
            // names were refused unless unique.
            let id_of = |name: &str| {
                created_users
                    .iter()
                    .find(|u| {
                        decrypt_json::<UserPayload>(&key, &u.payload)
                            .map(|p| same_name(&p.name, name))
                            .unwrap_or(false)
                    })
                    .map(|u| u.id)
            };
            let user_id = id_of(&me_val);

            // Step D - persist which participant this device is
            update_ls(ls_ctx, |state| upsert_project(state, project_id, user_id));

            // Step E - if authenticated, also save to DB, escrowing the key we just generated so
            // the account's other devices can open this project without the share link.
            if let Some(account) = account.as_ref() {
                // No claim label: these participants were created by this very request, so nobody
                // can be holding one, and `ensure_membership` attaches the label on the project
                // page this navigates to a line below.
                let escrowed = upsert_account_project(Json(UpsertAccountProject {
                    project_id,
                    user_id,
                    key: account_key.and_then(|ak| wrap_key(&ak, WrapUse::Escrow(project_id), &key)),
                    claim_label: None,
                    claim_token: Some(claim_token(&key).to_vec()),
                    amk_epoch: crate::auth::account_key::held_amk_epoch(),
                }))
                .await;
                crate::auth::account_key::note(&escrowed);

                // Step F - invitations need the project and its participants to exist. The modal
                // closes whatever they return: what failed is offered again on the project page.
                let invitees: Vec<Invitee> = others
                    .iter()
                    .filter_map(|d| {
                        d.friend.clone().map(|friend| Invitee { friend, user_id: id_of(&d.name) })
                    })
                    .collect();
                if !invitees.is_empty() {
                    let from = account_key.and_then(|ak| sender(account, &ak, &ls_ctx.peek()));
                    let unsent = invite_each(ls_ctx, project_id, &invitees, from, &key).await;
                    if !unsent.is_empty() {
                        invite_retry.set(Some(InviteRetry { project_id, unsent }));
                    }
                }
            }

            // Step G - close modal and navigate
            props.on_close.call(());
            nav.push(Route::ExpensesPage { project_id });
        });
    };

    let on_close = props.on_close;
    let typed = (project_name(), description(), emoji(), currency(), me_touched(), drafts());
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
    let taken = taken_names(&me_name(), &[], &[]);
    let invites = invite_count(&drafts.read());
    let ready = can_create(&me_name(), &drafts.read());
    let (tile, tile_is_emoji) = tile_text(&project_name(), emoji().as_deref());
    let name_required = tid!("add-project-name-required");
    let name_error = error_msg().filter(|e| *e == name_required);

    rsx! {
        div {
            id: "add-project-modal",
            class: "modal modal-open modal-bottom sm:modal-middle",
            role: "dialog",
            "data-sheet-dirty": if dirty { "true" },
            aria_modal: "true",
            aria_labelledby: "add-project-modal-title",
            tabindex: "-1",
            div { class: "modal-box max-w-sm p-0 flex flex-col",
                div { class: "px-4 pt-3 pb-3 border-b border-base-200 flex-shrink-0",
                    FormSheetBar {
                        close_id: "add-project-close",
                        confirm_id: "add-project-confirm",
                        confirm_label: tid!("create"),
                        form: Some("add-project-fields"),
                        disabled: loading() || !ready,
                        on_close: move |_| request_close(),
                        h2 { id: "add-project-modal-title", class: "flex-1 min-w-0 truncate font-bold text-lg font-display sheet-bar-title", {tid!("add-project-title")} }
                    }
                }

                form { id: "add-project-fields", class: "flex flex-col flex-1 overflow-hidden", onsubmit: on_submit,
                div { class: "flex-1 overflow-y-auto px-6 py-4 flex flex-col gap-5",
                if let Some(err) = error_msg().filter(|e| *e != name_required) {
                    div { id: "add-project-error", role: "alert", class: "alert alert-error text-sm", "{err}" }
                }

                    div { class: "flex flex-col gap-3",
                        div { class: "flex items-end gap-3",
                            div { class: "flex flex-col items-center shrink-0",
                                span { class: "text-xs font-semibold text-base-content/70 block mb-1", {tid!("emoji-picker-label")} }
                                button {
                                    id: "add-project-emoji",
                                    r#type: "button",
                                    class: if emoji_open() { "size-11 flex items-center justify-center rounded-2xl ring-2 ring-primary" } else { "size-11 flex items-center justify-center rounded-2xl" },
                                    aria_label: format!("{}, {}", tid!("emoji-picker-label"), emoji().unwrap_or_else(|| tid!("emoji-none"))),
                                    aria_expanded: emoji_open(),
                                    aria_controls: "add-project-emoji-grid",
                                    onclick: move |_| {
                                        haptic(Haptic::Light);
                                        emoji_open.toggle();
                                    },
                                    span { class: "{tile_class(tile_is_emoji)} bg-base-200 text-base-content/70", aria_hidden: "true", "{tile}" }
                                }
                            }
                            div { class: "flex-1 min-w-0",
                                label { class: "text-xs font-semibold text-base-content/70 block mb-1.5", r#for: "add-project-name", {tid!("add-project-name-label")} }
                                input {
                                    id: "add-project-name",
                                    class: "input w-full",
                                    r#type: "text",
                                    enterkeyhint: "next",
                                    aria_required: "true",
                                    aria_invalid: if error_msg().is_some() && project_name().trim().is_empty() { "true" },
                                    aria_describedby: if name_error.is_some() { "add-project-name-error" } else if error_msg().is_some() { "add-project-error" } else { "" },
                                    placeholder: tid!("add-project-name-placeholder"),
                                    value: "{project_name}",
                                    oninput: move |e| project_name.set(e.value()),
                                }
                                if let Some(err) = name_error.clone() {
                                    p { id: "add-project-name-error", role: "alert", class: "text-xs text-error mt-1", "{err}" }
                                }
                            }
                        }
                        if emoji_open() {
                            EmojiGrid {
                                id: "add-project-emoji-grid",
                                selected: emoji(),
                                none_preview: Some(project_initial(&project_name())),
                                onpick: move |picked| {
                                    emoji.set(picked);
                                    emoji_open.set(false);
                                },
                            }
                        }
                        div { class: "flex gap-2",
                            div { class: "flex-1 min-w-0",
                                label { class: "text-xs font-semibold text-base-content/70 block mb-1.5", r#for: "add-project-description", {tid!("field-description")} }
                                input {
                                    id: "add-project-description",
                                    class: "input w-full",
                                    r#type: "text",
                                    enterkeyhint: "next",
                                    placeholder: tid!("field-optional"),
                                    value: "{description}",
                                    oninput: move |e| description.set(e.value()),
                                }
                            }
                            div { class: "relative",
                                label { class: "text-xs font-semibold text-base-content/70 block mb-1.5", r#for: "add-project-currency", {tid!("project-currency")} }
                                CurrencyPicker {
                                    id: "add-project-currency",
                                    value: currency(),
                                    label: tid!("project-currency"),
                                    button_class: "input w-auto gap-1 font-medium cursor-pointer",
                                    onchange: move |code| currency.set(code),
                                }
                            }
                        }
                    }

                    div { class: "flex flex-col gap-1.5",
                        label { class: "text-xs font-semibold text-base-content/70", r#for: "add-project-me", {tid!("participants-you-label")} }
                        label { class: "input w-full",
                            span { class: "w-7 h-7 rounded-full bg-secondary text-secondary-content flex items-center justify-center text-xs font-semibold flex-shrink-0", aria_hidden: "true",
                                {crate::participants::participants_service::initial(&me_name())}
                            }
                            input {
                                id: "add-project-me",
                                r#type: "text",
                                enterkeyhint: "next",
                                aria_required: "true",
                                autocapitalize: "words",
                                autocomplete: "off",
                                autocorrect: "off",
                                spellcheck: "false",
                                placeholder: tid!("add-project-participant-placeholder"),
                                value: "{me_name}",
                                oninput: move |e| {
                                    me_touched.set(true);
                                    me_name.set(e.value());
                                },
                            }
                            span { class: "badge badge-secondary badge-sm", {tid!("participants-you-badge")} }
                            if creates_admin {
                                span { id: "add-project-admin-badge", class: "badge badge-primary badge-sm", {tid!("participants-admin-badge")} }
                            }
                        }
                    }

                    div { class: "flex flex-col gap-2",
                        div { class: "flex items-center justify-between",
                            h4 { class: "text-xs font-bold uppercase tracking-wide text-base-content/70", {tid!("participants-others")} }
                            if !drafts.read().is_empty() {
                                span { class: "badge badge-ghost badge-sm", "{drafts.read().len()}" }
                            }
                        }
                        ParticipantsEditor {
                            drafts,
                            taken,
                            friends: friend_list.friends.clone(),
                            signed_in: friend_list.signed_in,
                            id_prefix: "add-project",
                            show_empty: true,
                        }
                    }

                } // end scrollable body
                    div { class: "flex justify-end gap-2 px-6 py-4 border-t border-base-200 flex-shrink-0",
                        button {
                            r#type: "button",
                            class: "btn",
                            onclick: move |_| request_close(),
                            {tid!("cancel")}
                        }
                        button {
                            id: "add-project-submit",
                            r#type: "submit",
                            class: "btn btn-primary",
                            disabled: loading() || !ready,
                            if loading() {
                                {tid!("creating")}
                            } else if invites > 0 {
                                {tid!("add-project-create-invite", count: invites)}
                            } else {
                                {tid!("create")}
                            }
                        }
                    }
                }
            }

            div {
                class: "modal-backdrop",
                onclick: move |_| request_close(),
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

fn new_project_payload(name: &str, currency: &str, description: &str, emoji: Option<&str>) -> ProjectPayload {
    let mut payload = ProjectPayload {
        name: name.to_string(),
        currency: currency.to_string(),
        description: if description.is_empty() { None } else { Some(description.to_string()) },
        status: Some(shared::ProjectStatus::Ongoing),
        extra: Default::default(),
    };
    payload.set_emoji(emoji);
    payload
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypto::{decrypt_json, encrypt_json, generate_key};
    use shared::{ProjectPayload, User, UserPayload};
    use uuid::Uuid;

    fn make_creatable_project(
        key: &[u8; 32],
        name: &str,
        description: Option<&str>,
        currency: &str,
    ) -> Result<CreatableProject, String> {
        Ok(CreatableProject {
            payload: encrypt_json(key, &ProjectPayload {
                name: name.to_string(),
                currency: currency.to_string(),
                description: description.map(|d| d.to_string()),
                status: None,
                extra: Default::default(),
            })?,
            claim_verifier: Some(claim_verifier(key).to_vec()),
            demo: false,
            creator_verifier: None,
        })
    }

    fn make_creatable_user(
        key: &[u8; 32],
        name: &str,
        project_id: Uuid,
    ) -> Result<CreatableUser, String> {
        Ok(CreatableUser {
            payload: encrypt_json(key, &UserPayload { name: name.to_string(), removed: false })?,
            project_id,
        })
    }

    fn test_key() -> [u8; 32] {
        [0x42u8; 32]
    }

    fn make_user(key: &[u8; 32], id: i32, name: &str) -> User {
        User {
            id,
            payload: encrypt_json(key, &UserPayload { name: name.to_string(), removed: false }).unwrap(),
            ..Default::default()
        }
    }

    #[test]
    fn a_new_project_is_born_with_the_picked_emoji_or_none() {
        let picked = new_project_payload("Ski", "EUR", "", Some("🎿"));
        assert_eq!(picked.emoji().as_deref(), Some("🎿"));
        assert_eq!(picked.description, None);
        assert_eq!(new_project_payload("Ski", "EUR", "Tignes", None).extra.get("emoji"), None);
    }

    #[test]
    fn project_fields_roundtrip() {
        let key = test_key();
        let cp = make_creatable_project(&key, "Vacances", Some("Paris 2025"), "EUR").unwrap();
        let pp: ProjectPayload = decrypt_json(&key, &cp.payload).unwrap();
        assert_eq!(pp.name, "Vacances");
        assert_eq!(pp.currency, "EUR");
        assert_eq!(pp.description.unwrap(), "Paris 2025");
    }

    #[test]
    fn project_no_description_yields_none() {
        let key = test_key();
        let cp = make_creatable_project(&key, "Trip", None, "USD").unwrap();
        let pp: ProjectPayload = decrypt_json(&key, &cp.payload).unwrap();
        assert!(pp.description.is_none());
        assert_eq!(pp.name, "Trip");
    }

    #[test]
    fn project_wrong_key_fails() {
        let key = test_key();
        let cp = make_creatable_project(&key, "Secret", None, "EUR").unwrap();
        let mut bad = key;
        bad[0] ^= 0xFF;
        assert!(decrypt_json::<ProjectPayload>(&bad, &cp.payload).is_err());
    }

    #[test]
    fn project_currency_roundtrip() {
        let key = test_key();
        let cp = make_creatable_project(&key, "x", None, "GBP").unwrap();
        let pp: ProjectPayload = decrypt_json(&key, &cp.payload).unwrap();
        assert_eq!(pp.currency, "GBP");
    }

    #[test]
    fn user_name_roundtrip() {
        let key = test_key();
        let pid = Uuid::nil();
        let cu = make_creatable_user(&key, "Alice", pid).unwrap();
        let up: UserPayload = decrypt_json(&key, &cu.payload).unwrap();
        assert_eq!(up.name, "Alice");
        assert_eq!(cu.project_id, pid);
    }

    #[test]
    fn user_wrong_key_fails() {
        let key = test_key();
        let cu = make_creatable_user(&key, "Alice", Uuid::nil()).unwrap();
        let mut bad = key;
        bad[1] ^= 0xAB;
        assert!(decrypt_json::<UserPayload>(&bad, &cu.payload).is_err());
    }

    #[test]
    fn me_lookup_finds_correct_user() {
        let key = test_key();
        let users = [make_user(&key, 1, "Alice"),
            make_user(&key, 2, "Bob"),
            make_user(&key, 3, "Charlie")];
        let found = users.iter().find(|u| {
            decrypt_json::<UserPayload>(&key, &u.payload)
                .map(|p| p.name.to_lowercase() == "bob")
                .unwrap_or(false)
        });
        assert_eq!(found.unwrap().id, 2);
    }

    #[test]
    fn me_lookup_case_insensitive() {
        let key = test_key();
        let users = [make_user(&key, 7, "Jean-Pierre")];
        let found = users.iter().find(|u| {
            decrypt_json::<UserPayload>(&key, &u.payload)
                .map(|p| p.name.to_lowercase() == "jean-pierre")
                .unwrap_or(false)
        });
        assert_eq!(found.unwrap().id, 7);
    }

    #[test]
    fn me_lookup_no_match_returns_none() {
        let key = test_key();
        let users = [make_user(&key, 1, "Alice")];
        let found = users.iter().find(|u| {
            decrypt_json::<UserPayload>(&key, &u.payload)
                .map(|p| p.name.to_lowercase() == "unknown")
                .unwrap_or(false)
        });
        assert!(found.is_none());
    }

    #[test]
    fn generate_key_produces_unique_keys() {
        let k1 = generate_key();
        let k2 = generate_key();
        assert_ne!(k1, k2);
    }
}
