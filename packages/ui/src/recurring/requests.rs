use chrono::NaiveDate;
use shared::{
    CreatableExpense, CreatableRecurringExpense, EditableRecurringExpense, ExpensePayload,
    HistoryAction, HistoryContext, HistoryEntity, HistoryPayload, MaterializeRecurringRequest,
};
use uuid::Uuid;

use super::model::{DecryptedRule, RecurringPayload};
use super::split::side_amounts;
use crate::crypto::encrypt_padded_json;
use crate::expenses::helpers::expense_modal_helpers::with_shares;

/// A rule's history row. `None` without an actor to sign it, like an expense with no author. The
/// server files rule rows under the project.
pub fn history(key: &[u8; 32], actor: Option<i32>, summary: String, action: HistoryAction) -> Option<HistoryContext> {
    let actor = actor?;
    let payload = HistoryPayload {
        summary,
        actor_user_id: Some(actor),
        action: Some(action),
        entity: Some(HistoryEntity::Project),
        entity_id: None,
    };
    Some(HistoryContext { actor_user_id: None, payload: encrypt_padded_json(key, &payload).ok()? })
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
        author_id: None,
        participant_ids: Vec::new(),
        payload: encrypt_padded_json(
            key,
            &RecurringPayload { author_id: Some(author_id), ..payload.clone() },
        )?,
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
        participant_ids: Vec::new(),
        payload: encrypt_padded_json(
            key,
            &RecurringPayload { author_id: payload.author_id.or(rule.author_id), ..payload.clone() },
        )?,
        expected_version: rule.version,
        history,
    })
}

/// Claims `due` (consecutive, oldest first) and writes one expense per date. `summary` gives each
/// occurrence's history line; it is passed in already translated so this stays pure. `today` is the
/// sync's server date, sent as `due_through` in place of the last occurrence's date, which would
/// reveal the rule's schedule (audit 2026-10-19 M7).
pub fn materialize_request(
    key: &[u8; 32],
    project_id: Uuid,
    rule: &DecryptedRule,
    due: &[(u32, NaiveDate)],
    today: NaiveDate,
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
    let author = rule.author_id.unwrap_or(actor_user_id);
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
                extra: Default::default(),
            };
            let payload = with_shares(
                payload,
                Some(author),
                &side_amounts(&t.payers, t.amount, *index),
                &side_amounts(&t.debtors, t.amount, *index),
            );
            Ok(CreatableExpense {
                project_id,
                author_id: None,
                payload: encrypt_padded_json(key, &payload)?,
                payers: Vec::new(),
                debtors: Vec::new(),
                history: Some(HistoryContext {
                    actor_user_id: None,
                    payload: encrypt_padded_json(
                        key,
                        &HistoryPayload {
                            summary: summary(*date),
                            actor_user_id: Some(author),
                            action: Some(HistoryAction::Created),
                            entity: Some(HistoryEntity::Expense),
                            entity_id: None,
                        },
                    )?,
                }),
                client_op_id: Some(super::schedule::occurrence_id(rule.payload.seed, *date)),
            })
        })
        .collect::<Result<Vec<_>, String>>()?;
    let advanced = RecurringPayload {
        next: last.0 + 1,
        author_id: rule.payload.author_id.or(rule.author_id),
        ..rule.payload.clone()
    };
    Ok(MaterializeRecurringRequest {
        id: rule.id,
        project_id,
        expected_version: rule.version,
        payload: encrypt_padded_json(key, &advanced)?,
        due_through: today,
        occurrences,
    })
}

#[cfg(any(test, fuzzing))]
pub(crate) fn rule_rows(payloads: &[RecurringPayload], author_id: Option<i32>, created_at: chrono::NaiveDateTime) -> Vec<shared::RecurringExpense> {
    payloads
        .iter()
        .enumerate()
        .map(|(i, p)| shared::RecurringExpense {
            id: Uuid::from_u128(i as u128),
            project_id: Uuid::nil(),
            author_id,
            payload: crate::crypto::encrypt_json(&crate::common::test_fixtures::TEST_KEY, p).unwrap(),
            version: i as i64,
            created_at,
            listed_participants: false,
        })
        .collect()
}

#[cfg(any(test, fuzzing))]
pub(crate) fn exercise_hostile(rows: &[shared::RecurringExpense], me: i32) {
    use crate::common::test_fixtures::TEST_KEY;
    use crate::recurring::model::decrypt_rules;
    use crate::recurring::schedule::{due, last_date, resume_index, total, upcoming};
    use crate::recurring::view::{monthly_total, my_monthly_share, next_date, progress, rate_drift, sorted, state};

    let rules = decrypt_rules(&TEST_KEY, rows);
    let today = NaiveDate::from_ymd_opt(2026, 11, 1).unwrap();
    let _ = (sorted(&rules), monthly_total(&rules), my_monthly_share(&rules, me));
    for r in &rules {
        let p = &r.payload;
        let _ = (state(p), next_date(p), progress(p), total(&p.rule), last_date(&p.rule));
        let _ = (resume_index(&p.rule, p.next, today), upcoming(&p.rule, p.next, 5), rate_drift(&p.template, 1.1));
        let _ = (p.template.conversion(), p.template.participant_ids());
        let due = due(&p.rule, p.next, today, 50);
        assert!(due.len() <= 50);
        if !due.is_empty() {
            let _ = materialize_request(&TEST_KEY, Uuid::nil(), r, &due, today, me, |d| d.to_string());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::test_fixtures::test_key;
    use crate::crypto::decrypt_json;
    use crate::recurring::model::{Ends, Freq, Rule, Side, Template};

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
                    extra: Default::default(),
                },
                rule: Rule { freq: Freq::Monthly, interval: 1, anchor: d(2026, 8, 1), ends: Ends::Never },
                next,
                paused: false,
                seed: Uuid::from_u128(99),
                author_id: None,
                extra: Default::default(),
            },
        }
    }

    #[test]
    fn a_materialization_writes_each_date_and_advances_the_cursor_past_the_last() {
        let key = test_key();
        let r = rule(0);
        let due = [(0, d(2026, 8, 1)), (1, d(2026, 9, 1))];
        let req = materialize_request(&key, Uuid::nil(), &r, &due, d(2026, 9, 20), 2, |date| format!("auto {date}"))
            .unwrap();
        assert_eq!(req.expected_version, 4);
        assert_eq!(req.due_through, d(2026, 9, 20), "the server date, not the rule's schedule");
        assert_eq!(req.occurrences.len(), 2);

        let advanced: RecurringPayload = decrypt_json(&key, &req.payload).unwrap();
        assert_eq!(advanced.next, 2);

        let first: ExpensePayload = decrypt_json(&key, &req.occurrences[0].payload).unwrap();
        assert_eq!(first.date, "2026-08-01");
        assert_eq!(first.recurring_id, Some(r.id));
        assert!(first.estimate, "a variable rule writes estimates");
        assert_eq!(first.author_id, Some(1), "the rule's author, not the device");
        assert_eq!(req.occurrences[0].author_id, None);
        assert!(req.occurrences[0].payers.is_empty() && req.occurrences[0].debtors.is_empty());
        assert_eq!(
            req.occurrences[0].client_op_id,
            Some(crate::recurring::schedule::occurrence_id(r.payload.seed, d(2026, 8, 1)))
        );
    }

    #[test]
    fn the_extra_cent_follows_the_occurrence_index() {
        let key = test_key();
        let due = [(1, d(2026, 9, 1))];
        let req = materialize_request(&key, Uuid::nil(), &rule(1), &due, d(2026, 12, 1), 2, |_| String::new()).unwrap();
        let occurrence: ExpensePayload = decrypt_json(&key, &req.occurrences[0].payload).unwrap();
        let amounts: Vec<f64> =
            occurrence.shares.unwrap().iter().filter(|s| s.is_debt).map(|s| s.amount).collect();
        assert_eq!(amounts, vec![483.33, 483.34, 483.33]);
    }

    #[test]
    fn a_removed_author_falls_back_to_the_member_materializing() {
        let mut r = rule(0);
        r.author_id = None;
        let key = test_key();
        let req = materialize_request(&key, Uuid::nil(), &r, &[(0, d(2026, 8, 1))], d(2026, 8, 1), 2, |_| String::new())
            .unwrap();
        let occurrence: ExpensePayload = decrypt_json(&key, &req.occurrences[0].payload).unwrap();
        assert_eq!(occurrence.author_id, Some(2));
    }

    #[test]
    fn dates_that_skip_the_cursor_are_refused() {
        let r = rule(3);
        assert!(materialize_request(&test_key(), Uuid::nil(), &r, &[(4, d(2026, 12, 1))], d(2026, 12, 1), 2, |_| String::new()).is_err());
        assert!(materialize_request(&test_key(), Uuid::nil(), &r, &[], d(2026, 12, 1), 2, |_| String::new()).is_err());
    }

    #[test]
    fn create_and_edit_keep_participants_and_author_in_the_payload() {
        let key = test_key();
        let r = rule(0);
        let c = create_request(&key, Uuid::nil(), 1, &r.payload, None).unwrap();
        assert!(c.participant_ids.is_empty());
        assert_eq!(c.author_id, None);
        let p: RecurringPayload = decrypt_json(&key, &c.payload).unwrap();
        assert_eq!(p.author_id, Some(1));
        assert_eq!(p.template.participant_ids(), vec![1, 2, 3]);

        let e = edit_request(&key, Uuid::nil(), &r, &r.payload, None).unwrap();
        assert_eq!(e.expected_version, 4);
        assert!(e.participant_ids.is_empty());
        let p: RecurringPayload = decrypt_json(&key, &e.payload).unwrap();
        assert_eq!(p.author_id, Some(1), "a rule written before keeps the author its column held");
    }

    #[test]
    fn a_rule_history_row_needs_an_actor() {
        let key = test_key();
        assert!(history(&key, None, "Rent".to_string(), HistoryAction::Created).is_none());
        let ctx = history(&key, Some(2), "Rent".to_string(), HistoryAction::Updated).unwrap();
        assert_eq!(ctx.actor_user_id, None);
        let p: HistoryPayload = decrypt_json(&key, &ctx.payload).unwrap();
        assert_eq!(p.summary, "Rent");
        assert_eq!(p.actor_user_id, Some(2));
        assert_eq!((p.action, p.entity), (Some(HistoryAction::Updated), Some(HistoryEntity::Project)));
    }
    mod hostile_payloads {
        use crate::common::hostile;
        use proptest::prelude::*;

        proptest! {
            #![proptest_config(ProptestConfig::with_cases(300))]

            #[test]
            fn any_rule_schedules_and_materialises(payloads in prop::collection::vec(hostile::rule_payload(), 1..6), author_id in proptest::option::of(hostile::id()), me in hostile::id(), created_at in hostile::timestamp()) {
                let rows = crate::recurring::requests::rule_rows(&payloads, author_id, created_at);
                crate::recurring::requests::exercise_hostile(&rows, me);
            }
        }
    }
}
