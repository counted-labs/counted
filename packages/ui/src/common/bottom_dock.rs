use dioxus::prelude::*;
use crate::tid;

use crate::common::LocalStorageState;
use crate::icons::{ChartIcon, HomeIcon, UserIcon, ICON_ACTION};
use crate::route::{project_id_of, Route};
use uuid::Uuid;

#[derive(Clone, Copy, PartialEq, Debug)]
enum DockTab {
    Projects,
    Charts,
    Settings,
}

/// The tab a page sits under. Exhaustive, like `route_depth`: a new route has to say where it lives.
fn active_tab(route: &Route) -> Option<DockTab> {
    match route {
        Route::ProjectsPage {}
        | Route::ExpensesPage { .. }
        | Route::PaymentPage { .. }
        | Route::ProjectHistoryPage { .. }
        | Route::RecurringPage { .. }
        | Route::RecurringRulePage { .. } => Some(DockTab::Projects),
        Route::ChartsPage {} => Some(DockTab::Charts),
        Route::SettingsPage {}
        | Route::LoginPage {}
        | Route::RegisterPage {}
        | Route::ForgotPasswordPage {}
        | Route::ResetPage { .. }
        | Route::FriendsPage {}
        | Route::PaymentMethodsPage {}
        | Route::HelpPage {}
        | Route::PrivacyPage {}
        | Route::TermsPage {}
        | Route::LegalNoticePage {}
        | Route::LicensesPage {} => Some(DockTab::Settings),
        Route::VerifyEmailPage { .. } | Route::DemoPage { .. } | Route::NotFoundPage { .. } => None,
    }
}

/// The dock stays on pushed pages, as UIKit's tab bar and M3's navigation bar do. Only the two
/// pages that are a step in a flow rather than a place go without: the e-mail check and the demo
/// loader, both gone once they answer.
fn shows_dock(route: &Route) -> bool {
    !matches!(route, Route::VerifyEmailPage { .. } | Route::DemoPage { .. })
}

/// Each tab keeps its own place, as on iOS and Android: leaving a project for another tab and
/// tapping Projects again reopens that project, while tapping Projects from inside it goes up to
/// the list. Returns where to go (`None` for a re-tap) and the project the Projects tab remembers.
fn dock_nav(route: &Route, tapped: DockTab, remembered: Option<Uuid>) -> (Option<Route>, Option<Uuid>) {
    let in_project = project_id_of(route);
    let memory = match (route, tapped) {
        (Route::ProjectsPage {}, _) => None,
        (_, DockTab::Projects) if in_project.is_some() => None,
        _ => in_project.or(remembered),
    };
    let target = match tapped {
        DockTab::Projects if active_tab(route) != Some(DockTab::Projects) => remembered
            .map(|project_id| Route::ExpensesPage { project_id })
            .unwrap_or(Route::ProjectsPage {}),
        DockTab::Projects => Route::ProjectsPage {},
        DockTab::Charts => Route::ChartsPage {},
        DockTab::Settings => Route::SettingsPage {},
    };
    if *route == target {
        return (None, memory);
    }
    (Some(target), memory)
}

#[component]
pub fn BottomDock() -> Element {
    let route = use_route::<Route>();
    let nav = use_navigator();
    let ls = use_context::<Signal<LocalStorageState>>();
    // Lives here because AppLayout never remounts the dock, so it survives every tab switch.
    let mut last_project = use_signal(|| None::<Uuid>);

    if !shows_dock(&route) {
        return rsx! {};
    }

    // A column, not a centred icon: HIG tab bars label every tab, and an icon-only bar is the
    // single loudest "this is a web app" tell in the whole shell.
    //
    // Concentric: the tab pill's radius (h-12 / 2 = 24px) plus the dock's p-1.5 and 1px border
    // equals the dock's own radius (31px), so both arcs share a centre. The dock stays 62px tall,
    // which is what lets `.safe-bottom-fab` and every page's `pb-24`/`pb-36` stay as they are.
    //
    // The active item is a gradient, so a `hover:bg-*` would be painted over by its
    // `background-image` - brightness is the one thing the gradient can't occlude. `transition`
    // rather than `transition-colors` so the filter animates too. Tailwind wraps `hover:` in
    // `@media (hover: hover)`, so touch devices are unaffected.
    let btn_class = |active: bool| -> &'static str {
        if active {
            "w-full h-12 flex flex-col items-center justify-center gap-0.5 rounded-full bg-gradient-brand text-primary-content hover:brightness-110 active:brightness-95 transition"
        } else {
            "w-full h-12 flex flex-col items-center justify-center gap-0.5 rounded-full hover:text-secondary-content active:bg-base-200 active:text-secondary-content transition-colors"
        }
    };
    let label_class = "dock-label leading-none font-medium";

    let lit = active_tab(&route);
    let below_a_tab = !matches!(route, Route::ProjectsPage {} | Route::ChartsPage {} | Route::SettingsPage {});
    // "true", not "page", below a tab: the tab is the current section, not the current page.
    let aria_current = move |tab: DockTab| (lit == Some(tab)).then_some(if below_a_tab { "true" } else { "page" });

    // The route is read at tap time: three `move` closures cannot each capture `route`, which is
    // not `Copy`. A remembered project left or forgotten since falls back to the list.
    let mut go = move |tab: DockTab| {
        let remembered = last_project
            .peek()
            .filter(|id| ls.peek().projects.iter().any(|p| p.project_id == *id));
        let (target, memory) = dock_nav(&router().current::<Route>(), tab, remembered);
        last_project.set(memory);
        // No haptic: UITabBar and Material's navigation bar are silent (D4 in docs/plans/native-feel.md).
        if let Some(target) = target {
            let _ = nav.replace(target);
        }
    };

    rsx! {
        nav {
            class: "fixed safe-bottom-dock left-0 right-0 z-50 flex justify-center px-8 pointer-events-none",
            aria_label: tid!("nav-main"),
            // `chrome-material` replaces the solid fill: bars over content are translucent on iOS and
            // the web. It is in native-feel.css, which desktop does not link, so desktop keeps
            // `bg-base-100` from the class below it - material simply wins where it exists.
            div { class: "bg-base-100 chrome-material rounded-full p-1.5 grid grid-cols-3 items-center w-full max-w-xs shadow-soft border border-base-200 pointer-events-auto",
                button {
                    id: "nav-projects",
                    r#type: "button",
                    class: btn_class(lit == Some(DockTab::Projects)),
                    aria_label: tid!("nav-projects"),
                    aria_current: aria_current(DockTab::Projects),
                    // Dock entries are tabs, not steps: a push would make back re-enter the page
                    // just left - the project detail page above all, which the dock now shows on -
                    // and grow the stack on every tab cycle.
                    onclick: move |_| go(DockTab::Projects),
                    HomeIcon { size: ICON_ACTION }
                    span { class: label_class, {tid!("nav-projects")} }
                }
                button {
                    id: "nav-charts",
                    r#type: "button",
                    class: btn_class(lit == Some(DockTab::Charts)),
                    aria_label: tid!("nav-charts"),
                    aria_current: aria_current(DockTab::Charts),
                    onclick: move |_| go(DockTab::Charts),
                    ChartIcon { size: ICON_ACTION }
                    span { class: label_class, {tid!("nav-charts")} }
                }
                // Unconditional. This tab used to send anonymous users to /login instead, which left
                // them with no page of their own and no way to change a preference.
                button {
                    id: "nav-settings",
                    r#type: "button",
                    class: btn_class(lit == Some(DockTab::Settings)),
                    aria_label: tid!("nav-settings"),
                    aria_current: aria_current(DockTab::Settings),
                    onclick: move |_| go(DockTab::Settings),
                    UserIcon { size: ICON_ACTION }
                    span { class: label_class, {tid!("nav-settings")} }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn project(id: Uuid) -> Route {
        Route::ExpensesPage { project_id: id }
    }

    #[test]
    fn projects_tab_stays_lit_inside_a_project() {
        assert_eq!(active_tab(&project(Uuid::new_v4())), Some(DockTab::Projects));
        assert_eq!(active_tab(&Route::ProjectsPage {}), Some(DockTab::Projects));
    }

    #[test]
    fn settings_tab_stays_lit_on_login_and_register() {
        assert_eq!(active_tab(&Route::LoginPage {}), Some(DockTab::Settings));
        assert_eq!(active_tab(&Route::RegisterPage {}), Some(DockTab::Settings));
    }

    #[test]
    fn settings_from_login_goes_to_settings() {
        assert_eq!(dock_nav(&Route::LoginPage {}, DockTab::Settings, None), (Some(Route::SettingsPage {}), None));
    }

    #[test]
    fn retapping_the_projects_list_is_a_no_op() {
        assert_eq!(dock_nav(&Route::ProjectsPage {}, DockTab::Projects, None), (None, None));
    }

    #[test]
    fn retapping_another_tab_is_a_no_op_and_keeps_the_project() {
        let a = Uuid::new_v4();
        assert_eq!(dock_nav(&Route::ChartsPage {}, DockTab::Charts, Some(a)), (None, Some(a)));
    }

    #[test]
    fn projects_from_inside_a_project_goes_to_the_list_and_forgets_it() {
        let a = Uuid::new_v4();
        assert_eq!(
            dock_nav(&project(a), DockTab::Projects, Some(a)),
            (Some(Route::ProjectsPage {}), None)
        );
    }

    #[test]
    fn leaving_a_project_for_another_tab_remembers_it() {
        let a = Uuid::new_v4();
        assert_eq!(dock_nav(&project(a), DockTab::Charts, None), (Some(Route::ChartsPage {}), Some(a)));
        assert_eq!(dock_nav(&project(a), DockTab::Settings, None), (Some(Route::SettingsPage {}), Some(a)));
    }

    #[test]
    fn leaving_the_list_for_another_tab_forgets_any_project() {
        let a = Uuid::new_v4();
        assert_eq!(
            dock_nav(&Route::ProjectsPage {}, DockTab::Charts, Some(a)),
            (Some(Route::ChartsPage {}), None)
        );
    }

    #[test]
    fn projects_from_another_tab_returns_to_the_remembered_project() {
        let a = Uuid::new_v4();
        for from in [Route::ChartsPage {}, Route::SettingsPage {}, Route::LoginPage {}, Route::RegisterPage {}] {
            assert_eq!(dock_nav(&from, DockTab::Projects, Some(a)), (Some(project(a)), Some(a)), "from {from:?}");
        }
    }

    #[test]
    fn projects_from_another_tab_without_a_project_goes_to_the_list() {
        assert_eq!(
            dock_nav(&Route::ChartsPage {}, DockTab::Projects, None),
            (Some(Route::ProjectsPage {}), None)
        );
    }

    #[test]
    fn a_page_pushed_inside_a_project_keeps_the_dock_with_projects_lit() {
        let a = Uuid::new_v4();
        for route in [
            Route::PaymentPage { project_id: a, expense_id: 1 },
            Route::ProjectHistoryPage { project_id: a },
            Route::RecurringPage { project_id: a },
        ] {
            assert_eq!(active_tab(&route), Some(DockTab::Projects), "{route:?}");
            assert!(shows_dock(&route), "{route:?}");
        }
    }

    #[test]
    fn pages_pushed_from_settings_keep_the_dock_with_settings_lit() {
        for route in [Route::FriendsPage {}, Route::PaymentMethodsPage {}, Route::HelpPage {}, Route::LicensesPage {}, Route::PrivacyPage {}] {
            assert_eq!(active_tab(&route), Some(DockTab::Settings), "{route:?}");
            assert!(shows_dock(&route), "{route:?}");
        }
    }

    #[test]
    fn the_flow_steps_have_no_dock() {
        assert!(!shows_dock(&Route::VerifyEmailPage { token: String::new() }));
        assert!(!shows_dock(&Route::DemoPage { lang: String::new() }));
    }

    #[test]
    fn projects_from_a_page_deep_in_a_project_goes_to_the_list() {
        let a = Uuid::new_v4();
        assert_eq!(
            dock_nav(&Route::PaymentPage { project_id: a, expense_id: 1 }, DockTab::Projects, Some(a)),
            (Some(Route::ProjectsPage {}), None)
        );
    }

    #[test]
    fn leaving_a_page_deep_in_a_project_for_another_tab_remembers_the_project() {
        let a = Uuid::new_v4();
        assert_eq!(
            dock_nav(&Route::RecurringPage { project_id: a }, DockTab::Charts, None),
            (Some(Route::ChartsPage {}), Some(a))
        );
    }

    #[test]
    fn switching_between_other_tabs_keeps_the_project() {
        let a = Uuid::new_v4();
        assert_eq!(
            dock_nav(&Route::ChartsPage {}, DockTab::Settings, Some(a)),
            (Some(Route::SettingsPage {}), Some(a))
        );
    }
}
