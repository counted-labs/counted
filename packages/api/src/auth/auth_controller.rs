use dioxus::fullstack::Json;
use dioxus::prelude::*;
use shared::{
    Account, ChangePassword, DeleteAccount, Keypair, RotationSet, LoginPayload, LoginSalt, LoginSaltPayload, RegisterPayload,
    ResendVerificationPayload, UpdatePaymentMethods, UpdatePreferences, VerifyEmailPayload,
    VerifyEmailSaltPayload,
};

#[post("/api/v1/auth/register")]
pub async fn register(Json(payload): Json<RegisterPayload>) -> Result<(), ServerFnError> {
    crate::server::auth::register(payload).await
}

/// The first half of a login: the salt the client derives its proof with. Served to anyone, so an
/// unknown address must be answered exactly like a known one - a fake salt that is stable across
/// calls (or the change itself would be the tell) and unpredictable (or the caller could compute
/// it and compare). `HMAC(AUTH_SALT_PEPPER, email)` is both. The version is the other half: a
/// legacy account answers `1`, so while any legacy row exists an unknown address gets `1` or `2`
/// from the same keystream and neither answer proves anything. Once every account has upgraded,
/// only `2` exists and a `1` would prove *non*-existence - so the bit is used exactly while legacy
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

/// The login salt of the token's account, without consuming the token. The holder of a link owns
/// the mailbox it was sent to, so this tells them nothing they could not already learn.
#[post("/api/v1/auth/verify-email/salt")]
pub async fn verify_email_salt(
    Json(payload): Json<VerifyEmailSaltPayload>,
) -> Result<LoginSalt, ServerFnError> {
    crate::server::auth::verify_email_salt(payload).await
}

/// A login that also consumes the token. A request without a proof is refused as outdated.
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
/// wrote into a *shared* project stays, because those rows are the other members' ledger too - and
/// they are ciphertext whose key was derived from the password being destroyed here.
///
/// The session-only form, for clients below `KEY_LIFECYCLE_FROM`: once the floor is reached it
/// answers 400 `CLIENT_OUTDATED` and only the route below deletes (security review 2026-11-02 L3).
#[delete("/api/v1/auth/account")]
pub async fn delete_account() -> Result<(), ServerFnError> {
    crate::server::auth::delete_account().await
}

/// The same deletion, proven with the password like a password change (401 `INVALID_CREDENTIALS`
/// on a wrong proof, counted like a failed login), so that a session alone cannot erase an
/// account's only copies of its keys.
#[post("/api/v1/auth/account/delete")]
pub async fn delete_account_with_proof(Json(payload): Json<DeleteAccount>) -> Result<(), ServerFnError> {
    crate::server::auth::delete_account_with_proof(payload).await
}

/// Stores the caller's UI preferences. The body is opaque ciphertext produced with the account key;
/// nothing on this side can read it, and nothing here tries.
#[put("/api/v1/auth/preferences")]
pub async fn update_preferences(Json(preferences): Json<UpdatePreferences>) -> Result<(), ServerFnError> {
    crate::server::auth::update_preferences(preferences).await
}

/// Stores the caller's payment details. Same contract as `update_preferences`: the body is opaque
/// ciphertext produced with the account key, and nothing here can read the bank details inside it.
///
/// `shared` carries the subset re-encrypted under each project key, for the other members. Every
/// per-project copy the account has is replaced in the same transaction - cleared first, so a
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
/// the race simply re-reads `me()` - the server never lets a second public key replace the first.
#[put("/api/v1/auth/keypair")]
pub async fn set_keypair(Json(keypair): Json<Keypair>) -> Result<(), ServerFnError> {
    crate::server::auth::set_keypair(keypair).await
}

/// Seeds the account's ML-KEM-768 key, the post-quantum half of its hybrid keypair. Write-once.
#[put("/api/v1/auth/kem-key")]
pub async fn set_kem_key(Json(keypair): Json<shared::KemKeypair>) -> Result<(), ServerFnError> {
    crate::server::auth::set_kem_key(keypair).await
}

/// The same private key, re-wrapped typed. Refused (409) unless `public_key` is the stored one.
#[put("/api/v1/auth/keypair/wrap")]
pub async fn rewrap_private_key(Json(keypair): Json<Keypair>) -> Result<(), ServerFnError> {
    crate::server::auth::rewrap_private_key(keypair).await
}

/// Docs/plans/key-lifecycle.md §4: the current password's proof, the new one with the same
/// `login_salt`, and the master key under the new password-derived key. At epoch 0 every account
/// blob is re-encrypted under a new random master key, and what it supersedes is retired. Every
/// other session is deleted.
#[put("/api/v1/auth/password")]
pub async fn change_password(Json(payload): Json<ChangePassword>) -> Result<(), ServerFnError> {
    crate::server::auth::change_password(payload).await
}

/// The account's escrow rows and friend labels as stored, which a rotation must name exactly.
#[get("/api/v1/auth/rotation-set")]
pub async fn rotation_set() -> Result<RotationSet, ServerFnError> {
    crate::server::auth::rotation_set().await
}

/// Docs/plans/key-lifecycle.md §6.1. A first recovery key takes effect at once; a replacement 72
/// hours later, during which the current key cancels it.
#[put("/api/v1/auth/recovery-key")]
pub async fn set_recovery_key(Json(payload): Json<shared::SetRecoveryKey>) -> Result<(), ServerFnError> {
    crate::server::auth::recovery::set_recovery_key(payload).await
}

#[post("/api/v1/auth/recovery-key/cancel")]
pub async fn cancel_recovery_replacement(Json(payload): Json<shared::RecoveryAuth>) -> Result<(), ServerFnError> {
    crate::server::auth::recovery::cancel_recovery_replacement(payload).await
}

/// "Forgot password": the same answer for every address; a link only for a verified account.
#[post("/api/v1/auth/forgot-password")]
pub async fn forgot_password(Json(payload): Json<shared::ForgotPassword>) -> Result<(), ServerFnError> {
    crate::server::auth::recovery::forgot_password(payload).await
}

/// What the holder of a reset link needs to derive the recovery secrets and the new proof.
#[post("/api/v1/auth/recovery/start")]
pub async fn recovery_start(Json(payload): Json<shared::ResetToken>) -> Result<shared::RecoveryStart, ServerFnError> {
    crate::server::auth::recovery::recovery_start(payload).await
}

/// The link and the recovery key's auth secret, both checked, then the recovery wrap.
#[post("/api/v1/auth/recovery/wrap")]
pub async fn recovery_wrap(Json(payload): Json<shared::RecoveryWrapRequest>) -> Result<shared::RecoveryWrap, ServerFnError> {
    crate::server::auth::recovery::recovery_wrap(payload).await
}

#[post("/api/v1/auth/recovery/complete")]
pub async fn complete_recovery(Json(payload): Json<shared::CompleteRecovery>) -> Result<Account, ServerFnError> {
    crate::server::auth::recovery::complete_recovery(payload).await
}

/// A reset without the recovery key, in effect 72 hours later. Returns when.
#[post("/api/v1/auth/reset/request")]
pub async fn request_reset(Json(payload): Json<shared::RequestReset>) -> Result<chrono::NaiveDateTime, ServerFnError> {
    crate::server::auth::recovery::request_reset(payload).await
}

#[post("/api/v1/auth/reset/cancel")]
pub async fn cancel_reset(Json(payload): Json<shared::CancelReset>) -> Result<(), ServerFnError> {
    crate::server::auth::recovery::cancel_reset(payload).await
}

/// What a reset retired, after the proof of the password it replaced.
#[post("/api/v1/auth/retired")]
pub async fn retired_generation(Json(payload): Json<shared::RetiredProof>) -> Result<shared::RetiredGeneration, ServerFnError> {
    crate::server::auth::recovery::retired_generation(payload).await
}

#[post("/api/v1/auth/reset/undo")]
pub async fn undo_reset(Json(payload): Json<shared::UndoReset>) -> Result<Account, ServerFnError> {
    crate::server::auth::recovery::undo_reset(payload).await
}

#[get("/api/v1/auth/me")]
pub async fn me() -> Result<Option<Account>, ServerFnError> {
    crate::server::auth::me().await
}
