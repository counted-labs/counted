use api::account_projects::account_projects_controller::{batch_upsert_account_projects, get_account_projects};
use dioxus::fullstack::Json;
use shared::{Account, AccountProject};
use uuid::Uuid;

use crate::common::fx_cache::clear_fx;
use crate::common::offline_queue::clear_queue;
use crate::common::{
    key_of, left_elsewhere, read_from_ls, remove_quarantined_ls, to_push, write_to_ls, LocalStorageState,
};
use crate::crypto::{forget_decrypted, open_wrap, WrapUse};
use crate::decrypted::decrypt_project;

#[derive(Clone, PartialEq, Debug)]
pub enum EraseCheck {
    Checking,
    Pending(usize),
    Unreachable,
    /// Names of the projects whose key the account's escrow cannot give back: erasing loses them.
    Ready(Vec<String>),
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

/// Pushes what the escrow lacks first, as the projects page does, so that only a project the push
/// could not save is reported. Projects left on another device are not pushed back.
pub async fn check_erase(account: Account, account_key: Option<[u8; 32]>, pending: usize) -> EraseCheck {
    if pending > 0 {
        return EraseCheck::Pending(pending);
    }
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
    EraseCheck::Ready(project_names(&state, &unescrowed(&state, &server, account_key.as_ref())))
}

pub fn erase_device_stores() {
    write_to_ls(&erased(&read_from_ls()));
    remove_quarantined_ls();
    clear_queue();
    clear_fx();
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

    #[test]
    fn erasing_keeps_the_language_and_nothing_else() {
        let account = Uuid::new_v4();
        let state = LocalStorageState {
            projects: vec![holding(Uuid::new_v4(), &generate_key())],
            onboarding_seen: true,
            show_archived: true,
            cached_projects_list: Some(vec![]),
            account_key: Some("k".into()),
            language: Some("fr".into()),
            personal_project: Some(Uuid::new_v4()),
            typed_wrap_accounts: vec![account],
            friend_keys: [(account, "k".into())].into(),
            account_key_owner: Some(account),
            account_key_epoch: Some(1),
            amk_epoch_pins: [(account, 1)].into(),
            previous_account_keys: [(account, vec![crate::common::local_storage::PreviousAccountKey { key: "k0".into(), epoch: 0 }])].into(),
            membership_epochs: [(account, 1)].into(),
        };

        assert_eq!(erased(&state), LocalStorageState { language: Some("fr".into()), ..Default::default() });
    }
}
