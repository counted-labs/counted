//! Write paths racing each other against a real Postgres. Ignored by default; run against the e2e
//! database (`make e2e-backend`) or any migrated one with
//!
//!   COUNTED_TEST_DATABASE_URL=… cargo test -p api --features server --test concurrent_writes -- --ignored
//!
//! Needs committed transactions, so each case cleans its fixture project up itself.
#![cfg(feature = "server")]

use api::server::projects::projects_repository::update_project_by_id;
use api::server::recurring::{
    add as add_recurring, delete as delete_recurring, edit as edit_recurring, materialize,
};
use api::server::users::remove_participant;
use dioxus::prelude::ServerFnError;
use shared::{
    errors, CreatableExpense, CreatableRecurringExpense, DeleteRecurringExpenseRequest, EditableProject,
    EditableRecurringExpense, EncryptedPair, EncryptedUserAmount, HistoryAction, HistoryContext,
    MaterializeRecurringRequest, ProjectStatus,
};
use sqlx::{Connection, PgConnection};
use uuid::Uuid;

const E2E_DATABASE_URL: &str = "postgres://hcount_user:e2e@127.0.0.1:5432/hcount";

fn run<F: std::future::Future>(f: F) -> F::Output {
    tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap().block_on(f)
}

async fn connect() -> PgConnection {
    let url = std::env::var("COUNTED_TEST_DATABASE_URL").unwrap_or_else(|_| E2E_DATABASE_URL.into());
    PgConnection::connect(&url).await.expect("e2e database — run `make e2e-backend` first")
}

async fn participant(conn: &mut PgConnection, project_id: Uuid) -> i32 {
    let (id,): (i32,) =
        sqlx::query_as("INSERT INTO users (payload_ct, payload_iv) VALUES (decode('00','hex'), decode('00','hex')) RETURNING id")
            .fetch_one(&mut *conn)
            .await
            .unwrap();
    sqlx::query("INSERT INTO user_projects (project_id, user_id) VALUES ($1, $2)")
        .bind(project_id)
        .bind(id)
        .execute(&mut *conn)
        .await
        .unwrap();
    id
}

async fn someone_waits_on_a_lock(conn: &mut PgConnection) {
    loop {
        let (n,): (i64,) = sqlx::query_as(
            "SELECT count(*) FROM pg_stat_activity WHERE wait_event_type = 'Lock' AND datname = current_database()",
        )
        .fetch_one(&mut *conn)
        .await
        .unwrap();
        if n > 0 {
            return;
        }
    }
}

/// An expense paying the participant commits while the removal is in flight. The removal must
/// either see that payment and refuse, or make the expense fail — never cascade the payment away
/// and leave the expense unbalanced.
#[test]
#[ignore]
fn removal_never_cascades_a_payment_that_committed_meanwhile() {
    run(async {
        let mut setup = connect().await;
        let (project_id,): (Uuid,) = sqlx::query_as(
            "INSERT INTO projects (payload_ct, payload_iv) VALUES (decode('00','hex'), decode('00','hex')) RETURNING id",
        )
        .fetch_one(&mut setup)
        .await
        .unwrap();
        let removed = participant(&mut setup, project_id).await;
        let payer = participant(&mut setup, project_id).await;

        let mut writer = connect().await;
        sqlx::query("BEGIN").execute(&mut writer).await.unwrap();
        let (expense_id,): (i32,) = sqlx::query_as(
            "INSERT INTO expenses (project_id, author_id, payload_ct, payload_iv) \
             VALUES ($1, $2, decode('00','hex'), decode('00','hex')) RETURNING id",
        )
        .bind(project_id)
        .bind(payer)
        .fetch_one(&mut writer)
        .await
        .unwrap();
        sqlx::query(
            "INSERT INTO payments (expense_id, user_id, payload_ct, payload_iv) \
             VALUES ($1, $2, decode('00','hex'), decode('00','hex')), ($1, $3, decode('00','hex'), decode('00','hex'))",
        )
        .bind(expense_id)
        .bind(payer)
        .bind(removed)
        .execute(&mut writer)
        .await
        .unwrap();

        let local = tokio::task::LocalSet::new();
        let result = local
            .run_until(async {
                let removal = tokio::task::spawn_local(async move {
                let mut remover = connect().await;
                sqlx::query("BEGIN").execute(&mut remover).await.unwrap();
                let result = remove_participant(&mut remover, project_id, removed).await;
                let end = if result.is_ok() { "COMMIT" } else { "ROLLBACK" };
                sqlx::query(end).execute(&mut remover).await.unwrap();
                result
                });
                someone_waits_on_a_lock(&mut setup).await;
                sqlx::query("COMMIT").execute(&mut writer).await.unwrap();
                removal.await.unwrap()
            })
            .await;

        let (payments,): (i64,) =
            sqlx::query_as("SELECT count(*) FROM payments WHERE expense_id = $1")
                .bind(expense_id)
                .fetch_one(&mut setup)
                .await
                .unwrap();

        sqlx::query("DELETE FROM projects WHERE id = $1").bind(project_id).execute(&mut setup).await.unwrap();
        sqlx::query("DELETE FROM users WHERE id = ANY($1)")
            .bind(vec![removed, payer])
            .execute(&mut setup)
            .await
            .unwrap();

        assert!(result.is_err(), "the participant was removed while an expense paid them");
        assert_eq!(payments, 2, "a committed payment was cascaded away");
    });
}

/// A rename and a status change land at the same time. Both must survive: the update used to read
/// the row, patch one field and write both back, so the later writer restored the stale other one.
#[test]
#[ignore]
fn concurrent_partial_project_edits_both_survive() {
    run(async {
        let mut setup = connect().await;
        let (project_id,): (Uuid,) = sqlx::query_as(
            "INSERT INTO projects (payload_ct, payload_iv) VALUES (decode('00','hex'), decode('00','hex')) RETURNING id",
        )
        .fetch_one(&mut setup)
        .await
        .unwrap();

        let mut closer = connect().await;
        sqlx::query("BEGIN").execute(&mut closer).await.unwrap();
        update_project_by_id(
            &mut closer,
            EditableProject { id: project_id, payload: None, status: Some(ProjectStatus::Closed), history: None },
        )
        .await
        .unwrap();

        let renamed = EncryptedPair { ct: "cmVuYW1lZA==".into(), iv: "A".repeat(32) };
        let local = tokio::task::LocalSet::new();
        local
            .run_until(async {
                let rename = tokio::task::spawn_local(async move {
                    let mut renamer = connect().await;
                    sqlx::query("BEGIN").execute(&mut renamer).await.unwrap();
                    update_project_by_id(
                        &mut renamer,
                        EditableProject { id: project_id, payload: Some(renamed), status: None, history: None },
                    )
                    .await
                    .unwrap();
                    sqlx::query("COMMIT").execute(&mut renamer).await.unwrap();
                });
                someone_waits_on_a_lock(&mut setup).await;
                sqlx::query("COMMIT").execute(&mut closer).await.unwrap();
                rename.await.unwrap();
            })
            .await;

        let (status, payload_ct): (ProjectStatus, Vec<u8>) =
            sqlx::query_as("SELECT status, payload_ct FROM projects WHERE id = $1")
                .bind(project_id)
                .fetch_one(&mut setup)
                .await
                .unwrap();
        sqlx::query("DELETE FROM projects WHERE id = $1").bind(project_id).execute(&mut setup).await.unwrap();

        assert_eq!(payload_ct, b"renamed");
        assert_eq!(status, ProjectStatus::Closed, "the rename wrote the stale status back");
    });
}

async fn project(conn: &mut PgConnection) -> Uuid {
    let (id,): (Uuid,) = sqlx::query_as(
        "INSERT INTO projects (payload_ct, payload_iv) VALUES (decode('00','hex'), decode('00','hex')) RETURNING id",
    )
    .fetch_one(&mut *conn)
    .await
    .unwrap();
    id
}

async fn recurring_rule(conn: &mut PgConnection, project_id: Uuid, participants: &[i32]) -> Uuid {
    let (id,): (Uuid,) = sqlx::query_as(
        "INSERT INTO recurring_expenses (project_id, author_id, payload_ct, payload_iv) \
         VALUES ($1, $2, decode('00','hex'), decode('00','hex')) RETURNING id",
    )
    .bind(project_id)
    .bind(participants[0])
    .fetch_one(&mut *conn)
    .await
    .unwrap();
    for user_id in participants {
        sqlx::query("INSERT INTO recurring_expense_participants (recurring_id, user_id) VALUES ($1, $2)")
            .bind(id)
            .bind(user_id)
            .execute(&mut *conn)
            .await
            .unwrap();
    }
    id
}

fn today() -> chrono::NaiveDate {
    chrono::Utc::now().date_naive()
}

fn materialization(
    rule: Uuid,
    project_id: Uuid,
    payer: i32,
    debtor: i32,
    op_ids: &[Uuid],
) -> MaterializeRecurringRequest {
    let ua = |user_id| EncryptedUserAmount { user_id, payload: EncryptedPair::default() };
    MaterializeRecurringRequest {
        id: rule,
        project_id,
        expected_version: 0,
        payload: EncryptedPair::default(),
        due_through: today(),
        occurrences: op_ids
            .iter()
            .map(|op_id| CreatableExpense {
                project_id,
                author_id: payer,
                payload: EncryptedPair::default(),
                payers: vec![ua(payer)],
                debtors: vec![ua(debtor)],
                history: None,
                client_op_id: Some(*op_id),
            })
            .collect(),
    }
}

fn message(e: &ServerFnError) -> String {
    match e {
        ServerFnError::ServerError { message, .. } => message.clone(),
        other => panic!("expected a server error, got {other:?}"),
    }
}

async fn expense_count(conn: &mut PgConnection, project_id: Uuid) -> i64 {
    let (n,): (i64,) = sqlx::query_as("SELECT count(*) FROM expenses WHERE project_id = $1")
        .bind(project_id)
        .fetch_one(&mut *conn)
        .await
        .unwrap();
    n
}

async fn rule_version(conn: &mut PgConnection, rule: Uuid) -> i64 {
    let (v,): (i64,) = sqlx::query_as("SELECT version FROM recurring_expenses WHERE id = $1")
        .bind(rule)
        .fetch_one(&mut *conn)
        .await
        .unwrap();
    v
}

async fn drop_fixture(conn: &mut PgConnection, project_id: Uuid, users: Vec<i32>) {
    sqlx::query("DELETE FROM projects WHERE id = $1").bind(project_id).execute(&mut *conn).await.unwrap();
    sqlx::query("DELETE FROM users WHERE id = ANY($1)").bind(users).execute(&mut *conn).await.unwrap();
}

/// Two members open the project after the due date and both materialize from the same version.
/// The second must wait on the first's row lock, then match nothing: one set of occurrences, a
/// cursor advanced once, and a 409 that tells the loser to resync.
#[test]
#[ignore]
fn concurrent_materializations_write_each_occurrence_once() {
    run(async {
        let mut setup = connect().await;
        let project_id = project(&mut setup).await;
        let payer = participant(&mut setup, project_id).await;
        let debtor = participant(&mut setup, project_id).await;
        let rule = recurring_rule(&mut setup, project_id, &[payer, debtor]).await;
        let op_ids = [Uuid::new_v4(), Uuid::new_v4()];

        let mut first = connect().await;
        sqlx::query("BEGIN").execute(&mut first).await.unwrap();
        materialize(&mut first, materialization(rule, project_id, payer, debtor, &op_ids), today(), None)
            .await
            .unwrap();

        let local = tokio::task::LocalSet::new();
        let second = local
            .run_until(async {
                let second = tokio::task::spawn_local(async move {
                    let mut conn = connect().await;
                    sqlx::query("BEGIN").execute(&mut conn).await.unwrap();
                    let result = materialize(
                        &mut conn,
                        materialization(rule, project_id, payer, debtor, &op_ids),
                        today(),
                        None,
                    )
                    .await;
                    let end = if result.is_ok() { "COMMIT" } else { "ROLLBACK" };
                    sqlx::query(end).execute(&mut conn).await.unwrap();
                    result
                });
                someone_waits_on_a_lock(&mut setup).await;
                sqlx::query("COMMIT").execute(&mut first).await.unwrap();
                second.await.unwrap()
            })
            .await;

        let expenses = expense_count(&mut setup, project_id).await;
        let version = rule_version(&mut setup, rule).await;
        drop_fixture(&mut setup, project_id, vec![payer, debtor]).await;

        assert_eq!(message(&second.unwrap_err()), errors::RECURRING_STALE);
        assert_eq!(expenses, 2, "an occurrence was written twice");
        assert_eq!(version, 1, "the cursor advanced more than once");
    });
}

/// An occurrence that already exists aborts the whole materialization: the cursor must not move
/// past occurrences that were not written.
#[test]
#[ignore]
fn a_duplicate_occurrence_rolls_back_the_claim() {
    run(async {
        let mut setup = connect().await;
        let project_id = project(&mut setup).await;
        let payer = participant(&mut setup, project_id).await;
        let debtor = participant(&mut setup, project_id).await;
        let rule = recurring_rule(&mut setup, project_id, &[payer, debtor]).await;
        let existing = Uuid::new_v4();
        sqlx::query(
            "INSERT INTO expenses (project_id, author_id, payload_ct, payload_iv, client_op_id) \
             VALUES ($1, $2, decode('00','hex'), decode('00','hex'), $3)",
        )
        .bind(project_id)
        .bind(payer)
        .bind(existing)
        .execute(&mut setup)
        .await
        .unwrap();

        let mut conn = connect().await;
        sqlx::query("BEGIN").execute(&mut conn).await.unwrap();
        let result = materialize(
            &mut conn,
            materialization(rule, project_id, payer, debtor, &[Uuid::new_v4(), existing]),
            today(),
            None,
        )
        .await;
        sqlx::query("ROLLBACK").execute(&mut conn).await.unwrap();

        let expenses = expense_count(&mut setup, project_id).await;
        let version = rule_version(&mut setup, rule).await;
        drop_fixture(&mut setup, project_id, vec![payer, debtor]).await;

        assert!(result.is_err(), "a duplicate client_op_id was accepted");
        assert_eq!(expenses, 1, "the occurrence before the duplicate survived the rollback");
        assert_eq!(version, 0, "the cursor advanced past an unwritten occurrence");
    });
}

/// A payer the rule does not list is refused, even when they belong to the project.
#[test]
#[ignore]
fn an_occurrence_outside_the_rule_participants_is_refused() {
    run(async {
        let mut setup = connect().await;
        let project_id = project(&mut setup).await;
        let payer = participant(&mut setup, project_id).await;
        let debtor = participant(&mut setup, project_id).await;
        let outsider = participant(&mut setup, project_id).await;
        let rule = recurring_rule(&mut setup, project_id, &[payer, debtor]).await;

        let mut conn = connect().await;
        sqlx::query("BEGIN").execute(&mut conn).await.unwrap();
        let result = materialize(
            &mut conn,
            materialization(rule, project_id, payer, outsider, &[Uuid::new_v4()]),
            today(),
            None,
        )
        .await;
        sqlx::query("ROLLBACK").execute(&mut conn).await.unwrap();

        let version = rule_version(&mut setup, rule).await;
        drop_fixture(&mut setup, project_id, vec![payer, debtor, outsider]).await;

        assert_eq!(message(&result.unwrap_err()), errors::PARTICIPANT_NOT_IN_RECURRING);
        assert_eq!(version, 0);
    });
}

/// A participant a rule pays or charges stays in the project until the rule goes, so a rule can
/// never reference someone who no longer exists.
#[test]
#[ignore]
fn a_rule_participant_cannot_be_removed() {
    run(async {
        let mut setup = connect().await;
        let project_id = project(&mut setup).await;
        let payer = participant(&mut setup, project_id).await;
        let debtor = participant(&mut setup, project_id).await;
        let rule = recurring_rule(&mut setup, project_id, &[payer, debtor]).await;

        let mut conn = connect().await;
        sqlx::query("BEGIN").execute(&mut conn).await.unwrap();
        let result = remove_participant(&mut conn, project_id, debtor).await;
        sqlx::query("ROLLBACK").execute(&mut conn).await.unwrap();

        let (participants,): (i64,) =
            sqlx::query_as("SELECT count(*) FROM recurring_expense_participants WHERE recurring_id = $1")
                .bind(rule)
                .fetch_one(&mut setup)
                .await
                .unwrap();
        drop_fixture(&mut setup, project_id, vec![payer, debtor]).await;

        assert_eq!(message(&result.unwrap_err()), errors::USER_IN_RECURRING);
        assert_eq!(participants, 2);
    });
}

/// The removal starts while a materialization paying the participant is uncommitted. It waits on
/// the payment's FK lock and then refuses: the occurrences and the rule survive intact.
#[test]
#[ignore]
fn removal_racing_a_materialization_leaves_both_intact() {
    run(async {
        let mut setup = connect().await;
        let project_id = project(&mut setup).await;
        let payer = participant(&mut setup, project_id).await;
        let debtor = participant(&mut setup, project_id).await;
        let rule = recurring_rule(&mut setup, project_id, &[payer, debtor]).await;

        let mut writer = connect().await;
        sqlx::query("BEGIN").execute(&mut writer).await.unwrap();
        materialize(
            &mut writer,
            materialization(rule, project_id, payer, debtor, &[Uuid::new_v4()]),
            today(),
            None,
        )
        .await
        .unwrap();

        let local = tokio::task::LocalSet::new();
        let removal = local
            .run_until(async {
                let removal = tokio::task::spawn_local(async move {
                    let mut remover = connect().await;
                    sqlx::query("BEGIN").execute(&mut remover).await.unwrap();
                    let result = remove_participant(&mut remover, project_id, debtor).await;
                    let end = if result.is_ok() { "COMMIT" } else { "ROLLBACK" };
                    sqlx::query(end).execute(&mut remover).await.unwrap();
                    result
                });
                someone_waits_on_a_lock(&mut setup).await;
                sqlx::query("COMMIT").execute(&mut writer).await.unwrap();
                removal.await.unwrap()
            })
            .await;

        let expenses = expense_count(&mut setup, project_id).await;
        let version = rule_version(&mut setup, rule).await;
        drop_fixture(&mut setup, project_id, vec![payer, debtor]).await;

        assert!(removal.is_err(), "a participant was removed from under a live rule");
        assert_eq!(expenses, 1);
        assert_eq!(version, 1);
    });
}

fn rule_history(actor_user_id: i32) -> Option<HistoryContext> {
    Some(HistoryContext { actor_user_id, payload: EncryptedPair::default() })
}

async fn rule_history_actions(conn: &mut PgConnection, project_id: Uuid) -> Vec<HistoryAction> {
    sqlx::query_as::<_, (HistoryAction,)>(
        "SELECT action FROM project_history \
         WHERE project_id = $1 AND entity = 'project' AND entity_id IS NULL ORDER BY id",
    )
    .bind(project_id)
    .fetch_all(&mut *conn)
    .await
    .unwrap()
    .into_iter()
    .map(|(a,)| a)
    .collect()
}

/// Setting up, editing and stopping a rule each leave one history row, filed under the project.
#[test]
#[ignore]
fn each_rule_event_writes_one_history_row() {
    run(async {
        let mut setup = connect().await;
        let project_id = project(&mut setup).await;
        let payer = participant(&mut setup, project_id).await;
        let debtor = participant(&mut setup, project_id).await;

        let mut conn = connect().await;
        let rule = add_recurring(
            &mut conn,
            CreatableRecurringExpense {
                project_id,
                author_id: payer,
                participant_ids: vec![payer, debtor],
                payload: EncryptedPair::default(),
                history: rule_history(payer),
            },
            None,
        )
        .await
        .unwrap();
        edit_recurring(
            &mut conn,
            EditableRecurringExpense {
                id: rule.id,
                project_id,
                participant_ids: vec![payer, debtor],
                payload: EncryptedPair::default(),
                expected_version: rule.version,
                history: rule_history(debtor),
            },
            None,
        )
        .await
        .unwrap();
        delete_recurring(
            &mut conn,
            DeleteRecurringExpenseRequest { id: rule.id, project_id, history: rule_history(payer) },
            None,
        )
        .await
        .unwrap();

        let actions = rule_history_actions(&mut setup, project_id).await;
        drop_fixture(&mut setup, project_id, vec![payer, debtor]).await;

        assert_eq!(actions, vec![HistoryAction::Created, HistoryAction::Updated, HistoryAction::Deleted]);
    });
}

/// The history row shares the rule's transaction: an actor from outside the project is refused and
/// the edit it would have signed does not happen.
#[test]
#[ignore]
fn a_refused_history_actor_rolls_back_the_rule_edit() {
    run(async {
        let mut setup = connect().await;
        let project_id = project(&mut setup).await;
        let payer = participant(&mut setup, project_id).await;
        let debtor = participant(&mut setup, project_id).await;
        let elsewhere = project(&mut setup).await;
        let outsider = participant(&mut setup, elsewhere).await;
        let rule = recurring_rule(&mut setup, project_id, &[payer, debtor]).await;

        let mut conn = connect().await;
        sqlx::query("BEGIN").execute(&mut conn).await.unwrap();
        let result = edit_recurring(
            &mut conn,
            EditableRecurringExpense {
                id: rule,
                project_id,
                participant_ids: vec![payer, debtor],
                payload: EncryptedPair::default(),
                expected_version: 0,
                history: rule_history(outsider),
            },
            None,
        )
        .await;
        sqlx::query("ROLLBACK").execute(&mut conn).await.unwrap();

        let version = rule_version(&mut setup, rule).await;
        let actions = rule_history_actions(&mut setup, project_id).await;
        drop_fixture(&mut setup, project_id, vec![payer, debtor]).await;
        drop_fixture(&mut setup, elsewhere, vec![outsider]).await;

        assert_eq!(message(&result.unwrap_err()), errors::PARTICIPANT_NOT_IN_PROJECT);
        assert_eq!(version, 0, "the edit survived its refused history row");
        assert!(actions.is_empty());
    });
}
