use api::recurring::recurring_controller::add_recurring_expense;
use dioxus::fullstack::Json;
use dioxus::prelude::*;
use crate::tid;
use shared::{
    CreatableExpense, Expense, ExpensePayload, ExpenseType, HistoryAction, HistoryContext,
    HistoryEntity, HistoryPayload, User,
};
use std::collections::VecDeque;
use uuid::Uuid;

use crate::common::{
    error_message, fx_cache, queued_expenses, today_iso, write_queue, OpKind, ProjectKey, QueuedOp,
};

use super::expense_form::{ExpenseForm, RepeatSlot};
use crate::expenses::hooks::use_project_store::ProjectStore;
use crate::recurring::requests::{create_request, history};
use crate::recurring::split::side_from_entries;
use crate::recurring::view::{new_payload, template_from_form, Repeat};
use super::super::helpers::expense_form_helpers::{seed_entries, UserEntry};
use super::super::helpers::expense_modal_helpers::{
    conversion_matches_total, expense_payload, resolve_conversion, validate_expense_form,
    with_shares, Conversion, ValidatedExpense,
};
use crate::icons::{RightArrowIcon, WalletIcon};
use super::super::hooks::use_fx_rates::use_fx_rates;
use crate::crypto::encrypt_padded_json;
use crate::decrypted::user_names;

/// An add as queued: listed under a pending id until the replay brings the server's row.
#[derive(Clone, PartialEq)]
pub struct QueuedAdd {
    pub op_id: Uuid,
    pub row: Expense,
}

#[derive(Props, Clone, PartialEq)]
pub struct AddExpenseModalProps {
    pub on_close: EventHandler<()>,
    /// `None` for a recurring rule, which is written at once rather than queued.
    pub on_created: EventHandler<Option<QueuedAdd>>,
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
    /// The key to encrypt with, when the modal is opened outside the project's own page: the
    /// `ProjectKey` context holds whichever project the store last followed.
    #[props(default)]
    pub project_key: Option<[u8; 32]>,
    /// "Mes dépenses": no payer or beneficiary sections, and no transfer.
    #[props(default)]
    pub personal: bool,
    /// The currency `initial_amount` is in, when not the project's: the form converts it.
    #[props(default)]
    pub initial_currency: Option<String>,
    /// Fields the form does not show, kept from the expense this one replaces (a move).
    #[props(default)]
    pub carried: Option<ExpensePayload>,
    /// "From A to B": the form is moving an expense here, so it says so and offers no repeat.
    #[props(default)]
    pub move_banner: Option<String>,
}

#[component]
pub fn AddExpenseModal(props: AddExpenseModalProps) -> Element {
    let followed_key = use_context::<ProjectKey>().0;
    let own_key = props.project_key;
    let key_ctx = move || own_key.or_else(&*followed_key);

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
    // Starts as the project's currency, so a single-currency project behaves exactly as before -
    // no rate field, and `use_fx_rates` stays disabled, so nothing is fetched.
    let init_currency = props.initial_currency.clone().unwrap_or_else(|| props.currency.clone());
    let expense_currency = use_signal(move || init_currency);
    let source_amount = use_signal(move || init_amount);
    let rate_input: Signal<String> = use_signal(String::new);
    let init_category = props.initial_category.clone();
    let category: Signal<Option<String>> = use_signal(move || init_category);
    let payers_share_mode = use_signal(|| false);
    let debtors_share_mode = use_signal(|| false);

    // Names for display - blank when the key is unavailable.
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
    let repeat: Signal<Option<Repeat>> = use_signal(|| None);
    let store = use_context::<ProjectStore>();
    let today = store
        .recurring_for(props.project_id)
        .map(|r| r.today())
        .unwrap_or_else(crate::common::local_today);

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
    let carried = props.carried.clone();

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

        if let Some(r) = repeat() {
            if !is_online() {
                error_msg.set(Some(tid!("repeat-offline")));
                return;
            }
            let Ok(anchor) = chrono::NaiveDate::parse_from_str(&form.date, "%Y-%m-%d") else {
                error_msg.set(Some(tid!("expense-invalid-date")));
                return;
            };
            let template = template_from_form(
                &form,
                &expense_type(),
                category(),
                conversion.as_ref(),
                side_from_entries(&payers(), payers_share_mode()),
                side_from_entries(&debtors(), debtors_share_mode()),
                r.variable,
            );
            let summary = tid!("history-recurring-created", name: name_val.clone());
            let history = history(&key, Some(author_id), summary, HistoryAction::Created);
            let req = match create_request(&key, project_id, author_id, &new_payload(template, r, anchor), history) {
                Ok(req) => req,
                Err(e) => {
                    error_msg.set(Some(e));
                    return;
                }
            };
            loading.set(true);
            error_msg.set(None);
            spawn(async move {
                crate::common::project_token::scope(req.project_id);
                match add_recurring_expense(Json(req)).await {
                    Ok(_) => {
                        // The rule is the server's to list: the sync brings it, and the materializer
                        // writes its due occurrences from there.
                        let mut sync = store.sync;
                        sync.restart();
                        on_created.call(None);
                        on_close_submit.call(());
                    }
                    Err(e) => {
                        error_msg.set(Some(error_message(&e)));
                        loading.set(false);
                    }
                }
            });
            return;
        }

        let mut payload = match build_creatable_expense(
            &key,
            &form,
            &expense_type(),
            category(),
            project_id,
            author_id,
            tid!("history-expense-added", name: name_val.clone()),
            conversion.as_ref(),
            carried.as_ref(),
        ) {
            Ok(p) => p,
            Err(e) => {
                error_msg.set(Some(e));
                return;
            }
        };

        // Online or not, the add goes through the queue and the sheet closes now: the row is listed
        // as pending until the replay lands it. The op's id rides inside the payload so every
        // replay of it names the same write.
        let op_id = Uuid::new_v4();
        payload.client_op_id = Some(op_id);
        pending_ops.write().push_back(QueuedOp::new(op_id, &name_val, OpKind::AddExpense(payload)));
        write_queue(&pending_ops.read());
        let row = queued_expenses(&pending_ops.read(), project_id, chrono::Utc::now().naive_utc()).pop();
        on_created.call(row.map(|row| QueuedAdd { op_id, row }));
        on_close_submit.call(());
    };

    let title =
        if props.restrict_to_transfer { tid!("transfer-add") } else { tid!("expense-add") };
    let moving = props.move_banner.is_some();
    let submit_label = if moving {
        tid!("move-submit")
    } else if repeat().is_some() {
        tid!("add-and-repeat")
    } else {
        tid!("add")
    };
    let (banner, banner_icon) = match props.move_banner.clone() {
        Some(b) => (Some(b), Some(rsx! { RightArrowIcon { size: 18 } })),
        None if props.personal => (Some(tid!("personal-expense-banner")), Some(rsx! { WalletIcon { size: 18 } })),
        None => (None, None),
    };

    rsx! {
        ExpenseForm {
            categories: categories_of(props.project_id),
            manage_categories: Some(props.project_id),
            title,
            submit_label,
            repeat: (!props.restrict_to_transfer && !moving).then_some(RepeatSlot { value: repeat, today, backfill: true }),
            loading_label: tid!("adding"),
            currency: props.currency.clone(),
            show_type_selector: !props.restrict_to_transfer,
            personal: props.personal,
            banner,
            banner_icon,
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
    carried: Option<&ExpensePayload>,
) -> Result<CreatableExpense, String> {
    // `form.total` is already in the project currency - the form converts before validating, so
    // the payers and debtors below split the same number the payload records.
    let mut payload = expense_payload(&form.name, form.total, &form.date, expense_type, category, conversion);
    if let Some(c) = carried {
        payload.description = c.description.clone();
        payload.estimate = c.estimate;
        payload.extra = c.extra.clone();
    }
    let payload = with_shares(payload, Some(author_id), &form.payers, &form.debtors);
    Ok(CreatableExpense {
        payload: encrypt_padded_json(key, &payload)?,
        project_id,
        author_id: None,
        payers: Vec::new(),
        debtors: Vec::new(),
        history: Some(HistoryContext {
            actor_user_id: None,
            payload: encrypt_padded_json(
                key,
                &HistoryPayload {
                    summary: history_summary,
                    actor_user_id: Some(author_id),
                    action: Some(HistoryAction::Created),
                    entity: Some(HistoryEntity::Expense),
                    entity_id: None,
                },
            )?,
        }),
        client_op_id: None,
    })
}

/// The project's own categories for the picker, when its data is the one loaded.
fn categories_of(project_id: Uuid) -> crate::categories::ProjectCategories {
    try_consume_context::<crate::expenses::hooks::use_project_store::ProjectStore>()
        .and_then(|s| s.data_for(project_id))
        .map(|d| d.categories.clone())
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypto::decrypt_json;
    use shared::ExpensePayload;

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
            None,
        )
        .unwrap()
    }

    #[test]
    fn a_moved_expense_keeps_what_the_form_does_not_show() {
        let mut carried: ExpensePayload = decrypt_json(&test_key(), &build("x", 1.0, &[(1, 1.0)], &[(1, 1.0)]).payload).unwrap();
        carried.description = Some("two nights".into());
        carried.estimate = true;
        carried.recurring_id = Some(Uuid::new_v4());
        carried.extra.insert("tags".into(), serde_json::json!(["work"]));
        let ce = build_creatable_expense(
            &test_key(),
            &validated("Hotel", 92.0, &[(1, 92.0)], &[(1, 92.0)]),
            &ExpenseType::Expense,
            None,
            Uuid::nil(),
            1,
            "added".into(),
            None,
            Some(&carried),
        )
        .unwrap();
        let ep: ExpensePayload = decrypt_json(&test_key(), &ce.payload).unwrap();
        assert_eq!(ep.name, "Hotel");
        assert_eq!(ep.amount, 92.0);
        assert_eq!(ep.description.as_deref(), Some("two nights"));
        assert!(ep.estimate);
        assert_eq!(ep.extra["tags"], serde_json::json!(["work"]));
        assert_eq!(ep.recurring_id, None, "a moved occurrence leaves its rule behind");
    }

    #[test]
    fn expense_name_roundtrip() {
        let ce = build("Dîner", 45.0, &[(1, 45.0)], &[(2, 45.0)]);
        let ep: ExpensePayload = decrypt_json(&test_key(), &ce.payload).unwrap();
        assert_eq!(ep.name, "Dîner");
    }

    /// Who paid, who owes and who entered it travel inside the payload; nothing names a
    /// participant in clear.
    #[test]
    fn participants_and_author_are_only_in_the_ciphertext() {
        let ce = build("x", 30.0, &[(1, 20.0), (2, 10.0)], &[(3, 30.0)]);
        assert!(ce.payers.is_empty() && ce.debtors.is_empty());
        assert_eq!(ce.author_id, None);
        let ep: ExpensePayload = decrypt_json(&test_key(), &ce.payload).unwrap();
        assert_eq!(ep.author_id, Some(1));
        let shares = ep.shares.unwrap();
        assert_eq!(shares.iter().filter(|s| !s.is_debt).count(), 2);
        assert_eq!(shares.iter().filter(|s| s.is_debt).map(|s| s.user_id).collect::<Vec<_>>(), vec![3]);
    }

    #[test]
    fn the_history_actor_is_only_in_the_ciphertext() {
        let ctx = build("x", 30.0, &[(1, 30.0)], &[(2, 30.0)]).history.unwrap();
        assert_eq!(ctx.actor_user_id, None);
        let hp: HistoryPayload = decrypt_json(&test_key(), &ctx.payload).unwrap();
        assert_eq!(hp.actor_user_id, Some(1));
        assert_eq!((hp.action, hp.entity, hp.entity_id), (Some(HistoryAction::Created), Some(HistoryEntity::Expense), None));
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
