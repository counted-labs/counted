//! Hybrid X25519 + ML-KEM-768 boxes and each account's ML-KEM key (docs/plans/key-lifecycle.md §7).
//!
//! The sender draws an ephemeral X25519 key and an ML-KEM encapsulation for the recipient's two
//! public keys. Both shared secrets go through HKDF-SHA256 with both ciphertexts (the ephemeral
//! public key and the ML-KEM ciphertext) and both recipient public keys in its info, so a server
//! that substitutes one half cannot move the other; the result keys XChaCha20-Poly1305 with the
//! context as associated data. The box is as strong as the stronger of the two.
//!
//! ML-KEM keys come from a 64-byte seed and encapsulation from a 32-byte random message, both from
//! the getrandom 0.2 every other key here comes from.

use chacha20poly1305::{
    aead::{Aead, KeyInit, Payload},
    XChaCha20Poly1305,
};
use curve25519_dalek::montgomery::MontgomeryPoint;
use hkdf::Hkdf;
use ml_kem::array::Array;
use ml_kem::ml_kem_768::{Ciphertext, DecapsulationKey, EncapsulationKey};
use ml_kem::{Decapsulate, KeyExport};
use sha2::{Digest, Sha256};
use shared::HybridBox;

pub const KEM_SEED_LENGTH: usize = 64;

fn random<const N: usize>() -> [u8; N] {
    let mut bytes = [0u8; N];
    getrandom::getrandom(&mut bytes).expect("no CSPRNG available");
    bytes
}

pub fn generate_kem_seed() -> [u8; KEM_SEED_LENGTH] {
    random()
}

/// The ML-KEM-768 encapsulation key a seed expands to, as served in `accounts.kem_public_key`.
pub fn kem_public_key(seed: &[u8; KEM_SEED_LENGTH]) -> Vec<u8> {
    DecapsulationKey::from_seed(Array::from(*seed)).encapsulation_key().to_bytes().to_vec()
}

/// X25519 refuses what libsodium refuses: a public key of small order, whose shared secret is one
/// of at most 8 values.
fn x25519(secret: [u8; 32], public: &[u8]) -> Option<[u8; 32]> {
    let public: [u8; 32] = public.try_into().ok()?;
    let point = MontgomeryPoint(public);
    if (point * curve25519_dalek::scalar::Scalar::from(8u8)).to_bytes() == [0u8; 32] {
        return None;
    }
    Some(point.mul_clamped(secret).to_bytes())
}

fn combine(
    x25519_secret: &[u8; 32],
    kem_secret: &[u8],
    ephemeral: &[u8],
    to_x25519: &[u8],
    kem_ct: &[u8],
    to_kem: &[u8],
    context: &[u8],
) -> [u8; 32] {
    let info = [
        b"counted-hybrid-v1".as_slice(),
        ephemeral,
        to_x25519,
        Sha256::digest(kem_ct).as_slice(),
        Sha256::digest(to_kem).as_slice(),
        context,
    ]
    .concat();
    let mut key = [0u8; 32];
    Hkdf::<Sha256>::new(None, &[kem_secret, x25519_secret.as_slice()].concat())
        .expand(&info, &mut key)
        .expect("32 bytes is a valid HKDF-SHA256 length");
    key
}

fn seal_with(
    to_x25519: &[u8],
    to_kem: &[u8],
    context: &[u8],
    plaintext: &[u8],
    ephemeral_secret: [u8; 32],
    message: [u8; 32],
    nonce: [u8; 24],
) -> Option<HybridBox> {
    let kem_key = EncapsulationKey::new(to_kem.try_into().ok()?).ok()?;
    let (kem_ct, kem_secret) = kem_key.encapsulate_deterministic(&Array::from(message));
    let ephemeral = MontgomeryPoint::mul_base_clamped(ephemeral_secret).to_bytes();
    let x25519_secret = x25519(ephemeral_secret, to_x25519)?;
    let key = combine(&x25519_secret, &kem_secret, &ephemeral, to_x25519, &kem_ct, to_kem, context);
    let ct = XChaCha20Poly1305::new(&key.into())
        .encrypt(&nonce.into(), Payload { msg: plaintext, aad: context })
        .ok()?;
    Some(HybridBox { ephemeral: ephemeral.to_vec(), kem_ct: kem_ct.to_vec(), nonce: nonce.to_vec(), ct })
}

/// `plaintext` sealed to the holder of both private keys, bound to `context`.
pub fn seal(to_x25519: &[u8], to_kem: &[u8], context: &[u8], plaintext: &[u8]) -> Option<HybridBox> {
    seal_with(to_x25519, to_kem, context, plaintext, random(), random(), random())
}

/// `None` unless the box was sealed to these keys under this context and is untouched.
pub fn open(
    my_x25519: &[u8; 32],
    my_kem_seed: &[u8; KEM_SEED_LENGTH],
    context: &[u8],
    sealed: &HybridBox,
) -> Option<Vec<u8>> {
    let kem = DecapsulationKey::from_seed(Array::from(*my_kem_seed));
    let to_kem = kem.encapsulation_key().to_bytes().to_vec();
    let to_x25519 = MontgomeryPoint::mul_base_clamped(*my_x25519).to_bytes();
    let kem_ct = Ciphertext::try_from(sealed.kem_ct.as_slice()).ok()?;
    let kem_secret = kem.decapsulate(&kem_ct);
    let x25519_secret = x25519(*my_x25519, &sealed.ephemeral)?;
    let key = combine(&x25519_secret, &kem_secret, &sealed.ephemeral, &to_x25519, &sealed.kem_ct, &to_kem, context);
    let nonce: [u8; 24] = sealed.nonce.as_slice().try_into().ok()?;
    XChaCha20Poly1305::new(&key.into()).decrypt(&nonce.into(), Payload { msg: &sealed.ct, aad: context }).ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn recipient() -> ([u8; 32], [u8; 64], Vec<u8>, Vec<u8>) {
        let (x, seed) = (random::<32>(), generate_kem_seed());
        let x_pub = MontgomeryPoint::mul_base_clamped(x).to_bytes().to_vec();
        (x, seed, x_pub, kem_public_key(&seed))
    }

    #[test]
    fn a_box_opens_for_its_recipient_and_context_only() {
        let (x, seed, x_pub, kem_pub) = recipient();
        let sealed = seal(&x_pub, &kem_pub, b"ctx", b"project key").unwrap();

        assert_eq!(open(&x, &seed, b"ctx", &sealed).as_deref(), Some(b"project key".as_slice()));
        assert_eq!(open(&x, &seed, b"other", &sealed), None, "another context");
        let (x2, seed2, _, _) = recipient();
        assert_eq!(open(&x2, &seed, b"ctx", &sealed), None, "another X25519 key");
        assert_eq!(open(&x, &seed2, b"ctx", &sealed), None, "another ML-KEM key");
    }

    /// Plan §7: the combiner binds both halves, so a box sealed to a substituted ML-KEM key with
    /// the right X25519 key does not open for the real recipient, and every ciphertext byte counts.
    #[test]
    fn neither_half_can_be_swapped_or_altered() {
        let (x, seed, x_pub, kem_pub) = recipient();
        let (_, _, _, forged_kem) = recipient();
        let substituted = seal(&x_pub, &forged_kem, b"ctx", b"k").unwrap();
        assert_eq!(open(&x, &seed, b"ctx", &substituted), None);

        let sealed = seal(&x_pub, &kem_pub, b"ctx", b"k").unwrap();
        let mut tampered = sealed.clone();
        tampered.kem_ct[0] ^= 1;
        assert_eq!(open(&x, &seed, b"ctx", &tampered), None, "ML-KEM ciphertext");
        let mut tampered = sealed.clone();
        tampered.ephemeral[0] ^= 1;
        assert_eq!(open(&x, &seed, b"ctx", &tampered), None, "ephemeral key");
        let mut tampered = sealed;
        tampered.ct[0] ^= 1;
        assert_eq!(open(&x, &seed, b"ctx", &tampered), None, "ciphertext");
    }

    #[test]
    fn a_small_order_recipient_key_is_refused() {
        let (_, _, _, kem_pub) = recipient();
        assert!(seal(&[0u8; 32], &kem_pub, b"ctx", b"k").is_none());
        assert!(seal(&[1u8; 31], &kem_pub, b"ctx", b"k").is_none());
        assert!(seal(&random::<32>(), &kem_pub[..100], b"ctx", b"k").is_none());
    }

    /// Known answer (SPEC §16): fixed keys, randomness, context and plaintext give a fixed box. A
    /// change to the combiner, the KEM or the AEAD changes this digest.
    #[test]
    fn the_construction_matches_its_known_answer() {
        let seed = [7u8; 64];
        let x = [9u8; 32];
        let x_pub = MontgomeryPoint::mul_base_clamped(x).to_bytes();
        let sealed = seal_with(&x_pub, &kem_public_key(&seed), b"counted-kat", b"kat plaintext", [3u8; 32], [4u8; 32], [5u8; 24]).unwrap();
        let digest = Sha256::new()
            .chain_update(&sealed.ephemeral)
            .chain_update(&sealed.kem_ct)
            .chain_update(&sealed.nonce)
            .chain_update(&sealed.ct)
            .finalize();
        assert_eq!(base64::Engine::encode(&base64::engine::general_purpose::STANDARD, digest), "EsGlmnmZZnSBMZIDE+tgmIyLtUnNt0KDn0XKwQngE4g=");
        assert_eq!(open(&x, &seed, b"counted-kat", &sealed).as_deref(), Some(b"kat plaintext".as_slice()));
    }
}
