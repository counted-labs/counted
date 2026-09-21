use dioxus::prelude::*;
use crate::tid;

use super::UpdateRequired;

/// Covers the whole app once the server has refused this build (`UpdateRequired`). Mounted by the
/// native entry points only, beside `SplashScreen`, and outside the router so no page can render
/// over it. The button is a plain link: in the WebView an anchor click goes to the system browser,
/// which hands a store URL to the store.
#[component]
pub fn UpdateRequiredScreen(store_url: &'static str) -> Element {
    let required = use_context::<Signal<UpdateRequired>>();
    if !required().0 {
        return rsx! {};
    }
    let body = if cfg!(target_os = "ios") {
        tid!("update-required-body-testflight")
    } else {
        tid!("update-required-body")
    };
    rsx! {
        div {
            id: "update-required",
            role: "alertdialog",
            aria_modal: "true",
            class: "fixed inset-0 z-[999] bg-base-200 flex flex-col items-center justify-center gap-4 p-6 text-center",
            h1 { class: "text-2xl font-extrabold font-display", {tid!("update-required-title")} }
            p { class: "max-w-sm text-base-content/80", "{body}" }
            a { id: "update-required-button", class: "btn btn-primary", href: store_url, {tid!("update-required-button")} }
        }
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use super::*;
    use crate::common::test_dom::texts;
    use dioxus::dioxus_core::{Mutation, VirtualDom};

    #[derive(Props, Clone, PartialEq)]
    struct HarnessProps {
        required: bool,
    }

    #[component]
    fn Harness(props: HarnessProps) -> Element {
        crate::i18n::use_test_i18n();
        use_context_provider(|| Signal::new(UpdateRequired(props.required)));
        rsx! { UpdateRequiredScreen { store_url: "https://store.example/counted" } }
    }

    fn attrs(m: &dioxus::dioxus_core::Mutations, name: &str) -> Vec<String> {
        m.edits
            .iter()
            .filter_map(|e| match e {
                Mutation::SetAttribute { name: n, value, .. } if *n == name => {
                    Some(format!("{value:?}"))
                }
                _ => None,
            })
            .collect()
    }

    #[test]
    fn nothing_is_rendered_until_the_server_refuses_the_build() {
        let mut dom = VirtualDom::new_with_props(Harness, HarnessProps { required: false });
        let m = dom.rebuild_to_vec();
        assert!(texts(&m).is_empty());
        assert!(attrs(&m, "href").is_empty());
    }

    #[test]
    fn the_screen_names_the_update_and_links_to_the_store() {
        let mut dom = VirtualDom::new_with_props(Harness, HarnessProps { required: true });
        let m = dom.rebuild_to_vec();
        let texts = texts(&m);
        assert!(texts.iter().any(|t| t == "Update required"), "{texts:?}");
        assert!(texts.iter().any(|t| t == "Update"), "{texts:?}");
        let hrefs = attrs(&m, "href");
        assert_eq!(hrefs.len(), 1, "{hrefs:?}");
        assert!(hrefs[0].contains("https://store.example/counted"), "{hrefs:?}");
    }
}
