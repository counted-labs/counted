//! The password stays here. Both entry points turn it into what the server is allowed to see —
//! a login proof — and into the account key, which never leaves the client at all.

use api::auth::auth_controller::{login, login_salt, me, register, set_keypair};
use dioxus::fullstack::Json;
use dioxus::prelude::ServerFnError;
use shared::{
    Account, EncryptedPair, Keypair, LoginPayload, LoginSaltPayload, LoginUpgrade,
    RegisterPayload, AUTH_VERSION_CURRENT, AUTH_VERSION_LEGACY, MAX_PASSWORD_LENGTH,
    MIN_PASSWORD_LENGTH,
};

use crate::common::flush_session;
use crate::crypto::{
    derive_account_key_v1, derive_login_proof, generate_kdf_salt, generate_keypair,
    wrap_key,
};

/// Length bounds moved off the server when it stopped seeing the password. The HTML `minlength`
/// on the form is a hint, not enforcement; this is the check.
pub fn password_error_key(password: &str) -> Option<&'static str> {
    if password.len() < MIN_PASSWORD_LENGTH {
        return Some("error-password-too-short");
    }
    if password.len() > MAX_PASSWORD_LENGTH {
        return Some("error-invalid-password");
    }
    None
}

/// Asks for the salt, derives the proof, signs in, then derives the account key from the salt
/// the account carries. A legacy account is re-enrolled in the same request: the client already
/// holds the password, so it derives the current proof alongside and the server swaps them once
/// the legacy one verifies.
///
/// `lang` is read by the caller rather than here: `i18n::current_lang()` consumes the Dioxus
/// context and panics outside a scope that holds it.
pub async fn sign_in(
    email: String,
    password: &str,
    lang: String,
) -> Result<(Account, [u8; 32]), ServerFnError> {
    let salt = login_salt(Json(LoginSaltPayload { email: email.clone() })).await?;
    let proof = derive_login_proof(salt.auth_version, password, &salt.salt).to_vec();
    let upgrade = (salt.auth_version == AUTH_VERSION_LEGACY).then(|| {
        let login_salt = generate_kdf_salt();
        LoginUpgrade {
            proof: derive_login_proof(AUTH_VERSION_CURRENT, password, &login_salt).to_vec(),
            login_salt: login_salt.to_vec(),
        }
    });
    let account = login(Json(LoginPayload { email, proof, upgrade, lang: Some(lang) })).await?;
    flush_session();
    let account_key = derive_account_key_v1(password, &account.kdf_salt);
    let account = ensure_keypair(account, &account_key).await;
    Ok((account, account_key))
}

/// Seeds the keypair of an account that predates it. The server keeps the first one it is given,
/// so the account is re-read afterwards rather than trusted: another device may have won. Any
/// failure leaves the account as it was — the next login tries again.
pub async fn ensure_keypair(account: Account, account_key: &[u8; 32]) -> Account {
    if account.public_key.is_some() {
        return account;
    }
    let Some(keypair) = new_keypair(account_key) else {
        return account;
    };
    if set_keypair(Json(keypair)).await.is_err() {
        return account;
    }
    match me().await {
        Ok(Some(fresh)) => fresh,
        _ => account,
    }
}

pub async fn sign_up(
    email: String,
    password: &str,
    account_key: &[u8; 32],
    display_name: EncryptedPair,
    kdf_salt: [u8; 16],
    had_anonymous_membership: bool,
    lang: String,
) -> Result<(), ServerFnError> {
    let login_salt = generate_kdf_salt();
    let proof = derive_login_proof(AUTH_VERSION_CURRENT, password, &login_salt).to_vec();
    register(Json(RegisterPayload {
        email,
        proof,
        login_salt: login_salt.to_vec(),
        display_name,
        kdf_salt: kdf_salt.to_vec(),
        had_anonymous_membership,
        keypair: new_keypair(account_key),
        lang: Some(lang),
    }))
    .await
}

/// A fresh X25519 keypair, private half wrapped under the account key. `None` only if wrapping
/// fails, in which case the account seeds one at its next login instead.
pub fn new_keypair(account_key: &[u8; 32]) -> Option<Keypair> {
    let (public_key, private) = generate_keypair();
    Some(Keypair { public_key, private_key: wrap_key(account_key, &private)? })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn password_bounds_are_enforced_client_side() {
        assert_eq!(password_error_key("short"), Some("error-password-too-short"));
        assert_eq!(password_error_key(&"x".repeat(129)), Some("error-invalid-password"));
        assert_eq!(password_error_key("long enough"), None);
        assert_eq!(password_error_key(&"x".repeat(128)), None);
    }

    #[test]
    fn password_error_keys_exist_in_the_fallback_locale() {
        let known = crate::i18n::locale_ids(crate::i18n::FALLBACK);
        for key in [password_error_key("s").unwrap(), password_error_key(&"x".repeat(129)).unwrap()] {
            assert!(known.contains(key), "{key} missing from en.ftl");
        }
    }
}
