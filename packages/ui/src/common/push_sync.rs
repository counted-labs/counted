//! Keeps the server's copy of this device's push token in step with the session.
//!
//! The token arrives asynchronously after the OS permission prompt, through the file the platform
//! layer writes (see `push`), so registration polls that file for a short while after each request
//! rather than waiting on a callback. Registration is idempotent server-side, which lets the effect
//! re-run on every sign-in and language change without bookkeeping beyond "what did we last send".

use api::push::push_controller::{register_push_token, unregister_push_token};
use dioxus::{fullstack::Json, prelude::*};
use shared::{Account, RegisterPushToken, UnregisterPushToken};

use super::{native_push, read_push_token, sleep};
use crate::i18n::current_lang;

const TOKEN_POLL_MS: u32 = 1000;
const TOKEN_POLL_ROUNDS: u32 = 30;

/// Mounted once, from `use_app_contexts`. A no-op wherever no platform push is installed.
pub fn use_push_registration() {
    let auth = use_context::<Signal<Option<Account>>>();
    let mut registered: Signal<Option<(String, String)>> = use_signal(|| None);

    use_effect(move || {
        let signed_in = auth.read().is_some();
        let lang = current_lang();
        let Some(push) = native_push() else {
            return;
        };
        if !signed_in {
            registered.set(None);
            return;
        }
        (push.request)();
        spawn(async move {
            for _ in 0..TOKEN_POLL_ROUNDS {
                if let Some(token) = read_push_token() {
                    let sent = (token.clone(), lang.clone());
                    if registered.peek().as_ref() != Some(&sent) {
                        let payload = RegisterPushToken {
                            platform: push.platform,
                            token,
                            lang: lang.clone(),
                        };
                        if register_push_token(Json(payload)).await.is_ok() {
                            registered.set(Some(sent));
                        }
                    }
                }
                sleep(TOKEN_POLL_MS).await;
            }
        });
    });
}

/// Called before `logout()`, while the session cookie still authenticates the request. Failure is
/// ignored: the token stays registered until the vendor reports it dead or the account signs in
/// again on this device, and neither leaks anything — notifications carry no account data.
pub async fn unregister_push() {
    if native_push().is_none() {
        return;
    }
    if let Some(token) = read_push_token() {
        let _ = unregister_push_token(Json(UnregisterPushToken { token })).await;
    }
}
