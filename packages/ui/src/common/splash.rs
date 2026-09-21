use dioxus::prelude::*;
use crate::tid;

/// True once the client is live: after hydration on web, after the first render on mobile.
///
/// `use_effect` is client-only — it never runs during SSR, because its callback lands in
/// `pending_effects`, which is drained only from `wait_for_work()`, and the SSR path never calls
/// that. So the server render and the first WASM render both observe `false` and emit identical
/// markup. Diverging here is what produces the "root is undefined" hydration crash; the same gate
/// is used in `project_history_page.rs` and `app_layout.rs`.
///
/// Named `client_ready` rather than `hydrated` because there is no hydration on mobile.
pub fn use_client_ready() -> Signal<bool> {
    let mut ready = use_signal(|| false);
    use_effect(move || ready.set(true));
    ready
}

/// Full-screen brand splash, overlaid until the client is live.
///
/// Takes the logo instead of declaring its own: `packages/ui` has no `assets/` dir, and each
/// platform crate already declares — and references — its own icon, so the asset is guaranteed to
/// survive linking. manganis 0.7.10 compiles `#[used]` out of the link section, so an `Asset` that
/// no reachable code reads can be dropped by the linker and silently not bundled.
///
/// `id` is not decoration — `assets/main.css` hangs the no-JS failsafe off it.
#[component]
pub fn SplashScreen(logo: Asset) -> Element {
    rsx! {
        div {
            id: "counted-splash",
            class: "fixed inset-0 z-[999] bg-base-200 flex flex-col items-center justify-center gap-4",
            div { class: "flex items-center gap-3",
                img { src: logo, alt: "", width: "32", height: "32", class: "rounded-xl" }
                span { class: "text-2xl font-extrabold font-display text-gradient-brand", "Counted" }
            }
            span { class: "loading loading-spinner loading-md text-primary", role: "status", aria_label: tid!("loading") }
        }
    }
}
