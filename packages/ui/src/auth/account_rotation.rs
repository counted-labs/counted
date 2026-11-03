//! Changing the password, and moving every account blob to a new master key when it does
//! (docs/plans/key-lifecycle.md §4, §5).

use api::auth::auth_controller::{
    cancel_recovery_replacement as cancel_replacement, change_password as put_password, login_salt, me, rotation_set, set_recovery_key,
};
use dioxus::fullstack::Json;
use dioxus::prelude::ServerFnError;
use shared::{
    errors, Account, AccountRotation, ChangePassword, EncryptedPair, LoginSaltPayload, NewPublicKeys, RecoveryAuth, RecoverySetup,
    RotatedWrap, RotationSet, SetRecoveryKey,
};

use crate::auth::account_key::master_key;
use crate::auth::credentials::{account_key, checked, login_proof, nfc, password_forms};
use crate::common::read_from_ls;
use crate::crypto::{
    decrypt, encrypt, generate_kdf_salt, generate_key, generate_keypair, open_kem_seed, public_key_of, unwrap_key, wrap_amk, wrap_kem_seed, wrap_key,
    wrap_recovery, WrapUse,
};
use crate::auth::recovery_key;
use crate::hybrid::{generate_kem_seed, kem_public_key};

/// The same plaintext bytes sealed under the new key: padding, fields a newer build wrote and a
/// blob that never parsed all survive.
fn reseal(old: &[u8; 32], new: &[u8; 32], pair: &EncryptedPair) -> Option<EncryptedPair> {
    encrypt(new, &decrypt(old, pair).ok()?).ok()
}

/// Every account-key blob under `new`. What does not open under `old` is marked `None`: the server
/// retires it with the rest and clears it, and a device holding that project key pushes it again.
/// `None` when the display name does not open, which means `old` is not the account's key.
pub fn build(account: &Account, old: &[u8; 32], new: &[u8; 32], set: &RotationSet, raw_ok: bool) -> Option<AccountRotation> {
    let private_key = account
        .private_key
        .as_ref()
        .and_then(|wrapped| unwrap_key(old, WrapUse::PrivateKey, wrapped, raw_ok))
        .filter(|sk| account.public_key.as_ref() == Some(&public_key_of(sk)))
        .and_then(|sk| wrap_key(new, WrapUse::PrivateKey, &sk));
    let escrow = set
        .escrow
        .iter()
        .map(|row| RotatedWrap {
            id: row.id,
            wrap: row
                .wrap
                .as_ref()
                .and_then(|w| unwrap_key(old, WrapUse::Escrow(row.id), w, raw_ok))
                .and_then(|k| wrap_key(new, WrapUse::Escrow(row.id), &k)),
        })
        .collect();
    let friend_labels = set
        .friend_labels
        .iter()
        .map(|row| RotatedWrap { id: row.id, wrap: row.wrap.as_ref().and_then(|w| reseal(old, new, w)) })
        .collect();
    let kem_private_key = account
        .kem_private_key
        .as_ref()
        .and_then(|wrapped| open_kem_seed(old, wrapped))
        .filter(|seed| account.kem_public_key.as_ref() == Some(&kem_public_key(seed)))
        .and_then(|seed| wrap_kem_seed(new, &seed));
    Some(AccountRotation {
        display_name: reseal(old, new, &account.display_name)?,
        preferences: account.preferences.as_ref().and_then(|p| reseal(old, new, p)),
        payment_methods: account.payment_methods.as_ref().and_then(|p| reseal(old, new, p)),
        private_key,
        escrow,
        friend_labels,
        kem_private_key,
        new_keypair: None,
    })
}

/// Whether `key` is the account's master key: at epoch 0 nothing else checks it.
fn opens_account(account: &Account, key: &[u8; 32]) -> bool {
    decrypt(key, &account.display_name).is_ok()
}

pub(crate) struct Opened {
    pub form: String,
    pub pw_key: [u8; 32],
    pub master: [u8; 32],
}

/// The master key the current password opens, the password-derived key, and the form of the
/// password it was set as.
pub(crate) async fn open_with(account: &Account, current: &str) -> Result<Opened, ServerFnError> {
    let pinned = read_from_ls().amk_epoch_pins.get(&account.id).copied();
    for form in password_forms(current) {
        let pw_key = account_key(&form, account.kdf_salt.clone()).await?;
        if let Some(master) = master_key(account, &pw_key, pinned).filter(|k| opens_account(account, k)) {
            return Ok(Opened { form, pw_key, master });
        }
    }
    Err(ServerFnError::new(errors::INVALID_CREDENTIALS))
}

/// What a rotation hands back: the account as stored, its master key, and the new recovery key a
/// compromise rotation made, to show once.
pub struct Rotated {
    pub account: Account,
    pub master: [u8; 32],
    pub recovery_key: Option<String>,
}

/// Docs/plans/key-lifecycle.md §4. At epoch 0 the account moves to a random master key in the same
/// request; past it only the wrap changes. Returns the account as now stored and its master key.
pub async fn change_password(current: &str, new: &str) -> Result<Rotated, ServerFnError> {
    rotate(current, new, false).await
}

/// Plan §5: "my password leaked", "I lost a device". A new password, a new master key and a new
/// hybrid keypair in one request; friends see this account's code change, and pending invitations
/// are dropped. The thief keeps the project keys they already opened.
/// A recovery key in place is replaced too: the old one may sit where the thief looked. The new one
/// takes effect 72 hours later (plan §6.1).
pub async fn compromise_rotation(current: &str, new: &str) -> Result<Rotated, ServerFnError> {
    rotate(current, new, true).await
}

/// A fresh X25519 and ML-KEM keypair, private halves wrapped under `amk`.
fn new_keypair(rotation: &mut AccountRotation, amk: &[u8; 32]) -> Option<()> {
    let (public_key, private) = generate_keypair();
    let seed = generate_kem_seed();
    rotation.private_key = Some(wrap_key(amk, WrapUse::PrivateKey, &private)?);
    rotation.kem_private_key = Some(wrap_kem_seed(amk, &seed)?);
    rotation.new_keypair = Some(NewPublicKeys { public_key, kem_public_key: kem_public_key(&seed) });
    Some(())
}

async fn rotate(current: &str, new: &str, compromise: bool) -> Result<Rotated, ServerFnError> {
    let account = me().await?.ok_or_else(|| ServerFnError::new(errors::UNAUTHENTICATED))?;
    let salt = checked(login_salt(Json(LoginSaltPayload { email: account.email.clone() })).await?)?;
    let Opened { form: current_form, master: amk, .. } = open_with(&account, current).await?;
    let current_proof = login_proof(salt.auth_version, &current_form, salt.salt.clone()).await?;

    let new = nfc(new);
    let new_kdf_salt = generate_kdf_salt();
    let new_pw_key = account_key(&new, new_kdf_salt.to_vec()).await?;
    let new_proof = login_proof(salt.auth_version, &new, salt.salt).await?;

    let (rotation, master, epoch) = if account.amk_epoch == 0 || compromise {
        let set = rotation_set().await?;
        let fresh = generate_key();
        let raw_ok = read_from_ls().raw_wraps_ok(account.id);
        let mut rotation = build(&account, &amk, &fresh, &set, raw_ok).ok_or_else(|| ServerFnError::new(errors::ACCOUNT_KEY_CHANGED))?;
        if compromise {
            new_keypair(&mut rotation, &fresh).ok_or_else(|| ServerFnError::new(errors::INVALID_PAYLOAD))?;
        }
        (Some(rotation), fresh, account.amk_epoch + 1)
    } else {
        (None, amk, account.amk_epoch)
    };
    let amk_wrap = wrap_amk(&new_pw_key, account.id, epoch, &master).ok_or_else(|| ServerFnError::new(errors::INVALID_PAYLOAD))?;
    let fresh_recovery = (compromise && account.has_recovery_key).then(recovery_key::generate);
    let recovery = match fresh_recovery {
        Some(key) => Some(recovery_setup(&key, account.id, &master)?),
        None => None,
    };

    put_password(Json(ChangePassword {
        current_proof,
        new_proof,
        new_kdf_salt: new_kdf_salt.to_vec(),
        amk_wrap,
        expected_amk_epoch: account.amk_epoch,
        rotation,
        recovery,
    }))
    .await?;

    let stored = me().await?.ok_or_else(|| ServerFnError::new(errors::UNAUTHENTICATED))?;
    Ok(Rotated { account: stored, master, recovery_key: fresh_recovery.map(|k| recovery_key::encode(&k)) })
}

fn recovery_setup(key: &[u8; 32], account: uuid::Uuid, master: &[u8; 32]) -> Result<RecoverySetup, ServerFnError> {
    let derived = recovery_key::derive(key, account);
    Ok(RecoverySetup {
        recovery_wrap: wrap_recovery(&derived.rk_key, account, master).ok_or_else(|| ServerFnError::new(errors::INVALID_PAYLOAD))?,
        recovery_verifier: recovery_key::verifier(&derived.auth),
    })
}

/// Plan §6.1: the key the user has written down, saved. Needs the current password; at epoch 0 the
/// account first moves to a random master key, which the recovery key then wraps.
pub async fn save_recovery_key(current: &str, key_text: &str) -> Result<(Account, [u8; 32]), ServerFnError> {
    let key = recovery_key::parse(key_text).ok_or_else(|| ServerFnError::new(errors::RECOVERY_KEY_INVALID))?;
    let account = me().await?.ok_or_else(|| ServerFnError::new(errors::UNAUTHENTICATED))?;
    let salt = checked(login_salt(Json(LoginSaltPayload { email: account.email.clone() })).await?)?;
    let opened = open_with(&account, current).await?;
    let current_proof = login_proof(salt.auth_version, &opened.form, salt.salt).await?;

    let (rotation, amk_wrap, master) = if account.amk_epoch == 0 {
        let set = rotation_set().await?;
        let fresh = generate_key();
        let raw_ok = read_from_ls().raw_wraps_ok(account.id);
        let rotation = build(&account, &opened.master, &fresh, &set, raw_ok).ok_or_else(|| ServerFnError::new(errors::ACCOUNT_KEY_CHANGED))?;
        let wrap = wrap_amk(&opened.pw_key, account.id, 1, &fresh).ok_or_else(|| ServerFnError::new(errors::INVALID_PAYLOAD))?;
        (Some(rotation), Some(wrap), fresh)
    } else {
        (None, None, opened.master)
    };

    set_recovery_key(Json(SetRecoveryKey {
        current_proof,
        expected_amk_epoch: account.amk_epoch,
        setup: recovery_setup(&key, account.id, &master)?,
        rotation,
        amk_wrap,
    }))
    .await?;
    let stored = me().await?.ok_or_else(|| ServerFnError::new(errors::UNAUTHENTICATED))?;
    Ok((stored, master))
}

/// Plan §6.1: the current recovery key cancels a pending replacement.
pub async fn cancel_recovery_replacement(key_text: &str) -> Result<(), ServerFnError> {
    let key = recovery_key::parse(key_text).ok_or_else(|| ServerFnError::new(errors::RECOVERY_KEY_INVALID))?;
    let account = me().await?.ok_or_else(|| ServerFnError::new(errors::UNAUTHENTICATED))?;
    let derived = recovery_key::derive(&key, account.id);
    cancel_replacement(Json(RecoveryAuth { recovery_auth: derived.auth.to_vec() })).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypto::{encrypt_json, encrypt_padded_json, open_wrap, wrap_key_untyped, Unwrapped};
    use uuid::Uuid;

    fn account(key: &[u8; 32], sk: &[u8; 32]) -> Account {
        Account {
            id: Uuid::from_u128(1),
            display_name: encrypt_padded_json(key, &"Marie".to_string()).unwrap(),
            preferences: Some(encrypt(key, r#"{"language":"fr","futureField":3}   "#).unwrap()),
            payment_methods: Some(encrypt_json(key, &"methods").unwrap()),
            public_key: Some(public_key_of(sk)),
            private_key: wrap_key(key, WrapUse::PrivateKey, sk),
            ..Default::default()
        }
    }

    /// Plan §5: blobs are resealed byte for byte, the private key re-wrapped as it is, so the
    /// keypair and the friend code never change on a routine password change.
    #[test]
    fn every_blob_moves_to_the_new_key_unchanged() {
        let (old, new, sk) = (generate_key(), generate_key(), generate_key());
        let a = account(&old, &sk);
        let rotation = build(&a, &old, &new, &RotationSet::default(), false).unwrap();

        assert_eq!(decrypt(&new, &rotation.display_name), decrypt(&old, &a.display_name));
        assert_eq!(decrypt(&new, rotation.preferences.as_ref().unwrap()).unwrap(), r#"{"language":"fr","futureField":3}   "#);
        assert_eq!(decrypt(&new, rotation.payment_methods.as_ref().unwrap()), decrypt(&old, a.payment_methods.as_ref().unwrap()));
        assert_eq!(open_wrap(&new, WrapUse::PrivateKey, rotation.private_key.as_ref().unwrap()), Some(Unwrapped { key: sk, typed: true }));
    }

    #[test]
    fn rows_that_do_not_open_are_marked_and_the_rest_rewrapped_typed() {
        let (old, new, sk) = (generate_key(), generate_key(), generate_key());
        let (good, untyped, junk, label) = (Uuid::from_u128(10), Uuid::from_u128(11), Uuid::from_u128(12), Uuid::from_u128(13));
        let (kg, ku) = (generate_key(), generate_key());
        let set = RotationSet {
            escrow: vec![
                RotatedWrap { id: good, wrap: wrap_key(&old, WrapUse::Escrow(good), &kg) },
                RotatedWrap { id: untyped, wrap: wrap_key_untyped(&old, WrapUse::Escrow(untyped), &ku) },
                RotatedWrap { id: junk, wrap: wrap_key(&generate_key(), WrapUse::Escrow(junk), &kg) },
                RotatedWrap { id: Uuid::from_u128(14), wrap: None },
            ],
            friend_labels: vec![RotatedWrap { id: label, wrap: Some(encrypt_padded_json(&old, &"Paul").unwrap()) }],
        };

        let rotation = build(&account(&old, &sk), &old, &new, &set, true).unwrap();
        let opened: Vec<Option<[u8; 32]>> = rotation
            .escrow
            .iter()
            .map(|r| r.wrap.as_ref().and_then(|w| open_wrap(&new, WrapUse::Escrow(r.id), w)).filter(|u| u.typed).map(|u| u.key))
            .collect();
        assert_eq!(opened, vec![Some(kg), Some(ku), None, None]);
        assert_eq!(rotation.escrow.iter().map(|r| r.id).collect::<Vec<_>>(), set.escrow.iter().map(|r| r.id).collect::<Vec<_>>());
        assert_eq!(decrypt(&new, rotation.friend_labels[0].wrap.as_ref().unwrap()), decrypt(&old, set.friend_labels[0].wrap.as_ref().unwrap()));

        let refused = build(&account(&old, &sk), &old, &new, &set, false).unwrap();
        assert!(refused.escrow[1].wrap.is_none(), "an untyped row once this device has seen typed ones");
    }

    #[test]
    fn a_key_that_is_not_the_accounts_builds_nothing() {
        let (old, sk) = (generate_key(), generate_key());
        assert!(build(&account(&old, &sk), &generate_key(), &generate_key(), &RotationSet::default(), true).is_none());
    }

    #[test]
    fn the_kem_seed_moves_with_the_rest_and_a_compromise_replaces_both_halves() {
        let (old, new, sk, seed) = (generate_key(), generate_key(), generate_key(), generate_kem_seed());
        let mut a = account(&old, &sk);
        a.kem_public_key = Some(kem_public_key(&seed));
        a.kem_private_key = wrap_kem_seed(&old, &seed);

        let mut rotation = build(&a, &old, &new, &RotationSet::default(), true).unwrap();
        assert_eq!(open_kem_seed(&new, rotation.kem_private_key.as_ref().unwrap()), Some(seed));
        assert!(rotation.new_keypair.is_none());

        new_keypair(&mut rotation, &new).unwrap();
        let keys = rotation.new_keypair.clone().unwrap();
        let new_sk = unwrap_key(&new, WrapUse::PrivateKey, rotation.private_key.as_ref().unwrap(), false).unwrap();
        let new_seed = open_kem_seed(&new, rotation.kem_private_key.as_ref().unwrap()).unwrap();
        assert_eq!(public_key_of(&new_sk), keys.public_key);
        assert_eq!(kem_public_key(&new_seed), keys.kem_public_key);
        assert_ne!(Some(keys.public_key), a.public_key, "a new X25519 key");
        assert_ne!(new_seed, seed, "a new ML-KEM key");
    }

    /// A private key whose public half is not the served one is not carried over (M2).
    #[test]
    fn a_substituted_private_key_is_not_rewrapped() {
        let (old, new, sk) = (generate_key(), generate_key(), generate_key());
        let mut a = account(&old, &sk);
        a.public_key = Some(public_key_of(&generate_key()));
        assert!(build(&a, &old, &new, &RotationSet::default(), true).unwrap().private_key.is_none());
    }
}
