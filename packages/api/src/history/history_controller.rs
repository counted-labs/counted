use dioxus::prelude::*;
use shared::HistoryEntry;
use uuid::Uuid;

/// Open to any holder of the project UUID, like every other read here — the capability *is* the
/// project id, and gating this one endpoint would protect nothing while locking anonymous
/// share-link holders out of their own project's history. What the entries carry is bounded
/// instead: the payload is E2EE, `actor_account_id` never reaches the wire, the actor is validated
/// against the project on write, and the read is capped.
#[get("/api/v1/projects/{project_id}/history")]
pub async fn get_project_history(project_id: Uuid) -> Result<Vec<HistoryEntry>, ServerFnError> {
    crate::server::history::get_project_history(project_id).await
}
