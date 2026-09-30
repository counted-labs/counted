use dioxus::prelude::*;
use crate::tid;

use crate::common::{AppHeader, MailLink};
use crate::route::Route;

/// French legal notice, extracted to keys but translated only into `fr` and `en`.
///
/// The other locales fall back to English per message — deliberately: this is legal text about a
/// French publisher, and a translation nobody has reviewed is worse than an English one. Proper
/// nouns (the host's address, the statute reference) stay verbatim in every language.
#[component]
pub fn LegalNoticePage() -> Element {
    rsx! {
        div { class: "container app-container bg-base-100 overflow-auto p-4 max-w-md mx-auto flex flex-col gap-4",
            AppHeader {
                title: tid!("nav-legal"),
                back_button_route: Route::ProjectsPage {},
            }

            div { class: "card bg-base-100 shadow-soft",
                div { class: "card-body gap-4 text-sm leading-relaxed",

                    p { class: "text-xs text-base-content/70", {tid!("legal-updated")} }

                    h2 { class: "text-lg font-semibold", {tid!("legal-publisher-title")} }
                    p { {tid!("legal-publisher-body")} }
                    p {
                        {tid!("legal-contact-label")}
                        " "
                        MailLink { address: "contact@counted.fr" }
                    }
                    p { class: "text-xs text-base-content/70", {tid!("legal-publisher-address-note")} }

                    h2 { class: "text-lg font-semibold", {tid!("legal-director-title")} }
                    p { "Jonathan Bosi." }

                    h2 { class: "text-lg font-semibold", {tid!("legal-host-title")} }
                    p {
                        strong { "Hetzner Online GmbH" }
                        br {}
                        {tid!("legal-host-address")}
                        br {}
                        {tid!("legal-host-phone")}
                        br {}
                        a {
                            class: "link",
                            href: "https://www.hetzner.com",
                            target: "_blank",
                            rel: "noopener noreferrer",
                            "www.hetzner.com"
                        }
                    }
                    p { {tid!("legal-host-email-note")} }

                    h2 { class: "text-lg font-semibold", {tid!("legal-ip-title")} }
                    p { {tid!("legal-ip-body")} }
                    p {
                        {tid!("legal-ip-source")}
                        " "
                        a {
                            class: "link",
                            href: "https://github.com/counted-labs/counted",
                            target: "_blank",
                            rel: "noopener noreferrer",
                            "github.com/counted-labs/counted"
                        }
                        "."
                    }
                    p { {tid!("legal-ip-brand")} }
                    p {
                        {tid!("legal-ip-third-party")}
                        " "
                        Link { to: Route::LicensesPage {}, class: "link", {tid!("legal-licenses-title")} }
                        "."
                    }

                    h2 { class: "text-lg font-semibold", {tid!("legal-personal-data-title")} }
                    p {
                        {tid!("legal-personal-data-body")}
                        " "
                        Link { to: Route::PrivacyPage {}, class: "link", {tid!("nav-privacy")} }
                    }

                    h2 { class: "text-lg font-semibold", {tid!("legal-terms-title")} }
                    p {
                        {tid!("legal-terms-body")}
                        " "
                        Link { to: Route::TermsPage {}, class: "link", {tid!("nav-terms")} }
                    }

                    h2 { class: "text-lg font-semibold", {tid!("legal-report-title")} }
                    p {
                        {tid!("legal-report-body-a")}
                        " "
                        MailLink { address: "contact@counted.fr" }
                        ". "
                        {tid!("legal-report-body-b")}
                    }
                }
            }
        }
    }
}
