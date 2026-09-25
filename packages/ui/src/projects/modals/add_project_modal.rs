use api::account_projects::account_projects_controller::upsert_account_project;
use api::projects::projects_controller::add_project;
use api::users::users_controller::add_user;
use dioxus::fullstack::Json;
use dioxus::prelude::*;
use crate::tid;
use shared::{
    Account as AccountData, CreatableProject, CreatableUser, CreatableUserBatch,
    ProjectPayload, UpsertAccountProject, UserPayload,
};

use crate::common::{error_message, update_ls, upsert_project, upsert_project_key, LocalStorageState};
use crate::crypto::{
    claim_token, claim_verifier, decrypt_json, encrypt_json, generate_key, key_to_fragment,
    wrap_project_key,
};
use crate::icons::{TrashIcon, UserIcon, ICON_INLINE};
use crate::route::Route;

#[derive(Props, Clone, PartialEq)]
pub struct AddProjectModalProps {
    pub on_close: EventHandler<()>,
}

#[component]
pub fn AddProjectModal(props: AddProjectModalProps) -> Element {
    let mut project_name = use_signal(String::new);
    let mut description = use_signal(String::new);
    // The project's currency is fixed at creation — every amount in it is stored converted into
    // this one, and there is no migration that could re-price an existing ledger, so
    // `EditProjectModal` shows it read-only.
    let mut currency = use_signal(|| "EUR".to_string());
    let mut user_input = use_signal(String::new);
    let mut users: Signal<Vec<String>> = use_signal(Vec::new);
    let mut selected_name: Signal<Option<String>> = use_signal(|| None);
    let mut error_msg: Signal<Option<String>> = use_signal(|| None);
    let mut loading = use_signal(|| false);

    let nav = use_navigator();
    let auth_ctx = use_context::<Signal<Option<AccountData>>>();
    let account_key_ctx = use_context::<Signal<Option<[u8; 32]>>>();
    let ls_ctx = use_context::<Signal<LocalStorageState>>();
    let is_online = use_context::<Signal<bool>>();

    let on_submit = move |e: FormEvent| {
        e.prevent_default();

        if !is_online() {
            error_msg.set(Some(
                tid!("add-project-offline"),
            ));
            return;
        }

        let name_val = project_name().trim().to_string();
        if name_val.is_empty() {
            error_msg.set(Some(tid!("add-project-name-required")));
            return;
        }
        let user_list = users();
        if user_list.len() < 2 {
            error_msg.set(Some(tid!("add-project-need-two-participants")));
            return;
        }
        if selected_name().is_none() {
            error_msg.set(Some(tid!("add-project-pick-yourself")));
            return;
        }

        loading.set(true);
        error_msg.set(None);

        let desc_val = description().trim().to_string();
        let currency_val = currency();
        let is_auth = auth_ctx().is_some();
        let account_key = account_key_ctx();
        let sel_name = selected_name().unwrap_or_default();
        let key = generate_key();
        let key_fragment = key_to_fragment(&key);

        spawn(async move {
            // Encrypt all project fields as a single JSON blob
            let payload = match encrypt_json(&key, &ProjectPayload {
                name: name_val.clone(),
                currency: currency_val.clone(),
                description: if desc_val.is_empty() { None } else { Some(desc_val.clone()) },
            }) {
                Ok(v) => v,
                Err(e) => { error_msg.set(Some(e)); loading.set(false); return; }
            };

            // Step A — create project
            // Born with its verifier, so no one holding only the UUID can ever seed one.
            let claim_verifier = Some(claim_verifier(&key).to_vec());
            let project = match add_project(Json(CreatableProject { payload, claim_verifier })).await {
                Ok(p) => p,
                Err(e) => {
                    error_msg.set(Some(error_message(&e)));
                    loading.set(false);
                    return;
                }
            };

            // Step B — create encrypted users
            let mut creatables: Vec<CreatableUser> = Vec::new();
            for user_name in &user_list {
                let payload = match encrypt_json(&key, &UserPayload { name: user_name.clone() }) {
                    Ok(v) => v,
                    Err(e) => { error_msg.set(Some(e)); loading.set(false); return; }
                };
                let invited_email = if is_auth && user_name.contains('@') {
                    Some(user_name.clone())
                } else {
                    None
                };
                creatables.push(CreatableUser { payload, project_id: project.id, invited_email });
            }

            let created_users = match add_user(Json(CreatableUserBatch::Multiple(creatables))).await
            {
                Ok(u) => u,
                Err(e) => {
                    error_msg.set(Some(error_message(&e)));
                    loading.set(false);
                    return;
                }
            };

            // Step C — resolve which user the person selected (decrypt returned names to compare)
            let user_id = created_users
                .iter()
                .find(|u| {
                    decrypt_json::<UserPayload>(&key, &u.payload)
                        .map(|p| p.name.to_lowercase() == sel_name.to_lowercase())
                        .unwrap_or(false)
                })
                .map(|u| u.id);

            // Step D — persist to localStorage (project entry + encryption key)
            update_ls(ls_ctx, |state| {
                upsert_project(state, project.id, user_id);
                upsert_project_key(state, project.id, key_fragment);
            });

            // Step E — if authenticated, also save to DB, escrowing the key we just generated so
            // the account's other devices can open this project without the share link.
            if is_auth {
                // No claim label: these participants were created by this very request, so nobody
                // can be holding one, and `ensure_membership` attaches the label on the project
                // page this navigates to a line below.
                let _ = upsert_account_project(Json(UpsertAccountProject {
                    project_id: project.id,
                    user_id,
                    key: account_key.and_then(|ak| wrap_project_key(&ak, &key)),
                    claim_label: None,
                    claim_token: Some(claim_token(&key).to_vec()),
                }))
                .await;
            }

            // Step F — close modal and navigate
            props.on_close.call(());
            nav.push(Route::ExpensesPage { project_id: project.id });
        });
    };

    let on_close = props.on_close;

    rsx! {
        div { class: "modal modal-open modal-bottom sm:modal-middle", role: "dialog",
            div { class: "modal-box max-w-sm p-0 flex flex-col",
                div { class: "flex items-center justify-between px-6 pt-5 pb-4 border-b border-base-200 flex-shrink-0",
                    h3 { class: "font-bold text-lg font-display", {tid!("add-project-title")} }
                    button {
                        r#type: "button",
                        class: "btn btn-ghost btn-circle h-11 w-11 min-h-11 text-lg",
                        aria_label: tid!("close"),
                        onclick: move |_| on_close.call(()),
                        "✕"
                    }
                }

                form { class: "flex flex-col flex-1 overflow-hidden", onsubmit: on_submit,
                div { class: "flex-1 overflow-y-auto px-6 py-4 flex flex-col gap-4",
                if let Some(err) = error_msg() {
                    div { role: "alert", class: "alert alert-error text-sm", "{err}" }
                }

                    fieldset { class: "fieldset",
                        label { class: "fieldset-legend", r#for: "add-project-name", {tid!("add-project-name-label")} }
                        input {
                            id: "add-project-name",
                            class: "input w-full",
                            r#type: "text",
                            enterkeyhint: "next",
                            aria_required: "true",
                            placeholder: tid!("add-project-name-placeholder"),
                            value: "{project_name}",
                            oninput: move |e| project_name.set(e.value()),
                        }

                        label { class: "fieldset-legend", r#for: "add-project-description", {tid!("field-description")} }
                        input {
                            id: "add-project-description",
                            class: "input w-full",
                            r#type: "text",
                            enterkeyhint: "next",
                            placeholder: tid!("field-optional"),
                            value: "{description}",
                            oninput: move |e| description.set(e.value()),
                        }

                        label { class: "fieldset-legend", r#for: "add-project-currency", {tid!("project-currency")} }
                        select {
                            id: "add-project-currency",
                            class: "select w-full",
                            value: "{currency}",
                            onchange: move |e| currency.set(e.value()),
                            for c in shared::CURRENCIES {
                                option { value: c.code, "{c.code} — {c.name}" }
                            }
                        }
                        p { class: "label text-xs whitespace-normal", {tid!("project-currency-hint")} }
                    }

                    fieldset { class: "fieldset bg-base-200 border-base-300 rounded-box border p-4",
                        legend { class: "fieldset-legend", {tid!("add-project-participants")} }

                        div { class: "flex gap-2",
                            label { class: "input flex-1",
                                UserIcon {}
                                input {
                                    r#type: "text",
                                    aria_label: tid!("add-project-participant-name"),
                                    id: "add-project-user",
                                    placeholder: tid!("add-project-participant-placeholder"),
                                    enterkeyhint: "done",
                                    autocapitalize: "words",
                                    autocomplete: "off",
                                    value: "{user_input}",
                                    oninput: move |e| user_input.set(e.value()),
                                    // Enter adds the participant instead of submitting the form.
                                    // Nothing blurs, so the keyboard stays up and the view does not
                                    // reflow between names.
                                    onkeydown: move |e| {
                                        if e.key() == Key::Enter {
                                            e.prevent_default();
                                            let name = user_input().trim().to_string();
                                            if !name.is_empty() {
                                                users.write().push(name);
                                                user_input.set(String::new());
                                            }
                                        }
                                    },
                                }
                            }
                            button {
                                id: "add-project-user-add",
                                r#type: "button",
                                class: "btn btn-neutral",
                                onclick: move |_| {
                                    let name = user_input().trim().to_string();
                                    if !name.is_empty() {
                                        users.write().push(name);
                                        user_input.set(String::new());
                                    }
                                },
                                {tid!("add")}
                            }
                        }

                        ul { class: "flex flex-col gap-1 mt-2 max-h-48 overflow-y-auto",
                            for i in 0..users().len() {
                                {
                                    let name = users().get(i).cloned().unwrap_or_default();
                                    let is_me = selected_name() == Some(name.clone());
                                    rsx! {
                                    li { class: "flex items-center gap-2",
                                        button {
                                            r#type: "button",
                                            class: "btn btn-square btn-sm btn-soft",
                                            aria_label: tid!("add-project-remove-participant"),
                                            onclick: move |_| {
                                                let removed =
                                                    users().get(i).cloned().unwrap_or_default();
                                                users.write().remove(i);
                                                if selected_name() == Some(removed) {
                                                    selected_name.set(None);
                                                }
                                            },
                                            TrashIcon { size: ICON_INLINE }
                                        }
                                        span { class: "flex-1 text-sm", "{name}" }
                                        if is_me {
                                            div { class: "badge badge-soft badge-accent", {tid!("add-project-me-badge")} }
                                        } else {
                                            button {
                                                id: "add-project-user-me-{i}",
                                                r#type: "button",
                                                class: "btn btn-outline btn-xs",
                                                onclick: move |_| selected_name.set(Some(name.clone())),
                                                {tid!("add-project-thats-me")}
                                            }
                                        }
                                    }
                                }
                                }
                            }
                        }
                    }

                } // end scrollable body
                    div { class: "flex justify-end gap-2 px-6 py-4 border-t border-base-200 flex-shrink-0",
                        button {
                            r#type: "button",
                            class: "btn",
                            onclick: move |_| props.on_close.call(()),
                            {tid!("cancel")}
                        }
                        button {
                            id: "add-project-submit",
                            r#type: "submit",
                            class: "btn btn-primary",
                            disabled: loading(),
                            if loading() {
                                {tid!("creating")}
                            } else {
                                {tid!("create")}
                            }
                        }
                    }
                }
            }

            div {
                class: "modal-backdrop",
                onclick: move |_| props.on_close.call(()),
            }
        }
    }
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
            })?,
            claim_verifier: Some(claim_verifier(key).to_vec()),
        })
    }

    fn make_creatable_user(
        key: &[u8; 32],
        name: &str,
        project_id: Uuid,
        invited_email: Option<String>,
    ) -> Result<CreatableUser, String> {
        Ok(CreatableUser {
            payload: encrypt_json(key, &UserPayload { name: name.to_string() })?,
            project_id,
            invited_email,
        })
    }

    fn test_key() -> [u8; 32] {
        [0x42u8; 32]
    }

    fn make_user(key: &[u8; 32], id: i32, name: &str) -> User {
        User {
            id,
            payload: encrypt_json(key, &UserPayload { name: name.to_string() }).unwrap(),
            ..Default::default()
        }
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
        let cu = make_creatable_user(&key, "Alice", pid, None).unwrap();
        let up: UserPayload = decrypt_json(&key, &cu.payload).unwrap();
        assert_eq!(up.name, "Alice");
        assert_eq!(cu.project_id, pid);
        assert!(cu.invited_email.is_none());
    }

    #[test]
    fn user_with_email_preserved() {
        let key = test_key();
        let cu = make_creatable_user(&key, "Bob", Uuid::nil(), Some("bob@ex.com".into())).unwrap();
        assert_eq!(cu.invited_email, Some("bob@ex.com".into()));
        let up: UserPayload = decrypt_json(&key, &cu.payload).unwrap();
        assert_eq!(up.name, "Bob");
    }

    #[test]
    fn user_wrong_key_fails() {
        let key = test_key();
        let cu = make_creatable_user(&key, "Alice", Uuid::nil(), None).unwrap();
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
