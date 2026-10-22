use dioxus::{fullstack::Json, prelude::*};
use shared::{CreatableUserBatch, EditableUser, User};
use uuid::Uuid;

#[delete("/api/v1/projects/{project_id}/users/{user_id}")]
pub async fn delete_user(project_id: Uuid, user_id: i32) -> Result<(), ServerFnError> {
    crate::server::users::delete_user(project_id, user_id).await
}

/// Rewrites a participant's payload. How a participant is removed since
/// docs/plans/participant-links-encryption.md: `removed` inside it, never a delete.
#[put("/api/v1/users")]
pub async fn edit_user(Json(payload): Json<EditableUser>) -> Result<(), ServerFnError> {
    crate::server::users::edit_user(payload).await
}

/// Several [`edit_user`]s of one project in one call. Returns the ids not rewritten.
#[put("/api/v1/users/batch")]
pub async fn edit_users(Json(payload): Json<Vec<EditableUser>>) -> Result<Vec<i32>, ServerFnError> {
    crate::server::users::edit_users(payload).await
}

#[post("/api/v1/users")]
pub async fn add_user(Json(payload): Json<CreatableUserBatch>) -> Result<Vec<User>, ServerFnError> {
    crate::server::users::add_user(payload).await
}

#[get("/api/v1/projects/{project_id}/users")]
pub async fn get_users_by_project_id(project_id: Uuid) -> Result<Vec<User>, ServerFnError> {
    crate::server::users::get_users_by_project_id(project_id).await
}
