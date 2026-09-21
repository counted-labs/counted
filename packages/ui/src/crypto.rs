use chacha20poly1305::{
    aead::{generic_array::GenericArray, Aead, KeyInit},
    XChaCha20Poly1305,
};
use base64::{
    engine::general_purpose::{STANDARD, URL_SAFE_NO_PAD},
    Engine as _,
};
use chrono::NaiveDateTime;
use serde::{de::DeserializeOwned, Serialize};
use shared::{
    Account, EncryptedPair, Expense, ExpensePayload, ExpenseType, Payment, PaymentMethod,
    PaymentMethods, PaymentPayload, ProjectDto, ProjectPayload, ProjectStatus, User, UserPayload,
};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub struct DecryptedProject {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub currency: String,
    pub status: ProjectStatus,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DecryptedUser {
    pub id: i32,
    pub name: String,
    pub created_at: Option<NaiveDateTime>,
    /// An account holds this participant as its identity in the project.
    pub claimed: bool,
    /// That account's display name. `None` when the claim carries no label, or when it does not
    /// decrypt — the padlock still shows, only the name is missing.
    pub claim_name: Option<String>,
    /// The payment methods that account chose to share with the project. Empty when it shares
    /// nothing, when there is no claim, or when the copy does not decrypt.
    pub payment_methods: Vec<PaymentMethod>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DecryptedExpense {
    pub id: i32,
    pub author_id: Option<i32>,
    pub project_id: Uuid,
    pub created_at: NaiveDateTime,
    pub name: String,
    pub description: Option<String>,
    /// Always in the project's currency — see `shared::ExpensePayload`. Every reader sums this and
    /// only this; the three fields below are for display and for prefilling the edit form.
    pub amount: f64,
    pub expense_type: ExpenseType,
    pub date: String,
    pub category: Option<String>,
    pub source_currency: Option<String>,
    pub source_amount: Option<f64>,
    pub rate: Option<f64>,
}

impl DecryptedExpense {
    /// The original amount, its currency and the rate it was booked at — `None` unless all three
    /// are present and the rate is usable, so a half-written payload renders as a plain
    /// project-currency expense instead of a broken conversion line.
    pub fn conversion(&self) -> Option<(f64, &str, f64)> {
        let currency = self.source_currency.as_deref()?;
        let amount = self.source_amount?;
        let rate = self.rate?;
        (amount.is_finite() && shared::is_valid_rate(rate)).then_some((amount, currency, rate))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DecryptedPayment {
    pub id: i32,
    pub expense_id: i32,
    pub user_id: i32,
    pub is_debt: bool,
    pub amount: f64,
    pub created_at: NaiveDateTime,
}

pub fn generate_key() -> [u8; 32] {
    let mut key = [0u8; 32];
    getrandom::getrandom(&mut key).expect("getrandom failed");
    key
}

pub fn key_to_fragment(key: &[u8; 32]) -> String {
    URL_SAFE_NO_PAD.encode(key)
}

pub fn key_from_fragment(fragment: &str) -> Result<[u8; 32], String> {
    let bytes = URL_SAFE_NO_PAD.decode(fragment).map_err(|e| format!("base64: {e}"))?;
    bytes.try_into().map_err(|_| "key must be 32 bytes".to_string())
}

pub fn encrypt(key: &[u8; 32], plaintext: &str) -> Result<(String, String), String> {
    let mut iv = [0u8; 24];
    getrandom::getrandom(&mut iv).map_err(|e| format!("getrandom: {e}"))?;
    let ct = XChaCha20Poly1305::new_from_slice(key)
        .expect("key is 32 bytes")
        .encrypt(GenericArray::from_slice(&iv), plaintext.as_bytes())
        .map_err(|e| format!("encrypt: {e}"))?;
    Ok((STANDARD.encode(&ct), STANDARD.encode(iv)))
}

// The expenses-tab guard asserts on this count, not wall-clock — the stall it protects against was
// a call-count bug. Thread-local: `cargo test` runs tests concurrently.
#[cfg(test)]
thread_local! {
    pub(crate) static DECRYPT_COUNT: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

/// Real decryptions performed by `f`, on this thread.
#[cfg(test)]
pub(crate) fn count_decrypts<T>(f: impl FnOnce() -> T) -> (T, usize) {
    DECRYPT_COUNT.with(|c| c.set(0));
    let out = f();
    (out, DECRYPT_COUNT.with(|c| c.get()))
}

pub fn decrypt(key: &[u8; 32], ct_b64: &str, iv_b64: &str) -> Result<String, String> {
    #[cfg(test)]
    DECRYPT_COUNT.with(|c| c.set(c.get() + 1));

    let ct = STANDARD.decode(ct_b64).map_err(|e| format!("ct base64: {e}"))?;
    let iv_bytes = STANDARD.decode(iv_b64).map_err(|e| format!("iv base64: {e}"))?;
    if iv_bytes.len() != 24 {
        return Err(format!("invalid IV length: {}", iv_bytes.len()));
    }
    let plain = XChaCha20Poly1305::new_from_slice(key)
        .expect("key is 32 bytes")
        .decrypt(GenericArray::from_slice(&iv_bytes), ct.as_ref())
        .map_err(|e| format!("decrypt: {e}"))?;
    String::from_utf8(plain).map_err(|e| format!("utf8: {e}"))
}

pub fn encrypt_pair(key: &[u8; 32], plaintext: &str) -> Result<EncryptedPair, String> {
    let (ct, iv) = encrypt(key, plaintext)?;
    Ok(EncryptedPair { ct, iv })
}

pub fn decrypt_pair(key: &[u8; 32], pair: &EncryptedPair) -> Result<String, String> {
    decrypt(key, &pair.ct, &pair.iv)
}

pub fn encrypt_json<T: Serialize>(key: &[u8; 32], value: &T) -> Result<EncryptedPair, String> {
    let json = serde_json::to_string(value).map_err(|e| e.to_string())?;
    encrypt_pair(key, &json)
}

pub fn decrypt_json<T: DeserializeOwned>(key: &[u8; 32], pair: &EncryptedPair) -> Result<T, String> {
    let json = decrypt(key, &pair.ct, &pair.iv)?;
    serde_json::from_str(&json).map_err(|e| e.to_string())
}

/// Wraps a project key for escrow: the base64url fragment, encrypted under the account key.
///
/// The fragment rather than the raw bytes, so the escrowed value is exactly what a share link
/// carries and what `LocalStorageProject.encryption_key` holds — one representation, no conversion
/// to get wrong. See `docs/e2ee.md`, "Key escrow".
pub fn wrap_project_key(account_key: &[u8; 32], project_key: &[u8; 32]) -> Option<EncryptedPair> {
    encrypt_pair(account_key, &key_to_fragment(project_key)).ok()
}

/// Inverse of [`wrap_project_key`]. `None` on any failure — a wrong account key, a tampered blob,
/// or a fragment that no longer decodes. Never fatal: the project simply stays locked, which is a
/// state the UI already renders.
pub fn unwrap_project_key(account_key: &[u8; 32], wrapped: &EncryptedPair) -> Option<[u8; 32]> {
    decrypt_pair(account_key, wrapped).ok().and_then(|frag| key_from_fragment(&frag).ok())
}

/// This account's `display_name`, re-encrypted under the **project** key, so the other members can
/// see who claimed an identity.
///
/// The account key protects `display_name` against everyone, the server included — which also means
/// no other member can read it. Handing the name to the project is therefore a deliberate second
/// copy under a second key, made by the only party that can: the account itself, client-side. The
/// server still reads neither.
///
/// `None` when the name does not decrypt (wrong account key) or does not re-encrypt: the claim then
/// travels without a label, the padlock shows and the name does not.
///
/// There is no refresh path because there is nothing to refresh: no endpoint updates
/// `display_name` — it is written at registration and never changes.
pub fn claim_label(
    account: &Account,
    account_key: &[u8; 32],
    project_key: &[u8; 32],
) -> Option<EncryptedPair> {
    let name: String = decrypt_json(account_key, &account.display_name).ok()?;
    encrypt_json(project_key, &name).ok()
}

/// The account's X25519 keypair: what lets another account encrypt *to* this one. The account
/// key is symmetric and exists only on the owner's devices, so before this nothing could be
/// addressed to an account — see `docs/plans/friends.md`.
pub fn generate_keypair() -> (Vec<u8>, [u8; 32]) {
    let secret = crypto_box::SecretKey::from_bytes(generate_key());
    (secret.public_key().to_bytes().to_vec(), secret.to_bytes())
}

/// The private half, encrypted under the account key exactly like an escrowed project key.
pub fn wrap_private_key(account_key: &[u8; 32], private_key: &[u8; 32]) -> Option<EncryptedPair> {
    wrap_project_key(account_key, private_key)
}

pub fn unwrap_private_key(account_key: &[u8; 32], wrapped: &EncryptedPair) -> Option<[u8; 32]> {
    unwrap_project_key(account_key, wrapped)
}

fn chacha_box(my_private: &[u8; 32], their_public: &[u8]) -> Option<crypto_box::ChaChaBox> {
    let public = crypto_box::PublicKey::from_slice(their_public).ok()?;
    Some(crypto_box::ChaChaBox::new(&public, &crypto_box::SecretKey::from_bytes(*my_private)))
}

/// A project key boxed for one friend: X25519 between the two accounts, then the same
/// XChaCha20-Poly1305 as everything else, so the result rides in an ordinary `EncryptedPair`. The
/// plaintext is the key's fragment form, as for escrow. Authenticated, not sealed — the recipient
/// opens it with the sender's public key, which is how they know who sent it.
pub fn box_project_key(
    my_private: &[u8; 32],
    their_public: &[u8],
    project_key: &[u8; 32],
) -> Option<EncryptedPair> {
    let mut iv = [0u8; 24];
    getrandom::getrandom(&mut iv).ok()?;
    let ct = chacha_box(my_private, their_public)?
        .encrypt(GenericArray::from_slice(&iv), key_to_fragment(project_key).as_bytes())
        .ok()?;
    Some(EncryptedPair { ct: STANDARD.encode(ct), iv: STANDARD.encode(iv) })
}

pub fn unbox_project_key(
    my_private: &[u8; 32],
    their_public: &[u8],
    boxed: &EncryptedPair,
) -> Option<[u8; 32]> {
    let ct = STANDARD.decode(&boxed.ct).ok()?;
    let iv = STANDARD.decode(&boxed.iv).ok()?;
    if iv.len() != 24 {
        return None;
    }
    let plain = chacha_box(my_private, their_public)?
        .decrypt(GenericArray::from_slice(&iv), ct.as_ref())
        .ok()?;
    key_from_fragment(std::str::from_utf8(&plain).ok()?).ok()
}

/// Eight characters two friends can read to each other to check the server handed out the right
/// key: SHA-256 of the public key, first 40 bits, base32.
pub fn key_fingerprint(public_key: &[u8]) -> String {
    use sha2::{Digest, Sha256};
    const ALPHABET: &[u8; 32] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ234567";
    let digest = Sha256::digest(public_key);
    let mut bits: u64 = 0;
    for byte in &digest[..5] {
        bits = (bits << 8) | u64::from(*byte);
    }
    (0..8).rev().map(|i| ALPHABET[((bits >> (i * 5)) & 31) as usize] as char).collect()
}

/// Current KDF (v1): Argon2id with a per-account random salt.
/// OWASP "preferred" interactive params: m = 64 MiB, t = 3, p = 1, 32-byte output.
pub fn derive_account_key_v1(password: &str, salt: &[u8]) -> [u8; 32] {
    use argon2::{Algorithm, Argon2, Params, Version};
    let params = Params::new(65_536, 3, 1, Some(32)).expect("valid argon2 params");
    let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);
    let mut out = [0u8; 32];
    argon2
        .hash_password_into(password.as_bytes(), salt, &mut out)
        .expect("argon2 hash");
    out
}

pub fn generate_kdf_salt() -> [u8; 16] {
    let mut s = [0u8; 16];
    getrandom::getrandom(&mut s).expect("getrandom failed");
    s
}

/// Proof of holding a project key, for identity claims. A hash of the key behind a fixed label:
/// the server stores `claim_verifier(key)` on the project and compares what a claim presents
/// against it, so it can tell a key holder from a UUID holder without ever seeing the key.
/// Fixed-length secret after a fixed prefix, so plain SHA-256 needs no MAC construction.
pub fn claim_token(project_key: &[u8; 32]) -> [u8; 32] {
    use sha2::{Digest, Sha256};
    Sha256::new().chain_update(b"counted-claim-v1").chain_update(project_key).finalize().into()
}

/// What a freshly created project is born with: `SHA-256(claim_token)`.
pub fn claim_verifier(project_key: &[u8; 32]) -> [u8; 32] {
    use sha2::{Digest, Sha256};
    Sha256::digest(claim_token(project_key)).into()
}

/// What the server is sent instead of the password. Two derivations, selected by the account's
/// `auth_version` (see `shared::AUTH_VERSION_*`):
///
/// - legacy: exactly the Argon2id the server used to run on the plaintext (`Argon2::default()`:
///   m = 19 MiB, t = 2, p = 1), so the stored hash verifies it without re-enrolment;
/// - current: the account-key parameters, keyed with a fixed secret so the proof is a different
///   function of the password even under an equal salt — the server must never hold anything the
///   account key can be derived from.
///
/// Both are bound to the salt the server handed out for this address, so a stolen proof is
/// useless against any other account.
pub fn derive_login_proof(auth_version: i16, password: &str, salt: &[u8]) -> [u8; 32] {
    use argon2::{Algorithm, Argon2, Params, Version};
    let argon2 = if auth_version == shared::AUTH_VERSION_LEGACY {
        Argon2::new(Algorithm::Argon2id, Version::V0x13, Params::default())
    } else {
        let params = Params::new(65_536, 3, 1, Some(32)).expect("valid argon2 params");
        Argon2::new_with_secret(b"counted-login-v2", Algorithm::Argon2id, Version::V0x13, params)
            .expect("valid argon2 secret")
    };
    let mut out = [0u8; 32];
    argon2.hash_password_into(password.as_bytes(), salt, &mut out).expect("argon2 hash");
    out
}

pub fn decrypt_project(key: &[u8; 32], p: &ProjectDto) -> Result<DecryptedProject, String> {
    let pp: ProjectPayload = decrypt_json(key, &p.payload)?;
    Ok(DecryptedProject {
        id: p.id,
        name: pp.name,
        description: pp.description,
        currency: pp.currency,
        status: p.status.clone(),
        created_at: p.created_at,
    })
}

pub fn decrypt_user(key: &[u8; 32], u: &User) -> Result<DecryptedUser, String> {
    let up: UserPayload = decrypt_json(key, &u.payload)?;
    Ok(DecryptedUser {
        id: u.id,
        name: up.name,
        created_at: u.created_at,
        claimed: u.claimed,
        claim_name: u.claim_label.as_ref().and_then(|p| decrypt_json::<String>(key, p).ok()),
        payment_methods: u
            .payment_methods
            .as_ref()
            .and_then(|p| decrypt_json::<PaymentMethods>(key, p).ok())
            .map(|p| p.methods)
            .unwrap_or_default(),
    })
}

/// Empty when the payload cannot be decrypted: call sites render it directly, so a failure
/// degrades to a blank name rather than an error.
pub fn user_name(key: &[u8; 32], u: &User) -> String {
    decrypt_json::<UserPayload>(key, &u.payload).map(|p| p.name).unwrap_or_default()
}

/// Same, for the call sites that only hold an optional key (the key context before it resolves).
pub fn user_name_opt(key: Option<&[u8; 32]>, u: &User) -> String {
    key.map(|k| user_name(k, u)).unwrap_or_default()
}

/// Positionally matches `users`. Without a key every name is blank, but the length is kept so
/// callers can still zip against `users`.
pub fn user_names(key: Option<&[u8; 32]>, users: &[User]) -> Vec<String> {
    users.iter().map(|u| user_name_opt(key, u)).collect()
}

pub fn project_name(key: &[u8; 32], p: &ProjectDto) -> String {
    decrypt_json::<ProjectPayload>(key, &p.payload).map(|pp| pp.name).unwrap_or_default()
}

pub fn project_currency(key: &[u8; 32], p: &ProjectDto) -> String {
    decrypt_json::<ProjectPayload>(key, &p.payload).map(|pp| pp.currency).unwrap_or_default()
}

pub fn decrypt_expense(key: &[u8; 32], e: &Expense) -> Result<DecryptedExpense, String> {
    let ep: ExpensePayload = decrypt_json(key, &e.payload)?;
    let expense_type = match ep.expense_type.as_str() {
        "expense" => ExpenseType::Expense,
        "transfer" => ExpenseType::Transfer,
        "gain" => ExpenseType::Gain,
        other => return Err(format!("invalid expense type: {other}")),
    };
    Ok(DecryptedExpense {
        id: e.id,
        author_id: e.author_id,
        project_id: e.project_id,
        created_at: e.created_at,
        name: ep.name,
        description: ep.description,
        amount: ep.amount,
        expense_type,
        date: ep.date,
        category: ep.category,
        source_currency: ep.source_currency,
        source_amount: ep.source_amount,
        rate: ep.rate,
    })
}

pub fn decrypt_payment(key: &[u8; 32], p: &Payment) -> Result<DecryptedPayment, String> {
    let pp: PaymentPayload = decrypt_json(key, &p.payload)?;
    Ok(DecryptedPayment {
        id: p.id,
        expense_id: p.expense_id,
        user_id: p.user_id,
        is_debt: pp.is_debt,
        amount: pp.amount,
        created_at: p.created_at,
    })
}

/// True when an expense's payments no longer add up on either side.
///
/// The server only ever sees ciphertext, so it cannot enforce this and a tampered or outdated
/// client's expense lands in the database unchallenged. Honest clients detect it here, on read,
/// rather than computing balances that can never settle.
///
/// `payments` are that expense's decrypted payments. Empty means the key could not open them — a
/// missing key, not an inconsistency, so it reports false.
pub fn payments_are_inconsistent(total: f64, payments: &[DecryptedPayment]) -> bool {
    if payments.is_empty() {
        return false;
    }
    let side = |is_debt: bool| {
        payments.iter().filter(move |p| p.is_debt == is_debt).map(|p| p.amount)
    };
    !shared::sums_to_total(total, side(false)) || !shared::sums_to_total(total, side(true))
}

#[cfg(test)]
mod tests {
    use super::*;
    use base64::engine::general_purpose::{STANDARD, URL_SAFE_NO_PAD};

    use crate::common::test_fixtures::{make_expense, make_payment, make_user, test_key};

    fn enc(key: &[u8; 32], s: &str) -> EncryptedPair {
        encrypt_pair(key, s).unwrap()
    }

    fn dp(expense_id: i32, is_debt: bool, amount: f64) -> DecryptedPayment {
        DecryptedPayment {
            id: 0,
            expense_id,
            user_id: 0,
            is_debt,
            amount,
            created_at: NaiveDateTime::default(),
        }
    }

    fn account_with_name(account_key: &[u8; 32], name: &str) -> Account {
        Account {
            id: Uuid::nil(),
            email: "jonathan@example.com".into(),
            display_name: encrypt_json(account_key, &name.to_string()).unwrap(),
            created_at: NaiveDateTime::default(),
            email_verified: true,
            kdf_salt: vec![0u8; 16],
            kdf_version: 1,
            preferences: None,
            payment_methods: None,
            public_key: None,
            private_key: None,
        }
    }

    /// The whole point of the claim label: a name only the account could read becomes a name every
    /// member of the project can read, and nobody else — the server included.
    #[test]
    fn a_claim_label_crosses_from_the_account_key_to_the_project_key() {
        let account_key = generate_key();
        let project_key = generate_key();
        let account = account_with_name(&account_key, "Jonathan");

        let label = claim_label(&account, &account_key, &project_key).unwrap();

        assert_eq!(decrypt_json::<String>(&project_key, &label).unwrap(), "Jonathan");
        // The account key must not open it — it is a genuinely separate copy, not a re-wrap.
        assert!(decrypt_json::<String>(&account_key, &label).is_err());
    }

    #[test]
    fn a_claim_label_is_unreadable_with_another_project_key() {
        let account_key = generate_key();
        let project_key = generate_key();
        let account = account_with_name(&account_key, "Jonathan");

        let label = claim_label(&account, &account_key, &project_key).unwrap();

        assert!(decrypt_json::<String>(&generate_key(), &label).is_err());
    }

    /// A device whose account key is wrong (or restored from a different password) must degrade to
    /// "no name", never to a garbled one or a panic.
    #[test]
    fn a_wrong_account_key_produces_no_label() {
        let account = account_with_name(&generate_key(), "Jonathan");
        assert!(claim_label(&account, &generate_key(), &generate_key()).is_none());
    }

    /// `decrypt_user` is what the padlock reads: the flag comes from the server, the name from the
    /// project key, and a label that does not decrypt still leaves the lock visible.
    #[test]
    fn decrypt_user_carries_the_claim_through() {
        let key = test_key();
        let mut u = make_user(&key, 7, "Alice");
        u.claimed = true;
        u.claim_label = Some(encrypt_json(&key, &"Jonathan".to_string()).unwrap());

        let du = decrypt_user(&key, &u).unwrap();

        assert!(du.claimed);
        assert_eq!(du.claim_name.as_deref(), Some("Jonathan"));

        u.claim_label = Some(encrypt_json(&generate_key(), &"Jonathan".to_string()).unwrap());
        let du = decrypt_user(&key, &u).unwrap();
        assert!(du.claimed, "the lock survives a label it cannot read");
        assert_eq!(du.claim_name, None);
    }

    #[test]
    fn a_balanced_expense_is_consistent() {
        let pmts = [dp(1, false, 100.0), dp(1, true, 33.33), dp(1, true, 33.33), dp(1, true, 33.34)];
        assert!(!payments_are_inconsistent(100.0, &pmts));
    }

    #[test]
    fn the_reported_scenario_is_flagged() {
        // 100 expense, 60 + 60 paid, 70 + 70 owed.
        let pmts = [
            dp(1, false, 60.0),
            dp(1, false, 60.0),
            dp(1, true, 70.0),
            dp(1, true, 70.0),
        ];
        assert!(payments_are_inconsistent(100.0, &pmts));
    }

    #[test]
    fn either_side_alone_is_enough_to_flag() {
        assert!(payments_are_inconsistent(100.0, &[dp(1, false, 120.0), dp(1, true, 100.0)]));
        assert!(payments_are_inconsistent(100.0, &[dp(1, false, 100.0), dp(1, true, 90.0)]));
    }

    #[test]
    fn a_one_cent_gap_is_flagged() {
        assert!(payments_are_inconsistent(100.0, &[dp(1, false, 100.0), dp(1, true, 99.99)]));
    }

    #[test]
    fn an_undecryptable_expense_is_not_an_inconsistency() {
        // No payment decrypted — the key is wrong, the data is not necessarily broken.
        assert!(!payments_are_inconsistent(100.0, &[]));
    }

    #[test]
    fn a_nan_amount_is_flagged_without_panicking() {
        let pmts = [dp(1, false, f64::NAN), dp(1, true, 100.0)];
        assert!(payments_are_inconsistent(100.0, &pmts));
    }

    fn make_project(
        key: &[u8; 32],
        name: &str,
        description: Option<&str>,
        currency: &str,
    ) -> ProjectDto {
        ProjectDto {
            id: Uuid::nil(),
            payload: encrypt_json(key, &ProjectPayload {
                name: name.to_string(),
                currency: currency.to_string(),
                description: description.map(|s| s.to_string()),
            })
            .unwrap(),
            status: shared::ProjectStatus::Ongoing,
            created_at: chrono::NaiveDateTime::default(),
            owner_account_id: None,
        }
    }

    #[test]
    fn encrypt_decrypt_roundtrip() {
        let key = test_key();
        let (ct, iv) = encrypt(&key, "hello world").unwrap();
        assert_eq!(decrypt(&key, &ct, &iv).unwrap(), "hello world");
    }

    #[test]
    fn encrypt_decrypt_empty_string() {
        let key = test_key();
        let (ct, iv) = encrypt(&key, "").unwrap();
        assert_eq!(decrypt(&key, &ct, &iv).unwrap(), "");
    }

    #[test]
    fn encrypt_decrypt_unicode() {
        let key = test_key();
        let s = "Déjeuner à Paris — 日本語 🍜";
        let (ct, iv) = encrypt(&key, s).unwrap();
        assert_eq!(decrypt(&key, &ct, &iv).unwrap(), s);
    }

    #[test]
    fn decrypt_wrong_key_fails() {
        let key = test_key();
        let (ct, iv) = encrypt(&key, "secret").unwrap();
        let mut wrong = key;
        wrong[0] ^= 0xFF;
        assert!(decrypt(&wrong, &ct, &iv).is_err());
    }

    #[test]
    fn decrypt_flipped_ciphertext_byte_fails() {
        let key = test_key();
        let (ct_b64, iv) = encrypt(&key, "secret").unwrap();
        let mut ct = STANDARD.decode(&ct_b64).unwrap();
        ct[0] ^= 0x01;
        assert!(decrypt(&key, &STANDARD.encode(&ct), &iv).is_err());
    }

    #[test]
    fn iv_uniqueness() {
        let key = test_key();
        let p1 = encrypt_pair(&key, "same").unwrap();
        let p2 = encrypt_pair(&key, "same").unwrap();
        assert_ne!(p1.iv, p2.iv);
    }

    #[test]
    fn ciphertext_uniqueness() {
        let key = test_key();
        let p1 = encrypt_pair(&key, "same").unwrap();
        let p2 = encrypt_pair(&key, "same").unwrap();
        assert_ne!(p1.ct, p2.ct);
    }

    #[test]
    fn invalid_iv_length_fails() {
        let key = test_key();
        let bad_iv = STANDARD.encode([0u8; 8]);
        assert!(decrypt(&key, &STANDARD.encode([0u8; 32]), &bad_iv).is_err());
    }

    #[test]
    fn key_fragment_roundtrip() {
        let key = generate_key();
        let fragment = key_to_fragment(&key);
        let decoded = key_from_fragment(&fragment).unwrap();
        assert_eq!(key, decoded);
    }

    #[test]
    fn key_fragment_is_url_safe() {
        let key = generate_key();
        let fragment = key_to_fragment(&key);
        assert!(fragment.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_'));
    }

    #[test]
    fn key_from_fragment_wrong_length_fails() {
        let bad = URL_SAFE_NO_PAD.encode([0u8; 16]);
        assert!(key_from_fragment(&bad).is_err());
    }

    #[test]
    fn key_from_fragment_invalid_base64_fails() {
        assert!(key_from_fragment("not!valid@base64#chars").is_err());
    }

    #[test]
    fn encrypt_json_decrypt_json_roundtrip() {
        let key = test_key();
        let payload = UserPayload { name: "Alice".to_string() };
        let pair = encrypt_json(&key, &payload).unwrap();
        let decoded: UserPayload = decrypt_json(&key, &pair).unwrap();
        assert_eq!(decoded.name, "Alice");
    }

    #[test]
    fn encrypt_json_wrong_key_fails() {
        let key = test_key();
        let pair = encrypt_json(&key, &UserPayload { name: "Bob".to_string() }).unwrap();
        let mut bad = key;
        bad[0] ^= 0xFF;
        assert!(decrypt_json::<UserPayload>(&bad, &pair).is_err());
    }

    #[test]
    fn derive_account_key_v1_deterministic_with_same_salt() {
        let salt = [0x11u8; 16];
        let k1 = derive_account_key_v1("password", &salt);
        let k2 = derive_account_key_v1("password", &salt);
        assert_eq!(k1, k2);
    }

    #[test]
    fn derive_account_key_v1_different_salt_differs() {
        let k1 = derive_account_key_v1("password", &[0x11u8; 16]);
        let k2 = derive_account_key_v1("password", &[0x22u8; 16]);
        assert_ne!(k1, k2);
    }

    #[test]
    fn derive_account_key_v1_different_password_differs() {
        let salt = [0x11u8; 16];
        let k1 = derive_account_key_v1("password1", &salt);
        let k2 = derive_account_key_v1("password2", &salt);
        assert_ne!(k1, k2);
    }

    /// The legacy proof must equal the hash segment of a PHC string `Argon2::default()` produced
    /// on the server — that is what the migration turned into `login_hash`, and it is the only
    /// way an existing account logs in without re-enrolling.
    #[test]
    fn legacy_login_proof_matches_the_server_side_argon2_default() {
        let phc = "$argon2id$v=19$m=19456,t=2,p=1$LpH2yAl+C/509AhWo4P18w$Z66tA/LbS0eaxKXlv5gR0HO4QK9evX/soo4CT9tI9d4";
        let mut parts = phc.split('$').skip(4);
        let phc_b64 = base64::engine::general_purpose::STANDARD_NO_PAD;
        let salt = phc_b64.decode(parts.next().unwrap()).unwrap();
        let hash = phc_b64.decode(parts.next().unwrap()).unwrap();
        assert_eq!(salt.len(), 16);
        let proof = derive_login_proof(shared::AUTH_VERSION_LEGACY, "correct horse battery", &salt);
        assert_eq!(proof.to_vec(), hash);
    }

    /// The server holds the proof's hash; it must not be a value the account key derives from.
    #[test]
    fn current_login_proof_differs_from_the_account_key_under_the_same_salt() {
        let salt = [0x11u8; 16];
        let proof = derive_login_proof(shared::AUTH_VERSION_CURRENT, "password", &salt);
        assert_ne!(proof, derive_account_key_v1("password", &salt));
        assert_ne!(proof, derive_login_proof(shared::AUTH_VERSION_LEGACY, "password", &salt));
        assert_eq!(proof, derive_login_proof(shared::AUTH_VERSION_CURRENT, "password", &salt));
    }

    #[test]
    fn claim_verifier_is_the_hash_of_the_token_and_neither_is_the_key() {
        let key = [7u8; 32];
        use sha2::{Digest, Sha256};
        let expected: [u8; 32] = Sha256::digest(claim_token(&key)).into();
        assert_eq!(claim_verifier(&key), expected);
        assert_ne!(claim_token(&key), key);
        assert_ne!(claim_token(&key), claim_token(&[8u8; 32]));
    }

    #[test]
    fn a_boxed_project_key_opens_only_for_the_two_parties() {
        let (alice_pub, alice_priv) = generate_keypair();
        let (bob_pub, bob_priv) = generate_keypair();
        let (eve_pub, eve_priv) = generate_keypair();
        let project_key = generate_key();

        let boxed = box_project_key(&alice_priv, &bob_pub, &project_key).unwrap();

        assert_eq!(unbox_project_key(&bob_priv, &alice_pub, &boxed), Some(project_key));
        assert_eq!(unbox_project_key(&alice_priv, &bob_pub, &boxed), Some(project_key));
        assert_eq!(unbox_project_key(&eve_priv, &alice_pub, &boxed), None);
        assert_eq!(unbox_project_key(&bob_priv, &eve_pub, &boxed), None);
        assert!(decrypt_pair(&project_key, &boxed).is_err());
    }

    #[test]
    fn a_tampered_box_does_not_open() {
        let (alice_pub, alice_priv) = generate_keypair();
        let (bob_pub, bob_priv) = generate_keypair();
        let mut boxed = box_project_key(&alice_priv, &bob_pub, &generate_key()).unwrap();
        let mut ct = STANDARD.decode(&boxed.ct).unwrap();
        ct[0] ^= 1;
        boxed.ct = STANDARD.encode(ct);
        assert_eq!(unbox_project_key(&bob_priv, &alice_pub, &boxed), None);
        assert_eq!(unbox_project_key(&bob_priv, &[0u8; 31], &boxed), None);
    }

    #[test]
    fn the_private_key_round_trips_through_the_account_key() {
        let account_key = generate_key();
        let (_, private) = generate_keypair();
        let wrapped = wrap_private_key(&account_key, &private).unwrap();
        assert_eq!(unwrap_private_key(&account_key, &wrapped), Some(private));
        assert_eq!(unwrap_private_key(&generate_key(), &wrapped), None);
    }

    #[test]
    fn a_fingerprint_is_eight_stable_base32_characters() {
        let (public, _) = generate_keypair();
        let fp = key_fingerprint(&public);
        assert_eq!(fp.len(), 8);
        assert_eq!(fp, key_fingerprint(&public));
        assert!(fp.chars().all(|c| c.is_ascii_uppercase() || ('2'..='7').contains(&c)));
        assert_ne!(fp, key_fingerprint(&generate_keypair().0));
    }

    #[test]
    fn generate_kdf_salt_is_random() {
        let s1 = generate_kdf_salt();
        let s2 = generate_kdf_salt();
        assert_ne!(s1, s2);
    }

    #[test]
    fn decrypt_user_roundtrip() {
        let key = test_key();
        let user = make_user(&key, 7, "Alice");
        let du = decrypt_user(&key, &user).unwrap();
        assert_eq!(du.id, 7);
        assert_eq!(du.name, "Alice");
    }

    #[test]
    fn decrypt_user_wrong_key_fails() {
        let key = test_key();
        let user = make_user(&key, 1, "Bob");
        let mut bad = key;
        bad[0] ^= 0xFF;
        assert!(decrypt_user(&bad, &user).is_err());
    }

    #[test]
    fn decrypt_user_unicode_name() {
        let key = test_key();
        let user = make_user(&key, 2, "日本語 🍜");
        assert_eq!(decrypt_user(&key, &user).unwrap().name, "日本語 🍜");
    }

    #[test]
    fn decrypt_project_roundtrip() {
        let key = test_key();
        let p = make_project(&key, "Vacances", None, "EUR");
        let dp = decrypt_project(&key, &p).unwrap();
        assert_eq!(dp.name, "Vacances");
        assert_eq!(dp.currency, "EUR");
        assert!(dp.description.is_none());
    }

    #[test]
    fn decrypt_project_with_description() {
        let key = test_key();
        let p = make_project(&key, "Road trip", Some("Paris → Lyon"), "EUR");
        let dp = decrypt_project(&key, &p).unwrap();
        assert_eq!(dp.name, "Road trip");
        assert_eq!(dp.description.unwrap(), "Paris → Lyon");
    }

    #[test]
    fn decrypt_project_wrong_key_fails() {
        let key = test_key();
        let p = make_project(&key, "Secret", None, "EUR");
        let mut bad = key;
        bad[3] ^= 0xAB;
        assert!(decrypt_project(&bad, &p).is_err());
    }

    #[test]
    fn decrypt_expense_roundtrip() {
        let key = test_key();
        let e = make_expense(&key, 42, "Dîner", 45.50, ExpenseType::Expense, "2025-06-15");
        let de = decrypt_expense(&key, &e).unwrap();
        assert_eq!(de.id, 42);
        assert_eq!(de.name, "Dîner");
        assert!((de.amount - 45.50).abs() < 1e-9);
        assert_eq!(de.expense_type, ExpenseType::Expense);
        assert_eq!(de.date, "2025-06-15");
        assert!(de.description.is_none());
    }

    #[test]
    fn decrypt_expense_all_types() {
        let key = test_key();
        for t in [ExpenseType::Expense, ExpenseType::Transfer, ExpenseType::Gain] {
            let e = make_expense(&key, 1, "x", 10.0, t.clone(), "2025-01-01");
            assert_eq!(decrypt_expense(&key, &e).unwrap().expense_type, t);
        }
    }

    #[test]
    fn decrypt_expense_with_description() {
        let key = test_key();
        let e = Expense {
            id: 1,
            author_id: Some(1),
            project_id: Uuid::nil(),
            created_at: chrono::NaiveDateTime::default(),
            payload: encrypt_json(&key, &ExpensePayload {
                name: "Lunch".to_string(),
                amount: 20.0,
                expense_type: "expense".to_string(),
                date: "2025-01-01".to_string(),
                description: Some("Mediterranean restaurant".to_string()),
                category: None,
                source_currency: None,
                source_amount: None,
                rate: None,
            })
            .unwrap(),
        };
        let de = decrypt_expense(&key, &e).unwrap();
        assert_eq!(de.description.unwrap(), "Mediterranean restaurant");
    }

    #[test]
    fn decrypt_expense_wrong_key_fails() {
        let key = test_key();
        let e = make_expense(&key, 1, "secret", 100.0, ExpenseType::Expense, "2025-01-01");
        let mut bad = key;
        bad[0] ^= 0xFF;
        assert!(decrypt_expense(&bad, &e).is_err());
    }

    #[test]
    fn decrypt_expense_tampered_payload_fails() {
        let key = test_key();
        let mut e = make_expense(&key, 1, "test", 50.0, ExpenseType::Expense, "2025-01-01");
        let mut ct = STANDARD.decode(&e.payload.ct).unwrap();
        ct[0] ^= 0x01;
        e.payload.ct = STANDARD.encode(&ct);
        assert!(decrypt_expense(&key, &e).is_err());
    }

    #[test]
    fn decrypt_payment_payer_roundtrip() {
        let key = test_key();
        let p = make_payment(&key, 1, 10, 5, false, 30.0);
        let dp = decrypt_payment(&key, &p).unwrap();
        assert_eq!(dp.user_id, 5);
        assert!(!dp.is_debt);
        assert!((dp.amount - 30.0).abs() < 1e-9);
    }

    #[test]
    fn decrypt_payment_debtor_roundtrip() {
        let key = test_key();
        let p = make_payment(&key, 2, 10, 7, true, 15.0);
        let dp = decrypt_payment(&key, &p).unwrap();
        assert_eq!(dp.user_id, 7);
        assert!(dp.is_debt);
        assert!((dp.amount - 15.0).abs() < 1e-9);
    }

    #[test]
    fn decrypt_payment_wrong_key_fails() {
        let key = test_key();
        let p = make_payment(&key, 1, 1, 1, false, 10.0);
        let mut bad = key;
        bad[1] ^= 0x55;
        assert!(decrypt_payment(&bad, &p).is_err());
    }

    #[test]
    fn decrypt_payment_tampered_payload_fails() {
        let key = test_key();
        let mut p = make_payment(&key, 1, 1, 1, true, 10.0);
        let mut ct = STANDARD.decode(&p.payload.ct).unwrap();
        ct[0] ^= 0x01;
        p.payload.ct = STANDARD.encode(&ct);
        assert!(decrypt_payment(&key, &p).is_err());
    }

    #[test]
    fn twelve_byte_iv_is_rejected() {
        let key = test_key();
        let iv_b64 = STANDARD.encode([0u8; 12]);
        assert!(decrypt(&key, &STANDARD.encode([0u8; 32]), &iv_b64).is_err());
    }

    // enc helper is used for the unused-but-valid test below
    #[allow(dead_code)]
    fn _enc_used_check(key: &[u8; 32]) -> EncryptedPair {
        enc(key, "test")
    }
}
