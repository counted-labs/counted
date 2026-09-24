//! Reconciling this device's project list with the account's, in both directions.
//!
//! Signing in used to be a one-way pull: `get_account_projects` came down and was upserted locally,
//! but nothing went up. A device holding projects from before it signed in kept them to itself —
//! they reached `account_projects` only when each was individually *opened*, via
//! `ensure_membership`. So a phone that created A, B, C while logged out and then signed in to an
//! account holding C, D, E ended up with all five locally and the account still holding three.
//!
//! The push half is what [`shared::UpsertAccountProject`] batching was always for; see
//! `docs/project-membership.md` §4, which named this gap and the unused endpoint together.
//!
//! Keys ride along. A project key is wrapped under the account key (`crypto::wrap_project_key`) so
//! the server can store it without reading it, and unwrapped on any other device the account signs
//! in to — the only thing that stops a second device from showing a list of projects it cannot
//! decrypt. See `docs/e2ee.md`, "Key escrow".
//!
//! The push is not a blind union. Leaving a project on one device deletes the account's row, and
//! a second device still holding that project locally would otherwise push it straight back on its
//! next load — the leave undone, silently, and re-pulled onto the device that left. So every entry
//! remembers which account it was last seen synced with ([`LocalStorageProject::synced_account`]),
//! and an entry marked for *this* account that the server no longer lists is forgotten, not pushed
//! ([`left_elsewhere`]). An entry with no mark, or marked for another account, is what the push was
//! always for.

use shared::{Account, AccountProject, UpsertAccountProject};
use std::collections::HashMap;
use uuid::Uuid;

use super::local_storage::{key_of, mark_synced, LocalStorageState};
use crate::crypto::{claim_label, claim_token, unwrap_project_key, wrap_project_key};

/// Projects this device holds that `account_id` was synced with and has since left elsewhere: the
/// server no longer lists them for this account. The caller forgets them locally **before**
/// [`to_push`] runs, so they are never sent back up.
///
/// Only a `leave` (or the project's own deletion) removes an `account_projects` row, so an absent
/// row on a project this account was seen holding is never a stale-cache ambiguity.
pub fn left_elsewhere(
    state: &LocalStorageState,
    server: &[AccountProject],
    account_id: Uuid,
) -> Vec<Uuid> {
    state
        .projects
        .iter()
        .filter(|local| local.synced_account == Some(account_id))
        .filter(|local| !server.iter().any(|r| r.project_id == local.project_id))
        .map(|local| local.project_id)
        .collect()
}

/// What this device should send up, given what the server already has.
///
/// An entry is pushed when the server is missing the membership, the participant, or the escrowed
/// key. Nothing is pushed for a project the server already knows about in full — reconciliation
/// runs on every projects-page load with an account, and a batch of unchanged rows is a write the
/// database does not need.
///
/// `account_key` is `None` for a session restored from the cookie: the password is gone, so the
/// account key cannot be re-derived and no key can be wrapped. Memberships still go up.
///
/// `account` is what the claim label is built from — it needs the account key too, to read
/// `display_name` before re-encrypting it under the project key. Absent either, the claim goes up
/// unlabelled and the label is attached on the next project open.
pub fn to_push(
    state: &LocalStorageState,
    server: &[AccountProject],
    account_key: Option<&[u8; 32]>,
    account: Option<&Account>,
) -> Vec<UpsertAccountProject> {
    let known: HashMap<Uuid, &AccountProject> =
        server.iter().map(|p| (p.project_id, p)).collect();

    state
        .projects
        .iter()
        .filter_map(|local| {
            let remote = known.get(&local.project_id);

            // Only wrap what the server lacks. Re-wrapping an already-escrowed key on every load
            // would burn a fresh nonce and a write for nothing, and `COALESCE` would ignore it.
            let key = match (account_key, remote.map(|r| r.key.is_some())) {
                (Some(_), Some(true)) => None,
                (Some(ak), _) => key_of(local).and_then(|pk| wrap_project_key(ak, &pk)),
                (None, _) => None,
            };

            let project_key = key_of(local);
            // An identity without the key behind it cannot be proven and would only be refused;
            // it stays local until the key arrives. Reachable only from pre-escrow localStorage.
            let user_id = local.user_id.filter(|_| project_key.is_some());

            let user_id_missing = user_id.is_some() && remote.is_none_or(|r| r.user_id.is_none());
            // Seeding the verifier on a project that predates it, from the first key-holding
            // login: the token goes up whenever the server says the project still has none.
            let verifier_missing =
                project_key.is_some() && remote.is_none_or(|r| !r.claim_verifier_seeded);

            // Only for a project this device is actually claiming an identity in: a label on a
            // row with no `user_id` is served to nobody.
            let label = match (user_id, account, account_key, project_key) {
                (Some(_), Some(a), Some(ak), Some(pk)) => claim_label(a, ak, &pk),
                _ => None,
            };

            if remote.is_none() || user_id_missing || key.is_some() || verifier_missing {
                Some(UpsertAccountProject {
                    project_id: local.project_id,
                    user_id,
                    key,
                    claim_label: label,
                    claim_token: project_key.map(|pk| claim_token(&pk).to_vec()),
                })
            } else {
                None
            }
        })
        .collect()
}

/// Whether some project this device claims an identity in — key and `user_id` both held — lacks
/// the account's payment-methods copy server-side. That is the copy a settings save could not
/// produce: saved on a device without this project's key, or before this one claimed its identity
/// here. The caller refills through `PUT /auth/payment-methods` from a fresh `me()`, never from
/// the `Account` in context — see `payment_methods::refill_project_copies`.
pub fn copy_missing(state: &LocalStorageState, server: &[AccountProject]) -> bool {
    state.projects.iter().any(|local| {
        local.user_id.is_some()
            && key_of(local).is_some()
            && server
                .iter()
                .any(|r| r.project_id == local.project_id && r.user_id.is_some() && !r.payment_methods_shared)
    })
}

/// Folds the account's projects into the local store: the membership always, the key only when this
/// device does not already hold one.
///
/// A key that fails to unwrap is skipped rather than reported. It means this device cannot read that
/// project — a wrong account key, a blob written under a password since changed — which is exactly
/// the state the locked card on the projects list exists to show.
///
/// The local key wins over the escrowed one when both are present: it is the copy that came from a
/// share link, and overwriting it with a blob the server handed us would let a compromised server
/// swap in a key of its choosing on a device that already had the right one.
///
/// Every pulled entry is marked as synced with `account_id`, which is what makes a later absence
/// from the server mean "left elsewhere" — see [`left_elsewhere`].
pub fn apply_pull(
    state: &mut LocalStorageState,
    server: &[AccountProject],
    account_key: Option<&[u8; 32]>,
    account_id: Uuid,
) {
    use super::local_storage::{upsert_project, upsert_project_key};
    use crate::crypto::key_to_fragment;

    for remote in server {
        upsert_project(state, remote.project_id, remote.user_id);
        mark_synced(state, &[remote.project_id], account_id);

        let already_held = state
            .projects
            .iter()
            .any(|p| p.project_id == remote.project_id && p.encryption_key.is_some());
        if already_held {
            continue;
        }

        let Some(ak) = account_key else { continue };
        let Some(wrapped) = remote.key.as_ref() else { continue };
        if let Some(pk) = unwrap_project_key(ak, wrapped) {
            upsert_project_key(state, remote.project_id, key_to_fragment(&pk));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::local_storage::LocalStorageProject;
    use crate::crypto::{generate_key, key_to_fragment};

    fn local(id: Uuid, user_id: Option<i32>, key: Option<String>) -> LocalStorageProject {
        LocalStorageProject { project_id: id, user_id, encryption_key: key, ..Default::default() }
    }

    fn state(projects: Vec<LocalStorageProject>) -> LocalStorageState {
        LocalStorageState { projects, ..Default::default() }
    }

    /// A server row for a project whose verifier is already seeded and whose payment-methods copy
    /// is in place — the steady state.
    fn remote(project_id: Uuid, user_id: Option<i32>, key: Option<shared::EncryptedPair>) -> AccountProject {
        AccountProject { project_id, user_id, key, claim_verifier_seeded: true, payment_methods_shared: true }
    }

    /// A membership write never carries a payment-methods copy, whatever the server lacks: the
    /// copy has one writer, and this is not it.
    #[test]
    fn a_missing_project_copy_is_never_pushed_with_the_membership() {
        let id = Uuid::new_v4();
        let project_key = generate_key();
        let s = state(vec![local(id, Some(7), Some(key_to_fragment(&project_key)))]);
        let mut server = remote(id, Some(7), Some(shared::EncryptedPair { ct: "x".into(), iv: "y".into() }));
        server.payment_methods_shared = false;

        assert!(to_push(&s, std::slice::from_ref(&server), Some(&generate_key()), None).is_empty());
        assert!(copy_missing(&s, &[server]), "but the gap is reported for the refill");
    }

    #[test]
    fn a_copy_the_server_already_holds_is_not_missing() {
        let id = Uuid::new_v4();
        let s = state(vec![local(id, Some(7), Some(key_to_fragment(&generate_key())))]);
        let server = vec![remote(id, Some(7), Some(shared::EncryptedPair { ct: "x".into(), iv: "y".into() }))];

        assert!(!copy_missing(&s, &server));
    }

    /// A row with no identity is served to nobody, and a device without the key could not
    /// encrypt a copy anyway: neither counts as missing.
    #[test]
    fn no_copy_is_missing_for_a_row_without_an_identity_or_a_key() {
        let id = Uuid::new_v4();
        let mut server = remote(id, None, Some(shared::EncryptedPair { ct: "x".into(), iv: "y".into() }));
        server.payment_methods_shared = false;

        let no_identity = state(vec![local(id, None, Some(key_to_fragment(&generate_key())))]);
        assert!(!copy_missing(&no_identity, std::slice::from_ref(&server)));

        server.user_id = Some(7);
        let no_key = state(vec![local(id, Some(7), None)]);
        assert!(!copy_missing(&no_key, &[server]));
    }

    #[test]
    fn pushes_local_only_projects() {
        let (a, b, c) = (Uuid::new_v4(), Uuid::new_v4(), Uuid::new_v4());
        let s = state(vec![local(a, None, None), local(b, None, None), local(c, None, None)]);
        let server = vec![remote(c, None, None)];

        let push = to_push(&s, &server, None, None);

        let ids: Vec<Uuid> = push.iter().map(|p| p.project_id).collect();
        assert_eq!(ids, vec![a, b], "only what the account is missing");
    }

    /// The scenario the feature exists for: local A,B,C + account C,D,E must converge on all five.
    #[test]
    fn reconciles_to_the_union() {
        let (a, b, c, d, e) =
            (Uuid::new_v4(), Uuid::new_v4(), Uuid::new_v4(), Uuid::new_v4(), Uuid::new_v4());
        let mut s = state(vec![local(a, None, None), local(b, None, None), local(c, None, None)]);
        let server: Vec<AccountProject> = [c, d, e]
            .iter()
            .map(|id| remote(*id, None, None))
            .collect();

        let pushed: Vec<Uuid> = to_push(&s, &server, None, None).iter().map(|p| p.project_id).collect();
        apply_pull(&mut s, &server, None, Uuid::new_v4());

        let mut local_ids: Vec<Uuid> = s.projects.iter().map(|p| p.project_id).collect();
        let mut union: Vec<Uuid> = pushed.iter().chain([c, d, e].iter()).copied().collect();
        local_ids.sort();
        union.sort();
        union.dedup();
        assert_eq!(local_ids.len(), 5, "device holds A B C D E");
        assert_eq!(union.len(), 5, "account ends up holding A B C D E");
    }

    #[test]
    fn wraps_and_unwraps_a_key_round_trip() {
        let id = Uuid::new_v4();
        let account_key = generate_key();
        let project_key = generate_key();
        let s = state(vec![local(id, None, Some(key_to_fragment(&project_key)))]);

        let push = to_push(&s, &[], Some(&account_key), None);
        let wrapped = push[0].key.clone().expect("key is escrowed");

        // A second device: same account, empty store.
        let mut fresh = state(vec![]);
        apply_pull(
            &mut fresh,
            &[remote(id, None, Some(wrapped))],
            Some(&account_key),
            Uuid::new_v4(),
        );

        assert_eq!(key_of(&fresh.projects[0]), Some(project_key));
    }

    #[test]
    fn a_key_the_server_already_holds_is_not_rewrapped() {
        let id = Uuid::new_v4();
        let account_key = generate_key();
        let s = state(vec![local(id, Some(7), Some(key_to_fragment(&generate_key())))]);
        let server = vec![remote(id, Some(7), Some(shared::EncryptedPair { ct: "x".into(), iv: "y".into() }))];

        assert!(to_push(&s, &server, Some(&account_key), None).is_empty());
    }

    /// A project that predates the verifier: the first key-holding login seeds it, and nothing
    /// else about the row needs pushing for that to happen.
    #[test]
    fn a_key_holder_pushes_its_token_while_the_verifier_is_unseeded() {
        let id = Uuid::new_v4();
        let project_key = generate_key();
        let s = state(vec![local(id, Some(7), Some(key_to_fragment(&project_key)))]);
        let mut server = remote(id, Some(7), Some(shared::EncryptedPair { ct: "x".into(), iv: "y".into() }));
        server.claim_verifier_seeded = false;

        let push = to_push(&s, &[server], None, None);

        assert_eq!(push.len(), 1);
        assert_eq!(push[0].claim_token, Some(claim_token(&project_key).to_vec()));
        assert!(push[0].key.is_none(), "nothing else was missing");
    }

    /// An identity with no key behind it cannot be proven; it stays local rather than being sent
    /// to be refused.
    #[test]
    fn a_user_id_without_the_key_is_not_pushed() {
        let id = Uuid::new_v4();
        let s = state(vec![local(id, Some(42), None)]);

        let push = to_push(&s, &[], None, None);

        assert_eq!(push.len(), 1, "the membership still goes up");
        assert_eq!(push[0].user_id, None);
        assert_eq!(push[0].claim_token, None);
    }

    #[test]
    fn no_account_key_still_pushes_the_membership() {
        let id = Uuid::new_v4();
        let s = state(vec![local(id, None, Some(key_to_fragment(&generate_key())))]);

        let push = to_push(&s, &[], None, None);

        assert_eq!(push.len(), 1);
        assert!(push[0].key.is_none(), "nothing to wrap with");
    }

    #[test]
    fn a_wrong_account_key_leaves_the_project_locked() {
        let id = Uuid::new_v4();
        let wrapped = wrap_project_key(&generate_key(), &generate_key()).unwrap();
        let mut s = state(vec![]);

        apply_pull(
            &mut s,
            &[remote(id, None, Some(wrapped))],
            Some(&generate_key()),
            Uuid::new_v4(),
        );

        assert_eq!(s.projects.len(), 1, "the membership still lands");
        assert!(s.projects[0].encryption_key.is_none(), "but no key — the card renders locked");
    }

    #[test]
    fn the_local_key_wins_over_the_escrowed_one() {
        let id = Uuid::new_v4();
        let account_key = generate_key();
        let held = generate_key();
        let other = generate_key();
        let mut s = state(vec![local(id, None, Some(key_to_fragment(&held)))]);

        apply_pull(
            &mut s,
            &[remote(id, None, Some(wrap_project_key(&account_key, &other).unwrap()))],
            Some(&account_key),
            Uuid::new_v4(),
        );

        assert_eq!(key_of(&s.projects[0]), Some(held));
    }

    /// The reported edge case. Devices 1 and 2 both hold A, B, C synced with the account. Device 1
    /// leaves C. Device 2 must forget C, not push it back — otherwise the leave is undone and C
    /// comes back onto device 1 at its next pull.
    #[test]
    fn a_project_left_on_another_device_is_forgotten_not_pushed_back() {
        let (a, b, c) = (Uuid::new_v4(), Uuid::new_v4(), Uuid::new_v4());
        let account = Uuid::new_v4();
        let mut device_2 = state(vec![local(a, None, None), local(b, None, None), local(c, None, None)]);
        let before_leave: Vec<AccountProject> = [a, b, c].iter().map(|id| remote(*id, None, None)).collect();
        apply_pull(&mut device_2, &before_leave, None, account);

        let after_leave = &before_leave[..2];
        let left = left_elsewhere(&device_2, after_leave, account);
        assert_eq!(left, vec![c]);

        for id in &left {
            crate::common::local_storage::remove_project(&mut device_2, *id);
        }
        assert!(to_push(&device_2, after_leave, None, None).is_empty(), "C is not pushed back");
        let ids: Vec<Uuid> = device_2.projects.iter().map(|p| p.project_id).collect();
        assert_eq!(ids, vec![a, b]);
    }

    /// Without the drop, the same state pushes C straight back: this is the bug, kept as the
    /// negative of the test above so the two cannot silently diverge.
    #[test]
    fn to_push_alone_would_resurrect_the_left_project() {
        let c = Uuid::new_v4();
        let account = Uuid::new_v4();
        let mut device_2 = state(vec![local(c, None, None)]);
        apply_pull(&mut device_2, &[remote(c, None, None)], None, account);

        let push = to_push(&device_2, &[], None, None);

        assert_eq!(push.len(), 1, "to_push knows nothing about leaves; left_elsewhere must run first");
    }

    /// A project this device holds but the account has never held is exactly what the push exists
    /// for — it is not "left elsewhere", whatever the server's silence.
    #[test]
    fn a_never_synced_project_is_not_treated_as_left() {
        let id = Uuid::new_v4();
        let s = state(vec![local(id, None, None)]);

        assert!(left_elsewhere(&s, &[], Uuid::new_v4()).is_empty());
        assert_eq!(to_push(&s, &[], None, None).len(), 1);
    }

    /// Logging in as a different account on the same device hands it the local list (documented as
    /// deliberate in project-membership.md §4). The mark is per account, so account X's leaves
    /// never make account Y forget anything.
    #[test]
    fn a_project_synced_with_another_account_is_still_pushed() {
        let id = Uuid::new_v4();
        let (x, y) = (Uuid::new_v4(), Uuid::new_v4());
        let mut s = state(vec![local(id, None, None)]);
        apply_pull(&mut s, &[remote(id, None, None)], None, x);

        assert!(left_elsewhere(&s, &[], y).is_empty());
        assert_eq!(to_push(&s, &[], None, None).len(), 1);
    }

    /// The device that pushed a project must also know it is now synced — otherwise a leave on
    /// another device before this one's next pull would be undone all the same.
    #[test]
    fn an_accepted_push_marks_the_entry_synced() {
        let id = Uuid::new_v4();
        let account = Uuid::new_v4();
        let mut s = state(vec![local(id, None, None)]);

        mark_synced(&mut s, &[id], account);

        assert_eq!(left_elsewhere(&s, &[], account), vec![id]);
    }

    /// A project still on the server is untouched, and the pull re-stamps the mark.
    #[test]
    fn a_project_the_server_still_lists_is_kept() {
        let id = Uuid::new_v4();
        let account = Uuid::new_v4();
        let mut s = state(vec![local(id, None, None)]);
        apply_pull(&mut s, &[remote(id, None, None)], None, account);

        assert!(left_elsewhere(&s, &[remote(id, None, None)], account).is_empty());
        assert_eq!(s.projects[0].synced_account, Some(account));
    }

    #[test]
    fn a_user_id_the_server_lacks_is_pushed() {
        let id = Uuid::new_v4();
        let project_key = generate_key();
        let s = state(vec![local(id, Some(42), Some(key_to_fragment(&project_key)))]);
        let server = vec![remote(id, None, None)];

        let push = to_push(&s, &server, None, None);

        assert_eq!(push.len(), 1);
        assert_eq!(push[0].user_id, Some(42));
        assert_eq!(push[0].claim_token, Some(claim_token(&project_key).to_vec()), "with its proof");
    }

    #[test]
    fn nothing_to_do_pushes_nothing() {
        let id = Uuid::new_v4();
        let s = state(vec![local(id, Some(1), None)]);
        let server = vec![remote(id, Some(1), None)];

        assert!(to_push(&s, &server, None, None).is_empty());
    }
}
