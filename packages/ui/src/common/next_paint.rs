/// Dioxus runs the script as the *body* of an async function, so the top-level `await` is what
/// suspends it — same contract as `sleep_js`. rAF fires before paint; the `setTimeout` hop is what
/// lands after it. Web uses web-sys instead — eval is CSP-blocked there (see `common::web_dom`).
#[cfg(not(target_arch = "wasm32"))]
fn next_paint_js() -> &'static str {
    "await (document.hidden ? Promise.resolve() : new Promise(r => requestAnimationFrame(() => setTimeout(r, 0))));"
}

/// Resolves once the browser has painted. Call it before blocking the thread, so whatever was just
/// rendered is actually on screen first.
pub async fn next_paint() {
    #[cfg(target_arch = "wasm32")]
    crate::common::web_dom::next_paint().await;
    #[cfg(not(target_arch = "wasm32"))]
    dioxus::prelude::document::eval(next_paint_js()).await.ok();
}

#[cfg(test)]
mod tests {
    use super::next_paint_js;

    // Dioxus runs the script as an async function *body*. Without a top-level await the script
    // returns instantly and the caller resumes before anything has been painted.
    #[test]
    fn next_paint_js_awaits_at_the_top_level() {
        let js = next_paint_js();
        assert!(js.trim_start().starts_with("await "), "{js}");
    }

    // rAF alone fires *before* paint. Dropping the timer would make this resolve a frame too early,
    // which is exactly the bug it exists to prevent.
    #[test]
    fn next_paint_js_hops_past_the_frame() {
        let js = next_paint_js();
        assert!(js.contains("requestAnimationFrame"), "{js}");
        assert!(js.contains("setTimeout"), "{js}");
    }

    // A backgrounded webview never fires rAF, so the caller would hang until foregrounded.
    #[test]
    fn next_paint_js_skips_the_wait_when_hidden() {
        let js = next_paint_js();
        assert!(js.contains("document.hidden"), "{js}");
    }
}
