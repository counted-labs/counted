use serde::{Deserialize, Serialize};
use shared::{CreatableExpense, DeleteExpenseRequest, EditableExpense, EditableProject};
use std::collections::VecDeque;
use uuid::Uuid;

use super::persist::{read_json, write_json};

const QUEUE_KEY: &str = "counted_offline_queue";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueuedOp {
    pub id: Uuid,
    pub label: String,
    pub op: OpKind,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum OpKind {
    AddExpense(CreatableExpense),
    EditExpense(EditableExpense),
    DeleteExpense(DeleteExpenseRequest),
    UpdateProject(EditableProject),
}

/// Only the tests need the resolved path; the read/write pair goes through `persist` by key.
#[cfg(all(test, not(target_arch = "wasm32")))]
fn queue_file() -> std::path::PathBuf {
    super::persist::file_for(QUEUE_KEY)
}

pub fn read_queue() -> VecDeque<QueuedOp> {
    read_json(QUEUE_KEY)
}

pub fn write_queue(queue: &VecDeque<QueuedOp>) {
    write_json(QUEUE_KEY, "offline queue", queue);
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
            op: OpKind::AddExpense(shared::CreatableExpense {
                project_id: Uuid::nil(),
                author_id: 1,
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

    fn edit_expense_op() -> QueuedOp {
        QueuedOp {
            id: Uuid::new_v4(),
            label: "edit".into(),
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
            op: OpKind::UpdateProject(shared::EditableProject {
                id: Uuid::nil(),
                payload: None,
                status: Some(ProjectStatus::Closed),
                history: None,
            }),
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    mod native {
        use super::*;

        // with_data_dir takes the process-wide env_lock — local_storage's and export's tests set
        // the same COUNTED_DATA_DIR, so a module-local lock would not keep them apart. It also
        // starts from an empty dir, so no leftover queue file from a previous run can leak in.
        use crate::common::env_lock;
        use crate::common::test_fixtures::with_data_dir;

        fn with_tmp_dir<F: FnOnce()>(suffix: &str, f: F) {
            with_data_dir(&format!("queue_{suffix}"), |_| f())
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
