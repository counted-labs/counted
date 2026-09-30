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

        let result: Result<(), ServerFnError> = match queued.op.clone() {
            OpKind::AddExpense(p) => add_expense(Json(p)).await.map(|_| ()),
            OpKind::EditExpense(p) => edit_expense(Json(p)).await.map(|_| ()),
            OpKind::DeleteExpense(p) => delete_expense(Json(p)).await.map(|_| ()),
            OpKind::UpdateProject(p) => update_project_by_id(Json(p)).await.map(|_| ()),
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
                conflict_msg.set(Some(SyncFailure::Error(error_message(&e))));
                return false;
            }
        }
    }
}

/// Matched on the variant, never the text: `error_message` returns translated copy, which no
/// substring check can recognise in every locale.
fn is_conflict(err: &ServerFnError) -> bool {
    matches!(
        err,
        ServerFnError::ServerError { message, code: 404, .. }
            if message == shared::errors::EXPENSE_NOT_FOUND || message == shared::PROJECT_NOT_FOUND
    )
}

#[cfg(test)]
mod tests {
    use super::is_conflict;
    use dioxus::prelude::ServerFnError;

    fn server_error(message: &str, code: u16) -> ServerFnError {
        ServerFnError::ServerError { message: message.into(), code, details: None }
    }

    #[test]
    fn test_conflict_on_expense_not_found() {
        assert!(is_conflict(&server_error(shared::errors::EXPENSE_NOT_FOUND, 404)));
    }

    #[test]
    fn test_conflict_on_project_not_found() {
        assert!(is_conflict(&server_error(shared::PROJECT_NOT_FOUND, 404)));
    }

    #[test]
    fn test_no_conflict_on_not_found_text_without_404() {
        assert!(!is_conflict(&server_error(shared::errors::EXPENSE_NOT_FOUND, 500)));
    }

    #[test]
    fn test_no_conflict_on_unrelated_404() {
        assert!(!is_conflict(&server_error(shared::errors::USER_NOT_FOUND, 404)));
    }

    #[test]
    fn test_no_conflict_on_other_server_error() {
        assert!(!is_conflict(&ServerFnError::new("boom")));
    }

    #[test]
    fn test_no_conflict_on_stream_error() {
        assert!(!is_conflict(&ServerFnError::StreamError("reset".into())));
    }
}
