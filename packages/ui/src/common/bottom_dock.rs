use dioxus::prelude::*;
use crate::tid;

use crate::common::{haptic, Haptic};
use crate::icons::{ChartIcon, HomeIcon, UserIcon, ICON_ACTION};
use crate::route::Route;

#[component]
pub fn BottomDock() -> Element {
    let route = use_route::<Route>();
    let nav = use_navigator();

    let show = matches!(route, Route::ProjectsPage {} | Route::ChartsPage {} | Route::SettingsPage {} | Route::RegisterPage {} | Route::LoginPage {} | Route::ExpensesPage { .. });
    if !show {
        return rsx! {};
    }

    // A column, not a centred icon: HIG tab bars label every tab, and an icon-only bar is the
    // single loudest "this is a web app" tell in the whole shell. The icon drops from ICON_NAV to
    // ICON_ACTION and the button from `h-11` to `py-1.5` so that icon + gap + label lands back on
    // the same 2.75rem — the dock's height is unchanged, which is what lets `.safe-bottom-fab` and
    // every page's `pb-24`/`pb-36` stay as they are.
    //
    // The active item is a gradient, so a `hover:bg-*` would be painted over by its
    // `background-image` — brightness is the one thing the gradient can't occlude. `transition`
    // rather than `transition-colors` so the filter animates too. Tailwind wraps `hover:` in
    // `@media (hover: hover)`, so touch devices are unaffected.
    let btn_class = |active: bool| -> &'static str {
        if active {
            "justify-self-center flex flex-col items-center justify-center gap-0.5 px-3 py-1.5 rounded-2xl bg-gradient-brand text-primary-content hover:brightness-110 active:brightness-95 transition"
        } else {
            "justify-self-center flex flex-col items-center justify-center gap-0.5 px-3 py-1.5 rounded-2xl hover:text-secondary-content active:bg-base-200 active:text-secondary-content transition-colors"
        }
    };
    let label_class = "text-[10px] leading-none font-medium";

    // Resolved once, into `Copy` bools: three `move` closures cannot each capture `route`, which
    // is not `Copy`.
    let on_projects = matches!(route, Route::ProjectsPage {});
    let on_charts = matches!(route, Route::ChartsPage {});
    let on_settings = matches!(route, Route::SettingsPage {});

    // Only when the tab actually changes: re-tapping the tab you are on navigates nowhere, and a
    // haptic for a no-op is noise.
    let go = move |target: Route, active: bool| {
        if !active {
            haptic(Haptic::Light);
            let _ = nav.replace(target);
        }
    };

    rsx! {
        nav {
            class: "fixed safe-bottom-dock left-0 right-0 z-50 flex justify-center px-8 pointer-events-none",
            aria_label: tid!("nav-main"),
            // `chrome-material` replaces the solid fill: bars over content are translucent on iOS.
            // It is defined only in the mobile stylesheet, so web and desktop keep `bg-base-100`
            // from the class below it — the two compose, material simply wins where it exists.
            div { class: "bg-base-100 chrome-material rounded-full px-4 py-2 grid grid-cols-3 items-center w-full max-w-xs shadow-soft border border-base-200 pointer-events-auto",
                button {
                    id: "nav-projects",
                    r#type: "button",
                    class: btn_class(on_projects),
                    aria_label: tid!("nav-projects"),
                    aria_current: if on_projects { "page" },
                    // Dock entries are tabs, not steps: a push would make back re-enter the page
                    // just left — the project detail page above all, which the dock now shows on —
                    // and grow the stack on every tab cycle.
                    onclick: move |_| go(Route::ProjectsPage {}, on_projects),
                    HomeIcon { size: ICON_ACTION }
                    span { class: label_class, {tid!("nav-projects")} }
                }
                button {
                    id: "nav-charts",
                    r#type: "button",
                    class: btn_class(on_charts),
                    aria_label: tid!("nav-charts"),
                    aria_current: if on_charts { "page" },
                    onclick: move |_| go(Route::ChartsPage {}, on_charts),
                    ChartIcon { size: ICON_ACTION }
                    span { class: label_class, {tid!("nav-charts")} }
                }
                // Unconditional. This tab used to send anonymous users to /login instead, which left
                // them with no page of their own and no way to change a preference.
                button {
                    id: "nav-settings",
                    r#type: "button",
                    class: btn_class(on_settings),
                    aria_label: tid!("nav-settings"),
                    aria_current: if on_settings { "page" },
                    onclick: move |_| go(Route::SettingsPage {}, on_settings),
                    UserIcon { size: ICON_ACTION }
                    span { class: label_class, {tid!("nav-settings")} }
                }
            }
        }
    }
}
