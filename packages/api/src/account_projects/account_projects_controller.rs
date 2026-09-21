use dioxus::fullstack::Json;
use dioxus::prelude::*;
use shared::{AccountProject, BatchUpsertResult, UpsertAccountProject};

#[get("/api/v1/account/projects")]
pub async fn get_account_projects() -> Result<Vec<AccountProject>, ServerFnError> {
    crate::server::account_projects::get_account_projects().await
}

/// Membership is open to any holder of the project id — that is the sharing model. What is not
/// open is the identity: a `user_id` must name a participant of *this* project and, once the
/// project has a verifier, must come with the token only the project key produces. See
/// `account_projects_repository::check_claim`.
#[post("/api/v1/account/projects")]
pub async fn upsert_account_project(
    Json(payload): Json<UpsertAccountProject>,
) -> Result<(), ServerFnError> {
    crate::server::account_projects::upsert_account_project(payload).await
}

#[post("/api/v1/account/projects/batch")]
pub async fn batch_upsert_account_projects(
    Json(payload): Json<Vec<UpsertAccountProject>>,
) -> Result<BatchUpsertResult, ServerFnError> {
    crate::server::account_projects::batch_upsert_account_projects(payload).await
}
