use super::*;
use crate::common::local_storage::LocalStorageProject;
use crate::common::offline_queue::ProjectEdit;
use crate::common::queue_store::tests::MemStore;
use crate::common::queue_store::{write_entries, Snapshot};
use crate::crypto::{decrypt_bytes, decrypt_json, encrypt_json, generate_key, key_to_fragment};
use shared::{
    CreatableExpense, DeleteExpenseRequest, EditableExpense, EditableProject, EncryptedPair, EncryptedUserAmount, HistoryContext,
    ProjectStatus,
};
use uuid::Uuid;

const PROJECT: Uuid = Uuid::from_u128(5);
const OTHER: Uuid = Uuid::from_u128(6);

fn prep(op: &QueuedOp, state: &LocalStorageState) -> Result<(QueuedOp, [u8; 32]), SyncFailure> {
    prepare(&Entry::new(op.clone()), state)
}

fn holding(current: &[u8; 32], previous: &[[u8; 32]]) -> LocalStorageState {
    let previous_keys = previous.iter().enumerate().map(|(epoch, k)| (epoch as i32, key_to_fragment(k))).collect();
    let entry = LocalStorageProject { project_id: PROJECT, encryption_key: Some(key_to_fragment(current)), previous_keys, ..Default::default() };
    LocalStorageState { projects: vec![entry], ..Default::default() }
}

fn seal(key: &[u8; 32], text: &str) -> EncryptedPair {
    encrypt_json(key, &text).unwrap()
}

fn history(key: &[u8; 32]) -> Option<HistoryContext> {
    Some(HistoryContext { actor_user_id: Some(1), payload: seal(key, "history"), eid: None })
}

fn shares(key: &[u8; 32]) -> Vec<EncryptedUserAmount> {
    (1..=3).map(|user_id| EncryptedUserAmount { user_id, payload: seal(key, "share") }).collect()
}

fn queued(label_key: &[u8; 32], op: OpKind) -> QueuedOp {
    QueuedOp { id: Uuid::new_v4(), label: String::new(), sealed_label: Some(seal(label_key, "Hôtel")), op, after: None, queued_at: None }
}

/// Every op the queue can hold, its pairs under `stale` and its label under `fresh`: the label is
/// sealed with the key held when the op is queued, the payload may come from an older draft.
fn every_kind(stale: &[u8; 32], fresh: &[u8; 32]) -> Vec<QueuedOp> {
    let add = CreatableExpense { project_id: PROJECT, author_id: Some(1), payload: seal(stale, "add"), payers: shares(stale), debtors: shares(stale), history: history(stale), client_op_id: None };
    let edit = EditableExpense { id: 3, project_id: PROJECT, author_id: Some(1), payload: seal(stale, "edit"), payers: shares(stale), debtors: shares(stale), history: history(stale), eid: None, expected_rev: None };
    let delete = DeleteExpenseRequest { id: 3, project_id: PROJECT, history: history(stale), eid: None, expected_rev: None };
    let status = EditableProject { id: PROJECT, payload: None, status: Some(ProjectStatus::Closed), history: history(stale), expected_iv: None, notify_closed: true, expected_rev: None };
    let project_edit = ProjectEdit {
        project_id: PROJECT,
        name_description: Some(encrypt_json(stale, &("Corse", Some("été"))).unwrap()),
        status: Some(ProjectStatus::Closed),
        history: history(stale),
        emoji: Some(encrypt_json(stale, &Some("🏝️")).unwrap()),
    };
    vec![
        queued(fresh, OpKind::AddExpense(add)),
        queued(fresh, OpKind::EditExpense(edit)),
        queued(fresh, OpKind::DeleteExpense(delete)),
        queued(fresh, OpKind::UpdateProject(status)),
        queued(fresh, OpKind::EditProject(project_edit)),
    ]
}

fn pairs(op: &QueuedOp) -> Vec<EncryptedPair> {
    fn collect(value: &serde_json::Value, out: &mut Vec<EncryptedPair>) {
        match value {
            serde_json::Value::Object(map) if map.len() == 2 && map.contains_key("ct") && map.contains_key("iv") => {
                out.push(serde_json::from_value(value.clone()).unwrap())
            }
            serde_json::Value::Object(map) => map.values().for_each(|v| collect(v, out)),
            serde_json::Value::Array(items) => items.iter().for_each(|v| collect(v, out)),
            _ => {}
        }
    }
    let mut out = Vec::new();
    collect(&serde_json::to_value(op).unwrap(), &mut out);
    out
}

fn opens_whole_under(key: &[u8; 32], op: &QueuedOp) -> bool {
    let pairs = pairs(op);
    !pairs.is_empty() && pairs.iter().all(|p| decrypt_bytes(key, p).is_ok())
}

/// I12, Q5: every pair goes out under the current key, resealed from a previous key in what is
/// sent.
#[test]
fn every_kind_of_op_goes_out_whole_under_the_current_key() {
    let (k1, k2, k3) = (generate_key(), generate_key(), generate_key());
    let cases = [(k1, k2, holding(&k2, &[k1])), (k1, k3, holding(&k3, &[k1, k2])), (k2, k2, holding(&k2, &[k1]))];
    for (stale, current, state) in cases {
        for op in every_kind(&stale, &current) {
            let (sent, key) = prep(&op, &state).unwrap_or_else(|_| panic!("{:?} is held", op.op));
            assert_eq!(key, current, "the token goes with the key the op was checked under");
            assert!(opens_whole_under(&current, &sent), "{:?}", sent.op);
            assert_eq!((pairs(&sent).len(), sent.id), (pairs(&op).len(), op.id));
        }
    }
}

#[test]
fn a_resealed_project_edit_keeps_what_it_changes() {
    let (k1, k2) = (generate_key(), generate_key());
    let op = every_kind(&k1, &k2).pop().unwrap();
    let (sent, _) = prep(&op, &holding(&k2, &[k1])).unwrap();
    let OpKind::EditProject(edit) = sent.op else { panic!("not a project edit") };
    let named: (String, Option<String>) = decrypt_json(&k2, edit.name_description.as_ref().unwrap()).unwrap();
    assert_eq!(named, ("Corse".to_string(), Some("été".to_string())));
    assert_eq!(decrypt_json::<Option<String>>(&k2, edit.emoji.as_ref().unwrap()).unwrap().as_deref(), Some("🏝️"));
}

/// S1: a project this device holds no key for (left, forgotten) sends nothing.
#[test]
fn an_op_for_a_project_with_no_key_held_is_held_not_sent() {
    let k1 = generate_key();
    let op = every_kind(&k1, &k1).remove(0);
    assert!(matches!(prep(&op, &LocalStorageState::default()), Err(SyncFailure::Held(_))), "no entry");
    let mut keyless = holding(&k1, &[]);
    keyless.projects[0].encryption_key = None;
    assert!(matches!(prep(&op, &keyless), Err(SyncFailure::Held(_))), "an entry without a key");
}

/// S2: a held op is named when its label opens, so the message can say which change stayed.
#[test]
fn a_held_op_is_named_when_its_label_opens() {
    let (k1, k2, stranger) = (generate_key(), generate_key(), generate_key());
    let state = holding(&k2, &[k1]);
    let foreign = every_kind(&stranger, &k1).remove(0);
    assert!(matches!(prep(&foreign, &state), Err(SyncFailure::Held(name)) if name == "Hôtel"), "label under a previous key");
    let unnamed = every_kind(&stranger, &stranger).remove(0);
    assert!(matches!(prep(&unnamed, &state), Err(SyncFailure::Held(name)) if name.is_empty()));
    let legacy = QueuedOp { label: "Café".into(), sealed_label: None, ..unnamed };
    assert!(matches!(prep(&legacy, &state), Err(SyncFailure::Held(name)) if name == "Café"), "a clear label from an older build");
}

/// S4: a block lasts only while the keys it was hit under are the ones held.
#[test]
fn a_key_change_lifts_a_blocked_replay() {
    let (k1, k2) = (generate_key(), generate_key());
    let mut state = holding(&k1, &[]);
    let blocked_at = held_keys(&state);
    let mut unrelated = state.clone();
    unrelated.onboarding_seen = true;
    assert!(still_blocked(Some(&blocked_at), &unrelated), "nothing about the keys changed");
    crate::common::upsert_project_key(&mut state, PROJECT, key_to_fragment(&k2));
    assert!(!still_blocked(Some(&blocked_at), &state), "the switch lifts it");
    assert!(!still_blocked(None, &state));
}

fn server_error(message: &str, code: u16) -> ServerFnError {
    ServerFnError::ServerError { message: message.into(), code, details: None }
}

/// I13, Q3: for the types up to 3.4.0 a 404 is a landing for a delete; any other is dropped with a
/// notice.
#[test]
fn a_404_lands_a_delete_and_drops_any_other_entry_with_a_notice() {
    let k = generate_key();
    let gone = || Err(server_error(shared::errors::EXPENSE_NOT_FOUND, 404));
    let ops = every_kind(&k, &k);
    assert!(matches!(answer(&ops[2], gone()), Answer::Landed(Some(Landed::Deleted(3)))));
    assert!(matches!(answer(&ops[1], gone()), Answer::Dropped(SyncFailure::ConflictEdit(_))));
    for op in [&ops[0], &ops[3], &ops[4]] {
        assert!(matches!(answer(op, gone()), Answer::Dropped(SyncFailure::ConflictOther(_))));
    }
}

/// I13, Q4: refused for good is any refusal but a 404, a failure to reach the server, 5xx, 429 and
/// 409 `PROJECT_ROTATING`. 426 is kept out too: the update it asks for sends the entry.
#[test]
fn refusals_are_classified_and_only_a_refusal_for_good_is_listed() {
    let k = generate_key();
    let delete = every_kind(&k, &k).remove(2);
    let cases: Vec<(ServerFnError, bool)> = vec![
        (server_error(shared::errors::PROJECT_ROTATING, 409), false),
        (server_error("busy", 503), false),
        (server_error("boom", 500), false),
        (server_error("slow down", 429), false),
        (server_error("update", 426), false),
        (ServerFnError::StreamError("reset".into()), false),
        (ServerFnError::Deserialization("cut".into()), false),
        (server_error("Forbidden", 403), true),
        (server_error("Bad request", 400), true),
        (server_error("This was changed elsewhere", 409), true),
        (server_error("Not Found", 404), true),
    ];
    for (err, for_good) in cases {
        let found = answer(&delete, Err(err.clone()));
        assert_eq!(matches!(found, Answer::Refused(_)), for_good, "{err:?}");
        assert_eq!(matches!(found, Answer::Retry(_)), !for_good, "{err:?}");
    }
}

/// I13, Q4, Q9: an entry refused for good stops its project only, the other projects drain, and it
/// is listed as unsent with what stands behind nothing.
#[test]
fn a_refusal_for_good_stops_its_project_and_is_listed() {
    let k = generate_key();
    let mut state = holding(&k, &[]);
    state.projects.push(LocalStorageProject { project_id: OTHER, encryption_key: Some(key_to_fragment(&k)), ..Default::default() });
    let ops = every_kind(&k, &k);
    let refused_op = ops[0].clone();
    let mut other = ops[0].clone();
    other.id = Uuid::new_v4();
    if let OpKind::AddExpense(c) = &mut other.op {
        c.project_id = OTHER;
    }
    let raws: Vec<serde_json::Value> = [refused_op.clone(), ops[0].clone(), other.clone()].iter().map(|op| serde_json::to_value(op).unwrap()).collect();
    let mut store = MemStore::new();
    for (n, raw) in raws.iter().enumerate() {
        let mut raw = raw.clone();
        if n == 1 {
            raw["id"] = serde_json::json!(Uuid::new_v4());
        }
        assert!(write_entries(&mut store, std::slice::from_ref(&raw)));
    }
    let queue = Snapshot::read(&mut store).queue();
    let refused: BTreeMap<Uuid, String> = [(refused_op.id, "Forbidden".to_string())].into();
    let next = next_to_send(&queue, |e| e.id.is_some_and(|id| refused.contains_key(&id)));
    assert_eq!(next.map(|i| queue[i].id), Some(Some(other.id)), "the other project drains, the add behind it waits");
    let behind_it = [refused_op.clone(), ops[1].clone()].iter().map(|op| serde_json::to_value(op).unwrap()).collect::<Vec<_>>();
    let mut store = MemStore::new();
    for raw in &behind_it {
        assert!(write_entries(&mut store, std::slice::from_ref(raw)));
    }
    let edits = Snapshot::read(&mut store).queue();
    assert_eq!(next_to_send(&edits, |e| e.id == Some(refused_op.id)), Some(1), "an edit of a row is about another row than a creation (Q4)");
    let listed: Vec<Option<Uuid>> = unsent(&queue, &state, &refused).iter().map(|(e, _)| e.id).collect();
    assert_eq!(listed, vec![Some(refused_op.id)]);
    assert_eq!(unsent(&queue, &state, &refused)[0].1, "Hôtel");
}

/// Q9: what its user may discard: held entries, those no key held here opens, those that wait on an
/// id of which nothing is stored, each named when its label opens.
#[test]
fn unsent_lists_what_this_build_will_not_send() {
    let (k1, stranger) = (generate_key(), generate_key());
    let state = holding(&k1, &[]);
    let mut raws: Vec<serde_json::Value> = every_kind(&k1, &k1).into_iter().map(|op| serde_json::to_value(op).unwrap()).collect();
    raws.push(serde_json::to_value(every_kind(&stranger, &k1).remove(0)).unwrap());
    raws.push(serde_json::json!({
        "id": Uuid::new_v4(), "label": "", "sealed_label": seal(&k1, "Taxi"),
        "op": { "type": "remove", "project_id": PROJECT, "slot": "expense", "eid": null, "row": 1, "base_rev": null, "base": null }
    }));
    let mut orphan = every_kind(&k1, &k1).remove(2);
    orphan.after = Some(Uuid::new_v4());
    orphan.sealed_label = Some(seal(&k1, "Train"));
    raws.push(serde_json::to_value(orphan).unwrap());
    let mut store = MemStore::new();
    for raw in &raws {
        assert!(write_entries(&mut store, std::slice::from_ref(raw)));
    }
    let queue = Snapshot::read(&mut store).queue();
    let names: Vec<String> = unsent(&queue, &state, &BTreeMap::new()).into_iter().map(|(_, name)| name).collect();
    assert_eq!(names, ["Hôtel", "Taxi", "Train"]);
}
