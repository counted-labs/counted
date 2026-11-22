//! The client side of entity ids (SPEC 15.9.1, 15.9.2): the id drawn before a row's first
//! encryption and sealed in it, the reader rules R1 and R2, and a taken id on a creation.

use api::payments::payments_controller::sync_project_data;
use api::recurring::recurring_controller::add_recurring_expense;
use api::users::users_controller::{add_user, get_users_by_project_id};
use dioxus::fullstack::{Json, ServerFnError};
use serde::de::DeserializeOwned;
use shared::entity::{self, Slot};
use shared::{
    CreatableExpense, CreatableRecurringExpense, CreatableUser, CreatableUserBatch, EditableUser, EncryptedPair, ExpensePayload,
    HistoryContext, HistoryPayload, RecurringExpense, User, UserPayload, NO_CACHED_VERSION,
};
use uuid::Uuid;

use crate::common::is_id_taken_error;
use crate::crypto::{decrypt, decrypt_bytes, encrypt_padded_json};
use crate::recurring::model::RecurringPayload;

/// A payload served in one list, which names the slot R1 holds it to.
pub trait Row: DeserializeOwned {
    const SLOT: Slot;
}

impl Row for UserPayload {
    const SLOT: Slot = Slot::Participant;
}

impl Row for ExpensePayload {
    const SLOT: Slot = Slot::Expense;
}

impl Row for RecurringPayload {
    const SLOT: Slot = Slot::Rule;
}

impl Row for HistoryPayload {
    const SLOT: Slot = Slot::History;
}

/// A row that fails R1 or R2 is refused like one that does not open (SPEC 5.5).
pub fn decrypt_row<T: Row>(key: &[u8; 32], pair: &EncryptedPair) -> Result<T, String> {
    let plain = decrypt(key, pair)?;
    entity::read(&plain, T::SLOT).map_err(|e| format!("unreadable row: {e:?}"))?;
    serde_json::from_str(&plain).map_err(|e| e.to_string())
}

pub fn new_participant(key: &[u8; 32], project_id: Uuid, name: String) -> Result<CreatableUser, String> {
    let eid = Uuid::new_v4();
    let mut payload = UserPayload { name, removed: false, extra: Default::default() };
    entity::bind(&mut payload.extra, Slot::Participant, eid);
    Ok(CreatableUser { payload: encrypt_padded_json(key, &payload)?, project_id, eid: Some(eid) })
}

/// Removing or restoring a participant rewrites its stored payload, never one rebuilt from the form.
pub fn participant_edit(key: &[u8; 32], project_id: Uuid, served: &User, removed: bool) -> Result<EditableUser, String> {
    let mut payload: UserPayload = decrypt_row(key, &served.payload)?;
    payload.removed = removed;
    Ok(EditableUser { id: served.id, project_id, payload: encrypt_padded_json(key, &payload)? })
}

/// The expense takes `eid` whatever `payload` was built from, and sends it in clear as `client_op_id`.
pub fn new_expense(
    key: &[u8; 32],
    project_id: Uuid,
    eid: Uuid,
    mut payload: ExpensePayload,
    history: Option<HistoryContext>,
) -> Result<CreatableExpense, String> {
    entity::bind(&mut payload.extra, Slot::Expense, eid);
    Ok(CreatableExpense {
        project_id,
        author_id: None,
        payload: encrypt_padded_json(key, &payload)?,
        payers: Vec::new(),
        debtors: Vec::new(),
        history,
        client_op_id: Some(eid),
    })
}

/// An edit keeps `eid`, `slot` and the keys this build does not know, and each rebuilt share the
/// unknown keys of the share it replaces.
pub fn edited_expense(original: &ExpensePayload, mut edited: ExpensePayload) -> ExpensePayload {
    edited.extra = original.extra.clone();
    if let (Some(before), Some(after)) = (original.shares.as_deref(), edited.shares.as_mut()) {
        entity::keep_share_extras(before, after);
    }
    edited
}

pub fn sealed_history(key: &[u8; 32], mut payload: HistoryPayload) -> Result<HistoryContext, String> {
    let eid = Uuid::new_v4();
    entity::bind(&mut payload.extra, Slot::History, eid);
    Ok(HistoryContext { actor_user_id: None, payload: encrypt_padded_json(key, &payload)?, eid: Some(eid) })
}

/// After `ID_TAKEN`: the participants of the batch that this client's earlier delivery stored. The
/// server stores a batch whole or not at all, so `None` unless every id is held by a served row
/// that opens under the key.
pub fn landed_participants(key: &[u8; 32], served: &[User], batch: &[CreatableUser]) -> Option<Vec<User>> {
    if batch.is_empty() {
        return None;
    }
    batch
        .iter()
        .map(|sent| {
            let eid = sent.eid?;
            served.iter().find(|u| holds(key, &u.payload, Slot::Participant, eid)).cloned()
        })
        .collect()
}

fn holds(key: &[u8; 32], pair: &EncryptedPair, slot: Slot, eid: Uuid) -> bool {
    decrypt(key, pair).is_ok_and(|plain| entity::read(&plain, slot) == Ok(Some(eid)))
}

/// After `ID_TAKEN`: the rule served under `id`, when its payload opens under the key.
pub fn landed_rule(key: &[u8; 32], served: &[RecurringExpense], id: Uuid) -> Option<RecurringExpense> {
    served.iter().find(|r| r.id == id && decrypt_bytes(key, &r.payload).is_ok()).cloned()
}

/// A taken id that is not this client's own earlier delivery answers the refusal: nothing sends the
/// batch again, and no fresh id is drawn for it.
pub async fn add_participants(key: &[u8; 32], batch: Vec<CreatableUser>) -> Result<Vec<User>, ServerFnError> {
    let Some(project_id) = batch.first().map(|u| u.project_id) else { return Ok(Vec::new()) };
    crate::common::project_token::scope_key(Some(key));
    match add_user(Json(CreatableUserBatch::Multiple(batch.clone()))).await {
        Err(e) if is_id_taken_error(&e) => {
            crate::common::project_token::scope_key(Some(key));
            let served = get_users_by_project_id(project_id).await?;
            landed_participants(key, &served, &batch).ok_or(e)
        }
        sent => sent,
    }
}

pub async fn add_rule(key: &[u8; 32], req: CreatableRecurringExpense) -> Result<RecurringExpense, ServerFnError> {
    let (project_id, id) = (req.project_id, req.id);
    crate::common::project_token::scope(project_id);
    match add_recurring_expense(Json(req)).await {
        Err(e) if is_id_taken_error(&e) => {
            crate::common::project_token::scope(project_id);
            let served = sync_project_data(project_id, NO_CACHED_VERSION).await?.recurring.unwrap_or_default();
            id.and_then(|id| landed_rule(key, &served, id)).ok_or(e)
        }
        sent => sent,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::test_fixtures::{make_user, TEST_KEY};
    use crate::crypto::{encrypt, encrypt_json};
    use shared::time::DateTime;

    const OTHER_KEY: [u8; 32] = [7u8; 32];

    fn served(id: i32, sent: &CreatableUser) -> User {
        User { id, payload: sent.payload.clone(), ..Default::default() }
    }

    fn rule_row(id: Uuid, payload: EncryptedPair) -> RecurringExpense {
        RecurringExpense {
            id,
            project_id: Uuid::nil(),
            author_id: None,
            payload,
            version: 0,
            created_at: DateTime::default(),
            listed_participants: false,
        }
    }

    #[test]
    fn a_new_participant_seals_the_id_it_sends_in_clear() {
        let created = new_participant(&TEST_KEY, Uuid::nil(), "Ana".into()).unwrap();
        let eid = created.eid.expect("an id in clear");
        assert_eq!(eid.get_version_num(), 4);
        let plain = decrypt(&TEST_KEY, &created.payload).unwrap();
        assert_eq!(entity::read(&plain, Slot::Participant), Ok(Some(eid)));
        assert_eq!(plain.len(), 256, "padded");
        assert_ne!(new_participant(&TEST_KEY, Uuid::nil(), "Ana".into()).unwrap().eid, Some(eid), "one id per row");
    }

    #[test]
    fn removing_and_restoring_a_participant_keep_both_keys_and_the_keys_it_does_not_know() {
        let eid = Uuid::new_v4();
        let stored = format!(r#"{{"name":"Ana","eid":"{eid}","slot":"participant","later":{{"a":1}}}}"#);
        let row = User { id: 4, payload: encrypt(&TEST_KEY, &stored).unwrap(), ..Default::default() };
        let removed = participant_edit(&TEST_KEY, Uuid::nil(), &row, true).unwrap();
        assert_eq!(removed.id, 4);
        let plain = decrypt(&TEST_KEY, &removed.payload).unwrap();
        assert_eq!(entity::read(&plain, Slot::Participant), Ok(Some(eid)));
        assert!(plain.contains(r#""later":{"a":1}"#) && plain.contains(r#""removed":true"#));

        let restored = participant_edit(&TEST_KEY, Uuid::nil(), &User { payload: removed.payload, ..row }, false).unwrap();
        let plain = decrypt(&TEST_KEY, &restored.payload).unwrap();
        assert_eq!(entity::read(&plain, Slot::Participant), Ok(Some(eid)));
        assert!(plain.contains(r#""later":{"a":1}"#) && !plain.contains("removed"));
    }

    #[test]
    fn a_participant_without_an_id_is_edited_without_one() {
        let row = make_user(&TEST_KEY, 1, "Ana");
        let edit = participant_edit(&TEST_KEY, Uuid::nil(), &row, true).unwrap();
        let plain = decrypt(&TEST_KEY, &edit.payload).unwrap();
        assert_eq!(entity::read(&plain, Slot::Participant), Ok(None));
    }

    #[test]
    fn an_unreadable_participant_is_never_rewritten() {
        let eid = Uuid::new_v4();
        for stored in [
            format!(r#"{{"name":"Ana","eid":"{eid}","slot":"expense"}}"#),
            format!(r#"{{"name":"Ana","eid":"{eid}"}}"#),
            format!(r#"{{"name":"Ana","eid":"{eid}","slot":"participant","eid":"{eid}"}}"#),
        ] {
            let row = User { id: 1, payload: encrypt(&TEST_KEY, &stored).unwrap(), ..Default::default() };
            assert!(participant_edit(&TEST_KEY, Uuid::nil(), &row, true).is_err(), "{stored}");
        }
    }

    #[test]
    fn a_history_entry_seals_its_own_id_and_sends_it() {
        let payload = HistoryPayload {
            summary: "s".into(),
            actor_user_id: Some(1),
            action: None,
            entity: None,
            entity_id: Some(9),
            extra: Default::default(),
        };
        let ctx = sealed_history(&TEST_KEY, payload.clone()).unwrap();
        let eid = ctx.eid.expect("an id in clear");
        let plain = decrypt(&TEST_KEY, &ctx.payload).unwrap();
        assert_eq!(entity::read(&plain, Slot::History), Ok(Some(eid)));
        assert_eq!(decrypt_row::<HistoryPayload>(&TEST_KEY, &ctx.payload).unwrap().entity_id, Some(9), "the row it is about stays apart");
        assert_ne!(sealed_history(&TEST_KEY, payload).unwrap().eid, Some(eid));
    }

    #[test]
    fn r1_a_row_served_in_another_list_is_unreadable() {
        let participant = new_participant(&TEST_KEY, Uuid::nil(), "Ana".into()).unwrap().payload;
        assert!(decrypt_row::<UserPayload>(&TEST_KEY, &participant).is_ok());
        assert!(decrypt_row::<HistoryPayload>(&TEST_KEY, &participant).is_err());
        let as_history = encrypt(&TEST_KEY, &format!(r#"{{"summary":"s","eid":"{}","slot":"participant"}}"#, Uuid::new_v4())).unwrap();
        assert!(decrypt_row::<HistoryPayload>(&TEST_KEY, &as_history).is_err());
    }

    #[test]
    fn r2_a_malformed_pair_of_keys_is_unreadable() {
        let eid = Uuid::new_v4();
        for plain in [
            format!(r#"{{"name":"Ana","eid":"{eid}"}}"#),
            r#"{"name":"Ana","slot":"participant"}"#.to_string(),
            format!(r#"{{"name":"Ana","eid":"{}","slot":"participant"}}"#, eid.to_string().to_uppercase()),
            r#"{"name":"Ana","eid":5,"slot":"participant"}"#.to_string(),
            format!(r#"{{"name":"Ana","eid":"{eid}","slot":"participant","slot":"participant"}}"#),
        ] {
            assert!(decrypt_row::<UserPayload>(&TEST_KEY, &encrypt(&TEST_KEY, &plain).unwrap()).is_err(), "{plain}");
        }
        let v1 = encrypt_json(&TEST_KEY, &UserPayload { name: "Ana".into(), removed: false, extra: Default::default() }).unwrap();
        assert_eq!(decrypt_row::<UserPayload>(&TEST_KEY, &v1).unwrap().name, "Ana", "neither key reads as v1");
    }

    #[test]
    fn a_taken_participant_id_is_landed_only_when_every_row_of_the_batch_opens_and_holds_it() {
        let batch: Vec<CreatableUser> =
            ["Ana", "Ben"].map(|n| new_participant(&TEST_KEY, Uuid::nil(), n.into()).unwrap()).to_vec();
        let others = make_user(&TEST_KEY, 1, "Zoe");
        let both = [others.clone(), served(2, &batch[0]), served(3, &batch[1])];
        let landed = landed_participants(&TEST_KEY, &both, &batch).unwrap();
        assert_eq!(landed.iter().map(|u| u.id).collect::<Vec<_>>(), [2, 3]);

        assert_eq!(landed_participants(&TEST_KEY, &[others.clone(), served(2, &batch[0])], &batch), None, "one id held elsewhere");
        assert_eq!(landed_participants(&TEST_KEY, &[others], &batch), None);
        assert_eq!(landed_participants(&OTHER_KEY, &both, &batch), None, "a row that does not open under the key");

        let another_id = CreatableUser { eid: Some(Uuid::new_v4()), ..batch[0].clone() };
        assert_eq!(landed_participants(&TEST_KEY, &both, &[another_id]), None, "the id must be the one sealed");
        assert_eq!(landed_participants(&TEST_KEY, &both, &[CreatableUser { eid: None, ..batch[0].clone() }]), None);
        assert_eq!(landed_participants(&TEST_KEY, &both, &[]), None);
    }

    #[test]
    fn a_taken_rule_id_is_landed_only_when_a_rule_served_under_it_opens_under_the_key() {
        let id = Uuid::new_v4();
        let opens = encrypt_json(&TEST_KEY, &"rule").unwrap();
        let other_key = encrypt_json(&OTHER_KEY, &"rule").unwrap();
        assert_eq!(landed_rule(&TEST_KEY, &[rule_row(Uuid::new_v4(), opens.clone()), rule_row(id, opens.clone())], id).map(|r| r.id), Some(id));
        assert_eq!(landed_rule(&TEST_KEY, &[rule_row(id, other_key)], id), None);
        assert_eq!(landed_rule(&TEST_KEY, &[rule_row(Uuid::new_v4(), opens)], id), None);
        assert_eq!(landed_rule(&TEST_KEY, &[], id), None);
    }
}
