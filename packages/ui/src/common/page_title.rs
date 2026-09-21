use dioxus::prelude::*;

/// Sets `document.title`. Replaces `document::Title`, which dioxus-web implements through eval —
/// blocked by the app's CSP (see `common::web_dom`). Renders nothing on every target, so the
/// server-rendered tree and the hydrated one stay identical, and SSR still emits `<title>`.
#[component]
pub fn PageTitle(text: String) -> Element {
    // Only on change, like `document::Title` itself: the header this sits in re-renders on every
    // navigation, and on native each set_title costs an eval round-trip into the WebView.
    // `peek`, not a read: subscribing here would re-render the component on its own write.
    let mut last = use_signal(String::new);
    if *last.peek() != text {
        last.set(text.clone());

        #[cfg(target_arch = "wasm32")]
        crate::common::web_dom::set_title(&text);

        #[cfg(not(target_arch = "wasm32"))]
        dioxus::document::document().set_title(text);
    }

    rsx! {}
}
