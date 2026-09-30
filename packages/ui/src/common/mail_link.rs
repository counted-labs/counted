use dioxus::prelude::*;

/// An email address that opens the mail app.
///
/// On web a plain `mailto:` anchor. The phone apps cannot use one: the native interpreter hands
/// every link to `webbrowser::open`, which opens http(s) only, so the tap did nothing — it goes to
/// the OS through `open_external` instead. No SSR on mobile, so the two markups never meet.
#[component]
pub fn MailLink(#[props(into)] address: String) -> Element {
    #[cfg(any(target_os = "android", target_os = "ios"))]
    {
        let url = format!("mailto:{address}");
        rsx! {
            button {
                r#type: "button",
                class: "link",
                onclick: move |_| {
                    super::open_external(&url);
                },
                "{address}"
            }
        }
    }
    #[cfg(not(any(target_os = "android", target_os = "ios")))]
    rsx! {
        a { class: "link", href: "mailto:{address}", "{address}" }
    }
}
