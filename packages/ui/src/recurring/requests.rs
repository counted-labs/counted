use chrono::NaiveDate;
use shared::{
    CreatableExpense, CreatableRecurringExpense, EditableRecurringExpense, ExpensePayload,
    HistoryContext, HistoryPayload, MaterializeRecurringRequest,
};
use uuid::Uuid;

use super::model::{DecryptedRule, RecurringPayload};
use super::split::side_amounts;
use crate::crypto::encrypt_json;
use crate::expenses::helpers::expense_modal_helpers::encrypt_user_amounts;

/// A rule's history row. `None` without an actor to sign it, like an expense with no author.
pub fn history(key: &[u8; 32], actor: Option<i32>, summary: String) -> Option<HistoryContext> {
    Some(HistoryContext {
        actor_user_id: actor?,
        payload: encrypt_json(key, &HistoryPayload { summary, actor_user_id: None }).ok()?,
    })
}

pub fn create_request(
    key: &[u8; 32],
    project_id: Uuid,
    author_id: i32,
    payload: &RecurringPayload,
    history: Option<HistoryContext>,
) -> Result<CreatableRecurringExpense, String> {
    Ok(CreatableRecurringExpense {
        project_id,
        author_id,
        participant_ids: payload.template.participant_ids(),
        payload: encrypt_json(key, payload)?,
        history,
    })
}

pub fn edit_request(
    key: &[u8; 32],
    project_id: Uuid,
    rule: &DecryptedRule,
    payload: &RecurringPayload,
    history: Option<HistoryContext>,
) -> Result<EditableRecurringExpense, String> {
    Ok(EditableRecurringExpense {
        id: rule.id,
        project_id,
        participant_ids: payload.template.participant_ids(),
        payload: encrypt_json(key, payload)?,
        expected_version: rule.version,
        history,
    })
}

/// Claims `due` (consecutive, oldest first) and writes one expense per date. `summary` gives each
/// occurrence's history line; it is passed in already translated so this stays pure.
pub fn materialize_request(
    key: &[u8; 32],
    project_id: Uuid,
    rule: &DecryptedRule,
    due: &[(u32, NaiveDate)],
    actor_user_id: i32,
    summary: impl Fn(NaiveDate) -> String,
) -> Result<MaterializeRecurringRequest, String> {
    let (Some(first), Some(last)) = (due.first(), due.last()) else {
        return Err("nothing due".to_string());
    };
    if first.0 != rule.payload.next {
        return Err("due dates do not start at the cursor".to_string());
    }
    let t = &rule.payload.template;
    let occurrences = due
        .iter()
        .map(|(index, date)| {
            let payload = ExpensePayload {
                name: t.name.clone(),
                amount: t.amount,
                expense_type: t.expense_type.clone(),
                date: date.format("%Y-%m-%d").to_string(),
                description: None,
                category: t.category.clone(),
                source_currency: t.source_currency.clone(),
                source_amount: t.source_amount,
                rate: t.rate,
                recurring_id: Some(rule.id),
                estimate: t.variable,
                author_id: None,
                shares: None,
            };
            Ok(CreatableExpense {
                project_id,
                author_id: rule.author_id.unwrap_or(actor_user_id),
                payload: encrypt_json(key, &payload)?,
                payers: encrypt_user_amounts(key, &side_amounts(&t.payers, t.amount, *index), false)?,
                debtors: encrypt_user_amounts(key, &side_amounts(&t.debtors, t.amount, *index), true)?,
                history: Some(HistoryContext {
                    actor_user_id: rule.author_id.unwrap_or(actor_user_id),
                    payload: encrypt_json(key, &HistoryPayload { summary: summary(*date), actor_user_id: None })?,
                }),
                client_op_id: Some(super::schedule::occurrence_id(rule.payload.seed, *date)),
            })
        })
        .collect::<Result<Vec<_>, String>>()?;
    let advanced = RecurringPayload { next: last.0 + 1, ..rule.payload.clone() };
    Ok(MaterializeRecurringRequest {
        id: rule.id,
        project_id,
        expected_version: rule.version,
        payload: encrypt_json(key, &advanced)?,
        due_through: last.1,
        occurrences,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::test_fixtures::test_key;
    use crate::crypto::decrypt_json;
    use crate::recurring::model::{Ends, Freq, Rule, Side, Template};
    use shared::PaymentPayload;

    fn d(y: i32, m: u32, day: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, day).unwrap()
    }

    fn rule(next: u32) -> DecryptedRule {
        DecryptedRule {
            id: Uuid::from_u128(1),
            version: 4,
            author_id: Some(1),
            created_at: chrono::NaiveDateTime::default(),
            payload: RecurringPayload {
                template: Template {
                    name: "Rent".to_string(),
                    expense_type: "expense".to_string(),
                    category: None,
                    amount: 1450.0,
                    payers: Side { exact: true, entries: vec![(1, 1450.0)] },
                    debtors: Side { exact: false, entries: vec![(1, 1.0), (2, 1.0), (3, 1.0)] },
                    source_currency: None,
                    source_amount: None,
                    rate: None,
                    rate_ack: None,
                    variable: true,
                },
                rule: Rule { freq: Freq::Monthly, interval: 1, anchor: d(2026, 8, 1), ends: Ends::Never },
                next,
                paused: false,
                seed: Uuid::from_u128(99),
                author_id: None,
            },
        }
    }

    #[test]
    fn a_materialization_writes_each_date_and_advances_the_cursor_past_the_last() {
        let key = test_key();
        let r = rule(0);
        let due = [(0, d(2026, 8, 1)), (1, d(2026, 9, 1))];
        let req = materialize_request(&key, Uuid::nil(), &r, &due, 2, |date| format!("auto {date}"))
            .unwrap();
        assert_eq!(req.expected_version, 4);
        assert_eq!(req.due_through, d(2026, 9, 1));
        assert_eq!(req.occurrences.len(), 2);

        let advanced: RecurringPayload = decrypt_json(&key, &req.payload).unwrap();
        assert_eq!(advanced.next, 2);

        let first: ExpensePayload = decrypt_json(&key, &req.occurrences[0].payload).unwrap();
        assert_eq!(first.date, "2026-08-01");
        assert_eq!(first.recurring_id, Some(r.id));
        assert!(first.estimate, "a variable rule writes estimates");
        assert_eq!(req.occurrences[0].author_id, 1, "the rule's author, not the device");
        assert_eq!(
            req.occurrences[0].client_op_id,
            Some(crate::recurring::schedule::occurrence_id(r.payload.seed, d(2026, 8, 1)))
        );
    }

    #[test]
    fn the_extra_cent_follows_the_occurrence_index() {
        let key = test_key();
        let due = [(1, d(2026, 9, 1))];
        let req = materialize_request(&key, Uuid::nil(), &rule(1), &due, 2, |_| String::new()).unwrap();
        let amounts: Vec<f64> = req.occurrences[0]
            .debtors
            .iter()
            .map(|p| decrypt_json::<PaymentPayload>(&key, &p.payload).unwrap().amount)
            .collect();
        assert_eq!(amounts, vec![483.33, 483.34, 483.33]);
    }

    #[test]
    fn a_removed_author_falls_back_to_the_member_materializing() {
        let mut r = rule(0);
        r.author_id = None;
        let req = materialize_request(&test_key(), Uuid::nil(), &r, &[(0, d(2026, 8, 1))], 2, |_| String::new())
            .unwrap();
        assert_eq!(req.occurrences[0].author_id, 2);
    }

    #[test]
    fn dates_that_skip_the_cursor_are_refused() {
        let r = rule(3);
        assert!(materialize_request(&test_key(), Uuid::nil(), &r, &[(4, d(2026, 12, 1))], 2, |_| String::new()).is_err());
        assert!(materialize_request(&test_key(), Uuid::nil(), &r, &[], 2, |_| String::new()).is_err());
    }

    #[test]
    fn create_and_edit_name_every_participant_once() {
        let key = test_key();
        let r = rule(0);
        let c = create_request(&key, Uuid::nil(), 1, &r.payload, None).unwrap();
        assert_eq!(c.participant_ids, vec![1, 2, 3]);
        let e = edit_request(&key, Uuid::nil(), &r, &r.payload, None).unwrap();
        assert_eq!(e.expected_version, 4);
        assert_eq!(e.participant_ids, vec![1, 2, 3]);
    }

    #[test]
    fn a_rule_history_row_needs_an_actor() {
        let key = test_key();
        assert!(history(&key, None, "Rent".to_string()).is_none());
        let ctx = history(&key, Some(2), "Rent".to_string()).unwrap();
        assert_eq!(ctx.actor_user_id, 2);
        let p: HistoryPayload = decrypt_json(&key, &ctx.payload).unwrap();
        assert_eq!(p.summary, "Rent");
    }
}
