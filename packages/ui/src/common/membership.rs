//! Who is still in a project, and what leaving one means.
//!
//! A project is deleted when its last member leaves, so every holder has to be countable
//! server-side. Accounts already are, through `account_projects`. A holder without an account -
//! a share link opened in a browser, a deep link, a project created while logged out - registers
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
    cached_project_row, clear_user_id, key_of, read_from_ls, remove_project, set_anon_member_id,
    update_ls, update_ls_if_changed, upsert_project, upsert_project_key, LocalStorageState,
};
use crate::crypto::{claim_label, claim_token, key_to_fragment, wrap_key, WrapUse};
use crate::decrypted::{decrypt_project, DecryptedUser};

/// Whether `candidate` may become this device's key for a project - see `docs/e2ee.md`, "Which key
/// gets escrowed". A held key that has decrypted the project here (`opened`, or the `cached` row) is
/// never replaced: the server serves the rows a candidate is checked against, so it could otherwise
/// swap in a key of its own through a forged invitation. A held key with no such proof is replaced
/// only by one that decrypts the cached row, or the `served` one when nothing is cached.
pub fn may_adopt(
    held: Option<[u8; 32]>,
    candidate: &[u8; 32],
    opened: bool,
    cached: Option<&ProjectDto>,
    served: Option<&ProjectDto>,
) -> bool {
    let decrypts = |key: &[u8; 32], p: &ProjectDto| decrypt_project(key, p).is_ok();
    match held {
        None => true,
        Some(held) if held == *candidate => true,
        Some(held) if opened || cached.is_some_and(|p| decrypts(&held, p)) => false,
        Some(_) => cached.or(served).is_some_and(|p| decrypts(candidate, p)),
    }
}

/// Stores `candidate` as the project's key, under [`may_adopt`]. Fetches the project only when a
/// different, unproven key is held and nothing is cached; offline, the held key stays. True when
/// `candidate` is now the key.
pub async fn adopt_project_key(
    ls_ctx: Signal<LocalStorageState>,
    project_id: Uuid,
    candidate: [u8; 32],
) -> bool {
    let state = read_from_ls();
    let entry = state.projects.iter().find(|p| p.project_id == project_id);
    let held = entry.and_then(key_of);
    let opened = entry.is_some_and(|e| e.key_opened);
    let cached = cached_project_row(&state, project_id);
    let served = match (held, cached) {
        (Some(held), None) if held != candidate && !opened => {
            crate::common::project_token::scope(project_id);
            get_project(project_id).await.ok()
        }
        _ => None,
    };
    if !may_adopt(held, &candidate, opened, cached, served.as_ref()) {
        // A new link after a rotation: the held key is replaced only by one whose row names it
        // (docs/plans/key-lifecycle.md §8.4).
        let Some(held) = held.filter(|h| *h != candidate) else { return false };
        crate::common::project_token::scope_key(Some(&candidate));
        let Ok(served) = get_project(project_id).await else { return false };
        let chain_seen = entry.map_or(0, |e| e.chain_seen);
        if !crate::rotation::may_switch(&held, &candidate, &served, chain_seen) {
            return false;
        }
        let opened = crate::rotation::chain_len(&candidate, &served);
        update_ls(ls_ctx, |state| crate::rotation_flow::switch_local(state, project_id, &held, &candidate, served.key_epoch, opened));
        return true;
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

/// One wording for both outcomes - the client cannot know whether it is the last member without
/// asking the server, and that round trip would buy nothing.
pub const LEAVE_CONFIRM_MESSAGE: &str = "leave-project-message";

/// Whether the server accepted the identity this device believes it is.
///
/// `IdentityTaken`: another account claims that participant. `ParticipantGone`: another member
/// removed the participant. In both the local `user_id` is already cleared, which is what makes
/// `ExpensesPage` reopen the picker.
///
/// `ProofRejected`: the server has a verifier for this project and this device's key does not
/// produce it. The local `user_id` is kept - the identity may be right and the key stale, and a
/// fresh pick would be refused for the same reason.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClaimOutcome {
    Ok,
    IdentityTaken,
    ParticipantGone,
    ProofRejected,
}

/// Records this device as a member of the project. Idempotent, and called from `ExpensesPage` on
/// every open - which is every path that could have just acquired the key, so the escrowed key,
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
        (Some(ak), Some(pk)) => (wrap_key(&ak, WrapUse::Escrow(project_id), &pk), claim_label(&account, &ak, &pk)),
        _ => (None, None),
    };
    let result = upsert_account_project(Json(UpsertAccountProject {
        project_id,
        user_id,
        key,
        claim_label: label,
        // Sent with or without a claim: it also seeds the verifier of a project that predates it.
        claim_token: project_key.map(|pk| claim_token(&pk).to_vec()),
        amk_epoch: crate::auth::account_key::held_amk_epoch(),
    }))
    .await;
    crate::auth::account_key::note(&result);

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
    crate::common::project_token::scope(project_id);
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

    crate::common::project_token::scope(project_id);
    match leave_project(Json(LeaveProject { project_id, member_id })).await {
        Ok(()) => {}
        // Already gone - someone else was the last one out. Nothing to leave, so treat it as a
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
            &shared::ProjectPayload { name: "Trip".into(), currency: "EUR".into(), description: None, status: None, extra: Default::default() },
        )
        .unwrap();
        ProjectDto {
            id: Uuid::nil(),
            payload,
            status: shared::ProjectStatus::Ongoing,
            created_at: Default::default(),
            has_admin: false,
            key_lifecycle: false,
            key_epoch: 0,
            has_creator_proof: false,
            read_only: false,
        }
    }

    const GOOD: [u8; 32] = [1; 32];
    const WRONG: [u8; 32] = [2; 32];

    const SERVERS: [u8; 32] = [3; 32];

    #[test]
    fn a_wrong_key_never_replaces_the_held_one() {
        assert!(!may_adopt(Some(GOOD), &WRONG, false, None, Some(&project_under(&GOOD))));
        assert!(!may_adopt(Some(GOOD), &WRONG, false, Some(&project_under(&GOOD)), None));
    }

    #[test]
    fn an_unverifiable_key_never_replaces_the_held_one() {
        assert!(!may_adopt(Some(GOOD), &WRONG, false, None, None));
    }

    #[test]
    fn a_key_that_decrypts_replaces_a_stale_one() {
        assert!(may_adopt(Some(WRONG), &GOOD, false, None, Some(&project_under(&GOOD))));
        assert!(may_adopt(Some(WRONG), &GOOD, false, Some(&project_under(&GOOD)), None));
    }

    #[test]
    fn a_first_key_or_the_same_key_needs_no_proof() {
        assert!(may_adopt(None, &WRONG, false, None, None));
        assert!(may_adopt(Some(GOOD), &GOOD, true, None, None));
    }

    /// The server boxes a key of its own for a project this device holds and serves a row under it:
    /// a key that opened the project here stays, whatever the server serves or got cached.
    #[test]
    fn a_server_key_never_replaces_one_that_opened_the_project() {
        let served = project_under(&SERVERS);
        assert!(!may_adopt(Some(GOOD), &SERVERS, true, None, Some(&served)));
        assert!(!may_adopt(Some(GOOD), &SERVERS, true, Some(&served), None));
        assert!(!may_adopt(Some(GOOD), &SERVERS, false, Some(&project_under(&GOOD)), Some(&served)));
    }

    /// A cached row is what a candidate answers to, not the one the server serves now.
    #[test]
    fn the_cached_row_outranks_the_served_one() {
        assert!(!may_adopt(Some(WRONG), &SERVERS, false, Some(&project_under(&GOOD)), Some(&project_under(&SERVERS))));
    }

    fn user(id: i32, claimed: bool) -> DecryptedUser {
        DecryptedUser {
            id,
            name: "Alice".into(),
            created_at: None,
            claimed,
            admin: false,
            claim_name: claimed.then(|| "Jonathan".to_string()),
            payment_methods: vec![],
            removed: false,
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
