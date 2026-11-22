use dioxus::{fullstack::Json, prelude::*};
use shared::{DistinctionKeys, PublishDistinctions, SignDistinctions, SignedDistinction};

/// The public keys members check a shown badge with (docs/plans/gamification.md §4). `None` while
/// the server has no signing seed: shown badges are off.
#[get("/api/v1/distinctions/keys")]
pub async fn distinction_keys() -> Result<Option<DistinctionKeys>, ServerFnError> {
    crate::server::distinctions::keys().await
}

/// One signature per membership where the session's account holds a name, for a badge it has
/// earned. Nothing is stored.
#[post("/api/v1/distinctions/sign")]
pub async fn sign_distinctions(Json(payload): Json<SignDistinctions>) -> Result<Vec<SignedDistinction>, ServerFnError> {
    crate::server::distinctions::sign(payload).await
}

/// The holder's own copies, each encrypted under its project key. Every membership not listed is
/// cleared, so an empty list takes the shown badge down everywhere.
#[put("/api/v1/distinctions")]
pub async fn publish_distinctions(Json(payload): Json<PublishDistinctions>) -> Result<(), ServerFnError> {
    crate::server::distinctions::publish(payload).await
}
