use api::auth::auth_controller::resend_verification;
use dioxus::fullstack::Json;
use dioxus::prelude::*;
use crate::tid;
use shared::{Account, ResendVerificationPayload};

use crate::{
    auth::credentials::sign_in,
    common::{
        error_message, is_email_not_verified, next_paint, read_from_ls, write_to_ls, AppHeader,
        LocalStorageState,
    },
    crypto::key_to_fragment,
    i18n::current_lang,
    route::Route,
};

#[component]
pub fn LoginPage() -> Element {
    let mut email = use_signal(String::new);
    let mut password = use_signal(String::new);
    let mut error_msg: Signal<Option<String>> = use_signal(|| None);
    let mut loading = use_signal(|| false);
    let mut unverified = use_signal(|| false);
    let mut resend_loading = use_signal(|| false);
    let mut resend_done = use_signal(|| false);

    let nav = use_navigator();
    let mut auth_ctx = use_context::<Signal<Option<Account>>>();
    let mut account_enc_key_ctx = use_context::<Signal<Option<[u8; 32]>>>();
    let mut ls_ctx = use_context::<Signal<LocalStorageState>>();

    let on_submit = move |e: FormEvent| {
        e.prevent_default();
        let email_val = email();
        let password_val = password();
        let lang = current_lang();

        async move {
            loading.set(true);
            error_msg.set(None);
            unverified.set(false);
            resend_done.set(false);
            // Two Argon2id derivations run before the request now; without a paint in between
            // the spinner never shows.
            next_paint().await;

            match sign_in(email_val, &password_val, lang).await {
                Ok((account, account_key)) => {
                    // Persist so the key survives a reload — the password is gone once the
                    // session is restored from the cookie.
                    let mut state = read_from_ls();
                    state.account_key = Some(key_to_fragment(&account_key));
                    write_to_ls(&state);
                    ls_ctx.set(state);

                    account_enc_key_ctx.set(Some(account_key));
                    auth_ctx.set(Some(account));
                    nav.push("/");
                }
                Err(e) => {
                    // Matched on the error, not on the rendered text: that text is translated, so
                    // a `contains(EMAIL_NOT_VERIFIED)` would only ever match in English.
                    if is_email_not_verified(&e) {
                        unverified.set(true);
                        error_msg.set(None);
                    } else {
                        error_msg.set(Some(error_message(&e)));
                    }
                    loading.set(false);
                }
            }
        }
    };

    let on_resend = move |_| {
        let email_val = email();
        async move {
            resend_loading.set(true);
            resend_done.set(false);
            match resend_verification(Json(ResendVerificationPayload { email: email_val })).await {
                Ok(()) => resend_done.set(true),
                Err(e) => error_msg.set(Some(error_message(&e))),
            }
            resend_loading.set(false);
        }
    };

    rsx! {
        div { class: "container app-container bg-base-100 p-4 max-w-md w-full mx-auto flex flex-col gap-5 overflow-auto pb-24",
            AppHeader { title: tid!("login-title"), back_button_route: Route::ProjectsPage {} }

            if let Some(err) = error_msg() {
                div { role: "alert", class: "alert alert-error", "{err}" }
            }

            if unverified() {
                div { role: "alert", class: "alert alert-warning flex flex-col items-start gap-2",
                    p { {tid!("login-unverified")} }
                    if resend_done() {
                        p { class: "text-sm font-medium", {tid!("login-resend-sent")} }
                    } else {
                        button {
                            class: "btn btn-sm btn-outline",
                            r#type: "button",
                            disabled: resend_loading(),
                            onclick: on_resend,
                            if resend_loading() {
                                span { class: "loading loading-spinner loading-sm", role: "status", aria_label: tid!("loading") }
                                {tid!("login-resending")}
                            } else {
                                {tid!("login-resend")}
                            }
                        }
                    }
                }
            }

            form { class: "flex flex-col gap-4", onsubmit: on_submit,
                fieldset { class: "fieldset",
                    label { class: "fieldset-legend", r#for: "login-email", {tid!("field-email")} }
                    input {
                        id: "login-email",
                        class: "input w-full",
                        r#type: "email",
                        enterkeyhint: "next",
                        required: true,
                        placeholder: tid!("field-email-placeholder"),
                        autocomplete: "email",
                        value: "{email}",
                        oninput: move |e| email.set(e.value()),
                    }

                    label { class: "fieldset-legend", r#for: "login-password", {tid!("field-password")} }
                    input {
                        id: "login-password",
                        class: "input w-full",
                        r#type: "password",
                        required: true,
                        placeholder: tid!("login-password-placeholder"),
                        autocomplete: "current-password",
                        value: "{password}",
                        oninput: move |e| password.set(e.value()),
                    }
                }
                button {
                    id: "login-submit",
                    class: "btn btn-primary w-full",
                    r#type: "submit",
                    disabled: loading(),
                    if loading() {
                        span { class: "loading loading-spinner loading-sm", role: "status", aria_label: tid!("loading") }
                        {tid!("login-submitting")}
                    } else {
                        {tid!("login-submit")}
                    }
                }
            }

            div { class: "divider" }
            p { class: "text-center text-sm",
                {tid!("login-no-account")}
                " "
                Link {
                    class: "link link-primary",
                    to: Route::RegisterPage {},
                    {tid!("register-submit")}
                }
            }
        }
    }
}
