//! [`Rows`] over the server's routes (docs/plans/format-v2-f2a.md step 0a): `write_fresh` reads,
//! writes and deletes through it, every request under the token of `key`, the key its sealer
//! seals under.

use api::expenses::expenses_controller::{delete_expense, edit_expense, get_expense_by_id};
use api::projects::projects_controller::{get_project, update_project_by_id};
use dioxus::fullstack::Json;
use dioxus::prelude::ServerFnError;
use shared::merge::Group;
use shared::{errors, DeleteExpenseRequest, EditableExpense, EditableProject, ProjectStatus};
use uuid::Uuid;

use super::project_token::scope_key;
use super::write_fresh::{Address, Answer, Deletion, Outgoing, Read, Rows, Served};

pub struct ServerRows {
    pub key: [u8; 32],
}

impl Rows for ServerRows {
    async fn read(&mut self, at: Address) -> Read {
        scope_key(Some(&self.key));
        let served = match at {
            Address::Expense { project_id, row } => {
                get_expense_by_id(project_id, row).await.map(|e| Served { payload: e.payload, rev: e.rev, eid: e.eid })
            }
            Address::Project(id) => get_project(id).await.map(|p| Served { payload: p.payload, rev: p.rev, eid: None }),
        };
        read_answer(at, served)
    }

    async fn write(&mut self, at: Address, out: Outgoing) -> Answer {
        scope_key(Some(&self.key));
        match at {
            Address::Expense { project_id, row } => {
                let sent = edit_expense(Json(expense_edit(project_id, row, out))).await;
                write_answer(at, false, sent.map(|landed| landed.expense.rev))
            }
            Address::Project(id) => {
                let edit = project_edit(id, out);
                let named_iv = edit.expected_iv.is_some();
                write_answer(at, named_iv, update_project_by_id(Json(edit)).await.map(|p| p.rev))
            }
        }
    }

    async fn delete(&mut self, at: Address, out: Deletion) -> Answer {
        match at {
            Address::Expense { project_id, row } => {
                scope_key(Some(&self.key));
                write_answer(at, false, delete_expense(Json(expense_delete(project_id, row, out))).await.map(|_| None))
            }
            Address::Project(_) => Answer::Refused { status: 0, code: String::new() },
        }
    }
}

/// Shares in the payload, so the sides are empty and the server keeps no author column.
fn expense_edit(project_id: Uuid, row: i32, out: Outgoing) -> EditableExpense {
    EditableExpense {
        id: row,
        project_id,
        author_id: None,
        payload: out.payload,
        payers: Vec::new(),
        debtors: Vec::new(),
        history: out.history,
        expected_rev: out.expected_rev,
        eid: out.eid,
    }
}

fn expense_delete(project_id: Uuid, row: i32, out: Deletion) -> DeleteExpenseRequest {
    DeleteExpenseRequest { id: row, project_id, history: out.history, expected_rev: out.expected_rev, eid: out.eid }
}

/// The status column is never written: the payload's status is the one read since the floor.
fn project_edit(id: Uuid, out: Outgoing) -> EditableProject {
    let closes = out.changed.contains(&Group::Known("status"))
        && out.merged.get("status").and_then(|s| serde_json::from_value::<ProjectStatus>(s.clone()).ok()) == Some(ProjectStatus::Closed);
    EditableProject {
        id,
        payload: Some(out.payload),
        status: None,
        history: out.history,
        expected_iv: Some(out.read_iv),
        notify_closed: closes,
        expected_rev: out.expected_rev,
    }
}

/// `None` for a request that got no answer from the server.
fn refusal(e: ServerFnError) -> Option<(u16, String)> {
    match e {
        ServerFnError::ServerError { code, message, .. } => Some((code, message)),
        _ => None,
    }
}

/// An expense is gone with its project too.
fn gone(at: Address, code: &str) -> bool {
    match at {
        Address::Expense { .. } => code == errors::EXPENSE_NOT_FOUND || code == errors::PROJECT_NOT_FOUND,
        Address::Project(_) => code == errors::PROJECT_NOT_FOUND,
    }
}

fn read_answer(at: Address, result: Result<Served, ServerFnError>) -> Read {
    match result.map_err(refusal) {
        Ok(served) => Read::Row(served),
        Err(None) => Read::Unreachable,
        Err(Some((404, code))) if gone(at, &code) => Read::Gone,
        Err(Some((409, code))) if code == errors::PROJECT_ROTATING => Read::Rotating,
        Err(Some((status, code))) => Read::Refused { status, code },
    }
}

fn write_answer(at: Address, named_iv: bool, result: Result<Option<i64>, ServerFnError>) -> Answer {
    match result.map_err(refusal) {
        Ok(rev) => Answer::Landed { rev },
        Err(None) => Answer::Unreachable,
        Err(Some((404, code))) if gone(at, &code) => Answer::Gone,
        Err(Some((409, code))) if code == errors::STALE_REV || (named_iv && code == errors::PROJECT_STALE) => Answer::Stale,
        Err(Some((409, code))) if code == errors::PROJECT_ROTATING => Answer::Rotating,
        Err(Some((status, code))) => Answer::Refused { status, code },
    }
}

#[cfg(test)]
#[path = "write_fresh_rows_tests.rs"]
mod tests;
