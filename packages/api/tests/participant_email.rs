//! A participant named after an email address must not tell anyone whether that address has an
//! account. Ignored by default; run against the e2e database (`make e2e-backend`) or any migrated
//! one with
//!
//!   COUNTED_TEST_DATABASE_URL=… cargo test -p api --features server --test participant_email -- --ignored
//!
//! Every case runs in a transaction that is rolled back.
#![cfg(feature = "server")]

use api::server::users::users_repository::{add_users, get_users_by_project_id};
use shared::CreatableUser;
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

/// What a build older than the removal still sends: the participant's name as `invitedEmail`.
#[test]
#[ignore]
fn an_email_participant_reveals_nothing_about_the_account() {
    run(async {
        let mut conn = connect().await;
        sqlx::query("BEGIN").execute(&mut conn).await.unwrap();

        let email = format!("victim-{}@test.invalid", Uuid::new_v4());
        let (account_id,): (Uuid,) = sqlx::query_as(
            "INSERT INTO accounts (email, kdf_salt, login_salt, login_hash) VALUES ($1, $2, $2, $3) RETURNING id",
        )
        .bind(&email)
        .bind(vec![7u8; 16])
        .bind(vec![7u8; 32])
        .fetch_one(&mut conn)
        .await
        .unwrap();
        let (project_id,): (Uuid,) = sqlx::query_as(
            "INSERT INTO projects (payload_ct, payload_iv) VALUES (decode('00','hex'), decode('00','hex')) RETURNING id",
        )
        .fetch_one(&mut conn)
        .await
        .unwrap();

        let from_old_client: CreatableUser = serde_json::from_value(serde_json::json!({
            "payload": { "ct": "bmFtZQ==", "iv": "A".repeat(32) },
            "projectId": project_id,
            "invitedEmail": email,
        }))
        .unwrap();
        add_users(&mut conn, vec![from_old_client]).await.unwrap();

        let roster = get_users_by_project_id(&mut conn, project_id).await.unwrap();
        let (linked,): (i64,) =
            sqlx::query_as("SELECT count(*) FROM account_projects WHERE account_id = $1")
                .bind(account_id)
                .fetch_one(&mut conn)
                .await
                .unwrap();
        let (hashed,): (i64,) = sqlx::query_as(
            "SELECT count(*) FROM users u JOIN user_projects up ON up.user_id = u.id \
             WHERE up.project_id = $1 AND u.email_hash IS NOT NULL",
        )
        .bind(project_id)
        .fetch_one(&mut conn)
        .await
        .unwrap();

        sqlx::query("ROLLBACK").execute(&mut conn).await.unwrap();

        assert!(!roster[0].claimed, "`claimed` reveals that the address has an account");
        assert_eq!(linked, 0, "the project was put in a stranger's list");
        assert_eq!(hashed, 0, "the address was stored as an unsalted hash");
    });
}
