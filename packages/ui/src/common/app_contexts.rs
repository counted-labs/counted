use api::auth::auth_controller::me;
use dioxus::prelude::*;
use shared::Account;
use std::collections::VecDeque;

use super::{is_client_outdated_error, read_from_ls, read_queue, LocalStorageState, QueuedOp};
use crate::crypto::key_from_fragment;
use crate::i18n::{set_language, use_init_locale};

/// A one-off message raised from anywhere and rendered once, by `AppLayout`. It carries its own
/// severity: a failure raised through here has to read as a failure, not as a green confirmation.
#[derive(Clone, PartialEq)]
pub struct Flash {
    pub msg: String,
    pub success: bool,
}

impl Flash {
    pub fn ok(msg: impl Into<String>) -> Self {
        Self { msg: msg.into(), success: true }
    }

    pub fn err(msg: impl Into<String>) -> Self {
        Self { msg: msg.into(), success: false }
    }
}

/// Whether `me()` has come back yet, whatever it said.
///
/// `Signal<Option<Account>>` is `None` for an anonymous user *and* while the call is in flight, so
/// a page that renders one thing for each needs this to tell them apart — without it the settings
/// page flashes its sign-in card at someone who is signed in. A newtype rather than a
/// `Signal<bool>`: contexts are keyed by type and that one is already taken by `is_online`.
#[derive(Clone, Copy, PartialEq)]
pub struct AuthResolved(pub bool);

/// The server answered the boot-time `me()` with a 426: this build is below its floor. Only a
/// native app can end up here (web sends no version), and `UpdateRequiredScreen` covers the whole
/// app while it is true. Newtype for the same reason as [`AuthResolved`].
#[derive(Clone, Copy, PartialEq)]
pub struct UpdateRequired(pub bool);

/// The current project's E2EE key. A newtype for the same reason as [`AuthResolved`]: bare
/// `Signal<Option<[u8; 32]>>` is the *account* key, so without it, providing the project key above
/// the login or settings pages silently shadows the account key.
#[derive(Clone, Copy)]
pub struct ProjectKey(pub Signal<Option<[u8; 32]>>);

/// Reconciles this device's language with the account's, once `me()` has answered.
///
/// **The account wins on load; any pick pushes up.** No timestamps, no merge — the account's value
/// is the one the user last chose *as a user*, and adopting it is what makes a new device speak the
/// right language after login. When the account has none yet, this device's choice seeds it, which
/// is how an existing account gets its first blob.
///
/// Compares against the stored choice rather than `current_lang()`: the latter subscribes to the
/// locale signal, and this runs inside the effect that calls `me()` — which would then re-fetch the
/// account on every language change.
fn adopt_account_language(account: &Account, key: [u8; 32]) {
    let stored = read_from_ls().language;
    match crate::preferences::account_language(account, &key) {
        Some(code) => {
            if stored.as_deref() != Some(code.as_str()) {
                set_language(&code);
            }
        }
        None => {
            if let Some(local) = stored {
                crate::preferences::push_language(key, local);
            }
        }
    }
}

/// Registers every global context shared by all entry points (web, mobile).
/// Call this at the top of the root `app()` component before any other hook.
/// Each entry point then registers only its own platform-specific contexts
/// (e.g. NativeClipboardReader).
pub fn use_app_contexts() {
    // First, and before any conditional hook: `use_init_locale` calls `use_server_cached`, which
    // matches the server's hydration payload by hook order.
    let initial_lang = use_init_locale();

    let ls_state = read_from_ls();
    // The account key is derived from the password at login and persisted; without it a
    // reloaded session cannot decrypt the display name and falls back to the email.
    let restored_key = ls_state
        .account_key
        .as_deref()
        .and_then(|k| key_from_fragment(k).ok());
    let stored_lang = ls_state.language.clone();

    let auth: Signal<Option<Account>> = use_context_provider(|| Signal::new(None));
    let auth_resolved: Signal<AuthResolved> =
        use_context_provider(|| Signal::new(AuthResolved(false)));
    let update_required: Signal<UpdateRequired> =
        use_context_provider(|| Signal::new(UpdateRequired(false)));
    let account_enc_key: Signal<Option<[u8; 32]>> =
        use_context_provider(|| Signal::new(restored_key));
    let _flash: Signal<Option<Flash>> = use_context_provider(|| Signal::new(None));
    let _ls: Signal<LocalStorageState> = use_context_provider(|| Signal::new(ls_state));
    // No Signal<bool> for is_mobile: it would collide with is_online (contexts are keyed by type).
    // Always true on web; updated by DOM events on mobile native.
    // Mutation queue gate won't activate on web — offline writes fail with network errors.
    let is_online: Signal<bool> = use_context_provider(|| Signal::new(true));
    // Offline mutation queue — persisted to localStorage/file; drained on reconnect (mobile only).
    let _pending_ops: Signal<VecDeque<QueuedOp>> =
        use_context_provider(|| Signal::new(read_queue()));
    // Bumped after successful queue replay so child page resources re-execute.
    let _resource_version: Signal<u64> = use_context_provider(|| Signal::new(0u64));

    // Language, post-mount. Comparing captured values rather than reading the locale signal keeps
    // this effect from re-running on its own write.
    use_effect(move || {
        // An explicit choice wins over anything inferred. On web the server already applied it from
        // the cookie, so this only fires when the cookie was cleared — one repaint, same shape as
        // `onboarding_seen`.
        if let Some(code) = stored_lang.as_deref() {
            if code != initial_lang {
                set_language(code);
            }
            return;
        }
        // Nothing chosen. On web the server inferred from `Accept-Language` and the first render is
        // already right. Mobile and desktop had no request to infer from, so ask the WebView for the
        // device language — and do not persist it, so it keeps following the device until the user
        // picks something. `device_lang` returns None for a language we do not ship, which leaves
        // the English fallback in place.
        #[cfg(not(target_arch = "wasm32"))]
        {
            let already = initial_lang.clone();
            spawn(async move {
                if let Some(code) = crate::i18n::device_lang().await {
                    // Re-read rather than trusting the check above: `device_lang` awaits the
                    // WebView, and `me()` can land first and adopt the account's language — which
                    // `set_language` records here. Without this, signing in on a phone whose
                    // system language differs would lose the account's choice to a race.
                    if read_from_ls().language.is_none() && code != already {
                        crate::i18n::apply_inferred_language(code);
                    }
                }
            });
        }
    });

    use_effect(move || {
        let mut auth = auth;
        let mut auth_resolved = auth_resolved;
        let mut update_required = update_required;
        spawn(async move {
            match me().await {
                Ok(account) => {
                    let account = match (account, account_enc_key()) {
                        (Some(a), Some(key)) => {
                            adopt_account_language(&a, key);
                            Some(crate::auth::credentials::ensure_keypair(a, &key).await)
                        }
                        (account, _) => account,
                    };
                    auth.set(account);
                }
                // Only the explicit refusal; offline and server failures leave the app usable.
                Err(e) if is_client_outdated_error(&e) => update_required.set(UpdateRequired(true)),
                Err(_) => {}
            }
            // Set even when the call failed: the page has learned everything it is going to, and
            // leaving it false forever would leave a skeleton spinning behind an offline device.
            auth_resolved.set(AuthResolved(true));
        });
    });

    #[cfg(any(target_os = "android", target_os = "ios"))]
    use_effect(move || {
        let mut is_online = is_online;
        spawn(async move {
            let mut eval = document::eval(
                "window.addEventListener('online',  () => dioxus.send('online'));
                 window.addEventListener('offline', () => dioxus.send('offline'));",
            );
            loop {
                match eval.recv::<String>().await {
                    Ok(msg) => is_online.set(msg == "online"),
                    Err(_) => break,
                }
            }
        });
    });
    #[cfg(not(any(target_os = "android", target_os = "ios")))]
    let _ = is_online;

    super::use_enter_advances_focus();
    super::use_push_registration();
}
