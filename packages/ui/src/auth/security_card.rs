use dioxus::prelude::*;
use shared::Account;

use crate::auth::account_key::remember;
use crate::auth::account_rotation::{change_password, compromise_rotation, Rotated};
use crate::auth::credentials::password_error_key;
use crate::common::{error_message, haptic, Flash, Haptic, LocalStorageState};
use crate::tid;

/// Password, recovery and key rotation (docs/plans/key-lifecycle.md). Shown only once the floor
/// guarantees every client reads the account master key (`Account.key_lifecycle`).
#[derive(Clone, Copy, PartialEq)]
enum Sheet {
    ChangePassword,
    Compromised,
}

#[component]
pub fn SecurityCard(account: Account) -> Element {
    let mut sheet: Signal<Option<Sheet>> = use_signal(|| None);
    let mut new_recovery_key: Signal<Option<String>> = use_signal(|| None);
    let email = account.email.clone();

    rsx! {
        div { id: "security-card", class: "card bg-base-100 shadow-soft",
            div { class: "card-body gap-2",
                h2 { class: "card-title text-base", {tid!("settings-security")} }
                ul { class: "list",
                    crate::auth::recovery_ui::RecoveryKeyRow { account: account.clone() }
                    li { class: "list-row items-center",
                        div { class: "list-col-grow flex flex-col gap-1",
                            span { class: "font-medium", {tid!("settings-change-password")} }
                            span { class: "text-sm text-base-content/70", {tid!("settings-change-password-hint")} }
                        }
                        button {
                            id: "change-password-open",
                            r#type: "button",
                            class: "btn btn-sm",
                            onclick: move |_| {
                                haptic(Haptic::Light);
                                sheet.set(Some(Sheet::ChangePassword));
                            },
                            {tid!("settings-change-password")}
                        }
                    }
                    li { class: "list-row items-center",
                        div { class: "list-col-grow flex flex-col gap-1",
                            span { class: "font-medium", {tid!("settings-compromised")} }
                            span { class: "text-sm text-base-content/70", {tid!("settings-compromised-hint")} }
                        }
                        button {
                            id: "compromised-open",
                            r#type: "button",
                            class: "btn btn-sm btn-error btn-outline",
                            onclick: move |_| {
                                haptic(Haptic::Light);
                                sheet.set(Some(Sheet::Compromised));
                            },
                            {tid!("settings-compromised-open")}
                        }
                    }
                }
            }
        }
        if let Some(open) = sheet() {
            ChangePasswordSheet {
                compromise: open == Sheet::Compromised,
                on_close: move |_| sheet.set(None),
                on_new_recovery_key: move |key: String| new_recovery_key.set(Some(key)),
            }
        }
        if let Some(key_text) = new_recovery_key() {
            NewRecoveryKeySheet { key_text, email: email.clone(), on_close: move |_| new_recovery_key.set(None) }
        }
    }
}

/// The form's own check, before any Argon2 run: a key, never a sentence.
pub fn change_password_error(new: &str, confirm: &str) -> Option<&'static str> {
    password_error_key(new).or((new != confirm).then_some("change-password-mismatch"))
}

/// The same form for both: a compromise rotation is a password change that also replaces the
/// master key and the keypair.
#[component]
fn ChangePasswordSheet(compromise: bool, on_close: EventHandler<()>, on_new_recovery_key: EventHandler<String>) -> Element {
    crate::common::use_modal_focus("change-password-sheet");
    let auth = use_context::<Signal<Option<Account>>>();
    let key_ctx = use_context::<Signal<Option<[u8; 32]>>>();
    let ls_ctx = use_context::<Signal<LocalStorageState>>();
    let mut flash = use_context::<Signal<Option<Flash>>>();
    let mut current = use_signal(String::new);
    let mut new = use_signal(String::new);
    let mut confirm = use_signal(String::new);
    let mut error: Signal<Option<String>> = use_signal(|| None);
    let mut saving = use_signal(|| false);

    let on_submit = move |e: FormEvent| {
        e.prevent_default();
        if let Some(key) = change_password_error(&new(), &confirm()) {
            error.set(Some(tid!(key)));
            return;
        }
        let (current_val, new_val) = (current(), new());
        spawn(async move {
            saving.set(true);
            error.set(None);
            let changed = if compromise {
                compromise_rotation(&current_val, &new_val).await
            } else {
                change_password(&current_val, &new_val).await
            };
            match changed {
                Ok(Rotated { account, master, recovery_key }) => {
                    let (mut auth, mut key_ctx) = (auth, key_ctx);
                    remember(ls_ctx, &account, &master);
                    key_ctx.set(Some(master));
                    auth.set(Some(account));
                    haptic(Haptic::Success);
                    let done = if compromise { tid!("compromised-done") } else { tid!("change-password-done") };
                    flash.set(Some(Flash::ok(done)));
                    on_close.call(());
                    if let Some(key) = recovery_key {
                        on_new_recovery_key.call(key);
                    }
                }
                Err(e) => {
                    error.set(Some(error_message(&e)));
                    saving.set(false);
                }
            }
        });
    };

    rsx! {
        div {
            id: "change-password-sheet",
            class: "modal modal-open modal-bottom sm:modal-middle",
            role: "dialog",
            aria_modal: "true",
            aria_labelledby: "change-password-title",
            tabindex: "-1",
            div { class: "modal-box max-w-md",
                h2 { id: "change-password-title", class: "font-bold text-lg font-display mb-3",
                    if compromise {
                        {tid!("compromised-title")}
                    } else {
                        {tid!("change-password-title")}
                    }
                }
                if compromise {
                    div { class: "flex flex-col gap-2 mb-3 text-sm text-base-content/70",
                        p { {tid!("compromised-body")} }
                        p { {tid!("compromised-keeps")} }
                    }
                }
                form { class: "flex flex-col gap-3", onsubmit: on_submit,
                    if let Some(err) = error() {
                        div { id: "change-password-error", role: "alert", class: "alert alert-error text-sm", "{err}" }
                    }
                    fieldset { class: "fieldset",
                        label { class: "fieldset-legend", r#for: "change-password-current", {tid!("change-password-current")} }
                        input {
                            id: "change-password-current",
                            class: "input w-full",
                            r#type: "password",
                            required: true,
                            autocomplete: "current-password",
                            value: "{current}",
                            oninput: move |e| current.set(e.value()),
                        }
                        label { class: "fieldset-legend", r#for: "change-password-new", {tid!("change-password-new")} }
                        input {
                            id: "change-password-new",
                            class: "input w-full",
                            r#type: "password",
                            required: true,
                            minlength: "12",
                            autocomplete: "new-password",
                            value: "{new}",
                            oninput: move |e| new.set(e.value()),
                        }
                        label { class: "fieldset-legend", r#for: "change-password-confirm", {tid!("change-password-confirm")} }
                        input {
                            id: "change-password-confirm",
                            class: "input w-full",
                            r#type: "password",
                            required: true,
                            autocomplete: "new-password",
                            value: "{confirm}",
                            oninput: move |e| confirm.set(e.value()),
                        }
                        if !compromise {
                            p { class: "label whitespace-normal", {tid!("settings-change-password-hint")} }
                        }
                    }
                    div { class: "modal-action mt-0",
                        button {
                            id: "change-password-cancel",
                            r#type: "button",
                            class: "btn",
                            disabled: saving(),
                            onclick: move |_| on_close.call(()),
                            {tid!("cancel")}
                        }
                        button {
                            id: "change-password-submit",
                            r#type: "submit",
                            class: if compromise { "btn btn-error" } else { "btn btn-primary" },
                            disabled: saving(),
                            if saving() {
                                span { class: "loading loading-spinner loading-sm", role: "status", aria_label: tid!("loading") }
                                {tid!("change-password-submitting")}
                            } else if compromise {
                                {tid!("compromised-submit")}
                            } else {
                                {tid!("change-password-submit")}
                            }
                        }
                    }
                }
            }
            div { class: "modal-backdrop", onclick: move |_| if !saving() { on_close.call(()) } }
        }
    }
}

/// After a compromise rotation: the replacement recovery key, shown once, in effect in 72 hours.
#[component]
fn NewRecoveryKeySheet(key_text: String, email: String, on_close: EventHandler<()>) -> Element {
    crate::common::use_modal_focus("new-recovery-key-sheet");
    rsx! {
        div {
            id: "new-recovery-key-sheet",
            class: "modal modal-open modal-bottom sm:modal-middle",
            role: "dialog",
            aria_modal: "true",
            aria_labelledby: "new-recovery-key-title",
            tabindex: "-1",
            div { class: "modal-box max-w-md flex flex-col gap-3",
                h2 { id: "new-recovery-key-title", class: "font-bold text-lg font-display", {tid!("recovery-key-title")} }
                p { class: "text-sm", {tid!("compromised-new-key")} }
                crate::auth::recovery_ui::KeyDisplay { key_text, email }
                div { class: "modal-action mt-0",
                    button { id: "new-recovery-key-done", r#type: "button", class: "btn btn-primary", onclick: move |_| on_close.call(()), {tid!("recovery-key-next")} }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_form_checks_length_then_confirmation() {
        assert_eq!(change_password_error("short", "short"), Some("error-password-too-short"));
        assert_eq!(change_password_error("long enough pass", "long enough pasS"), Some("change-password-mismatch"));
        assert_eq!(change_password_error("long enough pass", "long enough pass"), None);
    }

    #[test]
    fn its_keys_exist_in_the_fallback_locale() {
        let known = crate::i18n::locale_ids(crate::i18n::FALLBACK);
        assert!(known.contains("change-password-mismatch"));
    }
}
