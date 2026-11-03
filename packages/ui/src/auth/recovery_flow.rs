//! Getting back into an account (docs/plans/key-lifecycle.md §6): with the recovery key, without it
//! after a delay, cancelling a reset, and undoing one with the old password.

use api::auth::auth_controller::{
    cancel_reset as post_cancel_reset, complete_recovery, forgot_password, login_salt, recovery_start, recovery_wrap,
    request_reset as post_request_reset, retired_generation, undo_reset as post_undo_reset,
};
use chrono::NaiveDateTime;
use dioxus::fullstack::Json;
use dioxus::prelude::ServerFnError;
use shared::{
    errors, Account, CancelReset, CompleteRecovery, ForgotPassword, LoginSaltPayload, NewPublicKeys, RecoveryStart, RecoveryWrapRequest,
    RequestReset, ResetToken, RetiredProof, UndoReset, AUTH_VERSION_CURRENT,
};

use crate::auth::credentials::{account_key, checked, login_proof, nfc, password_forms};
use crate::auth::recovery_key;
use crate::crypto::{
    encrypt_padded_json, generate_kdf_salt, generate_key, generate_keypair, open_amk, open_recovery, public_key_of, unwrap_key, wrap_amk,
    wrap_kem_seed, wrap_key, WrapUse,
};
use crate::hybrid::{generate_kem_seed, kem_public_key};

pub async fn send_reset_link(email: String) -> Result<(), ServerFnError> {
    forgot_password(Json(ForgotPassword { email: email.trim().to_string() })).await
}

pub async fn start(token: String) -> Result<RecoveryStart, ServerFnError> {
    recovery_start(Json(ResetToken { token })).await
}

/// A recovered master key must open the private key whose public half came with it, as at sign-in.
fn checks_out(master: &[u8; 32], public_key: Option<&Vec<u8>>, private_key: Option<&shared::EncryptedPair>) -> bool {
    match (public_key, private_key) {
        (Some(public), Some(wrapped)) => unwrap_key(master, WrapUse::PrivateKey, wrapped, false).is_some_and(|sk| &public_key_of(&sk) == public),
        _ => false,
    }
}

/// Plan §6.2: the link and the key, then a new password over the same master key. Nothing is
/// re-encrypted. When the key opens a generation a compromise rotation retired, that generation
/// comes back as the current one. Returns the signed-in account and its master key.
pub async fn recover_with_key(
    token: String,
    start: &RecoveryStart,
    key_text: &str,
    new_password: &str,
) -> Result<(Account, [u8; 32]), ServerFnError> {
    let key = recovery_key::parse(key_text).ok_or_else(|| ServerFnError::new(errors::RECOVERY_KEY_INVALID))?;
    let derived = recovery_key::derive(&key, start.account_id);
    let wrap = recovery_wrap(Json(RecoveryWrapRequest { token: token.clone(), recovery_auth: derived.auth.to_vec() })).await?;
    let master = open_recovery(&derived.rk_key, start.account_id, &wrap.recovery_wrap)
        .filter(|m| checks_out(m, wrap.public_key.as_ref(), wrap.private_key.as_ref()))
        .ok_or_else(|| ServerFnError::new(errors::RECOVERY_KEY_INVALID))?;

    let new_password = nfc(new_password);
    let new_kdf_salt = generate_kdf_salt();
    let pw_key = account_key(&new_password, new_kdf_salt.to_vec()).await?;
    let new_proof = login_proof(AUTH_VERSION_CURRENT, &new_password, start.login_salt.clone()).await?;
    let epoch = if wrap.generation.is_some() { start.amk_epoch + 1 } else { start.amk_epoch };
    let amk_wrap = wrap_amk(&pw_key, start.account_id, epoch, &master).ok_or_else(|| ServerFnError::new(errors::INVALID_PAYLOAD))?;

    let account = complete_recovery(Json(CompleteRecovery {
        token,
        recovery_auth: derived.auth.to_vec(),
        new_proof,
        new_kdf_salt: new_kdf_salt.to_vec(),
        amk_wrap,
        expected_amk_epoch: start.amk_epoch,
        generation: wrap.generation,
    }))
    .await?;
    crate::common::flush_session();
    Ok((account, master))
}

/// Plan §6.3: a fresh account - new master key, new hybrid keypair, the name given here - that takes
/// effect 72 hours from now unless the owner cancels it. Returns when.
pub async fn request_reset(token: String, start: &RecoveryStart, name: &str, new_password: &str) -> Result<NaiveDateTime, ServerFnError> {
    let new_password = nfc(new_password);
    let new_kdf_salt = generate_kdf_salt();
    let pw_key = account_key(&new_password, new_kdf_salt.to_vec()).await?;
    let new_proof = login_proof(AUTH_VERSION_CURRENT, &new_password, start.login_salt.clone()).await?;
    let master = generate_key();
    let (public_key, private) = generate_keypair();
    let seed = generate_kem_seed();
    let invalid = || ServerFnError::new(errors::INVALID_PAYLOAD);

    post_request_reset(Json(RequestReset {
        token,
        new_proof,
        new_kdf_salt: new_kdf_salt.to_vec(),
        amk_wrap: wrap_amk(&pw_key, start.account_id, start.amk_epoch + 1, &master).ok_or_else(invalid)?,
        display_name: encrypt_padded_json(&master, &name.trim().to_string()).map_err(ServerFnError::new)?,
        private_key: wrap_key(&master, WrapUse::PrivateKey, &private).ok_or_else(invalid)?,
        kem_private_key: wrap_kem_seed(&master, &seed).ok_or_else(invalid)?,
        keys: NewPublicKeys { public_key, kem_public_key: kem_public_key(&seed) },
    }))
    .await
}

async fn salt_of(email: &str) -> Result<Vec<u8>, ServerFnError> {
    Ok(checked(login_salt(Json(LoginSaltPayload { email: email.to_string() })).await?)?.salt)
}

/// The password in place when the reset was requested, tried composed then raw like a sign-in.
pub async fn cancel_reset_with_password(email: String, password: &str) -> Result<(), ServerFnError> {
    let salt = salt_of(&email).await?;
    let forms = password_forms(password);
    let mut last = ServerFnError::new(errors::INVALID_CREDENTIALS);
    for form in forms {
        let proof = login_proof(AUTH_VERSION_CURRENT, &form, salt.clone()).await?;
        match post_cancel_reset(Json(CancelReset { email: email.clone(), proof: Some(proof), recovery_auth: None })).await {
            Ok(()) => return Ok(()),
            Err(e) => last = e,
        }
    }
    Err(last)
}

/// Plan §6.3: the old password, if it had been in place 72 hours when the reset was requested,
/// brings the previous account back - memberships, keys and data. Its master key is opened here and
/// wrapped again at the current epoch plus one, so devices that switched to the reset never take it
/// for an older key.
pub async fn undo_reset(email: String, password: &str) -> Result<(Account, [u8; 32]), ServerFnError> {
    let salt = salt_of(&email).await?;
    for form in password_forms(password) {
        let proof = login_proof(AUTH_VERSION_CURRENT, &form, salt.clone()).await?;
        let Ok(retired) = retired_generation(Json(RetiredProof { email: email.clone(), proof: proof.clone() })).await else { continue };
        let pw_key = account_key(&form, retired.kdf_salt.clone()).await?;
        let master = match &retired.amk_wrap {
            None => pw_key,
            Some(wrap) => open_amk(&pw_key, retired.account_id, retired.amk_epoch, wrap).ok_or_else(|| ServerFnError::new(errors::ACCOUNT_KEY_CHANGED))?,
        };
        if retired.public_key.is_some() && !checks_out(&master, retired.public_key.as_ref(), retired.private_key.as_ref()) {
            return Err(ServerFnError::new(errors::ACCOUNT_KEY_CHANGED));
        }
        let epoch = retired.current_amk_epoch + 1;
        let amk_wrap = wrap_amk(&pw_key, retired.account_id, epoch, &master).ok_or_else(|| ServerFnError::new(errors::INVALID_PAYLOAD))?;
        let account = post_undo_reset(Json(UndoReset { email: email.clone(), proof, generation: retired.generation, amk_wrap })).await?;
        crate::common::flush_session();
        return Ok((account, master));
    }
    Err(ServerFnError::new(errors::INVALID_CREDENTIALS))
}

/// A device that saw `membership_epoch` move since it last synced must push its projects back: a
/// reset retired the memberships, and an absent row no longer means "left elsewhere".
pub fn memberships_reset(state: &crate::common::LocalStorageState, account: &Account) -> bool {
    state.membership_epochs.get(&account.id).copied().unwrap_or(0) != account.membership_epoch
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_recovered_key_must_open_the_served_private_key() {
        let (master, sk) = (generate_key(), generate_key());
        let wrapped = wrap_key(&master, WrapUse::PrivateKey, &sk);
        assert!(checks_out(&master, Some(&public_key_of(&sk)), wrapped.as_ref()));
        assert!(!checks_out(&generate_key(), Some(&public_key_of(&sk)), wrapped.as_ref()));
        assert!(!checks_out(&master, Some(&public_key_of(&generate_key())), wrapped.as_ref()));
        assert!(!checks_out(&master, None, None));
    }

    #[test]
    fn a_moved_membership_epoch_means_push_not_forget() {
        let account = Account { id: uuid::Uuid::from_u128(1), membership_epoch: 1, ..Default::default() };
        let mut state = crate::common::LocalStorageState::default();
        assert!(memberships_reset(&state, &account));
        state.membership_epochs.insert(account.id, 1);
        assert!(!memberships_reset(&state, &account));
        assert!(!memberships_reset(&crate::common::LocalStorageState::default(), &Account::default()));
    }
}
