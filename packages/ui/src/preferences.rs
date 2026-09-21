//! UI preferences that follow the account rather than the device.
//!
//! The server stores them as an opaque blob encrypted with the account key, exactly as it does the
//! display name, so syncing a preference never tells it anything about the account. Everything that
//! decides *what* to sync lives here; the picker and the settings page only call in.

use api::auth::auth_controller::update_preferences;
use dioxus::fullstack::Json;
use dioxus::prelude::*;
use shared::{Account, Preferences};

use crate::crypto::{decrypt_json, encrypt_json};

/// The account's stored language, or `None` when it has never saved one, the blob does not decrypt,
/// or the code is not one we ship.
pub fn account_language(account: &Account, key: &[u8; 32]) -> Option<String> {
    let pair = account.preferences.as_ref()?;
    let prefs: Preferences = decrypt_json(key, pair).ok()?;
    let code = prefs.language?;
    crate::i18n::SUPPORTED.iter().find(|(c, _)| *c == code).map(|(c, _)| c.to_string())
}

/// Encrypts and stores the language on the account. Best effort: a failure leaves the local choice
/// in place, which is the one that is actually driving the UI, so there is nothing to tell the user
/// and nothing they could do about it.
pub fn push_language(key: [u8; 32], code: String) {
    let Ok(pair) = encrypt_json(&key, &Preferences { language: Some(code) }) else {
        return;
    };
    spawn(async move {
        let _ = update_preferences(Json(pair)).await;
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;
    use shared::EncryptedPair;
    use uuid::Uuid;

    const KEY: [u8; 32] = [7u8; 32];

    fn account(preferences: Option<EncryptedPair>) -> Account {
        Account {
            id: Uuid::new_v4(),
            email: "a@b.c".to_string(),
            display_name: EncryptedPair::default(),
            created_at: NaiveDate::from_ymd_opt(2026, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            email_verified: true,
            kdf_salt: vec![0; 16],
            kdf_version: 1,
            preferences,
            payment_methods: None,
            public_key: None,
            private_key: None,
        }
    }

    #[test]
    fn a_stored_language_round_trips() {
        let pair = encrypt_json(&KEY, &Preferences { language: Some("de".to_string()) }).unwrap();

        assert_eq!(account_language(&account(Some(pair)), &KEY), Some("de".to_string()));
    }

    #[test]
    fn an_account_that_never_saved_one_has_none() {
        assert_eq!(account_language(&account(None), &KEY), None);
    }

    /// The wrong key is the everyday case, not an attack: a session restored from the cookie has no
    /// account key at all, and the display name already degrades the same way.
    #[test]
    fn an_undecryptable_blob_is_none_rather_than_a_panic() {
        let pair = encrypt_json(&KEY, &Preferences { language: Some("de".to_string()) }).unwrap();

        assert_eq!(account_language(&account(Some(pair)), &[9u8; 32]), None);
    }

    /// The blob is written by a client, so a language we no longer ship — or never did — must not
    /// reach `set_language` and be persisted onward.
    #[test]
    fn a_language_we_do_not_ship_is_rejected() {
        let pair = encrypt_json(&KEY, &Preferences { language: Some("ja".to_string()) }).unwrap();

        assert_eq!(account_language(&account(Some(pair)), &KEY), None);
    }

    /// Fields are added over time and old blobs must keep deserialising, or a preference added
    /// later would silently wipe the language of everyone who had not opened the new build.
    #[test]
    fn an_empty_blob_deserialises() {
        let pair = encrypt_json(&KEY, &serde_json::json!({})).unwrap();

        assert_eq!(account_language(&account(Some(pair)), &KEY), None);
    }
}
