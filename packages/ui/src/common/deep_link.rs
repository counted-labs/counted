//! Deep links from the OS (App/Universal Links and `counted://`). See docs/deep-links.md.
//!
//! The platform half — Android intent over JNI, tao events before the UI exists — is in
//! `packages/mobile/src/main.rs`. [`DeepLinkListener`] is the navigation half, and must render
//! *inside* the `Router`: `use_navigator` walks ancestors, so the Router's own parent cannot see
//! it. `AppLayout` renders it, like `NavigationSync`.

use dioxus::prelude::*;

/// Consumes the URL the OS started the activity with. Android only — iOS gets URLs from tao
/// directly. Provided per entry point like `NativeClipboardReader`, since the JNI call lives in
/// the platform crate.
///
/// A newtype, not a bare `Arc<dyn Fn…>`: that alias is structurally identical to
/// `NativeClipboardReader` and Dioxus contexts are keyed by type, so the two would share a slot
/// and whichever was provided second would silently win.
#[derive(Clone)]
pub struct NativeDeepLinkReader(pub std::sync::Arc<dyn Fn() -> Option<String> + Send + Sync>);

#[cfg(any(target_os = "android", target_os = "ios"))]
static PENDING_DEEP_LINKS: std::sync::Mutex<Vec<String>> = std::sync::Mutex::new(Vec::new());

/// True once [`DeepLinkListener`] is mounted and handling tao events itself.
///
/// A handoff, not a guard: `dioxus_desktop::launch` dispatches registered wry handlers
/// (`App::tick`) *before* the config's `custom_event_handler`, so both see the same event, listener
/// first. Before mount only `main()`'s handler exists and buffers; after mount it stands down.
#[cfg(any(target_os = "android", target_os = "ios"))]
static LISTENER_ACTIVE: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

/// Whether the mounted listener has taken over event handling. Called by the platform crate.
#[cfg(any(target_os = "android", target_os = "ios"))]
pub fn deep_link_listener_active() -> bool {
    LISTENER_ACTIVE.load(std::sync::atomic::Ordering::Relaxed)
}

/// Records a URL handed to the app by the OS. Safe to call before the UI exists.
#[cfg(any(target_os = "android", target_os = "ios"))]
pub fn push_deep_link(url: String) {
    tracing::info!("deep link received: {}", without_fragment(&url));
    if let Ok(mut q) = PENDING_DEEP_LINKS.lock() {
        q.push(url);
    }
}

/// Everything before the `#`. **Never log a share link whole**: the fragment is the project's
/// unrotatable E2EE key (docs/e2ee.md), and logcat / os_log is readable by a bug report, anyone
/// holding the device, and on older Android any app with log access. The URL carries the project
/// id, so logging both halves hands over the whole capability.
///
/// Not behind the mobile `cfg` its call sites are: gated, only an android/iOS bundle would compile
/// it and the tests below could never run.
#[cfg_attr(not(any(target_os = "android", target_os = "ios")), allow(dead_code))]
fn without_fragment(url: &str) -> &str {
    url.split('#').next().unwrap_or("")
}

#[cfg(any(target_os = "android", target_os = "ios"))]
fn take_deep_links() -> Vec<String> {
    PENDING_DEEP_LINKS.lock().map(|mut q| std::mem::take(&mut *q)).unwrap_or_default()
}

/// Navigates to any deep link the OS handed the app. Drains at two points, since a link can arrive
/// on either side of this component's lifetime: on mount (cold start, buffered by `main()`) and
/// from its own tao subscription thereafter.
#[cfg(any(target_os = "android", target_os = "ios"))]
#[component]
pub fn DeepLinkListener() -> Element {
    use crate::common::{
        parse_share_link, update_ls, upsert_project, upsert_project_key, LocalStorageState,
    };
    use crate::crypto::key_to_fragment;
    use crate::route::Route;
    use std::sync::atomic::Ordering;

    let nav = use_navigator();
    let ls_ctx = use_context::<Signal<LocalStorageState>>();
    let reader = use_context::<Option<NativeDeepLinkReader>>();

    let drain = use_callback(move |_: ()| {
        for url in take_deep_links() {
            let Some((project_id, key)) = parse_share_link(&url) else {
                // The Android app-link filter claims every path on counted.fr, so a non-project URL
                // is expected — leave the user put. Still stripped: a rejected URL can carry a
                // fragment anyway.
                tracing::info!(
                    "ignoring a deep link that is not a project link: {}",
                    without_fragment(&url)
                );
                continue;
            };
            // The key must be in local storage before navigating: mobile runs on MemoryHistory, so
            // ExpensesPage has no URL fragment to recover it from.
            update_ls(ls_ctx, |state| {
                upsert_project(state, project_id, None);
                upsert_project_key(state, project_id, key_to_fragment(&key));
            });
            nav.push(Route::ExpensesPage { project_id });
        }
    });

    use_effect(move || {
        drain.call(());
        LISTENER_ACTIVE.store(true, Ordering::Relaxed);
    });

    dioxus::mobile::use_wry_event_handler(move |event, _| {
        use dioxus::mobile::tao::event::Event;
        match event {
            // iOS delivers both counted:// (application:openURL:) and universal links
            // (application:continueUserActivity:) here — see tao's ios/view.rs.
            Event::Opened { urls } => {
                for url in urls {
                    push_deep_link(url.to_string());
                }
                drain.call(());
            }
            // Android has no equivalent event, so the intent is read on every resume.
            Event::Resumed => {
                if let Some(read) = reader.as_ref() {
                    if let Some(url) = (read.0)() {
                        push_deep_link(url);
                    }
                }
                drain.call(());
            }
            _ => {}
        }
    });

    rsx! {}
}

/// No-op on web and desktop — links there are just navigations.
#[cfg(not(any(target_os = "android", target_os = "ios")))]
#[component]
pub fn DeepLinkListener() -> Element {
    rsx! {}
}

#[cfg(test)]
mod tests {
    use super::without_fragment;

    #[test]
    fn a_share_link_loses_its_key() {
        let key = "u9Xk3vQ2mNp7bRt5wYz8aCd1eFg4hJk6lMn0oPq2rSt";
        assert_eq!(
            without_fragment(&format!("https://counted.fr/projects/{}#{key}", uuid::Uuid::nil())),
            format!("https://counted.fr/projects/{}", uuid::Uuid::nil())
        );
        assert!(!without_fragment(&format!("counted://projects/x#{key}")).contains(key));
    }

    #[test]
    fn a_url_without_a_fragment_is_unchanged() {
        assert_eq!(without_fragment("https://counted.fr/help"), "https://counted.fr/help");
    }

    #[test]
    fn only_the_first_hash_matters() {
        // Splitting on the first '#' makes it irrelevant that base64url cannot contain one.
        assert_eq!(without_fragment("https://counted.fr/p/1#a#b"), "https://counted.fr/p/1");
    }

    #[test]
    fn a_bare_fragment_yields_nothing() {
        assert_eq!(without_fragment("#secretkey"), "");
    }
}
