//! Rewrites a project written before docs/plans/participant-links-encryption.md: payers, debtors,
//! authors and history actors move into the payloads, the status into the project payload. Only a
//! key holder can do it, so it runs on the devices that open the project. Goes with phase 3.

use api::history::history_controller::get_project_history;
use api::payments::payments_controller::sync_project_data;
use api::projects::projects_controller::convert_project as send_conversion;
use dioxus::fullstack::Json;
use dioxus::prelude::*;
use shared::{
    AccountProject, ConvertProject, ConvertedRow, ConvertedRule, Expense, ExpensePayload, ExpenseShare,
    HistoryEntry, HistoryPayload, Payment, ProjectDto, ProjectPayload, ProjectStatus, ProjectSync,
    RecurringExpense, MAX_CONVERTED_EXPENSES, NO_CACHED_VERSION,
};
use std::collections::HashSet;
use uuid::Uuid;

use crate::common::{
    is_project_stale_error, is_recurring_stale_error, key_of, update_project_fresh, LocalStorageProject,
    LocalStorageState,
};
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
    crate::common::project_token::scope(project_id);
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
    crate::common::project_token::scope(project_id);
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
    let mut by_expense: std::collections::HashMap<i32, Vec<&Payment>> = std::collections::HashMap::new();
    for p in payments {
        by_expense.entry(p.expense_id).or_default().push(p);
    }
    expenses
        .iter()
        .filter_map(|e| converted_expense(key, e, by_expense.get(&e.id).map_or(&[][..], |rows| rows)))
        .collect()
}

fn converted_expense(key: &[u8; 32], e: &Expense, rows: &[&Payment]) -> Option<ConvertedRow<i32>> {
    let mut payload: ExpensePayload = decrypt_json(key, &e.payload).ok()?;
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

/// What the list's batched sync says about a held project's rows. Worked out where the rows are at
/// hand, so only this verdict waits for the account's answer.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Rows {
    /// Something still in the format before docs/plans/participant-links-encryption.md.
    Old,
    /// Every row known, and none in the old format.
    Clean,
    /// Not known complete: no rows in the answer and no cache the version claim vouches for, or an
    /// answer from a server that leaves the rules or the project out.
    Unknown,
    /// Read-only, or a key that does not open it: nothing this device can convert yet.
    Unusable,
}

/// One verdict per project the sync answered for, with the key it was judged under - `None` when
/// this device held none yet. A project missing from the answer no longer exists and gets none.
/// A project, the key its rows were judged under, and the judgement.
pub type Verdict = (Uuid, Option<[u8; 32]>, Rows);

pub fn rows_verdicts(before: &LocalStorageState, syncs: &[ProjectSync]) -> Vec<Verdict> {
    before
        .projects
        .iter()
        .filter_map(|entry| {
            let sync = syncs.iter().find(|s| s.project.as_ref().is_some_and(|p| p.id == entry.project_id))?;
            let key = key_of(entry);
            let rows = key.map_or(Rows::Unknown, |k| rows_verdict(&k, entry, sync));
            Some((entry.project_id, key, rows))
        })
        .collect()
}

fn rows_verdict(key: &[u8; 32], entry: &LocalStorageProject, sync: &ProjectSync) -> Rows {
    let (Some(project), Some(rules)) = (&sync.project, &sync.recurring) else { return Rows::Unknown };
    if project.read_only || decrypt_json::<ProjectPayload>(key, &project.payload).is_err() {
        return Rows::Unusable;
    }
    // Cached rows only under the version they were claimed at: anything else may be a web cache
    // cut to its ceiling, and a row it dropped would stay in clear.
    let rows = match (&sync.expenses, &sync.payments) {
        (Some(expenses), Some(payments)) => Some((expenses.clone(), payments.clone())),
        _ if entry.cached_data_version == Some(sync.data_version) => {
            entry.cached_expenses.clone().zip(entry.cached_payments.clone())
        }
        _ => None,
    };
    match rows {
        Some((expenses, payments)) if needs_conversion(key, project, &expenses, &payments, rules) => Rows::Old,
        Some(_) => Rows::Clean,
        None => Rows::Unknown,
    }
}

/// What a signed-in device does about the projects it holds, with no request of its own.
#[derive(Debug, Default, PartialEq)]
pub struct ConversionPlan {
    /// Nothing left to convert, the server's flag included: marked converted on the spot.
    pub mark: Vec<Uuid>,
    /// Through [`convert`], which reads everything again before it writes.
    pub convert: Vec<(Uuid, [u8; 32])>,
}

/// Skipping a conversion takes both proofs: complete rows in the payload format, and the account's
/// word that no history row still holds an actor (`history_has_actors`, which only the server can
/// see for `actor_account_id`). Anything unknown converts as before. A project already marked is
/// converted again only on evidence - a write replayed by an older build after the mark.
pub fn conversion_plan(
    held: &LocalStorageState,
    verdicts: &[Verdict],
    account: &[AccountProject],
    attempted: &HashSet<Uuid>,
) -> ConversionPlan {
    let mut plan = ConversionPlan::default();
    for &(project_id, judged_with, rows) in verdicts {
        let Some(entry) = held.projects.iter().find(|p| p.project_id == project_id) else { continue };
        let Some(key) = key_of(entry) else { continue };
        if attempted.contains(&project_id) {
            continue;
        }
        // A key that arrived or changed since - reconciliation's escrow - makes the rows unjudged.
        let rows = if judged_with == Some(key) { rows } else { Rows::Unknown };
        let flag = account.iter().find(|a| a.project_id == project_id).and_then(|a| a.history_has_actors);
        let convert = match (entry.converted, rows, flag) {
            (_, Rows::Unusable, _) => false,
            (_, Rows::Old, _) | (_, Rows::Clean, Some(true)) => true,
            (false, Rows::Clean, Some(false)) => {
                plan.mark.push(project_id);
                false
            }
            (false, _, _) => true,
            (true, _, _) => false,
        };
        if convert {
            plan.convert.push((project_id, key));
        }
    }
    plan
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
            payload: encrypt_json(&key, &HistoryPayload { summary: "added".into(), actor_user_id: None, action: None, entity: None, entity_id: None }).unwrap(),
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

    fn ongoing(key: &[u8; 32], id: Uuid) -> ProjectDto {
        let mut project = make_project(key, id, "Trip", "EUR");
        let mut pp: ProjectPayload = decrypt_json(key, &project.payload).unwrap();
        pp.status = Some(ProjectStatus::Ongoing);
        project.payload = encrypt_json(key, &pp).unwrap();
        project
    }

    fn entry(key: &[u8; 32], id: Uuid) -> LocalStorageProject {
        LocalStorageProject {
            project_id: id,
            encryption_key: Some(crate::crypto::key_to_fragment(key)),
            ..Default::default()
        }
    }

    fn answer(project: ProjectDto, rows: Option<(Vec<Expense>, Vec<Payment>)>, version: i64) -> ProjectSync {
        let (expenses, payments) = rows.map_or((None, None), |(e, p)| (Some(e), Some(p)));
        ProjectSync {
            data_version: version,
            project: Some(project),
            users: Some(vec![]),
            expenses,
            payments,
            recurring: Some(vec![]),
            server_date: None,
            rotated: None,
        }
    }

    fn verdict(before: LocalStorageProject, sync: ProjectSync) -> Rows {
        let state = LocalStorageState { projects: vec![before], ..Default::default() };
        rows_verdicts(&state, &[sync])[0].2
    }

    #[test]
    fn the_rows_verdict_trusts_only_complete_rows() {
        let key = test_key();
        let id = Uuid::new_v4();
        let (e, rows) = v1_expense(&key);
        let clean = Expense { author_id: None, ..make_expense(&key, 2, "Café", 3.0, ExpenseType::Expense, "2025-01-02") };

        assert_eq!(verdict(entry(&key, id), answer(ongoing(&key, id), Some((vec![clean.clone()], vec![])), 4)), Rows::Clean);
        assert_eq!(verdict(entry(&key, id), answer(ongoing(&key, id), Some((vec![e.clone()], rows.clone())), 4)), Rows::Old);

        let claimed = LocalStorageProject {
            cached_expenses: Some(vec![e]),
            cached_payments: Some(rows),
            cached_data_version: Some(4),
            ..entry(&key, id)
        };
        assert_eq!(verdict(claimed.clone(), answer(ongoing(&key, id), None, 4)), Rows::Old, "unchanged: the claimed cache");
        let truncated = LocalStorageProject { cached_data_version: None, ..claimed };
        assert_eq!(verdict(truncated, answer(ongoing(&key, id), None, 4)), Rows::Unknown, "a cache no claim vouches for");

        let mut no_rules = answer(ongoing(&key, id), Some((vec![clean.clone()], vec![])), 4);
        no_rules.recurring = None;
        assert_eq!(verdict(entry(&key, id), no_rules), Rows::Unknown, "an older server's answer");

        let mut frozen = ongoing(&key, id);
        frozen.read_only = true;
        assert_eq!(verdict(entry(&key, id), answer(frozen, Some((vec![], vec![])), 4)), Rows::Unusable);
        assert_eq!(
            verdict(entry(&[9u8; 32], id), answer(ongoing(&key, id), Some((vec![], vec![])), 4)),
            Rows::Unusable,
            "a key that does not open the project"
        );

        let state = LocalStorageState { projects: vec![entry(&key, id)], ..Default::default() };
        assert!(rows_verdicts(&state, &[]).is_empty(), "no answer for a project the server no longer has");
    }

    fn flagged(id: Uuid, flag: Option<bool>) -> Vec<AccountProject> {
        vec![AccountProject {
            project_id: id,
            user_id: None,
            key: None,
            claim_verifier_seeded: true,
            payment_methods_shared: false,
            history_has_actors: flag,
            admin: false,
        }]
    }

    fn plan(converted: bool, rows: Rows, flag: Option<bool>) -> ConversionPlan {
        let key = test_key();
        let id = Uuid::nil();
        let held = LocalStorageState { projects: vec![LocalStorageProject { converted, ..entry(&key, id) }], ..Default::default() };
        conversion_plan(&held, &[(id, Some(key), rows)], &flagged(id, flag), &HashSet::new())
    }

    #[test]
    fn a_conversion_is_skipped_only_on_both_proofs() {
        let key = test_key();
        let to_convert = ConversionPlan { mark: vec![], convert: vec![(Uuid::nil(), key)] };
        let nothing = ConversionPlan::default();

        assert_eq!(plan(false, Rows::Clean, Some(false)), ConversionPlan { mark: vec![Uuid::nil()], convert: vec![] });
        assert_eq!(plan(false, Rows::Clean, Some(true)), to_convert, "history actors left");
        assert_eq!(plan(false, Rows::Clean, None), to_convert, "an older server says nothing");
        assert_eq!(plan(false, Rows::Old, Some(false)), to_convert);
        assert_eq!(plan(false, Rows::Unknown, Some(false)), to_convert);
        assert_eq!(plan(false, Rows::Unusable, None), nothing, "never marked, so a later key still converts it");

        assert_eq!(plan(true, Rows::Old, Some(false)), to_convert, "an older build wrote after the mark");
        assert_eq!(plan(true, Rows::Clean, Some(true)), to_convert);
        assert_eq!(plan(true, Rows::Clean, None), nothing);
        assert_eq!(plan(true, Rows::Unknown, Some(true)), nothing);
    }

    #[test]
    fn a_plan_skips_a_project_left_converts_one_rekeyed_and_tries_once_per_session() {
        let key = test_key();
        let id = Uuid::nil();
        let verdicts = [(id, Some(key), Rows::Clean)];
        let flag = flagged(id, Some(false));
        assert_eq!(conversion_plan(&LocalStorageState::default(), &verdicts, &flag, &HashSet::new()), ConversionPlan::default());

        let escrowed = [3u8; 32];
        let rekeyed = LocalStorageState { projects: vec![entry(&escrowed, id)], ..Default::default() };
        let full_path = ConversionPlan { mark: vec![], convert: vec![(id, escrowed)] };
        assert_eq!(conversion_plan(&rekeyed, &verdicts, &flag, &HashSet::new()), full_path, "judged under another key");
        assert_eq!(conversion_plan(&rekeyed, &[(id, None, Rows::Unknown)], &flag, &HashSet::new()), full_path, "a key that arrived since");

        let held = LocalStorageState { projects: vec![entry(&key, id)], ..Default::default() };
        assert_eq!(conversion_plan(&held, &verdicts, &[], &HashSet::from([id])), ConversionPlan::default());
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
