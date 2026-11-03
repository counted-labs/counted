use argon2::{Algorithm, Argon2, Params, Version};
use base64::{
    engine::general_purpose::{STANDARD, URL_SAFE_NO_PAD},
    Engine as _,
};
use chacha20poly1305::{
    aead::{Aead, KeyInit},
    consts::U24,
    XChaCha20Poly1305,
};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use sha2::{Digest, Sha256};
use shared::{Account, EncryptedPair};
use uuid::Uuid;

fn random_bytes<const N: usize>() -> [u8; N] {
    let mut bytes = [0u8; N];
    getrandom::getrandom(&mut bytes).expect("no CSPRNG available");
    bytes
}

pub fn generate_key() -> [u8; 32] {
    random_bytes()
}

pub fn generate_kdf_salt() -> [u8; 16] {
    random_bytes()
}

pub fn key_to_fragment(key: &[u8; 32]) -> String {
    URL_SAFE_NO_PAD.encode(key)
}

pub fn key_from_fragment(fragment: &str) -> Result<[u8; 32], String> {
    let bytes = URL_SAFE_NO_PAD.decode(fragment).map_err(|e| format!("base64: {e}"))?;
    bytes.try_into().map_err(|_| "key must be 32 bytes".to_string())
}

fn seal(cipher: &impl Aead<NonceSize = U24>, plaintext: &[u8]) -> Result<EncryptedPair, String> {
    let iv: [u8; 24] = random_bytes();
    let ct = cipher.encrypt(&iv.into(), plaintext).map_err(|e| format!("encrypt: {e}"))?;
    Ok(EncryptedPair { ct: STANDARD.encode(ct), iv: STANDARD.encode(iv) })
}

fn open(cipher: &impl Aead<NonceSize = U24>, pair: &EncryptedPair) -> Result<Vec<u8>, String> {
    let ct = STANDARD.decode(&pair.ct).map_err(|e| format!("ct base64: {e}"))?;
    let iv: [u8; 24] = STANDARD
        .decode(&pair.iv)
        .map_err(|e| format!("iv base64: {e}"))?
        .try_into()
        .map_err(|iv: Vec<u8>| format!("invalid IV length: {}", iv.len()))?;
    cipher.decrypt(&iv.into(), ct.as_ref()).map_err(|e| format!("decrypt: {e}"))
}

/// Bytes in, bytes out, with no parsing: what a key rotation re-seals, so a row survives byte for
/// byte whatever its plaintext (docs/plans/key-lifecycle.md §8.3).
pub fn encrypt_bytes(key: &[u8; 32], plaintext: &[u8]) -> Result<EncryptedPair, String> {
    seal(&XChaCha20Poly1305::new(key.into()), plaintext)
}

/// `Err` means the AEAD tag did not check: the one thing a rotation counts as junk.
pub fn decrypt_bytes(key: &[u8; 32], pair: &EncryptedPair) -> Result<Vec<u8>, String> {
    open(&XChaCha20Poly1305::new(key.into()), pair)
}

pub fn encrypt(key: &[u8; 32], plaintext: &str) -> Result<EncryptedPair, String> {
    seal(&XChaCha20Poly1305::new(key.into()), plaintext.as_bytes())
}

// The expenses-tab guard asserts on this count, not wall-clock - the stall it protects against was
// a call-count bug. Thread-local: `cargo test` runs tests concurrently.
#[cfg(test)]
thread_local! {
    pub(crate) static DECRYPT_COUNT: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

/// Real decryptions performed by `f`, on this thread, starting from an empty [`OPENED`].
#[cfg(test)]
pub(crate) fn count_decrypts<T>(f: impl FnOnce() -> T) -> (T, usize) {
    forget_decrypted();
    DECRYPT_COUNT.with(|c| c.set(0));
    let out = f();
    (out, DECRYPT_COUNT.with(|c| c.get()))
}

/// Past this the map is emptied, not evicted: refilling costs one pass, which is the price of
/// every read before this cache existed.
const OPENED_CAP: usize = 30_000;

// The same rows reach `decrypt` from every page and memo that shows them - the projects list
// alone passed each group three times. Keyed by a digest of key, nonce and ciphertext together,
// so a different key, a reused nonce with other bytes, or one flipped bit misses and goes through
// the AEAD check. Only ever filled with what that check accepted.
thread_local! {
    static OPENED: std::cell::RefCell<std::collections::HashMap<[u8; 32], String>> =
        std::cell::RefCell::new(std::collections::HashMap::new());
}

fn opened_digest(key: &[u8; 32], pair: &EncryptedPair) -> [u8; 32] {
    Sha256::new()
        .chain_update(key)
        .chain_update(pair.iv.as_bytes())
        .chain_update(b":")
        .chain_update(pair.ct.as_bytes())
        .finalize()
        .into()
}

/// What this device has decrypted, gone - for logout, so no plaintext outlives the session.
pub fn forget_decrypted() {
    OPENED.with(|o| o.borrow_mut().clear());
}

pub fn decrypt(key: &[u8; 32], pair: &EncryptedPair) -> Result<String, String> {
    let digest = opened_digest(key, pair);
    if let Some(plain) = OPENED.with(|o| o.borrow().get(&digest).cloned()) {
        return Ok(plain);
    }

    #[cfg(test)]
    DECRYPT_COUNT.with(|c| c.set(c.get() + 1));

    let plain = open(&XChaCha20Poly1305::new(key.into()), pair)?;
    let plain = String::from_utf8(plain).map_err(|e| format!("utf8: {e}"))?;
    OPENED.with(|o| {
        let mut opened = o.borrow_mut();
        if opened.len() >= OPENED_CAP {
            opened.clear();
        }
        opened.insert(digest, plain.clone());
    });
    Ok(plain)
}

pub fn encrypt_json<T: Serialize>(key: &[u8; 32], value: &T) -> Result<EncryptedPair, String> {
    let json = serde_json::to_string(value).map_err(|e| e.to_string())?;
    encrypt(key, &json)
}

/// For a payload that names participants - expense, rule, history: its length must not count them.
pub fn encrypt_padded_json<T: Serialize>(key: &[u8; 32], value: &T) -> Result<EncryptedPair, String> {
    let json = serde_json::to_string(value).map_err(|e| e.to_string())?;
    encrypt(key, &shared::pad_json(json))
}

/// For a blob whose base64 ciphertext the server caps at `max_ct_b64`: padded like
/// [`encrypt_padded_json`] as far as the cap allows, never truncated, so whatever fitted unpadded
/// still fits.
pub fn encrypt_padded_json_within<T: Serialize>(key: &[u8; 32], value: &T, max_ct_b64: usize) -> Result<EncryptedPair, String> {
    let mut json = serde_json::to_string(value).map_err(|e| e.to_string())?;
    let largest_fitting = (max_ct_b64 / 4 * 3).saturating_sub(16);
    let target = json.len().max(256).next_power_of_two().min(largest_fitting).max(json.len());
    json.push_str(&" ".repeat(target - json.len()));
    encrypt(key, &json)
}

pub fn decrypt_json<T: DeserializeOwned>(key: &[u8; 32], pair: &EncryptedPair) -> Result<T, String> {
    serde_json::from_str(&decrypt(key, pair)?).map_err(|e| e.to_string())
}

/// What a key wrapped under the account key is for. A typed wrap names it, so a blob written for
/// one purpose - a friend label, another project's escrow row - never opens as another
/// (audit 2026-10-19 L1).
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum WrapUse {
    PrivateKey,
    Escrow(Uuid),
}

#[derive(Serialize, Deserialize)]
struct KeyWrap {
    #[serde(rename = "use")]
    use_: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    project: Option<Uuid>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    account: Option<Uuid>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    epoch: Option<i32>,
    key: String,
}

impl WrapUse {
    fn parts(self) -> (&'static str, Option<Uuid>) {
        match self {
            WrapUse::PrivateKey => ("account-sk", None),
            WrapUse::Escrow(project) => ("escrow", Some(project)),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Unwrapped {
    pub key: [u8; 32],
    pub typed: bool,
}

/// A project key or the private key, encrypted under the account key - see `docs/e2ee.md`, "Key
/// escrow". Always a typed [`KeyWrap`], whatever `Account.typed_wraps` says: every build the server
/// still serves reads it, and a server-held switch would let a hostile server keep wraps untyped.
pub fn wrap_key(account_key: &[u8; 32], use_: WrapUse, key: &[u8; 32]) -> Option<EncryptedPair> {
    let (use_, project) = use_.parts();
    encrypt_json(account_key, &KeyWrap { use_: use_.to_string(), project, account: None, epoch: None, key: key_to_fragment(key) }).ok()
}

/// The account's ML-KEM seed under the master key (docs/plans/key-lifecycle.md §7), strictly typed.
pub fn wrap_kem_seed(amk: &[u8; 32], seed: &[u8; crate::hybrid::KEM_SEED_LENGTH]) -> Option<EncryptedPair> {
    let wrap = KeyWrap { use_: "account-kem".into(), project: None, account: None, epoch: None, key: URL_SAFE_NO_PAD.encode(seed) };
    encrypt_json(amk, &wrap).ok()
}

pub fn open_kem_seed(amk: &[u8; 32], wrapped: &EncryptedPair) -> Option<[u8; crate::hybrid::KEM_SEED_LENGTH]> {
    let wrap: KeyWrap = serde_json::from_str(&decrypt(amk, wrapped).ok()?).ok()?;
    if wrap.use_ != "account-kem" || wrap.project.is_some() {
        return None;
    }
    URL_SAFE_NO_PAD.decode(&wrap.key).ok()?.try_into().ok()
}

/// The master key under the recovery key's `rk_key` (docs/plans/key-lifecycle.md §6.1), typed with
/// the account. No epoch: restoring a retired generation brings its recovery wrap back as it was.
pub fn wrap_recovery(rk_key: &[u8; 32], account: Uuid, amk: &[u8; 32]) -> Option<EncryptedPair> {
    let wrap = KeyWrap { use_: "amk-recovery".into(), project: None, account: Some(account), epoch: None, key: key_to_fragment(amk) };
    encrypt_json(rk_key, &wrap).ok()
}

pub fn open_recovery(rk_key: &[u8; 32], account: Uuid, wrapped: &EncryptedPair) -> Option<[u8; 32]> {
    let wrap: KeyWrap = serde_json::from_str(&decrypt(rk_key, wrapped).ok()?).ok()?;
    let matches = wrap.use_ == "amk-recovery" && wrap.project.is_none() && wrap.account == Some(account);
    matches.then(|| key_from_fragment(&wrap.key).ok()).flatten()
}

/// The account master key under the password-derived key (docs/plans/key-lifecycle.md §3).
pub fn wrap_amk(pw_key: &[u8; 32], account: Uuid, epoch: i32, amk: &[u8; 32]) -> Option<EncryptedPair> {
    let wrap = KeyWrap { use_: "amk".into(), project: None, account: Some(account), epoch: Some(epoch), key: key_to_fragment(amk) };
    encrypt_json(pw_key, &wrap).ok()
}

/// Strictly typed, unlike [`open_wrap`]: no untyped fallback, and only for the account and epoch the
/// server serves. Otherwise a hostile server could serve an old untyped escrow blob, holding a key
/// it knows, as the master key.
pub fn open_amk(pw_key: &[u8; 32], account: Uuid, epoch: i32, wrapped: &EncryptedPair) -> Option<[u8; 32]> {
    let wrap: KeyWrap = serde_json::from_str(&decrypt(pw_key, wrapped).ok()?).ok()?;
    let matches = wrap.use_ == "amk" && wrap.project.is_none() && wrap.account == Some(account) && wrap.epoch == Some(epoch);
    matches.then(|| key_from_fragment(&wrap.key).ok()).flatten()
}

/// The bare fragment builds before 3.2.6 wrote, for tests of the rows they left.
#[cfg(test)]
pub fn wrap_key_untyped(account_key: &[u8; 32], _use: WrapUse, key: &[u8; 32]) -> Option<EncryptedPair> {
    encrypt(account_key, &key_to_fragment(key)).ok()
}

/// A typed wrap opens only for the use it names; an untyped one for any use.
pub fn open_wrap(account_key: &[u8; 32], use_: WrapUse, wrapped: &EncryptedPair) -> Option<Unwrapped> {
    let plain = decrypt(account_key, wrapped).ok()?;
    if let Ok(wrap) = serde_json::from_str::<KeyWrap>(&plain) {
        let typed = (wrap.use_.as_str(), wrap.project) == use_.parts();
        return typed.then(|| key_from_fragment(&wrap.key).ok()).flatten().map(|key| Unwrapped { key, typed: true });
    }
    key_from_fragment(&plain).ok().map(|key| Unwrapped { key, typed: false })
}

/// `raw_ok` is false once this device has seen a typed wrap for the account: from then on a server
/// serving an untyped blob is refused (`LocalStorageState.typed_wrap_accounts`).
pub fn unwrap_key(account_key: &[u8; 32], use_: WrapUse, wrapped: &EncryptedPair, raw_ok: bool) -> Option<[u8; 32]> {
    open_wrap(account_key, use_, wrapped).filter(|u| u.typed || raw_ok).map(|u| u.key)
}

/// The account's `display_name`, re-encrypted under the project key so the other members can see
/// who claimed an identity - see `docs/e2ee.md`, "The claim label". `None` when the name does not
/// decrypt under `account_key`. Padded like the name it copies (audit 2026-10-19 L4).
pub fn claim_label(
    account: &Account,
    account_key: &[u8; 32],
    project_key: &[u8; 32],
) -> Option<EncryptedPair> {
    let name: String = decrypt_json(account_key, &account.display_name).ok()?;
    encrypt_padded_json(project_key, &name).ok()
}

/// Proof of holding a project key, for identity claims - see `docs/e2ee.md`, "The claim token".
/// A fixed label in front of a fixed-length secret, so plain SHA-256 needs no MAC construction.
pub fn claim_token(project_key: &[u8; 32]) -> [u8; 32] {
    Sha256::new().chain_update(b"counted-claim-v1").chain_update(project_key).finalize().into()
}

/// What the server stores and compares a presented token against.
pub fn claim_verifier(project_key: &[u8; 32]) -> [u8; 32] {
    Sha256::digest(claim_token(project_key)).into()
}

/// The account's X25519 keypair (public, private) - see `docs/e2ee.md`, "Account keypair and boxed
/// invitations".
pub fn generate_keypair() -> (Vec<u8>, [u8; 32]) {
    let secret = crypto_box::SecretKey::from_bytes(generate_key());
    (secret.public_key().to_bytes().to_vec(), secret.to_bytes())
}

pub fn public_key_of(private: &[u8; 32]) -> Vec<u8> {
    crypto_box::SecretKey::from_bytes(*private).public_key().to_bytes().to_vec()
}

/// `crypto_box` does not reject what libsodium does: a key of small order yields one of at most 8
/// shared secrets and leaks the sender's private scalar mod 8. Such a point is one that [8]P sends
/// to the identity, which the ladder encodes as u = 0.
fn has_small_order(public: &[u8; 32]) -> bool {
    use curve25519_dalek::{montgomery::MontgomeryPoint, scalar::Scalar};
    (MontgomeryPoint(*public) * Scalar::from(8u8)).to_bytes() == [0u8; 32]
}

fn chacha_box(my_private: &[u8; 32], their_public: &[u8]) -> Option<crypto_box::ChaChaBox> {
    let public = crypto_box::PublicKey::from_slice(their_public).ok()?;
    if has_small_order(public.as_bytes()) {
        return None;
    }
    Some(crypto_box::ChaChaBox::new(&public, &crypto_box::SecretKey::from_bytes(*my_private)))
}

/// A project key boxed for one friend. Authenticated, not sealed: the recipient opens it with the
/// sender's public key, which is how they know who sent it.
pub fn box_project_key(
    my_private: &[u8; 32],
    their_public: &[u8],
    project_key: &[u8; 32],
) -> Option<EncryptedPair> {
    seal(&chacha_box(my_private, their_public)?, key_to_fragment(project_key).as_bytes()).ok()
}

pub fn unbox_project_key(
    my_private: &[u8; 32],
    their_public: &[u8],
    boxed: &EncryptedPair,
) -> Option<[u8; 32]> {
    let plain = open(&chacha_box(my_private, their_public)?, boxed).ok()?;
    key_from_fragment(std::str::from_utf8(&plain).ok()?).ok()
}

/// Sixteen characters, in groups of four, that two friends can read to each other to check the
/// server handed out the right key: SHA-256 of the public key, first 80 bits, base32. 40 bits was
/// within reach of a server grinding a lookalike key - see `docs/e2ee.md`.
pub fn key_fingerprint(public_key: &[u8]) -> String {
    const ALPHABET: &[u8; 32] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ234567";
    let digest = Sha256::digest(public_key);
    let bits = digest[..10].iter().fold(0u128, |acc, byte| (acc << 8) | u128::from(*byte));
    let chars: Vec<char> =
        (0..16).rev().map(|i| ALPHABET[((bits >> (i * 5)) & 31) as usize] as char).collect();
    chars.chunks(4).map(|group| group.iter().collect::<String>()).collect::<Vec<_>>().join(" ")
}

/// Argon2id, m = 64 MiB, t = 3, 32-byte output: RFC 9106's second recommended option, run with
/// p = 1 because the WASM build has one thread.
const ACCOUNT_KDF_PARAMS: Params = match Params::new(65_536, 3, 1, Some(32)) {
    Ok(params) => params,
    Err(_) => panic!("invalid argon2 params"),
};

/// An `Err`, never a panic: the salt comes from the server, and argon2 refuses one under 8 bytes.
fn argon2_hash(argon2: &Argon2, password: &str, salt: &[u8]) -> Result<[u8; 32], String> {
    let mut out = [0u8; 32];
    argon2.hash_password_into(password.as_bytes(), salt, &mut out).map_err(|e| format!("argon2: {e}"))?;
    Ok(out)
}

pub fn derive_account_key_v1(password: &str, salt: &[u8]) -> Result<[u8; 32], String> {
    argon2_hash(&Argon2::new(Algorithm::Argon2id, Version::V0x13, ACCOUNT_KDF_PARAMS), password, salt)
}

/// What the server is sent instead of the password - see `docs/auth.md`, "Login proof".
///
/// - legacy: `Argon2::default()`, exactly what the server used to run on the plaintext, so the
///   stored hash verifies it without re-enrolment;
/// - current: the account-key parameters keyed with a fixed secret, so the proof is a different
///   function of the password than the account key even under an equal salt.
pub fn derive_login_proof(auth_version: i16, password: &str, salt: &[u8]) -> Result<[u8; 32], String> {
    let argon2 = if auth_version == shared::AUTH_VERSION_LEGACY {
        Argon2::new(Algorithm::Argon2id, Version::V0x13, Params::default())
    } else {
        Argon2::new_with_secret(b"counted-login-v2", Algorithm::Argon2id, Version::V0x13, ACCOUNT_KDF_PARAMS)
            .map_err(|e| format!("argon2: {e}"))?
    };
    argon2_hash(&argon2, password, salt)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDateTime;
    use shared::UserPayload;
    use uuid::Uuid;

    use crate::common::test_fixtures::test_key;

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
            typed_wraps: false,
            ..Default::default()
        }
    }

    /// The whole point of the claim label: a name only the account could read becomes a name every
    /// member of the project can read, and nobody else - the server included.
    #[test]
    fn a_claim_label_crosses_from_the_account_key_to_the_project_key() {
        let account_key = generate_key();
        let project_key = generate_key();
        let account = account_with_name(&account_key, "Jonathan");

        let label = claim_label(&account, &account_key, &project_key).unwrap();

        assert_eq!(decrypt_json::<String>(&project_key, &label).unwrap(), "Jonathan");
        // The account key must not open it - it is a genuinely separate copy, not a re-wrap.
        assert!(decrypt_json::<String>(&account_key, &label).is_err());
    }

    /// Audit 2026-10-19 L4: the server sees the same length whatever the name.
    #[test]
    fn a_claim_label_does_not_reveal_the_name_length() {
        let account_key = generate_key();
        let project_key = generate_key();
        let len = |name: &str| claim_label(&account_with_name(&account_key, name), &account_key, &project_key).unwrap().ct.len();

        assert_eq!(len("Al"), len("Alexandra-Marie de la Fontaine"));
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

    #[test]
    fn a_master_key_wrap_opens_only_for_its_account_and_epoch() {
        let (pw_key, amk, account) = (generate_key(), generate_key(), Uuid::new_v4());
        let wrapped = wrap_amk(&pw_key, account, 2, &amk).unwrap();

        assert_eq!(open_amk(&pw_key, account, 2, &wrapped), Some(amk));
        assert_eq!(open_amk(&pw_key, account, 1, &wrapped), None, "another epoch");
        assert_eq!(open_amk(&pw_key, Uuid::new_v4(), 2, &wrapped), None, "another account");
        assert_eq!(open_amk(&generate_key(), account, 2, &wrapped), None, "another password");
    }

    /// Plan §3, proof U1: an escrow blob, typed or untyped, holding a key the server knows must
    /// never open as the master key.
    #[test]
    fn no_other_wrap_opens_as_the_master_key() {
        let (pw_key, known, account) = (generate_key(), generate_key(), Uuid::new_v4());
        let untyped = wrap_key_untyped(&pw_key, WrapUse::Escrow(account), &known).unwrap();
        let escrow = wrap_key(&pw_key, WrapUse::Escrow(account), &known).unwrap();
        let private = wrap_key(&pw_key, WrapUse::PrivateKey, &known).unwrap();

        for blob in [untyped, escrow, private] {
            assert_eq!(open_amk(&pw_key, account, 1, &blob), None);
        }
    }

    #[test]
    fn a_recovery_wrap_opens_for_its_account_only() {
        let (rk, amk, account) = (generate_key(), generate_key(), Uuid::new_v4());
        let wrapped = wrap_recovery(&rk, account, &amk).unwrap();
        assert_eq!(open_recovery(&rk, account, &wrapped), Some(amk));
        assert_eq!(open_recovery(&rk, Uuid::new_v4(), &wrapped), None);
        assert_eq!(open_amk(&rk, account, 0, &wrapped), None);
        assert_eq!(open_recovery(&rk, account, &wrap_amk(&rk, account, 1, &amk).unwrap()), None);
    }

    #[test]
    fn a_kem_seed_wrap_opens_as_nothing_else() {
        let (amk, seed) = (generate_key(), crate::hybrid::generate_kem_seed());
        let wrapped = wrap_kem_seed(&amk, &seed).unwrap();

        assert_eq!(open_kem_seed(&amk, &wrapped), Some(seed));
        assert_eq!(open_kem_seed(&generate_key(), &wrapped), None);
        assert_eq!(open_wrap(&amk, WrapUse::PrivateKey, &wrapped), None);
        assert_eq!(open_kem_seed(&amk, &wrap_key(&amk, WrapUse::PrivateKey, &generate_key()).unwrap()), None);
    }

    #[test]
    fn a_master_key_wrap_does_not_open_as_another_use() {
        let (pw_key, amk, account) = (generate_key(), generate_key(), Uuid::new_v4());
        let wrapped = wrap_amk(&pw_key, account, 1, &amk).unwrap();

        assert_eq!(open_wrap(&pw_key, WrapUse::PrivateKey, &wrapped), None);
        assert_eq!(open_wrap(&pw_key, WrapUse::Escrow(account), &wrapped), None);
    }

    #[test]
    fn encrypt_decrypt_roundtrip() {
        let key = test_key();
        for s in ["hello world", "", "Déjeuner à Paris - 日本語 🍜"] {
            assert_eq!(decrypt(&key, &encrypt(&key, s).unwrap()).unwrap(), s);
        }
    }

    #[test]
    fn decrypt_wrong_key_fails() {
        let key = test_key();
        let pair = encrypt(&key, "secret").unwrap();
        let mut wrong = key;
        wrong[0] ^= 0xFF;
        assert!(decrypt(&wrong, &pair).is_err());
    }

    #[test]
    fn decrypt_flipped_ciphertext_byte_fails() {
        let key = test_key();
        let mut pair = encrypt(&key, "secret").unwrap();
        let mut ct = STANDARD.decode(&pair.ct).unwrap();
        ct[0] ^= 0x01;
        pair.ct = STANDARD.encode(&ct);
        assert!(decrypt(&key, &pair).is_err());
    }

    #[test]
    fn the_same_plaintext_never_encrypts_the_same_way_twice() {
        let key = test_key();
        let (p1, p2) = (encrypt(&key, "same").unwrap(), encrypt(&key, "same").unwrap());
        assert_ne!(p1.iv, p2.iv);
        assert_ne!(p1.ct, p2.ct);
    }

    #[test]
    fn only_a_24_byte_iv_is_accepted() {
        let key = test_key();
        for len in [8, 12, 23, 25] {
            let pair =
                EncryptedPair { ct: STANDARD.encode([0u8; 32]), iv: STANDARD.encode(vec![0u8; len]) };
            assert!(decrypt(&key, &pair).is_err());
            assert_eq!(unbox_project_key(&[1u8; 32], &generate_keypair().0, &pair), None);
        }
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
        let payload = UserPayload { name: "Alice".to_string(), removed: false };
        let pair = encrypt_json(&key, &payload).unwrap();
        let decoded: UserPayload = decrypt_json(&key, &pair).unwrap();
        assert_eq!(decoded.name, "Alice");
    }

    #[test]
    fn encrypt_json_wrong_key_fails() {
        let key = test_key();
        let pair = encrypt_json(&key, &UserPayload { name: "Bob".to_string(), removed: false }).unwrap();
        let mut bad = key;
        bad[0] ^= 0xFF;
        assert!(decrypt_json::<UserPayload>(&bad, &pair).is_err());
    }

    #[test]
    fn derive_account_key_v1_deterministic_with_same_salt() {
        let salt = [0x11u8; 16];
        let k1 = derive_account_key_v1("password", &salt).unwrap();
        let k2 = derive_account_key_v1("password", &salt).unwrap();
        assert_eq!(k1, k2);
    }

    #[test]
    fn derive_account_key_v1_different_salt_differs() {
        let k1 = derive_account_key_v1("password", &[0x11u8; 16]).unwrap();
        let k2 = derive_account_key_v1("password", &[0x22u8; 16]).unwrap();
        assert_ne!(k1, k2);
    }

    #[test]
    fn derive_account_key_v1_different_password_differs() {
        let salt = [0x11u8; 16];
        let k1 = derive_account_key_v1("password1", &salt).unwrap();
        let k2 = derive_account_key_v1("password2", &salt).unwrap();
        assert_ne!(k1, k2);
    }

    /// The legacy proof must equal the hash segment of a PHC string `Argon2::default()` produced
    /// on the server - that is what the migration turned into `login_hash`, and it is the only
    /// way an existing account logs in without re-enrolling.
    #[test]
    fn legacy_login_proof_matches_the_server_side_argon2_default() {
        let phc = "$argon2id$v=19$m=19456,t=2,p=1$LpH2yAl+C/509AhWo4P18w$Z66tA/LbS0eaxKXlv5gR0HO4QK9evX/soo4CT9tI9d4";
        let mut parts = phc.split('$').skip(4);
        let phc_b64 = base64::engine::general_purpose::STANDARD_NO_PAD;
        let salt = phc_b64.decode(parts.next().unwrap()).unwrap();
        let hash = phc_b64.decode(parts.next().unwrap()).unwrap();
        assert_eq!(salt.len(), 16);
        let proof = derive_login_proof(shared::AUTH_VERSION_LEGACY, "correct horse battery", &salt).unwrap();
        assert_eq!(proof.to_vec(), hash);
    }

    /// The server holds the proof's hash; it must not be a value the account key derives from.
    #[test]
    fn current_login_proof_differs_from_the_account_key_under_the_same_salt() {
        let salt = [0x11u8; 16];
        let proof = derive_login_proof(shared::AUTH_VERSION_CURRENT, "password", &salt).unwrap();
        assert_ne!(proof, derive_account_key_v1("password", &salt).unwrap());
        assert_ne!(proof, derive_login_proof(shared::AUTH_VERSION_LEGACY, "password", &salt).unwrap());
        assert_eq!(proof, derive_login_proof(shared::AUTH_VERSION_CURRENT, "password", &salt).unwrap());
    }

    #[test]
    fn claim_verifier_is_the_hash_of_the_token_and_neither_is_the_key() {
        let key = [7u8; 32];
        let expected: [u8; 32] = Sha256::digest(claim_token(&key)).into();
        assert_eq!(claim_verifier(&key), expected);
        assert_ne!(claim_token(&key), key);
        assert_ne!(claim_token(&key), claim_token(&[8u8; 32]));
    }

    /// Known answers: every byte here is already stored in production. A change that breaks one of
    /// these makes existing data unreadable.
    #[test]
    fn stored_ciphertext_still_opens() {
        let pair = EncryptedPair {
            ct: "/C2rdoVCtdYK3ZAej3L0eJOmwutfGEWl1v46733d".into(),
            iv: "CCqHN62xmXAe2DC7GSjfpPTREZeuOHm2".into(),
        };
        assert_eq!(decrypt_json::<String>(&[7u8; 32], &pair).unwrap(), "Weekend trip");
    }

    #[test]
    fn a_stored_box_still_opens() {
        let alice = crypto_box::SecretKey::from_bytes([1u8; 32]);
        let boxed = EncryptedPair {
            ct: "D9WCNfn2fqB0KJnUUYlu62jNE+MhMBhBgXxPC38URyIYssGmA5YzUaKbcceqhN1Y4IsojuLVKlXcCjw=".into(),
            iv: "3dibiCBYV3+7yGCn8BBp7Z/4zbNS5lki".into(),
        };
        assert_eq!(unbox_project_key(&[2u8; 32], alice.public_key().as_bytes(), &boxed), Some([7u8; 32]));
    }

    #[test]
    fn derivations_are_unchanged() {
        assert_eq!(STANDARD.encode(claim_token(&[7u8; 32])), "1KxVDogT+QG43nuv4CJUZaD2XTrcSRfgRtHfxV3loCg=");
        assert_eq!(
            STANDARD.encode(derive_account_key_v1("password", &[0x11u8; 16]).unwrap()),
            "/SPWqp3PDq/+ghu8SN2qps4tQD8ZZkfH/Isz8fQXok8="
        );
        assert_eq!(
            STANDARD.encode(derive_login_proof(shared::AUTH_VERSION_CURRENT, "password", &[0x11u8; 16]).unwrap()),
            "DUkKmgemaxbhVxsKh20fRqjNBC/eb8WqiyQPJFKT7ic="
        );
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
        assert!(decrypt(&project_key, &boxed).is_err());
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
        for wrap in [wrap_key, wrap_key_untyped] {
            let wrapped = wrap(&account_key, WrapUse::PrivateKey, &private).unwrap();
            assert_eq!(unwrap_key(&account_key, WrapUse::PrivateKey, &wrapped, true), Some(private));
            assert_eq!(unwrap_key(&generate_key(), WrapUse::PrivateKey, &wrapped, true), None);
        }
    }

    /// Audit 2026-10-19 L1: an escrow row of a project the attacker shares, served as the private
    /// key or as another project's row, does not open.
    #[test]
    fn a_typed_wrap_opens_only_for_the_use_it_names() {
        let account_key = generate_key();
        let (a, b) = (Uuid::new_v4(), Uuid::new_v4());
        let project_key = generate_key();
        let escrow = wrap_key(&account_key, WrapUse::Escrow(a), &project_key).unwrap();

        assert_eq!(unwrap_key(&account_key, WrapUse::Escrow(a), &escrow, false), Some(project_key));
        assert_eq!(unwrap_key(&account_key, WrapUse::Escrow(b), &escrow, true), None);
        assert_eq!(unwrap_key(&account_key, WrapUse::PrivateKey, &escrow, true), None);
    }

    #[test]
    fn an_untyped_wrap_opens_until_the_device_has_seen_a_typed_one() {
        let account_key = generate_key();
        let key = generate_key();
        let raw = wrap_key_untyped(&account_key, WrapUse::PrivateKey, &key).unwrap();

        assert_eq!(open_wrap(&account_key, WrapUse::PrivateKey, &raw), Some(Unwrapped { key, typed: false }));
        assert_eq!(unwrap_key(&account_key, WrapUse::PrivateKey, &raw, true), Some(key));
        assert_eq!(unwrap_key(&account_key, WrapUse::PrivateKey, &raw, false), None);
    }

    /// Audit 2026-10-19 L1: a co-member names a participant with a key fragment and the victim adds
    /// them as a friend. A JSON label served as the private key opens as neither form.
    #[test]
    fn a_friend_label_served_as_the_private_key_does_not_open() {
        let account_key = generate_key();
        let planted = key_to_fragment(&generate_key());
        let label = encrypt_json(&account_key, &planted).unwrap();

        assert_eq!(open_wrap(&account_key, WrapUse::PrivateKey, &label), None);
    }

    #[test]
    fn a_fingerprint_is_sixteen_stable_base32_characters_in_groups_of_four() {
        let (public, _) = generate_keypair();
        let fp = key_fingerprint(&public);
        let groups: Vec<&str> = fp.split(' ').collect();
        assert_eq!(groups.len(), 4);
        assert!(groups.iter().all(|g| g.len() == 4));
        assert!(fp.replace(' ', "").chars().all(|c| c.is_ascii_uppercase() || ('2'..='7').contains(&c)));
        assert_eq!(fp, key_fingerprint(&public));
        assert_ne!(fp, key_fingerprint(&generate_keypair().0));
    }

    /// The 80 bits are SHA-256's first ten bytes, five bits per character, most significant first.
    #[test]
    fn a_fingerprint_encodes_the_first_80_bits_of_the_digest() {
        let bits: String = Sha256::digest([0u8; 32])[..10].iter().map(|b| format!("{b:08b}")).collect();
        let expected: String = bits
            .as_bytes()
            .chunks(5)
            .map(|five| {
                let index = usize::from_str_radix(std::str::from_utf8(five).unwrap(), 2).unwrap();
                b"ABCDEFGHIJKLMNOPQRSTUVWXYZ234567"[index] as char
            })
            .collect();
        assert_eq!(key_fingerprint(&[0u8; 32]).replace(' ', ""), expected);
    }

    #[test]
    fn a_second_read_of_the_same_row_is_not_decrypted_again() {
        let key = test_key();
        let pair = encrypt(&key, "same row").unwrap();
        let (plain, n) = count_decrypts(|| (decrypt(&key, &pair).unwrap(), decrypt(&key, &pair).unwrap()));
        assert_eq!(plain, ("same row".to_string(), "same row".to_string()));
        assert_eq!(n, 1);
    }

    /// The cache must never answer for something the AEAD check would refuse.
    #[test]
    fn a_cached_row_still_refuses_another_key_or_a_flipped_bit() {
        let key = test_key();
        let pair = encrypt(&key, "secret").unwrap();
        decrypt(&key, &pair).unwrap();

        let mut wrong = key;
        wrong[0] ^= 1;
        assert!(decrypt(&wrong, &pair).is_err());

        let mut ct = STANDARD.decode(&pair.ct).unwrap();
        ct[0] ^= 1;
        let tampered = EncryptedPair { ct: STANDARD.encode(ct), iv: pair.iv.clone() };
        assert!(decrypt(&key, &tampered).is_err());

        let other = encrypt(&key, "other").unwrap();
        let swapped_nonce = EncryptedPair { ct: other.ct, iv: pair.iv.clone() };
        assert!(decrypt(&key, &swapped_nonce).is_err());
    }

    #[test]
    fn forgetting_makes_the_next_read_a_real_decrypt() {
        let key = test_key();
        let pair = encrypt(&key, "row").unwrap();
        let (_, n) = count_decrypts(|| {
            decrypt(&key, &pair).unwrap();
            forget_decrypted();
            decrypt(&key, &pair).unwrap();
        });
        assert_eq!(n, 2);
    }

    /// Unpadded, removing a participant added exactly `,"removed":true` to the ciphertext, and every
    /// name's length was on the server.
    #[test]
    fn a_participant_payload_does_not_tell_its_name_length_or_its_removal() {
        let key = test_key();
        let sealed = |name: &str, removed: bool| {
            encrypt_padded_json(&key, &UserPayload { name: name.into(), removed }).unwrap()
        };
        let lengths = [sealed("Al", false), sealed("Alexandrine-Marie", false), sealed("Al", true)]
            .map(|p| STANDARD.decode(p.ct).unwrap().len());
        assert!(lengths.iter().all(|l| *l == lengths[0]), "{lengths:?}");
        let back: UserPayload = decrypt_json(&key, &sealed("Al", true)).unwrap();
        assert!(back.removed && back.name == "Al");
    }

    /// libsodium's `has_small_order` list (`crypto_scalarmult/curve25519/ref10`): 0, 1, the two
    /// points of order 8, and p - 1, p, p + 1; each also with the ignored top bit set.
    #[test]
    fn a_small_order_public_key_boxes_and_opens_nothing() {
        let mut blacklist = vec![[0u8; 32], { let mut k = [0u8; 32]; k[0] = 1; k }];
        blacklist.push([
            0xe0, 0xeb, 0x7a, 0x7c, 0x3b, 0x41, 0xb8, 0xae, 0x16, 0x56, 0xe3, 0xfa, 0xf1, 0x9f, 0xc4, 0x6a,
            0xda, 0x09, 0x8d, 0xeb, 0x9c, 0x32, 0xb1, 0xfd, 0x86, 0x62, 0x05, 0x16, 0x5f, 0x49, 0xb8, 0x00,
        ]);
        blacklist.push([
            0x5f, 0x9c, 0x95, 0xbc, 0xa3, 0x50, 0x8c, 0x24, 0xb1, 0xd0, 0xb1, 0x55, 0x9c, 0x83, 0xef, 0x5b,
            0x04, 0x44, 0x5c, 0xc4, 0x58, 0x1c, 0x8e, 0x86, 0xd8, 0x22, 0x4e, 0xdd, 0xd0, 0x9f, 0x11, 0x57,
        ]);
        for low in [0xec, 0xed, 0xee] {
            let mut k = [0xffu8; 32];
            (k[0], k[31]) = (low, 0x7f);
            blacklist.push(k);
        }
        let with_top_bit: Vec<[u8; 32]> = blacklist.iter().map(|k| { let mut k = *k; k[31] |= 0x80; k }).collect();

        let (_, mine) = generate_keypair();
        let project_key = generate_key();
        for bad in blacklist.iter().chain(&with_top_bit) {
            assert!(has_small_order(bad), "{bad:02x?}");
            assert_eq!(box_project_key(&mine, bad, &project_key), None);
        }
        for _ in 0..32 {
            assert!(!has_small_order(&generate_keypair().0.try_into().unwrap()));
        }
    }

    /// Below the cap the buckets are `pad_json`'s; the bucket the cap cuts holds every length up to
    /// it; past the cap nothing is added, so the server refuses exactly what it refused unpadded.
    #[test]
    fn a_capped_blob_pads_to_its_bucket_and_never_past_the_cap() {
        let key = test_key();
        let cap = 4096;
        let plain_len = |text: &str| {
            let pair = encrypt_padded_json_within(&key, &text.to_string(), cap).unwrap();
            assert_eq!(decrypt_json::<String>(&key, &pair).unwrap(), text);
            (pair.ct.len(), STANDARD.decode(pair.ct).unwrap().len() - 16)
        };
        assert_eq!(plain_len("a").1, 256);
        assert_eq!(plain_len(&"a".repeat(300)).1, 512);
        assert_eq!(plain_len(&"a".repeat(2100)).1, 3056);
        assert_eq!(plain_len(&"a".repeat(3054)), (cap, 3056));
        assert_eq!(plain_len(&"a".repeat(3100)).1, 3102);
    }

    /// Every wrap written before typed wraps is in this format, and still opens where raw wraps are allowed.
    #[test]
    fn a_stored_wrap_still_opens() {
        let wrapped = EncryptedPair {
            ct: "RveOpblLHkKgFid5xZ0cHtR5kaqWQvHzqCuozAKfXoW+/sNVvKQR/vYp1hOD1iR7QwCbBKmouEXGclU=".into(),
            iv: "8hX+KTHlA5Lnv0BUext3cE2sra5UKNw1".into(),
        };
        assert_eq!(unwrap_key(&[7u8; 32], WrapUse::PrivateKey, &wrapped, true), Some([9u8; 32]));
    }

    /// Two friends read this aloud to each other, so it must not drift between builds.
    #[test]
    fn a_fingerprint_is_unchanged() {
        assert_eq!(key_fingerprint(&[0u8; 32]), "MZUH VLPY MK6X O3EP");
    }

    /// Known answers for the typed wraps (SPEC §16): each pair opens only through the function and
    /// purpose it was written for. A change that breaks one makes stored wraps unreadable.
    #[test]
    fn stored_typed_wraps_still_open() {
        let project = Uuid::parse_str("00000000-0000-0000-0000-000000000001").unwrap();
        let account = Uuid::parse_str("00000000-0000-0000-0000-000000000002").unwrap();
        let pair = |ct: &str, iv: &str| EncryptedPair { ct: ct.into(), iv: iv.into() };

        let escrow = pair(
            "4EgoMjw6glRbGcADHVhU/+gkku6+Ro3JrxFUK4ecPoK87ID40NRoeEdXiUN6rTVR19oKvdoR+/6BRCthI2t7AfKMthCFwp4uyCx42Z56WHj00aPcMZNP7T0dj/XUgjGesMRunQvZ4Utq5+LxnRoBsPwmb7nCn2t6zrEGMa1BOB1kTX1llg==",
            "eFBoH53JsWeknt68EbKTgQIccez8Or4J",
        );
        assert_eq!(unwrap_key(&[7u8; 32], WrapUse::Escrow(project), &escrow, false), Some([9u8; 32]));
        assert_eq!(unwrap_key(&[7u8; 32], WrapUse::PrivateKey, &escrow, true), None, "an escrow row is no private key");

        let private = pair(
            "+MWeqSq7YI3bZMMAl+ZXBjKMk0aW+M+MYBBtt3dFPiKtB2XfqD+ZPLNfdv55BsVTeMihStYuaM66vKeRcCpbwdQTArm7IRzgpqLCgBVb2oet8iva1oYKcg==",
            "FsvOOstyX/DDggzcFwNVBpUDFtLPcPSI",
        );
        assert_eq!(unwrap_key(&[7u8; 32], WrapUse::PrivateKey, &private, false), Some([9u8; 32]));

        let amk = pair(
            "xb/BOCpqw2zNvrInTfvUJN770eHJN7SE98Cz01p0zsPkrd48l/t4CMlPM8//LhXJzfOAzz5ag3fY+Nkla1CB9MwsQHyRxdcOnBQbQzdBgYXU1I0680R05ks0btwlDTWSBeAjGwWFgO/AKqghWLGVYGx83BDmvLUqsL7wnS9n6fKs1AceHs8UcO0B484=",
            "j9A9HhA57ntuS+Sx2HGUo9NoU44KFcsg",
        );
        assert_eq!(open_amk(&[7u8; 32], account, 1, &amk), Some([9u8; 32]));
        assert_eq!(open_amk(&[7u8; 32], account, 2, &amk), None, "another epoch");

        let recovery = pair(
            "/K2Bfcg8Dm4DBQsep1jzC8TsTQkV+qRQydmDeU20SpjdSObcaNBqTlNrEFJ3jAlg7wBqUPDoDzKcu7fXYjYEYL3L+OTPKSVbpr+74vU1kYflNDcLjbgJLurDvlHakh4Bp6t4D7F0EtsDN3QmyjM6M0qnOWIkO/ua/npw8h1Fr5/wmkX7OeSI2Qukfg==",
            "b+9DTnEXIF3segtubW/ohw6O1nVTqp9n",
        );
        assert_eq!(open_recovery(&[7u8; 32], account, &recovery), Some([9u8; 32]));

        let kem = pair(
            "cyUY9NDec3e+y7oXyzeyK6JXGVngewz8lRc5e+BHU/BEd9izvss0ctSVod321pZ9M6ctRzsId73nyD06x81YJ1jx1k3tn7e7vagHlqpE0WgWGWxRP0Rg54Sf3yQ2Nm8VC5FB9jB169O0622wxq6/FT6BPsB8mxunq+8x8HMKbrVvydU/",
            "R0pZGQ7YYkGUZhQs9cUqzUdBWEndopkT",
        );
        assert_eq!(open_kem_seed(&[7u8; 32], &kem), Some([7u8; 64]));
    }

    #[test]
    fn a_short_salt_from_the_server_is_an_error_not_a_panic() {
        for salt in [&[][..], &[1u8; 7][..]] {
            assert!(derive_account_key_v1("password", salt).is_err());
            assert!(derive_login_proof(shared::AUTH_VERSION_CURRENT, "password", salt).is_err());
            assert!(derive_login_proof(shared::AUTH_VERSION_LEGACY, "password", salt).is_err());
        }
    }

    #[test]
    #[ignore = "timing: cargo test -p ui --release -- --ignored --nocapture"]
    fn kdf_timing() {
        let t = std::time::Instant::now();
        derive_account_key_v1("password", &[0x11u8; 16]).unwrap();
        let account_key = t.elapsed();
        let t = std::time::Instant::now();
        derive_login_proof(shared::AUTH_VERSION_CURRENT, "password", &[0x11u8; 16]).unwrap();
        println!("argon2id account key {account_key:?}, login proof {:?}", t.elapsed());
    }

    #[test]
    fn generate_kdf_salt_is_random() {
        let s1 = generate_kdf_salt();
        let s2 = generate_kdf_salt();
        assert_ne!(s1, s2);
    }
}
