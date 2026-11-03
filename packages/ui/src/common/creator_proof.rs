//! Proof of creator (docs/plans/key-lifecycle.md §8.7, D9). A project created without an account
//! gets a secret that stays on the creating device, never in a link or a synced payload; the server
//! keeps only its `SHA-256`. Signing in on that device makes the creator admin while the project has
//! none.

use base64::{engine::general_purpose::STANDARD, Engine};
use dioxus::fullstack::{Json, ServerFnError};
use dioxus::prelude::*;
use sha2::{Digest, Sha256};
use shared::{AccountProject, ClaimCreator};
use uuid::Uuid;

use super::local_storage::{cached_project_row, read_from_ls, update_ls, LocalStorageState};

/// A fresh creator secret, as stored on the device, and the verifier `CreatableProject` carries.
pub fn new_creator_secret() -> (String, String) {
    let secret = crate::crypto::generate_key();
    (STANDARD.encode(secret), creator_verifier(&secret))
}

/// The secret to keep and the verifier to send, for a creator without an account only.
pub fn new_creator_proof(anonymous: bool) -> (Option<String>, Option<String>) {
    if !anonymous {
        return (None, None);
    }
    let (secret, verifier) = new_creator_secret();
    (Some(secret), Some(verifier))
}

pub fn creator_verifier(secret: &[u8]) -> String {
    STANDARD.encode(Sha256::digest(secret))
}

/// Kept on the projects this device created signed out, the demo excepted.
pub fn keep_creator_secret(state: &mut LocalStorageState, project_id: Uuid, secret: Option<String>) {
    if let Some(entry) = state.projects.iter_mut().find(|p| p.project_id == project_id) {
        entry.creator_secret = secret;
    }
}

/// Whether this device holds the creator secret of `project_id`.
pub fn holds_creator_secret(state: &LocalStorageState, project_id: Uuid) -> bool {
    state.projects.iter().any(|p| p.project_id == project_id && p.creator_secret.is_some())
}

/// The secrets worth sending: the account's membership already holds the key (the server promotes
/// only such a row), the account is not admin there, and the project, as last seen, still has its
/// proof and no admin.
pub fn claimable(state: &LocalStorageState, server: &[AccountProject]) -> Vec<(Uuid, String)> {
    state
        .projects
        .iter()
        .filter_map(|local| {
            let secret = local.creator_secret.clone()?;
            let row = server.iter().find(|r| r.project_id == local.project_id)?;
            let member = row.key.is_some() || row.user_id.is_some();
            let open = cached_project_row(state, local.project_id).is_none_or(|p| p.has_creator_proof && !p.has_admin);
            (member && !row.admin && open).then_some((local.project_id, secret))
        })
        .collect()
}

/// The server's final word on a secret: it is used up, wrong, or the project found an admin.
fn settled(e: &ServerFnError) -> bool {
    matches!(
        e,
        ServerFnError::ServerError { message, code, .. }
            if (*code == 403 && message == shared::errors::CREATOR_PROOF_INVALID)
                || (*code == 409 && message == shared::errors::PROJECT_HAS_ADMIN)
    )
}

/// Run after the sign-in reconciliation has written this device's memberships. A secret stays
/// until the server settles it, so a failed request is retried on the next sync.
pub async fn claim_created_projects(ls_ctx: Signal<LocalStorageState>) {
    if !read_from_ls().projects.iter().any(|p| p.creator_secret.is_some()) {
        return;
    }
    let Ok(server) = api::account_projects::account_projects_controller::get_account_projects().await else { return };
    for (project_id, secret) in claimable(&read_from_ls(), &server) {
        super::project_token::scope(project_id);
        let result = api::projects::projects_controller::claim_creator(Json(ClaimCreator { project_id, secret })).await;
        let claimed = result.is_ok();
        if claimed || result.as_ref().is_err_and(settled) {
            update_ls(ls_ctx, |state| {
                if let Some(entry) = state.projects.iter_mut().find(|p| p.project_id == project_id) {
                    entry.creator_secret = None;
                    entry.admin |= claimed;
                }
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::LocalStorageProject;
    use shared::{EncryptedPair, ProjectDto, ProjectStatus};

    fn row(project_id: Uuid, user_id: Option<i32>, admin: bool) -> AccountProject {
        AccountProject { project_id, user_id, key: None, claim_verifier_seeded: true, payment_methods_shared: true, history_has_actors: None, admin }
    }

    fn project(id: Uuid, has_admin: bool, has_creator_proof: bool) -> ProjectDto {
        ProjectDto {
            id,
            payload: EncryptedPair { ct: String::new(), iv: String::new() },
            status: ProjectStatus::Ongoing,
            created_at: Default::default(),
            has_admin,
            key_lifecycle: true,
            key_epoch: 0,
            has_creator_proof,
            read_only: false,
        }
    }

    fn holding(id: Uuid, secret: Option<&str>, cached: Option<ProjectDto>) -> LocalStorageProject {
        LocalStorageProject {
            project_id: id,
            creator_secret: secret.map(str::to_string),
            cached_project: cached,
            ..Default::default()
        }
    }

    #[test]
    fn the_verifier_is_the_base64_sha256_of_the_secret() {
        let (secret, verifier) = new_creator_secret();
        let bytes = STANDARD.decode(&secret).unwrap();
        assert_eq!(bytes.len(), shared::CREATOR_SECRET_LENGTH);
        assert_eq!(STANDARD.decode(&verifier).unwrap(), Sha256::digest(&bytes).to_vec());
        assert_eq!(creator_verifier(&[0u8; 32]), "Zmh6rfhivXdsj8GLjp+OIAiXFIVu4jOzkCpZHQ1fKSU=");
        assert_ne!(new_creator_secret().0, secret, "each project gets its own secret");
        assert_eq!(new_creator_proof(false), (None, None), "a signed-in creator is admin already");
        assert!(matches!(new_creator_proof(true), (Some(_), Some(_))));
    }

    #[test]
    fn only_a_held_secret_on_a_keyed_membership_without_an_admin_is_claimed() {
        let [open, no_member, keyless, admin_already, has_admin, no_proof, no_secret, uncached] =
            std::array::from_fn(|_| Uuid::new_v4());
        let state = LocalStorageState {
            projects: vec![
                holding(open, Some("s1"), Some(project(open, false, true))),
                holding(no_member, Some("s2"), None),
                holding(keyless, Some("s3"), None),
                holding(admin_already, Some("s4"), None),
                holding(has_admin, Some("s5"), Some(project(has_admin, true, true))),
                holding(no_proof, Some("s6"), Some(project(no_proof, false, false))),
                holding(no_secret, None, Some(project(no_secret, false, true))),
                holding(uncached, Some("s8"), None),
            ],
            ..Default::default()
        };
        let server = vec![
            row(open, Some(1), false),
            row(keyless, None, false),
            row(admin_already, Some(1), true),
            row(has_admin, Some(1), false),
            row(no_proof, Some(1), false),
            row(no_secret, Some(1), false),
            row(uncached, Some(1), false),
        ];
        assert_eq!(claimable(&state, &server), vec![(open, "s1".to_string()), (uncached, "s8".to_string())]);
    }

    #[test]
    fn a_secret_is_dropped_only_on_the_servers_final_word() {
        let with = |code: u16, message: &str| ServerFnError::ServerError { message: message.into(), code, details: None };
        assert!(settled(&with(403, shared::errors::CREATOR_PROOF_INVALID)));
        assert!(settled(&with(409, shared::errors::PROJECT_HAS_ADMIN)));
        assert!(!settled(&with(500, shared::errors::CREATOR_PROOF_INVALID)));
        assert!(!settled(&with(409, shared::errors::CREATOR_PROOF_INVALID)));
        assert!(!settled(&with(502, "error sending request")));
    }
}
