use dioxus::prelude::*;
use shared::Account;

use crate::auth::credentials::delete_with_password;
use crate::common::{error_message, haptic, Haptic, SheetBar, SheetFooter, SHEET_TITLE};
use crate::tid;

/// Deleting the account asks for the password (security review 2026-11-02 L3): the proof goes with
/// the request, so a session alone never erases the account's only copies of its keys.
#[component]
pub fn DeleteAccountSheet(on_close: EventHandler<()>, on_deleted: EventHandler<()>) -> Element {
    crate::common::use_modal_focus("delete-account-sheet");
    let auth = use_context::<Signal<Option<Account>>>();
    let mut password = use_signal(String::new);
    let mut error: Signal<Option<String>> = use_signal(|| None);
    let mut deleting = use_signal(|| false);

    let on_submit = move |e: FormEvent| {
        e.prevent_default();
        let Some(email) = auth().map(|a| a.email) else { return };
        let typed = password();
        spawn(async move {
            deleting.set(true);
            error.set(None);
            match delete_with_password(email, &typed).await {
                Ok(()) => {
                    haptic(Haptic::Warning);
                    on_deleted.call(());
                }
                Err(e) => {
                    error.set(Some(error_message(&e)));
                    deleting.set(false);
                }
            }
        });
    };

    rsx! {
        div {
            id: "delete-account-sheet",
            class: "modal modal-open modal-bottom sm:modal-middle",
            role: "dialog",
            aria_modal: "true",
            aria_labelledby: "delete-account-title",
            aria_describedby: "delete-account-body",
            tabindex: "-1",
            div { class: "modal-box max-w-md p-0 flex flex-col",
                SheetBar { close_id: "delete-account-close", task: true, on_close: move |_| if !deleting() { on_close.call(()) },
                    h2 { id: "delete-account-title", class: SHEET_TITLE, {tid!("account-delete-confirm-title")} }
                }
                form { class: "flex flex-col flex-1 min-h-0", onsubmit: on_submit,
                    div { class: "flex-1 overflow-y-auto flex flex-col gap-3 px-5 pt-1 pb-4",
                    p { id: "delete-account-body", class: "text-sm text-base-content/70", {tid!("account-delete-confirm-message")} }
                    if let Some(err) = error() {
                        div { id: "delete-account-error", role: "alert", class: "alert alert-error text-sm", "{err}" }
                    }
                    fieldset { class: "fieldset",
                        label { class: "fieldset-legend", r#for: "delete-account-password", {tid!("change-password-current")} }
                        input {
                            id: "delete-account-password",
                            class: "input w-full",
                            r#type: "password",
                            required: true,
                            autocomplete: "current-password",
                            value: "{password}",
                            oninput: move |e| password.set(e.value()),
                        }
                    }
                    }
                    SheetFooter {
                        on_cancel: move |_| if !deleting() { on_close.call(()) },
                        button {
                            id: "delete-account-confirm",
                            r#type: "submit",
                            class: "btn btn-error",
                            disabled: deleting(),
                            {tid!("delete")}
                        }
                    }
                }
            }
            div { class: "modal-backdrop", onclick: move |_| on_close.call(()) }
        }
    }
}
