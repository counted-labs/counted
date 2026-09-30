//! Write paths racing each other against a real Postgres. Ignored by default; run against the e2e
//! database (`make e2e-backend`) or any migrated one with
//!
//!   COUNTED_TEST_DATABASE_URL=… cargo test -p api --features server --test concurrent_writes -- --ignored
//!
//! Needs committed transactions, so each case cleans its fixture project up itself.
#![cfg(feature = "server")]

use api::server::projects::projects_repository::update_project_by_id;
use api::server::users::remove_participant;
use shared::{EditableProject, EncryptedPair, ProjectStatus};
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
