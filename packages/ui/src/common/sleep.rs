/// Dioxus runs the script as the *body* of an async function, so the top-level `await` is what
/// suspends it — an IIFE expression statement would resolve immediately. Same contract as
/// `copy_js`. Web uses a timer future instead — eval is CSP-blocked there (see `common::web_dom`).
#[cfg(not(target_arch = "wasm32"))]
fn sleep_js(ms: u32) -> String {
    format!("await new Promise(r => setTimeout(r, {ms}));")
}

pub async fn sleep(ms: u32) {
    #[cfg(target_arch = "wasm32")]
    crate::common::web_dom::sleep_ms(ms).await;
    #[cfg(not(target_arch = "wasm32"))]
    dioxus::prelude::document::eval(&sleep_js(ms)).await.ok();
}

#[cfg(test)]
mod tests {
    use super::sleep_js;

    const SUCCESS_MS: u32 = 4000;

    #[test]
    fn sleep_js_embeds_the_delay() {
        let js = sleep_js(SUCCESS_MS);
        assert!(js.contains(&SUCCESS_MS.to_string()), "{js}");
    }

    // Dioxus runs the script as an async function *body*. Without a top-level await the script
    // returns instantly and the caller resumes on the next tick instead of after the delay.
    #[test]
    fn sleep_js_awaits_at_the_top_level() {
        let js = sleep_js(SUCCESS_MS);
        assert!(js.trim_start().starts_with("await "), "{js}");
        assert!(js.contains("setTimeout"), "{js}");
    }
}
