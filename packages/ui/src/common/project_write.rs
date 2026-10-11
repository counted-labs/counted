use api::projects::projects_controller::{get_project, update_project_by_id};
use dioxus::fullstack::Json;
use dioxus::prelude::*;
use shared::{EditableProject, ProjectDto};
use uuid::Uuid;

use super::error_utils::is_project_stale_error;

const ATTEMPTS: usize = 3;

/// Builds the edit from the project as stored now, not as the page last saw it, and writes it only
/// if nothing replaced that payload in between — otherwise rereads and builds again. What another
/// device or a newer build wrote meanwhile is kept instead of overwritten.
pub async fn update_project_fresh(
    project_id: Uuid,
    edit: impl Fn(&ProjectDto) -> Result<EditableProject, String>,
) -> Result<ProjectDto, ServerFnError> {
    let mut stale = None;
    for _ in 0..ATTEMPTS {
        let stored = get_project(project_id).await?;
        let mut editable = edit(&stored).map_err(ServerFnError::new)?;
        editable.expected_iv = editable.payload.as_ref().map(|_| stored.payload.iv.clone());
        match update_project_by_id(Json(editable)).await {
            Err(e) if is_project_stale_error(&e) => stale = Some(e),
            result => return result,
        }
    }
    Err(stale.unwrap_or_else(|| ServerFnError::new(shared::errors::PROJECT_STALE)))
}
