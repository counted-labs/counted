use dioxus::prelude::*;
use crate::tid;
use shared::{Account, PaymentMethod};
use uuid::Uuid;

use crate::common::Flash;
use crate::icons::{PlusIcon, TrashIcon, ICON_INLINE};
use crate::payment_methods::{
    account_payment_methods, method_display_name, save_payment_methods, validate_method,
    SaveFailure, DEFAULT_KIND, KNOWN_KINDS, MAX_METHODS, OTHER_KIND,
};

/// `validate_method` returns keys, not sentences. Matching on literals is what keeps the i18n
/// guard in `i18n.rs` able to see every key this file can render — it only scans literal call
/// sites, so a key handed straight to the macro would never be checked against en.ftl.
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

/// One method, plus an id that is stable for as long as the card lives.
///
/// `PaymentMethod` carries no identity of its own and a position is not one: deleting a row shifts
/// every index after it, and each row here saves on its own, against a list it must be able to
/// address afterwards. The id never leaves the client — it is not part of what is stored.
#[derive(Clone, PartialEq)]
struct Row {
    id: u32,
    method: PaymentMethod,
}

fn methods_of(rows: &[Row]) -> Vec<PaymentMethod> {
    rows.iter().map(|r| r.method.clone()).collect()
}

/// The nonce of the blob the rows were seeded from — what the server must still hold for a save
/// to land. `None` for an account that never saved any.
fn expected_iv(auth_ctx: Signal<Option<Account>>) -> Option<String> {
    auth_ctx().and_then(|a| a.payment_methods).map(|p| p.iv)
}

/// Another device saved since this card was seeded. Refetch and reseed from what it stored;
/// clearing the guard is what lets the seeding effect run again for the same account.
async fn reload(mut auth_ctx: Signal<Option<Account>>, mut seeded_for: Signal<Option<Uuid>>) {
    if let Ok(account) = api::auth::auth_controller::me().await {
        seeded_for.set(None);
        auth_ctx.set(account);
    }
}

/// How the holder wants to be paid back. Each method is saved and deleted on its own.
///
/// The account still stores the whole list as one encrypted blob, so every write rewrites all of
/// it — but from `persisted`, the list as last stored, with only the row being acted on replaced.
/// A half-typed method therefore never rides along with its neighbour's save, and a row nobody can
/// finish never blocks the rest.
///
/// Signed-in only — `SettingsPage` renders it inside the branch that already has an account.
#[component]
pub fn PaymentMethodsCard() -> Element {
    let mut auth_ctx = use_context::<Signal<Option<Account>>>();
    let account_enc_key_ctx = use_context::<Signal<Option<[u8; 32]>>>();
    let is_online = use_context::<Signal<bool>>();
    let mut flash = use_context::<Signal<Option<Flash>>>();

    let mut rows: Signal<Vec<Row>> = use_signal(Vec::new);
    let mut persisted: Signal<Vec<Row>> = use_signal(Vec::new);
    let mut next_id = use_signal(|| 0u32);
    let mut busy: Signal<Option<u32>> = use_signal(|| None);
    let mut row_error: Signal<Option<(u32, String)>> = use_signal(|| None);
    let mut seeded_for: Signal<Option<Uuid>> = use_signal(|| None);

    // Seeded once per account, from whatever `me()` came back with. Reading the guard through
    // `peek` is what stops the write-back after a save from re-seeding over the user's next edit.
    //
    // Keyed on the account id rather than a bare "have I run" flag: if this card ever outlives a
    // switch from one account to another, a stale draft would let one account's bank details be
    // saved onto another's.
    use_effect(move || {
        let account = auth_ctx();
        let key = account_enc_key_ctx();
        let (Some(account), Some(key)) = (account, key) else {
            return;
        };
        if *seeded_for.peek() == Some(account.id) {
            return;
        }
        let seeded: Vec<Row> = account_payment_methods(&account, &key)
            .into_iter()
            .enumerate()
            .map(|(i, method)| Row { id: i as u32, method })
            .collect();
        next_id.set(seeded.len() as u32);
        rows.set(seeded.clone());
        persisted.set(seeded);
        seeded_for.set(Some(account.id));
    });

    let save_row = use_callback(move |id: u32| {
        row_error.set(None);

        // `OpKind` covers expenses and projects only, so there is nothing to queue this into.
        // Saying so beats a write that looks like it landed and never did.
        if !is_online() {
            row_error.set(Some((id, tid!("payment-methods-offline"))));
            return;
        }
        let Some(key) = account_enc_key_ctx() else {
            return;
        };
        let Some(row) = rows().into_iter().find(|r| r.id == id) else {
            return;
        };
        let method = match validate_method(&row.method) {
            Ok(method) => method,
            Err(key) => {
                row_error.set(Some((id, validation_message(key))));
                return;
            }
        };

        let mut next = persisted();
        match next.iter_mut().find(|r| r.id == id) {
            Some(stored) => stored.method = method.clone(),
            None => next.push(Row { id, method: method.clone() }),
        }
        if next.len() > MAX_METHODS {
            row_error.set(Some((id, validation_message("payment-method-limit"))));
            return;
        }

        spawn(async move {
            busy.set(Some(id));
            match save_payment_methods(key, methods_of(&next), expected_iv(auth_ctx)).await {
                Ok(pair) => {
                    persisted.set(next);
                    // The trimmed method is what was stored, so it is what the field must now show.
                    if let Some(row) = rows.write().iter_mut().find(|r| r.id == id) {
                        row.method = method;
                    }
                    if let Some(account) = auth_ctx.write().as_mut() {
                        account.payment_methods = Some(pair);
                    }
                    flash.set(Some(Flash::ok(tid!("payment-methods-saved"))));
                }
                Err(SaveFailure::Stale) => {
                    reload(auth_ctx, seeded_for).await;
                    row_error.set(Some((id, tid!("payment-methods-stale"))));
                }
                Err(SaveFailure::Other(e)) => row_error.set(Some((id, e))),
            }
            busy.set(None);
        });
    });

    let delete_row = use_callback(move |id: u32| {
        row_error.set(None);

        // A row that was never saved exists only in this form — dropping it is the whole deletion,
        // and asking the server to rewrite a blob that never mentioned it would be a wasted round
        // trip that also needs a connection.
        if !persisted().iter().any(|r| r.id == id) {
            rows.write().retain(|r| r.id != id);
            return;
        }
        if !is_online() {
            row_error.set(Some((id, tid!("payment-methods-offline"))));
            return;
        }
        let Some(key) = account_enc_key_ctx() else {
            return;
        };

        let next: Vec<Row> = persisted().into_iter().filter(|r| r.id != id).collect();
        spawn(async move {
            busy.set(Some(id));
            match save_payment_methods(key, methods_of(&next), expected_iv(auth_ctx)).await {
                Ok(pair) => {
                    persisted.set(next);
                    rows.write().retain(|r| r.id != id);
                    if let Some(account) = auth_ctx.write().as_mut() {
                        account.payment_methods = Some(pair);
                    }
                    flash.set(Some(Flash::ok(tid!("payment-method-deleted"))));
                }
                Err(SaveFailure::Stale) => {
                    reload(auth_ctx, seeded_for).await;
                    row_error.set(Some((id, tid!("payment-methods-stale"))));
                }
                Err(SaveFailure::Other(e)) => row_error.set(Some((id, e))),
            }
            busy.set(None);
        });
    });

    rsx! {
        div { class: "card bg-base-100 shadow-soft",
            fieldset { class: "fieldset card-body",
                legend { class: "fieldset-legend", {tid!("settings-payment-methods")} }
                // Not `p.label`: DaisyUI's `.label` is `white-space: nowrap`, so a sentence in one
                // widens the card past the page and every field with it.
                p { class: "text-sm text-base-content/70",
                    {tid!("settings-payment-methods-hint")}
                }
                p { class: "text-sm text-base-content/70",
                    {tid!("settings-payment-methods-share-warning")}
                }

                // A session without the account key (local store cleared) cannot read the blob and
                // must not write it either: saving from here would overwrite real details with an
                // empty list.
                if account_enc_key_ctx().is_none() {
                    p { class: "text-sm text-base-content/70",
                        {tid!("payment-methods-key-missing")}
                    }
                } else {

                    if rows().is_empty() {
                        p { class: "text-sm text-base-content/70", {tid!("payment-method-empty")} }
                    }

                    ul { class: "flex flex-col gap-3",
                        for (i , row) in rows().into_iter().enumerate() {
                            {
                                let id = row.id;
                                let method = row.method.clone();
                                let is_other = method.kind == OTHER_KIND;
                                let is_known = KNOWN_KINDS.iter().any(|(k, _)| *k == method.kind);
                                let label_value = method.label.clone().unwrap_or_default();
                                let stored = persisted().into_iter().find(|r| r.id == id);
                                let dirty = stored.map(|r| r.method) != Some(method.clone());
                                let working = busy() == Some(id);
                                let error = row_error().filter(|(at, _)| *at == id).map(|(_, m)| m);
                                rsx! {
                                    li { key: "{id}",
                                        fieldset {
                                            class: "fieldset bg-base-200 border-base-300 rounded-box border p-4",
                                            disabled: working,
                                            label {
                                                class: "fieldset-legend",
                                                r#for: "payment-method-kind-{i}",
                                                {tid!("payment-method-kind")}
                                            }
                                            select {
                                                id: "payment-method-kind-{i}",
                                                class: "select w-full",
                                                value: "{method.kind}",
                                                onchange: move |e| {
                                                    if let Some(row) = rows.write().iter_mut().find(|r| r.id == id) {
                                                        row.method.kind = e.value();
                                                    }
                                                },
                                                for (kind , brand) in KNOWN_KINDS {
                                                    option {
                                                        value: "{kind}",
                                                        selected: *kind == method.kind,
                                                        if brand.is_empty() {
                                                            {tid!("payment-method-kind-other")}
                                                        } else {
                                                            "{brand}"
                                                        }
                                                    }
                                                }
                                                // A kind written by a newer build has no entry
                                                // above. Without this option the browser would
                                                // show — and on the next save store — the first
                                                // one instead, silently rewriting it.
                                                if !is_known {
                                                    option {
                                                        value: "{method.kind}",
                                                        selected: true,
                                                        "{method.kind}"
                                                    }
                                                }
                                            }

                                            label {
                                                class: "fieldset-legend",
                                                r#for: "payment-method-label-{i}",
                                                {tid!("payment-method-label")}
                                            }
                                            input {
                                                id: "payment-method-label-{i}",
                                                class: "input w-full",
                                                r#type: "text",
                                                required: is_other,
                                                placeholder: tid!("payment-method-label-placeholder"),
                                                value: "{label_value}",
                                                oninput: move |e| {
                                                    if let Some(row) = rows.write().iter_mut().find(|r| r.id == id) {
                                                        row.method.label = Some(e.value());
                                                    }
                                                },
                                            }

                                            label {
                                                class: "fieldset-legend",
                                                r#for: "payment-method-value-{i}",
                                                {tid!("payment-method-value")}
                                            }
                                            input {
                                                id: "payment-method-value-{i}",
                                                class: "input w-full",
                                                r#type: "text",
                                                placeholder: tid!("payment-method-value-placeholder"),
                                                value: "{method.value}",
                                                oninput: move |e| {
                                                    if let Some(row) = rows.write().iter_mut().find(|r| r.id == id) {
                                                        row.method.value = e.value();
                                                    }
                                                },
                                            }

                                            label {
                                                class: "flex items-center gap-3 mt-3 cursor-pointer",
                                                r#for: "payment-method-share-{i}",
                                                input {
                                                    id: "payment-method-share-{i}",
                                                    r#type: "checkbox",
                                                    class: "toggle toggle-primary",
                                                    checked: method.shared,
                                                    onchange: move |e| {
                                                        if let Some(row) = rows.write().iter_mut().find(|r| r.id == id) {
                                                            row.method.shared = e.checked();
                                                        }
                                                    },
                                                }
                                                span { class: "flex flex-col",
                                                    span { class: "text-sm", {tid!("payment-method-share")} }
                                                    span { class: "text-xs text-base-content/70",
                                                        {tid!("payment-method-share-hint")}
                                                    }
                                                }
                                            }

                                            // Beside the row it belongs to, not at the top of the
                                            // card: with a save button per method, a banner over
                                            // the whole list would not say which one failed.
                                            if let Some(err) = error {
                                                div {
                                                    role: "alert",
                                                    class: "alert alert-error text-sm mt-2",
                                                    "{err}"
                                                }
                                            }

                                            div { class: "flex justify-end gap-2 mt-2",
                                                button {
                                                    id: "payment-method-remove-{i}",
                                                    r#type: "button",
                                                    class: "btn btn-square btn-soft",
                                                    // Named, or every remove button on the card
                                                    // reads identically to a screen reader.
                                                    aria_label: tid!(
                                                        "payment-method-remove", name : method_display_name(& method)
                                                    ),
                                                    onclick: move |_| delete_row.call(id),
                                                    TrashIcon { size: ICON_INLINE }
                                                }
                                                button {
                                                    id: "payment-method-save-{i}",
                                                    r#type: "button",
                                                    class: "btn btn-primary",
                                                    disabled: !dirty,
                                                    onclick: move |_| save_row.call(id),
                                                    if working {
                                                        span {
                                                            class: "loading loading-spinner loading-sm",
                                                            role: "status",
                                                            aria_label: tid!("loading"),
                                                        }
                                                    }
                                                    {tid!("save")}
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    if rows().len() < MAX_METHODS {
                        button {
                            id: "payment-method-add",
                            r#type: "button",
                            class: "btn btn-soft w-full",
                            onclick: move |_| {
                                let id = next_id();
                                next_id.set(id + 1);
                                rows
                                    .write()
                                    .push(Row {
                                        id,
                                        method: PaymentMethod {
                                            kind: DEFAULT_KIND.to_string(),
                                            ..Default::default()
                                        },
                                    });
                            },
                            PlusIcon { size: ICON_INLINE }
                            {tid!("payment-method-add")}
                        }
                    } else {
                        p { class: "text-xs text-base-content/70",
                            {tid!("payment-method-limit", max: MAX_METHODS.to_string())}
                        }
                    }
                }
            }
        }
    }
}
