use api::expenses::expenses_controller::{add_expense, delete_expense, edit_expense};
use api::projects::projects_controller::update_project_by_id;
use dioxus::fullstack::Json;
use dioxus::prelude::*;
use std::collections::VecDeque;

use super::error_utils::error_message;
use super::offline_queue::{write_queue, OpKind, QueuedOp};

/// Why a replay could not finish, as data rather than a sentence.
///
/// `replay_queue` runs inside a spawned task, where `tid!` would have to reach the i18n context
/// through `consume_context`. Handing the variant back and translating in `AppLayout` keeps the
/// lookup on the render path, and keeps every key a literal the `i18n` scanner can check.
#[derive(Clone, PartialEq)]
pub enum SyncFailure {
    /// The queued edit/delete/other targets something the server no longer has. Carries the label.
    ConflictEdit(String),
    ConflictDelete(String),
    ConflictOther(String),
    /// Anything else — the op is preserved and the drain stops. Carries the server's message.
    Error(String),
}

// Drains the queue FIFO, executing each op against the server.
// Returns true if the queue was fully drained, false if stopped due to a non-conflict error.
// 404-like conflicts are discarded with a toast; other errors halt and preserve the op.
pub async fn replay_queue(
    mut pending_ops: Signal<VecDeque<QueuedOp>>,
    mut conflict_msg: Signal<Option<SyncFailure>>,
) -> bool {
    loop {
        let Some(queued) = pending_ops.read().front().cloned() else {
            return true;
        };

        let result: Result<(), String> = match queued.op.clone() {
            OpKind::AddExpense(p) => {
                add_expense(Json(p)).await.map(|_| ()).map_err(|e| error_message(&e))
            }
            OpKind::EditExpense(p) => {
                edit_expense(Json(p)).await.map(|_| ()).map_err(|e| error_message(&e))
            }
            OpKind::DeleteExpense(p) => {
                delete_expense(Json(p)).await.map(|_| ()).map_err(|e| error_message(&e))
            }
            OpKind::UpdateProject(p) => {
                update_project_by_id(Json(p)).await.map(|_| ()).map_err(|e| error_message(&e))
            }
        };

        match result {
            Ok(()) => {
                pending_ops.write().pop_front();
                write_queue(&pending_ops.read());
            }
            Err(ref e) if is_conflict(e) => {
                let label = queued.label.clone();
                conflict_msg.set(Some(match &queued.op {
                    OpKind::EditExpense(_) => SyncFailure::ConflictEdit(label),
                    OpKind::DeleteExpense(_) => SyncFailure::ConflictDelete(label),
                    _ => SyncFailure::ConflictOther(label),
                }));
                pending_ops.write().pop_front();
                write_queue(&pending_ops.read());
            }
            Err(e) => {
                conflict_msg.set(Some(SyncFailure::Error(e)));
                return false;
            }
        }
    }
}

fn is_conflict(err: &str) -> bool {
    err.contains("404") || err.contains("not found") || err.contains("Not Found")
}

#[cfg(test)]
mod tests {
    use super::is_conflict;

    #[test]
    fn test_conflict_on_404() {
        assert!(is_conflict("server returned 404"));
    }

    #[test]
    fn test_conflict_on_404_standalone() {
        assert!(is_conflict("404"));
    }

    #[test]
    fn test_conflict_on_not_found_lowercase() {
        assert!(is_conflict("not found"));
    }

    #[test]
    fn test_conflict_on_not_found_in_sentence() {
        assert!(is_conflict("expense not found in database"));
    }

    #[test]
    fn test_conflict_on_not_found_titlecase() {
        assert!(is_conflict("Not Found"));
    }

    #[test]
    fn test_conflict_on_http_404_not_found() {
        assert!(is_conflict("HTTP 404 Not Found"));
    }

    #[test]
    fn test_no_conflict_on_network_error() {
        assert!(!is_conflict("error sending request"));
    }

    #[test]
    fn test_no_conflict_on_connection_refused() {
        assert!(!is_conflict("connection refused"));
    }

    #[test]
    fn test_no_conflict_on_timeout() {
        assert!(!is_conflict("request timed out"));
    }

    #[test]
    fn test_no_conflict_on_auth_error() {
        assert!(!is_conflict("401 Unauthorized"));
    }

    #[test]
    fn test_no_conflict_on_server_error() {
        assert!(!is_conflict("500 Internal Server Error"));
    }

    #[test]
    fn test_no_conflict_on_empty_string() {
        assert!(!is_conflict(""));
    }

    #[test]
    fn test_no_conflict_on_unrelated_message() {
        assert!(!is_conflict("serialization failed"));
    }
}
