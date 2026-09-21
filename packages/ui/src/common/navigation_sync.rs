use dioxus::prelude::*;

#[cfg(any(target_os = "android", target_os = "ios"))]
use crate::route::Route;

/// Arms the WebView back stack so the Android hardware back button (and the iOS edge-swipe, once
/// `allowsBackForwardNavigationGestures` is set natively) reaches the Dioxus router, which runs on
/// `MemoryHistory` and never touches `window.history` on its own.
///
/// It keeps **one** spare `window.history` entry while a back press has somewhere to go, and none
/// otherwise. Mirroring the router's depth entry-for-entry is not possible: `replace` — what the
/// dock's tabs use — and a `push` of the route already showing both leave `MemoryHistory` untouched,
/// so a `pushState` per route change drifted deeper than the router. On a `replace`d page (login,
/// reached from the dock's account tab) the extra entry swallowed the press and `go_back()` found an
/// empty history, repainting the same page — the "back just refreshes the screen" bug.
/// `can_go_back()` is the router's own answer, so it cannot drift.
///
/// At depth 0 off the home page the press navigates home instead of exiting, which is what a dock
/// tab reached by `replace` (or a cold-started deep link) should do. On the home page nothing is
/// armed, `WebView.canGoBack()` is false, and the system finishes the Activity.
#[cfg(any(target_os = "android", target_os = "ios"))]
#[component]
pub fn NavigationSync() -> Element {
    let nav = navigator();
    // Subscribes the component to the router, so the body re-runs on every navigation.
    let route = use_route::<Route>();

    let armed = nav.can_go_back() || !matches!(route, Route::ProjectsPage {});

    // Idempotent both ways — the flag lives in JS, so a re-render that changes nothing does nothing.
    // Disarming goes through history.back(), which fires popstate; the synthetic flag tells the
    // listener below that this one is ours and not a user press.
    document::eval(if armed {
        "if (!window._dioxusBackArmed) { \
            window._dioxusBackArmed = true; \
            window.history.pushState({}, '', window.location.href); \
        }"
    } else {
        "if (window._dioxusBackArmed) { \
            window._dioxusBackArmed = false; \
            window._dioxusSyntheticBack = true; \
            window.history.back(); \
        }"
    });

    // Register the popstate listener once on mount.
    // Android hardware back: OS → WebView.goBack() → popstate → dioxus.send → here.
    use_effect(move || {
        let nav = nav.clone();
        spawn(async move {
            let mut js = document::eval(
                "if (!window.dioxusPopstateRegistered) { \
                    window.dioxusPopstateRegistered = true; \
                    window.addEventListener('popstate', function() { \
                        if (window._dioxusSyntheticBack) { window._dioxusSyntheticBack = false; return; } \
                        window._dioxusBackArmed = false; \
                        dioxus.send('back'); \
                    }); \
                }",
            );
            loop {
                match js.recv::<String>().await {
                    Ok(_) => {
                        if nav.can_go_back() {
                            nav.go_back();
                        } else {
                            let _ = nav.replace(Route::ProjectsPage {});
                        }
                    }
                    Err(_) => break,
                }
            }
        });
    });

    rsx! {}
}

/// No-op stub for web and desktop — browser manages history natively.
#[cfg(not(any(target_os = "android", target_os = "ios")))]
#[component]
pub fn NavigationSync() -> Element {
    rsx! {}
}
