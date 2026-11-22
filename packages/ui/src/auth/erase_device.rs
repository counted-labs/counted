use api::account_projects::account_projects_controller::{batch_upsert_account_projects, get_account_projects};
use dioxus::fullstack::Json;
use dioxus::prelude::Signal;
use shared::{Account, AccountProject};
use uuid::Uuid;

use crate::common::fx_cache::clear_fx;
use crate::common::offline_queue::erase_queue;
use crate::common::project_create::still_confirming;
use crate::common::{
    key_of, left_elsewhere, read_from_ls, remove_quarantined_ls, to_push, write_to_ls, LocalStorageState,
};
use crate::crypto::{forget_decrypted, open_wrap, WrapUse};
use crate::decrypted::decrypt_project;

#[derive(Clone, PartialEq, Debug)]
pub enum EraseCheck {
    Checking,
    /// Queued changes not sent yet.
    Pending(usize),
    /// Entries that will never send by themselves (SPEC §15.9.5 Q9): only a discard clears them.
    Unsent(usize),
    /// A creation that may still commit: it settles within two minutes.
    Confirming,
    Unreachable,
    /// `at_risk` names the projects whose key the account's escrow cannot give back: erasing loses
    /// them, as it loses any creation still unconfirmed.
    Ready { at_risk: Vec<String>, unconfirmed: Unconfirmed },
}

/// Creations the server never confirmed, left after settling: if one landed, only this device holds
/// its key. `other_account` when one waits for an account other than the one signed in.
#[derive(Clone, PartialEq, Debug, Default)]
pub struct Unconfirmed {
    pub count: usize,
    pub other_account: bool,
}

/// Projects this device holds a key for that no other device can recover after the erase.
pub fn unescrowed(state: &LocalStorageState, server: &[AccountProject], account_key: Option<&[u8; 32]>) -> Vec<Uuid> {
    state
        .projects
        .iter()
        .filter(|local| !local.key_rotated)
        .filter_map(|local| {
            let project_key = key_of(local)?;
            let wrapped = server.iter().find(|r| r.project_id == local.project_id).and_then(|r| r.key.as_ref());
            // Without the account key here the wrap cannot be opened, but signing in derives the key
            // again from the password, so a wrap the server holds still brings the project back.
            let restorable = match (account_key, wrapped) {
                (Some(ak), Some(w)) => {
                    open_wrap(ak, WrapUse::Escrow(local.project_id), w).is_some_and(|u| u.key == project_key)
                }
                (None, Some(_)) => true,
                (_, None) => false,
            };
            (!restorable).then_some(local.project_id)
        })
        .collect()
}

pub fn project_names(state: &LocalStorageState, ids: &[Uuid]) -> Vec<String> {
    ids.iter()
        .map(|id| {
            state
                .projects
                .iter()
                .find(|p| p.project_id == *id)
                .and_then(|p| decrypt_project(&key_of(p)?, p.cached_project.as_ref()?).ok())
                .map_or_else(|| id.to_string(), |p| p.name)
        })
        .collect()
}

/// The language belongs to the device, as at logout. Everything else goes.
pub fn erased(state: &LocalStorageState) -> LocalStorageState {
    LocalStorageState { language: state.language.clone(), ..Default::default() }
}

/// What the queue in storage holds that an erase would lose, counted when the sheet opens and again
/// when the erase is confirmed, since another tab can queue in between. `None` when it is empty.
pub fn queue_check(queue: &[crate::common::Entry], state: &LocalStorageState, refused: &std::collections::BTreeMap<Uuid, String>) -> Option<EraseCheck> {
    let unsent = crate::common::unsent(queue, state, refused).len();
    match (unsent, queue.len()) {
        (0, 0) => None,
        (0, pending) => Some(EraseCheck::Pending(pending)),
        (unsent, _) => Some(EraseCheck::Unsent(unsent)),
    }
}

/// What stops an erase, or what it loses, once the pending creations were settled. An erase is never
/// refused for good: a creation that may still commit holds it back for two minutes at most.
pub fn held_back(queued: Option<EraseCheck>, state: &LocalStorageState, account: Uuid, now_ms: i64) -> Result<Unconfirmed, EraseCheck> {
    if let Some(queued) = queued {
        return Err(queued);
    }
    let records = &state.pending_creations;
    if records.iter().any(|p| still_confirming(p, now_ms)) {
        return Err(EraseCheck::Confirming);
    }
    Ok(Unconfirmed { count: records.len(), other_account: records.iter().any(|p| p.account.is_some_and(|a| a != account)) })
}

/// Settles the pending creations and pushes what the escrow lacks first, as the projects page does,
/// so that only a project the push could not save is reported. Projects left on another device are
/// not pushed back.
pub async fn check_erase(ls_ctx: Signal<LocalStorageState>, account: Account, account_key: Option<[u8; 32]>, queued: Option<EraseCheck>) -> EraseCheck {
    if queued.is_none() {
        crate::common::resolve_pending(ls_ctx).await;
    }
    let unconfirmed = match held_back(queued, &read_from_ls(), account.id, crate::common::project_create::now_ms()) {
        Ok(unconfirmed) => unconfirmed,
        Err(check) => return check,
    };
    let Ok(server) = get_account_projects().await else {
        return EraseCheck::Unreachable;
    };
    let mut state = read_from_ls();
    if !crate::auth::recovery_flow::memberships_reset(&state, &account) {
        let left = left_elsewhere(&state, &server, account.id);
        state.projects.retain(|p| !left.contains(&p.project_id));
    }

    let push = to_push(&state, &server, account_key.as_ref(), Some(&account));
    let server = if push.is_empty() {
        server
    } else {
        let written = batch_upsert_account_projects(Json(push)).await;
        crate::auth::account_key::note(&written);
        if written.is_err() {
            return EraseCheck::Unreachable;
        }
        let Ok(server) = get_account_projects().await else {
            return EraseCheck::Unreachable;
        };
        server
    };
    EraseCheck::Ready { at_risk: project_names(&state, &unescrowed(&state, &server, account_key.as_ref())), unconfirmed }
}

/// The device state without its project keys goes first (SPEC §15.9.5 Q3): a tab that has not seen
/// the erase yet then makes no entry, and the queue's `z` key ends what it holds.
pub fn erase_device_stores() {
    let (kept, theme) = (erased(&read_from_ls()), crate::common::stored_theme());
    write_to_ls(&kept);
    erase_queue();
    remove_quarantined_ls();
    crate::badges::clear_store();
    clear_fx();
    crate::common::store_seal::erase();
    write_to_ls(&kept);
    if theme != crate::common::Theme::default() {
        crate::common::store_theme(theme);
    }
    forget_decrypted();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::LocalStorageProject;
    use crate::crypto::{generate_key, key_to_fragment, wrap_key};

    fn holding(id: Uuid, key: &[u8; 32]) -> LocalStorageProject {
        LocalStorageProject { project_id: id, encryption_key: Some(key_to_fragment(key)), ..Default::default() }
    }

    fn row(project_id: Uuid, key: Option<shared::EncryptedPair>) -> AccountProject {
        AccountProject {
            project_id,
            user_id: None,
            key,
            claim_verifier_seeded: true,
            payment_methods_shared: true,
            history_has_actors: None,
            admin: false,
        }
    }

    /// An entry that will never send by itself points at the Unsent row, not at the network.
    #[test]
    fn the_erase_check_tells_unsent_from_pending() {
        let project = Uuid::from_u128(5);
        let state = LocalStorageState { projects: vec![holding(project, &generate_key())], ..Default::default() };
        let entry = |project_id: Uuid| {
            crate::common::Entry::new(crate::common::QueuedOp {
                id: Uuid::new_v4(),
                label: String::new(),
                sealed_label: None,
                after: None,
                queued_at: None,
                op: crate::common::OpKind::DeleteExpense(shared::DeleteExpenseRequest { id: 3, project_id, history: None }),
            })
        };
        let none = std::collections::BTreeMap::new();
        assert_eq!(queue_check(&[], &state, &none), None);
        assert_eq!(queue_check(&[entry(project)], &state, &none), Some(EraseCheck::Pending(1)));
        assert_eq!(queue_check(&[entry(project), entry(Uuid::from_u128(6))], &state, &none), Some(EraseCheck::Unsent(1)), "no key held here opens it");
    }

    #[test]
    fn only_a_key_the_escrow_cannot_give_back_is_at_risk() {
        let account_key = generate_key();
        let (escrowed, missing, no_row, wrong, locked) =
            (Uuid::new_v4(), Uuid::new_v4(), Uuid::new_v4(), Uuid::new_v4(), Uuid::new_v4());
        let keys: Vec<[u8; 32]> = (0..4).map(|_| generate_key()).collect();
        let state = LocalStorageState {
            projects: vec![
                holding(escrowed, &keys[0]),
                holding(missing, &keys[1]),
                holding(no_row, &keys[2]),
                holding(wrong, &keys[3]),
                LocalStorageProject { project_id: locked, ..Default::default() },
            ],
            ..Default::default()
        };
        let server = vec![
            row(escrowed, wrap_key(&account_key, WrapUse::Escrow(escrowed), &keys[0])),
            row(missing, None),
            row(wrong, wrap_key(&account_key, WrapUse::Escrow(wrong), &generate_key())),
            row(locked, None),
        ];

        assert_eq!(unescrowed(&state, &server, Some(&account_key)), vec![missing, no_row, wrong]);
    }

    #[test]
    fn without_the_account_key_a_wrap_on_the_server_still_counts() {
        let (wrapped, bare) = (Uuid::new_v4(), Uuid::new_v4());
        let (k1, k2) = (generate_key(), generate_key());
        let state = LocalStorageState { projects: vec![holding(wrapped, &k1), holding(bare, &k2)], ..Default::default() };
        let server = vec![row(wrapped, wrap_key(&generate_key(), WrapUse::Escrow(wrapped), &k1)), row(bare, None)];

        assert_eq!(unescrowed(&state, &server, None), vec![bare]);
    }

    #[test]
    fn a_project_whose_key_changed_loses_nothing_on_erase() {
        let (rotated, bare) = (Uuid::new_v4(), Uuid::new_v4());
        let state = LocalStorageState {
            projects: vec![
                LocalStorageProject { key_rotated: true, ..holding(rotated, &generate_key()) },
                holding(bare, &generate_key()),
            ],
            ..Default::default()
        };

        assert_eq!(unescrowed(&state, &[], None), vec![bare]);
    }

    #[test]
    fn a_project_with_no_readable_cache_is_named_by_its_id() {
        let id = Uuid::new_v4();
        let state = LocalStorageState { projects: vec![holding(id, &generate_key())], ..Default::default() };

        assert_eq!(project_names(&state, &[id]), vec![id.to_string()]);
    }

    fn creation(account: Option<Uuid>, sent_at: i64) -> crate::common::PendingCreation {
        crate::common::PendingCreation { project_id: Uuid::new_v4(), key: "k".into(), creator_secret: None, account, sent_at }
    }

    /// A of the confirmation pass: a device must never be impossible to erase.
    #[test]
    fn an_unconfirmed_creation_holds_the_erase_back_two_minutes_at_most_then_is_named() {
        let (me, other, now) = (Uuid::new_v4(), Uuid::new_v4(), 10_000_000);
        let mut state = LocalStorageState::default();
        assert_eq!(held_back(None, &state, me, now), Ok(Unconfirmed::default()));

        state.pending_creations.push(creation(Some(me), now - 1_000));
        assert_eq!(held_back(None, &state, me, now), Err(EraseCheck::Confirming));
        assert_eq!(held_back(Some(EraseCheck::Pending(2)), &state, me, now), Err(EraseCheck::Pending(2)), "queued changes keep their own count");

        let later = now + 120_000;
        assert_eq!(held_back(None, &state, me, later), Ok(Unconfirmed { count: 1, other_account: false }));
        state.pending_creations.push(creation(Some(other), now - 1_000));
        state.pending_creations.push(creation(None, now - 1_000));
        assert_eq!(held_back(None, &state, me, later), Ok(Unconfirmed { count: 3, other_account: true }));
    }

    #[test]
    fn erasing_keeps_the_language_and_nothing_else() {
        let account = Uuid::new_v4();
        let state = LocalStorageState {
            projects: vec![holding(Uuid::new_v4(), &generate_key())],
            onboarding_seen: true,
            invite_mail_off: true,
            show_archived: true,
            cached_projects_list: Some(vec![]),
            account_key: Some("k".into()),
            language: Some("fr".into()),
            personal_project: Some(Uuid::new_v4()),
            typed_wrap_accounts: vec![account],
            friend_keys: [(account, "k".into())].into(),
            friend_kem_keys: [(account, "k".into())].into(),
            account_key_owner: Some(account),
            account_key_epoch: Some(1),
            amk_epoch_pins: [(account, 1)].into(),
            previous_account_keys: [(account, vec![crate::common::local_storage::PreviousAccountKey { key: "k0".into(), epoch: 0 }])].into(),
            membership_epochs: [(account, 1)].into(),
            pending_creations: vec![crate::common::PendingCreation { project_id: Uuid::new_v4(), key: "k".into(), creator_secret: None, account: None, sent_at: 0 }],
        };

        assert_eq!(erased(&state), LocalStorageState { language: Some("fr".into()), ..Default::default() });
    }
}
