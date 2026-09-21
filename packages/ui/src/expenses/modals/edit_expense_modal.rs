use api::expenses::expenses_controller::edit_expense;
use dioxus::fullstack::Json;
use dioxus::prelude::*;
use crate::tid;
use shared::{
    EditableExpense, Expense, ExpensePayload, ExpenseType, ExpenseWithPayments, HistoryContext,
    HistoryPayload, Payment, User,
};
use std::collections::VecDeque;
use uuid::Uuid;

use crate::common::{error_message, fx_cache, write_queue, OpKind, ProjectKey, QueuedOp};

use super::expense_form::ExpenseForm;
use super::super::helpers::expense_form_helpers::{init_entries_from_payments, UserEntry};
use super::super::helpers::expense_modal_helpers::{
    conversion_matches_total, encrypt_expense_payload, encrypt_user_amounts, resolve_conversion,
    validate_expense_form, Conversion, ValidatedExpense,
};
use super::super::hooks::use_fx_rates::use_fx_rates;
use crate::crypto::{decrypt_json, decrypt_payment, encrypt_json, user_names};

#[derive(Props, Clone, PartialEq)]
pub struct EditExpenseModalProps {
    pub on_close: EventHandler<()>,
    /// `None` when the op was queued offline — there is no server row to report yet.
    pub on_edited: EventHandler<Option<ExpenseWithPayments>>,
    pub expense: Expense,
    pub payments: Vec<Payment>,
    pub users: Vec<User>,
    pub project_id: Uuid,
    pub stored_user_id: Option<i32>,
    pub currency: String,
}

#[component]
pub fn EditExpenseModal(props: EditExpenseModalProps) -> Element {
    let key_ctx = use_context::<ProjectKey>().0;
    let key_snap = key_ctx();

    let expense_id = props.expense.id;
    let expense_author_id = props.expense.author_id;

    // Decrypt initial expense values from single payload blob
    let decrypted_payload = key_snap
        .as_ref()
        .and_then(|k| decrypt_json::<ExpensePayload>(k, &props.expense.payload).ok());

    let (initial_name, initial_date, initial_amount, initial_type, initial_category) =
        decrypted_payload
            .as_ref()
            .map(|ep| {
                let etype = match ep.expense_type.as_str() {
                    "transfer" => ExpenseType::Transfer,
                    "gain" => ExpenseType::Gain,
                    _ => ExpenseType::Expense,
                };
                (ep.name.clone(), ep.date.clone(), ep.amount, etype, ep.category.clone())
            })
            .unwrap_or_else(|| {
                let today = chrono::Utc::now().naive_utc().date().format("%Y-%m-%d").to_string();
                (String::new(), today, 0.0, ExpenseType::Expense, None)
            });

    // The currency, amount and rate this expense was booked at, taken from the payload and **not
    // refetched**. Editing the name of a three-week-old expense must not silently reprice it at
    // today's rate; only deliberately changing the currency clears the stored rate.
    //
    // All three or none: a payload carrying only some of them is treated as project-currency, the
    // same reading `DecryptedExpense::conversion` applies.
    let stored_conversion = decrypted_payload.as_ref().and_then(|ep| {
        let currency = ep.source_currency.clone()?;
        let amount = ep.source_amount?;
        let rate = ep.rate?;
        (amount.is_finite() && shared::is_valid_rate(rate)).then_some((currency, amount, rate))
    });

    // Pre-decrypt all payments for payer/debtor initialization
    let decrypted_payments = key_snap
        .as_ref()
        .map(|k| props.payments.iter().filter_map(|p| decrypt_payment(k, p).ok()).collect::<Vec<_>>())
        .unwrap_or_default();

    // Names for display — blank when the key is unavailable.
    let names = user_names(key_snap.as_ref(), &props.users);

    let expense_name = use_signal(|| initial_name.clone());
    let date_str = use_signal(|| initial_date.clone());
    let total_amount = use_signal(|| initial_amount);
    let expense_type = use_signal(|| initial_type.clone());

    // Prefilled from the payload: the form opens showing what was entered, at the rate it was
    // entered at. `rate_input` is seeded with the stored rate rather than left blank, because blank
    // means "use this month's InforEuro rate" — which is exactly the silent repricing to avoid.
    let init_currency = stored_conversion
        .as_ref()
        .map(|(c, _, _)| c.clone())
        .unwrap_or_else(|| props.currency.clone());
    let expense_currency = use_signal(move || init_currency);
    let init_source = stored_conversion.as_ref().map(|(_, a, _)| *a).unwrap_or(initial_amount);
    let source_amount = use_signal(move || init_source);
    let init_rate =
        stored_conversion.as_ref().map(|(_, _, r)| r.to_string()).unwrap_or_default();
    let rate_input: Signal<String> = use_signal(move || init_rate);
    let category: Signal<Option<String>> = use_signal(|| initial_category.clone());
    let payers_share_mode = use_signal(|| false);
    let debtors_share_mode = use_signal(|| false);

    let init_payments_p = decrypted_payments.clone();
    let init_users_p = props.users.clone();
    let init_names_p = names.clone();
    let payers: Signal<Vec<UserEntry>> = use_signal(move || {
        init_entries_from_payments(&init_users_p, &init_names_p, &init_payments_p, false)
    });

    let init_payments_d = decrypted_payments.clone();
    let init_users_d = props.users.clone();
    let init_names_d = names.clone();
    let debtors: Signal<Vec<UserEntry>> = use_signal(move || {
        init_entries_from_payments(&init_users_d, &init_names_d, &init_payments_d, true)
    });

    let mut error_msg: Signal<Option<String>> = use_signal(|| None);
    let mut loading = use_signal(|| false);
    let is_online = use_context::<Signal<bool>>();
    let mut pending_ops = use_context::<Signal<VecDeque<QueuedOp>>>();

    // Same lazy gate as the add modal: an expense already in the project currency, staying there,
    // fetches nothing. An expense that *was* booked in a foreign currency does load the table, so
    // the hint can show today's rate next to the one it is stored at.
    let project_currency = props.currency.clone();
    let needs_fx = use_memo(move || expense_currency() != project_currency);
    let fx = use_fx_rates(needs_fx);

    let rate_currency = props.currency.clone();
    let auto_rate = use_memo(move || {
        let table = fx()?;
        fx_cache::cross_rate(&table, &expense_currency(), &rate_currency)
    });
    let rate_day = use_memo(move || fx().map(|t| t.day));

    let project_id = props.project_id;
    let stored_user_id = props.stored_user_id;
    let on_edited = props.on_edited;
    let on_close_submit = props.on_close;
    let currency_for_submit = props.currency.clone();

    let on_submit = move |e: FormEvent| {
        e.prevent_default();

        let key = match key_ctx() {
            Some(k) => k,
            None => {
                error_msg.set(Some(tid!("missing-encryption-key")));
                return;
            }
        };

        let form = match validate_expense_form(
            &expense_name(),
            total_amount(),
            &date_str(),
            &payers(),
            &debtors(),
        ) {
            Ok(v) => v,
            Err(e) => { error_msg.set(Some(e.message())); return; }
        };
        let conversion = match resolve_conversion(
            &expense_currency(),
            &currency_for_submit,
            source_amount(),
            &rate_input(),
            auto_rate(),
        ) {
            Ok(c) => c,
            Err(e) => { error_msg.set(Some(e.message())); return; }
        };
        if let Err(e) = conversion_matches_total(form.total, conversion.as_ref()) {
            error_msg.set(Some(e.message()));
            return;
        }

        let name_val = form.name.clone();
        let author_id = stored_user_id.or(expense_author_id);

        let payload = match build_editable_expense(
            &key,
            expense_id,
            &form,
            &expense_type(),
            category(),
            project_id,
            author_id,
            tid!("history-expense-edited", name: name_val.clone()),
            conversion.as_ref(),
        ) {
            Ok(p) => p,
            Err(e) => { error_msg.set(Some(e)); return; }
        };

        if !is_online() {
            pending_ops.write().push_back(QueuedOp {
                id: Uuid::new_v4(),
                label: name_val,
                op: OpKind::EditExpense(payload),
            });
            write_queue(&pending_ops.read());
            on_edited.call(None);
            on_close_submit.call(());
            return;
        }

        loading.set(true);
        error_msg.set(None);

        let on_edited = on_edited;
        let on_close_submit = on_close_submit;
        spawn(async move {
            match edit_expense(Json(payload)).await {
                Ok(edited) => {
                    on_edited.call(Some(edited));
                    on_close_submit.call(());
                }
                Err(e) => {
                    error_msg.set(Some(error_message(&e)));
                    loading.set(false);
                }
            }
        });
    };

    rsx! {
        ExpenseForm {
            title: tid!("expense-edit-title"),
            submit_label: tid!("save"),
            loading_label: tid!("saving"),
            currency: props.currency.clone(),
            show_type_selector: true,
            expense_name,
            date_str,
            total_amount,
            expense_type,
            category,
            payers,
            debtors,
            payers_share_mode,
            debtors_share_mode,
            error_msg,
            loading,
            expense_currency,
            source_amount,
            rate_input,
            auto_rate: auto_rate(),
            rate_day: rate_day(),
            on_submit,
            on_close: props.on_close,
        }
    }
}

// ---- pure helpers ----

// Nine arguments, and they stay nine: there is one production caller and one test, so a
// parameter struct would exist solely to satisfy the lint. Keeping them flat is what makes the
// helper callable from a test with no runtime, which is the whole reason it is split out.
#[allow(clippy::too_many_arguments)]
fn build_editable_expense(
    key: &[u8; 32],
    id: i32,
    form: &ValidatedExpense,
    expense_type: &ExpenseType,
    category: Option<String>,
    project_id: Uuid,
    author_id: Option<i32>,
    // Already translated by the caller: this stays pure so it is testable without a runtime.
    history_summary: String,
    conversion: Option<&Conversion>,
) -> Result<EditableExpense, String> {
    // No actor, no history row. This device knows of no participant it is acting as — the original
    // author was removed and nothing is stored locally — and the server validates the actor against
    // the project, so any id invented here would be rejected anyway.
    let history = match author_id {
        Some(actor_user_id) => Some(HistoryContext {
            actor_user_id,
            payload: encrypt_json(
                key,
                &HistoryPayload { summary: history_summary },
            )?,
        }),
        None => None,
    };

    Ok(EditableExpense {
        id,
        payload: encrypt_expense_payload(
            key,
            &form.name,
            form.total,
            &form.date,
            expense_type,
            category,
            conversion,
        )?,
        project_id,
        payers: encrypt_user_amounts(key, &form.payers, false)?,
        debtors: encrypt_user_amounts(key, &form.debtors, true)?,
        author_id,
        history,
    })
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::test_fixtures::{make_payment, test_key};
    use crate::crypto::{decrypt_json, decrypt_payment, DecryptedPayment};
    use shared::{ExpensePayload, PaymentPayload};

    // Payload-level assertions live in expense_modal_helpers; these cover what building an
    // EditableExpense adds on top, plus the payment pre-fill decoding this modal relies on.

    fn validated(
        name: &str,
        amount: f64,
        payers: &[(i32, f64)],
        debtors: &[(i32, f64)],
    ) -> ValidatedExpense {
        ValidatedExpense {
            name: name.to_string(),
            total: amount,
            date: "2025-02-01".to_string(),
            payers: payers.to_vec(),
            debtors: debtors.to_vec(),
        }
    }

    fn build(
        id: i32,
        name: &str,
        amount: f64,
        payers: &[(i32, f64)],
        debtors: &[(i32, f64)],
    ) -> EditableExpense {
        build_editable_expense(
            &test_key(),
            id,
            &validated(name, amount, payers, debtors),
            &ExpenseType::Expense,
            None,
            Uuid::nil(),
            Some(1),
            format!("edited: {name}"),
            None,
        )
        .unwrap()
    }

    #[test]
    fn editable_name_and_id_roundtrip() {
        let ee = build(1, "Loyer", 500.0, &[(1, 500.0)], &[(2, 500.0)]);
        assert_eq!(ee.id, 1);
        let ep: ExpensePayload = decrypt_json(&test_key(), &ee.payload).unwrap();
        assert_eq!(ep.name, "Loyer");
    }

    #[test]
    fn editable_payers_not_debt_debtors_are_debt() {
        let ee = build(1, "x", 30.0, &[(1, 30.0)], &[(2, 15.0), (3, 15.0)]);
        for p in &ee.payers {
            let pp: PaymentPayload = decrypt_json(&test_key(), &p.payload).unwrap();
            assert!(!pp.is_debt);
        }
        for d in &ee.debtors {
            let pp: PaymentPayload = decrypt_json(&test_key(), &d.payload).unwrap();
            assert!(pp.is_debt);
        }
    }

    #[test]
    fn editable_history_summary_names_the_expense() {
        let ee = build(1, "Loyer", 500.0, &[(1, 500.0)], &[(2, 500.0)]);
        let ctx = ee.history.unwrap();
        let hp: HistoryPayload = decrypt_json(&test_key(), &ctx.payload).unwrap();
        assert!(hp.summary.contains("Loyer"), "the name must reach the history entry: {}", hp.summary);
    }

    // ---- decrypt_payment for pre-fill ----

    #[test]
    fn payer_payment_decoded_for_prefill() {
        let key = test_key();
        let p = make_payment(&key, 1, 10, 5, false, 40.0);
        let dp = decrypt_payment(&key, &p).unwrap();
        assert_eq!(dp.user_id, 5);
        assert!(!dp.is_debt);
        assert!((dp.amount - 40.0).abs() < 1e-9);
    }

    #[test]
    fn debtor_payment_decoded_for_prefill() {
        let key = test_key();
        let p = make_payment(&key, 2, 10, 7, true, 20.0);
        let dp = decrypt_payment(&key, &p).unwrap();
        assert_eq!(dp.user_id, 7);
        assert!(dp.is_debt);
    }

    #[test]
    fn wrong_key_skips_payment_in_prefill() {
        let key = test_key();
        let payments = [make_payment(&key, 1, 1, 1, false, 10.0)];
        let mut bad = key;
        bad[0] ^= 0xFF;
        let decrypted: Vec<DecryptedPayment> =
            payments.iter().filter_map(|p| decrypt_payment(&bad, p).ok()).collect();
        assert!(decrypted.is_empty());
    }

    #[test]
    fn payers_and_debtors_split_correctly() {
        let key = test_key();
        let payments = [make_payment(&key, 1, 10, 1, false, 30.0),
            make_payment(&key, 2, 10, 2, true, 15.0),
            make_payment(&key, 3, 10, 3, true, 15.0)];
        let decrypted: Vec<DecryptedPayment> =
            payments.iter().filter_map(|p| decrypt_payment(&key, p).ok()).collect();
        let payers: Vec<_> = decrypted.iter().filter(|p| !p.is_debt).collect();
        let debtors: Vec<_> = decrypted.iter().filter(|p| p.is_debt).collect();
        assert_eq!(payers.len(), 1);
        assert_eq!(debtors.len(), 2);
        assert_eq!(payers[0].user_id, 1);
    }
}
