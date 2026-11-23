use dioxus::prelude::*;
use shared::Account;

use crate::auth::account_key::remember;
use crate::auth::account_rotation::{change_password, compromise_rotation, Rotated};
use crate::auth::credentials::password_error_key;
use crate::common::{error_message, haptic, Flash, Haptic, ListFooter, ListRow, LocalStorageState, RowTile, SheetBar, SheetFooter, SHEET_TITLE};
use crate::icons::LockIcon;
use crate::tid;

/// Password, recovery and key rotation (docs/plans/key-lifecycle.md). Shown only once the floor
/// guarantees every client reads the account master key (`Account.key_lifecycle`).
#[component]
pub fn SecurityCard(account: Account) -> Element {
    let mut sheet_open = use_signal(|| false);
    let mut new_recovery_key: Signal<Option<String>> = use_signal(|| None);
    let email = account.email.clone();
    let has_recovery_key = account.has_recovery_key;

    let footnote = match account.recovery_pending_at {
        Some(at) => Some(("recovery-key-pending", tid!("settings-recovery-key-pending", when: crate::auth::recovery_ui::format_moment(at)))),
        None if !account.has_recovery_key => Some(("recovery-key-missing", tid!("settings-recovery-key-footnote"))),
        None => None,
    };

    rsx! {
        section { id: "security-card", class: "flex flex-col",
            h2 { class: "list-header", {tid!("settings-security")} }
            ul { class: "list-group", style: "--separator-inset: 3.75rem",
                crate::auth::recovery_ui::RecoveryKeyRow { account: account.clone() }
                ListRow {
                    id: "change-password-open",
                    title: tid!("settings-password"),
                    chevron: true,
                    onclick: move |_| sheet_open.set(true),
                    RowTile { tint: "bg-base-200 text-base-content/70", LockIcon {} }
                }
            }
            if let Some((id, footnote)) = footnote {
                ListFooter { id, "{footnote}" }
            }
        }
        if sheet_open() {
            ChangePasswordSheet {
                has_recovery_key,
                on_close: move |_| sheet_open.set(false),
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

/// The switch turns the password change into a compromise rotation, which also replaces the
/// master key and the keypair.
#[component]
fn ChangePasswordSheet(has_recovery_key: bool, on_close: EventHandler<()>, on_new_recovery_key: EventHandler<String>) -> Element {
    crate::common::use_modal_focus("change-password-sheet");
    let mut compromise = use_signal(|| false);
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
        let (current_val, new_val, compromise) = (current(), new(), compromise());
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
    let compromising = compromise();

    rsx! {
        div {
            id: "change-password-sheet",
            class: "modal modal-open modal-bottom sm:modal-middle",
            role: "dialog",
            aria_modal: "true",
            aria_labelledby: "change-password-title",
            tabindex: "-1",
            div { class: "modal-box max-w-md p-0 flex flex-col",
                SheetBar { close_id: "change-password-close", task: true, on_close: move |_| if !saving() { on_close.call(()) },
                    h2 { id: "change-password-title", class: SHEET_TITLE,
                        if compromising {
                            {tid!("compromised-title")}
                        } else {
                            {tid!("change-password-title")}
                        }
                    }
                }
                form { class: "flex flex-col flex-1 min-h-0", onsubmit: on_submit,
                    div { class: "flex-1 overflow-y-auto flex flex-col gap-3 px-5 pt-1 pb-4",
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
                        if !compromising {
                            p { class: "label whitespace-normal", {tid!("settings-change-password-hint")} }
                        }
                    }
                    label {
                        class: if compromising { "flex items-start gap-3 px-4 py-3 rounded-field cursor-pointer bg-tile-red" } else { "flex items-start gap-3 px-4 py-3 rounded-field cursor-pointer bg-base-200" },
                        span { class: "flex-1 min-w-0",
                            span { class: "block font-medium", {tid!("compromised-switch")} }
                            span {
                                id: "compromised-switch-hint",
                                class: if compromising { "block text-sm leading-snug text-tile-red-content" } else { "block text-sm leading-snug text-base-content/70" },
                                {tid!("compromised-switch-hint")}
                            }
                        }
                        input {
                            id: "compromised-switch",
                            r#type: "checkbox",
                            role: "switch",
                            class: if compromising { "toggle toggle-error mt-0.5" } else { "toggle toggle-primary mt-0.5" },
                            aria_describedby: "compromised-switch-hint",
                            checked: compromising,
                            disabled: saving(),
                            onchange: move |e| compromise.set(e.checked()),
                        }
                    }
                    div { id: "compromised-details", aria_live: "polite", class: "flex flex-col gap-2 text-sm text-base-content/70",
                        if compromising {
                            p { {tid!("compromised-body")} }
                            if has_recovery_key {
                                p { {tid!("compromised-recovery-note")} }
                            }
                        }
                    }
                    }
                    SheetFooter {
                        on_cancel: move |_| if !saving() { on_close.call(()) },
                        button {
                            id: "change-password-submit",
                            r#type: "submit",
                            class: if compromising { "btn btn-error" } else { "btn btn-primary" },
                            disabled: saving(),
                            if saving() {
                                span { class: "loading loading-spinner loading-sm", role: "status", aria_label: tid!("loading") }
                                {tid!("change-password-submitting")}
                            } else if compromising {
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
            "data-sheet-locked": "true",
            role: "dialog",
            aria_modal: "true",
            aria_labelledby: "new-recovery-key-title",
            tabindex: "-1",
            div { class: "modal-box max-w-md p-0 flex flex-col",
                SheetBar {
                    h2 { id: "new-recovery-key-title", class: SHEET_TITLE, {tid!("recovery-key-title")} }
                }
                div { class: "flex-1 overflow-y-auto flex flex-col gap-3 px-5 pt-1 pb-4",
                    p { class: "text-sm", {tid!("compromised-new-key")} }
                    crate::auth::recovery_ui::KeyDisplay { key_text, email }
                }
                SheetFooter {
                    button { id: "new-recovery-key-done", r#type: "button", class: "btn btn-primary", onclick: move |_| on_close.call(()), {tid!("recovery-key-next")} }
                }
            }
            // Inert: Android back, Escape and a drag click it and the key stays on screen. Without
            // it back fell through to the page under the sheet (navigation_sync.rs).
            div { class: "modal-backdrop" }
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
