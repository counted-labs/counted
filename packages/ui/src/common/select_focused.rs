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

#[cfg(test)]
mod tests {
    use super::SELECT_FOCUSED_JS;

    #[test]
    fn select_focused_js_selects_after_the_tap() {
        assert!(SELECT_FOCUSED_JS.contains("setTimeout"), "{SELECT_FOCUSED_JS}");
        assert!(SELECT_FOCUSED_JS.contains("setSelectionRange"), "{SELECT_FOCUSED_JS}");
    }
}
