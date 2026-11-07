use api::expenses::expenses_controller::{add_expense, delete_expense, edit_expense};
use api::projects::projects_controller::update_project_by_id;
use dioxus::fullstack::Json;
use dioxus::prelude::*;
use shared::ExpenseWithPayments;
use std::collections::VecDeque;

use super::error_utils::{error_message, is_rate_limited, RATE_LIMIT_RETRY_MS, REQUEST_PACE_MS};
use super::sleep;
use super::local_storage::project_key;
use super::offline_queue::{drop_with_dependents, write_queue, OpKind, QueuedOp};
use super::project_write::update_project_fresh;

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
    /// Anything else - the op is preserved and the drain stops. Carries the server's message.
    Error(String),
}

/// What the server wrote for an expense op, handed to `on_landed` before the op leaves the queue,
/// so the row listed under a pending id is replaced in the same render instead of vanishing until
/// a refetch.
#[derive(Clone, PartialEq)]
pub enum Landed {
    Added(ExpenseWithPayments),
    Edited(ExpenseWithPayments),
    Deleted(i32),
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct ReplayEnd {
    /// False when an error stopped the drain; the op that hit it is still at the front.
    pub drained: bool,
    /// A project op landed or an op was dropped: only a refetch settles what the page shows.
    pub refetch: bool,
}

// Drains the queue FIFO, executing each op against the server.
// 404-like conflicts are discarded with a toast; other errors halt and preserve the op.
pub async fn replay_queue(
    mut pending_ops: Signal<VecDeque<QueuedOp>>,
    mut conflict_msg: Signal<Option<SyncFailure>>,
    on_landed: Callback<Landed>,
) -> ReplayEnd {
    let mut first = true;
    let mut refetch = false;
    loop {
        let Some(queued) = pending_ops.read().front().cloned() else {
            return ReplayEnd { drained: true, refetch };
        };
        if is_legacy_overwrite(&queued.op) {
            conflict_msg.set(Some(SyncFailure::ConflictOther(queued.label())));
            drop_with_dependents(&mut pending_ops.write(), queued.id);
            write_queue(&pending_ops.read());
            refetch = true;
            continue;
        }
        if !first {
            sleep(REQUEST_PACE_MS).await;
        }
        first = false;

        // nginx refuses before any of these endpoints runs, so the op was not applied.
        let result = match replay(&queued.op).await {
            Err(e) if is_rate_limited(&e) => {
                sleep(RATE_LIMIT_RETRY_MS).await;
                replay(&queued.op).await
            }
            result => result,
        };

        match result {
            Ok(landed) => {
                match landed {
                    Some(landed) => on_landed.call(landed),
                    None => refetch = true,
                }
                pending_ops.write().retain(|op| op.id != queued.id);
                write_queue(&pending_ops.read());
            }
            Err(ref e) if is_conflict(e) => {
                let label = queued.label();
                conflict_msg.set(Some(match &queued.op {
                    OpKind::EditExpense(_) => SyncFailure::ConflictEdit(label),
                    OpKind::DeleteExpense(_) => SyncFailure::ConflictDelete(label),
                    _ => SyncFailure::ConflictOther(label),
                }));
                drop_with_dependents(&mut pending_ops.write(), queued.id);
                write_queue(&pending_ops.read());
                refetch = true;
            }
            Err(e) => {
                conflict_msg.set(Some(SyncFailure::Error(error_message(&e))));
                return ReplayEnd { drained: false, refetch };
            }
        }
    }
}

async fn replay(op: &OpKind) -> Result<Option<Landed>, ServerFnError> {
    match op.clone() {
        OpKind::AddExpense(p) => {
            crate::common::project_token::scope(p.project_id);
            add_expense(Json(p)).await.map(|c| Some(Landed::Added(c)))
        }
        OpKind::EditExpense(p) => {
            crate::common::project_token::scope(p.project_id);
            edit_expense(Json(p)).await.map(|c| Some(Landed::Edited(c)))
        }
        OpKind::DeleteExpense(p) => {
            crate::common::project_token::scope(p.project_id);
            let id = p.id;
            delete_expense(Json(p)).await.map(|_| Some(Landed::Deleted(id)))
        }
        OpKind::UpdateProject(p) => {
            crate::common::project_token::scope(p.id);
            update_project_by_id(Json(p)).await.map(|_| None)
        }
        // Rebuilt on the payload the server holds now, so a field written meanwhile - by
        // another device or a newer build - survives the replay.
        OpKind::EditProject(edit) => match project_key(edit.project_id) {
            Some(key) => update_project_fresh(edit.project_id, |stored| edit.apply(&key, stored))
                .await
                .map(|_| None),
            None => Err(ServerFnError::ServerError {
                message: shared::PROJECT_NOT_FOUND.to_string(),
                code: 404,
                details: None,
            }),
        },
    }
}

/// A whole payload queued by a build before 3.2.5, the last to write `UpdateProject`. It carries no
/// `expected_iv`, which the server refuses with a payload, and was built on a payload long replaced.
fn is_legacy_overwrite(op: &OpKind) -> bool {
    matches!(op, OpKind::UpdateProject(p) if p.payload.is_some())
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
    use super::{is_conflict, is_legacy_overwrite, OpKind};
    use dioxus::prelude::ServerFnError;
    use shared::{EditableProject, EncryptedPair, ProjectStatus};
    use uuid::Uuid;

    fn update_project(payload: Option<EncryptedPair>) -> OpKind {
        OpKind::UpdateProject(EditableProject {
            id: Uuid::nil(),
            payload,
            status: Some(ProjectStatus::Closed),
            history: None,
            expected_iv: None,
            notify_closed: false,
        })
    }

    #[test]
    fn a_legacy_whole_payload_is_dropped() {
        assert!(is_legacy_overwrite(&update_project(Some(EncryptedPair { ct: "ct".into(), iv: "iv".into() }))));
    }

    #[test]
    fn a_legacy_status_alone_still_replays() {
        assert!(!is_legacy_overwrite(&update_project(None)));
    }

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
