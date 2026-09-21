use dioxus::prelude::*;
use ui::common::{use_app_contexts, use_client_ready, NativeClipboardReader, PageTitle, SplashScreen};
use ui::i18n::LocaleHead;
use ui::route::Route;

const ICON: Asset = asset!("/assets/counted.png");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

// Self-hosted fonts (no Google CDN). Referenced via asset!() so they are bundled
// with hashed URLs; @font-face is injected below with those resolved URLs.
const FONT_INTER_400: Asset = asset!("/assets/fonts/inter-400.woff2");
const FONT_INTER_500: Asset = asset!("/assets/fonts/inter-500.woff2");
const FONT_INTER_600: Asset = asset!("/assets/fonts/inter-600.woff2");
const FONT_JAKARTA_600: Asset = asset!("/assets/fonts/jakarta-600.woff2");
const FONT_JAKARTA_700: Asset = asset!("/assets/fonts/jakarta-700.woff2");

fn font_face_css() -> String {
    const TEMPLATE: &str = r#"@font-face{font-family:"Inter";font-style:normal;font-weight:400;font-display:swap;src:url("$I4") format("woff2")}
@font-face{font-family:"Inter";font-style:normal;font-weight:500;font-display:swap;src:url("$I5") format("woff2")}
@font-face{font-family:"Inter";font-style:normal;font-weight:600;font-display:swap;src:url("$I6") format("woff2")}
@font-face{font-family:"Plus Jakarta Sans";font-style:normal;font-weight:600;font-display:swap;src:url("$J6") format("woff2")}
@font-face{font-family:"Plus Jakarta Sans";font-style:normal;font-weight:700;font-display:swap;src:url("$J7") format("woff2")}"#;
    TEMPLATE
        .replace("$I4", &FONT_INTER_400.to_string())
        .replace("$I5", &FONT_INTER_500.to_string())
        .replace("$I6", &FONT_INTER_600.to_string())
        .replace("$J6", &FONT_JAKARTA_600.to_string())
        .replace("$J7", &FONT_JAKARTA_700.to_string())
}

fn main() {
    dioxus::logger::initialize_default();
    // Read per request by `auth_controller::login_salt`; checked here so an unset pepper fails
    // the boot instead of the first login.
    #[cfg(feature = "server")]
    {
        if std::env::var("AUTH_SALT_PEPPER").map_or(true, |p| p.len() < 32) {
            panic!("AUTH_SALT_PEPPER must be set to at least 32 characters (openssl rand -hex 32)");
        }
        // `dioxus::launch` is `serve(router(app))` with nothing added; spelling it out is what
        // lets the version gate sit on every route. See docs/dioxus-overrides.md.
        dioxus::serve(|| async {
            Ok(dioxus::server::router(app)
                .layer(dioxus::server::axum::middleware::from_fn(reject_outdated_clients)))
        });
    }
    // The SSR markup is in the request's locale; the wasm must hold that locale before its first
    // render or hydration diverges. Fetch it (hashed, immutable asset) and only then launch —
    // `dioxus_web::launch` is itself a `spawn_local`, so nesting it here changes nothing else.
    #[cfg(all(not(feature = "server"), target_arch = "wasm32"))]
    wasm_bindgen_futures::spawn_local(async {
        ui::i18n::preload_request_locale().await;
        dioxus::launch(app);
    });
    #[cfg(all(not(feature = "server"), not(target_arch = "wasm32")))]
    dioxus::launch(app);
}

/// 426 for a native app below `MIN_APP_VERSION`; see `api::app_version`.
#[cfg(feature = "server")]
async fn reject_outdated_clients(
    req: dioxus::server::axum::extract::Request,
    next: dioxus::server::axum::middleware::Next,
) -> dioxus::server::axum::response::Response {
    use api::app_version::{outdated_payload, refuses, HEADER};
    use dioxus::server::axum::{http, response::IntoResponse};

    // Borrows of `req` end here: the request body is !Sync, and a `&req` alive across the
    // `.await` below would make this future !Send, which `from_fn` requires.
    let refused = {
        let header = |name: &str| {
            req.headers().get(name).and_then(|v| v.to_str().ok()).map(|s| s.to_owned())
        };
        let app_version = header(HEADER);
        refuses(req.uri().path(), app_version.as_deref(), header("user-agent").as_deref())
            .then(|| (app_version, header("x-real-ip"), header("x-request-id")))
    };
    if let Some((app_version, ip, request_id)) = refused {
        tracing::info!(
            target: "counted::security",
            event = "client_outdated",
            outcome = "refused",
            subject = app_version.as_deref().unwrap_or("-"),
            ip = ip.as_deref().unwrap_or("-"),
            request_id = request_id.as_deref().unwrap_or("-"),
        );
        return (
            http::StatusCode::UPGRADE_REQUIRED,
            [(http::header::CONTENT_TYPE, "application/json")],
            outdated_payload(),
        )
            .into_response();
    }
    next.run(req).await
}

#[component]
fn app() -> Element {
    use_app_contexts();
    let ready = use_client_ready();
    // Native clipboard not available on web; the modal falls back to JS clipboard API
    use_context_provider(|| None::<NativeClipboardReader>);

    rsx! {
        // Global app resources
        PageTitle { text: "counted" }
        LocaleHead {}
        document::Link { rel: "icon", r#type: "image/png", href: ICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        document::Style { {font_face_css()} }

        // Overlaid, not swapped: SSR still ships the real markup below (crawlers and no-JS
        // clients get content), and hydration removes one node instead of diffing the tree.
        if !ready() {
            SplashScreen { logo: ICON }
        }

        main {
            "data-theme": "counted",
            class: "min-h-screen bg-base-200 flex flex-col items-center",
            Router::<Route> {}
        }
    }
}
