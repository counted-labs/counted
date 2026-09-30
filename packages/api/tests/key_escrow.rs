//! What the server escrows for an account, against a real Postgres. Ignored by default; run
//! against the e2e database (`make e2e-backend`) or any migrated one with
//!
//!   COUNTED_TEST_DATABASE_URL=… cargo test -p api --features server --test key_escrow -- --ignored
//!
//! Every case runs in a transaction that is rolled back.
#![cfg(feature = "server")]

use api::server::account_projects::account_projects_repository::batch_upsert_account_projects;
use sha2::{Digest, Sha256};
use shared::{EncryptedPair, UpsertAccountProject};
use sqlx::{Connection, PgConnection};
use uuid::Uuid;

const E2E_DATABASE_URL: &str = "postgres://hcount_user:e2e@127.0.0.1:5432/hcount";
const GOOD_TOKEN: [u8; 32] = [1; 32];
const WRONG_TOKEN: [u8; 32] = [2; 32];

fn run<F: std::future::Future>(f: F) -> F::Output {
    tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap().block_on(f)
}

async fn connect() -> PgConnection {
    let url = std::env::var("COUNTED_TEST_DATABASE_URL").unwrap_or_else(|_| E2E_DATABASE_URL.into());
    PgConnection::connect(&url).await.expect("e2e database — run `make e2e-backend` first")
}

fn wrapped(tag: &str) -> EncryptedPair {
    EncryptedPair { ct: tag.into(), iv: "A".repeat(32) }
}

/// A device that took a wrong key from a link or an invitation pushes it wrapped, with the claim
/// token derived from that same wrong key. The escrowed key must stay the good one.
#[test]
#[ignore]
fn a_key_whose_token_misses_the_verifier_is_not_escrowed() {
    run(async {
        let mut conn = connect().await;
        sqlx::query("BEGIN").execute(&mut conn).await.unwrap();

        let (account_id,): (Uuid,) = sqlx::query_as(
            "INSERT INTO accounts (email, kdf_salt, login_salt, login_hash) VALUES ($1, $2, $2, $3) RETURNING id",
        )
        .bind(format!("escrow-{}@test.invalid", Uuid::new_v4()))
        .bind(vec![7u8; 16])
        .bind(vec![7u8; 32])
        .fetch_one(&mut conn)
        .await
        .unwrap();
        let verifier: [u8; 32] = Sha256::digest(GOOD_TOKEN).into();
        let (project_id,): (Uuid,) = sqlx::query_as(
            "INSERT INTO projects (payload_ct, payload_iv, claim_verifier) \
             VALUES (decode('00','hex'), decode('00','hex'), $1) RETURNING id",
        )
        .bind(verifier.to_vec())
        .fetch_one(&mut conn)
        .await
        .unwrap();

        let push = |key: &str, token: [u8; 32]| UpsertAccountProject {
            project_id,
            user_id: None,
            key: Some(wrapped(key)),
            claim_label: None,
            claim_token: Some(token.to_vec()),
        };
        batch_upsert_account_projects(&mut conn, account_id, vec![push("R09PRA==", GOOD_TOKEN)])
            .await
            .unwrap();
        batch_upsert_account_projects(&mut conn, account_id, vec![push("V1JPTkc=", WRONG_TOKEN)])
            .await
            .unwrap();

        let (key_ct,): (Vec<u8>,) = sqlx::query_as(
            "SELECT key_ct FROM account_projects WHERE account_id = $1 AND project_id = $2",
        )
        .bind(account_id)
        .bind(project_id)
        .fetch_one(&mut conn)
        .await
        .unwrap();

        sqlx::query("ROLLBACK").execute(&mut conn).await.unwrap();

        assert_eq!(key_ct, b"R09PRA==", "a key that does not open the project replaced the escrowed one");
    });
}
