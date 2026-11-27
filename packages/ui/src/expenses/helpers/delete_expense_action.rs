//! Deleting an expense, shared by the detail page's `⋯` menu and the swipe shortcut on the list.
//! Both need the same two things - an encrypted history entry and the queued op - and keeping them
//! here is what stops the two paths drifting apart.

use dioxus::prelude::*;
use shared::{DeleteExpenseRequest, ExpenseType, HistoryAction, HistoryEntity, HistoryPayload, ProjectStatus};
use uuid::Uuid;

use crate::common::{enqueue, OpKind, Queue, QueuedOp};
use crate::entity_ids::sealed_history;

/// An archived project is frozen: nothing may be removed from it. Neither may the read-only demo.
pub(crate) fn can_delete_expense(status: &ProjectStatus, read_only: bool) -> bool {
    !read_only && *status != ProjectStatus::Archived
}

/// A closed project keeps its transfers editable - that is how a settlement is corrected after the
/// fact - but nothing else.
pub(crate) fn can_edit_expense(
    status: &ProjectStatus,
    read_only: bool,
    expense_type: &ExpenseType,
) -> bool {
    !read_only
        && *status != ProjectStatus::Archived
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
    let history = sealed_history(
        key,
        HistoryPayload {
            summary: history_summary,
            actor_user_id: Some(actor_user_id),
            action: Some(HistoryAction::Deleted),
            entity: Some(HistoryEntity::Expense),
            entity_id: Some(expense_id),
            extra: Default::default(),
        },
    )
    .ok();
    DeleteExpenseRequest { id: expense_id, project_id, history, eid: None, expected_rev: None }
}

/// Queues the deletion, online or not, then runs `on_done` at once: the row leaves the list now and
/// the queue sends the request. A refused delete surfaces as the queue's toast, the op kept; one the
/// queue could not store runs nothing, the user told by the queue.
///
/// `pending_ops` is passed in rather than read here: this runs from an event handler, and
/// `use_context` is a hook.
pub(crate) fn run_delete_expense(
    req: DeleteExpenseRequest,
    label: String,
    pending_ops: Signal<Queue>,
    on_done: impl FnOnce() + 'static,
) {
    if enqueue(pending_ops, QueuedOp::new(Uuid::new_v4(), &label, OpKind::DeleteExpense(req))) {
        on_done();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_archived_project_allows_neither() {
        let s = ProjectStatus::Archived;
        assert!(!can_delete_expense(&s, false));
        assert!(!can_edit_expense(&s, false, &ExpenseType::Expense));
        assert!(!can_edit_expense(&s, false, &ExpenseType::Transfer));
    }

    #[test]
    fn a_closed_project_keeps_transfers_editable() {
        let s = ProjectStatus::Closed;
        assert!(can_delete_expense(&s, false));
        assert!(!can_edit_expense(&s, false, &ExpenseType::Expense));
        assert!(!can_edit_expense(&s, false, &ExpenseType::Gain));
        assert!(can_edit_expense(&s, false, &ExpenseType::Transfer));
    }

    #[test]
    fn an_ongoing_project_allows_everything() {
        let s = ProjectStatus::Ongoing;
        assert!(can_delete_expense(&s, false));
        assert!(can_edit_expense(&s, false, &ExpenseType::Expense));
        assert!(can_edit_expense(&s, false, &ExpenseType::Transfer));
    }

    #[test]
    fn a_read_only_project_allows_neither() {
        let s = ProjectStatus::Ongoing;
        assert!(!can_delete_expense(&s, true));
        assert!(!can_edit_expense(&s, true, &ExpenseType::Expense));
        assert!(!can_edit_expense(&s, true, &ExpenseType::Transfer));
    }

    #[test]
    fn the_request_carries_the_ids_and_an_encrypted_summary() {
        let key = [0x42u8; 32];
        let project_id = Uuid::nil();
        let req = delete_expense_request(&key, 7, project_id, "deleted: Pizza".into(), 3);
        assert_eq!(req.id, 7);
        assert_eq!(req.project_id, project_id);
        let history = req.history.expect("a summary is always attached");
        assert_eq!(history.actor_user_id, None, "the actor travels in the payload");
        let payload = crate::crypto::decrypt_json::<HistoryPayload>(&key, &history.payload).unwrap();
        assert_eq!(payload.summary, "deleted: Pizza");
        assert_eq!(payload.actor_user_id, Some(3));
        assert_eq!(
            (payload.action, payload.entity, payload.entity_id),
            (Some(HistoryAction::Deleted), Some(HistoryEntity::Expense), Some(7)),
            "audit 2026-10-19 L5: what the columns say, inside the payload"
        );
        let plain = crate::crypto::decrypt(&key, &history.payload).unwrap();
        assert!(history.eid.is_some());
        assert_eq!(shared::entity::read(&plain, shared::entity::Slot::History), Ok(history.eid));
    }
}
