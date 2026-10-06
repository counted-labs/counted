use chrono::NaiveDate;
use dioxus::prelude::*;
use shared::User;
use uuid::Uuid;

use super::actions::{next_on_is_valid, save_rule};
use super::model::{DecryptedRule, Side};
use super::split::{side_amounts, side_from_entries};
use super::view::{edited_payload, next_date, repeat_of, template_from_form, Repeat};
use crate::common::{format_date, fx_cache, Flash, ProjectKey};
use crate::decrypted::{pickable_users, user_names};
use crate::expenses::helpers::expense_form_helpers::UserEntry;
use crate::expenses::helpers::expense_modal_helpers::{
    conversion_matches_total, resolve_conversion, validate_expense_form,
};
use crate::expenses::hooks::use_fx_rates::use_fx_rates;
use crate::expenses::modals::expense_form::{ExpenseForm, RepeatSlot};
use crate::tid;

/// The form's rows for one side of a template, split as the next occurrence would be.
fn entries_for(users: &[User], names: &[String], side: &Side, total: f64, rotation: u32) -> Vec<UserEntry> {
    let amounts = side_amounts(side, total, rotation);
    users
        .iter()
        .enumerate()
        .map(|(i, u)| {
            let amount = amounts.iter().find(|(id, _)| *id == u.id).map(|(_, a)| *a);
            let weight = side.entries.iter().find(|(id, _)| *id == u.id).map(|(_, w)| *w);
            UserEntry {
                user: u.clone(),
                display_name: names.get(i).cloned().unwrap_or_default(),
                checked: amount.is_some(),
                amount: amount.unwrap_or(0.0),
                shares: if side.exact { amount.map_or(0.0, |_| 1.0) } else { weight.unwrap_or(0.0) },
            }
        })
        .collect()
}

fn uses_shares(side: &Side) -> bool {
    !side.exact && side.entries.windows(2).any(|w| w[0].1 != w[1].1)
}

#[component]
pub fn EditRuleModal(
    rule: DecryptedRule,
    users: Vec<User>,
    project_id: Uuid,
    currency: String,
    today: NaiveDate,
    actor: Option<i32>,
    on_close: EventHandler<()>,
    on_saved: EventHandler<()>,
) -> Element {
    let key_ctx = use_context::<ProjectKey>().0;
    let flash = use_context::<Signal<Option<Flash>>>();
    let t = rule.payload.template.clone();
    let involved: Vec<i32> = t.payers.entries.iter().chain(&t.debtors.entries).map(|(id, _)| *id).collect();
    let users = pickable_users(key_ctx().as_ref(), &users, &involved);
    let names = user_names(key_ctx().as_ref(), &users);
    let next = next_date(&rule.payload).unwrap_or(today);
    let rotation = rule.payload.next;

    let expense_name = use_signal(|| t.name.clone());
    let date_str = use_signal(|| next.format("%Y-%m-%d").to_string());
    let total_amount = use_signal(|| t.amount);
    let expense_type = use_signal(|| t.expense_type());
    let category = use_signal(|| t.category.clone());
    let conversion = t.conversion().map(|(a, c, r)| (a, c.to_string(), r));
    let init_currency = conversion.as_ref().map(|(_, c, _)| c.clone()).unwrap_or_else(|| currency.clone());
    let expense_currency = use_signal(move || init_currency);
    let init_source = conversion.as_ref().map(|(a, _, _)| *a).unwrap_or(t.amount);
    let source_amount = use_signal(move || init_source);
    let init_rate = conversion.as_ref().map(|(_, _, r)| r.to_string()).unwrap_or_default();
    let rate_input = use_signal(move || init_rate);
    let payers_share_mode = use_signal(|| uses_shares(&t.payers));
    let debtors_share_mode = use_signal(|| uses_shares(&t.debtors));
    let (pu, pn, ps, pt) = (users.clone(), names.clone(), t.payers.clone(), t.amount);
    let payers = use_signal(move || entries_for(&pu, &pn, &ps, pt, rotation));
    let (du, dn, ds, dt) = (users.clone(), names.clone(), t.debtors.clone(), t.amount);
    let debtors = use_signal(move || entries_for(&du, &dn, &ds, dt, rotation));
    let initial_repeat = repeat_of(&rule.payload);
    let repeat: Signal<Option<Repeat>> = use_signal(move || Some(initial_repeat));
    let mut error_msg: Signal<Option<String>> = use_signal(|| None);
    let mut loading = use_signal(|| false);

    let project_currency = currency.clone();
    let needs_fx = use_memo(move || expense_currency() != project_currency);
    let fx = use_fx_rates(needs_fx);
    let rate_currency = currency.clone();
    let auto_rate = use_memo(move || {
        let table = fx()?;
        fx_cache::cross_rate(&table, &expense_currency(), &rate_currency)
    });
    let rate_day = use_memo(move || fx().map(|t| t.day));

    let currency_for_submit = currency.clone();
    let on_submit = move |e: FormEvent| {
        e.prevent_default();
        let Some(key) = key_ctx() else {
            error_msg.set(Some(tid!("missing-encryption-key")));
            return;
        };
        let form = match validate_expense_form(&expense_name(), total_amount(), &date_str(), &payers(), &debtors()) {
            Ok(f) => f,
            Err(e) => return error_msg.set(Some(e.message())),
        };
        let conversion = match resolve_conversion(
            &expense_currency(),
            &currency_for_submit,
            source_amount(),
            &rate_input(),
            auto_rate(),
        ) {
            Ok(c) => c,
            Err(e) => return error_msg.set(Some(e.message())),
        };
        if let Err(e) = conversion_matches_total(form.total, conversion.as_ref()) {
            return error_msg.set(Some(e.message()));
        }
        let Some(r) = repeat() else {
            return error_msg.set(Some(tid!("recurring-use-stop")));
        };
        let Ok(next_on) = NaiveDate::parse_from_str(&form.date, "%Y-%m-%d") else {
            return error_msg.set(Some(tid!("expense-invalid-date")));
        };
        if !next_on_is_valid(&rule.payload, next_on) {
            return error_msg.set(Some(tid!("recurring-next-too-early")));
        }
        let template = template_from_form(
            &form,
            &expense_type(),
            category(),
            conversion.as_ref(),
            side_from_entries(&payers(), payers_share_mode()),
            side_from_entries(&debtors(), debtors_share_mode()),
            r.variable,
        );
        let payload = edited_payload(&rule.payload, template, r, next_on);
        loading.set(true);
        save_rule(key, project_id, rule.clone(), payload, actor, flash, move || on_saved.call(()));
    };

    rsx! {
        ExpenseForm {
            title: tid!("recurring-edit-title"),
            submit_label: tid!("save"),
            loading_label: tid!("saving"),
            currency: currency.clone(),
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
            on_close,
            amount_hint: None,
            repeat: RepeatSlot { value: repeat, today, backfill: false },
            date_chips: false,
            banner: tid!("recurring-edit-banner", date: format_date(next)),
        }
    }
}
