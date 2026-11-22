//! Shown badges (docs/plans/gamification.md §4): one at a time, none by default. The server signs a
//! copy per membership; this device seals each under its project key and publishes them; members
//! check the server's two signatures before showing anything.

use api::distinctions::distinctions_controller::{distinction_keys, publish_distinctions, sign_distinctions};
use shared::time::Date;
use base64::{engine::general_purpose::STANDARD as B64, Engine};
use dioxus::fullstack::{Json, ServerFnError};
use ed25519_dalek::Verifier;
use ml_dsa::MlDsa65;
use shared::{
    distinction_message, Account, Distinction, DistinctionCopy, DistinctionKeys, Preferences, PublishDistinctions,
    SignDistinctions, User, DISTINCTION_CONTEXT, PREMIERE_HEURE_BEFORE, SHOWN_ANNIVERSAIRE, SHOWN_PREMIERE_HEURE,
};
use std::cell::{Cell, RefCell};
use uuid::Uuid;

use crate::common::{key_of, LocalStorageState};
use crate::crypto::{claim_token, decrypt_json, encrypt_json};

pub const PREMIERE_HEURE: &str = "premiere_heure";
pub const ANNIVERSAIRE: &str = "anniversaire";
/// A copy this close to `not_after` is signed again on the next pass.
const RENEW_WITHIN_SECS: i64 = 30 * 86_400;

pub fn code_of(id: &str) -> Option<u16> {
    match id {
        PREMIERE_HEURE => Some(SHOWN_PREMIERE_HEURE),
        ANNIVERSAIRE => Some(SHOWN_ANNIVERSAIRE),
        _ => None,
    }
}

fn years_since(created: Date, today: Date) -> i32 {
    let mut years = today.year() - created.year();
    if (today.month(), today.day()) < (created.month(), created.day()) {
        years -= 1;
    }
    years
}

/// The shown badges this account has earned, with the years Anniversaire would carry. The server
/// checks again before it signs; this only decides what the choice offers.
pub fn eligible(account: &Account, today: Date) -> Vec<(&'static str, u8)> {
    let created = account.created_at.date();
    let mut out = vec![];
    let (y, m, d) = PREMIERE_HEURE_BEFORE;
    if Date::from_ymd_opt(y, m, d).is_some_and(|cutoff| created < cutoff) {
        out.push((PREMIERE_HEURE, 0));
    }
    let years = years_since(created, today);
    if years >= 1 {
        out.push((ANNIVERSAIRE, years.min(u8::MAX as i32) as u8));
    }
    out
}

pub fn choice(account: &Account, key: &[u8; 32]) -> Option<String> {
    account
        .preferences
        .as_ref()
        .and_then(|p| decrypt_json::<Preferences>(key, p).ok())
        .and_then(|p| p.badges.shown)
        .filter(|id| code_of(id).is_some())
}

thread_local! {
    static KEYS: RefCell<Option<Option<DistinctionKeys>>> = const { RefCell::new(None) };
    static RENEW_CHECKED: Cell<bool> = const { Cell::new(false) };
}

/// The server's public keys, asked once per session. `None` when shown badges are off.
pub async fn keys() -> Option<DistinctionKeys> {
    if let Some(known) = KEYS.with(|k| k.borrow().clone()) {
        return known;
    }
    let fetched = distinction_keys().await.ok()?;
    KEYS.with(|k| *k.borrow_mut() = Some(fetched.clone()));
    fetched
}

fn open(pair: &shared::EncryptedPair, project_key: &[u8; 32]) -> Option<Distinction> {
    let b64: String = decrypt_json(project_key, pair).ok()?;
    Distinction::from_bytes(&B64.decode(b64).ok()?)
}

/// The badge `user` shows in `project_id`, only if both of the server's signatures hold for that
/// very membership and the copy has not expired.
pub fn verified(keys: &DistinctionKeys, user: &User, project_id: Uuid, project_key: &[u8; 32], now: i64) -> Option<(&'static str, u8)> {
    let d = open(user.distinctions.as_ref()?, project_key)?;
    if d.not_after < now {
        return None;
    }
    let message = distinction_message(d.badge, d.years, d.not_after, project_id, user.id);
    let ed_key = ed25519_dalek::VerifyingKey::from_bytes(keys.ed25519.as_slice().try_into().ok()?).ok()?;
    let ed_sig = ed25519_dalek::Signature::from_slice(&d.ed25519).ok()?;
    let ml_key = ml_dsa::VerifyingKey::<MlDsa65>::decode(&keys.ml_dsa.as_slice().try_into().ok()?);
    let ml_sig = ml_dsa::Signature::<MlDsa65>::try_from(d.ml_dsa.as_slice()).ok()?;
    if ed_key.verify(&message, &ed_sig).is_err() || !ml_key.verify_with_context(&message, DISTINCTION_CONTEXT, &ml_sig) {
        return None;
    }
    let id = match d.badge {
        SHOWN_PREMIERE_HEURE => PREMIERE_HEURE,
        SHOWN_ANNIVERSAIRE => ANNIVERSAIRE,
        _ => return None,
    };
    Some((id, d.years))
}

/// Signs and seals the chosen badge for every membership this device holds a key for, in one
/// request each way; `None` takes it down everywhere.
pub async fn publish(ls: &LocalStorageState, choice: Option<&str>) -> Result<(), ServerFnError> {
    let Some(badge) = choice.and_then(code_of) else {
        return publish_distinctions(Json(PublishDistinctions { copies: vec![] })).await;
    };
    let signed = sign_distinctions(Json(SignDistinctions { badge })).await?;
    let copies = signed
        .into_iter()
        .filter_map(|s| {
            let project = ls.projects.iter().find(|p| p.project_id == s.project_id && p.user_id == Some(s.user_id))?;
            let key = key_of(project)?;
            let pair = encrypt_json(&key, &B64.encode(s.distinction.to_bytes()?)).ok()?;
            Some(DistinctionCopy { project_id: s.project_id, pair, token: Some(claim_token(&key).to_vec()) })
        })
        .collect();
    publish_distinctions(Json(PublishDistinctions { copies })).await
}

/// Whether a membership this device holds lacks a copy of `wanted` that verifies under the served
/// keys and is not near its end: a new claim, a key rotation (which clears copies), a new signing
/// seed, a choice made elsewhere, a year gone by, or the yearly renewal.
pub fn needs_publish(ls: &LocalStorageState, keys: &DistinctionKeys, wanted: (&str, u8), now: i64) -> bool {
    ls.projects.iter().filter(|p| Some(p.project_id) != ls.personal_project).any(|p| {
        let (Some(me), Some(key)) = (p.user_id, key_of(p)) else { return false };
        let Some(mine) = p.cached_users.iter().flatten().find(|u| u.id == me) else { return false };
        let lasting = mine.distinctions.as_ref().and_then(|pair| open(pair, &key)).is_some_and(|d| d.not_after - now >= RENEW_WITHIN_SECS);
        !lasting || verified(keys, mine, p.project_id, &key, now) != Some(wanted)
    })
}

/// Once per session, after the account and its key are known: republishes when a membership is
/// missing a good copy. Costs nothing while no badge is chosen.
pub fn renew_if_needed(account: &Account, key: &[u8; 32], ls: &LocalStorageState) {
    if RENEW_CHECKED.with(Cell::get) {
        return;
    }
    let Some(chosen) = choice(account, key) else { return };
    let Some(wanted) = eligible(account, crate::common::local_today()).into_iter().find(|(id, _)| *id == chosen) else { return };
    RENEW_CHECKED.with(|c| c.set(true));
    let ls = ls.clone();
    dioxus::prelude::spawn(async move {
        let Some(keys) = keys().await else { return };
        if needs_publish(&ls, &keys, wanted, shared::time::DateTime::now_utc().unix_secs()) {
            let _ = publish(&ls, Some(&chosen)).await;
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    fn account(created: &str) -> Account {
        Account { created_at: Date::parse_ymd(created).unwrap().and_hms_opt(9, 0, 0).unwrap(), ..Default::default() }
    }

    fn day(s: &str) -> Date {
        Date::parse_ymd(s).unwrap()
    }

    #[test]
    fn an_early_account_is_offered_premiere_heure_then_anniversaire_after_a_year() {
        assert_eq!(eligible(&account("2026-06-01"), day("2026-11-18")), vec![(PREMIERE_HEURE, 0)]);
        assert_eq!(eligible(&account("2026-06-01"), day("2027-06-01")), vec![(PREMIERE_HEURE, 0), (ANNIVERSAIRE, 1)]);
        assert_eq!(eligible(&account("2027-01-01"), day("2027-12-31")), vec![]);
    }

    struct Signer {
        ed25519: ed25519_dalek::SigningKey,
        ml_dsa: ml_dsa::SigningKey<MlDsa65>,
    }

    impl Signer {
        fn new(seed: u8) -> Self {
            Self {
                ed25519: ed25519_dalek::SigningKey::from_bytes(&[seed; 32]),
                ml_dsa: ml_dsa::SigningKey::<MlDsa65>::from_seed(&ml_dsa::Seed::from([seed; 32])),
            }
        }

        fn public(&self) -> DistinctionKeys {
            DistinctionKeys {
                ed25519: self.ed25519.verifying_key().to_bytes().to_vec(),
                ml_dsa: self.ml_dsa.expanded_key().verifying_key().encode().to_vec(),
            }
        }

        fn sign(&self, badge: u16, years: u8, not_after: i64, project_id: Uuid, user_id: i32) -> Distinction {
            use ed25519_dalek::Signer as _;
            let message = distinction_message(badge, years, not_after, project_id, user_id);
            Distinction {
                badge,
                years,
                not_after,
                ed25519: self.ed25519.sign(&message).to_bytes().to_vec(),
                ml_dsa: self.ml_dsa.expanded_key().sign_deterministic(&message, DISTINCTION_CONTEXT).unwrap().encode().to_vec(),
            }
        }
    }

    fn holding(copy: Distinction) -> (LocalStorageState, [u8; 32]) {
        let key = [7u8; 32];
        let project_id = Uuid::from_u128(1);
        let pair = encrypt_json(&key, &B64.encode(copy.to_bytes().unwrap())).unwrap();
        let me = User { id: 5, distinctions: Some(pair), ..Default::default() };
        let project = crate::common::LocalStorageProject {
            project_id,
            user_id: Some(5),
            encryption_key: Some(crate::crypto::key_to_fragment(&key)),
            cached_users: Some(vec![me]),
            ..Default::default()
        };
        (LocalStorageState { projects: vec![project], ..Default::default() }, key)
    }

    const NOW: i64 = 1_800_000_000;
    const YEAR: i64 = 365 * 86_400;

    #[test]
    fn a_fresh_copy_of_the_chosen_badge_needs_no_publish() {
        let signer = Signer::new(1);
        let (ls, _) = holding(signer.sign(SHOWN_PREMIERE_HEURE, 0, NOW + YEAR, Uuid::from_u128(1), 5));
        assert!(!needs_publish(&ls, &signer.public(), (PREMIERE_HEURE, 0), NOW));
    }

    #[test]
    fn a_copy_shows_only_for_the_membership_it_was_signed_for() {
        let signer = Signer::new(1);
        let (ls, key) = holding(signer.sign(SHOWN_PREMIERE_HEURE, 0, NOW + YEAR, Uuid::from_u128(1), 5));
        let mine = ls.projects[0].cached_users.as_ref().unwrap()[0].clone();
        let keys = signer.public();
        assert_eq!(verified(&keys, &mine, Uuid::from_u128(1), &key, NOW), Some((PREMIERE_HEURE, 0)));
        assert_eq!(verified(&keys, &mine, Uuid::from_u128(2), &key, NOW), None);
        assert_eq!(verified(&keys, &User { id: 6, ..mine.clone() }, Uuid::from_u128(1), &key, NOW), None);
        assert_eq!(verified(&keys, &mine, Uuid::from_u128(1), &key, NOW + 2 * YEAR), None);
    }

    #[test]
    fn a_copy_signed_under_an_earlier_seed_is_signed_again() {
        let (ls, _) = holding(Signer::new(1).sign(SHOWN_PREMIERE_HEURE, 0, NOW + YEAR, Uuid::from_u128(1), 5));
        assert!(needs_publish(&ls, &Signer::new(2).public(), (PREMIERE_HEURE, 0), NOW));
    }

    #[test]
    fn a_copy_of_another_badge_or_year_or_near_its_end_is_signed_again() {
        let signer = Signer::new(1);
        let (ls, _) = holding(signer.sign(SHOWN_ANNIVERSAIRE, 1, NOW + YEAR, Uuid::from_u128(1), 5));
        assert!(needs_publish(&ls, &signer.public(), (ANNIVERSAIRE, 2), NOW));
        assert!(needs_publish(&ls, &signer.public(), (PREMIERE_HEURE, 0), NOW));
        let (ls, _) = holding(signer.sign(SHOWN_ANNIVERSAIRE, 1, NOW + 86_400, Uuid::from_u128(1), 5));
        assert!(needs_publish(&ls, &signer.public(), (ANNIVERSAIRE, 1), NOW));
    }

    #[test]
    fn only_known_badges_have_a_code() {
        assert_eq!(code_of(PREMIERE_HEURE), Some(SHOWN_PREMIERE_HEURE));
        assert_eq!(code_of(ANNIVERSAIRE), Some(SHOWN_ANNIVERSAIRE));
        assert_eq!(code_of("touriste"), None);
    }
}
