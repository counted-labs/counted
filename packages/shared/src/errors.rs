//! Every error message the API can return.
//!
//! Server functions return these constants and nothing else - never a formatted string, never a
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
/// Only a client bug sends one, so it has no message of its own.
pub const DUPLICATE_BATCH_ITEM: &str = "An item appears twice in the same batch";
pub const INVALID_PAYLOAD: &str = "Invalid encrypted payload";
pub const PAYERS_REQUIRED: &str = "payers cannot be empty";
pub const DEBTORS_REQUIRED: &str = "debtors cannot be empty";
/// The amounts themselves are encrypted, so the server can only reject what it can see: the same
/// participant twice on one side, an id from another project, or an implausible row count.
pub const DUPLICATE_PARTICIPANT: &str = "A participant appears twice on the same side";
pub const PARTICIPANT_NOT_IN_PROJECT: &str = "Participant does not belong to this project";
pub const TOO_MANY_PARTICIPANTS: &str = "Too many participants for this expense";
/// A payload longer than `MAX_PAYLOAD_LENGTH`: what bounds an expense once its participants
/// are inside the ciphertext.
pub const PAYLOAD_TOO_LARGE: &str = "Encrypted payload too large";
pub const INVALID_PUBLIC_KEY: &str = "Invalid public key";
pub const SELF_FRIEND_REQUEST: &str = "You cannot add yourself as a friend";
/// The invitee is not an accepted friend of the caller, or has no keypair to box a key to.
pub const NOT_A_FRIEND: &str = "Invitations can only be sent to friends";
pub const FRIEND_HAS_NO_KEY: &str = "This friend has not updated the app yet";
/// A materialization with no occurrence, more than `MAX_OCCURRENCES_PER_CALL`, one without a
/// `client_op_id`, or one filed under another project.
pub const INVALID_OCCURRENCES: &str = "Invalid recurring occurrences";
/// `due_through` is later than tomorrow on the server's clock: the client's clock is wrong.
pub const OCCURRENCE_IN_FUTURE: &str = "Recurring occurrence is in the future";
pub const PARTICIPANT_NOT_IN_RECURRING: &str = "Participant is not part of this recurring expense";

// -------- 401 --------
pub const INVALID_CREDENTIALS: &str = "Invalid email or password";
pub const UNAUTHENTICATED: &str = "Not authenticated";

// -------- 403 --------
pub const EMAIL_NOT_VERIFIED: &str = "Email not verified";
/// An identity claim without a valid `claim_token`, i.e. without the project key. Membership itself
/// stays open to any holder of the UUID; only claiming a participant needs the key.
pub const CLAIM_PROOF_INVALID: &str = "Claiming an identity requires the project key";
pub const PROJECT_READ_ONLY: &str = "This project is read-only";
/// A project route without the claim token of a project that has a verifier, once enforced
/// (docs/plans/key-lifecycle.md §8.2): the id alone no longer opens it.
pub const PROJECT_KEY_REQUIRED: &str = "This project needs its key";
/// A route that moves an account off master-key epoch 0 before `MIN_APP_VERSION` reaches
/// `KEY_LIFECYCLE_FROM`: an older client would read blobs under the new key with the old one.
pub const KEY_LIFECYCLE_UNAVAILABLE: &str = "Not available until every app is updated";
/// An admin action (granting the role, changing the key, removing a member) from an account that
/// does not administer the project.
pub const NOT_PROJECT_ADMIN: &str = "Only the group's admins can do this";
/// A token that was this project's before its key changed (docs/plans/key-lifecycle.md §8.3): the
/// device switches through a rotation box or its escrow, or asks for the new link.
pub const PROJECT_KEY_ROTATED: &str = "The group key changed";
/// The new password of a reset that has not taken effect yet (docs/plans/key-lifecycle.md §6.3).
pub const RESET_PENDING: &str = "This password works once the reset takes effect";
/// An account deleted while a reset was younger than 30 days: disabled until it is erased.
pub const ACCOUNT_DISABLED: &str = "This account is disabled and will be erased";
/// A recovery key that matches nothing the account has, or that is too young to cancel a pending
/// reset.
pub const RECOVERY_KEY_INVALID: &str = "This recovery key does not open this account";

// -------- 404 --------
/// Sent when a project id resolves to no row. Shared so the client can recognise a deleted
/// project by value instead of matching on the driver's error text.
pub const PROJECT_NOT_FOUND: &str = "Project not found";
pub const EXPENSE_NOT_FOUND: &str = "Expense not found";
pub const USER_NOT_FOUND: &str = "User not found";
pub const FRIEND_REQUEST_NOT_FOUND: &str = "Friend request not found";
pub const INVITATION_NOT_FOUND: &str = "Invitation not found";
pub const TRICOUNT_NOT_FOUND: &str = "Tricount introuvable ou erreur API";
pub const RECURRING_NOT_FOUND: &str = "Recurring expense not found";

// -------- 409 --------
pub const TOO_MANY_MEMBERS: &str = "Too many members for this project";
pub const USER_HAS_PAYMENTS: &str =
    "User has existing payments in this project and cannot be removed";
/// A participant may back at most one account per project. Enforced by the partial unique index
/// `account_projects_identity_uniq`, never by a preceding `SELECT` - the constraint is what makes
/// two simultaneous claims impossible.
pub const IDENTITY_TAKEN: &str = "Identity already claimed by another account";
/// `PUT /auth/payment-methods` with an `expected_iv` that no longer matches the stored blob:
/// another device saved in between, and the caller must reload before writing.
pub const PAYMENT_METHODS_STALE: &str = "Payment methods were changed elsewhere";
/// `PUT /auth/preferences` with an `expected_iv` that no longer matches the stored blob.
pub const PREFERENCES_STALE: &str = "Preferences were changed elsewhere";
/// `PUT /auth/keypair/wrap` for a public key that is not the stored one.
pub const KEYPAIR_CHANGED: &str = "The account keypair is not the one presented";
/// `PUT /projects` with an `expected_iv` that no longer matches the stored payload, or a conversion
/// whose `expected_data_version` is behind.
pub const PROJECT_STALE: &str = "Project was changed elsewhere";
/// The caller's `expected_version` is behind: another member edited or materialized the rule
/// first. The caller resyncs; for a materialization, the winner's expenses come with it.
pub const RECURRING_STALE: &str = "Recurring expense was changed elsewhere";
pub const TOO_MANY_RECURRING: &str = "Too many recurring expenses for this project";
pub const USER_IN_RECURRING: &str =
    "User is part of a recurring expense in this project and cannot be removed";
/// The participant is an account's claimed identity. Deleting it would clear that claim, and the
/// delete route takes no session.
pub const USER_CLAIMED: &str = "User is claimed by an account and cannot be removed";
/// Granting the admin role to a participant no account claims: only account holders can be admins.
pub const USER_NOT_CLAIMED: &str = "This participant has no account";
/// Revoking the role from a participant whose account is not an admin.
pub const USER_NOT_ADMIN: &str = "This participant is not an admin";
/// A creator claim (D9) whose secret is not the project's, on a project that has none, or from an
/// account that holds no membership with the key.
pub const CREATOR_PROOF_INVALID: &str = "This device did not create the project";
/// A creator claim on a project that has an admin already: the proof only fills an empty role.
pub const PROJECT_HAS_ADMIN: &str = "The project has an admin already";
/// A write to a project whose key is being changed. The offline queue replays it after the switch.
pub const PROJECT_ROTATING: &str = "The group key is being changed";
/// A rotation that expired, was abandoned, or belongs to someone else.
pub const ROTATION_EXPIRED: &str = "This key change is no longer under way";
/// A commit whose re-encrypted rows do not match the project's rows.
pub const ROTATION_ROWS_CHANGED: &str = "The group changed during the key change";
/// An account-key write whose `amk_epoch` is not the account's: the device holds an older master
/// key, and the blob would be unreadable to every other device.
pub const ACCOUNT_KEY_CHANGED: &str = "The account key changed on another device";
/// A rotation that does not name exactly the account's escrow rows and friend labels: one was
/// added or removed while it was being built.
pub const ROTATION_INCOMPLETE: &str = "The account changed while its key was being rotated";
/// A password that was this account's until a reset in the last 30 days: the reset can be undone.
pub const RESET_UNDOABLE: &str = "This password was replaced by a recent reset";

// -------- 429 --------
pub const RESEND_COOLDOWN: &str = "Veuillez attendre 60 secondes avant de renvoyer l'email.";
pub const TOO_MANY_FRIEND_REQUESTS: &str = "Too many friend requests";
pub const TOO_MANY_INVITATIONS: &str = "Too many pending invitations";
/// Never shown: push registration is silent, and the app simply tries again at its next boot.
pub const TOO_MANY_PUSH_CHALLENGES: &str = "Too many push verifications";

// -------- 500 --------
/// Everything a caller must not see: sqlx errors, upstream bodies, task joins. The detail goes to
/// stderr instead - see `api::utils::fail`.
pub const INTERNAL: &str = "Something went wrong. Please try again.";
