use dioxus::prelude::*;
use shared::Account;

use crate::auth::account_key::remember;
use crate::auth::recovery_flow::{recover_with_key, request_reset, start};
use crate::auth::recovery_ui::format_moment;
use crate::auth::security_card::change_password_error;
use crate::common::{error_message, AppHeader, Flash, LocalStorageState};
use crate::route::Route;
use crate::tid;

/// The mailed link (docs/plans/key-lifecycle.md §6.2, §6.3): the recovery key gets everything back
/// at once; without it, a fresh account 72 hours later.
#[component]
pub fn ResetPage(token: String) -> Element {
    let link = token.clone();
    let started = use_resource(move || {
        let token = link.clone();
        async move { start(token).await }
    });
    let mut without_key = use_signal(|| false);

    rsx! {
        div { class: "container app-container bg-base-100 p-4 max-w-md w-full mx-auto flex flex-col gap-5 overflow-auto pb-24",
            AppHeader { title: tid!("reset-title"), back_button_route: Route::LoginPage {} }
            match started() {
                None => rsx! {
                    div { class: "skeleton h-48 w-full", aria_hidden: "true" }
                },
                Some(Err(e)) => rsx! {
                    div { id: "reset-error", role: "alert", class: "alert alert-error", {error_message(&e)} }
                },
                Some(Ok(info)) => rsx! {
                    if let Some(at) = info.pending_reset_at {
                        div { role: "status", class: "alert alert-warning text-sm", {tid!("reset-pending-already", when: format_moment(at))} }
                    }
                    if info.has_recovery_key && !without_key() {
                        WithKeyForm { token: token.clone(), info: info.clone() }
                        button {
                            id: "reset-no-key",
                            r#type: "button",
                            class: "btn btn-ghost btn-sm self-center",
                            onclick: move |_| without_key.set(true),
                            {tid!("reset-no-key")}
                        }
                    } else {
                        WithoutKeyForm { token: token.clone(), info: info.clone() }
                    }
                },
            }
        }
    }
}

#[component]
fn WithKeyForm(token: String, info: shared::RecoveryStart) -> Element {
    let nav = use_navigator();
    let mut auth = use_context::<Signal<Option<Account>>>();
    let mut key_ctx = use_context::<Signal<Option<[u8; 32]>>>();
    let ls_ctx = use_context::<Signal<LocalStorageState>>();
    let mut flash = use_context::<Signal<Option<Flash>>>();
    let mut key = use_signal(String::new);
    let new = use_signal(String::new);
    let confirm = use_signal(String::new);
    let mut error: Signal<Option<String>> = use_signal(|| None);
    let mut busy = use_signal(|| false);

    let on_submit = move |e: FormEvent| {
        e.prevent_default();
        if let Some(problem) = change_password_error(&new(), &confirm()) {
            error.set(Some(tid!(problem)));
            return;
        }
        let (token, info, key_text, password) = (token.clone(), info.clone(), key(), new());
        spawn(async move {
            busy.set(true);
            error.set(None);
            match recover_with_key(token, &info, &key_text, &password).await {
                Ok((account, master)) => {
                    remember(ls_ctx, &account, &master);
                    key_ctx.set(Some(master));
                    auth.set(Some(account));
                    flash.set(Some(Flash::ok(tid!("recovered-done"))));
                    nav.replace(Route::ProjectsPage {});
                }
                Err(e) => {
                    error.set(Some(error_message(&e)));
                    busy.set(false);
                }
            }
        });
    };

    rsx! {
        form { id: "reset-with-key", class: "flex flex-col gap-4", onsubmit: on_submit,
            p { class: "text-sm text-base-content/70", {tid!("reset-with-key-body")} }
            if let Some(err) = error() {
                div { role: "alert", class: "alert alert-error text-sm", "{err}" }
            }
            fieldset { class: "fieldset",
                label { class: "fieldset-legend", r#for: "reset-key", {tid!("recovery-key-field")} }
                textarea {
                    id: "reset-key",
                    class: "textarea w-full font-mono uppercase",
                    rows: "3",
                    autocomplete: "off",
                    autocapitalize: "characters",
                    autocorrect: "off",
                    spellcheck: "false",
                    required: true,
                    value: "{key}",
                    oninput: move |e| key.set(e.value()),
                }
                PasswordPair { new, confirm }
            }
            button { id: "reset-submit-key", class: "btn btn-primary w-full", r#type: "submit", disabled: busy(),
                if busy() {
                    span { class: "loading loading-spinner loading-sm", role: "status", aria_label: tid!("loading") }
                }
                {tid!("reset-submit-key")}
            }
        }
    }
}

#[component]
fn WithoutKeyForm(token: String, info: shared::RecoveryStart) -> Element {
    let mut name = use_signal(String::new);
    let new = use_signal(String::new);
    let confirm = use_signal(String::new);
    let mut error: Signal<Option<String>> = use_signal(|| None);
    let mut busy = use_signal(|| false);
    let mut done: Signal<Option<chrono::NaiveDateTime>> = use_signal(|| None);

    let on_submit = move |e: FormEvent| {
        e.prevent_default();
        if let Some(problem) = change_password_error(&new(), &confirm()) {
            error.set(Some(tid!(problem)));
            return;
        }
        let (token, info, name, password) = (token.clone(), info.clone(), name(), new());
        spawn(async move {
            busy.set(true);
            error.set(None);
            match request_reset(token, &info, &name, &password).await {
                Ok(at) => done.set(Some(at)),
                Err(e) => error.set(Some(error_message(&e))),
            }
            busy.set(false);
        });
    };

    rsx! {
        if let Some(at) = done() {
            div { id: "reset-requested", role: "status", class: "alert alert-success", {tid!("reset-requested", when: format_moment(at))} }
        } else {
            form { id: "reset-without-key", class: "flex flex-col gap-4", onsubmit: on_submit,
                h2 { class: "font-semibold", {tid!("reset-without-key-title")} }
                p { class: "text-sm text-base-content/70", {tid!("reset-without-key-body")} }
                if let Some(err) = error() {
                    div { role: "alert", class: "alert alert-error text-sm", "{err}" }
                }
                fieldset { class: "fieldset",
                    label { class: "fieldset-legend", r#for: "reset-name", {tid!("reset-name")} }
                    input {
                        id: "reset-name",
                        class: "input w-full",
                        required: true,
                        autocomplete: "name",
                        autocapitalize: "words",
                        autocorrect: "off",
                        spellcheck: "false",
                        value: "{name}",
                        oninput: move |e| name.set(e.value()),
                    }
                    PasswordPair { new, confirm }
                }
                button { id: "reset-submit-request", class: "btn btn-warning w-full", r#type: "submit", disabled: busy(),
                    {tid!("reset-submit-request")}
                }
            }
        }
    }
}

#[component]
fn PasswordPair(new: Signal<String>, confirm: Signal<String>) -> Element {
    rsx! {
        label { class: "fieldset-legend", r#for: "reset-new", {tid!("change-password-new")} }
        input {
            id: "reset-new",
            class: "input w-full",
            r#type: "password",
            required: true,
            minlength: "12",
            autocomplete: "new-password",
            value: "{new}",
            oninput: move |e| new.set(e.value()),
        }
        label { class: "fieldset-legend", r#for: "reset-confirm", {tid!("change-password-confirm")} }
        input {
            id: "reset-confirm",
            class: "input w-full",
            r#type: "password",
            required: true,
            autocomplete: "new-password",
            value: "{confirm}",
            oninput: move |e| confirm.set(e.value()),
        }
    }
}
