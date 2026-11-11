/// The `setTimeout` lets the tap that focused the field place its caret first, which would
/// otherwise collapse the selection. `setSelectionRange` because iOS ignores `select()`.
#[cfg(not(target_arch = "wasm32"))]
const SELECT_FOCUSED_JS: &str = "setTimeout(() => { const el = document.activeElement; \
    if (el && el.setSelectionRange) el.setSelectionRange(0, el.value.length); }, 0);";

pub async fn select_focused_input() {
    #[cfg(target_arch = "wasm32")]
    crate::common::web_dom::select_focused_input().await;
    #[cfg(not(target_arch = "wasm32"))]
    dioxus::prelude::document::eval(SELECT_FOCUSED_JS).await.ok();
}

#[cfg(not(target_arch = "wasm32"))]
const FIELD_FOCUSED_JS: &str =
    "return ['INPUT', 'TEXTAREA', 'SELECT'].includes(document.activeElement?.tagName);";

/// Whether a form field holds the focus - a button that opened a modal does not count.
pub async fn a_field_has_focus() -> bool {
    #[cfg(target_arch = "wasm32")]
    return crate::common::web_dom::a_field_has_focus();
    #[cfg(not(target_arch = "wasm32"))]
    dioxus::prelude::document::eval(FIELD_FOCUSED_JS).join::<bool>().await.unwrap_or(false)
}

/// Focuses the field `id` with `preventScroll`, the one flag WebKit honours against its focus
/// reveal (`keyboard.rs`): a plain `focus()` into a sheet pans the page to the field, and on iOS to
/// wherever the field is mid-slide. Made from Rust, not page script, so iOS shows the keyboard.
pub async fn focus_field(id: &str) {
    #[cfg(target_arch = "wasm32")]
    crate::common::web_dom::focus_field(id);
    #[cfg(not(target_arch = "wasm32"))]
    dioxus::prelude::document::eval(&format!(
        "const el = document.getElementById({}); if (el) el.focus({{ preventScroll: true }});",
        serde_json::to_string(id).unwrap_or_default()
    ))
    .await
    .ok();
}

#[cfg(test)]
mod tests {
    use super::SELECT_FOCUSED_JS;

    #[test]
    fn select_focused_js_selects_after_the_tap() {
        assert!(SELECT_FOCUSED_JS.contains("setTimeout"), "{SELECT_FOCUSED_JS}");
        assert!(SELECT_FOCUSED_JS.contains("setSelectionRange"), "{SELECT_FOCUSED_JS}");
    }
}
