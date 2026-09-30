use dioxus::prelude::*;
use crate::tid;

use crate::common::{AppHeader, MailLink};
use crate::route::Route;

/// Support copy for a whole page of it.
///
/// Answers that mix prose with emphasis are split into `-term` / `-def` (or `-a` / `-b`) pairs
/// rather than one message with markup in it: Fluent cannot carry a component, and a translator
/// editing raw `<strong>` would be one typo away from breaking the page.
#[component]
pub fn HelpPage() -> Element {
    rsx! {
        div { class: "container app-container bg-base-100 overflow-auto p-4 max-w-md mx-auto flex flex-col gap-4",
            AppHeader { title: tid!("nav-help"), back_button_route: Route::ProjectsPage {} }

            p { class: "text-sm text-base-content/70 px-1",
                {tid!("help-intro")}
            }

            div { class: "flex flex-col gap-2",

                FaqItem {
                    question: tid!("help-create-project-q"),
                    answer: rsx! {
                        p { {tid!("help-create-project-a")} }
                    },
                }

                FaqItem {
                    question: tid!("help-add-participants-q"),
                    answer: rsx! {
                        p { {tid!("help-add-participants-a")} }
                    },
                }

                FaqItem {
                    question: tid!("help-share-project-q"),
                    answer: rsx! {
                        p { {tid!("help-share-project-a")} }
                    },
                }

                FaqItem {
                    question: tid!("help-add-expense-q"),
                    answer: rsx! {
                        p { {tid!("help-add-expense-a")} }
                    },
                }

                FaqItem {
                    question: tid!("help-types-q"),
                    answer: rsx! {
                        ul { class: "list-disc list-inside flex flex-col gap-1",
                            li {
                                strong { {tid!("expense-type-expense")} }
                                " "
                                {tid!("help-types-expense")}
                            }
                            li {
                                strong { {tid!("expense-type-transfer")} }
                                " "
                                {tid!("help-types-transfer")}
                            }
                            li {
                                strong { {tid!("expense-type-gain")} }
                                " "
                                {tid!("help-types-gain")}
                            }
                        }
                    },
                }

                FaqItem {
                    question: tid!("help-past-date-q"),
                    answer: rsx! {
                        p { {tid!("help-past-date-a")} }
                    },
                }

                FaqItem {
                    question: tid!("help-who-owes-q"),
                    answer: rsx! {
                        p { {tid!("help-who-owes-a")} }
                    },
                }

                FaqItem {
                    question: tid!("help-minimal-transfers-q"),
                    answer: rsx! {
                        p { {tid!("help-minimal-transfers-a")} }
                    },
                }

                FaqItem {
                    question: tid!("help-import-tricount-q"),
                    answer: rsx! {
                        p {
                            {tid!("help-import-tricount-a")}
                            " "
                            strong { {tid!("projects-import-tricount")} }
                            ". "
                            {tid!("help-import-tricount-b")}
                        }
                    },
                }

                FaqItem {
                    question: tid!("help-encryption-q"),
                    answer: rsx! {
                        p { {tid!("help-encryption-a")} }
                        ul { class: "list-disc list-inside flex flex-col gap-1 mt-2",
                            li {
                                strong { {tid!("help-encryption-e2ee-term")} }
                                " "
                                {tid!("help-encryption-e2ee-def")}
                            }
                            li {
                                strong { {tid!("help-encryption-zero-term")} }
                                " "
                                {tid!("help-encryption-zero-def")}
                            }
                        }
                        p { class: "mt-2",
                            {tid!("help-encryption-see")}
                            " "
                            Link { to: Route::PrivacyPage {}, class: "link", {tid!("nav-privacy")} }
                        }
                    },
                }

                FaqItem {
                    question: tid!("help-forgot-password-q"),
                    answer: rsx! {
                        p { class: "text-warning",
                            strong { {tid!("help-forgot-password-warning")} }
                        }
                        p { class: "mt-2", {tid!("help-forgot-password-a")} }
                    },
                }

                FaqItem {
                    question: tid!("help-archive-delete-q"),
                    answer: rsx! {
                        p {
                            {tid!("help-archive-delete-a")}
                            " "
                            strong { {tid!("project-archive")} }
                            " "
                            {tid!("help-archive-delete-b")}
                        }
                    },
                }

                FaqItem {
                    question: tid!("help-delete-account-q"),
                    answer: rsx! {
                        p { {tid!("help-delete-account-a")} }
                    },
                }
            }

            div { class: "text-sm text-center text-base-content/70 mt-4 mb-2",
                {tid!("help-contact")}
                " "
                MailLink { address: "contact@counted.fr" }
            }
        }
    }
}

#[derive(PartialEq, Props, Clone)]
struct FaqItemProps {
    question: String,
    answer: Element,
}

#[component]
fn FaqItem(props: FaqItemProps) -> Element {
    rsx! {
        div { class: "collapse collapse-arrow bg-base-100 border border-base-300",
            input { r#type: "checkbox", aria_label: "{props.question}" }
            div { class: "collapse-title font-semibold text-sm", "{props.question}" }
            div { class: "collapse-content text-sm", {props.answer} }
        }
    }
}
