use dioxus::prelude::*;
use crate::tid;

use crate::auth::credentials::{password_error_key, sign_up};
use crate::common::{error_message, next_paint, read_from_ls, AppHeader, Mascot, MascotPose};
use crate::crypto::{derive_account_key_v1, encrypt_json, generate_kdf_salt};
use crate::i18n::current_lang;
use crate::icons::{LockIcon, ICON_INLINE};
use crate::route::Route;

#[component]
pub fn RegisterPage() -> Element {
    let demo = crate::demo::use_demo_mode();
    rsx! {
        if demo {
            crate::demo::DemoSignInBlocked {}
        } else {
            RegisterForm {}
        }
    }
}

#[component]
fn RegisterForm() -> Element {
    let mut email = use_signal(String::new);
    let mut password = use_signal(String::new);
    let mut display_name = use_signal(String::new);
    let mut error_msg: Signal<Option<String>> = use_signal(|| None);
    let mut loading = use_signal(|| false);
    let mut email_sent = use_signal(|| false);

    let on_submit = move |e: FormEvent| {
        e.prevent_default();
        let email_val = email();
        let password_val = password();
        let display_name_val = display_name();
        let lang = current_lang();

        async move {
            if let Some(key) = password_error_key(&password_val) {
                error_msg.set(Some(tid!(key)));
                return;
            }
            loading.set(true);
            error_msg.set(None);
            // The KDF below blocks the thread — without this the spinner paints only once it is over.
            next_paint().await;

            let kdf_salt = generate_kdf_salt();
            let account_key = derive_account_key_v1(&password_val, &kdf_salt);
            let encrypted_display_name = match encrypt_json(&account_key, &display_name_val) {
                Ok(p) => p,
                Err(e) => {
                    error_msg.set(Some(e));
                    loading.set(false);
                    return;
                }
            };

            let had_anonymous_membership = read_from_ls()
                .projects
                .iter()
                .any(|p| p.anon_member_id.is_some());

            match sign_up(
                email_val,
                &password_val,
                &account_key,
                encrypted_display_name,
                kdf_salt,
                had_anonymous_membership,
                lang,
            )
            .await
            {
                Ok(()) => {
                    email_sent.set(true);
                }
                Err(e) => {
                    error_msg.set(Some(error_message(&e)));
                    loading.set(false);
                }
            }
        }
    };

    if email_sent() {
        return rsx! {
            div { class: "container app-container bg-base-100 p-4 max-w-md w-full mx-auto flex flex-col gap-6 overflow-auto pb-24",
                AppHeader {
                    title: tid!("register-check-email-title"),
                    back_button_route: Route::ProjectsPage {},
                }
                div { class: "flex flex-col items-center gap-4 py-4 text-center",
                    Mascot { pose: MascotPose::Mail }
                    div { class: "flex flex-col gap-1",
                        p { class: "font-semibold", {tid!("register-email-sent")} }
                        p { class: "text-sm text-base-content/70",
                            {tid!("register-email-sent-hint")}
                        }
                    }
                }
                div { class: "divider" }
                // Split around the link: Fluent has no way to embed a component in a message, so
                // each half is its own key. Word order differs by language, hence the two halves
                // rather than one sentence with a placeholder.
                p { class: "text-center text-sm text-base-content/70",
                    {tid!("register-not-received-prefix")}
                    " "
                    Link { class: "link link-primary", to: Route::LoginPage {}, {tid!("register-sign-in-link")} }
                    " "
                    {tid!("register-not-received-suffix")}
                }
            }
        };
    }

    rsx! {
        div { class: "container app-container bg-base-100 p-4 max-w-md w-full mx-auto flex flex-col gap-5 overflow-auto pb-24",
            AppHeader {
                title: tid!("register-submit"),
                back_button_route: Route::ProjectsPage {},
            }

            if let Some(err) = error_msg() {
                div { role: "alert", class: "alert alert-error", "{err}" }
            }

            form { class: "flex flex-col gap-4", onsubmit: on_submit,
                fieldset { class: "fieldset", disabled: loading(),
                    label { class: "fieldset-legend", r#for: "register-name", {tid!("field-name")} }
                    input {
                        id: "register-name",
                        class: "input w-full",
                        r#type: "text",
                        enterkeyhint: "next",
                        required: true,
                        placeholder: tid!("field-name-placeholder"),
                        autocomplete: "name",
                        value: "{display_name}",
                        oninput: move |e| display_name.set(e.value()),
                    }

                    label { class: "fieldset-legend", r#for: "register-email", {tid!("field-email")} }
                    input {
                        id: "register-email",
                        class: "input w-full",
                        r#type: "email",
                        enterkeyhint: "next",
                        required: true,
                        placeholder: tid!("field-email-placeholder"),
                        autocomplete: "email",
                        value: "{email}",
                        oninput: move |e| email.set(e.value()),
                    }

                    label { class: "fieldset-legend", r#for: "register-password", {tid!("field-password")} }
                    input {
                        id: "register-password",
                        class: "input w-full",
                        r#type: "password",
                        required: true,
                        minlength: "8",
                        maxlength: "128",
                        placeholder: tid!("register-password-placeholder"),
                        autocomplete: "new-password",
                        value: "{password}",
                        oninput: move |e| password.set(e.value()),
                    }
                }
                div { class: "alert alert-info alert-soft text-sm",
                    LockIcon { size: ICON_INLINE }
                    span {
                        {tid!("register-password-warning")}
                    }
                }
                button {
                    id: "register-submit",
                    class: "btn btn-primary w-full",
                    r#type: "submit",
                    disabled: loading(),
                    if loading() {
                        span { class: "loading loading-spinner loading-sm", role: "status", aria_label: tid!("loading") }
                        {tid!("register-submitting")}
                    } else {
                        {tid!("register-submit")}
                    }
                }
            }

            div { class: "divider" }
            p { class: "text-center text-sm",
                {tid!("register-have-account")}
                " "
                Link { class: "link link-primary", to: Route::LoginPage {}, {tid!("login-submit")} }
            }
            p { class: "text-center text-xs text-base-content/60",
                {tid!("register-terms-prefix")}
                " "
                Link { class: "link", to: Route::TermsPage {}, {tid!("register-terms-link")} }
                " "
                {tid!("register-terms-and")}
                " "
                Link { class: "link", to: Route::PrivacyPage {}, {tid!("register-privacy-link")} }
            }
        }
    }
}
