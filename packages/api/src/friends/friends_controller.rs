use dioxus::{fullstack::Json, prelude::*};
use shared::{FriendRequestByEmail, FriendRequestFromProject, FriendsView};
use uuid::Uuid;

/// Always `200 {}`. The address lookup and the insert both run whether or not the address has an
/// account; the only difference between the two cases is a NULL bound to one column. Nothing in
/// the response, the timing or the requester's later views says which it was.
#[post("/api/v1/friends/requests")]
pub async fn request_by_email(
    Json(payload): Json<FriendRequestByEmail>,
) -> Result<(), ServerFnError> {
    crate::server::friends::request_by_email(payload).await
}

/// "Add as friend" on a claimed participant. Nothing to hide here — a claimed participant is
/// visibly an account holder to everyone in the project — but the account id still never crosses
/// the wire: the roster endpoint needs no session, so it is resolved from `(project, user_id)`
/// for a caller who holds the project.
#[post("/api/v1/friends/requests/from-project")]
pub async fn request_from_project(
    Json(payload): Json<FriendRequestFromProject>,
) -> Result<(), ServerFnError> {
    crate::server::friends::request_from_project(payload).await
}

#[get("/api/v1/friends")]
pub async fn get_friends() -> Result<FriendsView, ServerFnError> {
    crate::server::friends::get_friends().await
}

#[post("/api/v1/friends/requests/{id}/accept")]
pub async fn accept_request(id: Uuid) -> Result<(), ServerFnError> {
    crate::server::friends::accept_request(id).await
}

/// Requester: withdraws. Target: declines — the row stays, marked `rejected`, and the requester
/// keeps seeing it as pending.
#[delete("/api/v1/friends/requests/{id}")]
pub async fn delete_request(id: Uuid) -> Result<(), ServerFnError> {
    crate::server::friends::delete_request(id).await
}

#[delete("/api/v1/friends/{id}")]
pub async fn remove_friend(id: Uuid) -> Result<(), ServerFnError> {
    crate::server::friends::remove_friend(id).await
}
