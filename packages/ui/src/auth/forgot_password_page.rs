use dioxus::prelude::*;

use crate::auth::recovery_flow::send_reset_link;
use crate::common::{error_message, AppHeader};
use crate::route::Route;
use crate::tid;

/// "Forgot password" (docs/plans/key-lifecycle.md §6.2): the same answer whether or not the address
/// has an account.
#[component]
pub fn ForgotPasswordPage() -> Element {
    let mut email = use_signal(String::new);
    let mut sent = use_signal(|| false);
    let mut loading = use_signal(|| false);
    let mut error: Signal<Option<String>> = use_signal(|| None);

    let on_submit = move |e: FormEvent| {
        e.prevent_default();
        let value = email();
        spawn(async move {
            loading.set(true);
            error.set(None);
            match send_reset_link(value).await {
                Ok(()) => sent.set(true),
                Err(e) => error.set(Some(error_message(&e))),
            }
            loading.set(false);
        });
    };

    rsx! {
        div { class: "container app-container bg-base-100 p-4 max-w-md w-full mx-auto flex flex-col gap-5 overflow-auto pb-24",
            AppHeader { title: tid!("forgot-title"), back_button_route: Route::LoginPage {} }
            if let Some(err) = error() {
                div { id: "forgot-error", role: "alert", class: "alert alert-error", "{err}" }
            }
            if sent() {
                div { id: "forgot-sent", role: "status", class: "alert alert-success", {tid!("forgot-sent")} }
            } else {
                p { class: "text-sm text-base-content/70", {tid!("forgot-body")} }
                form { class: "flex flex-col gap-4", onsubmit: on_submit,
                    fieldset { class: "fieldset",
                        label { class: "fieldset-legend", r#for: "forgot-email", {tid!("field-email")} }
                        input {
                            id: "forgot-email",
                            class: "input w-full",
                            r#type: "email",
                            required: true,
                            autocomplete: "email",
                            placeholder: tid!("field-email-placeholder"),
                            value: "{email}",
                            oninput: move |e| email.set(e.value()),
                        }
                    }
                    button {
                        id: "forgot-submit",
                        class: "btn btn-primary w-full",
                        r#type: "submit",
                        disabled: loading(),
                        {tid!("forgot-submit")}
                    }
                }
            }
        }
    }
}
