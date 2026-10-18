use dioxus::prelude::*;

#[cfg(any(target_os = "android", target_os = "ios"))]
use crate::route::Route;

/// Arms the WebView back stack so the Android hardware back button (and the iOS edge-swipe, once
/// `allowsBackForwardNavigationGestures` is set natively) reaches the Dioxus router, which runs on
/// `MemoryHistory` and never touches `window.history` on its own.
///
/// It keeps **one** spare `window.history` entry while a back press has somewhere to go, and none
/// otherwise. Mirroring the router's depth entry-for-entry is not possible: `replace` - what the
/// dock's tabs use - and a `push` of the route already showing both leave `MemoryHistory` untouched,
/// so a `pushState` per route change drifted deeper than the router. On a `replace`d page (login,
/// reached from the dock's account tab) the extra entry swallowed the press and `go_back()` found an
/// empty history, repainting the same page - the "back just refreshes the screen" bug.
/// `can_go_back()` is the router's own answer, so it cannot drift.
///
/// At depth 0 off the home page the press navigates home instead of exiting, which is what a dock
/// tab reached by `replace` (or a cold-started deep link) should do. On the home page nothing is
/// armed, `WebView.canGoBack()` is false, and the system finishes the Activity.
///
/// An open sheet (any `.modal.modal-open`) arms it too, and the press closes the topmost sheet
/// instead of navigating: it clicks that sheet's own `.modal-backdrop`, so the sheet closes exactly
/// as a tap outside it does, unsaved-changes prompt included. A `MutationObserver` re-evaluates
/// whenever the DOM changes, because sheets open and close without any route change.
#[cfg(any(target_os = "android", target_os = "ios"))]
#[component]
pub fn NavigationSync() -> Element {
    let nav = navigator();
    // Subscribes the component to the router, so the body re-runs on every navigation.
    let route = use_route::<Route>();

    let armed = nav.can_go_back() || !matches!(route, Route::ProjectsPage {});

    // Idempotent: `_syncBack` only pushes or pops when the wanted state differs from the current one.
    // Before the listener below is installed it does not exist yet, and the install calls it once.
    document::eval(if armed {
        "window._routeArmed = true; if (window._syncBack) { window._syncBack(); }"
    } else {
        "window._routeArmed = false; if (window._syncBack) { window._syncBack(); }"
    });

    // Register the popstate listener once on mount.
    // Android hardware back: OS → WebView.goBack() → popstate → dioxus.send → here.
    use_effect(move || {
        let nav = nav.clone();
        spawn(async move {
            let mut js = document::eval(
                "if (!window.dioxusPopstateRegistered) { \
                    window.dioxusPopstateRegistered = true; \
                    const topSheet = function() { \
                        const open = document.querySelectorAll('.modal.modal-open'); \
                        return open.length ? open[open.length - 1] : null; \
                    }; \
                    window._syncBack = function() { \
                        const want = !!window._routeArmed || !!topSheet(); \
                        if (want && !window._dioxusBackArmed) { \
                            window._dioxusBackArmed = true; \
                            window.history.pushState({}, '', window.location.href); \
                        } else if (!want && window._dioxusBackArmed) { \
                            window._dioxusBackArmed = false; \
                            window._dioxusSyntheticBack = true; \
                            window.history.back(); \
                        } \
                    }; \
                    new MutationObserver(function() { window._syncBack(); }) \
                        .observe(document.body, { childList: true, subtree: true }); \
                    window.addEventListener('popstate', function() { \
                        if (window._dioxusSyntheticBack) { window._dioxusSyntheticBack = false; return; } \
                        window._dioxusBackArmed = false; \
                        const sheet = topSheet(); \
                        const backdrop = sheet && Array.from(sheet.querySelectorAll('.modal-backdrop')) \
                            .find(function(b) { return b.closest('.modal') === sheet; }); \
                        if (backdrop) { backdrop.click(); window._syncBack(); return; } \
                        dioxus.send('back'); \
                    }); \
                    window._syncBack(); \
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

/// No-op stub for web and desktop - browser manages history natively.
#[cfg(not(any(target_os = "android", target_os = "ios")))]
#[component]
pub fn NavigationSync() -> Element {
    rsx! {}
}
