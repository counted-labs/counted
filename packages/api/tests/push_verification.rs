//! The Web Push verification state machine against a real Postgres: the e2e database, which
//! `make e2e-backend` starts and migrates. Ignored by default; run with
//!
//!   cargo test -p api --features server --test push_verification -- --ignored
//!
//! `COUNTED_TEST_DATABASE_URL` overrides the e2e default. Every case runs in a transaction that is
//! rolled back, except the concurrency case, which needs two committed transactions and deletes its
//! fixture account afterwards.
#![cfg(feature = "server")]

use api::server::push::push_repository as repo;
use shared::PushPlatform;
use sqlx::{Connection, PgConnection};
use uuid::Uuid;

const E2E_DATABASE_URL: &str = "postgres://hcount_user:e2e@127.0.0.1:5432/hcount";
const ENDPOINT: &str = "https://ntfy.sh/upTESTTEST0001";

fn run<F: std::future::Future>(f: F) -> F::Output {
    tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap().block_on(f)
}

async fn connect() -> PgConnection {
    let url = std::env::var("COUNTED_TEST_DATABASE_URL").unwrap_or_else(|_| E2E_DATABASE_URL.into());
    PgConnection::connect(&url).await.expect("e2e database — run `make e2e-backend` first")
}

async fn account(conn: &mut PgConnection) -> Uuid {
    let (id,): (Uuid,) = sqlx::query_as(
        "INSERT INTO accounts (email, kdf_salt, login_salt, login_hash) VALUES ($1, $2, $2, $3) RETURNING id",
    )
    .bind(format!("push-{}@test.invalid", Uuid::new_v4()))
    .bind(vec![7u8; 16])
    .bind(vec![7u8; 32])
    .fetch_one(conn)
    .await
    .unwrap();
    id
}

fn keys(seed: u8) -> Option<(Vec<u8>, Vec<u8>)> {
    let mut p256dh = vec![seed; 65];
    p256dh[0] = 0x04;
    Some((p256dh, vec![seed; 16]))
}

async fn verified(conn: &mut PgConnection, token: &str) -> bool {
    let (v,): (bool,) = sqlx::query_as("SELECT verified_at IS NOT NULL FROM push_tokens WHERE token = $1")
        .bind(token)
        .fetch_one(conn)
        .await
        .unwrap();
    v
}

async fn backdate_challenge(conn: &mut PgConnection, token: &str) {
    sqlx::query("UPDATE push_tokens SET challenged_at = challenged_at - interval '2 hours' WHERE token = $1")
        .bind(token)
        .execute(conn)
        .await
        .unwrap();
}

async fn challenge_and_verify(conn: &mut PgConnection, account: Uuid) {
    assert!(repo::issue_challenge(conn, ENDPOINT, &H1).await.unwrap());
    assert!(repo::verify(conn, account, ENDPOINT, &H1).await.unwrap());
}

async fn targets(conn: &mut PgConnection, account: Uuid) -> Vec<String> {
    repo::tokens_of(conn, &[account]).await.unwrap().into_iter().map(|t| t.token).collect()
}

const H1: [u8; 32] = [1; 32];
const H2: [u8; 32] = [2; 32];

#[test]
#[ignore = "needs the e2e database: make e2e-backend"]
fn ios_rows_are_born_verified_and_android_rows_are_not() {
    run(async {
        let mut conn = connect().await;
        let mut tx = conn.begin().await.unwrap();
        let me = account(&mut tx).await;
        let apns = "ab".repeat(32);

        repo::upsert(&mut tx, me, PushPlatform::Ios, &apns, "en", None).await.unwrap();
        repo::upsert(&mut tx, me, PushPlatform::Android, ENDPOINT, "en", keys(1)).await.unwrap();

        assert!(verified(&mut tx, &apns).await);
        assert!(!verified(&mut tx, ENDPOINT).await);
        assert_eq!(targets(&mut tx, me).await, vec![apns], "an unverified endpoint is never a target");
        tx.rollback().await.unwrap();
    });
}

#[test]
#[ignore = "needs the e2e database: make e2e-backend"]
fn one_challenge_per_hour_per_unverified_endpoint() {
    run(async {
        let mut conn = connect().await;
        let mut tx = conn.begin().await.unwrap();
        let me = account(&mut tx).await;
        repo::upsert(&mut tx, me, PushPlatform::Android, ENDPOINT, "en", keys(1)).await.unwrap();

        assert!(repo::issue_challenge(&mut tx, ENDPOINT, &H1).await.unwrap());
        assert!(!repo::issue_challenge(&mut tx, ENDPOINT, &H2).await.unwrap(), "within the hour");
        repo::upsert(&mut tx, me, PushPlatform::Android, ENDPOINT, "fr", keys(1)).await.unwrap();
        assert!(!repo::issue_challenge(&mut tx, ENDPOINT, &H2).await.unwrap(), "a re-registration does not re-arm");

        backdate_challenge(&mut tx, ENDPOINT).await;
        assert!(repo::issue_challenge(&mut tx, ENDPOINT, &H2).await.unwrap(), "after an hour");
        tx.rollback().await.unwrap();
    });
}

#[test]
#[ignore = "needs the e2e database: make e2e-backend"]
fn only_the_right_fresh_proof_from_the_owner_verifies_once() {
    run(async {
        let mut conn = connect().await;
        let mut tx = conn.begin().await.unwrap();
        let me = account(&mut tx).await;
        let other = account(&mut tx).await;
        repo::upsert(&mut tx, me, PushPlatform::Android, ENDPOINT, "en", keys(1)).await.unwrap();
        assert!(repo::issue_challenge(&mut tx, ENDPOINT, &H1).await.unwrap());

        assert!(!repo::verify(&mut tx, me, ENDPOINT, &H2).await.unwrap(), "wrong proof");
        assert!(!repo::verify(&mut tx, other, ENDPOINT, &H1).await.unwrap(), "another account");
        assert!(!repo::verify(&mut tx, me, "https://ntfy.sh/upOTHEROTHER1", &H1).await.unwrap(), "another endpoint");
        assert!(!verified(&mut tx, ENDPOINT).await);

        assert!(repo::verify(&mut tx, me, ENDPOINT, &H1).await.unwrap());
        assert!(verified(&mut tx, ENDPOINT).await);
        assert_eq!(targets(&mut tx, me).await, vec![ENDPOINT.to_string()]);
        assert!(!repo::verify(&mut tx, me, ENDPOINT, &H1).await.unwrap(), "a proof works once");
        assert!(!repo::issue_challenge(&mut tx, ENDPOINT, &H2).await.unwrap(), "a verified row is not challenged");
        tx.rollback().await.unwrap();
    });
}

#[test]
#[ignore = "needs the e2e database: make e2e-backend"]
fn an_expired_proof_does_not_verify() {
    run(async {
        let mut conn = connect().await;
        let mut tx = conn.begin().await.unwrap();
        let me = account(&mut tx).await;
        repo::upsert(&mut tx, me, PushPlatform::Android, ENDPOINT, "en", keys(1)).await.unwrap();
        assert!(repo::issue_challenge(&mut tx, ENDPOINT, &H1).await.unwrap());
        backdate_challenge(&mut tx, ENDPOINT).await;

        assert!(!repo::verify(&mut tx, me, ENDPOINT, &H1).await.unwrap());
        tx.rollback().await.unwrap();
    });
}

#[test]
#[ignore = "needs the e2e database: make e2e-backend"]
fn verification_survives_only_an_unchanged_binding() {
    run(async {
        let mut conn = connect().await;
        let mut tx = conn.begin().await.unwrap();
        let me = account(&mut tx).await;
        let other = account(&mut tx).await;

        repo::upsert(&mut tx, me, PushPlatform::Android, ENDPOINT, "en", keys(1)).await.unwrap();
        challenge_and_verify(&mut tx, me).await;
        assert!(verified(&mut tx, ENDPOINT).await);

        repo::upsert(&mut tx, me, PushPlatform::Android, ENDPOINT, "de", keys(1)).await.unwrap();
        assert!(verified(&mut tx, ENDPOINT).await, "same account and keys, new language: still verified");

        repo::upsert(&mut tx, me, PushPlatform::Android, ENDPOINT, "de", keys(2)).await.unwrap();
        assert!(!verified(&mut tx, ENDPOINT).await, "new keys: re-verify");

        challenge_and_verify(&mut tx, me).await;
        assert!(verified(&mut tx, ENDPOINT).await);
        repo::upsert(&mut tx, other, PushPlatform::Android, ENDPOINT, "de", keys(2)).await.unwrap();
        assert!(!verified(&mut tx, ENDPOINT).await, "moved to another account: re-verify");
        assert!(targets(&mut tx, other).await.is_empty());
        assert!(targets(&mut tx, me).await.is_empty());
        tx.rollback().await.unwrap();
    });
}

#[test]
#[ignore = "needs the e2e database: make e2e-backend"]
fn the_budget_is_per_account_per_hour() {
    run(async {
        let mut conn = connect().await;
        let mut tx = conn.begin().await.unwrap();
        let me = account(&mut tx).await;

        for expected in 1..=shared::PUSH_CHALLENGES_PER_HOUR + 1 {
            assert_eq!(repo::consume_challenge_budget(&mut tx, me).await.unwrap(), expected);
        }
        sqlx::query("UPDATE accounts SET push_challenge_window = push_challenge_window - interval '2 hours' WHERE id = $1")
            .bind(me)
            .execute(&mut *tx)
            .await
            .unwrap();
        assert_eq!(repo::consume_challenge_budget(&mut tx, me).await.unwrap(), 1, "a new window");
        tx.rollback().await.unwrap();
    });
}

/// Two registrations of the same endpoint racing: the second UPDATE blocks on the row lock, then
/// re-reads `challenged_at` under READ COMMITTED and arms nothing.
#[test]
#[ignore = "needs the e2e database: make e2e-backend"]
fn concurrent_registrations_arm_one_challenge() {
    run(async {
        let mut setup = connect().await;
        let me = account(&mut setup).await;
        repo::upsert(&mut setup, me, PushPlatform::Android, ENDPOINT, "en", keys(1)).await.unwrap();

        let mut first = connect().await;
        let mut tx1 = first.begin().await.unwrap();
        assert!(repo::issue_challenge(&mut tx1, ENDPOINT, &H1).await.unwrap());

        let second = tokio::spawn(async move {
            let mut conn = connect().await;
            let mut tx2 = conn.begin().await.unwrap();
            let armed = repo::issue_challenge(&mut tx2, ENDPOINT, &H2).await.unwrap();
            tx2.commit().await.unwrap();
            armed
        });
        // Long enough for the spawned UPDATE to be waiting on tx1's row lock.
        sqlx::query("SELECT pg_sleep(0.5)").execute(&mut *tx1).await.unwrap();
        tx1.commit().await.unwrap();

        let armed_twice = second.await.unwrap();
        sqlx::query("DELETE FROM accounts WHERE id = $1").bind(me).execute(&mut setup).await.unwrap();
        assert!(!armed_twice, "the second registration must not arm a second challenge");
    });
}
