use crate::auth::{LoginPage, RegisterPage, SettingsPage, VerifyEmailPage};
use crate::charts::ChartsPage;
use crate::common::AppLayout;
use crate::expenses::ExpensesPage;
use crate::friends::FriendsPage;
use crate::help::HelpPage;
use crate::legal::{LegalNoticePage, LicensesPage, TermsPage};
use crate::not_found::NotFoundPage;
use crate::payments::PaymentPage;
use crate::privacy::PrivacyPage;
use crate::project_history::ProjectHistoryPage;
use crate::projects::ProjectsPage;
use dioxus::prelude::*;
use uuid::Uuid;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    // The root is served by the static site (nginx `location = /`), so the app home lives at
    // /projects. This redirect is not cosmetic: MemoryHistory::default() starts at "/" and panics
    // with "index route does not exist" if nothing matches — mobile and desktop would die on launch.
    #[redirect("/", || Route::ProjectsPage {})]
    #[layout(AppLayout)]
        #[route("/projects")]
        ProjectsPage {},
        #[route("/projects/:project_id")]
        ExpensesPage { project_id: Uuid },
        #[route("/projects/:project_id/expenses/:expense_id")]
        PaymentPage { project_id: Uuid, expense_id: i32 },
        #[route("/projects/:project_id/history")]
        ProjectHistoryPage { project_id: Uuid },
        #[route("/charts")]
        ChartsPage {},
        #[route("/login")]
        LoginPage {},
        #[route("/register")]
        RegisterPage {},
        // `/account` predates the page being settings-for-everyone. Kept as a redirect: it is a
        // year-old bookmark for anyone who ever opened their account page.
        #[redirect("/account", || Route::SettingsPage {})]
        #[route("/settings")]
        SettingsPage {},
        #[route("/friends")]
        FriendsPage {},
        #[route("/verify-email/:token")]
        VerifyEmailPage { token: String },
        #[route("/privacy")]
        PrivacyPage {},
        #[route("/terms")]
        TermsPage {},
        #[route("/legal")]
        LegalNoticePage {},
        #[route("/licenses")]
        LicensesPage {},
        #[route("/help")]
        HelpPage {},
        // A security control, not a nicety. Without a catch-all, Route::from_str fails, the router
        // throws ParseRouteError, and dioxus-fullstack writes the parse error — which quotes the
        // raw request path — into the 404 body with no Content-Type at all. Only `nosniff` kept
        // that from being reflected XSS, and script-src has to keep 'unsafe-inline', so an XSS
        // reads the E2EE key straight out of localStorage. Catch-alls sort last, so every route
        // above still wins.
        #[route("/:..segments")]
        NotFoundPage { segments: Vec<String> },
}

/// The project a route is about, if any. Drives the one project store held by `AppLayout`: it is
/// what decides whether a navigation is "same project" (fetch nothing) or not.
///
/// Exhaustively matched on purpose, like `route_depth` — a new route must fail to compile rather
/// than silently default to fetching nothing.
pub fn project_id_of(route: &Route) -> Option<Uuid> {
    match route {
        Route::ExpensesPage { project_id }
        | Route::PaymentPage { project_id, .. }
        | Route::ProjectHistoryPage { project_id } => Some(*project_id),
        Route::ProjectsPage {}
        | Route::ChartsPage {}
        | Route::SettingsPage {}
        | Route::FriendsPage {}
        | Route::LoginPage {}
        | Route::RegisterPage {}
        | Route::VerifyEmailPage { .. }
        | Route::HelpPage {}
        | Route::PrivacyPage {}
        | Route::TermsPage {}
        | Route::LegalNoticePage {}
        | Route::LicensesPage {}
        | Route::NotFoundPage { .. } => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    fn roundtrip(route: Route) {
        let url = route.to_string();
        let parsed = Route::from_str(&url).expect("route URL should parse");
        assert_eq!(route, parsed);
    }

    #[test]
    fn route_projects_page_roundtrip() {
        roundtrip(Route::ProjectsPage {});
    }

    #[test]
    fn route_expenses_page_roundtrip() {
        roundtrip(Route::ExpensesPage { project_id: Uuid::new_v4() });
    }

    #[test]
    fn route_payment_page_roundtrip() {
        roundtrip(Route::PaymentPage { project_id: Uuid::new_v4(), expense_id: 42 });
    }

    #[test]
    fn route_project_history_roundtrip() {
        roundtrip(Route::ProjectHistoryPage { project_id: Uuid::new_v4() });
    }

    #[test]
    fn route_charts_roundtrip() {
        roundtrip(Route::ChartsPage {});
    }

    #[test]
    fn route_login_page_roundtrip() {
        roundtrip(Route::LoginPage {});
    }

    #[test]
    fn route_register_page_roundtrip() {
        roundtrip(Route::RegisterPage {});
    }

    #[test]
    fn route_settings_page_roundtrip() {
        roundtrip(Route::SettingsPage {});
    }

    /// The page moved from `/account` to `/settings` when it stopped being account-only. Anyone who
    /// bookmarked the old URL must still land somewhere.
    #[test]
    fn account_url_still_reaches_settings() {
        assert_eq!(Route::from_str("/account").unwrap(), Route::SettingsPage {});
    }

    // The app home moved off "/" so the static site can own the root.
    #[test]
    fn projects_page_is_projects() {
        assert_eq!(Route::ProjectsPage {}.to_string(), "/projects");
        assert_eq!(Route::from_str("/projects").unwrap(), Route::ProjectsPage {});
    }

    // Guards the mobile/desktop launch: MemoryHistory::default() parses "/" and panics if it fails.
    #[test]
    fn root_redirects_to_projects_page() {
        assert_eq!(Route::from_str("/").unwrap(), Route::ProjectsPage {});
    }

    // /projects (static) must not be swallowed by /projects/:project_id (dynamic), and vice versa.
    #[test]
    fn projects_page_and_expenses_page_do_not_collide() {
        let id = Uuid::new_v4();
        assert_eq!(
            Route::from_str(&format!("/projects/{id}")).unwrap(),
            Route::ExpensesPage { project_id: id }
        );
    }

    #[test]
    fn route_verify_email_page_roundtrip() {
        roundtrip(Route::VerifyEmailPage { token: "abc123".to_string() });
    }

    #[test]
    fn verify_email_url_format() {
        let token = "deadbeef".repeat(8); // 64 chars
        let route = Route::VerifyEmailPage { token: token.clone() };
        assert_eq!(route.to_string(), format!("/verify-email/{}", token));
    }

    #[test]
    fn route_privacy_page_roundtrip() {
        roundtrip(Route::PrivacyPage {});
    }

    #[test]
    fn route_terms_page_roundtrip() {
        roundtrip(Route::TermsPage {});
    }

    #[test]
    fn route_legal_notice_page_roundtrip() {
        roundtrip(Route::LegalNoticePage {});
    }

    #[test]
    fn route_licenses_page_roundtrip() {
        roundtrip(Route::LicensesPage {});
    }

    #[test]
    fn route_help_roundtrip() {
        roundtrip(Route::HelpPage {});
    }

    #[test]
    fn unknown_path_resolves_to_not_found() {
        assert!(matches!(Route::from_str("/zzz-does-not-exist").unwrap(), Route::NotFoundPage { .. }));
        assert!(matches!(Route::from_str("/api/v1/nope").unwrap(), Route::NotFoundPage { .. }));
    }

    // All three project routes share one store, which is what makes expenses -> expense -> back
    // cost no requests.
    #[test]
    fn project_id_of_names_every_project_route() {
        let id = Uuid::new_v4();
        assert_eq!(project_id_of(&Route::ExpensesPage { project_id: id }), Some(id));
        assert_eq!(
            project_id_of(&Route::PaymentPage { project_id: id, expense_id: 42 }),
            Some(id)
        );
        assert_eq!(project_id_of(&Route::ProjectHistoryPage { project_id: id }), Some(id));
    }

    // Off a project the store must fetch nothing at all — /login and /help cost no request.
    #[test]
    fn project_id_of_is_none_off_a_project() {
        for route in [
            Route::ProjectsPage {},
            Route::ChartsPage {},
            Route::SettingsPage {},
            Route::LoginPage {},
            Route::RegisterPage {},
            Route::HelpPage {},
            Route::PrivacyPage {},
            Route::TermsPage {},
            Route::LegalNoticePage {},
            Route::LicensesPage {},
            Route::VerifyEmailPage { token: "abc".to_string() },
            Route::NotFoundPage { segments: vec!["zzz".to_string()] },
        ] {
            assert_eq!(project_id_of(&route), None, "{route:?} is not a project route");
        }
    }

    // The catch-all must never swallow a real route — that would silently 404 the whole app.
    #[test]
    fn catch_all_does_not_shadow_real_routes() {
        assert_eq!(Route::from_str("/projects").unwrap(), Route::ProjectsPage {});
        assert_eq!(Route::from_str("/login").unwrap(), Route::LoginPage {});
        assert_eq!(Route::from_str("/settings").unwrap(), Route::SettingsPage {});
        assert_eq!(Route::from_str("/help").unwrap(), Route::HelpPage {});
        assert_eq!(Route::from_str("/terms").unwrap(), Route::TermsPage {});
        assert_eq!(Route::from_str("/legal").unwrap(), Route::LegalNoticePage {});
        assert_eq!(Route::from_str("/licenses").unwrap(), Route::LicensesPage {});
        assert_eq!(Route::from_str("/").unwrap(), Route::ProjectsPage {});
    }
}
