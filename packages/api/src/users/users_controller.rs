use dioxus::{fullstack::Json, prelude::*};
use shared::{CreatableUserBatch, User};
use uuid::Uuid;

#[delete("/api/v1/projects/{project_id}/users/{user_id}")]
pub async fn delete_user(project_id: Uuid, user_id: i32) -> Result<(), ServerFnError> {
    crate::server::users::delete_user(project_id, user_id).await
}

#[post("/api/v1/users")]
pub async fn add_user(Json(payload): Json<CreatableUserBatch>) -> Result<Vec<User>, ServerFnError> {
    crate::server::users::add_user(payload).await
}

#[get("/api/v1/projects/{project_id}/users")]
pub async fn get_users_by_project_id(project_id: Uuid) -> Result<Vec<User>, ServerFnError> {
    crate::server::users::get_users_by_project_id(project_id).await
}
