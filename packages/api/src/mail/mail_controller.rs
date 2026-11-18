use dioxus::{fullstack::Json, prelude::*};
use shared::UnsubscribePayload;

/// The button of the page an invitation mail's link opens. Mail providers' one-click POST goes to
/// `/api/v1/mail/unsubscribe/{token}` instead, a plain route outside the version gate
/// (docs/plans/invite-emails.md §8).
#[post("/api/v1/mail/unsubscribe")]
pub async fn unsubscribe(Json(payload): Json<UnsubscribePayload>) -> Result<(), ServerFnError> {
    crate::server::mail::unsubscribe(&payload.token).await
}
