use dioxus::fullstack::Json;
use dioxus::prelude::*;
use shared::{
    Account, EncryptedPair, Keypair, LoginPayload, LoginSalt, LoginSaltPayload, RegisterPayload,
    ResendVerificationPayload, UpdatePaymentMethods, VerifyEmailPayload,
};

#[post("/api/v1/auth/register")]
pub async fn register(Json(payload): Json<RegisterPayload>) -> Result<(), ServerFnError> {
    crate::server::auth::register(payload).await
}

/// The first half of a login: the salt the client derives its proof with. Served to anyone, so an
/// unknown address must be answered exactly like a known one — a fake salt that is stable across
/// calls (or the change itself would be the tell) and unpredictable (or the caller could compute
/// it and compare). `HMAC(AUTH_SALT_PEPPER, email)` is both. The version is the other half: a
/// legacy account answers `1`, so while any legacy row exists an unknown address gets `1` or `2`
/// from the same keystream and neither answer proves anything. Once every account has upgraded,
/// only `2` exists and a `1` would prove *non*-existence — so the bit is used exactly while legacy
/// rows remain, decided by the table itself rather than by a follow-up someone has to remember.
/// The DB lookups and the HMAC all run on every call, so the two branches cost the same.
#[post("/api/v1/auth/login-salt")]
pub async fn login_salt(Json(payload): Json<LoginSaltPayload>) -> Result<LoginSalt, ServerFnError> {
    crate::server::auth::login_salt(payload).await
}

#[post("/api/v1/auth/login")]
pub async fn login(Json(payload): Json<LoginPayload>) -> Result<Account, ServerFnError> {
    crate::server::auth::login(payload).await
}

#[post("/api/v1/auth/verify-email")]
pub async fn verify_email(
    Json(payload): Json<VerifyEmailPayload>,
) -> Result<Account, ServerFnError> {
    crate::server::auth::verify_email(payload).await
}

#[post("/api/v1/auth/resend-verification")]
pub async fn resend_verification(
    Json(payload): Json<ResendVerificationPayload>,
) -> Result<(), ServerFnError> {
    crate::server::auth::resend_verification(payload).await
}

#[post("/api/v1/auth/logout")]
pub async fn logout() -> Result<(), ServerFnError> {
    crate::server::auth::logout().await
}

/// RGPD art. 17. The account row and everything keyed to it go in one transaction; what the caller
/// wrote into a *shared* project stays, because those rows are the other members' ledger too — and
/// they are ciphertext whose key was derived from the password being destroyed here.
#[delete("/api/v1/auth/account")]
pub async fn delete_account() -> Result<(), ServerFnError> {
    crate::server::auth::delete_account().await
}

/// Stores the caller's UI preferences. The body is opaque ciphertext produced with the account key;
/// nothing on this side can read it, and nothing here tries.
#[put("/api/v1/auth/preferences")]
pub async fn update_preferences(Json(preferences): Json<EncryptedPair>) -> Result<(), ServerFnError> {
    crate::server::auth::update_preferences(preferences).await
}

/// Stores the caller's payment details. Same contract as `update_preferences`: the body is opaque
/// ciphertext produced with the account key, and nothing here can read the bank details inside it.
///
/// `shared` carries the subset re-encrypted under each project key, for the other members. Every
/// per-project copy the account has is replaced in the same transaction — cleared first, so a
/// client that sends none (an older build, or a device without the keys) leaves nothing stale.
/// This endpoint is the copies' only writer; `expected_iv` (see `UpdatePaymentMethods`) is what
/// stops a device holding an older list from restoring a copy a newer save withdrew.
#[put("/api/v1/auth/payment-methods")]
pub async fn update_payment_methods(
    Json(payload): Json<UpdatePaymentMethods>,
) -> Result<(), ServerFnError> {
    crate::server::auth::update_payment_methods(payload).await
}

/// Seeds the keypair of an account that predates it. Write-once on the row, so a device that lost
/// the race simply re-reads `me()` — the server never lets a second public key replace the first.
#[put("/api/v1/auth/keypair")]
pub async fn set_keypair(Json(keypair): Json<Keypair>) -> Result<(), ServerFnError> {
    crate::server::auth::set_keypair(keypair).await
}

#[get("/api/v1/auth/me")]
pub async fn me() -> Result<Option<Account>, ServerFnError> {
    crate::server::auth::me().await
}
