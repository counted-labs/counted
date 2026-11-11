//! The recovery key in settings, and the banner a pending reset shows (docs/plans/key-lifecycle.md
//! §6). Every flow lives in `account_rotation` and `recovery_flow`; this only renders and calls.

use chrono::{Local, NaiveDateTime, TimeZone, Utc};
use dioxus::prelude::*;
use shared::Account;

use crate::auth::account_key::remember;
use crate::auth::account_rotation::{cancel_recovery_replacement, save_recovery_key};
use crate::auth::recovery_flow::cancel_reset_with_password;
use crate::auth::recovery_key;
use crate::common::{copy_text, error_message, format_date, haptic, Flash, Haptic, ListRow, LocalStorageState, RowTile};
use crate::icons::{KeyIcon, UndoIcon};
use crate::tid;

/// A server time (UTC) as the device's local date and time.
pub fn format_moment(at: NaiveDateTime) -> String {
    let local = Utc.from_utc_datetime(&at).with_timezone(&Local);
    format!("{} {}", format_date(local.date_naive()), local.format("%H:%M"))
}

/// Which group of the key the confirmation asks for: one the user cannot skip by reading the first.
pub fn group_to_confirm(key_text: &str) -> usize {
    2 + (key_text.bytes().map(usize::from).sum::<usize>() % 12)
}

#[component]
pub fn RecoveryKeyRow(account: Account) -> Element {
    let mut sheet = use_signal(|| false);
    let mut cancelling = use_signal(|| false);

    let (status, status_class, tint) = match (account.recovery_pending_at.is_some(), account.has_recovery_key) {
        (true, _) => (tid!("settings-recovery-key-replacing"), "text-tile-amber-content", "bg-tile-amber text-tile-amber-content"),
        (false, true) => (tid!("settings-recovery-key-created"), "text-base-content/70", "bg-tile-green text-tile-green-content"),
        (false, false) => (tid!("settings-recovery-key-missing"), "text-tile-amber-content", "bg-tile-amber text-tile-amber-content"),
    };

    rsx! {
        ListRow {
            id: "recovery-key-open",
            class: "recovery-key-row",
            title: tid!("settings-recovery-key"),
            subtitle: status,
            subtitle_class: status_class,
            chevron: true,
            onclick: move |_| {
                haptic(Haptic::Light);
                sheet.set(true);
            },
            RowTile { tint, KeyIcon {} }
        }
        if account.recovery_pending_at.is_some() {
            ListRow {
                id: "recovery-key-cancel-open",
                title: tid!("recovery-key-cancel-replacement"),
                title_class: "text-error",
                onclick: move |_| cancelling.set(true),
                RowTile { tint: "bg-tile-red text-tile-red-content", UndoIcon {} }
            }
        }
        if sheet() {
            RecoveryKeySheet { replacing: account.has_recovery_key, on_close: move |_| sheet.set(false) }
        }
        if cancelling() {
            CancelReplacementSheet { on_close: move |_| cancelling.set(false) }
        }
    }
}

/// The key once, with copy, save and print; then one group typed back.
#[component]
pub fn KeyDisplay(key_text: String, email: String) -> Element {
    let mut copied = use_signal(|| false);
    let file = format!("{}\n{}\n\n{}\n", tid!("recovery-key-file-intro"), email, key_text);
    let to_copy = key_text.clone();

    rsx! {
        div { class: "flex flex-col gap-3",
            p { class: "text-sm text-base-content/70", {tid!("recovery-key-body")} }
            div { id: "recovery-key-text", class: "grid grid-cols-4 gap-2 font-mono text-center bg-base-200 rounded-box p-3 select-all",
                for (i, group) in key_text.split(' ').enumerate() {
                    span { key: "{i}", class: "text-sm tracking-wider", "{group}" }
                }
            }
            div { class: "flex flex-wrap gap-2",
                button {
                    id: "recovery-key-copy",
                    r#type: "button",
                    class: "btn btn-sm",
                    onclick: move |_| {
                        let text = to_copy.clone();
                        spawn(async move {
                            copied.set(copy_text(&text).await);
                        });
                    },
                    if copied() {
                        {tid!("recovery-key-copied")}
                    } else {
                        {tid!("recovery-key-copy")}
                    }
                }
                button {
                    id: "recovery-key-save",
                    r#type: "button",
                    class: "btn btn-sm",
                    onclick: move |_| crate::expenses::helpers::export::trigger_download(&file, "counted-recovery-key.txt", "text/plain;charset=utf-8"),
                    {tid!("recovery-key-save")}
                }
                if cfg!(target_arch = "wasm32") {
                    button {
                        id: "recovery-key-print",
                        r#type: "button",
                        class: "btn btn-sm",
                        onclick: move |_| print_page(),
                        {tid!("recovery-key-print")}
                    }
                }
            }
        }
    }
}

fn print_page() {
    #[cfg(target_arch = "wasm32")]
    if let Some(window) = web_sys::window() {
        let _ = window.print();
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Step {
    Show,
    Confirm,
    Password,
}

#[component]
fn RecoveryKeySheet(replacing: bool, on_close: EventHandler<()>) -> Element {
    crate::common::use_modal_focus("recovery-key-sheet");
    let auth = use_context::<Signal<Option<Account>>>();
    let key_ctx = use_context::<Signal<Option<[u8; 32]>>>();
    let ls_ctx = use_context::<Signal<LocalStorageState>>();
    let mut flash = use_context::<Signal<Option<Flash>>>();
    let key_text = use_hook(|| recovery_key::encode(&recovery_key::generate()));
    let mut step = use_signal(|| Step::Show);
    let mut typed = use_signal(String::new);
    let mut password = use_signal(String::new);
    let mut error: Signal<Option<String>> = use_signal(|| None);
    let mut saving = use_signal(|| false);
    let asked = group_to_confirm(&key_text);
    let email = auth.peek().as_ref().map(|a| a.email.clone()).unwrap_or_default();

    let expected = recovery_key::group(&key_text, asked - 1).unwrap_or_default().to_string();
    let on_confirm = move |e: FormEvent| {
        e.prevent_default();
        if typed().trim().eq_ignore_ascii_case(&expected) {
            error.set(None);
            step.set(Step::Password);
        } else {
            error.set(Some(tid!("recovery-key-confirm-wrong")));
        }
    };
    let saved_text = key_text.clone();
    let on_save = move |e: FormEvent| {
        e.prevent_default();
        let (text, current) = (saved_text.clone(), password());
        spawn(async move {
            saving.set(true);
            error.set(None);
            match save_recovery_key(&current, &text).await {
                Ok((account, master)) => {
                    let (mut auth, mut key_ctx) = (auth, key_ctx);
                    remember(ls_ctx, &account, &master);
                    key_ctx.set(Some(master));
                    auth.set(Some(account));
                    haptic(Haptic::Success);
                    let done = if replacing { tid!("recovery-key-replacement-done") } else { tid!("recovery-key-done") };
                    flash.set(Some(Flash::ok(done)));
                    on_close.call(());
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
            id: "recovery-key-sheet",
            class: "modal modal-open modal-bottom sm:modal-middle",
            role: "dialog",
            aria_modal: "true",
            aria_labelledby: "recovery-key-title",
            tabindex: "-1",
            div { class: "modal-box max-w-md flex flex-col gap-3",
                h2 { id: "recovery-key-title", class: "font-bold text-lg font-display", {tid!("recovery-key-title")} }
                if let Some(err) = error() {
                    div { id: "recovery-key-error", role: "alert", class: "alert alert-error text-sm", "{err}" }
                }
                match step() {
                    Step::Show => rsx! {
                        KeyDisplay { key_text: key_text.clone(), email: email.clone() }
                        div { class: "modal-action mt-0",
                            button { r#type: "button", class: "btn", onclick: move |_| on_close.call(()), {tid!("cancel")} }
                            button {
                                id: "recovery-key-next",
                                r#type: "button",
                                class: "btn btn-primary",
                                onclick: move |_| step.set(Step::Confirm),
                                {tid!("recovery-key-next")}
                            }
                        }
                    },
                    Step::Confirm => rsx! {
                        form { class: "flex flex-col gap-3", onsubmit: on_confirm,
                            label { class: "text-sm", r#for: "recovery-key-group", {tid!("recovery-key-confirm-body", n: asked as i64)} }
                            input {
                                id: "recovery-key-group",
                                class: "input w-full font-mono uppercase",
                                maxlength: "4",
                                autocomplete: "off",
                                autocapitalize: "characters",
                                autocorrect: "off",
                                spellcheck: "false",
                                required: true,
                                value: "{typed}",
                                oninput: move |e| typed.set(e.value()),
                            }
                            div { class: "modal-action mt-0",
                                button { r#type: "button", class: "btn", onclick: move |_| step.set(Step::Show), {tid!("back")} }
                                button { id: "recovery-key-confirm", r#type: "submit", class: "btn btn-primary", {tid!("recovery-key-next")} }
                            }
                        }
                    },
                    Step::Password => rsx! {
                        form { class: "flex flex-col gap-3", onsubmit: on_save,
                            label { class: "text-sm", r#for: "recovery-key-password", {tid!("recovery-key-password-body")} }
                            input {
                                id: "recovery-key-password",
                                class: "input w-full",
                                r#type: "password",
                                autocomplete: "current-password",
                                required: true,
                                value: "{password}",
                                oninput: move |e| password.set(e.value()),
                            }
                            div { class: "modal-action mt-0",
                                button { r#type: "button", class: "btn", disabled: saving(), onclick: move |_| on_close.call(()), {tid!("cancel")} }
                                button {
                                    id: "recovery-key-submit",
                                    r#type: "submit",
                                    class: "btn btn-primary",
                                    disabled: saving(),
                                    if saving() {
                                        span { class: "loading loading-spinner loading-sm", role: "status", aria_label: tid!("loading") }
                                    }
                                    {tid!("recovery-key-submit")}
                                }
                            }
                        }
                    },
                }
            }
            div { class: "modal-backdrop", onclick: move |_| if !saving() { on_close.call(()) } }
        }
    }
}

#[component]
fn CancelReplacementSheet(on_close: EventHandler<()>) -> Element {
    crate::common::use_modal_focus("recovery-cancel-sheet");
    let mut auth = use_context::<Signal<Option<Account>>>();
    let mut flash = use_context::<Signal<Option<Flash>>>();
    let mut key = use_signal(String::new);
    let mut error: Signal<Option<String>> = use_signal(|| None);
    let mut busy = use_signal(|| false);

    let on_submit = move |e: FormEvent| {
        e.prevent_default();
        let text = key();
        spawn(async move {
            busy.set(true);
            match cancel_recovery_replacement(&text).await {
                Ok(()) => {
                    if let Ok(fresh) = api::auth::auth_controller::me().await {
                        auth.set(fresh);
                    }
                    flash.set(Some(Flash::ok(tid!("recovery-key-cancel-done"))));
                    on_close.call(());
                }
                Err(e) => {
                    error.set(Some(error_message(&e)));
                    busy.set(false);
                }
            }
        });
    };

    rsx! {
        div {
            id: "recovery-cancel-sheet",
            class: "modal modal-open modal-bottom sm:modal-middle",
            role: "dialog",
            aria_modal: "true",
            aria_labelledby: "recovery-cancel-title",
            tabindex: "-1",
            div { class: "modal-box max-w-md",
                h2 { id: "recovery-cancel-title", class: "font-bold text-lg font-display mb-3", {tid!("recovery-key-cancel-replacement")} }
                form { class: "flex flex-col gap-3", onsubmit: on_submit,
                    if let Some(err) = error() {
                        div { role: "alert", class: "alert alert-error text-sm", "{err}" }
                    }
                    label { class: "fieldset-legend", r#for: "recovery-cancel-key", {tid!("recovery-key-field")} }
                    textarea {
                        id: "recovery-cancel-key",
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
                    div { class: "modal-action mt-0",
                        button { r#type: "button", class: "btn", onclick: move |_| on_close.call(()), {tid!("cancel")} }
                        button { id: "recovery-cancel-submit", r#type: "submit", class: "btn btn-error", disabled: busy(), {tid!("recovery-key-cancel-replacement")} }
                    }
                }
            }
            div { class: "modal-backdrop", onclick: move |_| on_close.call(()) }
        }
    }
}

/// Plan §6.3: every signed-in device is told about a pending reset, and the password that was in
/// place when it was requested cancels it.
#[component]
pub fn PendingResetBanner() -> Element {
    let mut auth = use_context::<Signal<Option<Account>>>();
    let mut flash = use_context::<Signal<Option<Flash>>>();
    let mut open = use_signal(|| false);
    let mut password = use_signal(String::new);
    let mut error: Signal<Option<String>> = use_signal(|| None);
    let mut busy = use_signal(|| false);
    let Some((email, at)) = auth().and_then(|a| Some((a.email.clone(), a.pending_reset_at?))) else {
        return rsx! {};
    };

    let on_submit = move |e: FormEvent| {
        e.prevent_default();
        let (email, typed) = (email.clone(), password());
        spawn(async move {
            busy.set(true);
            error.set(None);
            match cancel_reset_with_password(email, &typed).await {
                Ok(()) => {
                    if let Ok(fresh) = api::auth::auth_controller::me().await {
                        auth.set(fresh);
                    }
                    flash.set(Some(Flash::ok(tid!("pending-reset-cancelled"))));
                }
                Err(e) => error.set(Some(error_message(&e))),
            }
            busy.set(false);
        });
    };

    rsx! {
        div { id: "pending-reset-banner", role: "alert", class: "alert alert-warning flex flex-col items-start gap-2",
            p { class: "font-semibold", {tid!("pending-reset-title")} }
            p { class: "text-sm", {tid!("pending-reset-body", when: format_moment(at))} }
            if open() {
                form { class: "flex flex-col gap-2 w-full", onsubmit: on_submit,
                    if let Some(err) = error() {
                        p { class: "text-sm text-error", "{err}" }
                    }
                    input {
                        id: "pending-reset-password",
                        class: "input input-sm w-full",
                        r#type: "password",
                        autocomplete: "current-password",
                        required: true,
                        aria_label: tid!("change-password-current"),
                        value: "{password}",
                        oninput: move |e| password.set(e.value()),
                    }
                    button { id: "pending-reset-submit", r#type: "submit", class: "btn btn-sm btn-error", disabled: busy(), {tid!("pending-reset-cancel")} }
                }
            } else {
                button { id: "pending-reset-open", r#type: "button", class: "btn btn-sm btn-outline", onclick: move |_| open.set(true), {tid!("pending-reset-cancel")} }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_confirmed_group_is_never_the_first() {
        for _ in 0..50 {
            let text = recovery_key::encode(&recovery_key::generate());
            let n = group_to_confirm(&text);
            assert!((2..=13).contains(&n), "{n}");
            assert_eq!(recovery_key::group(&text, n - 1).map(str::len), Some(4));
        }
    }
}
