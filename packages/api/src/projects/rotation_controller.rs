//! Project key rotation (docs/plans/key-lifecycle.md §8.3-8.6). Every route but the account's own
//! boxes carries the project's token; the rotation routes also need an admin session.

use dioxus::fullstack::Json;
use dioxus::prelude::*;
use shared::{
    AbortRotation, CommitRotation, ProjectRef, ReadRotationRows, RotationBox, RotationMember, RotationRow, RotationStarted,
    SetMemberKey, StageRotationRows, StartRotation,
};

/// Freezes the project for an hour: other writes answer 409 `PROJECT_ROTATING` until the commit.
#[post("/api/v1/projects/rotation")]
pub async fn start_rotation(Json(req): Json<StartRotation>) -> Result<RotationStarted, ServerFnError> {
    crate::server::rotation::start_rotation(req).await
}

#[post("/api/v1/projects/rotation/rows")]
pub async fn read_rotation_rows(Json(req): Json<ReadRotationRows>) -> Result<Vec<RotationRow>, ServerFnError> {
    crate::server::rotation::read_rotation_rows(req).await
}

#[put("/api/v1/projects/rotation/rows")]
pub async fn stage_rotation_rows(Json(req): Json<StageRotationRows>) -> Result<(), ServerFnError> {
    crate::server::rotation::stage_rotation_rows(req).await
}

#[post("/api/v1/projects/rotation/members")]
pub async fn rotation_members(Json(req): Json<ProjectRef>) -> Result<Vec<RotationMember>, ServerFnError> {
    crate::server::rotation::rotation_members(req).await
}

#[post("/api/v1/projects/rotation/commit")]
pub async fn commit_rotation(Json(req): Json<CommitRotation>) -> Result<(), ServerFnError> {
    crate::server::rotation::commit_rotation(req).await
}

#[post("/api/v1/projects/rotation/abort")]
pub async fn abort_rotation(Json(req): Json<AbortRotation>) -> Result<(), ServerFnError> {
    crate::server::rotation::abort_rotation(req).await
}

#[get("/api/v1/account/rotation-boxes")]
pub async fn rotation_boxes() -> Result<Vec<RotationBox>, ServerFnError> {
    crate::server::rotation::rotation_boxes().await
}

#[post("/api/v1/account/rotation-boxes/done")]
pub async fn drop_rotation_box(Json(req): Json<ProjectRef>) -> Result<(), ServerFnError> {
    crate::server::rotation::drop_rotation_box(req).await
}

#[put("/api/v1/account/projects/member-key")]
pub async fn set_member_key(Json(req): Json<SetMemberKey>) -> Result<(), ServerFnError> {
    crate::server::rotation::set_member_key(req).await
}
