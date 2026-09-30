//! Keeps the server's copy of this device's push token in step with the session.
//!
//! The token arrives asynchronously after the OS permission prompt, through the file the platform
//! layer writes (see `push`), so registration polls that file for a short while after each request
//! rather than waiting on a callback. Registration is idempotent server-side, which lets the effect
//! re-run on every sign-in and language change without bookkeeping beyond "what did we last send".

use api::push::push_controller::{register_push_token, unregister_push_token, verify_push_token};
use dioxus::core::Task;
use dioxus::{fullstack::Json, prelude::*};
use shared::{Account, RegisterPushToken, UnregisterPushToken, VerifyPushToken};

use super::{native_push, push::parse_push_token, push::take_push_proof, read_push_token, sleep};
use crate::i18n::current_lang;

const TOKEN_POLL_MS: u32 = 1000;
const TOKEN_POLL_ROUNDS: u32 = 30;

/// Mounted once, from `use_app_contexts`. A no-op wherever no platform push is installed.
pub fn use_push_registration() {
    let auth = use_context::<Signal<Option<Account>>>();
    let mut registered: Signal<Option<(String, String)>> = use_signal(|| None);
    let mut polling: Signal<Option<Task>> = use_signal(|| None);

    use_effect(move || {
        let signed_in = auth.read().is_some();
        let lang = current_lang();
        // One poll at a time: a language change used to leave the previous loop running, and the two
        // re-registered the token under alternating languages until both ran out.
        if let Some(previous) = polling.write().take() {
            previous.cancel();
        }
        let Some(push) = native_push() else {
            return;
        };
        if !signed_in {
            registered.set(None);
            return;
        }
        (push.request)();
        let task = spawn(async move {
            for _ in 0..TOKEN_POLL_ROUNDS {
                // The server's verification push lands here, usually a second or two after the
                // registration below; answering it is what makes the endpoint receive anything.
                if let (Some(proof), Some(raw)) = (take_push_proof(), read_push_token()) {
                    let (token, _) = parse_push_token(&raw);
                    let _ = verify_push_token(Json(VerifyPushToken { token, proof })).await;
                }
                if let Some(raw) = read_push_token() {
                    let sent = (raw.clone(), lang.clone());
                    if registered.peek().as_ref() != Some(&sent) {
                        let (token, keys) = parse_push_token(&raw);
                        let payload = RegisterPushToken {
                            platform: push.platform,
                            token,
                            lang: lang.clone(),
                            keys,
                        };
                        if register_push_token(Json(payload)).await.is_ok() {
                            registered.set(Some(sent));
                        }
                    }
                }
                sleep(TOKEN_POLL_MS).await;
            }
        });
        polling.set(Some(task));
    });
}

/// Called before `logout()`, while the session cookie still authenticates the request. Failure is
/// ignored: the token stays registered until the vendor reports it dead or the account signs in
/// again on this device, and neither leaks anything — notifications carry no account data.
pub async fn unregister_push() {
    if native_push().is_none() {
        return;
    }
    if let Some(raw) = read_push_token() {
        let (token, _) = parse_push_token(&raw);
        let _ = unregister_push_token(Json(UnregisterPushToken { token })).await;
    }
}
