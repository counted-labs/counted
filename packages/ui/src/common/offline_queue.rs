use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use shared::{
    CreatableExpense, DeleteExpenseRequest, EditableExpense, EditableProject, EncryptedPair, Expense,
    HistoryContext, ProjectDto, ProjectPayload, ProjectStatus,
};

use crate::crypto::{decrypt_json, encrypt_json, encrypt_padded_json};
use std::collections::VecDeque;
use uuid::Uuid;

use super::local_storage::project_key;
use super::persist::{read_json, write_json};

const QUEUE_KEY: &str = "counted_offline_queue";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueuedOp {
    pub id: Uuid,
    /// Only in queues an older build wrote, which kept the name in clear.
    #[serde(default)]
    pub label: String,
    /// The expense or project name, sealed under the project key like the op itself: the queue is
    /// a plain file or localStorage entry that outlives logout.
    #[serde(default)]
    pub sealed_label: Option<EncryptedPair>,
    pub op: OpKind,
}

impl QueuedOp {
    /// Every op is queued from a page that holds its project's key, so the key is in the store.
    pub fn new(id: Uuid, label: &str, op: OpKind) -> Self {
        let sealed_label = project_key(op.project_id()).and_then(|k| encrypt_json(&k, &label).ok());
        Self { id, label: String::new(), sealed_label, op }
    }

    /// Empty when the key is gone from this device since the op was queued.
    pub fn label(&self) -> String {
        match &self.sealed_label {
            Some(sealed) => project_key(self.op.project_id())
                .and_then(|k| decrypt_json::<String>(&k, sealed).ok())
                .unwrap_or_default(),
            None => self.label.clone(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum OpKind {
    AddExpense(CreatableExpense),
    EditExpense(EditableExpense),
    DeleteExpense(DeleteExpenseRequest),
    /// A whole payload built when queued. Only replayed from queues an older build wrote: it
    /// overwrites anything written to the project in the meantime.
    UpdateProject(EditableProject),
    EditProject(ProjectEdit),
}

impl OpKind {
    pub fn project_id(&self) -> Uuid {
        match self {
            OpKind::AddExpense(p) => p.project_id,
            OpKind::EditExpense(p) => p.project_id,
            OpKind::DeleteExpense(p) => p.project_id,
            OpKind::UpdateProject(p) => p.id,
            OpKind::EditProject(e) => e.project_id,
        }
    }
}

/// A project edit queued offline, kept as what the user changed rather than as a payload, so the
/// replay applies it to whatever the project holds by then instead of overwriting it.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProjectEdit {
    pub project_id: Uuid,
    /// Name and description travel together: the form edits both, and an `Option<Option<_>>`
    /// would lose "description cleared" through JSON. Sealed under the project key, since the queue
    /// is a plain file or localStorage entry that outlives logout.
    #[serde(default)]
    pub name_description: Option<EncryptedPair>,
    #[serde(default)]
    pub status: Option<ProjectStatus>,
    #[serde(default)]
    pub history: Option<HistoryContext>,
}

impl ProjectEdit {
    /// The write that applies this edit to `stored`, as the online paths build it: a status goes
    /// in the payload - see `status_edit`.
    pub fn apply(&self, key: &[u8; 32], stored: &ProjectDto) -> Result<EditableProject, String> {
        let mut payload: ProjectPayload = decrypt_json(key, &stored.payload)?;
        let mut changed = false;
        if let Some(sealed) = &self.name_description {
            let (name, description): (String, Option<String>) = decrypt_json(key, sealed)?;
            payload.name = name;
            payload.description = description;
            changed = true;
        }
        if let Some(status) = &self.status {
            payload.status = Some(status.clone());
            changed = true;
        }
        Ok(EditableProject {
            id: self.project_id,
            payload: if changed { Some(encrypt_padded_json(key, &payload)?) } else { None },
            status: (self.status.is_some() && stored.status != ProjectStatus::Ongoing)
                .then_some(ProjectStatus::Ongoing),
            history: self.history.clone(),
            expected_iv: None,
            notify_closed: self.status == Some(ProjectStatus::Closed),
        })
    }
}

/// Only the tests need the resolved path; the read/write pair goes through `persist` by key.
#[cfg(all(test, not(target_arch = "wasm32")))]
fn queue_file() -> std::path::PathBuf {
    super::persist::file_for(QUEUE_KEY)
}

/// The id an expense queued offline is listed under until its replay lands. No server row has it:
/// `expenses.id` is a SERIAL, so it is never negative.
pub fn is_pending_expense(id: i32) -> bool {
    id < 0
}

/// The expenses queued offline for `project_id`, as rows the page lists like any other: the queued
/// ciphertext carries its shares, so totals and balances count them before the replay. One leaves
/// this list when the replay pops its op, and the refetch that follows brings the server's row.
pub fn queued_expenses(queue: &VecDeque<QueuedOp>, project_id: Uuid, now: NaiveDateTime) -> Vec<Expense> {
    queue
        .iter()
        .enumerate()
        .filter_map(|(position, queued)| match &queued.op {
            OpKind::AddExpense(c) if c.project_id == project_id => Some(Expense {
                id: -(position as i32) - 1,
                author_id: c.author_id,
                project_id,
                created_at: now,
                payload: c.payload.clone(),
            }),
            _ => None,
        })
        .collect()
}

pub fn read_queue() -> VecDeque<QueuedOp> {
    read_json(QUEUE_KEY)
}

pub fn write_queue(queue: &VecDeque<QueuedOp>) {
    write_json(QUEUE_KEY, "offline queue", queue);
}

pub fn clear_queue() {
    super::persist::remove_store(QUEUE_KEY);
}

#[cfg(test)]
mod tests {
    use super::*;
    use shared::{EncryptedPair, ProjectStatus};

    fn ep() -> EncryptedPair {
        EncryptedPair { ct: "ct".into(), iv: "iv".into() }
    }

    fn add_expense_op(label: &str) -> QueuedOp {
        let id = Uuid::new_v4();
        QueuedOp {
            id,
            label: label.to_string(),
            sealed_label: None,
            op: OpKind::AddExpense(shared::CreatableExpense {
                project_id: Uuid::nil(),
                author_id: Some(1),
                payload: ep(),
                payers: vec![],
                debtors: vec![],
                history: None,
                client_op_id: Some(id),
            }),
        }
    }

    /// The idempotency key must survive the round trip through disk, or a replay after a restart
    /// would be the duplicate it exists to prevent.
    #[test]
    fn a_queued_add_keeps_its_client_op_id_across_persistence() {
        let op = add_expense_op("a");
        let json = serde_json::to_string(&op).unwrap();
        let loaded: QueuedOp = serde_json::from_str(&json).unwrap();

        match loaded.op {
            OpKind::AddExpense(p) => assert_eq!(p.client_op_id, Some(op.id)),
            _ => panic!("not an add"),
        }
    }

    #[test]
    fn only_this_projects_queued_adds_are_listed_and_under_pending_ids() {
        let other = Uuid::new_v4();
        let mut elsewhere = add_expense_op("elsewhere");
        if let OpKind::AddExpense(c) = &mut elsewhere.op {
            c.project_id = other;
        }
        let queue: VecDeque<QueuedOp> =
            [add_expense_op("a"), edit_expense_op(), elsewhere, add_expense_op("b")].into();
        let now = chrono::NaiveDate::from_ymd_opt(2026, 10, 17).unwrap().and_hms_opt(18, 42, 0).unwrap();

        let rows = queued_expenses(&queue, Uuid::nil(), now);

        assert_eq!(rows.iter().map(|e| e.id).collect::<Vec<_>>(), vec![-1, -4]);
        assert!(rows.iter().all(|e| is_pending_expense(e.id) && e.project_id == Uuid::nil()));
        assert_eq!(rows[0].payload, ep());
        assert_eq!(queued_expenses(&queue, other, now).iter().map(|e| e.id).collect::<Vec<_>>(), vec![-3]);
    }

    fn edit_expense_op() -> QueuedOp {
        QueuedOp {
            id: Uuid::new_v4(),
            label: "edit".into(),
            sealed_label: None,
            op: OpKind::EditExpense(shared::EditableExpense {
                id: 42,
                project_id: Uuid::nil(),
                author_id: Some(1),
                payload: ep(),
                payers: vec![],
                debtors: vec![],
                history: None,
            }),
        }
    }

    fn delete_expense_op() -> QueuedOp {
        QueuedOp {
            id: Uuid::new_v4(),
            label: "delete".into(),
            sealed_label: None,
            op: OpKind::DeleteExpense(shared::DeleteExpenseRequest {
                id: 7,
                project_id: Uuid::nil(),
                history: None,
            }),
        }
    }

    fn update_project_op() -> QueuedOp {
        QueuedOp {
            id: Uuid::new_v4(),
            label: "project".into(),
            sealed_label: None,
            op: OpKind::UpdateProject(shared::EditableProject {
                id: Uuid::nil(),
                payload: None,
                status: Some(ProjectStatus::Closed),
                history: None,
                expected_iv: None,
                notify_closed: false,
            }),
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    mod native {
        use super::*;

        // with_data_dir takes the process-wide env_lock - local_storage's and export's tests set
        // the same COUNTED_DATA_DIR, so a module-local lock would not keep them apart. It also
        // starts from an empty dir, so no leftover queue file from a previous run can leak in.
        use crate::common::env_lock;
        use crate::common::test_fixtures::with_data_dir;

        fn with_tmp_dir<F: FnOnce()>(suffix: &str, f: F) {
            with_data_dir(&format!("queue_{suffix}"), |_| f())
        }

        #[test]
        fn a_queued_name_is_sealed_under_the_project_key() {
            with_tmp_dir("sealed_label", || {
                let project_id = Uuid::from_u128(7);
                let key = crate::crypto::generate_key();
                let mut state = crate::common::LocalStorageState::default();
                crate::common::upsert_project(&mut state, project_id, None);
                crate::common::upsert_project_key(&mut state, project_id, crate::crypto::key_to_fragment(&key));
                crate::common::write_to_ls(&state);

                let edit = ProjectEdit { project_id, name_description: None, status: Some(ProjectStatus::Closed), history: None };
                let op = QueuedOp::new(Uuid::new_v4(), "Corse", OpKind::EditProject(edit));

                assert!(!serde_json::to_string(&op).unwrap().contains("Corse"), "the queue holds no plaintext name");
                assert_eq!(op.label(), "Corse", "and the conflict message still names it");
            });
        }

        /// A queue written before the label was sealed must still load and still name its ops.
        #[test]
        fn a_queue_from_an_older_build_keeps_its_clear_label() {
            let old = r#"{"id":"00000000-0000-0000-0000-000000000001","label":"Café","op":{"type":"delete_expense","id":3,"projectId":"00000000-0000-0000-0000-000000000002"}}"#;
            let op: QueuedOp = serde_json::from_str(old).unwrap();
            assert_eq!(op.label(), "Café");
        }

        #[test]
        fn test_read_queue_empty_when_no_file() {
            with_tmp_dir("no_file", || {
                let q = read_queue();
                assert!(q.is_empty());
            });
        }

        #[test]
        fn test_write_read_roundtrip_single_op() {
            with_tmp_dir("roundtrip_single", || {
                let mut queue = VecDeque::new();
                queue.push_back(add_expense_op("Café"));
                write_queue(&queue);
                let loaded = read_queue();
                assert_eq!(loaded.len(), 1);
                assert_eq!(loaded.front().unwrap().label, "Café");
            });
        }

        #[test]
        fn test_write_read_preserves_fifo_order() {
            with_tmp_dir("fifo_order", || {
                let mut queue = VecDeque::new();
                queue.push_back(add_expense_op("first"));
                queue.push_back(add_expense_op("second"));
                queue.push_back(add_expense_op("third"));
                write_queue(&queue);
                let loaded = read_queue();
                assert_eq!(loaded.len(), 3);
                let labels: Vec<&str> = loaded.iter().map(|op| op.label.as_str()).collect();
                assert_eq!(labels, vec!["first", "second", "third"]);
            });
        }

        #[test]
        fn test_write_empty_queue_roundtrip() {
            with_tmp_dir("empty_roundtrip", || {
                let queue: VecDeque<QueuedOp> = VecDeque::new();
                write_queue(&queue);
                let loaded = read_queue();
                assert!(loaded.is_empty());
            });
        }

        #[test]
        fn test_second_write_overwrites_first() {
            with_tmp_dir("overwrite", || {
                let mut q1 = VecDeque::new();
                q1.push_back(add_expense_op("original"));
                write_queue(&q1);

                let mut q2 = VecDeque::new();
                q2.push_back(add_expense_op("replacement"));
                write_queue(&q2);

                let loaded = read_queue();
                assert_eq!(loaded.len(), 1);
                assert_eq!(loaded.front().unwrap().label, "replacement");
            });
        }

        #[test]
        fn test_write_creates_missing_dir() {
            // Robustness: write_queue must create missing parent dirs.
            let _guard = env_lock();
            let base = std::env::temp_dir().join("counted_queue_missing_dir");
            std::fs::remove_dir_all(&base).ok();
            let nested = base.join("x").join("y"); // does not exist yet
            std::env::set_var("COUNTED_DATA_DIR", nested.to_str().unwrap());

            let mut queue = VecDeque::new();
            queue.push_back(add_expense_op("Café"));
            write_queue(&queue);
            let loaded = read_queue();
            let file_exists = nested.join("counted_offline_queue.json").exists();

            std::env::remove_var("COUNTED_DATA_DIR");
            std::fs::remove_dir_all(&base).ok();

            assert!(file_exists, "write_queue should create missing parent dirs");
            assert_eq!(loaded.len(), 1);
        }

        #[test]
        fn test_write_atomic_no_tmp_left() {
            // Robustness: atomic write (tmp + rename) must not leave a .json.tmp behind.
            with_tmp_dir("atomic", || {
                let mut queue = VecDeque::new();
                queue.push_back(add_expense_op("x"));
                write_queue(&queue);

                let tmp_left = queue_file().with_extension("json.tmp").exists();
                let loaded = read_queue();

                assert!(!tmp_left, "temp file must not remain after atomic write");
                assert_eq!(loaded.len(), 1);
            });
        }

        #[test]
        fn test_corrupt_json_returns_empty() {
            with_tmp_dir("corrupt", || {
                let path = queue_file();
                std::fs::write(&path, b"this is not json {{{").unwrap();
                let loaded = read_queue();
                assert!(loaded.is_empty());
            });
        }

        #[test]
        fn test_all_opkind_variants_roundtrip() {
            with_tmp_dir("all_variants", || {
                let mut queue = VecDeque::new();
                queue.push_back(add_expense_op("add"));
                queue.push_back(edit_expense_op());
                queue.push_back(delete_expense_op());
                queue.push_back(update_project_op());
                write_queue(&queue);

                let loaded = read_queue();
                assert_eq!(loaded.len(), 4);
                assert!(matches!(loaded[0].op, OpKind::AddExpense(_)));
                assert!(matches!(loaded[1].op, OpKind::EditExpense(_)));
                assert!(matches!(loaded[2].op, OpKind::DeleteExpense(_)));
                assert!(matches!(loaded[3].op, OpKind::UpdateProject(_)));
            });
        }

        #[test]
        fn test_opkind_json_tag_format() {
            // Verify serde(tag = "type", rename_all = "snake_case") produces correct JSON keys.
            let ops = [
                add_expense_op("a"),
                edit_expense_op(),
                delete_expense_op(),
                update_project_op(),
            ];
            let expected_types = ["add_expense", "edit_expense", "delete_expense", "update_project"];
            for (op, expected) in ops.iter().zip(expected_types.iter()) {
                let json = serde_json::to_string(&op.op).unwrap();
                let v: serde_json::Value = serde_json::from_str(&json).unwrap();
                assert_eq!(v["type"].as_str().unwrap(), *expected, "wrong type tag for {expected}");
            }
        }

        #[test]
        fn test_uuid_preserved_after_roundtrip() {
            with_tmp_dir("uuid_preserved", || {
                let id = Uuid::parse_str("550e8400-e29b-41d4-a716-446655440099").unwrap();
                let op = QueuedOp {
                    id,
                    label: "uuid test".into(),
                    sealed_label: None,
                    op: OpKind::DeleteExpense(shared::DeleteExpenseRequest {
                        id: 1,
                        project_id: Uuid::nil(),
                        history: None,
                    }),
                };
                let mut queue = VecDeque::new();
                queue.push_back(op);
                write_queue(&queue);

                let loaded = read_queue();
                assert_eq!(loaded.front().unwrap().id, id);
            });
        }

        #[test]
        fn test_pop_front_then_write_removes_op() {
            with_tmp_dir("pop_front", || {
                let mut queue = VecDeque::new();
                queue.push_back(add_expense_op("to remove"));
                queue.push_back(add_expense_op("to keep"));
                write_queue(&queue);

                queue.pop_front();
                write_queue(&queue);

                let loaded = read_queue();
                assert_eq!(loaded.len(), 1);
                assert_eq!(loaded.front().unwrap().label, "to keep");
            });
        }
    }

    // These tests run on all targets (pure logic, no I/O)
    #[test]
    fn test_queued_op_clone() {
        let op = add_expense_op("clone test");
        let cloned = op.clone();
        assert_eq!(op.label, cloned.label);
        assert_eq!(op.id, cloned.id);
    }

    #[test]
    fn test_opkind_serde_roundtrip_in_memory() {
        for op in [add_expense_op("a"), edit_expense_op(), delete_expense_op(), update_project_op()] {
            let json = serde_json::to_string(&op).unwrap();
            let back: QueuedOp = serde_json::from_str(&json).unwrap();
            assert_eq!(op.id, back.id);
            assert_eq!(op.label, back.label);
        }
    }
}

#[cfg(test)]
mod project_edit_tests {
    use super::*;
    use crate::common::test_fixtures::{make_project, test_key};
    use crate::crypto::encrypt_json;

    fn stored_with(key: &[u8; 32], f: impl FnOnce(&mut ProjectPayload)) -> ProjectDto {
        let mut p = make_project(key, Uuid::nil(), "Corsica", "EUR");
        let mut payload: ProjectPayload = decrypt_json(key, &p.payload).unwrap();
        f(&mut payload);
        p.payload = encrypt_json(key, &payload).unwrap();
        p
    }

    fn payload_of(key: &[u8; 32], e: &EditableProject) -> ProjectPayload {
        decrypt_json(key, e.payload.as_ref().unwrap()).unwrap()
    }

    fn rename(name: &str, description: Option<&str>) -> ProjectEdit {
        let sealed = encrypt_json(&test_key(), &(name, description)).unwrap();
        ProjectEdit {
            project_id: Uuid::nil(),
            name_description: Some(sealed),
            status: None,
            history: None,
        }
    }

    /// The point of queueing the intent: a field another device wrote while this one was offline
    /// is still there after the replay.
    #[test]
    fn a_replayed_rename_keeps_what_was_written_meanwhile() {
        let k = test_key();
        let stored = stored_with(&k, |p| {
            p.extra.insert("kind".into(), "personal".into());
            p.extra.insert("finance".into(), serde_json::json!({ "budgets": [1] }));
        });
        let written = rename("Corse", Some("été")).apply(&k, &stored).unwrap();
        let after = payload_of(&k, &written);
        assert_eq!(after.name, "Corse");
        assert_eq!(after.description.as_deref(), Some("été"));
        assert!(after.is_personal());
        assert_eq!(after.extra["finance"], serde_json::json!({ "budgets": [1] }));
        assert_eq!(after.currency, "EUR");
    }

    #[test]
    fn a_cleared_description_survives_the_queue_on_disk() {
        let edit = rename("Corse", None);
        let back: ProjectEdit = serde_json::from_str(&serde_json::to_string(&edit).unwrap()).unwrap();
        assert_eq!(back, edit);
        let k = test_key();
        let stored = stored_with(&k, |p| p.description = Some("old".into()));
        assert_eq!(payload_of(&k, &back.apply(&k, &stored).unwrap()).description, None);
    }

    #[test]
    fn a_replayed_status_goes_in_the_payload_and_clears_the_column() {
        let k = test_key();
        let close = ProjectEdit {
            project_id: Uuid::nil(),
            name_description: None,
            status: Some(ProjectStatus::Closed),
            history: None,
        };
        let mut legacy = stored_with(&k, |p| p.status = None);
        legacy.status = ProjectStatus::Archived;
        let written = close.apply(&k, &legacy).unwrap();
        assert_eq!(payload_of(&k, &written).status, Some(ProjectStatus::Closed));
        assert_eq!(written.status, Some(ProjectStatus::Ongoing));
        assert!(written.notify_closed);

        let renamed = rename("Corse", None).apply(&k, &legacy).unwrap();
        assert_eq!(renamed.status, None, "an edit without a status leaves the column to conversion");
        assert!(!renamed.notify_closed);
    }

    #[test]
    fn a_queued_intent_round_trips_as_an_op() {
        let edit = rename("Corse", Some("Sous la tente"));
        let op = QueuedOp { id: Uuid::new_v4(), label: "Corse".into(), sealed_label: None, op: OpKind::EditProject(edit.clone()) };
        let stored = serde_json::to_string(&op).unwrap();
        let back: QueuedOp = serde_json::from_str(&stored).unwrap();
        assert!(matches!(back.op, OpKind::EditProject(e) if e == edit));
        assert!(!stored.contains("Sous la tente"), "the queue holds no plaintext description");
    }
}
