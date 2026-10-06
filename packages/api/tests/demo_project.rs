//! `/demo` projects: never owned, never a member, so the nightly sweep removes them. Ignored by
//! default; run against the e2e database (`make e2e-backend`) or any migrated one with
//!
//!   COUNTED_TEST_DATABASE_URL=… cargo test -p api --features server --test demo_project -- --ignored
//!
//! Each test runs in a transaction it rolls back.
#![cfg(feature = "server")]

use api::server::account_projects::account_projects_repository::upsert_account_project;
use api::server::projects::projects_repository::{add_anonymous_member, add_project};
use shared::{CreatableProject, EncryptedPair};
use sqlx::{Connection, PgConnection};
use uuid::Uuid;

const E2E_DATABASE_URL: &str = "postgres://hcount_user:e2e@127.0.0.1:5432/hcount";

fn run<F: std::future::Future>(f: F) -> F::Output {
    tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap().block_on(f)
}

async fn begin() -> PgConnection {
    let url = std::env::var("COUNTED_TEST_DATABASE_URL").unwrap_or_else(|_| E2E_DATABASE_URL.into());
    let mut conn = PgConnection::connect(&url).await.expect("e2e database — run `make e2e-backend` first");
    sqlx::query("BEGIN").execute(&mut conn).await.unwrap();
    conn
}

async fn rollback(mut conn: PgConnection) {
    sqlx::query("ROLLBACK").execute(&mut conn).await.unwrap();
}

async fn account(conn: &mut PgConnection) -> Uuid {
    let (id,): (Uuid,) = sqlx::query_as(
        "INSERT INTO accounts (email, kdf_salt, login_salt, login_hash) VALUES ($1, $2, $2, $3) RETURNING id",
    )
    .bind(format!("demo-{}@test.invalid", Uuid::new_v4()))
    .bind(vec![7u8; 16])
    .bind(vec![7u8; 32])
    .fetch_one(&mut *conn)
    .await
    .unwrap();
    id
}

async fn project(conn: &mut PgConnection, demo: bool, owner: Option<Uuid>) -> Uuid {
    let payload = EncryptedPair { ct: "ZGVtbw==".into(), iv: "A".repeat(32) };
    add_project(conn, CreatableProject { payload, claim_verifier: None, demo }, owner).await.unwrap()
}

async fn row(conn: &mut PgConnection, project_id: Uuid) -> Option<(bool, Option<Uuid>)> {
    sqlx::query_as("SELECT demo, owner_account_id FROM projects WHERE id = $1")
        .bind(project_id)
        .fetch_optional(&mut *conn)
        .await
        .unwrap()
}

async fn count(conn: &mut PgConnection, table: &str, project_id: Uuid) -> i64 {
    let (n,): (i64,) = sqlx::query_as(&format!("SELECT count(*) FROM {table} WHERE project_id = $1"))
        .bind(project_id)
        .fetch_one(&mut *conn)
        .await
        .unwrap();
    n
}

/// A signed-in visitor's session must not make the demo theirs, by any path.
#[test]
#[ignore]
fn a_demo_project_never_gets_an_owner_or_a_member() {
    run(async {
        let mut conn = begin().await;
        let owner = account(&mut conn).await;
        let id = project(&mut conn, true, Some(owner)).await;
        let owned_at_insert = row(&mut conn, id).await.unwrap().1;

        sqlx::query("UPDATE projects SET owner_account_id = $2 WHERE id = $1")
            .bind(id)
            .bind(owner)
            .execute(&mut conn)
            .await
            .unwrap();
        let owned_after_update = row(&mut conn, id).await.unwrap().1;

        let anonymous_join = add_anonymous_member(&mut conn, id, Uuid::new_v4()).await;
        let _ = upsert_account_project(&mut conn, owner, id, None, None, None).await;
        let anonymous = count(&mut conn, "anonymous_project_members", id).await;
        let accounts = count(&mut conn, "account_projects", id).await;
        rollback(conn).await;

        assert_eq!(owned_at_insert, None);
        assert_eq!(owned_after_update, None);
        assert!(anonymous_join.is_ok());
        assert_eq!((anonymous, accounts), (0, 0));
    });
}

/// Turning a real project into a demo one would hand it to the sweep.
#[test]
#[ignore]
fn the_flag_is_fixed_at_insert() {
    run(async {
        let mut conn = begin().await;
        let real = project(&mut conn, false, None).await;
        let demo = project(&mut conn, true, None).await;
        sqlx::query("UPDATE projects SET demo = NOT demo WHERE id = ANY($1)")
            .bind(vec![real, demo])
            .execute(&mut conn)
            .await
            .unwrap();
        let (real_flag, demo_flag) = (row(&mut conn, real).await.unwrap().0, row(&mut conn, demo).await.unwrap().0);
        rollback(conn).await;

        assert!(!real_flag);
        assert!(demo_flag);
    });
}

/// The `counted-db-sweep` project rule (nix/configuration.nix), scoped to this test's rows.
#[test]
#[ignore]
fn the_existing_sweep_takes_a_day_old_demo_and_spares_a_held_project() {
    run(async {
        let mut conn = begin().await;
        let demo = project(&mut conn, true, None).await;
        let held = project(&mut conn, false, None).await;
        add_anonymous_member(&mut conn, held, Uuid::new_v4()).await.unwrap();
        add_anonymous_member(&mut conn, demo, Uuid::new_v4()).await.unwrap();
        sqlx::query("UPDATE projects SET created_at = NOW() - INTERVAL '25 hours' WHERE id = ANY($1)")
            .bind(vec![demo, held])
            .execute(&mut conn)
            .await
            .unwrap();

        sqlx::query(
            "DELETE FROM projects p \
             WHERE p.id = ANY($1) \
               AND p.created_at < NOW() - INTERVAL '24 hours' \
               AND p.created_at >= TIMESTAMP '2026-08-16 00:00:00' \
               AND p.owner_account_id IS NULL \
               AND NOT p.read_only \
               AND NOT EXISTS (SELECT 1 FROM account_projects ap WHERE ap.project_id = p.id) \
               AND NOT EXISTS (SELECT 1 FROM anonymous_project_members am WHERE am.project_id = p.id)",
        )
        .bind(vec![demo, held])
        .execute(&mut conn)
        .await
        .unwrap();
        let (demo_left, held_left) = (row(&mut conn, demo).await, row(&mut conn, held).await);
        rollback(conn).await;

        assert!(demo_left.is_none());
        assert!(held_left.is_some());
    });
}
