use dioxus::prelude::*;
use shared::{Account, PaymentMethod};
use uuid::Uuid;

use crate::common::{ConfirmModal, DiscardConfirm, Flash, FormSheetBar};
use crate::payment_methods::{
    method_display_name, method_kind_name, save_payment_methods, validate_method, with_method, without_method,
    writable_methods, SaveFailure, DEFAULT_KIND, KNOWN_KINDS, MAX_METHODS, OTHER_KIND,
};
use crate::tid;

/// Literal keys, so the i18n guard in `i18n.rs` sees every key this file can render.
fn validation_message(key: &str) -> String {
    match key {
        "payment-method-value-required" => tid!("payment-method-value-required"),
        "payment-method-label-required" => tid!("payment-method-label-required"),
        "payment-method-too-long" => tid!("payment-method-too-long"),
        "payment-method-invalid-characters" => tid!("payment-method-invalid-characters"),
        "payment-method-limit" => tid!("payment-method-limit", max: MAX_METHODS.to_string()),
        other => other.to_string(),
    }
}

/// The list as the server last stored it, and the nonce a write must still find there; `None` when
/// no write may start from it (`writable_methods`).
fn stored(account: &Option<Account>, key: [u8; 32], account_id: Uuid) -> Option<(Vec<PaymentMethod>, Option<String>)> {
    let account = account.as_ref()?;
    let methods = writable_methods(account, &key, account_id)?;
    Some((methods, account.payment_methods.as_ref().map(|p| p.iv.clone())))
}

fn store_pair(mut auth_ctx: Signal<Option<Account>>, account_id: Uuid, pair: shared::EncryptedPair) {
    if let Some(account) = auth_ctx.write().as_mut().filter(|a| a.id == account_id) {
        account.payment_methods = Some(pair);
    }
}

async fn reload(mut auth_ctx: Signal<Option<Account>>) {
    if let Ok(account) = api::auth::auth_controller::me().await {
        auth_ctx.set(account);
    }
}

/// Adds or edits one method (docs/plans/friends-settings-redesign.md §2.3). The sheet holds a draft
/// and the method it was opened on; every write rebuilds the list from what the account stores now,
/// so a half-typed draft never rides along with anything and a stale sheet never overwrites a
/// neighbour. Closes only once the server has the new list.
#[component]
pub fn PaymentMethodSheet(
    original: Option<PaymentMethod>,
    account_id: Uuid,
    account_key: [u8; 32],
    on_close: EventHandler<()>,
) -> Element {
    crate::common::use_modal_focus("payment-method-sheet");
    let auth_ctx = use_context::<Signal<Option<Account>>>();
    let is_online = use_context::<Signal<bool>>();
    let mut flash = use_context::<Signal<Option<Flash>>>();

    let opened = original.clone().unwrap_or_else(|| PaymentMethod { kind: DEFAULT_KIND.to_string(), ..Default::default() });
    let mut draft = use_signal(|| opened.clone());
    let mut busy = use_signal(|| false);
    let mut error: Signal<Option<String>> = use_signal(|| None);
    let mut discarding = use_signal(|| false);
    let mut deleting = use_signal(|| false);

    let dirty = draft() != opened;
    let online = is_online();
    let mut request_close = move || {
        if busy() {
            return;
        }
        if dirty {
            discarding.set(true);
        } else {
            on_close.call(());
        }
    };

    let edited_from = original.clone();
    let mut save = move |_: ()| {
        error.set(None);
        if !is_online() || busy() {
            return;
        }
        let method = match validate_method(&draft()) {
            Ok(method) => method,
            Err(key) => {
                error.set(Some(validation_message(key)));
                return;
            }
        };
        let Some((current, expected_iv)) = stored(&auth_ctx.peek(), account_key, account_id) else {
            error.set(Some(tid!("payment-methods-key-missing")));
            return;
        };
        let next = with_method(&current, edited_from.as_ref(), method);
        if next.len() > MAX_METHODS {
            error.set(Some(validation_message("payment-method-limit")));
            return;
        }
        spawn(async move {
            busy.set(true);
            match save_payment_methods(account_key, next, expected_iv).await {
                Ok(pair) => {
                    store_pair(auth_ctx, account_id, pair);
                    flash.set(Some(Flash::ok(tid!("payment-methods-saved"))));
                    busy.set(false);
                    on_close.call(());
                }
                Err(SaveFailure::Stale) => {
                    reload(auth_ctx).await;
                    error.set(Some(tid!("payment-methods-stale")));
                    busy.set(false);
                }
                Err(SaveFailure::Other(e)) => {
                    error.set(Some(e));
                    busy.set(false);
                }
            }
        });
    };

    let deleted = original.clone();
    let delete = move |_| {
        deleting.set(false);
        error.set(None);
        let Some(deleted) = deleted.clone() else { return };
        if !is_online() || busy() {
            return;
        }
        let Some((current, expected_iv)) = stored(&auth_ctx.peek(), account_key, account_id) else {
            error.set(Some(tid!("payment-methods-key-missing")));
            return;
        };
        if !current.contains(&deleted) {
            on_close.call(());
            return;
        }
        let next = without_method(&current, &deleted);
        spawn(async move {
            busy.set(true);
            match save_payment_methods(account_key, next, expected_iv).await {
                Ok(pair) => {
                    store_pair(auth_ctx, account_id, pair);
                    flash.set(Some(Flash::ok(tid!("payment-method-deleted"))));
                    busy.set(false);
                    on_close.call(());
                }
                Err(SaveFailure::Stale) => {
                    reload(auth_ctx).await;
                    error.set(Some(tid!("payment-methods-stale")));
                    busy.set(false);
                }
                Err(SaveFailure::Other(e)) => {
                    error.set(Some(e));
                    busy.set(false);
                }
            }
        });
    };

    let current = draft();
    let is_other = current.kind == OTHER_KIND;
    let is_known = KNOWN_KINDS.iter().any(|(k, _)| *k == current.kind);
    let title = method_kind_name(&current).unwrap_or_else(|| tid!("payment-method-kind-other"));
    let label_value = current.label.clone().unwrap_or_default();
    let delete_name = original.as_ref().map(method_display_name).unwrap_or_default();

    rsx! {
        div {
            id: "payment-method-sheet",
            class: "modal modal-open modal-bottom sm:modal-middle",
            role: "dialog",
            "data-sheet-dirty": if dirty { "true" },
            aria_modal: "true",
            aria_labelledby: "payment-method-title",
            tabindex: "-1",
            div { class: "modal-box max-w-md p-0",
                form {
                    id: "payment-method-form",
                    class: "flex flex-col gap-4 px-6 pt-4 pb-6",
                    onsubmit: move |e| {
                        e.prevent_default();
                        save(());
                    },
                    FormSheetBar {
                        close_id: "payment-method-close",
                        confirm_id: "payment-method-save",
                        confirm_label: tid!("save"),
                        form: "payment-method-form",
                        disabled: busy() || !dirty || !online,
                        on_close: move |_| request_close(),
                        h2 { id: "payment-method-title", class: "w-full text-center font-display font-bold text-lg truncate", "{title}" }
                    }
                    if !online {
                        div { id: "payment-method-offline", role: "status", class: "alert alert-warning alert-soft text-sm",
                            {tid!("payment-methods-offline")}
                        }
                    }
                    if let Some(err) = error() {
                        div { id: "payment-method-error", role: "alert", class: "alert alert-error text-sm", "{err}" }
                    }
                    fieldset { class: "flex flex-col gap-4", disabled: busy(),
                        label { class: "flex flex-col gap-1.5", r#for: "payment-method-kind",
                            span { class: "text-sm font-medium text-base-content/70", {tid!("payment-method-kind")} }
                            select {
                                id: "payment-method-kind",
                                class: "select w-full",
                                value: "{current.kind}",
                                onchange: move |e| draft.write().kind = e.value(),
                                for (kind , brand) in KNOWN_KINDS {
                                    option { value: "{kind}", selected: *kind == current.kind,
                                        if brand.is_empty() {
                                            {tid!("payment-method-kind-other")}
                                        } else {
                                            "{brand}"
                                        }
                                    }
                                }
                                // A kind written by a newer build: without its own option the select
                                // would show, and the next save store, the first kind instead.
                                if !is_known {
                                    option { value: "{current.kind}", selected: true, "{current.kind}" }
                                }
                            }
                        }
                        label { class: "flex flex-col gap-1.5", r#for: "payment-method-label",
                            span { class: "text-sm font-medium text-base-content/70",
                                if is_other {
                                    {tid!("payment-method-label")}
                                } else {
                                    {tid!("payment-method-label-optional")}
                                }
                            }
                            input {
                                id: "payment-method-label",
                                class: "input w-full",
                                r#type: "text",
                                required: is_other,
                                maxlength: "60",
                                placeholder: tid!("payment-method-label-placeholder"),
                                value: "{label_value}",
                                oninput: move |e| draft.write().label = Some(e.value()),
                            }
                        }
                        label { class: "flex flex-col gap-1.5", r#for: "payment-method-value",
                            span { class: "text-sm font-medium text-base-content/70", {tid!("payment-method-value")} }
                            input {
                                id: "payment-method-value",
                                class: "input w-full font-mono",
                                r#type: "text",
                                required: true,
                                maxlength: "140",
                                autocapitalize: "off",
                                autocorrect: "off",
                                autocomplete: "off",
                                spellcheck: "false",
                                placeholder: tid!("payment-method-value-placeholder"),
                                value: "{current.value}",
                                oninput: move |e| draft.write().value = e.value(),
                            }
                        }
                        label { class: "flex items-center gap-3 min-h-11 cursor-pointer", r#for: "payment-method-share",
                            span { class: "flex-1 min-w-0", {tid!("payment-method-share-groups")} }
                            input {
                                id: "payment-method-share",
                                r#type: "checkbox",
                                role: "switch",
                                class: "toggle toggle-primary",
                                checked: current.shared,
                                onchange: move |e| draft.write().shared = e.checked(),
                            }
                        }
                    }
                    if original.is_some() {
                        button {
                            id: "payment-method-delete",
                            r#type: "button",
                            class: "btn btn-ghost text-error",
                            disabled: busy() || !online,
                            onclick: move |_| deleting.set(true),
                            {tid!("delete")}
                        }
                    }
                }
            }
            div { class: "modal-backdrop", onclick: move |_| request_close() }
        }
        if discarding() {
            DiscardConfirm {
                on_keep: move |_| discarding.set(false),
                on_discard: move |_| {
                    discarding.set(false);
                    on_close.call(());
                },
            }
        }
        if deleting() {
            ConfirmModal {
                title: tid!("payment-method-delete-confirm-title"),
                message: tid!("payment-method-delete-confirm-message", name: delete_name),
                confirm_label: tid!("delete"),
                action_sheet: true,
                on_confirm: delete,
                on_cancel: move |_| deleting.set(false),
            }
        }
    }
}
