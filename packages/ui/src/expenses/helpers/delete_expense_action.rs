//! Deleting an expense, shared by the detail page's `⋯` menu and the swipe shortcut on the list.
//! Both need the same three things — an encrypted history entry, an offline fallback, and the
//! server call — and keeping them here is what stops the two paths drifting apart.

use api::expenses::expenses_controller::delete_expense;
use dioxus::fullstack::Json;
use dioxus::prelude::*;
use shared::{DeleteExpenseRequest, ExpenseType, HistoryContext, HistoryPayload, ProjectStatus};
use std::collections::VecDeque;
use uuid::Uuid;

use crate::common::{error_message, write_queue, Flash, OpKind, QueuedOp};
use crate::crypto::encrypt_json;

/// An archived project is frozen: nothing may be removed from it.
pub(crate) fn can_delete_expense(status: &ProjectStatus) -> bool {
    *status != ProjectStatus::Archived
}

/// A closed project keeps its transfers editable — that is how a settlement is corrected after the
/// fact — but nothing else.
pub(crate) fn can_edit_expense(status: &ProjectStatus, expense_type: &ExpenseType) -> bool {
    *status != ProjectStatus::Archived
        && (*status != ProjectStatus::Closed || *expense_type == ExpenseType::Transfer)
}

/// The summary is encrypted here: the server stores a project history it cannot read.
pub(crate) fn delete_expense_request(
    key: &[u8; 32],
    expense_id: i32,
    project_id: Uuid,
    // Already translated by the caller: this stays pure so it is testable without a runtime.
    history_summary: String,
    actor_user_id: i32,
) -> DeleteExpenseRequest {
    let history =
        encrypt_json(key, &HistoryPayload { summary: history_summary })
            .ok()
            .map(|payload| HistoryContext { actor_user_id, payload });
    DeleteExpenseRequest { id: expense_id, project_id, history }
}

/// Sends the deletion, or queues it when offline, then runs `on_done`.
///
/// `on_done` navigates away or refetches the list, so it runs only once the row is really gone: a
/// rejected delete raises the failure on `flash` and leaves the caller where it was. Queuing while
/// offline *is* a success — the op replays on reconnect.
///
/// `is_online`, `pending_ops` and `flash` are passed in rather than read here: this runs from an
/// event handler, and `use_context` is a hook.
pub(crate) fn run_delete_expense(
    req: DeleteExpenseRequest,
    label: String,
    is_online: bool,
    mut pending_ops: Signal<VecDeque<QueuedOp>>,
    mut flash: Signal<Option<Flash>>,
    on_done: impl FnOnce() + 'static,
) {
    if !is_online {
        pending_ops.write().push_back(QueuedOp {
            id: Uuid::new_v4(),
            label,
            op: OpKind::DeleteExpense(req),
        });
        write_queue(&pending_ops.read());
        on_done();
        return;
    }
    spawn(async move {
        match delete_expense(Json(req)).await {
            Ok(()) => on_done(),
            Err(e) => flash.set(Some(Flash::err(error_message(&e)))),
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_archived_project_allows_neither() {
        let s = ProjectStatus::Archived;
        assert!(!can_delete_expense(&s));
        assert!(!can_edit_expense(&s, &ExpenseType::Expense));
        assert!(!can_edit_expense(&s, &ExpenseType::Transfer));
    }

    #[test]
    fn a_closed_project_keeps_transfers_editable() {
        let s = ProjectStatus::Closed;
        assert!(can_delete_expense(&s));
        assert!(!can_edit_expense(&s, &ExpenseType::Expense));
        assert!(!can_edit_expense(&s, &ExpenseType::Gain));
        assert!(can_edit_expense(&s, &ExpenseType::Transfer));
    }

    #[test]
    fn an_ongoing_project_allows_everything() {
        let s = ProjectStatus::Ongoing;
        assert!(can_delete_expense(&s));
        assert!(can_edit_expense(&s, &ExpenseType::Expense));
        assert!(can_edit_expense(&s, &ExpenseType::Transfer));
    }

    #[test]
    fn the_request_carries_the_ids_and_an_encrypted_summary() {
        let key = [0x42u8; 32];
        let project_id = Uuid::nil();
        let req = delete_expense_request(&key, 7, project_id, "deleted: Pizza".into(), 3);
        assert_eq!(req.id, 7);
        assert_eq!(req.project_id, project_id);
        let history = req.history.expect("a summary is always attached");
        assert_eq!(history.actor_user_id, 3);
        let summary =
            crate::crypto::decrypt_json::<HistoryPayload>(&key, &history.payload).unwrap().summary;
        assert_eq!(summary, "deleted: Pizza");
    }
}
