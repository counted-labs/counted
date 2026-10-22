//! The password stays here. Both entry points turn it into what the server is allowed to see -
//! a login proof - and into the account key, which never leaves the client at all.

use api::auth::auth_controller::{
    login, login_salt, me, register, rewrap_private_key, set_keypair, verify_email, verify_email_salt,
};
use dioxus::fullstack::Json;
use dioxus::prelude::ServerFnError;
use shared::{
    errors, Account, EncryptedPair, Keypair, LoginPayload, LoginSalt, LoginSaltPayload,
    RegisterPayload, VerifyEmailPayload, VerifyEmailSaltPayload, AUTH_VERSION_CURRENT,
    LOGIN_SALT_LENGTH, MAX_PASSWORD_LENGTH, MIN_PASSWORD_LENGTH,
};

use crate::common::{flush_session, read_from_ls, write_to_ls, LocalStorageState};
use crate::crypto::{
    derive_account_key_v1, derive_login_proof, generate_kdf_salt, generate_keypair,
    key_to_fragment, open_wrap, public_key_of, wrap_key, WrapUse,
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
/// the account carries.
///
/// `lang` is read by the caller rather than here: `i18n::current_lang()` consumes the Dioxus
/// context and panics outside a scope that holds it.
pub async fn sign_in(
    email: String,
    password: &str,
    lang: String,
) -> Result<(Account, [u8; 32]), ServerFnError> {
    let salt = checked(login_salt(Json(LoginSaltPayload { email: email.clone() })).await?)?;
    let proof = login_proof(salt.auth_version, password, salt.salt).await?;
    let account = login(Json(LoginPayload { email, proof, upgrade: None, lang: Some(lang) })).await?;
    flush_session();
    let account_key = account_key_of(&account, password).await?;
    let account = ensure_keypair(account, &account_key).await;
    Ok((account, account_key))
}

/// The mailed link plus the password: the link alone no longer signs anyone in. Ends exactly
/// where `sign_in` does, with the session and the account key both on this device.
pub async fn verify_and_sign_in(
    token: String,
    password: &str,
) -> Result<(Account, [u8; 32]), ServerFnError> {
    let salt = checked(verify_email_salt(Json(VerifyEmailSaltPayload { token: token.clone() })).await?)?;
    let proof = login_proof(salt.auth_version, password, salt.salt).await?;
    let account = verify_email(Json(VerifyEmailPayload { token, proof: Some(proof) })).await?;
    flush_session();
    let account_key = account_key_of(&account, password).await?;
    let account = ensure_keypair(account, &account_key).await;
    Ok((account, account_key))
}

/// The served salt, refused unless it is the current scheme's: a v1 answer for a v2 account makes
/// the proof about five times cheaper to guess offline, and no v1 account is left (audit
/// 2026-10-19 L2). A salt that is not 16 bytes would panic Argon2, which freezes wasm (I1).
fn checked(salt: LoginSalt) -> Result<LoginSalt, ServerFnError> {
    if salt.auth_version != AUTH_VERSION_CURRENT || salt.salt.len() != LOGIN_SALT_LENGTH {
        return Err(ServerFnError::new(errors::INVALID_CREDENTIALS));
    }
    Ok(salt)
}

async fn account_key_of(account: &Account, password: &str) -> Result<[u8; 32], ServerFnError> {
    if account.kdf_salt.len() != LOGIN_SALT_LENGTH {
        return Err(ServerFnError::new(errors::INVALID_PAYLOAD));
    }
    account_key(password, account.kdf_salt.clone()).await
}

/// Keeps the account key across a reload: once the session is restored from the cookie, the
/// password is gone.
pub fn remember_account_key(account_key: &[u8; 32]) -> LocalStorageState {
    let mut state = read_from_ls();
    state.account_key = Some(key_to_fragment(account_key));
    write_to_ls(&state);
    state
}

/// Seeds the keypair of an account that predates it, or re-wraps an untyped private key once typed
/// wraps are on (audit 2026-10-19 L1). The server keeps the first keypair it is given, so the
/// account is re-read afterwards rather than trusted: another device may have won. Any failure
/// leaves the account as it was - the next login tries again.
pub async fn ensure_keypair(account: Account, account_key: &[u8; 32]) -> Account {
    let sent = match account.public_key {
        None => match new_keypair(account_key) {
            Some(keypair) => set_keypair(Json(keypair)).await,
            None => return account,
        },
        Some(_) => match typed_rewrap(&account, account_key) {
            Some(keypair) => rewrap_private_key(Json(keypair)).await,
            None => return account,
        },
    };
    if sent.is_err() {
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
    let proof = login_proof(AUTH_VERSION_CURRENT, password, login_salt.to_vec()).await?;
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

pub async fn account_key(password: &str, salt: Vec<u8>) -> Result<[u8; 32], ServerFnError> {
    let password = password.to_owned();
    off_ui_thread(move || derive_account_key_v1(&password, &salt)).await
}

async fn login_proof(auth_version: i16, password: &str, salt: Vec<u8>) -> Result<Vec<u8>, ServerFnError> {
    let password = password.to_owned();
    Ok(off_ui_thread(move || derive_login_proof(auth_version, &password, &salt)).await?.to_vec())
}

/// On both phones the Dioxus event loop is the platform UI thread (see `scan.rs`), and a 64 MiB
/// Argon2id run there freezes the app. WASM has one thread: it yields a frame instead, so the
/// spinner repaints between two runs.
async fn off_ui_thread(
    derive: impl FnOnce() -> Result<[u8; 32], String> + Send + 'static,
) -> Result<[u8; 32], ServerFnError> {
    #[cfg(target_arch = "wasm32")]
    let derived = {
        crate::common::next_paint().await;
        derive()
    };
    #[cfg(not(target_arch = "wasm32"))]
    let derived = {
        use std::sync::{Arc, Mutex};
        type Slot = Arc<Mutex<Option<Result<[u8; 32], String>>>>;
        let slot: Slot = Arc::new(Mutex::new(None));
        let out = slot.clone();
        std::thread::spawn(move || {
            let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(derive))
                .unwrap_or_else(|_| Err("argon2 panicked".to_string()));
            if let Ok(mut guard) = out.lock() {
                *guard = Some(result);
            }
        });
        loop {
            if let Some(done) = slot.lock().ok().and_then(|mut g| g.take()) {
                break done;
            }
            crate::common::sleep(KDF_POLL_MS).await;
        }
    };
    derived.map_err(ServerFnError::new)
}

#[cfg(not(target_arch = "wasm32"))]
const KDF_POLL_MS: u32 = 20;

/// A fresh X25519 keypair, private half wrapped under the account key. `None` only if wrapping
/// fails, in which case the account seeds one at its next login instead.
pub fn new_keypair(account_key: &[u8; 32]) -> Option<Keypair> {
    let (public_key, private) = generate_keypair();
    Some(Keypair { public_key, private_key: wrap_key(account_key, WrapUse::PrivateKey, &private, false)? })
}

/// The same private key wrapped typed, when typed wraps are on and the stored one is untyped. Not
/// on a device that already refuses untyped wraps for the account, which would turn a served one
/// into a trusted one, nor for a key whose public half is not the served one (M2's warning).
fn typed_rewrap(account: &Account, account_key: &[u8; 32]) -> Option<Keypair> {
    if !account.typed_wraps {
        return None;
    }
    let opened = open_wrap(account_key, WrapUse::PrivateKey, account.private_key.as_ref()?)?;
    let public_key = account.public_key.clone()?;
    if opened.typed || public_key_of(&opened.key) != public_key || !read_from_ls().raw_wraps_ok(account.id) {
        return None;
    }
    Some(Keypair { public_key, private_key: wrap_key(account_key, WrapUse::PrivateKey, &opened.key, true)? })
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Audit 2026-10-19 L2 and I1: a server answering v1, or a salt Argon2 would panic on, gets
    /// the invalid-credentials error and no proof.
    #[test]
    fn only_a_current_sixteen_byte_salt_is_used() {
        let salt = |salt: Vec<u8>, auth_version| LoginSalt { salt, auth_version };

        assert!(checked(salt(vec![1; 16], AUTH_VERSION_CURRENT)).is_ok());
        assert!(checked(salt(vec![1; 16], shared::AUTH_VERSION_LEGACY)).is_err());
        assert!(checked(salt(vec![1; 4], AUTH_VERSION_CURRENT)).is_err());
        assert!(checked(salt(vec![], AUTH_VERSION_CURRENT)).is_err());
    }

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
