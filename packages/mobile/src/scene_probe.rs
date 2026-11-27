//! Debug iOS builds only: what `mirror/.github/ios-scenes.mjs` reads off the page to check the scene
//! life cycle (docs/plans/ios-uiscene.md §3.3). Counted in `main.rs`'s event handler, which runs
//! from the first event, so a cold-start link that arrives before this mounts is still seen.

use dioxus::prelude::*;
use std::sync::atomic::{AtomicUsize, Ordering};

static OPENED: AtomicUsize = AtomicUsize::new(0);
static RESUMED: AtomicUsize = AtomicUsize::new(0);

pub fn count<T>(event: &dioxus::mobile::tao::event::Event<'_, T>) {
    use dioxus::mobile::tao::event::Event;
    match event {
        Event::Opened { urls } => _ = OPENED.fetch_add(urls.len(), Ordering::Relaxed),
        Event::Resumed => _ = RESUMED.fetch_add(1, Ordering::Relaxed),
        _ => {}
    }
}

#[component]
pub fn SceneProbe() -> Element {
    let mut tick = use_signal(|| 0u32);
    dioxus::mobile::use_wry_event_handler(move |event, _| {
        use dioxus::mobile::tao::event::Event;
        if matches!(event, Event::Opened { .. } | Event::Resumed) {
            tick += 1;
        }
    });
    tick();

    let mtm = objc2::MainThreadMarker::new();
    let key_window = mtm.and_then(crate::key_window).is_some();
    let scenes = mtm.map_or(0, |mtm| objc2_ui_kit::UIApplication::sharedApplication(mtm).connectedScenes().count());
    let opened = OPENED.load(Ordering::Relaxed);
    let resumed = RESUMED.load(Ordering::Relaxed);

    rsx! {
        div {
            id: "scene-probe",
            hidden: true,
            "data-opened": "{opened}",
            "data-resumed": "{resumed}",
            "data-key-window": "{key_window}",
            "data-scenes": "{scenes}",
        }
    }
}
