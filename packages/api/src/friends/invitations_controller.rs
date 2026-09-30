use dioxus::{fullstack::Json, prelude::*};
use shared::{CreatableProjectInvitation, ProjectInvitation, SentInvitation};
use uuid::Uuid;

/// The project key, boxed by the caller to a friend. The server stores bytes it cannot open —
/// nothing here holds a private key — and the friend joins through the ordinary key-in-local-store
/// path once they accept, or deletes the row to decline.
#[post("/api/v1/projects/{project_id}/invitations")]
pub async fn invite(
    project_id: Uuid,
    Json(payload): Json<CreatableProjectInvitation>,
) -> Result<(), ServerFnError> {
    crate::server::friends::invitations::invite(project_id, payload).await
}

#[get("/api/v1/projects/invitations")]
pub async fn get_invitations() -> Result<Vec<ProjectInvitation>, ServerFnError> {
    crate::server::friends::invitations::get_invitations().await
}

/// The caller's own pending invitations into this project that name a participant. Only the
/// sender's: another member learns nothing about whom they invited.
#[get("/api/v1/projects/{project_id}/invitations/sent")]
pub async fn get_sent_invitations(project_id: Uuid) -> Result<Vec<SentInvitation>, ServerFnError> {
    crate::server::friends::invitations::get_sent_invitations(project_id).await
}

/// Accept and decline both end here: accepting is a client-side act (open the box, keep the key),
/// after which the row has done its job.
#[delete("/api/v1/projects/invitations/{id}")]
pub async fn delete_invitation(id: Uuid) -> Result<(), ServerFnError> {
    crate::server::friends::invitations::delete_invitation(id).await
}
