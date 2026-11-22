use dioxus::prelude::*;
use crate::tid;

use crate::auth::credentials::{account_key, nfc, password_error_key, sign_up};
use crate::common::{
    error_message, next_paint, read_from_ls, take_invited_email, AppHeader, AuthResolved, Mascot, MascotPose,
};
use crate::crypto::{encrypt_padded_json, generate_kdf_salt};
use crate::i18n::current_lang;
use crate::icons::{CheckIcon, LockIcon, ICON_INLINE};
use crate::route::Route;
use shared::Account;

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

/// Fills the address an invitation link carried, once, after hydration and the session check.
/// Already signed in, the request is waiting in Amis.
fn use_invitation(mut email: Signal<String>) -> Signal<bool> {
    let mut invited = use_signal(|| false);
    let auth_ctx = use_context::<Signal<Option<Account>>>();
    let auth_resolved = use_context::<Signal<AuthResolved>>();
    let nav = use_navigator();
    use_effect(move || {
        if !auth_resolved().0 {
            return;
        }
        let Some(address) = take_invited_email() else { return };
        if auth_ctx.peek().is_some() {
            nav.replace(Route::FriendsPage {});
            return;
        }
        email.set(address);
        invited.set(true);
    });
    invited
}

#[component]
fn RegisterForm() -> Element {
    let mut email = use_signal(String::new);
    let invited = use_invitation(email);
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
            // The KDF below blocks the thread - without this the spinner paints only once it is over.
            next_paint().await;

            let kdf_salt = generate_kdf_salt();
            let account_key = match account_key(&nfc(&password_val), kdf_salt.to_vec()).await {
                Ok(k) => k,
                Err(e) => {
                    error_msg.set(Some(error_message(&e)));
                    loading.set(false);
                    return;
                }
            };
            let encrypted_display_name = match encrypt_padded_json(&account_key, &display_name_val) {
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

            if invited() {
                div { id: "register-invited", class: "alert alert-success alert-soft flex flex-col items-start gap-1",
                    p { class: "font-semibold", {tid!("register-invited-title")} }
                    p { class: "text-sm", {tid!("register-invited-body")} }
                }
            }

            if let Some(err) = error_msg() {
                div { id: "register-error", role: "alert", class: "alert alert-error", "{err}" }
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
                        autocapitalize: "words",
                        autocorrect: "off",
                        spellcheck: "false",
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
                        aria_describedby: if invited() { "register-email-hint" },
                        value: "{email}",
                        oninput: move |e| email.set(e.value()),
                    }
                    if invited() {
                        p { id: "register-email-hint", class: "text-xs text-base-content/70",
                            {tid!("register-email-from-link")}
                        }
                    }

                    label { class: "fieldset-legend", r#for: "register-password", {tid!("field-password")} }
                    input {
                        id: "register-password",
                        class: "input w-full",
                        r#type: "password",
                        required: true,
                        minlength: "{shared::MIN_PASSWORD_LENGTH}",
                        maxlength: "128",
                        placeholder: tid!("register-password-placeholder"),
                        autocomplete: "new-password",
                        aria_invalid: if error_msg().is_some() && password_error_key(&password()).is_some() { "true" },
                        aria_describedby: if error_msg().is_some() { "register-error" },
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

            if invited() {
                div { id: "register-perks", class: "rounded-box bg-base-200 p-4 flex flex-col gap-2 text-sm",
                    p { class: "text-xs font-semibold uppercase tracking-wide text-base-content/60",
                        {tid!("register-perks-title")}
                    }
                    for perk in [tid!("settings-upsell-title"), tid!("register-perk-friends"), tid!("register-perk-payment")] {
                        div { class: "flex gap-2 items-start",
                            span { class: "text-success", CheckIcon { size: ICON_INLINE } }
                            span { "{perk}" }
                        }
                    }
                }
                div { class: "flex flex-col items-center gap-1 text-center text-xs text-base-content/60",
                    Link { class: "link", to: Route::ProjectsPage {}, {tid!("register-continue-without")} }
                    p { {tid!("register-continue-without-hint")} }
                }
            }
        }
    }
}
