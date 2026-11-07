use crate::common::{BackButtonArrow, PageTitle};
use crate::route::Route;
use dioxus::prelude::*;

#[derive(PartialEq, Props, Clone)]
pub struct AppHeaderProps {
    title: String,
    sub_title: Option<String>,
    back_button_route: Route,
    /// False on the dock's own pages, where the dock already is the way out.
    #[props(default = true)]
    back: bool,
    /// Pins the header to the top of the page's scroll container, as a navigation bar is. Scrolled
    /// it gets a `backdrop-filter`, which traps any `position: fixed` child in the bar:
    /// render a sheet as a sibling of the header, never as one of its children.
    #[props(default = true)]
    sticky: bool,
    #[props(default = false)]
    brand: bool,
    /// A short label beside the title, such as "Perso".
    #[props(default)]
    badge: Option<String>,
    /// The title is decrypted data: it stays out of the browser tab, which history and extensions
    /// with the `tabs` permission keep.
    #[props(default = false)]
    private_title: bool,
    /// A tab root. On iOS the title is also drawn large under the bar and hands over to the bar's
    /// inline title as it scrolls under it (native-feel.css, boot.js); the copy is hidden elsewhere.
    #[props(default = false)]
    large: bool,
    children: Element,
}

#[component]
pub fn AppHeader(props: AppHeaderProps) -> Element {
    rsx! {
        PageTitle { text: if props.private_title { "counted".to_string() } else { format!("{} · counted", props.title) } }
        // `bg-base-100` is load-bearing when sticky: the header is transparent otherwise and the
        // page would scroll through it. `z-20` sits above the page, below the dock (z-50) and the
        // offline banner (z-60).
        //
        // `nav-material` (native-feel.css) swaps that fill for a blur that appears on scroll, toggled
        // by `.is-scrolled` from boot.js; the desktop shell links neither and keeps the solid bar.
        // The bar's height, its title and the back glyph are per platform there too (`.nav-bar`).
        //
        // Not daisyUI's `navbar-start/center/end`: that is `50% / shrink-0 / 50%`, built for a
        // fixed-width logo, and a title longer than half the bar was pushed off-screen. The
        // `1fr auto 1fr` grid keeps a short title centred; `min-w-0` on the middle cell is what
        // lets a long one truncate instead of overflowing (e2e: web/tests/long-names.spec.ts).
        header {
            "data-large": props.large.then_some("true"),
            class: if props.sticky { "nav-bar grid grid-cols-[1fr_auto_1fr] items-center w-full min-h-16 py-2 sticky top-0 z-20 bg-base-100 nav-material" } else { "nav-bar grid grid-cols-[1fr_auto_1fr] items-center w-full min-h-16 py-2" },
            div { class: "flex items-center",
                if props.back {
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
            }
            div { class: "nav-title-cell flex flex-col items-center min-w-0 text-center",
                div { class: "flex items-center gap-2 min-w-0 max-w-full",
                    h1 { id: "page-title", class: if props.brand { "text-2xl font-extrabold font-display text-gradient-brand truncate max-w-full" } else { "nav-title text-xl font-bold font-display truncate max-w-full" }, "{props.title}" }
                    if let Some(badge) = props.badge {
                        span { id: "page-title-badge", class: "badge badge-sm badge-soft badge-success shrink-0 font-semibold", "{badge}" }
                    }
                }
                if let Some(sub_title) = props.sub_title {
                    div { class: "text-sm mb-3 truncate max-w-full", "{sub_title}" }
                }
            }
            div { class: "flex items-center gap-1 justify-self-end", {props.children} }
        }
        if props.large {
            div { class: "large-title", aria_hidden: "true", "{props.title}" }
        }
    }
}
