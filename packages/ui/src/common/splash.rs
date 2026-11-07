use dioxus::prelude::*;
use crate::tid;

/// Set while the splash fades out, just before `use_client_ready` turns true and unmounts it.
static SPLASH_LEAVING: GlobalSignal<bool> = Signal::global(|| false);

/// Matches `#counted-splash.splash-leaving` in every `main.css`.
const FADE_MS: u32 = 200;

/// The apps hold the splash until the first page has painted under it, at most this long: their
/// first render already holds the cached list (`use_device_cache`), so the handoff shows it rather
/// than a skeleton.
#[cfg(not(target_arch = "wasm32"))]
const HOLD_JS: &str = "await new Promise(r => { if (document.hidden) return r(); requestAnimationFrame(() => setTimeout(r, 0)); setTimeout(r, 1000); });";

/// True once the client is live: after hydration on web, after the first page has painted on
/// mobile (capped at 1 s) and the splash has faded.
///
/// `use_effect` is client-only - it never runs during SSR, because its callback lands in
/// `pending_effects`, which is drained only from `wait_for_work()`, and the SSR path never calls
/// that. So the server render and the first WASM render both observe `false` and emit identical
/// markup. Diverging here is what produces the "root is undefined" hydration crash; the same gate
/// is used in `project_history_page.rs` and `app_layout.rs`.
///
/// Named `client_ready` rather than `hydrated` because there is no hydration on mobile.
pub fn use_client_ready() -> Signal<bool> {
    let mut ready = use_signal(|| false);
    use_effect(move || {
        spawn(async move {
            #[cfg(not(target_arch = "wasm32"))]
            document::eval(HOLD_JS).await.ok();
            *SPLASH_LEAVING.write() = true;
            crate::common::sleep(FADE_MS).await;
            ready.set(true);
        });
    });
    ready
}

/// Full-screen brand splash, overlaid until the client is live. Base-200 like every launch layer
/// before it (DOCUMENTATION.md §5.2.1); its 200ms fade is what hands over to the base-100 page.
///
/// Takes the logo instead of declaring its own: `packages/ui` has no `assets/` dir, and each
/// platform crate already declares - and references - its own icon, so the asset is guaranteed to
/// survive linking. manganis 0.7.10 compiles `#[used]` out of the link section, so an `Asset` that
/// no reachable code reads can be dropped by the linker and silently not bundled.
///
/// `id` is not decoration - `assets/main.css` hangs the no-JS failsafe and the fade off it.
#[component]
pub fn SplashScreen(logo: Asset) -> Element {
    rsx! {
        div {
            id: "counted-splash",
            class: if SPLASH_LEAVING() { "splash-leaving fixed inset-0 z-[999] bg-base-200 flex flex-col items-center justify-center gap-4" } else { "fixed inset-0 z-[999] bg-base-200 flex flex-col items-center justify-center gap-4" },
            div { class: "flex items-center gap-3",
                img { src: logo, alt: "", width: "32", height: "32", class: "rounded-xl" }
                span { class: "text-2xl font-extrabold font-display text-gradient-brand", "Counted" }
            }
            span { class: "loading loading-spinner loading-md text-primary", role: "status", aria_label: tid!("loading") }
        }
    }
}

#[cfg(test)]
mod tests {
    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn the_hold_waits_for_a_paint_and_never_past_a_second() {
        let js = super::HOLD_JS;
        assert!(js.trim_start().starts_with("await "), "the eval body must suspend: {js}");
        assert!(js.contains("requestAnimationFrame") && js.contains("setTimeout(r, 0)"), "{js}");
        assert!(js.contains("setTimeout(r, 1000)"), "the cap: {js}");
        assert!(js.contains("document.hidden"), "a backgrounded launch fires no frame: {js}");
    }
}
