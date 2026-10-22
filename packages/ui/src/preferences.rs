//! UI preferences that follow the account rather than the device.
//!
//! The server stores them as an opaque blob encrypted with the account key, exactly as it does the
//! display name, so syncing a preference never tells it anything about the account. Everything that
//! decides *what* to sync lives here; the picker and the settings page only call in.

use api::auth::auth_controller::{me, update_preferences};
use dioxus::fullstack::Json;
use dioxus::prelude::*;
use shared::{Account, Preferences, ProjectPayload, UpdatePreferences, UserPayload};
use std::collections::HashSet;
use uuid::Uuid;

use crate::common::{is_preferences_stale_error, key_of, update_ls, Flash, LocalStorageState};
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
pub fn push_language(mut auth: Signal<Option<Account>>, account: &Account, key: [u8; 32], code: String) {
    let account = account.clone();
    spawn(async move {
        if let Ok(written) = save(account, key, move |prefs| with_language(prefs, code.clone())).await {
            auth.set(Some(written));
        }
    });
}

/// Writes `change` over the stored blob, on condition that it is still the stored one: an
/// unconditional write let a device left open revert what another had switched since (audit
/// 2026-10-19 L3). When another device did write in between, the account is read again and the
/// change applied to what it holds now, once. Returns the account as written.
async fn save(
    account: Account,
    key: [u8; 32],
    change: impl Fn(Preferences) -> Preferences,
) -> Result<Account, ServerFnError> {
    match write(&account, &key, &change).await {
        Err(e) if is_preferences_stale_error(&e) => {
            let fresh = me().await?.ok_or_else(|| ServerFnError::new(shared::errors::UNAUTHENTICATED))?;
            write(&fresh, &key, &change).await
        }
        other => other,
    }
}

async fn write(
    account: &Account,
    key: &[u8; 32],
    change: &impl Fn(Preferences) -> Preferences,
) -> Result<Account, ServerFnError> {
    let pair = encrypt_json(key, &change(stored(account, key))).map_err(ServerFnError::new)?;
    let expected_iv = account.preferences.as_ref().map(|p| p.iv.clone());
    update_preferences(Json(UpdatePreferences { preferences: pair.clone(), expected_iv })).await?;
    Ok(Account { preferences: Some(pair), ..account.clone() })
}

/// A blob that does not decrypt is replaced, as before: nothing in it can be kept.
fn stored(account: &Account, key: &[u8; 32]) -> Preferences {
    account
        .preferences
        .as_ref()
        .and_then(|pair| decrypt_json::<Preferences>(key, pair).ok())
        .unwrap_or_default()
}


fn with_language(prefs: Preferences, code: String) -> Preferences {
    Preferences { language: Some(code), ..prefs }
}

pub fn personal_project_id(account: &Account, key: &[u8; 32]) -> Option<Uuid> {
    stored(account, key).personal_project
}

/// The one project builds before 3.2.6 made as "Mes dépenses" and that this device can still tell
/// apart: its payload says personal and its only participant left is this device's identity in
/// it. Zero or several such projects adopt nothing - a co-member can mark a shared project
/// personal, but cannot make it a project of one.
pub fn adoptable_personal_project(state: &LocalStorageState) -> Option<Uuid> {
    let mut found = state.projects.iter().filter(|p| {
        let (Some(key), Some(me)) = (key_of(p), p.user_id) else { return false };
        let personal = p
            .cached_project
            .as_ref()
            .and_then(|dto| decrypt_json::<ProjectPayload>(&key, &dto.payload).ok())
            .is_some_and(|payload| payload.is_personal());
        let present: Vec<i32> = p
            .cached_users
            .iter()
            .flatten()
            .filter(|u| decrypt_json::<UserPayload>(&key, &u.payload).is_ok_and(|up| !up.removed))
            .map(|u| u.id)
            .collect();
        personal && present == [me]
    });
    let first = found.next()?;
    found.next().is_none().then_some(first.project_id)
}

/// Brings this device's copy in line with the account once `me()` answers: the account's choice
/// when it has one, otherwise the one project adoption allows, which is then written to the
/// account. Returns the account as it now stands.
pub async fn sync_personal_project(account: Account, key: [u8; 32], ls_ctx: Signal<LocalStorageState>) -> Account {
    let chosen = personal_project_id(&account, &key);
    let adoptable = if chosen.is_none() { adoptable_personal_project(&ls_ctx.peek()) } else { None };
    let (account, id) = match (chosen, adoptable) {
        (Some(id), _) => (account, Some(id)),
        (None, Some(id)) => match remember_personal_project(account.clone(), key, id).await {
            Ok(written) => (written, Some(id)),
            Err(_) => (account, Some(id)),
        },
        (None, None) => (account, None),
    };
    if ls_ctx.peek().personal_project != id {
        update_ls(ls_ctx, |state| state.personal_project = id);
    }
    account
}

pub async fn remember_personal_project(account: Account, key: [u8; 32], id: Uuid) -> Result<Account, ServerFnError> {
    save(account, key, move |prefs| Preferences { personal_project: Some(id), ..prefs }).await
}

/// The groups switched off "Compter dans mes dépenses" (designs R4, R5).
pub fn uncounted_projects(account: &Account, key: &[u8; 32]) -> HashSet<Uuid> {
    stored(account, key).uncounted_projects.into_iter().collect()
}

fn with_counted(mut prefs: Preferences, project_id: Uuid, counted: bool) -> Preferences {
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
    let Ok(pair) = encrypt_json(&key, &with_counted(stored(&account, &key), project_id, counted)) else {
        return;
    };
    if let Some(a) = auth.write().as_mut() {
        a.preferences = Some(pair);
    }
    spawn(async move {
        match save(account.clone(), key, move |prefs| with_counted(prefs, project_id, counted)).await {
            Ok(written) => auth.set(Some(written)),
            Err(_) => {
                auth.set(Some(account));
                flash.set(Some(Flash::err(tid!("counted-save-failed"))));
            }
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
            typed_wraps: false,
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
        let saved: Preferences = serde_json::from_str(r#"{"language":"de","theme":"dark"}"#).unwrap();
        let pair = encrypt_json(&KEY, &saved).unwrap();

        let prefs = with_language(stored(&account(Some(pair)), &KEY), "fr".to_string());

        assert_eq!(serde_json::to_string(&prefs).unwrap(), r#"{"language":"fr","theme":"dark"}"#);
        assert_eq!(with_language(stored(&account(None), &KEY), "fr".to_string()).language.as_deref(), Some("fr"));
    }

    #[test]
    fn switching_a_group_off_and_on_keeps_the_rest() {
        let (a, b) = (Uuid::new_v4(), Uuid::new_v4());
        let saved: Preferences = serde_json::from_str(r#"{"language":"de","theme":"dark"}"#).unwrap();
        let pair = encrypt_json(&KEY, &saved).unwrap();

        let off = with_counted(stored(&account(Some(pair)), &KEY), a, false);
        assert_eq!(off.uncounted_projects, vec![a]);
        assert_eq!(off.language.as_deref(), Some("de"));
        assert!(off.extra.contains_key("theme"));

        let pair = encrypt_json(&KEY, &with_counted(stored(&account(Some(encrypt_json(&KEY, &off).unwrap())), &KEY), b, false)).unwrap();
        assert_eq!(uncounted_projects(&account(Some(pair.clone())), &KEY), HashSet::from([a, b]));

        let on = with_counted(stored(&account(Some(pair)), &KEY), a, true);
        assert_eq!(on.uncounted_projects, vec![b]);
    }

    /// Every group counts until switched off, so an account that never touched it writes nothing new.
    #[test]
    fn no_switched_off_group_writes_no_field() {
        let prefs = with_language(stored(&account(None), &KEY), "fr".to_string());
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

    mod adoption {
        use super::*;
        use crate::common::test_fixtures::{make_project, make_user, test_key};
        use crate::common::LocalStorageProject;
        use shared::User;

        fn cached(kind: Option<&str>, users: Vec<User>, me: i32) -> LocalStorageProject {
            let k = test_key();
            let mut project = make_project(&k, Uuid::new_v4(), "Mes dépenses", "EUR");
            if let Some(kind) = kind {
                let mut payload: ProjectPayload = decrypt_json(&k, &project.payload).unwrap();
                payload.extra.insert("kind".into(), kind.into());
                project.payload = encrypt_json(&k, &payload).unwrap();
            }
            LocalStorageProject {
                project_id: project.id,
                user_id: Some(me),
                encryption_key: Some(crate::crypto::key_to_fragment(&k)),
                cached_project: Some(project),
                cached_users: Some(users),
                ..Default::default()
            }
        }

        fn removed(id: i32) -> User {
            User {
                id,
                payload: encrypt_json(&test_key(), &UserPayload { name: "Bob".into(), removed: true }).unwrap(),
                ..Default::default()
            }
        }

        fn adopt(projects: Vec<LocalStorageProject>) -> Option<Uuid> {
            adoptable_personal_project(&LocalStorageState { projects, ..Default::default() })
        }

        #[test]
        fn the_one_personal_project_of_one_is_adopted() {
            let k = test_key();
            let mine = cached(Some("personal"), vec![make_user(&k, 1, "Me"), removed(2)], 1);
            let trip = cached(None, vec![make_user(&k, 1, "Me")], 1);

            assert_eq!(adopt(vec![trip, mine.clone()]), Some(mine.project_id));
        }

        /// Audit 2026-10-19 M3: a co-member can write `kind: personal` into a shared project, but
        /// cannot make it a project of one.
        #[test]
        fn a_shared_project_marked_personal_is_not_adopted() {
            let k = test_key();
            assert_eq!(adopt(vec![cached(Some("personal"), vec![make_user(&k, 1, "Me"), make_user(&k, 2, "Eve")], 1)]), None);
            assert_eq!(adopt(vec![cached(Some("personal"), vec![make_user(&k, 2, "Eve")], 1)]), None);
        }

        #[test]
        fn several_candidates_adopt_nothing() {
            let k = test_key();
            let one = cached(Some("personal"), vec![make_user(&k, 1, "Me")], 1);
            let two = cached(Some("personal"), vec![make_user(&k, 1, "Me")], 1);

            assert_eq!(adopt(vec![one.clone()]), Some(one.project_id));
            assert_eq!(adopt(vec![one, two]), None);
        }

        #[test]
        fn the_accounts_choice_reads_back() {
            let id = Uuid::new_v4();
            let pair = encrypt_json(&KEY, &Preferences { personal_project: Some(id), ..Default::default() }).unwrap();

            assert_eq!(personal_project_id(&account(Some(pair)), &KEY), Some(id));
            assert_eq!(personal_project_id(&account(None), &KEY), None);
        }
    }
}
