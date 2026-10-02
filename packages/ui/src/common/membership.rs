//! Who is still in a project, and what leaving one means.
//!
//! A project is deleted when its last member leaves, so every holder has to be countable
//! server-side. Accounts already are, through `account_projects`. A holder without an account —
//! a share link opened in a browser, a deep link, a project created while logged out — registers
//! a random `member_id` instead, one per project so the server cannot group a person's projects.
//!
//! See docs/project-membership.md.

use api::account_projects::account_projects_controller::upsert_account_project;
use api::projects::projects_controller::{get_project, join_project, leave_project};
use dioxus::fullstack::Json;
use dioxus::prelude::*;
use shared::{Account, JoinProject, LeaveProject, ProjectDto, UpsertAccountProject};
use uuid::Uuid;

use super::error_utils::{
    error_message, is_claim_proof_error, is_identity_taken_error, is_participant_gone_error,
    is_project_gone_error,
};
use super::local_storage::{
    clear_user_id, key_of, project_key, read_from_ls, remove_project, set_anon_member_id,
    update_ls, update_ls_if_changed, upsert_project, upsert_project_key, LocalStorageState,
};
use crate::crypto::{claim_label, claim_token, key_to_fragment, wrap_key};
use crate::decrypted::{decrypt_project, DecryptedUser};

/// Whether `candidate` may become this device's key for a project. A key already held is replaced
/// only by one proven to decrypt the project, so a mistyped or hostile link cannot overwrite the
/// only working copy — see `docs/e2ee.md`, "Which key gets escrowed".
pub fn may_adopt(held: Option<[u8; 32]>, candidate: &[u8; 32], project: Option<&ProjectDto>) -> bool {
    match held {
        None => true,
        Some(held) if held == *candidate => true,
        Some(_) => project.is_some_and(|p| decrypt_project(candidate, p).is_ok()),
    }
}

/// Stores `candidate` as the project's key, under [`may_adopt`]. Fetches the project only when a
/// different key is already held; offline, the held key stays. True when `candidate` is now the key.
pub async fn adopt_project_key(
    ls_ctx: Signal<LocalStorageState>,
    project_id: Uuid,
    candidate: [u8; 32],
) -> bool {
    let held = project_key(project_id);
    let project = match held {
        Some(held) if held != candidate => get_project(project_id).await.ok(),
        _ => None,
    };
    if !may_adopt(held, &candidate, project.as_ref()) {
        return false;
    }
    update_ls_if_changed(ls_ctx, |state| {
        upsert_project(state, project_id, None);
        upsert_project_key(state, project_id, key_to_fragment(&candidate));
    });
    true
}

/// Translation keys, not sentences: both pages render them through `tid!`, and keeping them keys
/// is what lets this module stay free of an i18n context.
pub const LEAVE_CONFIRM_TITLE: &str = "leave-project-title";

/// One wording for both outcomes — the client cannot know whether it is the last member without
/// asking the server, and that round trip would buy nothing.
pub const LEAVE_CONFIRM_MESSAGE: &str = "leave-project-message";

/// Whether the server accepted the identity this device believes it is.
///
/// `IdentityTaken`: another account claims that participant. `ParticipantGone`: another member
/// removed the participant. In both the local `user_id` is already cleared, which is what makes
/// `ExpensesPage` reopen the picker.
///
/// `ProofRejected`: the server has a verifier for this project and this device's key does not
/// produce it. The local `user_id` is kept — the identity may be right and the key stale, and a
/// fresh pick would be refused for the same reason.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClaimOutcome {
    Ok,
    IdentityTaken,
    ParticipantGone,
    ProofRejected,
}

/// Records this device as a member of the project. Idempotent, and called from `ExpensesPage` on
/// every open — which is every path that could have just acquired the key, so the escrowed key,
/// the claim label and the claim token ride on this write rather than getting calls of their own.
pub async fn ensure_membership(
    ls_ctx: Signal<LocalStorageState>,
    project_id: Uuid,
    account: Option<Account>,
    account_key: Option<[u8; 32]>,
) -> ClaimOutcome {
    // Without an account the identity lives only in localStorage: nothing for the server to refuse.
    let Some(account) = account else {
        ensure_anonymous_membership(ls_ctx, project_id).await;
        return ClaimOutcome::Ok;
    };

    let state = read_from_ls();
    let entry = state.projects.iter().find(|p| p.project_id == project_id);
    let user_id = entry.and_then(|p| p.user_id);
    let project_key = entry.and_then(key_of);
    // `account_key` is None when this device holds a session but not the key (local store cleared,
    // or a session from before the key was persisted): the membership still goes up unkeyed.
    // No payment-methods copy: its only writer is `payment_methods::refill_project_copies`.
    let (key, label) = match (account_key, project_key) {
        (Some(ak), Some(pk)) => (wrap_key(&ak, &pk), claim_label(&account, &ak, &pk)),
        _ => (None, None),
    };
    let result = upsert_account_project(Json(UpsertAccountProject {
        project_id,
        user_id,
        key,
        claim_label: label,
        // Sent with or without a claim: it also seeds the verifier of a project that predates it.
        claim_token: project_key.map(|pk| claim_token(&pk).to_vec()),
    }))
    .await;

    match result {
        Err(e) if is_identity_taken_error(&e) => {
            update_ls(ls_ctx, |state| clear_user_id(state, project_id));
            ClaimOutcome::IdentityTaken
        }
        Err(e) if is_participant_gone_error(&e) => {
            update_ls(ls_ctx, |state| clear_user_id(state, project_id));
            ClaimOutcome::ParticipantGone
        }
        Err(e) if is_claim_proof_error(&e) => ClaimOutcome::ProofRejected,
        _ => ClaimOutcome::Ok,
    }
}

async fn ensure_anonymous_membership(ls_ctx: Signal<LocalStorageState>, project_id: Uuid) {
    if read_from_ls()
        .projects
        .iter()
        .any(|p| p.project_id == project_id && p.anon_member_id.is_some())
    {
        return;
    }
    // Persist only once the server has the id: the other order leaves a membership the server
    // never recorded, so leaving would delete nothing and the project could never be cleaned up.
    let member_id = Uuid::new_v4();
    if join_project(Json(JoinProject { project_id, member_id })).await.is_ok() {
        update_ls(ls_ctx, |state| set_anon_member_id(state, project_id, member_id));
    }
}

/// Whether this participant is spoken for by *someone else's* account: claimed, and not the
/// identity this device already holds, so a person's own claim stays selectable.
pub fn identity_locked(user: &DecryptedUser, my_user_id: Option<i32>) -> bool {
    user.claimed && Some(user.id) != my_user_id
}

/// Leaves the project and drops every local trace of it.
///
/// Local state is cleared only on success: forgetting the `member_id` while the server still
/// holds the row would leave a member nobody can remove, and the project could never be deleted.
pub async fn leave_project_and_forget(
    ls_ctx: Signal<LocalStorageState>,
    project_id: Uuid,
) -> Result<(), String> {
    let member_id = read_from_ls()
        .projects
        .iter()
        .find(|p| p.project_id == project_id)
        .and_then(|p| p.anon_member_id);

    match leave_project(Json(LeaveProject { project_id, member_id })).await {
        Ok(()) => {}
        // Already gone — someone else was the last one out. Nothing to leave, so treat it as a
        // success and let the local entry go, otherwise a dangling project would be unremovable.
        Err(e) if is_project_gone_error(&e) => {}
        Err(e) => return Err(error_message(&e)),
    }

    update_ls(ls_ctx, |state| remove_project(state, project_id));

    Ok(())
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod key_guard {
    /// Rendered with `tid!` on a value, so the crate-wide scanner in `i18n` does not see them.
    #[test]
    fn leave_confirm_keys_are_known_messages() {
        let known = crate::i18n::locale_ids(crate::i18n::FALLBACK);
        for key in [super::LEAVE_CONFIRM_TITLE, super::LEAVE_CONFIRM_MESSAGE] {
            assert!(known.contains(key), "{key} is not defined in the fallback locale");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn project_under(key: &[u8; 32]) -> ProjectDto {
        let payload = crate::crypto::encrypt_json(
            key,
            &shared::ProjectPayload { name: "Trip".into(), currency: "EUR".into(), description: None },
        )
        .unwrap();
        ProjectDto {
            id: Uuid::nil(),
            payload,
            status: shared::ProjectStatus::Ongoing,
            created_at: Default::default(),
            owner_account_id: None,
            read_only: false,
        }
    }

    const GOOD: [u8; 32] = [1; 32];
    const WRONG: [u8; 32] = [2; 32];

    #[test]
    fn a_wrong_key_never_replaces_the_held_one() {
        assert!(!may_adopt(Some(GOOD), &WRONG, Some(&project_under(&GOOD))));
    }

    #[test]
    fn an_unverifiable_key_never_replaces_the_held_one() {
        assert!(!may_adopt(Some(GOOD), &WRONG, None));
    }

    #[test]
    fn a_key_that_decrypts_replaces_a_stale_one() {
        assert!(may_adopt(Some(WRONG), &GOOD, Some(&project_under(&GOOD))));
    }

    #[test]
    fn a_first_key_or_the_same_key_needs_no_proof() {
        assert!(may_adopt(None, &WRONG, None));
        assert!(may_adopt(Some(GOOD), &GOOD, None));
    }

    fn user(id: i32, claimed: bool) -> DecryptedUser {
        DecryptedUser {
            id,
            name: "Alice".into(),
            created_at: None,
            claimed,
            claim_name: claimed.then(|| "Jonathan".to_string()),
            payment_methods: vec![],
        }
    }

    #[test]
    fn an_unclaimed_participant_is_never_locked() {
        assert!(!identity_locked(&user(1, false), None));
        assert!(!identity_locked(&user(1, false), Some(2)));
    }

    #[test]
    fn a_participant_claimed_by_someone_else_is_locked() {
        assert!(identity_locked(&user(1, true), None));
        assert!(identity_locked(&user(1, true), Some(2)));
    }

    /// The case that matters most: locking someone out of their own identity would make the picker
    /// unusable for exactly the people who claimed correctly.
    #[test]
    fn my_own_claim_stays_selectable() {
        assert!(!identity_locked(&user(1, true), Some(1)));
    }
}
