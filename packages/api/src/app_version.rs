//! Refuses native apps older than what the server can still talk to.
//!
//! The web bundle ships with the server and is never gated. Native builds send
//! `X-App-Version` (their `CARGO_PKG_VERSION`, the store-visible version) and the floor is
//! `MIN_APP_VERSION`, read from the environment on every request so raising it is a `.env` edit
//! and a restart, not a rebuild. The pre-header 0.2.0 sends neither the header nor a
//! `User-Agent` — reqwest sets none unless asked, every browser does — which is how it is told
//! apart from a browser and refused too.
//!
//! The 426 body is `{"message","code"}`: dioxus-fullstack turns that into
//! `ServerFnError::ServerError { message, code }` on every client since 0.7.9, so an app predating
//! this module shows its existing "outdated" login error instead of a decode failure.
//!
//! Only the decision lives here, so `cargo test -p api` covers it; the axum layer is in
//! `packages/web/src/main.rs`.

use shared::errors;

pub const HEADER: &str = "x-app-version";
pub const MIN_ENV: &str = "MIN_APP_VERSION";

fn triple(s: &str) -> Option<(u32, u32, u32)> {
    let mut parts = s.trim().split('.').map(|p| p.parse::<u32>().ok());
    let v = (parts.next()??, parts.next()??, parts.next()??);
    parts.next().is_none().then_some(v)
}

/// Gate closed unless `min` parses; then refused when the app version parses below it, or when
/// the request names neither an app version nor a user agent. A malformed version is let through:
/// locking every phone out on a parse bug is worse than letting one odd build in.
pub fn outdated(app_version: Option<&str>, user_agent: Option<&str>, min: Option<&str>) -> bool {
    let Some(min) = min.and_then(triple) else { return false };
    match app_version {
        Some(v) => triple(v).is_some_and(|v| v < min),
        None => user_agent.is_none_or(|ua| ua.trim().is_empty()),
    }
}

pub fn outdated_payload() -> String {
    serde_json::json!({ "message": errors::CLIENT_OUTDATED, "code": 426 }).to_string()
}

/// The decision for one request, from its path and headers. The axum layer that calls this lives
/// in `packages/web` — the only crate that links `dioxus_server`, whose axum is the router's.
pub fn refuses(path: &str, app_version: Option<&str>, user_agent: Option<&str>) -> bool {
    path.starts_with("/api/")
        && outdated(app_version, user_agent, std::env::var(MIN_ENV).ok().as_deref())
}

#[cfg(test)]
mod tests {
    use super::*;

    const UA: Option<&str> = Some("Mozilla/5.0");

    #[test]
    fn no_floor_means_no_gate() {
        assert!(!outdated(Some("0.0.1"), None, None));
        assert!(!outdated(None, None, None));
        assert!(!outdated(Some("0.0.1"), None, Some("")));
        assert!(!outdated(Some("0.0.1"), None, Some("latest")));
    }

    #[test]
    fn older_is_refused_equal_and_newer_pass() {
        assert!(outdated(Some("0.1.9"), UA, Some("0.2.0")));
        assert!(!outdated(Some("0.2.0"), UA, Some("0.2.0")));
        assert!(!outdated(Some("0.2.1"), UA, Some("0.2.0")));
        assert!(!outdated(Some("1.0.0"), UA, Some("0.9.9")));
    }

    #[test]
    fn components_compare_numerically_not_lexically() {
        assert!(!outdated(Some("0.10.0"), UA, Some("0.9.0")));
        assert!(outdated(Some("0.9.0"), UA, Some("0.10.0")));
    }

    #[test]
    fn a_malformed_version_is_let_through() {
        assert!(!outdated(Some("banana"), None, Some("1.0.0")));
        assert!(!outdated(Some("1.0"), None, Some("1.0.0")));
        assert!(!outdated(Some("1.0.0.0"), None, Some("1.0.0")));
        assert!(!outdated(Some(""), None, Some("1.0.0")));
    }

    #[test]
    fn whitespace_around_a_version_is_ignored() {
        assert!(outdated(Some(" 0.1.0 "), UA, Some("0.2.0\n")));
    }

    #[test]
    fn no_header_and_no_user_agent_is_the_pre_header_app() {
        assert!(outdated(None, None, Some("0.2.1")));
        assert!(outdated(None, Some(""), Some("0.2.1")));
        assert!(!outdated(None, UA, Some("0.2.1")));
        assert!(!outdated(None, Some("curl/8.0"), Some("0.2.1")));
    }

    #[test]
    fn the_payload_is_what_the_client_decodes() {
        let v: serde_json::Value = serde_json::from_str(&outdated_payload()).unwrap();
        assert_eq!(v["code"], 426);
        assert_eq!(v["message"], errors::CLIENT_OUTDATED);
        assert!(v.get("data").is_none());
    }
}
