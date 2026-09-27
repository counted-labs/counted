use dioxus::prelude::*;
use crate::tid;

use super::use_third_party_licenses::{use_third_party_licenses, Licenses};
use crate::common::AppHeader;
use crate::route::Route;

#[component]
pub fn LicensesPage() -> Element {
    let licenses = use_third_party_licenses();

    rsx! {
        div { class: "container app-container bg-base-100 overflow-auto p-4 max-w-md mx-auto flex flex-col gap-4",
            AppHeader {
                title: tid!("legal-licenses-title"),
                back_button_route: Route::LegalNoticePage {},
            }

            div { class: "card bg-base-100 shadow-soft",
                div { class: "card-body gap-4 text-sm leading-relaxed",
                    match licenses() {
                        Licenses::Loading => rsx! {
                            span { class: "loading loading-spinner loading-sm self-center" }
                        },
                        Licenses::Failed => rsx! {
                            p { {tid!("legal-licenses-unavailable")} }
                        },
                        Licenses::Loaded(text) => rsx! {
                            pre { class: "text-xs whitespace-pre-wrap break-words font-mono", "{text}" }
                        },
                    }
                }
            }
        }
    }
}
