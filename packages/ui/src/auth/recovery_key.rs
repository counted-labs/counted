//! The recovery key (docs/plans/key-lifecycle.md §6.1): 256 random bits, shown as 52 base32
//! characters in 13 groups of four like friend codes. Two secrets come from it, each salted with the
//! account id so no search runs across every account at once: `rk_key` wraps the master key, and
//! `auth` proves to the server that a caller holds the key without giving it `rk_key`.

use hkdf::Hkdf;
use sha2::{Digest, Sha256};
use uuid::Uuid;

pub const KEY_BYTES: usize = 32;
const ALPHABET: &[u8; 32] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ234567";
const CHARS: usize = 52;

pub fn generate() -> [u8; KEY_BYTES] {
    crate::crypto::generate_key()
}

pub fn encode(key: &[u8; KEY_BYTES]) -> String {
    let mut bits = 0u32;
    let mut held = 0u32;
    let mut chars = String::with_capacity(CHARS + 12);
    let push = |c: u8, chars: &mut String| {
        if !chars.is_empty() && chars.chars().filter(|c| *c != ' ').count() % 4 == 0 {
            chars.push(' ');
        }
        chars.push(ALPHABET[c as usize] as char);
    };
    for byte in key {
        held = (held << 8) | u32::from(*byte);
        bits += 8;
        while bits >= 5 {
            bits -= 5;
            push(((held >> bits) & 31) as u8, &mut chars);
        }
    }
    push(((held << (5 - bits)) & 31) as u8, &mut chars);
    chars
}

/// Forgiving about what a person types back - case, spaces, dashes, and 0/1 for O/I, which the
/// alphabet does not use - and strict about everything else, the four padding bits included.
pub fn parse(text: &str) -> Option<[u8; KEY_BYTES]> {
    let symbols: Vec<u8> = text
        .chars()
        .filter(|c| !c.is_whitespace() && *c != '-')
        .map(|c| match c.to_ascii_uppercase() {
            '0' => 'O',
            '1' => 'I',
            other => other,
        })
        .map(|c| ALPHABET.iter().position(|a| *a as char == c).map(|i| i as u8))
        .collect::<Option<_>>()?;
    if symbols.len() != CHARS {
        return None;
    }
    let mut key = [0u8; KEY_BYTES];
    let (mut held, mut bits, mut at) = (0u32, 0u32, 0usize);
    for symbol in symbols {
        held = (held << 5) | u32::from(symbol);
        bits += 5;
        if bits >= 8 {
            bits -= 8;
            if at < KEY_BYTES {
                key[at] = (held >> bits) as u8;
            }
            at += 1;
        }
    }
    (held & ((1 << bits) - 1) == 0).then_some(key)
}

/// The groups a person is asked to type back, by position.
pub fn group(text: &str, index: usize) -> Option<&str> {
    text.split(' ').nth(index)
}

pub struct Derived {
    pub rk_key: [u8; 32],
    pub auth: [u8; 32],
}

pub fn derive(key: &[u8; KEY_BYTES], account: Uuid) -> Derived {
    let hkdf = Hkdf::<Sha256>::new(Some(account.as_bytes()), key);
    let (mut rk_key, mut auth) = ([0u8; 32], [0u8; 32]);
    hkdf.expand(b"counted-recovery-v1", &mut rk_key).expect("32 bytes is a valid HKDF-SHA256 length");
    hkdf.expand(b"counted-recovery-auth-v1", &mut auth).expect("32 bytes is a valid HKDF-SHA256 length");
    Derived { rk_key, auth }
}

/// What the server stores and compares `SHA-256(auth)` against.
pub fn verifier(auth: &[u8; 32]) -> Vec<u8> {
    Sha256::digest(auth).to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_key_reads_back_as_52_characters_in_13_groups() {
        let key = generate();
        let text = encode(&key);
        assert_eq!(text.split(' ').count(), 13);
        assert!(text.split(' ').all(|g| g.len() == 4), "{text}");
        assert_eq!(parse(&text), Some(key));
    }

    #[test]
    fn typing_mistakes_that_cannot_be_ambiguous_are_forgiven() {
        let key = [0xA5u8; 32];
        let text = encode(&key);
        assert_eq!(parse(&text.to_lowercase()), Some(key));
        assert_eq!(parse(&text.replace(' ', "-")), Some(key));
        assert_eq!(parse(&text.replace(' ', "")), Some(key));
        assert_eq!(parse(&text.replace('O', "0").replace('I', "1")), Some(key));
    }

    #[test]
    fn anything_else_is_refused() {
        let text = encode(&[7u8; 32]);
        assert_eq!(parse(&text[..text.len() - 1]), None, "a character short");
        assert_eq!(parse(&format!("{text}A")), None, "a character long");
        assert_eq!(parse(&text.replacen(|c: char| c.is_ascii_uppercase(), "8", 1)), None, "outside the alphabet");
        let last = text.chars().last().unwrap();
        let padded = format!("{}{}", &text[..text.len() - 1], if last == 'B' { 'C' } else { 'B' });
        assert!(parse(&padded).is_none_or(|k| k != [7u8; 32]), "a padding bit set");
    }

    /// Known answer (SPEC §16): a change to the derivation would orphan every recovery key set.
    #[test]
    fn the_derivation_matches_its_known_answer() {
        use base64::{engine::general_purpose::STANDARD, Engine as _};
        let derived = derive(&[1u8; 32], Uuid::from_u128(2));
        assert_eq!(STANDARD.encode(derived.rk_key), "7WPuQhHnKOkTShIg+MR9dnfOt/h2/B/EHoEvF3NzZOQ=");
        assert_eq!(STANDARD.encode(derived.auth), "oWpyXzGTisnU5tMSLfOwkR9anADMPZlke6oqzYeNlUw=");
        assert_ne!(derived.rk_key, derive(&[1u8; 32], Uuid::from_u128(3)).rk_key, "salted by account");
    }
}
