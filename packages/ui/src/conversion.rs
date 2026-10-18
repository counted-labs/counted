//! Rewrites a project written before docs/plans/participant-links-encryption.md: payers, debtors,
//! authors and history actors move into the payloads, the status into the project payload. Only a
//! key holder can do it, so it runs on the devices that open the project. Goes with phase 3.

use api::history::history_controller::get_project_history;
use api::payments::payments_controller::sync_project_data;
use api::projects::projects_controller::convert_project as send_conversion;
use dioxus::fullstack::Json;
use dioxus::prelude::*;
use shared::{
    ConvertProject, ConvertedRow, ConvertedRule, Expense, ExpensePayload, ExpenseShare,
    HistoryEntry, HistoryPayload, Payment, ProjectDto, ProjectPayload, ProjectStatus,
    RecurringExpense, MAX_CONVERTED_EXPENSES, NO_CACHED_VERSION,
};
use uuid::Uuid;

use crate::common::{is_project_stale_error, is_recurring_stale_error, update_project_fresh};
use crate::crypto::{claim_token, decrypt_json, encrypt_padded_json};
use crate::decrypted::{decrypt_payment, project_status, status_edit};
use crate::recurring::model::RecurringPayload;

const ATTEMPTS: usize = 3;

/// Whether what the page already loaded still has anything in the old format. Free: every expense
/// in it has payment rows and an author column, a rule its participant rows or author column.
pub fn needs_conversion(
    key: &[u8; 32],
    project: &ProjectDto,
    expenses: &[Expense],
    payments: &[Payment],
    rules: &[RecurringExpense],
) -> bool {
    !project.read_only
        && (status_needs_conversion(key, project)
            || !payments.is_empty()
            || expenses.iter().any(|e| e.author_id.is_some())
            || rules.iter().any(|r| r.author_id.is_some() || r.listed_participants))
}

fn status_needs_conversion(key: &[u8; 32], project: &ProjectDto) -> bool {
    project.status != ProjectStatus::Ongoing
        || decrypt_json::<ProjectPayload>(key, &project.payload).is_ok_and(|p| p.status.is_none())
}

/// Converts everything the project holds in the old format. Reads it all first: the page's cache
/// can be partial, and a row it misses would stay in clear.
pub async fn convert(key: [u8; 32], project_id: Uuid) -> Result<(), ServerFnError> {
    let mut stale = None;
    for _ in 0..ATTEMPTS {
        match convert_once(&key, project_id).await {
            Err(e) if is_project_stale_error(&e) || is_recurring_stale_error(&e) => stale = Some(e),
            result => return result,
        }
    }
    Err(stale.unwrap_or_else(|| ServerFnError::new(shared::errors::PROJECT_STALE)))
}

async fn convert_once(key: &[u8; 32], project_id: Uuid) -> Result<(), ServerFnError> {
    let sync = sync_project_data(project_id, NO_CACHED_VERSION).await?;
    let Some(project) = sync.project else { return Ok(()) };
    // A key that does not open the project would rewrite nothing and still clear the history actors.
    if project.read_only || decrypt_json::<ProjectPayload>(key, &project.payload).is_err() {
        return Ok(());
    }

    if status_needs_conversion(key, &project) {
        // The status as every reader sees it now, payload first: nothing changes, nothing rings.
        let edit = |stored: &ProjectDto| {
            let mut edit = status_edit(key, stored, &project_status(Some(key), stored))?;
            edit.notify_closed = false;
            Ok(edit)
        };
        update_project_fresh(project_id, edit).await?;
    }

    let expenses =
        converted_expenses(key, &sync.expenses.unwrap_or_default(), &sync.payments.unwrap_or_default());
    let rules = converted_rules(key, &sync.recurring.unwrap_or_default());
    let entries = get_project_history(project_id).await?;
    let history = converted_history(key, &entries);
    let history_through = entries.iter().map(|h| h.id).max();
    if expenses.is_empty() && rules.is_empty() && history_through.is_none() {
        return Ok(());
    }

    let token = Some(claim_token(key).to_vec());
    let mut version = sync.data_version;
    if !rules.is_empty() || history_through.is_some() {
        version = send_conversion(Json(ConvertProject {
            project_id,
            expected_data_version: version,
            expenses: Vec::new(),
            recurring: rules,
            history,
            history_through,
            claim_token: token.clone(),
        }))
        .await?;
    }
    for batch in expense_batches(expenses) {
        version = send_conversion(Json(ConvertProject {
            project_id,
            expected_data_version: version,
            expenses: batch,
            recurring: Vec::new(),
            history: Vec::new(),
            history_through: None,
            claim_token: token.clone(),
        }))
        .await?;
    }
    Ok(())
}

/// Leaves room under nginx's `client_max_body_size 1m` for the JSON around the payloads.
const BATCH_BODY_BUDGET: usize = 768 * 1024;

/// Batches bounded by encoded size as well as count: 500 padded expenses can exceed the 1 MB cap.
pub fn expense_batches(rows: Vec<ConvertedRow<i32>>) -> Vec<Vec<ConvertedRow<i32>>> {
    let mut batches = Vec::new();
    let mut batch: Vec<ConvertedRow<i32>> = Vec::new();
    let mut bytes = 0;
    for row in rows {
        let size = row.payload.ct.len() + row.payload.iv.len() + 64;
        if !batch.is_empty() && (bytes + size > BATCH_BODY_BUDGET || batch.len() == MAX_CONVERTED_EXPENSES) {
            batches.push(std::mem::take(&mut batch));
            bytes = 0;
        }
        bytes += size;
        batch.push(row);
    }
    if !batch.is_empty() {
        batches.push(batch);
    }
    batches
}

/// Each expense in the old format, rewritten with its shares and author inside. One whose rows
/// will not all open is left as it is rather than rewritten with a part of its ledger.
pub fn converted_expenses(key: &[u8; 32], expenses: &[Expense], payments: &[Payment]) -> Vec<ConvertedRow<i32>> {
    expenses.iter().filter_map(|e| converted_expense(key, e, payments)).collect()
}

fn converted_expense(key: &[u8; 32], e: &Expense, payments: &[Payment]) -> Option<ConvertedRow<i32>> {
    let mut payload: ExpensePayload = decrypt_json(key, &e.payload).ok()?;
    let rows: Vec<&Payment> = payments.iter().filter(|p| p.expense_id == e.id).collect();
    if payload.shares.is_some() && e.author_id.is_none() && rows.is_empty() {
        return None;
    }
    let shares = match payload.shares.take() {
        Some(shares) => shares,
        None => rows
            .iter()
            .map(|p| {
                decrypt_payment(key, p)
                    .ok()
                    .map(|d| ExpenseShare { user_id: d.user_id, amount: d.amount, is_debt: d.is_debt })
            })
            .collect::<Option<Vec<_>>>()?,
    };
    payload.author_id = payload.author_id.or(e.author_id);
    payload.shares = Some(shares.clone());
    let pair = encrypt_padded_json(key, &payload).ok()?;
    // The rows are deleted once this lands: what goes up must read back as exactly what they held.
    let back: ExpensePayload = decrypt_json(key, &pair).ok()?;
    (back.shares == Some(shares) && back.author_id == payload.author_id)
        .then_some(ConvertedRow { id: e.id, payload: pair })
}

pub fn converted_rules(key: &[u8; 32], rules: &[RecurringExpense]) -> Vec<ConvertedRule> {
    rules
        .iter()
        .filter(|r| r.author_id.is_some() || r.listed_participants)
        .filter_map(|r| {
            let mut payload: RecurringPayload = decrypt_json(key, &r.payload).ok()?;
            payload.author_id = payload.author_id.or(r.author_id);
            Some(ConvertedRule {
                id: r.id,
                expected_version: r.version,
                payload: encrypt_padded_json(key, &payload).ok()?,
            })
        })
        .collect()
}

pub fn converted_history(key: &[u8; 32], entries: &[HistoryEntry]) -> Vec<ConvertedRow<i64>> {
    entries
        .iter()
        .filter(|h| h.actor_user_id.is_some())
        .filter_map(|h| {
            let mut payload: HistoryPayload = decrypt_json(key, &h.payload).ok()?;
            payload.actor_user_id = payload.actor_user_id.or(h.actor_user_id);
            Some(ConvertedRow { id: h.id, payload: encrypt_padded_json(key, &payload).ok()? })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::test_fixtures::{make_expense, make_payment, make_project, test_key};
    use crate::crypto::encrypt_json;
    use crate::decrypted::decrypt_ledger;
    use shared::{EncryptedPair, ExpenseType, HistoryAction, HistoryEntity};

    fn v1_expense(key: &[u8; 32]) -> (Expense, Vec<Payment>) {
        let mut e = make_expense(key, 1, "Pizza", 30.0, ExpenseType::Expense, "2025-01-01");
        e.author_id = Some(7);
        let rows = vec![
            make_payment(key, 10, 1, 7, false, 30.0),
            make_payment(key, 11, 1, 7, true, 10.0),
            make_payment(key, 12, 1, 8, true, 20.0),
        ];
        (e, rows)
    }

    fn balances(ledger: &[crate::decrypted::DecryptedPayment]) -> Vec<(i32, bool, i64)> {
        let mut v: Vec<_> =
            ledger.iter().map(|p| (p.user_id, p.is_debt, shared::to_cents(p.amount))).collect();
        v.sort();
        v
    }

    /// The conversion's whole contract: the same ledger, with nothing left in clear.
    #[test]
    fn a_converted_expense_reads_back_as_the_rows_it_replaces() {
        let key = test_key();
        let (e, rows) = v1_expense(&key);
        let before = decrypt_ledger(&key, std::slice::from_ref(&e), &rows);

        let converted = converted_expenses(&key, std::slice::from_ref(&e), &rows);
        assert_eq!(converted.len(), 1);
        let after_row = Expense { author_id: None, payload: converted[0].payload.clone(), ..e.clone() };
        let after = decrypt_ledger(&key, std::slice::from_ref(&after_row), &[]);

        assert_eq!(balances(&before.1), balances(&after.1));
        assert_eq!(after.0[0].author_id, Some(7), "the author moves into the payload");
        assert_eq!(after.0[0].name, "Pizza");
    }

    #[test]
    fn an_expense_already_in_the_payload_format_is_left_alone() {
        let key = test_key();
        let (e, rows) = v1_expense(&key);
        let converted = converted_expenses(&key, std::slice::from_ref(&e), &rows);
        let v2 = Expense { author_id: None, payload: converted[0].payload.clone(), ..e };
        assert!(converted_expenses(&key, &[v2], &[]).is_empty());
    }

    /// Converting with a row missing would delete the rest and lose that share for good.
    #[test]
    fn an_expense_whose_rows_do_not_all_open_is_not_converted() {
        let key = test_key();
        let (e, mut rows) = v1_expense(&key);
        rows[2].payload = EncryptedPair { ct: "not base64!".into(), iv: rows[2].payload.iv.clone() };
        assert!(converted_expenses(&key, &[e], &rows).is_empty());
    }

    #[test]
    fn a_history_row_takes_its_actor_inside() {
        let key = test_key();
        let entry = HistoryEntry {
            id: 5,
            project_id: Uuid::nil(),
            actor_user_id: Some(3),
            actor_account_id: None,
            action: HistoryAction::Created,
            entity: HistoryEntity::Expense,
            entity_id: Some(1),
            payload: encrypt_json(&key, &HistoryPayload { summary: "added".into(), actor_user_id: None }).unwrap(),
            created_at: chrono::NaiveDateTime::default(),
        };
        let converted = converted_history(&key, std::slice::from_ref(&entry));
        let back: HistoryPayload = decrypt_json(&key, &converted[0].payload).unwrap();
        assert_eq!((back.summary.as_str(), back.actor_user_id), ("added", Some(3)));

        let done = HistoryEntry { actor_user_id: None, ..entry };
        assert!(converted_history(&key, &[done]).is_empty());
    }

    #[test]
    fn what_the_page_holds_says_whether_to_convert() {
        let key = test_key();
        let (e, rows) = v1_expense(&key);
        let mut project = make_project(&key, Uuid::nil(), "Trip", "EUR");
        let mut pp: ProjectPayload = decrypt_json(&key, &project.payload).unwrap();
        pp.status = Some(ProjectStatus::Ongoing);
        project.payload = encrypt_json(&key, &pp).unwrap();

        assert!(!needs_conversion(&key, &project, &[], &[], &[]));
        assert!(needs_conversion(&key, &project, std::slice::from_ref(&e), &rows, &[]));
        assert!(needs_conversion(&key, &project, &[e], &[], &[]), "an author column alone");

        let mut closed = project.clone();
        closed.status = ProjectStatus::Closed;
        assert!(needs_conversion(&key, &closed, &[], &[], &[]), "a status column");

        let mut frozen = closed;
        frozen.read_only = true;
        assert!(!needs_conversion(&key, &frozen, &[], &[], &[]), "a read-only project cannot be written");
    }

    #[test]
    fn a_batch_stays_under_the_body_cap_and_the_count_cap() {
        let row = |id: i32, ct_len: usize| ConvertedRow {
            id,
            payload: EncryptedPair { ct: "A".repeat(ct_len), iv: "B".repeat(32) },
        };
        let large: Vec<_> = (0..500).map(|i| row(i, 2800)).collect();
        let batches = expense_batches(large);
        assert!(batches.len() > 1, "500 padded 2 KiB expenses do not fit one request");
        for b in &batches {
            let bytes: usize = b.iter().map(|r| r.payload.ct.len() + r.payload.iv.len() + 64).sum();
            assert!(bytes <= BATCH_BODY_BUDGET);
        }
        assert_eq!(batches.iter().map(Vec::len).sum::<usize>(), 500);

        let small: Vec<_> = (0..1200).map(|i| row(i, 10)).collect();
        let counts: Vec<usize> = expense_batches(small).iter().map(Vec::len).collect();
        assert_eq!(counts, vec![MAX_CONVERTED_EXPENSES, MAX_CONVERTED_EXPENSES, 200]);
        assert!(expense_batches(Vec::new()).is_empty());
    }
}
