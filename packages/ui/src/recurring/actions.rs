use api::recurring::recurring_controller::{delete_recurring_expense, edit_recurring_expense};
use chrono::NaiveDate;
use dioxus::fullstack::Json;
use dioxus::prelude::*;
use shared::DeleteRecurringExpenseRequest;
use uuid::Uuid;

use super::model::{DecryptedRule, RecurringPayload, Side, Template};
use super::requests::{edit_request, history};
use super::schedule::{nth_date, resume_index};
use crate::common::{error_message, is_recurring_stale_error, Flash};
use crate::tid;

pub fn paused(p: &RecurringPayload) -> RecurringPayload {
    RecurringPayload { paused: true, ..p.clone() }
}

pub fn resumed(p: &RecurringPayload, today: NaiveDate) -> RecurringPayload {
    RecurringPayload { paused: false, next: resume_index(&p.rule, p.next, today), ..p.clone() }
}

/// The date of the last occurrence already written, which a re-anchored rule must stay after:
/// its `client_op_id` would collide and abort every later materialization.
pub fn last_written(p: &RecurringPayload) -> Option<NaiveDate> {
    p.next.checked_sub(1).and_then(|n| nth_date(&p.rule, n))
}

pub fn next_on_is_valid(p: &RecurringPayload, next_on: NaiveDate) -> bool {
    last_written(p).is_none_or(|last| next_on > last)
}

/// The template repriced at `rate`. Exact sides become proportional weights, since their old
/// amounts no longer add up to the new total.
pub fn with_rate(t: &Template, rate: f64) -> Template {
    let Some(source_amount) = t.source_amount else { return t.clone() };
    let weights = |s: &Side| Side { exact: false, entries: s.entries.clone() };
    Template {
        amount: shared::convert_to_project(source_amount, rate),
        rate: Some(rate),
        rate_ack: None,
        payers: if t.payers.exact { weights(&t.payers) } else { t.payers.clone() },
        debtors: if t.debtors.exact { weights(&t.debtors) } else { t.debtors.clone() },
        ..t.clone()
    }
}

pub fn rate_kept(t: &Template, rate: f64) -> Template {
    Template { rate_ack: Some(rate), ..t.clone() }
}

/// The history line a save writes: pausing and resuming are named, anything else is an edit.
pub fn change_key(before: &RecurringPayload, after: &RecurringPayload) -> &'static str {
    match (before.paused, after.paused) {
        (false, true) => "history-recurring-paused",
        (true, false) => "history-recurring-resumed",
        _ => "history-recurring-edited",
    }
}

/// Saves `payload` over `rule`, then resyncs either way: on success to show it, on a 409 because
/// another member's version is the one to show. `actor` signs the history row.
pub fn save_rule(
    key: [u8; 32],
    project_id: Uuid,
    rule: DecryptedRule,
    payload: RecurringPayload,
    actor: Option<i32>,
    mut flash: Signal<Option<Flash>>,
    on_done: impl FnOnce() + 'static,
) {
    let summary = tid!(change_key(&rule.payload, &payload), name: payload.template.name.clone());
    let req = match edit_request(&key, project_id, &rule, &payload, history(&key, actor, summary)) {
        Ok(r) => r,
        Err(e) => {
            flash.set(Some(Flash::err(e)));
            return;
        }
    };
    spawn(async move {
        match edit_recurring_expense(Json(req)).await {
            Ok(_) => {}
            Err(e) if is_recurring_stale_error(&e) => {
                flash.set(Some(Flash::err(tid!("error-recurring-stale"))))
            }
            Err(e) => flash.set(Some(Flash::err(error_message(&e)))),
        }
        on_done();
    });
}

pub fn stop_rule(
    key: [u8; 32],
    project_id: Uuid,
    rule: &DecryptedRule,
    actor: Option<i32>,
    mut flash: Signal<Option<Flash>>,
    on_done: impl FnOnce() + 'static,
) {
    let summary = tid!("history-recurring-stopped", name: rule.payload.template.name.clone());
    let req = DeleteRecurringExpenseRequest {
        id: rule.id,
        project_id,
        history: history(&key, actor, summary),
    };
    spawn(async move {
        if let Err(e) = delete_recurring_expense(Json(req)).await {
            flash.set(Some(Flash::err(error_message(&e))));
        }
        on_done();
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::recurring::model::{Ends, Freq, Rule};

    fn d(y: i32, m: u32, day: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, day).unwrap()
    }

    fn payload(next: u32) -> RecurringPayload {
        RecurringPayload {
            template: Template {
                name: "Parking".to_string(),
                expense_type: "expense".to_string(),
                category: None,
                amount: 192.6,
                payers: Side { exact: true, entries: vec![(1, 192.6)] },
                debtors: Side { exact: true, entries: vec![(1, 100.0), (2, 92.6)] },
                source_currency: Some("CHF".to_string()),
                source_amount: Some(180.0),
                rate: Some(1.07),
                rate_ack: None,
                variable: false,
            },
            rule: Rule { freq: Freq::Monthly, interval: 1, anchor: d(2026, 3, 5), ends: Ends::Never },
            next,
            paused: true,
            seed: Uuid::nil(),
        }
    }

    #[test]
    fn a_new_rate_reprices_and_turns_exact_sides_into_weights() {
        let t = with_rate(&payload(0).template, 1.005);
        assert_eq!(t.amount, 180.9);
        assert_eq!(t.rate, Some(1.005));
        assert!(!t.payers.exact && !t.debtors.exact);
        let split = crate::recurring::split::side_amounts(&t.debtors, t.amount, 0);
        assert!(shared::sums_to_total(t.amount, split.iter().map(|(_, a)| *a)));
    }

    #[test]
    fn keeping_the_rate_records_it_and_changes_nothing_else() {
        let t = rate_kept(&payload(0).template, 1.005);
        assert_eq!(t.rate_ack, Some(1.005));
        assert_eq!(t.amount, 192.6);
    }

    #[test]
    fn resuming_unpauses_and_skips_the_missed_dates() {
        let p = resumed(&payload(4), d(2026, 9, 30));
        assert!(!p.paused);
        assert_eq!(nth_date(&p.rule, p.next), Some(d(2026, 10, 5)));
        assert!(paused(&p).paused);
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn a_save_is_named_after_what_changed() {
        let p = payload(3);
        let running = RecurringPayload { paused: false, ..p.clone() };
        assert_eq!(change_key(&running, &p), "history-recurring-paused");
        assert_eq!(change_key(&p, &running), "history-recurring-resumed");
        assert_eq!(change_key(&running, &running), "history-recurring-edited");
        let ids = crate::i18n::locale_ids(crate::i18n::FALLBACK);
        for key in ["history-recurring-paused", "history-recurring-resumed", "history-recurring-edited"] {
            assert!(ids.contains(key), "{key} missing from en.ftl");
        }
    }

    #[test]
    fn the_next_date_must_stay_after_the_last_written_one() {
        let p = payload(7);
        assert_eq!(last_written(&p), Some(d(2026, 9, 5)));
        assert!(!next_on_is_valid(&p, d(2026, 9, 5)));
        assert!(next_on_is_valid(&p, d(2026, 9, 6)));
        assert!(next_on_is_valid(&payload(0), d(2020, 1, 1)));
    }
}
