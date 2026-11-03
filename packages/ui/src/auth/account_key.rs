//! The account master key a device holds, and what happens when another device changes it
//! (docs/plans/key-lifecycle.md §3).
//!
//! At epoch 0 the master key is the password-derived key itself. Past it, `amk_wrap` holds a random
//! master key under the password-derived key, and every account-key write names the epoch it was
//! made under: the server refuses a stale one with 409 `ACCOUNT_KEY_CHANGED`, which lands here.

use api::auth::auth_controller::{logout, me};
use dioxus::prelude::*;
use shared::{errors, Account};
use uuid::Uuid;

use crate::common::{flush_session, read_from_ls, write_to_ls, Flash, LocalStorageState};
use crate::crypto::{forget_decrypted, key_to_fragment, open_amk, public_key_of, unwrap_key, WrapUse};

/// Set by any write the server refused with `ACCOUNT_KEY_CHANGED`; `use_app_contexts` settles it.
pub static ACCOUNT_KEY_STALE: GlobalSignal<bool> = Signal::global(|| false);

/// The epoch of the key this device holds, for every account-key write.
pub fn held_amk_epoch() -> Option<i32> {
    Some(read_from_ls().held_amk_epoch())
}

pub fn is_account_key_changed(e: &ServerFnError) -> bool {
    matches!(e, ServerFnError::ServerError { message, .. } if message == errors::ACCOUNT_KEY_CHANGED)
}

/// The one handler for a refused account-key write, whatever wrote it.
pub fn note<T>(result: &Result<T, ServerFnError>) {
    if let Err(e) = result {
        if is_account_key_changed(e) {
            *ACCOUNT_KEY_STALE.write() = true;
        }
    }
}

/// The master key `pw_key` opens for `account`, checked before use: at epoch 0 it is `pw_key`
/// itself; past it, the strictly typed `amk_wrap` must open and the master key must open the
/// private key whose public half the server serves. `None` below the epoch this device pinned.
pub fn master_key(account: &Account, pw_key: &[u8; 32], pinned: Option<i32>) -> Option<[u8; 32]> {
    if pinned.is_some_and(|pin| account.amk_epoch < pin) {
        return None;
    }
    if account.amk_epoch == 0 {
        return account.amk_wrap.is_none().then_some(*pw_key);
    }
    let amk = open_amk(pw_key, account.id, account.amk_epoch, account.amk_wrap.as_ref()?)?;
    opens_private_key(account, &amk).then_some(amk)
}

fn opens_private_key(account: &Account, key: &[u8; 32]) -> bool {
    match (&account.private_key, &account.public_key) {
        (Some(wrapped), Some(public)) => {
            unwrap_key(key, WrapUse::PrivateKey, wrapped, false).is_some_and(|sk| &public_key_of(&sk) == public)
        }
        _ => false,
    }
}

#[derive(Debug, PartialEq)]
pub enum Reconciled {
    Unchanged,
    CaughtUp([u8; 32]),
    Lost,
}

/// What the held key is worth against the account the server serves. A device behind tries the
/// served wrap with what it holds, which works for an epoch-0 device after a rotation that kept the
/// password; anything else needs the password again.
pub fn reconcile(account: &Account, held: &[u8; 32], held_epoch: i32) -> Reconciled {
    if account.amk_epoch == held_epoch {
        return Reconciled::Unchanged;
    }
    if account.amk_epoch < held_epoch {
        return Reconciled::Lost;
    }
    match master_key(account, held, Some(held_epoch)) {
        Some(amk) => Reconciled::CaughtUp(amk),
        None => Reconciled::Lost,
    }
}

pub fn remember(ls_ctx: Signal<LocalStorageState>, account: &Account, key: &[u8; 32]) {
    let mut ls_ctx = ls_ctx;
    let mut state = read_from_ls();
    state.remember_account_key(account.id, account.amk_epoch, key_to_fragment(key));
    write_to_ls(&state);
    ls_ctx.set(state);
}

/// Signs this device out of an account whose key it can no longer use, keeping that key aside for
/// what a reset retired. The projects and their keys stay.
pub async fn sign_out_stale(
    mut ls_ctx: Signal<LocalStorageState>,
    mut key_ctx: Signal<Option<[u8; 32]>>,
    mut auth: Signal<Option<Account>>,
) {
    let _ = logout().await;
    flush_session();
    let mut state = read_from_ls();
    state.set_aside_account_key(None);
    state.account_key_owner = None;
    write_to_ls(&state);
    ls_ctx.set(state);
    forget_decrypted();
    key_ctx.set(None);
    auth.set(None);
    if let Some(mut flash) = try_consume_context::<Signal<Option<Flash>>>() {
        flash.set(Some(Flash::err(crate::tid!("error-account-key-changed"))));
    }
}

/// After a refused write: catch up if the held key opens the new epoch, sign out otherwise.
pub async fn settle_stale(
    ls_ctx: Signal<LocalStorageState>,
    key_ctx: Signal<Option<[u8; 32]>>,
    auth: Signal<Option<Account>>,
) {
    let mut key_ctx = key_ctx;
    let mut auth = auth;
    let (Ok(Some(account)), Some(held)) = (me().await, key_ctx()) else { return };
    match reconcile(&account, &held, read_from_ls().held_amk_epoch()) {
        Reconciled::Unchanged => auth.set(Some(account)),
        Reconciled::CaughtUp(amk) => {
            remember(ls_ctx, &account, &amk);
            key_ctx.set(Some(amk));
            auth.set(Some(account));
        }
        Reconciled::Lost => sign_out_stale(ls_ctx, key_ctx, auth).await,
    }
}

/// The owner of a key remembered before owners were recorded: the session's account, since the
/// key and the session are set and cleared together.
pub fn adopt_owner(state: &mut LocalStorageState, account_id: Uuid) -> bool {
    if state.account_key.is_some() && state.account_key_owner.is_none() {
        state.account_key_owner = Some(account_id);
        return true;
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypto::{encrypt_json, generate_key, wrap_amk, wrap_key};
    use chrono::NaiveDateTime;

    fn account(epoch: i32, amk_wrap: Option<shared::EncryptedPair>, sk_key: &[u8; 32], sk: &[u8; 32]) -> Account {
        Account {
            id: Uuid::from_u128(7),
            email: "a@example.com".into(),
            display_name: encrypt_json(sk_key, &"A".to_string()).unwrap(),
            created_at: NaiveDateTime::default(),
            email_verified: true,
            kdf_salt: vec![0; 16],
            kdf_version: 1,
            preferences: None,
            payment_methods: None,
            public_key: Some(public_key_of(sk)),
            private_key: wrap_key(sk_key, WrapUse::PrivateKey, sk),
            typed_wraps: true,
            amk_epoch: epoch,
            amk_wrap,
            key_lifecycle: true,
            ..Default::default()
        }
    }

    #[test]
    fn at_epoch_zero_the_master_key_is_the_password_key() {
        let (pw, sk) = (generate_key(), generate_key());
        assert_eq!(master_key(&account(0, None, &pw, &sk), &pw, None), Some(pw));
    }

    #[test]
    fn past_epoch_zero_the_wrap_must_open_the_served_private_key() {
        let (pw, amk, sk) = (generate_key(), generate_key(), generate_key());
        let id = Uuid::from_u128(7);
        let good = account(1, wrap_amk(&pw, id, 1, &amk), &amk, &sk);
        assert_eq!(master_key(&good, &pw, None), Some(amk));

        let forged = generate_key();
        let substituted = account(1, wrap_amk(&pw, id, 1, &forged), &amk, &sk);
        assert_eq!(master_key(&substituted, &pw, None), None, "a key that opens no private key");
    }

    #[test]
    fn a_served_epoch_below_the_pin_is_refused() {
        let (pw, sk) = (generate_key(), generate_key());
        assert_eq!(master_key(&account(0, None, &pw, &sk), &pw, Some(1)), None);
    }

    #[test]
    fn an_epoch_zero_account_serving_a_wrap_is_refused() {
        let (pw, sk) = (generate_key(), generate_key());
        let wrap = wrap_amk(&pw, Uuid::from_u128(7), 0, &generate_key());
        assert_eq!(master_key(&account(0, wrap, &pw, &sk), &pw, None), None);
    }

    #[test]
    fn an_epoch_zero_device_catches_up_after_a_rotation_that_kept_the_password() {
        let (pw, amk, sk) = (generate_key(), generate_key(), generate_key());
        let rotated = account(1, wrap_amk(&pw, Uuid::from_u128(7), 1, &amk), &amk, &sk);

        assert_eq!(reconcile(&rotated, &pw, 0), Reconciled::CaughtUp(amk));
        assert_eq!(reconcile(&rotated, &amk, 1), Reconciled::Unchanged);
    }

    #[test]
    fn a_device_whose_key_opens_nothing_new_must_sign_in_again() {
        let (pw, new_pw, amk, sk) = (generate_key(), generate_key(), generate_key(), generate_key());
        let changed = account(2, wrap_amk(&new_pw, Uuid::from_u128(7), 2, &amk), &amk, &sk);

        assert_eq!(reconcile(&changed, &pw, 1), Reconciled::Lost);
        assert_eq!(reconcile(&changed, &amk, 3), Reconciled::Lost, "a server serving an older epoch");
    }

    #[test]
    fn a_replaced_key_is_kept_aside_for_its_account_only() {
        let mut state = LocalStorageState::default();
        let (a, b) = (Uuid::from_u128(1), Uuid::from_u128(2));
        state.remember_account_key(a, 0, "k0".into());
        state.remember_account_key(a, 1, "k1".into());
        state.remember_account_key(b, 0, "kb".into());

        assert_eq!(state.previous_account_keys[&a].iter().map(|p| p.key.as_str()).collect::<Vec<_>>(), ["k0", "k1"]);
        assert!(!state.previous_account_keys.contains_key(&b));
        assert_eq!(state.amk_epoch_pins[&a], 1);
        assert_eq!(state.held_amk_epoch(), 0);
    }

    #[test]
    fn the_account_key_error_has_a_message() {
        let known = crate::i18n::locale_ids(crate::i18n::FALLBACK);
        assert!(known.contains("error-account-key-changed"));
        assert_eq!(crate::common::error_key(&ServerFnError::new(errors::ACCOUNT_KEY_CHANGED)), "error-account-key-changed");
    }
}
