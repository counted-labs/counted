use dioxus::{fullstack::Json, prelude::*};
use shared::{RegisterPushToken, UnregisterPushToken, VerifyPushToken};

/// Idempotent: the app calls it on every boot and on every language change.
#[put("/api/v1/push/token")]
pub async fn register_push_token(
    Json(payload): Json<RegisterPushToken>,
) -> Result<(), ServerFnError> {
    crate::server::push::register_push_token(payload).await
}

/// A POST with a body rather than a DELETE: the token is too long and too opaque for a path
/// segment, and DELETE bodies are dropped by enough intermediaries to not be trusted. Idempotent —
/// the row may already be gone after the vendor reported the token dead.
#[post("/api/v1/push/unregister")]
pub async fn unregister_push_token(
    Json(payload): Json<UnregisterPushToken>,
) -> Result<(), ServerFnError> {
    crate::server::push::unregister_push_token(payload).await
}

/// The proof from the verification push `register_push_token` sent to this endpoint. Until it comes
/// back, the endpoint receives nothing else.
#[post("/api/v1/push/verify")]
pub async fn verify_push_token(
    Json(payload): Json<VerifyPushToken>,
) -> Result<(), ServerFnError> {
    crate::server::push::verify_push_token(payload).await
}
