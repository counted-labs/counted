use api::expenses::expenses_controller::add_expense;
use dioxus::fullstack::Json;
use dioxus::prelude::*;
use crate::tid;
use shared::{
    CreatableExpense, ExpenseType, ExpenseWithPayments, HistoryContext, HistoryPayload, User,
};
use std::collections::VecDeque;
use uuid::Uuid;

use crate::common::{error_message, fx_cache, write_queue, OpKind, ProjectKey, QueuedOp};

use super::expense_form::ExpenseForm;
use super::super::helpers::expense_form_helpers::{seed_entries, UserEntry};
use super::super::helpers::expense_modal_helpers::{
    conversion_matches_total, encrypt_expense_payload, encrypt_user_amounts, resolve_conversion,
    validate_expense_form, Conversion, ValidatedExpense,
};
use super::super::hooks::use_fx_rates::use_fx_rates;
use crate::crypto::{encrypt_json, user_names};

#[derive(Props, Clone, PartialEq)]
pub struct AddExpenseModalProps {
    pub on_close: EventHandler<()>,
    /// `None` when the op was queued offline — there is no server row to report yet.
    pub on_created: EventHandler<Option<ExpenseWithPayments>>,
    pub project_id: Uuid,
    pub users: Vec<User>,
    pub stored_user_id: Option<i32>,
    pub currency: String,
    // Optional pre-fill (used when opening from a reimbursement suggestion)
    pub initial_name: Option<String>,
    pub initial_amount: Option<f64>,
    pub initial_expense_type: Option<ExpenseType>,
    pub initial_payer_id: Option<i32>,
    pub initial_debtor_id: Option<i32>,
    // When true: type is locked to Transfer and title changes
    #[props(default = false)]
    pub restrict_to_transfer: bool,
    /// `%Y-%m-%d`. Defaults to today. Set by a receipt scan, which reads the printed date.
    pub initial_date: Option<String>,
    /// A `CHART_CATEGORIES` id. Left unset, the form infers one from the name on submit as before.
    pub initial_category: Option<String>,
    /// A message key shown above the amount, or `None`. A scan sets it when the total was not
    /// clearly printed.
    pub amount_hint: Option<&'static str>,
}

/// Today, in the shape `date_str` holds. Extracted so the default stays one expression now that
/// `initial_date` can override it.
fn today_iso() -> String {
    chrono::Utc::now().naive_utc().date().format("%Y-%m-%d").to_string()
}

#[component]
pub fn AddExpenseModal(props: AddExpenseModalProps) -> Element {
    let key_ctx = use_context::<ProjectKey>().0;

    let init_name = props.initial_name.clone().unwrap_or_default();
    let init_amount = props.initial_amount.unwrap_or(0.0);
    let init_type = props.initial_expense_type.clone().unwrap_or(ExpenseType::Expense);
    let initial_payer_id = props.initial_payer_id.or(props.stored_user_id);
    let initial_debtor_id = props.initial_debtor_id;

    let expense_name = use_signal(move || init_name);
    let init_date = props.initial_date.clone().unwrap_or_else(today_iso);
    let date_str = use_signal(move || init_date);
    let total_amount = use_signal(move || init_amount);
    let expense_type = use_signal(move || init_type);
    // Starts as the project's currency, so a single-currency project behaves exactly as before —
    // no rate field, and `use_fx_rates` stays disabled, so nothing is fetched.
    let init_currency = props.currency.clone();
    let expense_currency = use_signal(move || init_currency);
    let source_amount = use_signal(move || init_amount);
    let rate_input: Signal<String> = use_signal(String::new);
    let init_category = props.initial_category.clone();
    let category: Signal<Option<String>> = use_signal(move || init_category);
    let payers_share_mode = use_signal(|| false);
    let debtors_share_mode = use_signal(|| false);

    // Names for display — blank when the key is unavailable.
    let names = user_names(key_ctx().as_ref(), &props.users);

    let init_payers = props.users.clone();
    let init_payer_names = names.clone();
    let payers: Signal<Vec<UserEntry>> = use_signal(move || {
        seed_entries(&init_payers, &init_payer_names, initial_payer_id, init_amount, false)
    });

    // No preselected debtor means "split with everyone", hence check_rest = true.
    let init_debtors = props.users.clone();
    let init_debtor_names = names.clone();
    let debtors: Signal<Vec<UserEntry>> = use_signal(move || {
        seed_entries(&init_debtors, &init_debtor_names, initial_debtor_id, init_amount, true)
    });

    let mut error_msg: Signal<Option<String>> = use_signal(|| None);
    let mut loading = use_signal(|| false);
    let is_online = use_context::<Signal<bool>>();
    let mut pending_ops = use_context::<Signal<VecDeque<QueuedOp>>>();

    // Only ever true once the user picks a different currency, so a single-currency project never
    // reaches `use_fx_rates`' resource body and never hits the server.
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
    let users_for_author = props.users.clone();
    let currency_for_submit = props.currency.clone();
    let on_created = props.on_created;
    let on_close_submit = props.on_close;

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
            Err(e) => {
                error_msg.set(Some(e.message()));
                return;
            }
        };
        let conversion = match resolve_conversion(
            &expense_currency(),
            &currency_for_submit,
            source_amount(),
            &rate_input(),
            auto_rate(),
        ) {
            Ok(c) => c,
            Err(e) => {
                error_msg.set(Some(e.message()));
                return;
            }
        };
        if let Err(e) = conversion_matches_total(form.total, conversion.as_ref()) {
            error_msg.set(Some(e.message()));
            return;
        }

        let name_val = form.name.clone();
        let author_id =
            stored_user_id.unwrap_or_else(|| users_for_author.first().map(|u| u.id).unwrap_or(0));

        let mut payload = match build_creatable_expense(
            &key,
            &form,
            &expense_type(),
            category(),
            project_id,
            author_id,
            tid!("history-expense-added", name: name_val.clone()),
            conversion.as_ref(),
        ) {
            Ok(p) => p,
            Err(e) => {
                error_msg.set(Some(e));
                return;
            }
        };

        if !is_online() {
            // The op's id rides inside the payload so every replay of it names the same write.
            let op_id = Uuid::new_v4();
            payload.client_op_id = Some(op_id);
            pending_ops.write().push_back(QueuedOp {
                id: op_id,
                label: name_val,
                op: OpKind::AddExpense(payload),
            });
            write_queue(&pending_ops.read());
            on_created.call(None);
            on_close_submit.call(());
            return;
        }

        loading.set(true);
        error_msg.set(None);

        let on_created = on_created;
        let on_close_submit = on_close_submit;
        spawn(async move {
            match add_expense(Json(payload)).await {
                Ok(created) => {
                    on_created.call(Some(created));
                    on_close_submit.call(());
                }
                Err(e) => {
                    error_msg.set(Some(error_message(&e)));
                    loading.set(false);
                }
            }
        });
    };

    let title =
        if props.restrict_to_transfer { tid!("transfer-add") } else { tid!("expense-add") };

    rsx! {
        ExpenseForm {
            title,
            submit_label: tid!("add"),
            loading_label: tid!("adding"),
            currency: props.currency.clone(),
            show_type_selector: !props.restrict_to_transfer,
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
            amount_hint: props.amount_hint,
        }
    }
}

// ---- pure helpers ----

#[allow(clippy::too_many_arguments)]
fn build_creatable_expense(
    key: &[u8; 32],
    form: &ValidatedExpense,
    expense_type: &ExpenseType,
    category: Option<String>,
    project_id: Uuid,
    author_id: i32,
    // Already translated by the caller: this stays pure so it is testable without a runtime.
    history_summary: String,
    conversion: Option<&Conversion>,
) -> Result<CreatableExpense, String> {
    Ok(CreatableExpense {
        // `form.total` is already in the project currency — the form converts before validating, so
        // the payers and debtors below split the same number the payload records.
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
        author_id,
        payers: encrypt_user_amounts(key, &form.payers, false)?,
        debtors: encrypt_user_amounts(key, &form.debtors, true)?,
        history: Some(HistoryContext {
            actor_user_id: author_id,
            payload: encrypt_json(
                key,
                &HistoryPayload { summary: history_summary },
            )?,
        }),
        client_op_id: None,
    })
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypto::decrypt_json;
    use shared::{ExpensePayload, PaymentPayload};

    // Payload-level assertions (amount, date, type, description, wrong key) live in
    // expense_modal_helpers; these cover what building a CreatableExpense adds on top.

    use crate::common::test_fixtures::test_key;

    fn validated(
        name: &str,
        amount: f64,
        payers: &[(i32, f64)],
        debtors: &[(i32, f64)],
    ) -> ValidatedExpense {
        ValidatedExpense {
            name: name.to_string(),
            total: amount,
            date: "2025-06-15".to_string(),
            payers: payers.to_vec(),
            debtors: debtors.to_vec(),
        }
    }

    fn build(
        name: &str,
        amount: f64,
        payers: &[(i32, f64)],
        debtors: &[(i32, f64)],
    ) -> CreatableExpense {
        build_with_conversion(name, amount, payers, debtors, None)
    }

    fn build_with_conversion(
        name: &str,
        amount: f64,
        payers: &[(i32, f64)],
        debtors: &[(i32, f64)],
        conversion: Option<&Conversion>,
    ) -> CreatableExpense {
        build_creatable_expense(
            &test_key(),
            &validated(name, amount, payers, debtors),
            &ExpenseType::Expense,
            None,
            Uuid::nil(),
            1,
            format!("added: {name}"),
            conversion,
        )
        .unwrap()
    }

    #[test]
    fn expense_name_roundtrip() {
        let ce = build("Dîner", 45.0, &[(1, 45.0)], &[(2, 45.0)]);
        let ep: ExpensePayload = decrypt_json(&test_key(), &ce.payload).unwrap();
        assert_eq!(ep.name, "Dîner");
    }

    #[test]
    fn expense_payers_are_not_debt() {
        let ce = build("x", 30.0, &[(1, 20.0), (2, 10.0)], &[(3, 30.0)]);
        assert_eq!(ce.payers.len(), 2);
        for p in &ce.payers {
            let pp: PaymentPayload = decrypt_json(&test_key(), &p.payload).unwrap();
            assert!(!pp.is_debt);
        }
    }

    #[test]
    fn expense_debtors_are_debt() {
        let ce = build("x", 30.0, &[(1, 30.0)], &[(2, 15.0), (3, 15.0)]);
        assert_eq!(ce.debtors.len(), 2);
        for d in &ce.debtors {
            let pp: PaymentPayload = decrypt_json(&test_key(), &d.payload).unwrap();
            assert!(pp.is_debt);
        }
    }

    #[test]
    fn expense_project_id_preserved() {
        let pid = Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000").unwrap();
        let ce = build_creatable_expense(
            &test_key(),
            &validated("x", 10.0, &[(1, 10.0)], &[(2, 10.0)]),
            &ExpenseType::Expense,
            None,
            pid,
            1,
            "added: x".into(),
            None,
        )
        .unwrap();
        assert_eq!(ce.project_id, pid);
    }

    #[test]
    fn expense_history_summary_names_the_expense() {
        let ce = build("Dîner", 45.0, &[(1, 45.0)], &[(2, 45.0)]);
        let ctx = ce.history.unwrap();
        let hp: HistoryPayload = decrypt_json(&test_key(), &ctx.payload).unwrap();
        assert!(hp.summary.contains("Dîner"), "the name must reach the history entry: {}", hp.summary);
    }
}
