//! Every error message the API can return.
//!
//! Server functions return these constants and nothing else — never a formatted string, never a
//! driver or upstream error. The client matches on the value (see `ui::common::error_utils`), so an
//! inline literal on either side silently breaks that match.
//!
//! Internal detail is logged server-side and replaced by [`INTERNAL`] on the wire.

// -------- 400 --------
pub const INVALID_EMAIL: &str = "Invalid email";
/// Client-side only since the login proof replaced the password on the wire: the server cannot
/// see a length it is never sent.
pub const INVALID_PASSWORD: &str = "Invalid password";
pub const PASSWORD_TOO_SHORT: &str = "Password must be at least 8 characters";
pub const INVALID_KDF_SALT: &str = "Invalid kdf_salt";
pub const INVALID_LOGIN_PROOF: &str = "Invalid login proof";
/// A build predating the login proof sent a password instead. Nothing was read from it.
pub const CLIENT_OUTDATED: &str = "This version of the app is outdated. Please update it.";
pub const INVALID_LINK: &str = "Invalid link";
pub const BATCH_TOO_LARGE: &str = "Batch size exceeds the maximum";
pub const MIXED_PROJECT_BATCH: &str = "All participants in a batch must belong to the same project";
pub const INVALID_PAYLOAD: &str = "Invalid encrypted payload";
pub const PAYERS_REQUIRED: &str = "payers cannot be empty";
pub const DEBTORS_REQUIRED: &str = "debtors cannot be empty";
/// The amounts themselves are encrypted, so the server can only reject what it can see: the same
/// participant twice on one side, an id from another project, or an implausible row count.
pub const DUPLICATE_PARTICIPANT: &str = "A participant appears twice on the same side";
pub const PARTICIPANT_NOT_IN_PROJECT: &str = "Participant does not belong to this project";
pub const TOO_MANY_PARTICIPANTS: &str = "Too many participants for this expense";
pub const INVALID_PUBLIC_KEY: &str = "Invalid public key";
pub const SELF_FRIEND_REQUEST: &str = "You cannot add yourself as a friend";
/// The invitee is not an accepted friend of the caller, or has no keypair to box a key to.
pub const NOT_A_FRIEND: &str = "Invitations can only be sent to friends";
pub const FRIEND_HAS_NO_KEY: &str = "This friend has not updated the app yet";

// -------- 401 --------
pub const INVALID_CREDENTIALS: &str = "Invalid email or password";
pub const UNAUTHENTICATED: &str = "Not authenticated";

// -------- 403 --------
pub const EMAIL_NOT_VERIFIED: &str = "Email not verified";
/// An identity claim without a valid `claim_token`, i.e. without the project key. Membership itself
/// stays open to any holder of the UUID; only claiming a participant needs the key.
pub const CLAIM_PROOF_INVALID: &str = "Claiming an identity requires the project key";

// -------- 404 --------
/// Sent when a project id resolves to no row. Shared so the client can recognise a deleted
/// project by value instead of matching on the driver's error text.
pub const PROJECT_NOT_FOUND: &str = "Project not found";
pub const EXPENSE_NOT_FOUND: &str = "Expense not found";
pub const USER_NOT_FOUND: &str = "User not found";
pub const FRIEND_REQUEST_NOT_FOUND: &str = "Friend request not found";
pub const INVITATION_NOT_FOUND: &str = "Invitation not found";
pub const TRICOUNT_NOT_FOUND: &str = "Tricount introuvable ou erreur API";

// -------- 409 --------
pub const TOO_MANY_MEMBERS: &str = "Too many members for this project";
pub const USER_HAS_PAYMENTS: &str =
    "User has existing payments in this project and cannot be removed";
/// A participant may back at most one account per project. Enforced by the partial unique index
/// `account_projects_identity_uniq`, never by a preceding `SELECT` — the constraint is what makes
/// two simultaneous claims impossible.
pub const IDENTITY_TAKEN: &str = "Identity already claimed by another account";
/// `PUT /auth/payment-methods` with an `expected_iv` that no longer matches the stored blob:
/// another device saved in between, and the caller must reload before writing.
pub const PAYMENT_METHODS_STALE: &str = "Payment methods were changed elsewhere";

// -------- 429 --------
pub const RESEND_COOLDOWN: &str = "Veuillez attendre 60 secondes avant de renvoyer l'email.";
pub const TOO_MANY_FRIEND_REQUESTS: &str = "Too many friend requests";
pub const TOO_MANY_INVITATIONS: &str = "Too many pending invitations";
/// Never shown: push registration is silent, and the app simply tries again at its next boot.
pub const TOO_MANY_PUSH_CHALLENGES: &str = "Too many push verifications";

// -------- 500 --------
/// Everything a caller must not see: sqlx errors, upstream bodies, task joins. The detail goes to
/// stderr instead — see `api::utils::fail`.
pub const INTERNAL: &str = "Something went wrong. Please try again.";
