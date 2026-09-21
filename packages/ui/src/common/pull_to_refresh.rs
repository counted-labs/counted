//! The pull-to-refresh indicator and its half of the handshake with `overscroll.rs`.
//!
//! The gesture lives in JS and owns every visual state; this component only receives the one
//! "refresh" trigger, runs the page's refresh, and reports back when `busy` drops so the JS can
//! retract the indicator. Rendered as the first child of a page's `.app-container` — the gesture
//! looks for `:scope > .ptr`, and a page without one gets the bounce but no refresh.

use dioxus::prelude::*;

#[cfg(any(target_os = "android", target_os = "ios"))]
use crate::common::overscroll::{DONE_HOOK, REFRESH_HOOK};
#[cfg(any(target_os = "android", target_os = "ios"))]
use crate::common::{haptic, Haptic};

/// Whether the resource's future is running. Reads `state()`, which subscribes — that is the point:
/// the page re-renders when the refresh lands, and `busy` drops with it.
pub fn pending<T: 'static>(r: &Resource<T>) -> bool {
    matches!(*r.state().read(), UseResourceState::Pending)
}

#[cfg(any(target_os = "android", target_os = "ios"))]
#[component]
pub fn PullToRefresh(on_refresh: Callback<()>, busy: bool) -> Element {
    let mut refreshing = use_signal(|| false);

    // Scope-bound: leaving the page drops the loop, and the next data page's mount overwrites the
    // window hook. Between the two the gesture finds no `.ptr` in the live scroller and stays quiet.
    use_hook(move || {
        spawn(async move {
            let mut eval =
                document::eval(&format!("window.{REFRESH_HOOK} = function () {{ dioxus.send('refresh'); }};"));
            while eval.recv::<String>().await.is_ok() {
                refreshing.set(true);
                haptic(Haptic::Medium);
                on_refresh.call(());
            }
        });
    });

    // `busy` is already true on the render after the trigger: `Resource::restart()` sets `Pending`
    // synchronously. A refresh that restarts nothing reports done at once; the JS still shows the
    // spinner for its minimum. `use_reactive!` because `busy` is a plain prop: an effect re-runs
    // only when a signal it read changes, so without it the fetch landing never reached here and
    // the spinner never retracted.
    use_effect(use_reactive!(|busy| {
        if refreshing() && !busy {
            document::eval(&format!("if (window.{DONE_HOOK}) window.{DONE_HOOK}();"));
            refreshing.set(false);
        }
    }));

    let plat = if cfg!(target_os = "ios") { "ios" } else { "android" };

    rsx! {
        div { class: "ptr", "data-plat": plat, aria_hidden: "true",
            div { class: "ptr-indicator",
                svg {
                    class: "ptr-arc",
                    view_box: "0 0 24 24",
                    fill: "none",
                    stroke: "currentColor",
                    stroke_width: "2.5",
                    stroke_linecap: "round",
                    circle { cx: "12", cy: "12", r: "9", class: "ptr-track" }
                    path { d: "M12 3a9 9 0 0 1 9 9", class: "ptr-head" }
                }
            }
        }
    }
}

#[cfg(not(any(target_os = "android", target_os = "ios")))]
#[component]
pub fn PullToRefresh(on_refresh: Callback<()>, busy: bool) -> Element {
    let _ = (on_refresh, busy);
    rsx! {}
}
