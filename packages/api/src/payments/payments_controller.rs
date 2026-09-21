use dioxus::prelude::*;
use shared::{Payment, ProjectSync};
use uuid::Uuid;

#[get("/api/v1/projects/{project_id}/expenses/{expense_id}/payments")]
pub async fn get_payments_by_expense_id(
    project_id: Uuid,
    expense_id: i32,
) -> Result<Vec<Payment>, ServerFnError> {
    crate::server::payments::get_payments_by_expense_id(project_id, expense_id).await
}

#[get("/api/v1/projects/{project_id}/payments")]
pub async fn get_payments_by_project_id(project_id: Uuid) -> Result<Vec<Payment>, ServerFnError> {
    crate::server::payments::get_payments_by_project_id(project_id).await
}

/// The project page's only request. Clients with nothing cached send `NO_CACHED_VERSION`.
///
/// One `REPEATABLE READ` transaction for all four reads, so a write landing mid-request lands
/// before every read or after every one — that is what keeps the version consistent with the rows
/// it answers about. Under the default `READ COMMITTED` each statement takes its own snapshot,
/// and the code was only correct because the version happened to be read first: reordering the
/// reads would have let a client cache version N+1 against N's rows and be told "unchanged"
/// forever. Why only the rows are conditional: `docs/plans/expenses-tab-performance.md` §5.
#[get("/api/v1/projects/{project_id}/sync/{known_version}")]
pub async fn sync_project_data(
    project_id: Uuid,
    known_version: i64,
) -> Result<ProjectSync, ServerFnError> {
    crate::server::payments::sync_project_data(project_id, known_version).await
}
