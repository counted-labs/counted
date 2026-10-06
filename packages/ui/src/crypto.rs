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
use serde::{de::DeserializeOwned, Serialize};
use sha2::{Digest, Sha256};
use shared::{Account, EncryptedPair};

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

pub fn encrypt(key: &[u8; 32], plaintext: &str) -> Result<EncryptedPair, String> {
    seal(&XChaCha20Poly1305::new(key.into()), plaintext.as_bytes())
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

pub fn decrypt(key: &[u8; 32], pair: &EncryptedPair) -> Result<String, String> {
    #[cfg(test)]
    DECRYPT_COUNT.with(|c| c.set(c.get() + 1));

    let plain = open(&XChaCha20Poly1305::new(key.into()), pair)?;
    String::from_utf8(plain).map_err(|e| format!("utf8: {e}"))
}

pub fn encrypt_json<T: Serialize>(key: &[u8; 32], value: &T) -> Result<EncryptedPair, String> {
    let json = serde_json::to_string(value).map_err(|e| e.to_string())?;
    encrypt(key, &json)
}

pub fn decrypt_json<T: DeserializeOwned>(key: &[u8; 32], pair: &EncryptedPair) -> Result<T, String> {
    serde_json::from_str(&decrypt(key, pair)?).map_err(|e| e.to_string())
}

/// A project key or the private key, encrypted under the account key. Stored as its fragment, so an
/// escrowed project key is byte-identical to what a share link carries — see `docs/e2ee.md`,
/// "Key escrow".
pub fn wrap_key(account_key: &[u8; 32], key: &[u8; 32]) -> Option<EncryptedPair> {
    encrypt(account_key, &key_to_fragment(key)).ok()
}

pub fn unwrap_key(account_key: &[u8; 32], wrapped: &EncryptedPair) -> Option<[u8; 32]> {
    decrypt(account_key, wrapped).ok().and_then(|fragment| key_from_fragment(&fragment).ok())
}

/// The account's `display_name`, re-encrypted under the project key so the other members can see
/// who claimed an identity — see `docs/e2ee.md`, "The claim label". `None` when the name does not
/// decrypt under `account_key`.
pub fn claim_label(
    account: &Account,
    account_key: &[u8; 32],
    project_key: &[u8; 32],
) -> Option<EncryptedPair> {
    let name: String = decrypt_json(account_key, &account.display_name).ok()?;
    encrypt_json(project_key, &name).ok()
}

/// Proof of holding a project key, for identity claims — see `docs/e2ee.md`, "The claim token".
/// A fixed label in front of a fixed-length secret, so plain SHA-256 needs no MAC construction.
pub fn claim_token(project_key: &[u8; 32]) -> [u8; 32] {
    Sha256::new().chain_update(b"counted-claim-v1").chain_update(project_key).finalize().into()
}

/// What the server stores and compares a presented token against.
pub fn claim_verifier(project_key: &[u8; 32]) -> [u8; 32] {
    Sha256::digest(claim_token(project_key)).into()
}

/// The account's X25519 keypair (public, private) — see `docs/e2ee.md`, "Account keypair and boxed
/// invitations".
pub fn generate_keypair() -> (Vec<u8>, [u8; 32]) {
    let secret = crypto_box::SecretKey::from_bytes(generate_key());
    (secret.public_key().to_bytes().to_vec(), secret.to_bytes())
}

fn chacha_box(my_private: &[u8; 32], their_public: &[u8]) -> Option<crypto_box::ChaChaBox> {
    let public = crypto_box::PublicKey::from_slice(their_public).ok()?;
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
/// within reach of a server grinding a lookalike key — see `docs/e2ee.md`.
pub fn key_fingerprint(public_key: &[u8]) -> String {
    const ALPHABET: &[u8; 32] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ234567";
    let digest = Sha256::digest(public_key);
    let bits = digest[..10].iter().fold(0u128, |acc, byte| (acc << 8) | u128::from(*byte));
    let chars: Vec<char> =
        (0..16).rev().map(|i| ALPHABET[((bits >> (i * 5)) & 31) as usize] as char).collect();
    chars.chunks(4).map(|group| group.iter().collect::<String>()).collect::<Vec<_>>().join(" ")
}

/// OWASP "preferred" interactive Argon2id: m = 64 MiB, t = 3, p = 1, 32-byte output.
fn account_kdf_params() -> Params {
    Params::new(65_536, 3, 1, Some(32)).expect("valid argon2 params")
}

fn argon2_hash(argon2: &Argon2, password: &str, salt: &[u8]) -> [u8; 32] {
    let mut out = [0u8; 32];
    argon2.hash_password_into(password.as_bytes(), salt, &mut out).expect("argon2 hash");
    out
}

pub fn derive_account_key_v1(password: &str, salt: &[u8]) -> [u8; 32] {
    argon2_hash(&Argon2::new(Algorithm::Argon2id, Version::V0x13, account_kdf_params()), password, salt)
}

/// What the server is sent instead of the password — see `docs/auth.md`, "Login proof".
///
/// - legacy: `Argon2::default()`, exactly what the server used to run on the plaintext, so the
///   stored hash verifies it without re-enrolment;
/// - current: the account-key parameters keyed with a fixed secret, so the proof is a different
///   function of the password than the account key even under an equal salt.
pub fn derive_login_proof(auth_version: i16, password: &str, salt: &[u8]) -> [u8; 32] {
    let argon2 = if auth_version == shared::AUTH_VERSION_LEGACY {
        Argon2::new(Algorithm::Argon2id, Version::V0x13, Params::default())
    } else {
        Argon2::new_with_secret(b"counted-login-v2", Algorithm::Argon2id, Version::V0x13, account_kdf_params())
            .expect("valid argon2 secret")
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

    #[test]
    fn encrypt_decrypt_roundtrip() {
        let key = test_key();
        for s in ["hello world", "", "Déjeuner à Paris — 日本語 🍜"] {
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
            STANDARD.encode(derive_account_key_v1("password", &[0x11u8; 16])),
            "/SPWqp3PDq/+ghu8SN2qps4tQD8ZZkfH/Isz8fQXok8="
        );
        assert_eq!(
            STANDARD.encode(derive_login_proof(shared::AUTH_VERSION_CURRENT, "password", &[0x11u8; 16])),
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
        let wrapped = wrap_key(&account_key, &private).unwrap();
        assert_eq!(unwrap_key(&account_key, &wrapped), Some(private));
        assert_eq!(unwrap_key(&generate_key(), &wrapped), None);
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
    fn generate_kdf_salt_is_random() {
        let s1 = generate_kdf_salt();
        let s2 = generate_kdf_salt();
        assert_ne!(s1, s2);
    }
}
