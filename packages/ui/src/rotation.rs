//! The pure half of a project key rotation (docs/plans/key-lifecycle.md §8.3-8.6): what the new
//! project row says about the key it replaces, the member key directory, the boxes, and which key
//! a device may switch to. The network half is `rotation_flow`.

use base64::{engine::general_purpose::STANDARD as B64, Engine as _};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha2::{Digest, Sha256};
use shared::{EncryptedPair, HybridBox, ProjectDto, RotationMember};
use uuid::Uuid;

use crate::crypto::{decrypt, decrypt_bytes, decrypt_json, encrypt, encrypt_bytes, encrypt_json};

pub const ROTATED_FROM: &str = "rotatedFrom";
const BOX_CONTEXT: &[u8] = b"counted-rotate-v1";

/// `SHA-256("counted-rotated-from-v1" || K)`: names an earlier key without giving it to later
/// joiners, and without the claim token, which the server sees.
pub fn rotated_from_hash(key: &[u8; 32]) -> String {
    B64.encode(Sha256::new().chain_update(b"counted-rotated-from-v1").chain_update(key).finalize())
}

/// The new project payload: the old one, field for field, with `H(K1)` at the head of the chain.
/// The one row a rotation does not re-seal byte for byte.
pub fn chained_payload(k1: &[u8; 32], plaintext: &str) -> Result<String, String> {
    let mut payload: Value = serde_json::from_str(plaintext).map_err(|e| e.to_string())?;
    let object = payload.as_object_mut().ok_or("project payload is not an object")?;
    let mut chain = vec![Value::String(rotated_from_hash(k1))];
    if let Some(Value::Array(earlier)) = object.get(ROTATED_FROM) {
        chain.extend(earlier.iter().cloned());
    }
    object.insert(ROTATED_FROM.into(), Value::Array(chain));
    Ok(shared::pad_json(serde_json::to_string(&payload).map_err(|e| e.to_string())?))
}

fn chain(key: &[u8; 32], project: &ProjectDto) -> Option<Vec<String>> {
    let payload: Value = serde_json::from_str(&decrypt(key, &project.payload).ok()?).ok()?;
    Some(
        payload
            .get(ROTATED_FROM)
            .and_then(Value::as_array)
            .map(|a| a.iter().filter_map(|h| h.as_str().map(str::to_string)).collect())
            .unwrap_or_default(),
    )
}

/// How many keys the row `key` opens says it replaced: one hash per rotation, so the length of the
/// chain is the row's key epoch as the ciphertext itself states it, which the clear `key_epoch`
/// column does not. A row that does not open, or has no chain, counts 0.
pub fn chain_len(key: &[u8; 32], project: &ProjectDto) -> usize {
    chain(key, project).map_or(0, |c| c.len())
}

/// §8.4: a device replaces the key it holds by `candidate` only if `candidate` opens the served
/// project row, that row names the held key as one it replaced, and its chain is longer than any
/// this device has opened for the project (`chain_seen`): a kept box or escrow row of an epoch the
/// device has moved past does not roll it back (security review 2026-11-02 L1). A hostile server
/// alone cannot build such a row; it needs `H(held)`, which only holders of the old or new key know.
pub fn may_switch(held: &[u8; 32], candidate: &[u8; 32], served: &ProjectDto, chain_seen: usize) -> bool {
    held != candidate
        && chain(candidate, served).is_some_and(|c| c.len() > chain_seen && c.contains(&rotated_from_hash(held)))
}

/// A row re-sealed under the new key, or `None` when its tag does not check under the old one.
pub fn reseal(k1: &[u8; 32], k2: &[u8; 32], pair: &EncryptedPair) -> Option<EncryptedPair> {
    let plain = decrypt_bytes(k1, pair).ok()?;
    encrypt_bytes(k2, &plain).ok()
}

#[derive(Serialize, Deserialize)]
struct MemberKey {
    #[serde(rename = "use")]
    use_: String,
    account: Uuid,
    x25519: String,
    kem: String,
}

/// This member's directory entry: their hybrid public key, vouched for by the group's key.
pub fn member_key_entry(project_key: &[u8; 32], account: Uuid, x25519: &[u8], kem: &[u8]) -> Option<EncryptedPair> {
    let entry = MemberKey { use_: "member-key".into(), account, x25519: B64.encode(x25519), kem: B64.encode(kem) };
    encrypt_json(project_key, &entry).ok()
}

/// The keys to box to, only when the entry opens under the group's key, names this account, and
/// matches both halves the server serves for it.
pub fn recipient_keys(project_key: &[u8; 32], member: &RotationMember) -> Option<(Vec<u8>, Vec<u8>)> {
    let entry: MemberKey = decrypt_json(project_key, member.member_key.as_ref()?).ok()?;
    let (x25519, kem) = (B64.decode(&entry.x25519).ok()?, B64.decode(&entry.kem).ok()?);
    let served = (member.public_key.as_deref()?, member.kem_public_key.as_deref()?);
    (entry.use_ == "member-key" && entry.account == member.account_id && served == (x25519.as_slice(), kem.as_slice()))
        .then_some((x25519, kem))
}

#[derive(Serialize, Deserialize)]
struct BoxedKey {
    #[serde(rename = "use")]
    use_: String,
    project: Uuid,
    to: Uuid,
    key_epoch: i32,
    key: String,
}

pub fn seal_box(x25519: &[u8], kem: &[u8], project: Uuid, to: Uuid, key_epoch: i32, key: &[u8; 32]) -> Option<HybridBox> {
    let plain = serde_json::to_vec(&BoxedKey { use_: "rotate".into(), project, to, key_epoch, key: B64.encode(key) }).ok()?;
    crate::hybrid::seal(x25519, kem, BOX_CONTEXT, &plain)
}

/// Opened only if every field matches: a box for another project, account or epoch is refused.
pub fn open_box(
    my_x25519: &[u8; 32],
    my_kem_seed: &[u8; crate::hybrid::KEM_SEED_LENGTH],
    project: Uuid,
    me: Uuid,
    rotation_box: &shared::RotationBox,
) -> Option<[u8; 32]> {
    let plain = crate::hybrid::open(my_x25519, my_kem_seed, BOX_CONTEXT, &rotation_box.sealed)?;
    let boxed: BoxedKey = serde_json::from_slice(&plain).ok()?;
    let fits = boxed.use_ == "rotate" && boxed.project == project && boxed.to == me && boxed.key_epoch == rotation_box.key_epoch;
    if !fits || rotation_box.project_id != project {
        return None;
    }
    B64.decode(boxed.key).ok()?.try_into().ok()
}

/// Every sealed pair inside a queued op, re-sealed under the new key, so ops queued before a switch
/// replay under it (§8.6). A pair that does not open under the old key is left as it is.
pub fn rekey_value(value: &mut Value, k1: &[u8; 32], k2: &[u8; 32]) {
    match value {
        Value::Object(map) => {
            let pair = match (map.get("ct"), map.get("iv")) {
                (Some(Value::String(ct)), Some(Value::String(iv))) if map.len() == 2 => {
                    Some(EncryptedPair { ct: ct.clone(), iv: iv.clone() })
                }
                _ => None,
            };
            match pair.and_then(|p| reseal(k1, k2, &p)) {
                Some(new) => {
                    map.insert("ct".into(), Value::String(new.ct));
                    map.insert("iv".into(), Value::String(new.iv));
                }
                None => map.values_mut().for_each(|v| rekey_value(v, k1, k2)),
            }
        }
        Value::Array(items) => items.iter_mut().for_each(|v| rekey_value(v, k1, k2)),
        _ => {}
    }
}

/// D13: the participant row of someone removed from the project, hidden under the new key, every
/// other field kept. `None` when it does not open under the old key or is no JSON object.
pub fn reseal_removed(k1: &[u8; 32], k2: &[u8; 32], pair: &EncryptedPair) -> Option<EncryptedPair> {
    let mut payload: Value = serde_json::from_str(&decrypt(k1, pair).ok()?).ok()?;
    payload.as_object_mut()?.insert("removed".into(), Value::Bool(true));
    encrypt(k2, &shared::pad_json(serde_json::to_string(&payload).ok()?)).ok()
}

/// What a row is staged as: re-sealed byte for byte, except the participant row a removal hides.
/// That one falls back to the byte-exact re-seal when it is no object, so it is never set aside.
pub fn restage(table: &str, id: &str, hidden: Option<i32>, k1: &[u8; 32], k2: &[u8; 32], pair: &EncryptedPair) -> Option<EncryptedPair> {
    let hides = table == "users" && hidden.is_some_and(|h| h.to_string() == id);
    match hides {
        true => reseal_removed(k1, k2, pair).or_else(|| reseal(k1, k2, pair)),
        false => reseal(k1, k2, pair),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypto::{encrypt_padded_json, generate_key};
    use chrono::NaiveDateTime;

    fn project(key: &[u8; 32], plaintext: &str) -> ProjectDto {
        ProjectDto {
            id: Uuid::nil(),
            payload: encrypt(key, plaintext).unwrap(),
            status: shared::ProjectStatus::Ongoing,
            created_at: NaiveDateTime::default(),
            has_admin: true,
            key_lifecycle: true,
            read_only: false,
            key_epoch: 1,
            has_creator_proof: false,
        }
    }

    #[test]
    fn the_known_answer_for_rotated_from_is_pinned() {
        assert_eq!(rotated_from_hash(&[7u8; 32]), "qlxSNkAhIGk5XXSR7+HWwqugI9v848gRMGdmdZt52UA=");
    }

    /// SPEC §16: a stored directory entry still checks, and the rotation plaintext keeps its shape.
    #[test]
    fn the_member_key_entry_and_the_rotation_plaintext_are_pinned() {
        let project = Uuid::parse_str("00000000-0000-0000-0000-000000000001").unwrap();
        let account = Uuid::parse_str("00000000-0000-0000-0000-000000000002").unwrap();
        let entry = EncryptedPair { ct: MEMBER_KEY_KAT.into(), iv: "61ArW+WvBxSCi8QNg3nAOVScEustdYN6".into() };
        let plain = decrypt(&[7u8; 32], &entry).unwrap();
        assert!(plain.starts_with(r#"{"use":"member-key","account":"00000000-0000-0000-0000-000000000002","x25519":"AQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQE=","kem":"AgIC"#));
        let member = RotationMember {
            account_id: account,
            user_id: None,
            public_key: Some(vec![1u8; 32]),
            kem_public_key: Some(vec![2u8; 1184]),
            member_key: Some(entry),
        };
        assert_eq!(recipient_keys(&[7u8; 32], &member), Some((vec![1u8; 32], vec![2u8; 1184])));

        let boxed = BoxedKey { use_: "rotate".into(), project, to: account, key_epoch: 1, key: B64.encode([7u8; 32]) };
        assert_eq!(
            serde_json::to_string(&boxed).unwrap(),
            r#"{"use":"rotate","project":"00000000-0000-0000-0000-000000000001","to":"00000000-0000-0000-0000-000000000002","key_epoch":1,"key":"BwcHBwcHBwcHBwcHBwcHBwcHBwcHBwcHBwcHBwcHBwc="}"#
        );
    }

    const MEMBER_KEY_KAT: &str = "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";

    #[test]
    fn a_chained_payload_keeps_every_field_and_prepends_the_old_key() {
        let (k0, k1) = (generate_key(), generate_key());
        let plain = serde_json::json!({"name": "Trip", "currency": "EUR", "futureField": [1, 2], ROTATED_FROM: [rotated_from_hash(&k0)]}).to_string();
        let chained: Value = serde_json::from_str(&chained_payload(&k1, &plain).unwrap()).unwrap();
        assert_eq!(chained["name"], "Trip");
        assert_eq!(chained["futureField"], serde_json::json!([1, 2]));
        assert_eq!(chained[ROTATED_FROM], serde_json::json!([rotated_from_hash(&k1), rotated_from_hash(&k0)]));
    }

    #[test]
    fn a_device_switches_only_to_a_key_whose_row_names_the_one_it_holds() {
        let (k1, k2, stranger) = (generate_key(), generate_key(), generate_key());
        let good = project(&k2, &chained_payload(&k1, r#"{"name":"Trip"}"#).unwrap());
        assert!(may_switch(&k1, &k2, &good, 0));
        assert!(!may_switch(&stranger, &k2, &good, 0), "a chain missing the held key");
        let unchained = project(&k2, r#"{"name":"Trip"}"#);
        assert!(!may_switch(&k1, &k2, &unchained, 0), "a row without rotatedFrom");
        assert!(!may_switch(&k1, &stranger, &good, 0), "a key that does not open the row");
        let junk = ProjectDto { payload: EncryptedPair { ct: "AAAA".into(), iv: B64.encode([0u8; 24]) }, ..good.clone() };
        assert!(!may_switch(&k1, &k2, &junk, 0));
    }

    /// Security review 2026-11-02 L1: a device that has opened epoch n refuses a kept box or
    /// escrow row of an earlier epoch, and may skip epochs forward.
    #[test]
    fn a_switch_only_moves_to_a_longer_chain_than_the_device_has_opened() {
        let (k0, k1, k2, k3) = (generate_key(), generate_key(), generate_key(), generate_key());
        let epoch1 = chained_payload(&k0, r#"{"name":"Trip"}"#).unwrap();
        let epoch2 = chained_payload(&k1, &epoch1).unwrap();
        let epoch3 = chained_payload(&k2, &epoch2).unwrap();
        let (row1, row2, row3) = (project(&k1, &epoch1), project(&k2, &epoch2), project(&k3, &epoch3));
        assert_eq!((chain_len(&k1, &row1), chain_len(&k2, &row2), chain_len(&k3, &row3)), (1, 2, 3));
        assert_eq!(chain_len(&k0, &project(&k0, r#"{"name":"Trip"}"#)), 0, "an epoch-0 row");
        assert_eq!(chain_len(&k0, &row1), 0, "a row the key does not open");

        assert!(may_switch(&k1, &k2, &row2, 1), "the next epoch");
        assert!(may_switch(&k1, &k3, &row3, 1), "skipping an epoch the device never saw");
        assert!(!may_switch(&k1, &k2, &row2, 2), "a kept epoch-2 row served to a device that opened epoch 2");
        assert!(!may_switch(&k1, &k2, &row2, 3), "or epoch 3");
        assert!(may_switch(&k2, &k3, &row3, 2));
    }

    #[test]
    fn resealing_is_byte_exact_and_junk_is_none() {
        let (k1, k2) = (generate_key(), generate_key());
        for plain in [&b"{\"a\":1}"[..], &[0xff, 0x00, 0x80][..], b"not json at all"] {
            let pair = encrypt_bytes(&k1, plain).unwrap();
            assert_eq!(decrypt_bytes(&k2, &reseal(&k1, &k2, &pair).unwrap()).unwrap(), plain);
        }
        let padded = encrypt_padded_json(&k1, &serde_json::json!({"unknown": true})).unwrap();
        let resealed = reseal(&k1, &k2, &padded).unwrap();
        assert_eq!(decrypt_bytes(&k2, &resealed).unwrap(), decrypt_bytes(&k1, &padded).unwrap(), "padding kept");
        let junk = EncryptedPair { ct: B64.encode([9u8; 40]), iv: B64.encode([1u8; 24]) };
        assert!(reseal(&k1, &k2, &junk).is_none());
    }

    #[test]
    fn a_directory_entry_is_used_only_when_it_matches_the_served_keys() {
        let k = generate_key();
        let account = Uuid::new_v4();
        let (x, kem) = (vec![1u8; 32], vec![2u8; shared::KEM_PUBLIC_KEY_LENGTH]);
        let member = RotationMember {
            account_id: account,
            user_id: Some(3),
            public_key: Some(x.clone()),
            kem_public_key: Some(kem.clone()),
            member_key: member_key_entry(&k, account, &x, &kem),
        };
        assert_eq!(recipient_keys(&k, &member), Some((x.clone(), kem.clone())));
        let swapped = RotationMember { kem_public_key: Some(vec![9u8; shared::KEM_PUBLIC_KEY_LENGTH]), ..member.clone() };
        assert_eq!(recipient_keys(&k, &swapped), None, "the server served another key");
        let other = RotationMember { account_id: Uuid::new_v4(), ..member.clone() };
        assert_eq!(recipient_keys(&k, &other), None, "an entry copied onto another account");
        assert_eq!(recipient_keys(&generate_key(), &member), None, "an entry under another key");
    }

    #[test]
    fn a_box_opens_only_for_its_project_account_and_epoch() {
        let (x, seed) = (generate_key(), crate::hybrid::generate_kem_seed());
        let x_pub = crate::crypto::public_key_of(&x);
        let kem_pub = crate::hybrid::kem_public_key(&seed);
        let (project_id, me, key) = (Uuid::new_v4(), Uuid::new_v4(), generate_key());
        let sealed = seal_box(&x_pub, &kem_pub, project_id, me, 2, &key).unwrap();
        let boxed = shared::RotationBox { project_id, key_epoch: 2, sealed };
        assert_eq!(open_box(&x, &seed, project_id, me, &boxed), Some(key));
        assert_eq!(open_box(&x, &seed, Uuid::new_v4(), me, &boxed), None);
        assert_eq!(open_box(&x, &seed, project_id, Uuid::new_v4(), &boxed), None);
        assert_eq!(open_box(&x, &seed, project_id, me, &shared::RotationBox { key_epoch: 3, ..boxed.clone() }), None);
    }

    #[test]
    fn queued_ops_are_resealed_under_the_new_key() {
        let (k1, k2) = (generate_key(), generate_key());
        let sealed = encrypt_padded_json(&k1, &serde_json::json!({"amount": 12})).unwrap();
        let mut op = serde_json::json!({"type": "add_expense", "projectId": Uuid::nil(), "payload": sealed, "history": {"payload": sealed}});
        rekey_value(&mut op, &k1, &k2);
        for path in ["/payload", "/history/payload"] {
            let pair: EncryptedPair = serde_json::from_value(op.pointer(path).unwrap().clone()).unwrap();
            assert_eq!(decrypt_bytes(&k2, &pair).unwrap(), decrypt_bytes(&k1, &sealed).unwrap());
        }
    }

    #[test]
    fn a_removed_participant_is_hidden_under_the_new_key_with_every_field_kept() {
        let (k1, k2) = (generate_key(), generate_key());
        let row = encrypt_padded_json(&k1, &serde_json::json!({"name": "Léa", "futureField": 3})).unwrap();
        let hidden: shared::UserPayload = crate::crypto::decrypt_json(&k2, &reseal_removed(&k1, &k2, &row).unwrap()).unwrap();
        assert_eq!((hidden.name.as_str(), hidden.removed), ("Léa", true));
        let kept: Value = crate::crypto::decrypt_json(&k2, &reseal_removed(&k1, &k2, &row).unwrap()).unwrap();
        assert_eq!(kept["futureField"], 3);
        assert!(reseal_removed(&k2, &k1, &row).is_none(), "a row the old key does not open");
    }

    #[test]
    fn only_the_removed_participant_is_staged_hidden() {
        let (k1, k2) = (generate_key(), generate_key());
        let row = encrypt_padded_json(&k1, &serde_json::json!({"name": "Léa"})).unwrap();
        let removed = |pair: &EncryptedPair| crate::crypto::decrypt_json::<shared::UserPayload>(&k2, pair).unwrap().removed;
        assert!(removed(&restage("users", "7", Some(7), &k1, &k2, &row).unwrap()));
        assert!(!removed(&restage("users", "8", Some(7), &k1, &k2, &row).unwrap()), "another participant");
        assert!(!removed(&restage("users", "7", None, &k1, &k2, &row).unwrap()), "a plain rotation");
        let same = restage("expenses", "7", Some(7), &k1, &k2, &row).unwrap();
        assert_eq!(decrypt_bytes(&k2, &same).unwrap(), decrypt_bytes(&k1, &row).unwrap(), "an expense with the same id");
        let not_json = encrypt_bytes(&k1, b"\xff\x00").unwrap();
        assert_eq!(decrypt_bytes(&k2, &restage("users", "7", Some(7), &k1, &k2, &not_json).unwrap()).unwrap(), b"\xff\x00");
    }
}
