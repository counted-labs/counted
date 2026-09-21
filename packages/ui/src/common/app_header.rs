use crate::common::{BackButtonArrow, PageTitle};
use crate::route::Route;
use dioxus::prelude::*;

#[derive(PartialEq, Props, Clone)]
pub struct AppHeaderProps {
    title: String,
    sub_title: Option<String>,
    back_button_route: Route,
    /// Pins the header to the top of the page's scroll container.
    #[props(default = false)]
    sticky: bool,
    children: Element,
}

/// Toggles `.is-scrolled` on the sticky header once its scroll container has moved, which is what
/// makes the bar transparent at rest and translucent over content — the iOS navigation-bar
/// behaviour. The classes it drives live in `packages/mobile/assets/main.css`.
///
/// Plain JS on a passive **capture-phase** listener, for two reasons: `scroll` does not bubble, so
/// a listener on `main` only sees the `.app-container` scrolling underneath it during capture; and
/// every event dispatched to Rust on a native mobile target costs a synchronous XHR over wry's
/// bridge, so a Dioxus `onscroll` here would pay that once per frame of every scroll. Same
/// technique and same reason as the `scrollTop` restore in `page_transition.rs`.
///
/// Mobile-only: the web build must never call `document::eval` (DOCUMENTATION.md §8), and
/// `.nav-material` is defined only in the mobile stylesheet, so there is nothing to drive elsewhere.
#[cfg(any(target_os = "android", target_os = "ios"))]
fn use_scroll_material() {
    use_effect(|| {
        document::eval(
            r#"
            (function() {
                if (window._countedNavMaterial) return;
                const main = document.querySelector('main');
                if (!main) return;
                // Only once `main` was actually found — setting it above would burn the guard on a
                // first run that attached nothing, and no later mount would retry.
                window._countedNavMaterial = true;
                main.addEventListener('scroll', function(e) {
                    const t = e.target;
                    if (!t || !t.querySelector) return;
                    // Scoped to the container that scrolled, never document-wide: mid-transition
                    // there are two page shells in `main` — the incoming one and the exiting
                    // overlay — and on a push the overlay is inserted *before* the live shell, so
                    // a document-wide lookup would style the dying header and leave the real one
                    // flat. The sticky header is a child of the scroller, so this is exact.
                    const bar = t.querySelector('.nav-material');
                    if (bar) bar.classList.toggle('is-scrolled', t.scrollTop > 2);
                }, { capture: true, passive: true });
            })();
        "#,
        );
    });
}

#[cfg(not(any(target_os = "android", target_os = "ios")))]
fn use_scroll_material() {}

#[component]
pub fn AppHeader(props: AppHeaderProps) -> Element {
    use_scroll_material();

    rsx! {
        PageTitle { text: "{props.title} · counted" }
        // `bg-base-100` is load-bearing when sticky: the header is transparent otherwise and the
        // page would scroll through it. `z-20` sits above the page, below the dock (z-50) and the
        // offline banner (z-60).
        //
        // `nav-material` overrides that fill on mobile only, where it is swapped for a blur that
        // appears on scroll — the class is undefined on web and desktop, so those keep the solid
        // bar and nothing changes for them.
        header { class: if props.sticky { "navbar px-0 sticky top-0 z-20 bg-base-100 nav-material" } else { "navbar px-0" },
            div { class: "navbar-start",
                BackButtonArrow {
                    onclick: move |_| {
                        // `go_back` alone was a dead press on any page the dock reached by
                        // `replace` (login) or a deep link opened cold: MemoryHistory is empty
                        // there, so it only repainted. The declared route is the fallback.
                        #[cfg(any(target_os = "android", target_os = "ios"))]
                        {
                            let nav = navigator();
                            if nav.can_go_back() {
                                nav.go_back();
                            } else {
                                let _ = nav.replace(props.back_button_route.clone());
                            }
                        }
                        #[cfg(not(any(target_os = "android", target_os = "ios")))]
                        navigator().push(props.back_button_route.clone());
                    },
                }
            }
            div { class: "navbar-center flex-col",
                h1 { class: "text-xl font-bold font-display", "{props.title}" }
                if let Some(sub_title) = props.sub_title {
                    div { class: "text-sm mb-3", "{sub_title}" }
                }

            }
            div { class: "navbar-end", {props.children} }
        }
    }
}
