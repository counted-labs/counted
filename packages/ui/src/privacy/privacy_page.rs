use dioxus::prelude::*;
use crate::tid;

use crate::common::AppHeader;
use crate::route::Route;

/// Privacy policy, extracted to keys but translated only into `fr` and `en` — see
/// `LegalNoticePage` for why the other locales deliberately fall back to English.
///
/// The `counted_lang` cookie is named in §8 alongside the session cookie: both are strictly
/// necessary and neither needs consent, but both have to be declared.
#[component]
pub fn PrivacyPage() -> Element {
    rsx! {
        div { class: "container app-container bg-base-100 overflow-auto p-4 max-w-md mx-auto flex flex-col gap-4",
            AppHeader {
                title: tid!("nav-privacy"),
                back_button_route: Route::ProjectsPage {},
            }

            div { class: "card bg-base-100 shadow-soft",
                div { class: "card-body gap-4 text-sm leading-relaxed",

                    p { class: "text-xs text-base-content/70", {tid!("legal-updated")} }

                    p {
                        {tid!("privacy-intro-a")}
                        " "
                        em { {tid!("privacy-intro-em")} }
                        " "
                        {tid!("privacy-intro-b")}
                    }

                    div { class: "divider my-0" }

                    h2 { class: "text-lg font-semibold", {tid!("privacy-controller-title")} }
                    p {
                        {tid!("privacy-controller-a")}
                        " "
                        Link { to: Route::LegalNoticePage {}, class: "link", {tid!("nav-legal")} }
                        ". "
                        {tid!("privacy-controller-b")}
                        " "
                        a { class: "link", href: "mailto:contact@counted.fr", "contact@counted.fr" }
                    }

                    h2 { class: "text-lg font-semibold", {tid!("privacy-collected-title")} }
                    ul { class: "list-disc list-inside flex flex-col gap-1",
                        li { strong { {tid!("privacy-collected-email-term")} } " " {tid!("privacy-collected-email-def")} }
                        li { strong { {tid!("privacy-collected-hash-term")} } " " {tid!("privacy-collected-hash-def")} }
                        li { strong { {tid!("privacy-collected-salt-term")} } " " {tid!("privacy-collected-salt-def")} }
                        li { strong { {tid!("privacy-collected-content-term")} } " " {tid!("privacy-collected-content-def")} }
                        li { strong { {tid!("privacy-collected-prefs-term")} } " " {tid!("privacy-collected-prefs-def")} }
                        li { strong { {tid!("privacy-collected-keys-term")} } " " {tid!("privacy-collected-keys-def")} }
                        li { strong { {tid!("privacy-collected-invite-term")} } " " {tid!("privacy-collected-invite-def")} }
                        li { strong { {tid!("privacy-collected-friends-term")} } " " {tid!("privacy-collected-friends-def")} }
                        li { strong { {tid!("privacy-collected-logs-term")} } " " {tid!("privacy-collected-logs-def")} }
                    }

                    h2 { class: "text-lg font-semibold", {tid!("privacy-purposes-title")} }
                    ul { class: "list-disc list-inside flex flex-col gap-1",
                        li { {tid!("privacy-purpose-1")} }
                        li { {tid!("privacy-purpose-2")} }
                        li { {tid!("privacy-purpose-3")} }
                        li { {tid!("privacy-purpose-4")} }
                    }

                    h2 { class: "text-lg font-semibold", {tid!("privacy-legal-basis-title")} }
                    p { {tid!("privacy-legal-basis-body")} }

                    h2 { class: "text-lg font-semibold", {tid!("privacy-e2ee-title")} }
                    p {
                        {tid!("privacy-e2ee-a")}
                        " "
                        em { "zero-knowledge" }
                        " "
                        {tid!("privacy-e2ee-b")}
                    }
                    p { class: "text-warning",
                        strong { {tid!("privacy-e2ee-consequence-label")} }
                        " "
                        {tid!("privacy-e2ee-consequence-a")}
                        " "
                        em { {tid!("privacy-e2ee-consequence-em")} }
                        " "
                        {tid!("privacy-e2ee-consequence-b")}
                    }

                    h2 { class: "text-lg font-semibold", {tid!("privacy-processors-title")} }
                    ul { class: "list-disc list-inside flex flex-col gap-1",
                        li { strong { "Hetzner Online GmbH " } {tid!("privacy-processor-hetzner")} }
                        li { strong { "Scaleway TEM " } {tid!("privacy-processor-scaleway")} }
                        li { strong { "Bunq / Tricount " } {tid!("privacy-processor-tricount")} }
                        li { strong { "Grafana Labs " } {tid!("privacy-processor-grafana")} }
                    }
                    p { {tid!("privacy-no-transfer-outside-eu")} }

                    h2 { class: "text-lg font-semibold", {tid!("privacy-retention-title")} }
                    p {
                        {tid!("privacy-retention-a")}
                        " "
                        em { {tid!("privacy-retention-em")} }
                        " "
                        {tid!("privacy-retention-b")}
                    }
                    p { {tid!("privacy-sweep-intro")} }
                    ul { class: "list-disc list-inside flex flex-col gap-1",
                        li { {tid!("privacy-sweep-1")} }
                        li { {tid!("privacy-sweep-2")} }
                        li { {tid!("privacy-sweep-3")} }
                    }
                    p {
                        {tid!("privacy-logs-a")}
                        " "
                        strong { {tid!("privacy-logs-duration")} }
                        " "
                        {tid!("privacy-logs-b")}
                    }
                    p { {tid!("privacy-shared-expenses-survive")} }

                    h2 { class: "text-lg font-semibold", {tid!("privacy-cookies-title")} }
                    p {
                        {tid!("privacy-session-cookie-a")}
                        " ("
                        code { class: "text-xs bg-base-200 px-1 rounded", "HttpOnly" }
                        ", "
                        code { class: "text-xs bg-base-200 px-1 rounded", "SameSite=Lax" }
                        ", "
                        code { class: "text-xs bg-base-200 px-1 rounded", "Secure" }
                        ") "
                        {tid!("privacy-session-cookie-b")}
                    }
                    p {
                        {tid!("privacy-lang-cookie-a")}
                        " ("
                        code { class: "text-xs bg-base-200 px-1 rounded", "counted_lang" }
                        ") "
                        {tid!("privacy-lang-cookie-b")}
                    }
                    p {
                        {tid!("privacy-local-storage-a")}
                        " "
                        em { {tid!("privacy-local-storage-em")} }
                        " "
                        {tid!("privacy-local-storage-b")}
                    }
                    ul { class: "list-disc list-inside flex flex-col gap-1",
                        li { strong { {tid!("privacy-ls-keys-term")} } " " {tid!("privacy-ls-keys-def")} }
                        li { strong { {tid!("privacy-ls-projects-term")} } " " {tid!("privacy-ls-projects-def")} }
                        li { strong { {tid!("privacy-ls-cache-term")} } " " {tid!("privacy-ls-cache-def")} }
                        li { strong { {tid!("privacy-ls-queue-term")} } " " {tid!("privacy-ls-queue-def")} }
                        li { strong { {tid!("privacy-ls-prefs-term")} } " " {tid!("privacy-ls-prefs-def")} }
                    }
                    p {
                        {tid!("privacy-ls-necessary-a")}
                        " "
                        strong { {tid!("privacy-ls-warning-label")} }
                        " "
                        {tid!("privacy-ls-necessary-b")}
                    }

                    h2 { class: "text-lg font-semibold", {tid!("privacy-rights-title")} }
                    p { {tid!("privacy-rights-intro")} }
                    ul { class: "list-disc list-inside flex flex-col gap-1",
                        li { {tid!("privacy-right-access")} }
                        li { {tid!("privacy-right-erasure")} }
                        li { {tid!("privacy-right-portability")} }
                        li { {tid!("privacy-right-object")} }
                        li { {tid!("privacy-right-withdraw")} }
                        li {
                            {tid!("privacy-right-complaint")}
                            " "
                            a { class: "link", href: "https://www.cnil.fr/fr/plaintes", target: "_blank", "CNIL" }
                        }
                    }
                    p {
                        {tid!("privacy-rights-contact")}
                        " "
                        a { class: "link", href: "mailto:contact@counted.fr", "contact@counted.fr" }
                    }

                    h2 { class: "text-lg font-semibold", {tid!("privacy-security-title")} }
                    p { {tid!("privacy-security-body")} }

                    h2 { class: "text-lg font-semibold", {tid!("privacy-scan-title")} }
                    p { {tid!("privacy-scan-body")} }
                    p { {tid!("privacy-scan-retention")} }

                    h2 { class: "text-lg font-semibold", {tid!("privacy-changes-title")} }
                    p { {tid!("privacy-changes-body")} }
                }
            }
        }
    }
}
