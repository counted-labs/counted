use dioxus::fullstack::Json;
use dioxus::prelude::*;
use shared::{
    BatchProject, CreatableProject, EditableProject, JoinProject, LeaveProject, ProjectDto,
};
use uuid::Uuid;

#[get("/api/v1/projects/{project_id}")]
pub async fn get_project(project_id: Uuid) -> Result<ProjectDto, ServerFnError> {
    crate::server::projects::get_project(project_id).await
}

#[post("/api/v1/projects/batch")]
pub async fn get_projects_by_ids(
    Json(payload): Json<BatchProject>,
) -> Result<Vec<ProjectDto>, ServerFnError> {
    crate::server::projects::get_projects_by_ids(payload).await
}

#[post("/api/v1/projects")]
pub async fn add_project(
    Json(creatable_project): Json<CreatableProject>,
) -> Result<ProjectDto, ServerFnError> {
    crate::server::projects::add_project(creatable_project).await
}

#[put("/api/v1/projects")]
pub async fn update_project_by_id(
    Json(editable_project): Json<EditableProject>,
) -> Result<ProjectDto, ServerFnError> {
    crate::server::projects::update_project_by_id(editable_project).await
}

/// Registers a holder that has no account, so the project knows someone is still in it.
/// Idempotent: re-registering the same `member_id` is a no-op.
#[post("/api/v1/projects/members")]
pub async fn join_project(Json(payload): Json<JoinProject>) -> Result<(), ServerFnError> {
    crate::server::projects::join_project(payload).await
}

/// Leaves a project. Removing the last member does **not** delete it here.
///
/// No session is required — anonymous holders have to be able to leave too. That is exactly why
/// teardown cannot happen inline: joining is free, so anyone holding a project id could join a
/// dormant project and leave again, driving the member count 0 → 1 → 0 and destroying it and every
/// expense in it with two unauthenticated requests. A project left with no members is reaped by the
/// out-of-band sweep instead, which can tell an abandoned project from a freshly-vacated one.
#[post("/api/v1/projects/leave")]
pub async fn leave_project(Json(payload): Json<LeaveProject>) -> Result<(), ServerFnError> {
    crate::server::projects::leave_project(payload).await
}
