//! Entity ids (SPEC 15.9.1) and the `eid` and `slot` keys inside v1 payloads (SPEC 15.9.2).

use serde::de::{IgnoredAny, MapAccess, SeqAccess, Visitor};
use serde::{Deserialize, Deserializer};
use serde_json::Value;
use std::fmt;
use uuid::Uuid;

use crate::{ExpenseShare, Unknown};

pub const EID: &str = "eid";
pub const SLOT: &str = "slot";

/// The list a row is served in: `users`, `expenses`, `recurring_expenses`, `project_history`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Slot {
    Participant,
    Expense,
    Rule,
    History,
}

impl Slot {
    pub const fn name(self) -> &'static str {
        match self {
            Slot::Participant => "participant",
            Slot::Expense => "expense",
            Slot::Rule => "rule",
            Slot::History => "history",
        }
    }
}

/// Exactly the 36-character hyphenated form in lower case. The version and variant bits are not
/// tested.
pub fn parse_canonical(s: &str) -> Option<Uuid> {
    Uuid::try_parse(s).ok().filter(|id| id.hyphenated().to_string() == s)
}

/// An entity id as a reader takes it: the canonical form, and never the all-zero value.
pub fn entity_id(s: &str) -> Option<Uuid> {
    parse_canonical(s).filter(|id| !id.is_nil())
}

/// The optional id of a creation on the wire. The all-zero id passes here and is the server's to
/// refuse with `INVALID_PAYLOAD`.
pub(crate) fn optional_canonical<'de, D: Deserializer<'de>>(d: D) -> Result<Option<Uuid>, D::Error> {
    match Option::<String>::deserialize(d)? {
        None => Ok(None),
        Some(s) => parse_canonical(&s)
            .map(Some)
            .ok_or_else(|| serde::de::Error::custom("an entity id in a form other than the canonical one")),
    }
}

/// W1 and W3: a creation writes both keys, and a row built from another row's payload replaces the
/// source's with its own.
pub fn bind(extra: &mut Unknown, slot: Slot, eid: Uuid) {
    extra.insert(EID.into(), Value::String(eid.hyphenated().to_string()));
    extra.insert(SLOT.into(), Value::String(slot.name().into()));
}

/// W5: a share rebuilt by an edit keeps the unknown keys of the share it replaces for the same
/// participant on the same side; a share the edit adds has none.
pub fn keep_share_extras(before: &[ExpenseShare], after: &mut [ExpenseShare]) {
    for share in after.iter_mut() {
        share.extra = before
            .iter()
            .find(|old| old.user_id == share.user_id && old.is_debt == share.is_debt)
            .map(|old| old.extra.clone())
            .unwrap_or_default();
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Unreadable {
    NotJson,
    /// R1.
    WrongSlot,
    /// R2, each below.
    OneKeyWithoutTheOther,
    NotAString,
    NotAnEntityId,
    NamedTwice,
}

/// R1 and R2 on a decrypted v1 plaintext served in the list of `served_in`. `Ok(Some(eid))` when
/// both keys are there and well formed, `Ok(None)` when neither is (or the plaintext is not an
/// object), and an error for a row that is unreadable.
pub fn read(plaintext: &str, served_in: Slot) -> Result<Option<Uuid>, Unreadable> {
    let mut de = serde_json::Deserializer::from_str(plaintext);
    let found = de.deserialize_any(TopLevel).map_err(|_| Unreadable::NotJson)?;
    de.end().map_err(|_| Unreadable::NotJson)?;
    let Some((eids, slots)) = found else { return Ok(None) };
    if eids.len() > 1 || slots.len() > 1 {
        return Err(Unreadable::NamedTwice);
    }
    let as_str = |v: Option<&Value>| match v {
        None => Ok(None),
        Some(Value::String(s)) => Ok(Some(s.clone())),
        Some(_) => Err(Unreadable::NotAString),
    };
    let (eid, slot) = (as_str(eids.first())?, as_str(slots.first())?);
    if slot.as_deref().is_some_and(|s| s != served_in.name()) {
        return Err(Unreadable::WrongSlot);
    }
    match (eid, slot) {
        (None, None) => Ok(None),
        (Some(eid), Some(_)) => entity_id(&eid).map(Some).ok_or(Unreadable::NotAnEntityId),
        _ => Err(Unreadable::OneKeyWithoutTheOther),
    }
}

/// Every top-level value named `eid` or `slot`, duplicates included, which a map would fold into
/// one. `None` for a plaintext that is not an object.
struct TopLevel;

type Found = Option<(Vec<Value>, Vec<Value>)>;

impl<'de> Visitor<'de> for TopLevel {
    type Value = Found;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("a JSON value")
    }

    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Found, A::Error> {
        let (mut eids, mut slots) = (Vec::new(), Vec::new());
        while let Some(key) = map.next_key::<String>()? {
            match key.as_str() {
                EID => eids.push(map.next_value::<Value>()?),
                SLOT => slots.push(map.next_value::<Value>()?),
                _ => {
                    map.next_value::<IgnoredAny>()?;
                }
            }
        }
        Ok(Some((eids, slots)))
    }

    fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Found, A::Error> {
        while seq.next_element::<IgnoredAny>()?.is_some() {}
        Ok(None)
    }

    fn visit_str<E>(self, _: &str) -> Result<Found, E> {
        Ok(None)
    }

    fn visit_bool<E>(self, _: bool) -> Result<Found, E> {
        Ok(None)
    }

    fn visit_i64<E>(self, _: i64) -> Result<Found, E> {
        Ok(None)
    }

    fn visit_u64<E>(self, _: u64) -> Result<Found, E> {
        Ok(None)
    }

    fn visit_f64<E>(self, _: f64) -> Result<Found, E> {
        Ok(None)
    }

    fn visit_unit<E>(self) -> Result<Found, E> {
        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{pad_json, ExpensePayload, HistoryPayload, UserPayload};

    const ID: &str = "0f8e2c4a-6b1d-4e3f-9a7b-5c2d1e0f3a4b";
    const OTHER: &str = "7d2c9b1e-3a4f-4c5d-8e6f-0a1b2c3d4e5f";
    const OCCURRENCE: &str = "3c1f0a9e-77d2-8b4c-a5e6-1f2a3b4c5d6e";

    fn id(s: &str) -> Uuid {
        Uuid::parse_str(s).unwrap()
    }

    #[test]
    fn an_entity_id_is_read_in_the_canonical_form_only_and_never_all_zero() {
        assert_eq!(entity_id(ID), Some(id(ID)));
        assert_eq!(entity_id(OCCURRENCE), Some(id(OCCURRENCE)), "version 8 is read like any other");
        for other in [
            "0F8E2C4A-6B1D-4E3F-9A7B-5C2D1E0F3A4B",
            "0f8e2c4a6b1d4e3f9a7b5c2d1e0f3a4b",
            "{0f8e2c4a-6b1d-4e3f-9a7b-5c2d1e0f3a4b}",
            "urn:uuid:0f8e2c4a-6b1d-4e3f-9a7b-5c2d1e0f3a4b",
            " 0f8e2c4a-6b1d-4e3f-9a7b-5c2d1e0f3a4b",
            "00000000-0000-0000-0000-000000000000",
            "",
        ] {
            assert_eq!(entity_id(other), None, "{other} was read");
        }
        assert_eq!(parse_canonical("00000000-0000-0000-0000-000000000000"), Some(Uuid::nil()));
    }

    #[test]
    fn a_creation_writes_both_keys_and_its_reader_finds_them() {
        let mut payload: UserPayload = serde_json::from_str(r#"{"name":"Ana"}"#).unwrap();
        bind(&mut payload.extra, Slot::Participant, id(ID));
        let json = pad_json(serde_json::to_string(&payload).unwrap());
        assert!(json.contains(&format!(r#""eid":"{ID}""#)) && json.contains(r#""slot":"participant""#));
        assert_eq!(read(&json, Slot::Participant), Ok(Some(id(ID))));
    }

    #[test]
    fn every_rewrite_keeps_both_keys_and_the_keys_it_does_not_know() {
        let stored = format!(r#"{{"name":"Ana","eid":"{ID}","slot":"participant","later":{{"a":1}}}}"#);
        let mut participant: UserPayload = serde_json::from_str(&stored).unwrap();
        participant.name = "Ana B".into();
        participant.removed = true;
        let rewritten = serde_json::to_string(&participant).unwrap();
        assert_eq!(read(&rewritten, Slot::Participant), Ok(Some(id(ID))));
        assert!(rewritten.contains(r#""later":{"a":1}"#));

        let stored = format!(r#"{{"summary":"x","eid":"{ID}","slot":"history","later":true}}"#);
        let history: HistoryPayload = serde_json::from_str(&stored).unwrap();
        let rewritten = serde_json::to_string(&history).unwrap();
        assert_eq!(read(&rewritten, Slot::History), Ok(Some(id(ID))));
        assert!(rewritten.contains(r#""later":true"#));

        let stored = format!(
            r#"{{"name":"Taxi","amount":12.5,"expenseType":"expense","date":"2026-11-21","eid":"{ID}","slot":"expense"}}"#
        );
        let mut expense: ExpensePayload = serde_json::from_str(&stored).unwrap();
        expense.amount = 13.0;
        assert_eq!(read(&serde_json::to_string(&expense).unwrap(), Slot::Expense), Ok(Some(id(ID))));
    }

    #[test]
    fn a_row_built_from_another_row_takes_its_own_eid() {
        let source = format!(
            r#"{{"name":"Taxi","amount":12.5,"expenseType":"expense","date":"2026-11-21","eid":"{ID}","slot":"expense","later":1}}"#
        );
        let mut moved: ExpensePayload = serde_json::from_str(&source).unwrap();
        bind(&mut moved.extra, Slot::Expense, id(OTHER));
        let json = serde_json::to_string(&moved).unwrap();
        assert_eq!(read(&json, Slot::Expense), Ok(Some(id(OTHER))));
        assert!(!json.contains(ID));
        assert!(json.contains(r#""later":1"#));
    }

    #[test]
    fn a_rebuilt_share_keeps_the_unknown_keys_of_the_share_it_replaces_and_an_added_one_has_none() {
        let share = |user_id: i32, is_debt: bool, extra: &str| ExpenseShare {
            user_id,
            amount: 1.0,
            is_debt,
            extra: serde_json::from_str(extra).unwrap(),
        };
        let before = [share(1, false, r#"{"k":"paid"}"#), share(1, true, r#"{"k":"owed"}"#), share(2, true, r#"{"k":"two"}"#)];
        let mut after = [share(1, true, "{}"), share(1, false, "{}"), share(3, true, r#"{"k":"stale"}"#), share(2, false, "{}")];
        keep_share_extras(&before, &mut after);
        let k = |s: &ExpenseShare| s.extra.get("k").and_then(Value::as_str).map(str::to_string);
        assert_eq!(k(&after[0]).as_deref(), Some("owed"));
        assert_eq!(k(&after[1]).as_deref(), Some("paid"));
        assert_eq!(k(&after[2]), None, "a share the edit adds has none");
        assert_eq!(k(&after[3]), None, "same participant, other side");

        let json = r#"{"userId":1,"amount":2.5,"isDebt":true,"later":[1,2]}"#;
        let read_back: ExpenseShare = serde_json::from_str(json).unwrap();
        assert!(serde_json::to_string(&read_back).unwrap().contains(r#""later":[1,2]"#));
    }

    #[test]
    fn neither_key_is_a_row_as_v1_reads_it() {
        assert_eq!(read(r#"{"name":"Ana"}"#, Slot::Participant), Ok(None));
        assert_eq!(read(&pad_json(r#"{"name":"Ana"}"#.into()), Slot::Participant), Ok(None));
        assert_eq!(read(r#""Ana""#, Slot::Participant), Ok(None));
        assert_eq!(read("[1,{\"eid\":2}]", Slot::Participant), Ok(None));
        let nested = format!(r#"{{"template":{{"eid":"{ID}","slot":"expense"}}}}"#);
        assert_eq!(read(&nested, Slot::Rule), Ok(None), "only the top level counts");
    }

    #[test]
    fn r1_a_slot_other_than_the_list_it_was_served_in_is_unreadable() {
        let expense = format!(r#"{{"eid":"{ID}","slot":"expense"}}"#);
        assert_eq!(read(&expense, Slot::Expense), Ok(Some(id(ID))));
        for list in [Slot::Participant, Slot::Rule, Slot::History] {
            assert_eq!(read(&expense, list), Err(Unreadable::WrongSlot));
        }
        for slot in ["project", "Expense", "", "later"] {
            let json = format!(r#"{{"eid":"{ID}","slot":"{slot}"}}"#);
            assert_eq!(read(&json, Slot::Expense), Err(Unreadable::WrongSlot), "{slot}");
        }
        assert_eq!(read(r#"{"slot":"rule"}"#, Slot::Expense), Err(Unreadable::WrongSlot));
    }

    #[test]
    fn r2_one_key_without_the_other_is_unreadable() {
        assert_eq!(read(&format!(r#"{{"eid":"{ID}"}}"#), Slot::Expense), Err(Unreadable::OneKeyWithoutTheOther));
        assert_eq!(read(r#"{"slot":"expense"}"#, Slot::Expense), Err(Unreadable::OneKeyWithoutTheOther));
    }

    #[test]
    fn r2_a_value_that_is_not_a_string_is_unreadable() {
        for eid in ["null", "5", "true", "{}", format!(r#"["{ID}"]"#).as_str()] {
            let json = format!(r#"{{"eid":{eid},"slot":"expense"}}"#);
            assert_eq!(read(&json, Slot::Expense), Err(Unreadable::NotAString), "{eid}");
        }
        for slot in ["null", "1", r#"["expense"]"#] {
            let json = format!(r#"{{"eid":"{ID}","slot":{slot}}}"#);
            assert_eq!(read(&json, Slot::Expense), Err(Unreadable::NotAString), "{slot}");
        }
        assert_eq!(read(r#"{"eid":null}"#, Slot::Expense), Err(Unreadable::NotAString));
    }

    #[test]
    fn r2_an_eid_that_is_not_an_entity_id_is_unreadable() {
        for eid in [
            "0F8E2C4A-6B1D-4E3F-9A7B-5C2D1E0F3A4B",
            "0f8e2c4a6b1d4e3f9a7b5c2d1e0f3a4b",
            "{0f8e2c4a-6b1d-4e3f-9a7b-5c2d1e0f3a4b}",
            "00000000-0000-0000-0000-000000000000",
            "Ana",
            "",
        ] {
            let json = format!(r#"{{"eid":"{eid}","slot":"expense"}}"#);
            assert_eq!(read(&json, Slot::Expense), Err(Unreadable::NotAnEntityId), "{eid}");
        }
    }

    #[test]
    fn r2_a_key_named_twice_is_unreadable_even_with_the_same_value() {
        let cases = [
            format!(r#"{{"eid":"{ID}","slot":"expense","eid":"{ID}"}}"#),
            format!(r#"{{"eid":"{ID}","slot":"expense","eid":"{OTHER}"}}"#),
            format!(r#"{{"eid":"{ID}","slot":"expense","slot":"expense"}}"#),
            format!(r#"{{"eid":"{ID}","slot":"expense","e{}u0069d":"{ID}"}}"#, '\\'),
        ];
        for json in cases {
            assert_eq!(read(&json, Slot::Expense), Err(Unreadable::NamedTwice), "{json}");
        }
    }

    #[test]
    fn a_plaintext_that_is_not_json_is_unreadable() {
        assert_eq!(read("{", Slot::Expense), Err(Unreadable::NotJson));
        assert_eq!(read(r#"{"name":"Ana"} x"#, Slot::Expense), Err(Unreadable::NotJson));
    }
}
