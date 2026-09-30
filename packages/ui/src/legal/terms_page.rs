use dioxus::prelude::*;
use crate::tid;

use crate::common::{AppHeader, MailLink};
use crate::route::Route;

/// Terms of use, extracted to keys but translated only into `fr` and `en` — see
/// `LegalNoticePage` for why the other locales deliberately fall back to English.
#[component]
pub fn TermsPage() -> Element {
    rsx! {
        div { class: "container app-container bg-base-100 overflow-auto p-4 max-w-md mx-auto flex flex-col gap-4",
            AppHeader {
                title: tid!("nav-terms"),
                back_button_route: Route::ProjectsPage {},
            }

            div { class: "card bg-base-100 shadow-soft",
                div { class: "card-body gap-4 text-sm leading-relaxed",

                    p { class: "text-xs text-base-content/70", {tid!("legal-updated")} }

                    p { {tid!("terms-intro")} }

                    div { class: "divider my-0" }

                    h2 { class: "text-lg font-semibold", {tid!("terms-purpose-title")} }
                    p {
                        {tid!("terms-purpose-body")}
                        " "
                        Link { to: Route::LegalNoticePage {}, class: "link", {tid!("nav-legal")} }
                    }

                    h2 { class: "text-lg font-semibold", {tid!("terms-acceptance-title")} }
                    p { {tid!("terms-acceptance-body")} }

                    h2 { class: "text-lg font-semibold", {tid!("terms-access-title")} }
                    p { {tid!("terms-access-body")} }

                    h2 { class: "text-lg font-semibold", {tid!("terms-account-title")} }
                    p { {tid!("terms-account-body")} }
                    p { class: "text-warning",
                        strong { {tid!("terms-account-key-point")} }
                        " "
                        {tid!("terms-account-warning-a")}
                        " "
                        em { {tid!("terms-account-warning-em")} }
                        " "
                        {tid!("terms-account-warning-b")}
                    }

                    h2 { class: "text-lg font-semibold", {tid!("terms-acceptable-use-title")} }
                    p { {tid!("terms-acceptable-use-intro")} }
                    ul { class: "list-disc list-inside flex flex-col gap-1",
                        li { {tid!("terms-acceptable-use-1")} }
                        li { {tid!("terms-acceptable-use-2")} }
                        li { {tid!("terms-acceptable-use-3")} }
                        li { {tid!("terms-acceptable-use-4")} }
                        li { {tid!("terms-acceptable-use-5")} }
                    }

                    h2 { class: "text-lg font-semibold", {tid!("terms-your-content-title")} }
                    p { {tid!("terms-your-content-body")} }

                    h2 { class: "text-lg font-semibold", {tid!("terms-availability-title")} }
                    p { {tid!("terms-availability-body")} }

                    h2 { class: "text-lg font-semibold", {tid!("terms-deletion-title")} }
                    p { {tid!("terms-deletion-account")} }
                    p { {tid!("terms-deletion-project")} }

                    h2 { class: "text-lg font-semibold", {tid!("terms-backup-title")} }
                    p { {tid!("terms-backup-body")} }

                    h2 { class: "text-lg font-semibold", {tid!("terms-liability-title")} }
                    p { {tid!("terms-liability-body")} }
                    p { {tid!("terms-liability-suspension")} }

                    h2 { class: "text-lg font-semibold", {tid!("terms-personal-data-title")} }
                    p {
                        {tid!("legal-personal-data-body")}
                        " "
                        Link { to: Route::PrivacyPage {}, class: "link", {tid!("nav-privacy")} }
                    }

                    h2 { class: "text-lg font-semibold", {tid!("terms-changes-title")} }
                    p { {tid!("terms-changes-body")} }

                    h2 { class: "text-lg font-semibold", {tid!("terms-law-title")} }
                    p {
                        {tid!("terms-law-body-a")}
                        " "
                        MailLink { address: "contact@counted.fr" }
                        ". "
                        {tid!("terms-law-body-b")}
                    }
                }
            }
        }
    }
}
