use dioxus::fullstack::Json;
use dioxus::prelude::*;

use super::tricount_models::{TricountImportRequest, TricountRegistry};

#[post("/api/v1/fetch/tricount")]
pub async fn fetch_tricount_registry(
    Json(payload): Json<TricountImportRequest>,
) -> Result<TricountRegistry, ServerFnError> {
    crate::server::tricount::fetch_tricount_registry(payload).await
}
