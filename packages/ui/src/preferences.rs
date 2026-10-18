//! UI preferences that follow the account rather than the device.
//!
//! The server stores them as an opaque blob encrypted with the account key, exactly as it does the
//! display name, so syncing a preference never tells it anything about the account. Everything that
//! decides *what* to sync lives here; the picker and the settings page only call in.

use api::auth::auth_controller::update_preferences;
use dioxus::fullstack::Json;
use dioxus::prelude::*;
use shared::{Account, Preferences};
use std::collections::HashSet;
use uuid::Uuid;

use crate::common::Flash;
use crate::crypto::{decrypt_json, encrypt_json};
use crate::tid;

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
pub fn push_language(account: &Account, key: [u8; 32], code: String) {
    let Ok(pair) = encrypt_json(&key, &with_language(account, &key, code)) else {
        return;
    };
    spawn(async move {
        let _ = update_preferences(Json(pair)).await;
    });
}

/// A blob that does not decrypt is replaced, as before: nothing in it can be kept.
fn stored(account: &Account, key: &[u8; 32]) -> Preferences {
    account
        .preferences
        .as_ref()
        .and_then(|pair| decrypt_json::<Preferences>(key, pair).ok())
        .unwrap_or_default()
}

fn with_language(account: &Account, key: &[u8; 32], code: String) -> Preferences {
    Preferences { language: Some(code), ..stored(account, key) }
}

/// The groups switched off "Compter dans mes dépenses" (designs R4, R5).
pub fn uncounted_projects(account: &Account, key: &[u8; 32]) -> HashSet<Uuid> {
    stored(account, key).uncounted_projects.into_iter().collect()
}

fn with_counted(account: &Account, key: &[u8; 32], project_id: Uuid, counted: bool) -> Preferences {
    let mut prefs = stored(account, key);
    prefs.uncounted_projects.retain(|id| *id != project_id);
    if !counted {
        prefs.uncounted_projects.push(project_id);
    }
    prefs
}

/// Applied to the signed-in account at once, so every page reading it follows the switch, and put
/// back if the server refuses it.
pub fn push_counted(
    mut auth: Signal<Option<Account>>,
    key: [u8; 32],
    project_id: Uuid,
    counted: bool,
    mut flash: Signal<Option<Flash>>,
) {
    let Some(account) = auth.peek().clone() else { return };
    let Ok(pair) = encrypt_json(&key, &with_counted(&account, &key, project_id, counted)) else {
        return;
    };
    let before = account.preferences.clone();
    if let Some(a) = auth.write().as_mut() {
        a.preferences = Some(pair.clone());
    }
    spawn(async move {
        if update_preferences(Json(pair)).await.is_err() {
            if let Some(a) = auth.write().as_mut() {
                a.preferences = before;
            }
            flash.set(Some(Flash::err(tid!("counted-save-failed"))));
        }
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
        let pair = encrypt_json(&KEY, &Preferences { language: Some("de".to_string()), ..Default::default() }).unwrap();

        assert_eq!(account_language(&account(Some(pair)), &KEY), Some("de".to_string()));
    }

    #[test]
    fn an_account_that_never_saved_one_has_none() {
        assert_eq!(account_language(&account(None), &KEY), None);
    }

    /// The wrong key is an ordinary case, not an attack: a device can hold a session without the
    /// account key (local store cleared), and the display name already degrades the same way.
    #[test]
    fn an_undecryptable_blob_is_none_rather_than_a_panic() {
        let pair = encrypt_json(&KEY, &Preferences { language: Some("de".to_string()), ..Default::default() }).unwrap();

        assert_eq!(account_language(&account(Some(pair)), &[9u8; 32]), None);
    }

    /// The blob is written by a client, so a language we no longer ship - or never did - must not
    /// reach `set_language` and be persisted onward.
    #[test]
    fn a_language_we_do_not_ship_is_rejected() {
        let pair = encrypt_json(&KEY, &Preferences { language: Some("ja".to_string()), ..Default::default() }).unwrap();

        assert_eq!(account_language(&account(Some(pair)), &KEY), None);
    }

    #[test]
    fn changing_the_language_keeps_preferences_from_newer_builds() {
        let stored: Preferences = serde_json::from_str(r#"{"language":"de","theme":"dark"}"#).unwrap();
        let pair = encrypt_json(&KEY, &stored).unwrap();

        let prefs = with_language(&account(Some(pair)), &KEY, "fr".to_string());

        assert_eq!(serde_json::to_string(&prefs).unwrap(), r#"{"language":"fr","theme":"dark"}"#);
        assert_eq!(with_language(&account(None), &KEY, "fr".to_string()).language.as_deref(), Some("fr"));
    }

    #[test]
    fn switching_a_group_off_and_on_keeps_the_rest() {
        let (a, b) = (Uuid::new_v4(), Uuid::new_v4());
        let stored: Preferences = serde_json::from_str(r#"{"language":"de","theme":"dark"}"#).unwrap();
        let pair = encrypt_json(&KEY, &stored).unwrap();

        let off = with_counted(&account(Some(pair)), &KEY, a, false);
        assert_eq!(off.uncounted_projects, vec![a]);
        assert_eq!(off.language.as_deref(), Some("de"));
        assert!(off.extra.contains_key("theme"));

        let pair = encrypt_json(&KEY, &with_counted(&account(Some(encrypt_json(&KEY, &off).unwrap())), &KEY, b, false)).unwrap();
        assert_eq!(uncounted_projects(&account(Some(pair.clone())), &KEY), HashSet::from([a, b]));

        let on = with_counted(&account(Some(pair)), &KEY, a, true);
        assert_eq!(on.uncounted_projects, vec![b]);
    }

    /// Every group counts until switched off, so an account that never touched it writes nothing new.
    #[test]
    fn no_switched_off_group_writes_no_field() {
        let prefs = with_language(&account(None), &KEY, "fr".to_string());
        assert_eq!(serde_json::to_string(&prefs).unwrap(), r#"{"language":"fr"}"#);
        assert!(uncounted_projects(&account(None), &KEY).is_empty());
    }

    /// Fields are added over time and old blobs must keep deserialising, or a preference added
    /// later would silently wipe the language of everyone who had not opened the new build.
    #[test]
    fn an_empty_blob_deserialises() {
        let pair = encrypt_json(&KEY, &serde_json::json!({})).unwrap();

        assert_eq!(account_language(&account(Some(pair)), &KEY), None);
    }
}
