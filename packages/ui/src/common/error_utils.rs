use dioxus::fullstack::ServerFnError;
use crate::tid;

/// Every error the API can return, mapped to the message shown for it.
///
/// `shared::errors` documents its constants as values the client matches on — they are already
/// codes in all but name, so translating them needs no wire change and no server change. Anything
/// not listed here renders `error-generic`, which also means raw server text can never reach the
/// DOM.
const KEYS: &[(&str, &str)] = &[
    (shared::errors::INVALID_EMAIL, "error-invalid-email"),
    (shared::errors::INVALID_PASSWORD, "error-invalid-password"),
    (shared::errors::PASSWORD_TOO_SHORT, "error-password-too-short"),
    (shared::errors::INVALID_KDF_SALT, "error-invalid-kdf-salt"),
    (shared::errors::INVALID_LOGIN_PROOF, "error-invalid-password"),
    (shared::errors::CLIENT_OUTDATED, "error-client-outdated"),
    (shared::errors::INVALID_LINK, "error-invalid-link"),
    (shared::errors::BATCH_TOO_LARGE, "error-batch-too-large"),
    (shared::errors::MIXED_PROJECT_BATCH, "error-mixed-project-batch"),
    (shared::errors::INVALID_PAYLOAD, "error-invalid-payload"),
    (shared::errors::PAYERS_REQUIRED, "error-payers-required"),
    (shared::errors::DEBTORS_REQUIRED, "error-debtors-required"),
    (shared::errors::DUPLICATE_PARTICIPANT, "error-duplicate-participant"),
    (shared::errors::PARTICIPANT_NOT_IN_PROJECT, "error-participant-not-in-project"),
    (shared::errors::TOO_MANY_PARTICIPANTS, "error-too-many-participants"),
    (shared::errors::INVALID_PUBLIC_KEY, "error-invalid-public-key"),
    (shared::errors::INVALID_CREDENTIALS, "error-invalid-credentials"),
    (shared::errors::UNAUTHENTICATED, "error-unauthenticated"),
    (shared::errors::EMAIL_NOT_VERIFIED, "error-email-not-verified"),
    (shared::errors::CLAIM_PROOF_INVALID, "error-claim-proof-invalid"),
    (shared::errors::PROJECT_NOT_FOUND, "error-project-not-found"),
    (shared::errors::EXPENSE_NOT_FOUND, "error-expense-not-found"),
    (shared::errors::USER_NOT_FOUND, "error-user-not-found"),
    (shared::errors::TRICOUNT_NOT_FOUND, "error-tricount-not-found"),
    (shared::errors::TOO_MANY_MEMBERS, "error-too-many-members"),
    (shared::errors::IDENTITY_TAKEN, "error-identity-taken"),
    (shared::errors::USER_HAS_PAYMENTS, "error-user-has-payments"),
    (shared::errors::PAYMENT_METHODS_STALE, "error-payment-methods-stale"),
    (shared::errors::RESEND_COOLDOWN, "error-resend-cooldown"),
    (shared::errors::SELF_FRIEND_REQUEST, "error-self-friend-request"),
    (shared::errors::NOT_A_FRIEND, "error-not-a-friend"),
    (shared::errors::FRIEND_HAS_NO_KEY, "error-friend-has-no-key"),
    (shared::errors::FRIEND_REQUEST_NOT_FOUND, "error-friend-request-not-found"),
    (shared::errors::INVITATION_NOT_FOUND, "error-invitation-not-found"),
    (shared::errors::TOO_MANY_FRIEND_REQUESTS, "error-too-many-friend-requests"),
    (shared::errors::TOO_MANY_INVITATIONS, "error-too-many-invitations"),
];

fn is_network_error(e: &str) -> bool {
    e.contains("error sending request")
        || e.contains("Failed to fetch")
        || e.contains("connection refused")
        || e.contains("timed out")
        || e.contains("network")
        || e.contains("os error")
}

/// The translation key for a server function error.
///
/// Pure, so it is unit-testable without a Dioxus runtime — `error_message` is the half that needs
/// one. Never use `e.to_string()` for either: `ServerFnError`'s `Display` wraps the backend message
/// in `error running server function: {message} (details: {details:#?})`, and the other variants
/// render English crate internals.
pub fn error_key(e: &ServerFnError) -> &'static str {
    match e {
        // Server-side reqwest failures (Tricount import) arrive here with their own network text.
        ServerFnError::ServerError { message, .. } if is_network_error(message) => "error-network",
        ServerFnError::ServerError { message, .. } => KEYS
            .iter()
            .find(|(sent, _)| sent == message)
            .map(|(_, key)| *key)
            .unwrap_or("error-generic"),
        ServerFnError::Request(_) => "error-network",
        _ => "error-generic",
    }
}

/// User-facing text for a server function error.
///
/// Safe from a spawned task as well as from a render: dioxus polls a task with its originating
/// scope on the stack, so `tid!`'s `consume_context` resolves either way.
pub fn error_message(e: &ServerFnError) -> String {
    tid!(error_key(e))
}

/// True when the server reports the project no longer exists — drives the "retirer de ma liste"
/// recovery instead of a dead-end error alert.
pub fn is_project_gone_error(e: &ServerFnError) -> bool {
    matches!(
        e,
        ServerFnError::ServerError { message, code, .. }
            if *code == 404 && message == shared::PROJECT_NOT_FOUND
    )
}

/// True when another account already claims the participant this device tried to become — drives
/// dropping the local `user_id` and reopening the "who am I?" picker.
///
/// Matches on the message **and** the 409, like `is_project_gone_error`: an internal failure that
/// happened to carry this text must not make a device forget which participant it is.
pub fn is_identity_taken_error(e: &ServerFnError) -> bool {
    matches!(
        e,
        ServerFnError::ServerError { message, code, .. }
            if *code == 409 && message == shared::errors::IDENTITY_TAKEN
    )
}

/// True when the server refused an identity claim for want of the project key. Matched by code
/// and message together, like `is_identity_taken_error`.
pub fn is_claim_proof_error(e: &ServerFnError) -> bool {
    matches!(
        e,
        ServerFnError::ServerError { message, code, .. }
            if *code == 403 && message == shared::errors::CLAIM_PROOF_INVALID
    )
}

/// True when the server refused a `user_id` because that participant is no longer in the project
/// — removed by another member since this device picked it. Code and message together, as above.
pub fn is_participant_gone_error(e: &ServerFnError) -> bool {
    matches!(
        e,
        ServerFnError::ServerError { message, code, .. }
            if *code == 400 && message == shared::errors::PARTICIPANT_NOT_IN_PROJECT
    )
}

/// True when `PUT /auth/payment-methods` found the stored blob no longer the one the caller
/// derived its write from: another device saved in between. Code and message together, as above.
pub fn is_payment_methods_stale_error(e: &ServerFnError) -> bool {
    matches!(
        e,
        ServerFnError::ServerError { message, code, .. }
            if *code == 409 && message == shared::errors::PAYMENT_METHODS_STALE
    )
}

/// True when the server refused this build outright — drives the blocking update screen.
///
/// Matched on the 426 alone, unlike the pairs above: nothing else in the stack emits that status,
/// and the gate answers from a layer *in front of* the server functions, so its body is not a
/// contract this side should depend on. The same message with a 400 is the login form telling a
/// pre-header build off, which stays a form error.
pub fn is_client_outdated_error(e: &ServerFnError) -> bool {
    matches!(e, ServerFnError::ServerError { code: 426, .. })
}

/// True when the account exists but has not confirmed its email — drives the "resend the link"
/// affordance on the login page.
///
/// Tests the error rather than the rendered message: that message is translated now, so
/// `contains(EMAIL_NOT_VERIFIED)` would only ever match in English.
pub fn is_email_not_verified(e: &ServerFnError) -> bool {
    matches!(e, ServerFnError::ServerError { message, .. } if message == shared::EMAIL_NOT_VERIFIED)
}

/// True when the request never reached the server — drives the offline cache fallbacks.
///
/// Tests the error rather than the rendered message, for the same reason as above.
pub fn is_offline_error(e: &ServerFnError) -> bool {
    matches!(e, ServerFnError::Request(_)) || is_network_error(&e.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use dioxus::fullstack::RequestError;

    #[test]
    fn test_server_error_maps_to_the_key_for_the_message_it_sent() {
        assert_eq!(error_key(&ServerFnError::new(shared::errors::INVALID_CREDENTIALS)), "error-invalid-credentials");
        assert_eq!(error_key(&ServerFnError::new(shared::PROJECT_NOT_FOUND)), "error-project-not-found");
    }

    /// The server's own text must never be shown. An unrecognised message is a message the client
    /// has no translation for, which also means it may be text this client never vetted.
    #[test]
    fn test_unrecognised_server_message_is_generic() {
        assert_eq!(error_key(&ServerFnError::new("relation \"users\" does not exist")), "error-generic");
    }

    #[test]
    fn test_server_error_carrying_network_text_is_humanized() {
        let e = ServerFnError::new("error sending request: connection reset");
        assert_eq!(error_key(&e), "error-network");
    }

    #[test]
    fn test_email_not_verified_is_recognised_by_the_error_not_the_text() {
        let e = ServerFnError::new(shared::EMAIL_NOT_VERIFIED);
        assert!(is_email_not_verified(&e));
        assert_eq!(error_key(&e), "error-email-not-verified");
        assert!(!is_email_not_verified(&ServerFnError::new(shared::errors::INVALID_CREDENTIALS)));
    }

    #[test]
    fn test_request_error_is_network_message() {
        let e = ServerFnError::Request(RequestError::Timeout("after 30s".into()));
        assert_eq!(error_key(&e), "error-network");
        assert!(is_offline_error(&e));
    }

    #[test]
    fn test_deserialization_error_is_generic() {
        let e = ServerFnError::Deserialization("expected `,`".into());
        assert_eq!(error_key(&e), "error-generic");
        assert!(!is_offline_error(&e));
    }

    #[test]
    fn test_plain_server_error_is_not_offline() {
        assert!(!is_offline_error(&ServerFnError::new(shared::errors::INVALID_CREDENTIALS)));
    }

    fn with_code(message: &str, code: u16) -> ServerFnError {
        ServerFnError::ServerError { message: message.into(), code, details: None }
    }

    // `ServerFnError::new` is a 500. Matching on the message alone would let an internal failure
    // that happens to carry this text trigger the "project deleted" recovery.
    #[test]
    fn test_project_gone_requires_both_the_message_and_a_404() {
        assert!(is_project_gone_error(&with_code(shared::PROJECT_NOT_FOUND, 404)));
        assert!(!is_project_gone_error(&with_code(shared::PROJECT_NOT_FOUND, 500)));
        assert!(!is_project_gone_error(&with_code("Expense not found", 404)));
    }

    /// Same reasoning as above: this one makes a device forget which participant it is, so a 500
    /// that happens to carry the text must not trigger it.
    #[test]
    fn test_identity_taken_requires_both_the_message_and_a_409() {
        let taken = shared::errors::IDENTITY_TAKEN;
        assert!(is_identity_taken_error(&with_code(taken, 409)));
        assert!(!is_identity_taken_error(&with_code(taken, 500)));
        assert!(!is_identity_taken_error(&with_code(shared::errors::TOO_MANY_MEMBERS, 409)));
        assert_eq!(error_key(&with_code(taken, 409)), "error-identity-taken");
    }

    #[test]
    fn test_claim_proof_requires_both_the_message_and_a_403() {
        let refused = shared::errors::CLAIM_PROOF_INVALID;
        assert!(is_claim_proof_error(&with_code(refused, 403)));
        assert!(!is_claim_proof_error(&with_code(refused, 400)));
        assert!(!is_claim_proof_error(&with_code(shared::errors::EMAIL_NOT_VERIFIED, 403)));
        assert_eq!(error_key(&with_code(refused, 403)), "error-claim-proof-invalid");
    }

    #[test]
    fn test_client_outdated_is_the_426_alone() {
        let outdated = shared::errors::CLIENT_OUTDATED;
        assert!(is_client_outdated_error(&with_code(outdated, 426)));
        assert!(is_client_outdated_error(&with_code("whatever the gate says", 426)));
        assert!(!is_client_outdated_error(&with_code(outdated, 400)));
        assert!(!is_offline_error(&with_code(outdated, 426)));
        assert_eq!(error_key(&with_code(outdated, 426)), "error-client-outdated");
    }

    /// `error_message` calls `tid!` on a value, not a literal, so the crate-wide scanner in `i18n`
    /// does not see these. A typo here would surface as a raw key inside an error alert.
    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn every_error_key_is_a_known_message() {
        let known = crate::i18n::locale_ids(crate::i18n::FALLBACK);
        for key in KEYS.iter().map(|(_, k)| *k).chain(["error-network", "error-generic"]) {
            assert!(known.contains(key), "{key} is not defined in the fallback locale");
        }
    }

    /// Two constants sharing a message would make the mapping order-dependent and silently show
    /// the wrong one.
    #[test]
    fn no_two_errors_send_the_same_message() {
        for (i, (sent, _)) in KEYS.iter().enumerate() {
            assert!(
                !KEYS[i + 1..].iter().any(|(other, _)| other == sent),
                "{sent} is mapped twice"
            );
        }
    }
}
