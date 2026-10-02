//! The read-only demo project, enforced by triggers. Ignored by default; run against the e2e
//! database (`make e2e-backend`) or any migrated one with
//!
//!   COUNTED_TEST_DATABASE_URL=… cargo test -p api --features server --test read_only_project -- --ignored
#![cfg(feature = "server")]

use api::server::projects::projects_repository::{add_anonymous_member, update_project_by_id};
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

async fn user(conn: &mut PgConnection) -> i32 {
    let (id,): (i32,) =
        sqlx::query_as("INSERT INTO users (payload_ct, payload_iv) VALUES (decode('00','hex'), decode('00','hex')) RETURNING id")
            .fetch_one(&mut *conn)
            .await
            .unwrap();
    id
}

async fn join(conn: &mut PgConnection, project_id: Uuid, user_id: i32) -> Result<(), sqlx::Error> {
    sqlx::query("INSERT INTO user_projects (project_id, user_id) VALUES ($1, $2)")
        .bind(project_id)
        .bind(user_id)
        .execute(&mut *conn)
        .await
        .map(|_| ())
}

async fn add_expense(conn: &mut PgConnection, project_id: Uuid, author: i32) -> Result<i32, sqlx::Error> {
    let (id,): (i32,) = sqlx::query_as(
        "INSERT INTO expenses (project_id, author_id, payload_ct, payload_iv) \
         VALUES ($1, $2, decode('00','hex'), decode('00','hex')) RETURNING id",
    )
    .bind(project_id)
    .bind(author)
    .fetch_one(&mut *conn)
    .await?;
    sqlx::query(
        "INSERT INTO payments (expense_id, user_id, payload_ct, payload_iv) \
         VALUES ($1, $2, decode('00','hex'), decode('00','hex'))",
    )
    .bind(id)
    .bind(author)
    .execute(&mut *conn)
    .await?;
    Ok(id)
}

async fn exec(conn: &mut PgConnection, sql: &str, project_id: Uuid) -> Result<u64, sqlx::Error> {
    sqlx::query(sql).bind(project_id).execute(&mut *conn).await.map(|r| r.rows_affected())
}

async fn set_read_only(conn: &mut PgConnection, project_id: Uuid, on: bool) {
    sqlx::query("UPDATE projects SET read_only = $2 WHERE id = $1")
        .bind(project_id)
        .bind(on)
        .execute(&mut *conn)
        .await
        .unwrap();
}

/// A project with one participant, one expense and one recurring rule, then frozen.
async fn frozen_project(conn: &mut PgConnection) -> (Uuid, i32, i32) {
    let (project_id,): (Uuid,) = sqlx::query_as(
        "INSERT INTO projects (payload_ct, payload_iv) VALUES (decode('00','hex'), decode('00','hex')) RETURNING id",
    )
    .fetch_one(&mut *conn)
    .await
    .unwrap();
    let member = user(conn).await;
    join(conn, project_id, member).await.unwrap();
    let expense = add_expense(conn, project_id, member).await.unwrap();
    exec(
        conn,
        "INSERT INTO recurring_expenses (project_id, payload_ct, payload_iv) \
         VALUES ($1, decode('00','hex'), decode('00','hex'))",
        project_id,
    )
    .await
    .unwrap();
    set_read_only(conn, project_id, true).await;
    (project_id, member, expense)
}

async fn cleanup(conn: &mut PgConnection, project_id: Uuid, strays: &[i32]) {
    set_read_only(conn, project_id, false).await;
    exec(conn, "DELETE FROM users WHERE id IN (SELECT user_id FROM user_projects WHERE project_id = $1)", project_id)
        .await
        .unwrap();
    exec(conn, "DELETE FROM projects WHERE id = $1", project_id).await.unwrap();
    sqlx::query("DELETE FROM users WHERE id = ANY($1)").bind(strays).execute(&mut *conn).await.unwrap();
}

#[test]
#[ignore]
fn a_read_only_project_refuses_every_content_write() {
    run(async {
        let mut conn = connect().await;
        let (project_id, member, expense) = frozen_project(&mut conn).await;

        let newcomer = user(&mut conn).await;
        let refused = [
            ("add expense", add_expense(&mut conn, project_id, member).await.is_err()),
            (
                "edit expense",
                sqlx::query("UPDATE expenses SET payload_ct = decode('01','hex') WHERE id = $1")
                    .bind(expense)
                    .execute(&mut conn)
                    .await
                    .is_err(),
            ),
            (
                "delete expense",
                sqlx::query("DELETE FROM expenses WHERE id = $1").bind(expense).execute(&mut conn).await.is_err(),
            ),
            ("add participant", join(&mut conn, project_id, newcomer).await.is_err()),
            ("remove participant", remove_participant(&mut conn, project_id, member).await.is_err()),
            (
                "add recurring rule",
                exec(
                    &mut conn,
                    "INSERT INTO recurring_expenses (project_id, payload_ct, payload_iv) \
                     VALUES ($1, decode('00','hex'), decode('00','hex'))",
                    project_id,
                )
                .await
                .is_err(),
            ),
            (
                "delete recurring rule",
                exec(&mut conn, "DELETE FROM recurring_expenses WHERE project_id = $1", project_id).await.is_err(),
            ),
            (
                "rename project",
                update_project_by_id(
                    &mut conn,
                    EditableProject {
                        id: project_id,
                        payload: Some(EncryptedPair { ct: "cmVuYW1lZA==".into(), iv: "A".repeat(32) }),
                        status: None,
                        history: None,
                    },
                )
                .await
                .is_err(),
            ),
            (
                "change status",
                update_project_by_id(
                    &mut conn,
                    EditableProject { id: project_id, payload: None, status: Some(ProjectStatus::Archived), history: None },
                )
                .await
                .is_err(),
            ),
        ];

        let (expenses,): (i64,) = sqlx::query_as("SELECT count(*) FROM expenses WHERE project_id = $1")
            .bind(project_id)
            .fetch_one(&mut conn)
            .await
            .unwrap();
        cleanup(&mut conn, project_id, &[newcomer]).await;

        for (write, was_refused) in refused {
            assert!(was_refused, "{write} went through on a read-only project");
        }
        assert_eq!(expenses, 1);
    });
}

/// Opening the demo must not make anyone a member, and must not fail for trying.
#[test]
#[ignore]
fn joining_a_read_only_project_succeeds_and_writes_nothing() {
    run(async {
        let mut conn = connect().await;
        let (project_id, _, _) = frozen_project(&mut conn).await;

        let joined = add_anonymous_member(&mut conn, project_id, Uuid::new_v4()).await;
        let (members,): (i64,) =
            sqlx::query_as("SELECT count(*) FROM anonymous_project_members WHERE project_id = $1")
                .bind(project_id)
                .fetch_one(&mut conn)
                .await
                .unwrap();
        cleanup(&mut conn, project_id, &[]).await;

        assert!(joined.is_ok());
        assert_eq!(members, 0);
    });
}

/// A signed-in visitor's first upsert seeds the claim verifier; refusing it would fail their login.
#[test]
#[ignore]
fn administrative_project_columns_stay_writable() {
    run(async {
        let mut conn = connect().await;
        let (project_id, _, _) = frozen_project(&mut conn).await;

        let seeded = exec(
            &mut conn,
            "UPDATE projects SET claim_verifier = decode(repeat('00', 32),'hex') WHERE id = $1",
            project_id,
        )
        .await;
        cleanup(&mut conn, project_id, &[]).await;

        assert_eq!(seeded.unwrap(), 1);
    });
}

#[test]
#[ignore]
fn turning_the_flag_off_restores_writes() {
    run(async {
        let mut conn = connect().await;
        let (project_id, member, _) = frozen_project(&mut conn).await;

        set_read_only(&mut conn, project_id, false).await;
        let added = add_expense(&mut conn, project_id, member).await;
        cleanup(&mut conn, project_id, &[]).await;

        assert!(added.is_ok());
    });
}
