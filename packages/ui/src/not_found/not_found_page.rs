use dioxus::fullstack::{FullstackContext, StatusCode};
use dioxus::prelude::*;
use crate::tid;

use crate::common::AppHeader;
use crate::route::Route;

// `segments` is deliberately never rendered: it is the raw, attacker-controlled URL path, and this
// component exists precisely so that path stops reaching the response body.
#[component]
pub fn NotFoundPage(segments: Vec<String>) -> Element {
    let _ = segments;

    // The route now parses, so the router no longer throws and dioxus would answer 200. Off-server
    // FullstackContext::current() is None and this is a no-op.
    FullstackContext::commit_http_status(StatusCode::NOT_FOUND, None);

    rsx! {
        div { class: "container app-container bg-base-100 p-4 max-w-md mx-auto flex flex-col gap-4",
            AppHeader { title: tid!("not-found-title"), back_button_route: Route::ProjectsPage {} }

            div { class: "card bg-base-100 shadow-soft",
                div { class: "card-body gap-4 text-sm leading-relaxed items-center text-center",
                    p { class: "text-5xl font-bold font-display text-base-content/20", "404" }
                    p { {tid!("not-found-hint")} }
                    Link {
                        class: "btn btn-primary btn-sm",
                        to: Route::ProjectsPage {},
                        {tid!("not-found-back")}
                    }
                }
            }
        }
    }
}
