use dioxus::prelude::*;
use crate::tid;
use shared::Account;

use crate::auth::credentials::{remember_account_key, verify_and_sign_in};
use crate::common::{error_key, error_message, next_paint, Flash, LocalStorageState};
use crate::preferences::sync_personal_project;
use crate::route::Route;

#[component]
pub fn VerifyEmailPage(token: String) -> Element {
    let mut password = use_signal(String::new);
    let mut error_msg: Signal<Option<String>> = use_signal(|| None);
    let mut link_dead = use_signal(|| false);
    let mut loading = use_signal(|| false);
    let mut auth_ctx = use_context::<Signal<Option<Account>>>();
    let mut account_enc_key_ctx = use_context::<Signal<Option<[u8; 32]>>>();
    let mut ls_ctx = use_context::<Signal<LocalStorageState>>();
    let mut flash = use_context::<Signal<Option<Flash>>>();
    let nav = use_navigator();

    let on_submit = move |e: FormEvent| {
        e.prevent_default();
        let token_val = token.clone();
        let password_val = password();
        async move {
            loading.set(true);
            error_msg.set(None);
            next_paint().await;
            match verify_and_sign_in(token_val, &password_val).await {
                Ok((account, account_key)) => {
                    ls_ctx.set(remember_account_key(&account_key));
                    let account = sync_personal_project(account, account_key, ls_ctx).await;
                    account_enc_key_ctx.set(Some(account_key));
                    auth_ctx.set(Some(account));
                    flash.set(Some(Flash::ok(tid!("verify-email-welcome"))));
                    nav.push(Route::ProjectsPage {});
                }
                Err(e) => {
                    link_dead.set(error_key(&e) == "error-invalid-link");
                    error_msg.set(Some(error_message(&e)));
                    loading.set(false);
                }
            }
        }
    };

    rsx! {
        div { class: "container app-container bg-base-100 p-4 max-w-md w-full mx-auto flex flex-col gap-6 mt-16",
            if let Some(err) = error_msg() {
                div { id: "verify-email-error", role: "alert", class: "alert alert-error", "{err}" }
            }
            if link_dead() {
                Link {
                    class: "btn btn-outline w-full",
                    to: Route::LoginPage {},
                    {tid!("verify-email-back-to-login")}
                }
            } else {
                p { class: "text-base-content/70", {tid!("verify-email-intro")} }
                form { class: "flex flex-col gap-4", onsubmit: on_submit,
                    fieldset { class: "fieldset",
                        label { class: "fieldset-legend", r#for: "verify-email-password", {tid!("field-password")} }
                        input {
                            id: "verify-email-password",
                            class: "input w-full",
                            r#type: "password",
                            required: true,
                            placeholder: tid!("login-password-placeholder"),
                            autocomplete: "current-password",
                            aria_describedby: if error_msg().is_some() { "verify-email-error" },
                            value: "{password}",
                            oninput: move |e| password.set(e.value()),
                        }
                    }
                    button {
                        id: "verify-email-submit",
                        class: "btn btn-primary w-full",
                        r#type: "submit",
                        disabled: loading(),
                        if loading() {
                            span { class: "loading loading-spinner loading-sm", role: "status", aria_label: tid!("loading") }
                            {tid!("verify-email-checking")}
                        } else {
                            {tid!("verify-email-submit")}
                        }
                    }
                }
            }
        }
    }
}
