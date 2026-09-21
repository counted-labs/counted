//! Who is still in a project, and what leaving one means.
//!
//! A project is deleted when its last member leaves, so every holder has to be countable
//! server-side. Accounts already are, through `account_projects`. A holder without an account —
//! a share link opened in a browser, a deep link, a project created while logged out — registers
//! a random `member_id` instead, one per project so the server cannot group a person's projects.
//!
//! See docs/project-membership.md.

use api::account_projects::account_projects_controller::upsert_account_project;
use api::projects::projects_controller::{join_project, leave_project};
use dioxus::fullstack::Json;
use dioxus::prelude::*;
use shared::{Account, JoinProject, LeaveProject, UpsertAccountProject};
use uuid::Uuid;

use super::error_utils::{
    error_message, is_claim_proof_error, is_identity_taken_error, is_participant_gone_error,
    is_project_gone_error,
};
use super::local_storage::{
    clear_user_id, key_of, read_from_ls, remove_project, set_anon_member_id, update_ls,
    LocalStorageState,
};
use crate::crypto::{claim_label, claim_token, wrap_project_key, DecryptedUser};

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

/// Translation keys, not sentences: both pages render them through `tid!`, and keeping them keys
/// is what lets this module stay free of an i18n context.
pub const LEAVE_CONFIRM_TITLE: &str = "leave-project-title";

/// One wording for both outcomes — the client cannot know whether it is the last member without
/// asking the server, and that round trip would buy nothing.
pub const LEAVE_CONFIRM_MESSAGE: &str = "leave-project-message";

/// Whether the server accepted the identity this device believes it is.
///
/// `IdentityTaken` means another account claims that participant; `ParticipantGone` means the
/// participant was removed from the project by another member. In both the local `user_id` has
/// already been cleared by the time it is returned, which is what makes `ExpensesPage` reopen the
/// picker — the caller only has to say why. Left in place, a gone participant was a dead end:
/// the picker never reopened and every write was refused with a generic error.
///
/// `ProofRejected` means the server has a verifier for this project and this device's key does
/// not produce it. The local `user_id` is kept: the identity may well be right and the key stale,
/// and a fresh pick would be refused for the same reason.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClaimOutcome {
    Ok,
    IdentityTaken,
    ParticipantGone,
    ProofRejected,
}

/// Records this device as a member of the project. Idempotent, and safe to call on every visit.
///
/// Called from `ExpensesPage`, which every entry path lands on — create, join modal, deep link,
/// raw share link, Tricount import — rather than from each of them.
pub async fn ensure_membership(
    ls_ctx: Signal<LocalStorageState>,
    project_id: Uuid,
    account: Option<Account>,
    account_key: Option<[u8; 32]>,
) -> ClaimOutcome {
    if let Some(account) = account {
        let state = read_from_ls();
        let entry = state.projects.iter().find(|p| p.project_id == project_id);
        let user_id = entry.and_then(|p| p.user_id);
        let project_key = entry.and_then(key_of);
        // Escrow rides on the membership write rather than getting its own call: this runs on every
        // project open, which is also every path that could have just acquired the key.
        // `account_key` is None for a session restored from the cookie — the membership still goes
        // up, and the key is escrowed at the next sign-in. See docs/e2ee.md, "Key escrow".
        let key = match (account_key, project_key) {
            (Some(ak), Some(pk)) => wrap_project_key(&ak, &pk),
            _ => None,
        };
        // The claim label rides along on the same terms and for the same reason: this is the one
        // call that happens on every open, so it is the one place that catches a device which has
        // only just acquired the key. `COALESCE` server-side makes re-sending it free.
        let label = match (account_key, project_key) {
            (Some(ak), Some(pk)) => claim_label(&account, &ak, &pk),
            _ => None,
        };
        // No payment-methods copy here: unlike the label they change, and a copy derived from the
        // `Account` this device booted with could put back what a newer save on another device
        // withdrew. Copies are written only by `PUT /auth/payment-methods` — see
        // `payment_methods::refill_project_copies`.
        //
        // Sent on every write, not only with a claim: it is also what seeds the verifier on a
        // project that predates it, and this is the one call every key-holding open goes through.
        let token = project_key.map(|pk| claim_token(&pk).to_vec());
        let result = upsert_account_project(Json(UpsertAccountProject {
            project_id,
            user_id,
            key,
            claim_label: label,
            claim_token: token,
        }))
        .await;

        // Someone else holds this participant. Dropping the local `user_id` is what reopens the
        // picker; keeping it would leave this device showing an identity the account does not have,
        // invisibly, and no other device of the same account would ever learn one.
        if result.as_ref().err().is_some_and(is_identity_taken_error) {
            update_ls(ls_ctx, |state| clear_user_id(state, project_id));
            return ClaimOutcome::IdentityTaken;
        }
        if result.as_ref().err().is_some_and(is_participant_gone_error) {
            update_ls(ls_ctx, |state| clear_user_id(state, project_id));
            return ClaimOutcome::ParticipantGone;
        }
        if result.as_ref().err().is_some_and(is_claim_proof_error) {
            return ClaimOutcome::ProofRejected;
        }
        return ClaimOutcome::Ok;
    }

    if read_from_ls()
        .projects
        .iter()
        .any(|p| p.project_id == project_id && p.anon_member_id.is_some())
    {
        return ClaimOutcome::Ok;
    }

    // Persist only once the server has the id. The other order leaves this device holding a
    // membership the server never recorded, so leaving would delete nothing and the project
    // could never be cleaned up.
    let member_id = Uuid::new_v4();
    if join_project(Json(JoinProject { project_id, member_id })).await.is_ok() {
        update_ls(ls_ctx, |state| set_anon_member_id(state, project_id, member_id));
    }

    // A device with no account claims nothing: its identity lives only in localStorage, so there is
    // nothing for the server to refuse. The picker still greys out taken identities — see
    // [`identity_locked`] — but that is a UI courtesy, not a boundary.
    ClaimOutcome::Ok
}

/// Whether this participant is spoken for by *someone else's* account.
///
/// The whole rule, in one place and testable without a Dioxus runtime: claimed, and not the
/// identity this device already holds. Comparing against the local `user_id` is what keeps a
/// person's own claim selectable rather than locking them out of themselves.
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
