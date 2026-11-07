use dioxus::prelude::*;
use std::sync::atomic::{AtomicBool, Ordering};

#[cfg(any(target_os = "android", target_os = "ios"))]
use crate::route::Route;

static ROUTE_ARMED: AtomicBool = AtomicBool::new(false);
static SHEET_OPEN: AtomicBool = AtomicBool::new(false);

/// Whether a back has anything to do: a route to go back to, or (on iOS, where `_syncBack` reports
/// it) a sheet to close. The iOS VoiceOver escape answers UIKit with it synchronously, before the
/// back it asks the page for has run (packages/mobile/src/main.rs).
pub fn back_armed() -> bool {
    ROUTE_ARMED.load(Ordering::Relaxed) || SHEET_OPEN.load(Ordering::Relaxed)
}

/// Arms the WebView back stack so the Android hardware back button and the iOS edge swipe (the
/// JS strip in `page_transition.rs`) reach the Dioxus router, which runs on `MemoryHistory` and
/// never touches `window.history` on its own.
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
/// armed, and the system finishes the Activity. On Android every change of the armed state is
/// reported to `MainActivity` (`CountedBack.setArmed`), whose `OnBackPressedCallback` is enabled
/// only while armed, so the system's predictive back-to-home preview plays at the root.
///
/// The `history.back()` that disarms is a traversal, and it lands after any script that runs
/// before it. A sheet closing as a route opens (creating a project) disarmed and re-armed at once,
/// the push landed first, and the traversal then stepped back past it: armed on paper, nothing to
/// go back to, and the next press closed the app. So nothing re-arms while that back is in flight;
/// its own `popstate` runs `_syncBack` again.
///
/// An open sheet (any `.modal.modal-open`) arms it too, and the press closes the topmost sheet
/// instead of navigating: it clicks that sheet's own `.modal-backdrop`, so the sheet closes exactly
/// as a tap outside it does, unsaved-changes prompt included. The shared observer
/// (`window._countedObserve`) re-evaluates whenever the DOM changes, because sheets open and close
/// without any route change.
///
/// On iOS WebKit's own swipe (`allowsBackForwardNavigationGestures`) stays off: the edge strip in
/// `page_transition.rs` drags the page itself and, on commit, stamps `window._countedSwipeBack`
/// and calls `history.back()`, which lands in the listener below like an Android press. Every
/// `_syncBack` tells the strip (`_countedEdgeSync`) whether a back is armed and no sheet is open,
/// the only time it shows, and sends `sheet-open` / `sheet-closed` when the top sheet comes or goes,
/// for [`back_armed`].
#[cfg(any(target_os = "android", target_os = "ios"))]
#[component]
pub fn NavigationSync() -> Element {
    let nav = navigator();
    // Subscribes the component to the router, so the body re-runs on every navigation.
    let route = use_route::<Route>();

    let armed = nav.can_go_back() || !matches!(route, Route::ProjectsPage {});
    ROUTE_ARMED.store(armed, Ordering::Relaxed);

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
                    const setArmed = function(armed) { \
                        window._dioxusBackArmed = armed; \
                        if (window.CountedBack) { window.CountedBack.setArmed(armed); } \
                    }; \
                    const ios = document.documentElement.dataset.plat === 'ios'; \
                    window._syncBack = function() { \
                        const sheetOpen = !!topSheet(); \
                        if (window._countedEdgeSync) { window._countedEdgeSync(); } \
                        if (ios && sheetOpen !== !!window._countedSheetOpen) { \
                            window._countedSheetOpen = sheetOpen; \
                            dioxus.send(sheetOpen ? 'sheet-open' : 'sheet-closed'); \
                        } \
                        if (window._dioxusSyntheticBack) { return; } \
                        const want = !!window._routeArmed || sheetOpen; \
                        if (want && !window._dioxusBackArmed) { \
                            window.history.pushState({}, '', window.location.href); \
                            setArmed(true); \
                        } else if (!want && window._dioxusBackArmed) { \
                            setArmed(false); \
                            window._dioxusSyntheticBack = true; \
                            window.history.back(); \
                        } \
                    }; \
                    window._countedObserve(function() { window._syncBack(); }); \
                    window.addEventListener('popstate', function() { \
                        if (window._dioxusSyntheticBack) { \
                            window._dioxusSyntheticBack = false; \
                            window._syncBack(); \
                            return; \
                        } \
                        setArmed(false); \
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
                match js.recv::<String>().await.as_deref() {
                    Ok("sheet-open") => SHEET_OPEN.store(true, Ordering::Relaxed),
                    Ok("sheet-closed") => SHEET_OPEN.store(false, Ordering::Relaxed),
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
