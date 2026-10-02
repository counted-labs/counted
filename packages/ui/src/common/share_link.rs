use crate::crypto::{key_from_fragment, key_to_fragment};
use crate::route::Route;
use std::str::FromStr;
use uuid::Uuid;

pub const APP_BASE_URL: &str = "https://counted.fr";

/// Source of truth: `packages/mobile/Dioxus.toml` (`[deep_links] schemes`) and
/// `packages/mobile/ios/Info.plist` (`CFBundleURLSchemes`) must match this.
pub const APP_SCHEME: &str = "counted";

fn is_shareable_origin(origin: &str) -> bool {
    let host = origin
        .strip_prefix("https://")
        .or_else(|| origin.strip_prefix("http://"))
        .unwrap_or("")
        .trim_end_matches('/');
    !host.is_empty() && host != "dioxus.index.html" && host != "null"
}

pub fn share_url(origin: Option<&str>, project_id: Uuid, key: &[u8; 32]) -> String {
    let base = origin
        .map(str::trim)
        .filter(|o| is_shareable_origin(o))
        .map(|o| o.trim_end_matches('/'))
        .unwrap_or(APP_BASE_URL);
    format!("{base}{}#{}", Route::ExpensesPage { project_id }, key_to_fragment(key))
}

// None off wasm32: the mobile WebView URL is never the current route.
pub fn current_origin() -> Option<String> {
    #[cfg(target_arch = "wasm32")]
    {
        web_sys::window().and_then(|w| w.location().origin().ok())
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        None
    }
}

pub fn share_link_for(project_id: Uuid, key: &[u8; 32]) -> String {
    share_url(current_origin().as_deref(), project_id, key)
}

/// `counted://projects/<uuid>#<key>` — the custom-scheme twin of `share_url`, and the web "open in
/// the app" button's only handoff where universal links are unavailable (AltStore-resigned iOS
/// builds carry no associated-domains entitlement).
pub fn scheme_link_for(project_id: Uuid, key: &[u8; 32]) -> String {
    format!("{APP_SCHEME}:/{}#{}", Route::ExpensesPage { project_id }, key_to_fragment(key))
}

/// Inverse of [`share_url`] / [`scheme_link_for`]: pulls `(project_id, key)` out of any link shape
/// the app can emit (`https://counted.fr/...`, dev `http://localhost:8080/...`, `counted://...`).
/// None for anything without a valid key.
///
/// Goes through `Route::from_str`, the parser `share_url` formats with, so the two cannot drift.
/// Only `ExpensesPage` is accepted: the Android app-link filter claims every path on the host,
/// so everything else must fall through to "not a share link" rather than be guessed at.
pub fn parse_share_link(input: &str) -> Option<(Uuid, [u8; 32])> {
    let (before, fragment) = input.trim().split_once('#')?;
    let key = key_from_fragment(fragment.trim()).ok()?;

    // Strip the scheme, then the authority for http(s) only: `counted://projects/x` has no host
    // segment, so everything after the scheme is already the path.
    let path = if let Some(rest) = before.strip_prefix("https://").or(before.strip_prefix("http://"))
    {
        let start = rest.find('/')?;
        &rest[start..]
    } else {
        before.strip_prefix(&format!("{APP_SCHEME}://"))?
    };

    match_project(path).map(|id| (id, key))
}

/// The token of a mailed `https://counted.fr/verify-email/<token>` link. http(s) only: the email
/// never carries `counted://`, and accepting it would let any web page log the user in as someone
/// else through the custom scheme.
pub fn parse_verify_email_link(input: &str) -> Option<String> {
    let input = input.trim();
    let rest = input.strip_prefix("https://").or(input.strip_prefix("http://"))?;
    let path = rest[rest.find('/')?..].split(['#', '?']).next()?;
    match Route::from_str(path).ok()? {
        Route::VerifyEmailPage { token } if !token.is_empty() => Some(token),
        _ => None,
    }
}

/// Accepts a path with or without its leading slash, and only when it is a project page.
fn match_project(path: &str) -> Option<Uuid> {
    let path = if path.starts_with('/') { path.to_string() } else { format!("/{path}") };
    match Route::from_str(&path).ok()? {
        Route::ExpensesPage { project_id } => Some(project_id),
        _ => None,
    }
}

// Resolves to a bool. `text` goes in via serde_json::to_string, never interpolated raw. The script
// is the *body* of an async function (dioxus-web PROMISE_WRAPPER / dioxus-desktop AsyncFunction),
// so it must `return` — an IIFE expression statement evaluates to undefined. execCommand fallback:
// navigator.clipboard is undefined in a non-secure WebView context.
pub fn copy_js(text: &str) -> String {
    let literal = serde_json::to_string(text).unwrap_or_else(|_| "\"\"".to_string());
    format!(
        r#"const text = {literal};
try {{
    if (navigator.clipboard && navigator.clipboard.writeText) {{
        await navigator.clipboard.writeText(text);
        return true;
    }}
}} catch (e) {{}}
try {{
    const ta = document.createElement('textarea');
    ta.value = text;
    ta.setAttribute('readonly', '');
    ta.style.position = 'fixed';
    ta.style.opacity = '0';
    document.body.appendChild(ta);
    ta.select();
    ta.setSelectionRange(0, text.length);
    const ok = document.execCommand('copy');
    document.body.removeChild(ta);
    return ok;
}} catch (e) {{
    return false;
}}"#
    )
}

/// Reads the clipboard, `null` when the WebView refuses. Same contract as [`copy_js`]: the script
/// is the *body* of an async function, so it must `return` — wrapped in an IIFE it would evaluate
/// to undefined and every paste would silently do nothing. `navigator.clipboard` is undefined in a
/// non-secure WebView context, hence the guard.
pub const READ_CLIPBOARD_JS: &str = r#"try {
    if (navigator.clipboard && navigator.clipboard.readText) {
        return await navigator.clipboard.readText();
    }
} catch (e) {}
return null;"#;

/// Copies to the system clipboard, false when the browser or WebView refuses. Web goes straight
/// to the clipboard API: eval is CSP-blocked there (see `common::web_dom`).
pub async fn copy_text(text: &str) -> bool {
    #[cfg(target_arch = "wasm32")]
    {
        super::web_dom::copy_to_clipboard(text).await
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        dioxus::prelude::document::eval(&copy_js(text))
            .await
            .ok()
            .and_then(|v| v.as_bool())
            .unwrap_or(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypto::key_from_fragment;
    use std::str::FromStr;

    const KEY: [u8; 32] = [0x42u8; 32];

    const BAD_ORIGINS: [&str; 6] =
        ["dioxus://index.html", "https://dioxus.index.html/", "http://dioxus.index.html", "null", "", "   "];

    fn fragment_of(url: &str) -> &str {
        url.split_once('#').expect("share url must carry the key fragment").1
    }

    #[test]
    fn no_origin_uses_the_canonical_base() {
        let url = share_url(None, Uuid::nil(), &KEY);
        assert!(url.starts_with("https://counted.fr/projects/"), "{url}");
    }

    #[test]
    fn path_is_the_project_route() {
        let id = Uuid::new_v4();
        let url = share_url(None, id, &KEY);
        assert!(url.starts_with(&format!("https://counted.fr/projects/{id}#")), "{url}");
    }

    #[test]
    fn fragment_round_trips_to_the_key() {
        let url = share_url(None, Uuid::new_v4(), &KEY);
        assert_eq!(key_from_fragment(fragment_of(&url)).unwrap(), KEY);
    }

    #[test]
    fn path_parses_back_into_the_router() {
        let id = Uuid::new_v4();
        let url = share_url(None, id, &KEY);
        let path = url.trim_start_matches(APP_BASE_URL).split('#').next().unwrap().to_string();
        assert_eq!(Route::from_str(&path).unwrap(), Route::ExpensesPage { project_id: id });
    }

    #[test]
    fn web_origin_is_honoured() {
        let id = Uuid::nil();
        let url = share_url(Some("http://localhost:8080"), id, &KEY);
        assert!(url.starts_with(&format!("http://localhost:8080/projects/{id}#")), "{url}");
    }

    #[test]
    fn trailing_slash_is_stripped() {
        let url = share_url(Some("https://counted.fr/"), Uuid::nil(), &KEY);
        assert!(!url.contains("fr//"), "{url}");
        assert!(url.starts_with("https://counted.fr/projects/"), "{url}");
    }

    #[test]
    fn ios_webview_origin_falls_back_to_the_canonical_base() {
        let url = share_url(Some("dioxus://index.html"), Uuid::nil(), &KEY);
        assert!(url.starts_with("https://counted.fr/projects/"), "{url}");
    }

    #[test]
    fn android_webview_origin_falls_back_to_the_canonical_base() {
        let url = share_url(Some("https://dioxus.index.html/"), Uuid::nil(), &KEY);
        assert!(url.starts_with("https://counted.fr/projects/"), "{url}");
    }

    #[test]
    fn null_and_empty_origins_fall_back_to_the_canonical_base() {
        for o in ["null", "", "   "] {
            let url = share_url(Some(o), Uuid::nil(), &KEY);
            assert!(url.starts_with("https://counted.fr/projects/"), "{o} -> {url}");
        }
    }

    #[test]
    fn no_origin_ever_produces_a_dioxus_link_or_drops_the_key() {
        let id = Uuid::new_v4();
        for o in BAD_ORIGINS {
            let url = share_url(Some(o), id, &KEY);
            assert!(!url.contains("dioxus"), "{o} -> {url}");
            assert_eq!(key_from_fragment(fragment_of(&url)).unwrap(), KEY, "{o} -> {url}");
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn current_origin_is_none_on_native() {
        assert!(current_origin().is_none());
    }

    #[test]
    fn copy_js_embeds_the_url_as_a_json_literal() {
        let url = share_url(None, Uuid::nil(), &KEY);
        assert!(copy_js(&url).contains(&format!("\"{url}\"")));
    }

    #[test]
    fn copy_js_escapes_quotes_backslashes_and_newlines() {
        let js = copy_js("a\"b\\c\nd</script>");
        assert!(js.contains(r#""a\"b\\c\nd</script>""#), "{js}");
        // One opening and one closing quote for the literal — it cannot break out.
        assert_eq!(js.matches("const text = ").count(), 1);
    }

    #[test]
    fn copy_js_has_a_fallback_for_webviews_without_navigator_clipboard() {
        assert!(copy_js("x").contains("execCommand"));
    }

    // Dioxus runs the script as an async function *body*, so an IIFE expression statement
    // resolves to undefined and every copy reports failure. It has to `return`.
    #[test]
    fn copy_js_returns_its_result_instead_of_being_an_iife() {
        let js = copy_js("x");
        assert!(!js.trim_start().starts_with("(async"), "{js}");
        assert!(js.contains("return true;"), "{js}");
        assert!(js.contains("return ok;"), "{js}");
        assert!(js.contains("return false;"), "{js}");
    }

    // Same trap as `copy_js`: an IIFE resolves to undefined and every paste silently does nothing.
    #[test]
    fn read_clipboard_js_returns_its_result_instead_of_being_an_iife() {
        assert!(!READ_CLIPBOARD_JS.trim_start().starts_with("(async"), "{READ_CLIPBOARD_JS}");
        assert!(READ_CLIPBOARD_JS.contains("return await navigator.clipboard.readText();"));
        assert!(READ_CLIPBOARD_JS.contains("return null;"));
    }

    #[test]
    fn read_clipboard_js_guards_a_missing_clipboard_api() {
        assert!(READ_CLIPBOARD_JS.contains("navigator.clipboard && navigator.clipboard.readText"));
    }

    /// The parser must undo exactly what the builder does, for every origin it can produce.
    #[test]
    fn parses_back_every_url_share_url_can_build() {
        let id = Uuid::new_v4();
        for o in [None, Some("https://counted.fr"), Some("http://localhost:8080"), Some("null")] {
            let url = share_url(o, id, &KEY);
            assert_eq!(parse_share_link(&url), Some((id, KEY)), "{o:?} -> {url}");
        }
    }

    #[test]
    fn scheme_link_round_trips() {
        let id = Uuid::new_v4();
        let url = scheme_link_for(id, &KEY);
        assert!(url.starts_with(&format!("counted://projects/{id}#")), "{url}");
        assert_eq!(parse_share_link(&url), Some((id, KEY)));
    }

    /// What actually arrives off a clipboard or an intent extra.
    #[test]
    fn surrounding_whitespace_is_tolerated() {
        let id = Uuid::new_v4();
        let url = share_url(None, id, &KEY);
        assert_eq!(parse_share_link(&format!("  {url}\n")), Some((id, KEY)));
    }

    #[test]
    fn rejects_links_without_a_key_fragment() {
        let id = Uuid::new_v4();
        assert_eq!(parse_share_link(&format!("https://counted.fr/projects/{id}")), None);
        assert_eq!(parse_share_link(&format!("https://counted.fr/projects/{id}#")), None);
    }

    #[test]
    fn rejects_a_fragment_that_is_not_a_32_byte_key() {
        let id = Uuid::new_v4();
        // Valid base64url, wrong length.
        assert_eq!(parse_share_link(&format!("https://counted.fr/projects/{id}#abcd")), None);
        // Not base64url at all.
        assert_eq!(parse_share_link(&format!("https://counted.fr/projects/{id}#!!!!")), None);
    }

    #[test]
    fn rejects_a_malformed_project_id() {
        let frag = key_to_fragment(&KEY);
        assert_eq!(parse_share_link(&format!("https://counted.fr/projects/not-a-uuid#{frag}")), None);
    }

    /// The Android app-link filter claims every path on counted.fr, so a non-project page must
    /// parse as "not a share link" rather than be coerced into one.
    #[test]
    fn rejects_every_other_route_on_the_host() {
        let id = Uuid::new_v4();
        let frag = key_to_fragment(&KEY);
        for path in [
            "/".to_string(),
            "/projects".to_string(),
            "/charts".to_string(),
            "/login".to_string(),
            "/account".to_string(),
            "/settings".to_string(),
            "/privacy".to_string(),
            format!("/projects/{id}/history"),
            format!("/projects/{id}/expenses/7"),
        ] {
            let url = format!("https://counted.fr{path}#{frag}");
            assert_eq!(parse_share_link(&url), None, "{url}");
        }
    }

    #[test]
    fn rejects_unrelated_input() {
        let frag = key_to_fragment(&KEY);
        let id = Uuid::new_v4();
        for input in [
            "".to_string(),
            "   ".to_string(),
            "not a link".to_string(),
            "https://tricount.com/AbCdEf123".to_string(),
            // Right shape, foreign scheme.
            format!("tricount://projects/{id}#{frag}"),
            // Scheme but no path.
            format!("https://counted.fr#{frag}"),
        ] {
            assert_eq!(parse_share_link(&input), None, "{input}");
        }
    }

    /// A link copied from a phone must still work: the fragment is the only place the key lives,
    /// and it survives both Android intents and iOS user activities.
    #[test]
    fn parses_links_from_a_foreign_host() {
        let id = Uuid::new_v4();
        let frag = key_to_fragment(&KEY);
        assert_eq!(
            parse_share_link(&format!("https://staging.counted.fr/projects/{id}#{frag}")),
            Some((id, KEY))
        );
    }

    #[test]
    fn parses_the_mailed_verify_email_link() {
        let token = "deadbeef".repeat(8);
        let url = format!("https://counted.fr{}", Route::VerifyEmailPage { token: token.clone() });
        assert_eq!(parse_verify_email_link(&url), Some(token.clone()));
        assert_eq!(parse_verify_email_link(&format!("  {url}\n")), Some(token));
    }

    #[test]
    fn verify_email_link_rejects_every_other_shape() {
        let id = Uuid::new_v4();
        let frag = key_to_fragment(&KEY);
        for input in [
            "".to_string(),
            "https://counted.fr/verify-email/".to_string(),
            "https://counted.fr/verify-email".to_string(),
            format!("https://counted.fr/projects/{id}#{frag}"),
            "https://counted.fr/login".to_string(),
            "counted://verify-email/abc".to_string(),
        ] {
            assert_eq!(parse_verify_email_link(&input), None, "{input}");
        }
    }

    #[test]
    fn copy_js_of_a_share_url_carries_the_whole_link() {
        let id = Uuid::new_v4();
        let url = share_url(None, id, &KEY);
        let js = copy_js(&url);
        assert!(js.contains(&format!("https://counted.fr/projects/{id}#{}", key_to_fragment(&KEY))), "{js}");
    }
}
