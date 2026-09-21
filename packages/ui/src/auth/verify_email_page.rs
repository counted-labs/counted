use api::auth::auth_controller::verify_email;
use dioxus::fullstack::Json;
use dioxus::prelude::*;
use crate::tid;
use shared::{Account, VerifyEmailPayload};

use crate::common::{error_message, flush_session, Flash};
use crate::route::Route;

#[component]
pub fn VerifyEmailPage(token: String) -> Element {
    let mut error_msg: Signal<Option<String>> = use_signal(|| None);
    let mut auth_ctx = use_context::<Signal<Option<Account>>>();
    let mut flash = use_context::<Signal<Option<Flash>>>();
    let nav = use_navigator();

    use_effect(move || {
        let token_val = token.clone();
        spawn(async move {
            match verify_email(Json(VerifyEmailPayload { token: token_val })).await {
                Ok(account) => {
                    flush_session();
                    auth_ctx.set(Some(account));
                    flash.set(Some(Flash::ok(tid!("verify-email-welcome"))));
                    nav.push(Route::ProjectsPage {});
                }
                Err(e) => {
                    error_msg.set(Some(error_message(&e)));
                }
            }
        });
    });

    rsx! {
        div { class: "container app-container bg-base-100 p-4 max-w-md w-full mx-auto flex flex-col gap-6 mt-16",
            if error_msg().is_none() {
                p { class: "text-center text-base-content/70", {tid!("verify-email-checking")} }
            }
            if let Some(err) = error_msg() {
                div { role: "alert", class: "alert alert-error", "{err}" }
                Link {
                    class: "btn btn-outline w-full",
                    to: Route::LoginPage {},
                    {tid!("verify-email-back-to-login")}
                }
            }
        }
    }
}
