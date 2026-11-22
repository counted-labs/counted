use api::projects::projects_controller::{get_project, update_project_by_id};
use dioxus::fullstack::Json;
use dioxus::prelude::*;
use shared::{EditableProject, ProjectDto};
use uuid::Uuid;

use super::error_utils::is_project_stale_error;

const ATTEMPTS: usize = 3;

/// Builds the edit from the project as stored now, not as the page last saw it, and writes it only
/// if nothing replaced that payload in between - otherwise rereads and builds again. What another
/// device or a newer build wrote meanwhile is kept instead of overwritten.
pub async fn update_project_fresh(
    project_id: Uuid,
    edit: impl Fn(&ProjectDto) -> Result<EditableProject, String>,
) -> Result<ProjectDto, ServerFnError> {
    fresh(project_id, None, edit).await
}

/// [`update_project_fresh`] with `key`'s token on both requests, the key `edit` seals under: a
/// switch between them cannot pair this payload with another key's token.
pub async fn update_project_fresh_under(
    project_id: Uuid,
    key: [u8; 32],
    edit: impl Fn(&ProjectDto) -> Result<EditableProject, String>,
) -> Result<ProjectDto, ServerFnError> {
    fresh(project_id, Some(key), edit).await
}

async fn fresh(
    project_id: Uuid,
    key: Option<[u8; 32]>,
    edit: impl Fn(&ProjectDto) -> Result<EditableProject, String>,
) -> Result<ProjectDto, ServerFnError> {
    let scope = |id: Uuid| match &key {
        Some(key) => crate::common::project_token::scope_key(Some(key)),
        None => crate::common::project_token::scope(id),
    };
    let mut stale = None;
    for _ in 0..ATTEMPTS {
        scope(project_id);
        let stored = get_project(project_id).await?;
        let mut editable = edit(&stored).map_err(ServerFnError::new)?;
        editable.expected_iv = editable.payload.as_ref().map(|_| stored.payload.iv.clone());
        scope(editable.id);
        match update_project_by_id(Json(editable)).await {
            Err(e) if is_project_stale_error(&e) => stale = Some(e),
            result => return result,
        }
    }
    Err(stale.unwrap_or_else(|| ServerFnError::new(shared::errors::PROJECT_STALE)))
}
