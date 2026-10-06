use api::account_projects::account_projects_controller::upsert_account_project;
use api::expenses::expenses_controller::batch_add_expenses;
use api::projects::projects_controller::add_project;
use api::tricount::tricount_controller::fetch_tricount_registry;
use api::tricount::tricount_models::TricountImportRequest;
use api::users::users_controller::add_user;
use dioxus::fullstack::Json;
use dioxus::prelude::*;
use crate::tid;
use shared::{
    round_currency, sums_to_total, Account as AccountData, CreatableExpense, CreatableProject,
    CreatableUser, CreatableUserBatch, EncryptedUserAmount, ExpensePayload, ExpenseType,
    PaymentPayload, ProjectPayload, UpsertAccountProject, UserPayload,
};
use std::collections::{HashMap, HashSet};

use crate::common::{error_message, update_ls, upsert_project, upsert_project_key, LocalStorageState, NativeClipboardReader};
use crate::crypto::{
    claim_token, claim_verifier, encrypt_json, generate_key, key_to_fragment, wrap_key,
};
use crate::icons::{CloseIcon, ICON_HEADER};
use crate::route::Route;

#[derive(Props, Clone, PartialEq)]
pub struct ImportTricountModalProps {
    pub on_close: EventHandler<()>,
}

#[component]
pub fn ImportTricountModal(props: ImportTricountModalProps) -> Element {
    let mut tricount_key = use_signal(String::new);
    let mut error_msg: Signal<Option<String>> = use_signal(|| None);
    let mut loading = use_signal(|| false);

    // WebView only: a native paste (long-press → Coller) never reaches oninput there. On web
    // oninput already fires on paste, and this listener would need eval — CSP-blocked
    // (`common::web_dom`).
    #[cfg(not(target_arch = "wasm32"))]
    use_effect(move || {
        spawn(async move {
            let mut eval = document::eval(
                r#"(function(){var i=document.getElementById('tricount-link-input');if(!i)return;i.addEventListener('paste',function(e){var t=(e.clipboardData||window.clipboardData).getData('text/plain');if(t)dioxus.send(t);});})()"#,
            );
            while let Ok(val) = eval.recv::<serde_json::Value>().await {
                if let Some(text) = val.as_str() {
                    let text = text.trim().to_string();
                    if !text.is_empty() {
                        tricount_key.set(text);
                        error_msg.set(None);
                    }
                }
            }
        });
    });

    let clipboard_ctx = use_context::<Option<NativeClipboardReader>>();

    let nav = use_navigator();
    let auth_ctx = use_context::<Signal<Option<AccountData>>>();
    let account_key_ctx = use_context::<Signal<Option<[u8; 32]>>>();
    let ls_ctx = use_context::<Signal<LocalStorageState>>();

    let on_submit = move |e: FormEvent| {
        e.prevent_default();
        let key_input = tricount_key().trim().to_string();
        if key_input.is_empty() {
            error_msg.set(Some(tid!("import-tricount-key-required")));
            return;
        }
        loading.set(true);
        error_msg.set(None);

        let is_auth = auth_ctx().is_some();
        let account_key = account_key_ctx();

        spawn(async move {
    // Server-side HTTP proxy.
            let registry =
                match fetch_tricount_registry(Json(TricountImportRequest { tricount_key: key_input }))
                    .await
                {
                    Ok(r) => r,
                    Err(e) => {
                        error_msg.set(Some(error_message(&e)));
                        loading.set(false);
                        return;
                    }
                };

            let members: Vec<_> =
                registry.memberships.iter().filter_map(|m| m.non_user.as_ref()).collect();
            let known: HashSet<&str> = members.iter().map(|m| m.uuid.as_str()).collect();
            let planned = match plan_entries(
                registry.all_registry_entry.iter().filter_map(|w| w.entry.as_ref()),
                &known,
            ) {
                Ok(p) => p,
                Err(count) => {
                    error_msg.set(Some(tid!("import-tricount-unimportable", count: count as i64)));
                    loading.set(false);
                    return;
                }
            };

            let enc_key = generate_key();
            let key_fragment = key_to_fragment(&enc_key);

            let project_payload = match encrypt_json(&enc_key, &ProjectPayload {
                name: registry.title.clone(),
                currency: registry.currency.clone(),
                description: None,
                status: None,
            }) {
                Ok(p) => p,
                Err(_) => {
                    error_msg.set(Some(tid!("import-tricount-encryption-failed")));
                    loading.set(false);
                    return;
                }
            };

            let project = match add_project(Json(CreatableProject {
                payload: project_payload,
                claim_verifier: Some(claim_verifier(&enc_key).to_vec()),
                demo: crate::common::persist::is_demo(),
            }))
            .await
            {
                Ok(p) => p,
                Err(e) => {
                    error_msg.set(Some(error_message(&e)));
                    loading.set(false);
                    return;
                }
            };
            // Stored the moment the project exists, not after the last request: a failure in the
            // participants or expenses step used to leave a project nobody could ever decrypt. A
            // partial import now stays in the list, openable and leavable.
            update_ls(ls_ctx, |state| {
                upsert_project(state, project.id, None);
                upsert_project_key(state, project.id, key_fragment);
            });

            let creatables: Vec<CreatableUser> = match members
                .iter()
                .map(|m| {
                    Ok(CreatableUser {
                        payload: encrypt_json(&enc_key, &UserPayload {
                            name: m.alias.display_name.clone(),
                            removed: false,
                        })?,
                        project_id: project.id,
                    })
                })
                .collect::<Result<Vec<_>, String>>()
            {
                Ok(v) => v,
                Err(e) => {
                    error_msg.set(Some(e));
                    loading.set(false);
                    return;
                }
            };

            let created_users = if creatables.is_empty() {
                vec![]
            } else {
                match add_user(Json(CreatableUserBatch::Multiple(creatables))).await {
                    Ok(u) => u,
                    Err(e) => {
                        error_msg.set(Some(error_message(&e)));
                        loading.set(false);
                        return;
                    }
                }
            };

            let mut uuid_to_id: HashMap<String, i32> = HashMap::new();
            for (member, user) in members.iter().zip(created_users.iter()) {
                uuid_to_id.insert(member.uuid.clone(), user.id);
            }

            let mut expenses_batch: Vec<CreatableExpense> = Vec::new();
            for entry in &planned {
                // Every uuid was checked against `known` by `plan_entries`, and `uuid_to_id` holds
                // one participant per member: a miss here means `add_user` returned fewer rows.
                let (Some(&payer_id), Some(debtor_ids)) = (
                    uuid_to_id.get(entry.payer),
                    entry.debtors.iter().map(|(u, a)| uuid_to_id.get(*u).map(|id| (*id, *a))).collect::<Option<Vec<_>>>(),
                ) else {
                    error_msg.set(Some(tid!("import-tricount-unimportable", count: 1i64)));
                    loading.set(false);
                    return;
                };

                let expense_type =
                    if entry.transfer { ExpenseType::Transfer } else { ExpenseType::Expense };

                // No conversion: the imported project adopts the Tricount registry's own currency,
                // so every amount is already in the project currency.
                let expense_payload = match encrypt_json(&enc_key, &ExpensePayload {
                    name: entry.name.to_string(),
                    amount: entry.total,
                    expense_type: expense_type.as_str().to_string(),
                    date: entry.date.to_string(),
                    description: None,
                    category: None,
                    source_currency: None,
                    source_amount: None,
                    rate: None,
                    recurring_id: None,
                    estimate: false,
                    author_id: None,
                    shares: None,
                }) {
                    Ok(p) => p,
                    Err(_) => continue,
                };

                let payers = match encrypt_json(&enc_key, &PaymentPayload { amount: entry.total, is_debt: false })
                    .map(|payload| vec![EncryptedUserAmount { user_id: payer_id, payload }])
                {
                    Ok(v) => v,
                    Err(_) => continue,
                };

                let debtors: Vec<EncryptedUserAmount> = match debtor_ids
                    .iter()
                    .map(|(uid, amt)| {
                        encrypt_json(&enc_key, &PaymentPayload { amount: *amt, is_debt: true })
                            .map(|payload| EncryptedUserAmount { user_id: *uid, payload })
                    })
                    .collect::<Result<Vec<_>, String>>()
                {
                    Ok(v) => v,
                    Err(_) => continue,
                };

                expenses_batch.push(CreatableExpense {
                    payload: expense_payload,
                    project_id: project.id,
                    author_id: payer_id,
                    payers,
                    debtors,
                    history: None,
                    client_op_id: None,
                });
            }

            if !expenses_batch.is_empty() {
                if let Err(e) = batch_add_expenses(Json(expenses_batch)).await {
                    error_msg.set(Some(error_message(&e)));
                    loading.set(false);
                    return;
                }
            }

            // user_id is None — the user identifies themselves later via UserSelectionModal, which
            // is also what attaches the claim label. Nothing to claim here yet.
            if is_auth {
                let _ = upsert_account_project(Json(UpsertAccountProject {
                    key: account_key.and_then(|ak| wrap_key(&ak, &enc_key)),
                    project_id: project.id,
                    user_id: None,
                    claim_label: None,
                    claim_token: Some(claim_token(&enc_key).to_vec()),
                }))
                .await;
            }

            props.on_close.call(());
            nav.push(Route::ExpensesPage { project_id: project.id });
        });
    };

    let on_close = props.on_close;

    rsx! {
        div { class: "modal modal-open modal-bottom sm:modal-middle", role: "dialog",
            div { class: "modal-box max-w-sm p-0 flex flex-col",
                div { class: "flex items-center justify-between px-6 pt-5 pb-4 border-b border-base-200 flex-shrink-0",
                    h3 { class: "font-bold text-lg font-display", {tid!("projects-import-tricount")} }
                    button {
                        r#type: "button",
                        class: "btn btn-ghost btn-circle h-11 w-11 min-h-11",
                        aria_label: tid!("close"),
                        onclick: move |_| on_close.call(()),
                        CloseIcon { size: ICON_HEADER }
                    }
                }
                form { class: "flex flex-col flex-1 overflow-hidden", onsubmit: on_submit,
                    div { class: "flex-1 overflow-y-auto px-6 py-4 flex flex-col gap-4",
                        if let Some(err) = error_msg() {
                            div { role: "alert", class: "alert alert-error text-sm", "{err}" }
                        }
                        fieldset { class: "fieldset",
                            label { class: "fieldset-legend", r#for: "tricount-link-input", {tid!("import-tricount-link-label")} }
                            div { class: "join w-full",
                                input {
                                    id: "tricount-link-input",
                                    class: "input join-item flex-1 min-w-0",
                                    r#type: "text",
                                    placeholder: "https://tricount.com/AbCdEf123",
                                    value: "{tricount_key}",
                                    oninput: move |e| tricount_key.set(e.value()),
                                }
                                button {
                                    id: "paste-btn",
                                    r#type: "button",
                                    class: "btn btn-outline join-item shrink-0",
                                    onclick: move |_| {
                                        match &clipboard_ctx {
                                            Some(reader) => {
                                                if let Some(text) = reader() {
                                                    let text = text.trim().to_string();
                                                    if !text.is_empty() {
                                                        tricount_key.set(text);
                                                        error_msg.set(None);
                                                    }
                                                }
                                            }
                                            None => {
                                                spawn(async move {
                                                    #[cfg(target_arch = "wasm32")]
                                                    let pasted = crate::common::web_dom::read_clipboard()
                                                        .await;
                                                    #[cfg(not(target_arch = "wasm32"))]
                                                    let pasted = document::eval(
                                                            crate::common::READ_CLIPBOARD_JS,
                                                        )
                                                        .await
                                                        .ok()
                                                        .and_then(|v| v.as_str().map(str::to_string));
                                                    if let Some(text) = pasted {
                                                        let text = text.trim().to_string();
                                                        if !text.is_empty() {
                                                            tricount_key.set(text);
                                                            error_msg.set(None);
                                                        }
                                                    }
                                                });
                                            }
                                        }
                                    },
                                    {tid!("paste")}
                                }
                            }
                        }
                    }
                    div { class: "flex justify-end gap-2 px-6 py-4 border-t border-base-200 flex-shrink-0",
                        button {
                            r#type: "button",
                            class: "btn",
                            onclick: move |_| props.on_close.call(()),
                            {tid!("cancel")}
                        }
                        button {
                            r#type: "submit",
                            class: "btn btn-primary",
                            disabled: loading(),
                            if loading() { {tid!("importing")} } else { {tid!("import")} }
                        }
                    }
                }
            }
            div { class: "modal-backdrop", onclick: move |_| props.on_close.call(()) }
        }
    }
}

/// One Tricount entry, resolved to member uuids and validated, before any participant exists.
#[derive(Debug, PartialEq)]
struct PlannedEntry<'a> {
    name: &'a str,
    date: &'a str,
    total: f64,
    transfer: bool,
    payer: &'a str,
    debtors: Vec<(&'a str, f64)>,
}

/// The entries to import, or how many cannot be imported faithfully. Checked before the project
/// is created, so a refusal leaves nothing behind.
///
/// Only plain members (`RegistryMembershipNonUser`) become participants. An entry paid by, or
/// owed by, anyone else used to be imported without them — its debts landed on the last remaining
/// debtor, a wrong balance from day one with no warning. Entries without a description or an
/// amount are not expenses and are skipped, as before.
fn plan_entries<'a>(
    entries: impl Iterator<Item = &'a serde_json::Value>,
    members: &HashSet<&str>,
) -> Result<Vec<PlannedEntry<'a>>, usize> {
    let member = |membership: &'a serde_json::Value| {
        membership["RegistryMembershipNonUser"]["uuid"].as_str().filter(|u| members.contains(u))
    };
    let mut planned = Vec::new();
    let mut unimportable = 0;
    for entry in entries {
        let Some(name) = entry["description"].as_str().filter(|s| !s.is_empty()) else { continue };
        let Some(total) = entry["amount"]["value"].as_str().and_then(|s| s.parse::<f64>().ok())
        else {
            continue;
        };
        let total = round_currency(total.abs());
        // `s.get(..10)`, not `&s[..10]`: this is a field of a third-party API response, and
        // byte-slicing panics when index 10 is not a UTF-8 char boundary. `len() >= 10` does not
        // rule that out — it counts bytes.
        let date = entry["created"].as_str().and_then(|s| s.get(..10)).unwrap_or("2000-01-01");

        let Some(payer) = member(&entry["membership_owned"]) else {
            unimportable += 1;
            continue;
        };
        let mut debtors = Vec::new();
        let mut mapped = true;
        for alloc in entry["allocations"].as_array().into_iter().flatten() {
            let amount = alloc["amount"]["value"].as_str().and_then(|s| s.parse::<f64>().ok());
            let amount = round_currency(amount.unwrap_or(0.0).abs());
            if amount == 0.0 {
                continue;
            }
            match member(&alloc["membership"]) {
                Some(uuid) => debtors.push((uuid, amount)),
                None => mapped = false,
            }
        }
        // Tricount's allocations can round off against its own total, and imports bypass the form
        // — nothing else would catch it before the expense lands permanently flagged inconsistent.
        if !mapped || !balance_allocations(total, &mut debtors) {
            unimportable += 1;
            continue;
        }
        let transfer = entry["type_transaction"].as_str() == Some("BALANCE");
        planned.push(PlannedEntry { name, date, total, transfer, payer, debtors });
    }
    if unimportable > 0 {
        Err(unimportable)
    } else {
        Ok(planned)
    }
}

/// Makes the debtors sum to the total exactly. A lone debtor owes the whole total whatever its
/// allocation says. Several absorb at most a cent each of rounding, pushed onto the last one; a
/// larger gap is bad data, and absorbing it silently moved someone's debt onto someone else.
fn balance_allocations<T>(total: f64, allocations: &mut [(T, f64)]) -> bool {
    let sum: f64 = allocations.iter().map(|(_, a)| a).sum();
    if sums_to_total(total, [sum]) {
        return true;
    }
    let count = allocations.len();
    let Some(last) = allocations.last_mut() else {
        return false;
    };
    if count > 1 && (total - sum).abs() > 0.01 * count as f64 + 1e-9 {
        return false;
    }
    let fixed = round_currency(last.1 + total - sum);
    if fixed < 0.0 {
        return false;
    }
    last.1 = fixed;
    true
}

#[cfg(test)]
mod tests {
    use super::{balance_allocations, plan_entries, PlannedEntry};
    use serde_json::{json, Value};
    use shared::sums_to_total;
    use std::collections::HashSet;

    fn non_user(uuid: &str) -> Value {
        json!({ "RegistryMembershipNonUser": { "uuid": uuid } })
    }

    fn entry(payer: Value, allocations: &[(Value, &str)]) -> Value {
        json!({
            "description": "Dinner",
            "created": "2026-09-01 20:00:00",
            "amount": { "value": "-30.00" },
            "membership_owned": payer,
            "allocations": allocations
                .iter()
                .map(|(m, a)| json!({ "membership": m, "amount": { "value": a } }))
                .collect::<Vec<_>>(),
        })
    }

    fn members() -> HashSet<&'static str> {
        ["a", "b"].into_iter().collect()
    }

    #[test]
    fn a_plain_split_is_planned() {
        let entries = [entry(non_user("a"), &[(non_user("a"), "-15.00"), (non_user("b"), "-15.00")])];
        let planned = plan_entries(entries.iter(), &members()).unwrap();
        assert_eq!(
            planned,
            vec![PlannedEntry {
                name: "Dinner",
                date: "2026-09-01",
                total: 30.0,
                transfer: false,
                payer: "a",
                debtors: vec![("a", 15.0), ("b", 15.0)],
            }]
        );
    }

    // The M11 case: a member with a Tricount account is another variant, and has no participant.
    #[test]
    fn a_debtor_who_is_not_a_plain_member_refuses_the_import() {
        let account_member = json!({ "RegistryMembershipUser": { "uuid": "c" } });
        let entries = [entry(non_user("a"), &[(non_user("a"), "-15.00"), (account_member, "-15.00")])];
        assert_eq!(plan_entries(entries.iter(), &members()), Err(1));
    }

    #[test]
    fn an_unknown_payer_refuses_the_import() {
        let entries = [entry(non_user("z"), &[(non_user("a"), "-30.00")])];
        assert_eq!(plan_entries(entries.iter(), &members()), Err(1));
    }

    #[test]
    fn allocations_far_from_the_total_refuse_the_import() {
        let entries = [entry(non_user("a"), &[(non_user("a"), "-10.00"), (non_user("b"), "-10.00")])];
        assert_eq!(plan_entries(entries.iter(), &members()), Err(1));
    }

    #[test]
    fn entries_that_are_not_expenses_are_skipped_not_counted() {
        let entries = [json!({ "amount": { "value": "-5.00" } }), json!({ "description": "x" })];
        assert_eq!(plan_entries(entries.iter(), &members()), Ok(vec![]));
    }

    #[test]
    fn a_gap_beyond_a_cent_per_debtor_is_refused() {
        let mut allocations = vec![(1, 50.0), (2, 49.97)];
        assert!(!balance_allocations(100.0, &mut allocations));
    }

    #[test]
    fn absorbs_a_rounding_cent_on_the_last_allocation() {
        let mut allocations = vec![(1, 33.33), (2, 33.33), (3, 33.33)];
        assert!(balance_allocations(100.0, &mut allocations));
        assert_eq!(allocations[2].1, 33.34);
        assert!(sums_to_total(100.0, allocations.iter().map(|(_, a)| *a)));
    }

    #[test]
    fn leaves_an_exact_split_alone() {
        let mut allocations = vec![(1, 50.0), (2, 50.0)];
        assert!(balance_allocations(100.0, &mut allocations));
        assert_eq!(allocations, vec![(1, 50.0), (2, 50.0)]);
    }

    #[test]
    fn a_single_allocation_takes_the_whole_total() {
        let mut allocations = vec![(1, 9.99)];
        assert!(balance_allocations(10.0, &mut allocations));
        assert_eq!(allocations[0].1, 10.0);
    }

    #[test]
    fn an_over_allocation_is_trimmed() {
        let mut allocations = vec![(1, 50.0), (2, 50.01)];
        assert!(balance_allocations(100.0, &mut allocations));
        assert_eq!(allocations[1].1, 50.0);
    }

    #[test]
    fn an_empty_slice_is_refused_without_panicking() {
        assert!(!balance_allocations::<i32>(10.0, &mut []));
    }

    #[test]
    fn a_correction_that_would_go_negative_is_refused() {
        // 50.50 allocated against a 1.00 total is not a rounding artefact: absorbing it would owe
        // the last debtor −49.50. The entry is skipped instead.
        let mut allocations = vec![(1, 50.0), (2, 0.5)];
        assert!(!balance_allocations(1.0, &mut allocations));
        assert_eq!(allocations, vec![(1, 50.0), (2, 0.5)], "the slice must be left untouched");
    }

    #[test]
    fn a_lone_allocation_is_never_driven_negative() {
        // The only allocation always takes the whole total — it can't come out below zero.
        let mut allocations = vec![(1, 50.0)];
        assert!(balance_allocations(1.0, &mut allocations));
        assert_eq!(allocations[0].1, 1.0);
    }
}
