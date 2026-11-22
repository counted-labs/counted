use api::expenses::expenses_controller::batch_add_expenses;
use shared::time::Date;
use dioxus::fullstack::Json;
use dioxus::prelude::*;
use shared::{to_cents, CreatableExpense, CreatableProject, ExpensePayload, ExpenseType, ProjectPayload};
use uuid::Uuid;

use super::dataset::Dataset;
use crate::entity_ids::{add_participants, new_expense, new_participant};
use crate::common::{add_project_fresh, error_message, update_ls, upsert_project, upsert_project_key, LocalStorageState};
use crate::crypto::{claim_verifier, encrypt_padded_json, generate_key, key_to_fragment};
use crate::expenses::helpers::expense_modal_helpers::with_shares;
use crate::tid;

/// The visitor's own copy, through the same three calls as a Tricount import, with them as its first
/// participant. Stored only once complete: a half-seeded project has no member and the sweep takes it.
pub async fn seed_demo(ls_ctx: Signal<LocalStorageState>, data: &'static Dataset) -> Result<Uuid, String> {
    let key = generate_key();
    // A demo keeps no pending record (`add_project_fresh`): it is no one's, and the sweep takes an
    // unfinished one a day later.
    let seal = |id, key: &[u8; 32]| {
        Ok(CreatableProject {
            id: Some(id),
            payload: encrypt_padded_json(key, &ProjectPayload {
                name: data.project.to_string(),
                currency: "EUR".to_string(),
                description: None,
                status: Some(shared::ProjectStatus::Ongoing),
                extra: Default::default(),
            })?,
            claim_verifier: Some(claim_verifier(key).to_vec()),
            demo: true,
            creator_verifier: None,
        })
    };
    let project = add_project_fresh(ls_ctx, None, &key, None, seal).await.map_err(|e| e.message())?.project;

    let users = data
        .participants
        .iter()
        .map(|name| new_participant(&key, project.id, name.to_string()))
        .collect::<Result<Vec<_>, String>>()?;
    let user_ids: Vec<i32> = add_participants(&key, users)
        .await
        .map_err(|e| error_message(&e))?
        .iter()
        .map(|u| u.id)
        .collect();
    if user_ids.len() != data.participants.len() {
        return Err(tid!("demo-failed"));
    }

    let expenses = expense_creatables(&key, data, project.id, &user_ids, crate::common::local_today())?;
    crate::common::project_token::scope_key(Some(&key));
    batch_add_expenses(Json(expenses)).await.map_err(|e| error_message(&e))?;

    update_ls(ls_ctx, |state| {
        upsert_project(state, project.id, Some(user_ids[0]));
        upsert_project_key(state, project.id, key_to_fragment(&key));
        state.onboarding_seen = true;
    });
    Ok(project.id)
}

/// Every expense split equally between everyone, the form's default.
fn expense_creatables(
    key: &[u8; 32],
    data: &Dataset,
    project_id: Uuid,
    user_ids: &[i32],
    today: Date,
) -> Result<Vec<CreatableExpense>, String> {
    data.expenses
        .iter()
        .map(|e| {
            let payer = user_ids[e.payer];
            let payload = ExpensePayload {
                name: e.name.to_string(),
                amount: e.amount,
                expense_type: ExpenseType::Expense.as_str().to_string(),
                date: today.checked_sub_days(e.days_ago).unwrap_or(today).ymd(),
                description: None,
                category: Some(e.category.to_string()),
                source_currency: None,
                source_amount: None,
                rate: None,
                recurring_id: None,
                estimate: false,
                author_id: None,
                shares: None,
                extra: Default::default(),
            };
            let debtors: Vec<(i32, f64)> =
                user_ids.iter().copied().zip(equal_shares(e.amount, user_ids.len())).collect();
            let payload = with_shares(payload, Some(payer), &[(payer, e.amount)], &debtors);
            new_expense(key, project_id, Uuid::new_v4(), payload, None)
        })
        .collect()
}

/// Whole cents, the leftover ones on the first shares, so they always add back up to `total`.
fn equal_shares(total: f64, n: usize) -> Vec<f64> {
    let cents = to_cents(total);
    let (base, rest) = (cents / n as i64, (cents % n as i64) as usize);
    (0..n).map(|i| (base + (i < rest) as i64) as f64 / 100.0).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypto::decrypt_json;
    use crate::demo::dataset::{EN, FR};

    #[test]
    fn equal_shares_add_up_to_the_cent() {
        assert_eq!(equal_shares(18.60, 5), vec![3.72; 5]);
        assert_eq!(equal_shares(27.30, 4), vec![6.83, 6.83, 6.82, 6.82]);
        let shares = equal_shares(134.51, 5);
        assert_eq!(shares.iter().map(|s| to_cents(*s)).sum::<i64>(), 13451);
    }

    #[test]
    fn every_seeded_expense_balances_and_is_dated_back_from_today() {
        let key = generate_key();
        let today = Date::from_ymd_opt(2026, 10, 4).unwrap();
        for data in [&FR, &EN] {
            let ids = [10, 11, 12, 13, 14];
            let expenses = expense_creatables(&key, data, Uuid::nil(), &ids, today).unwrap();
            assert_eq!(expenses.len(), data.expenses.len());
            for (created, source) in expenses.iter().zip(data.expenses) {
                let payload: ExpensePayload = decrypt_json(&key, &created.payload).unwrap();
                assert_eq!(payload.date, today.checked_sub_days(source.days_ago).unwrap().to_string());
                assert_eq!(payload.author_id, Some(ids[source.payer]));
                assert!(created.payers.is_empty() && created.debtors.is_empty());
                let owed: i64 = payload
                    .shares
                    .unwrap()
                    .iter()
                    .filter(|s| s.is_debt)
                    .map(|s| to_cents(s.amount))
                    .sum();
                assert_eq!(owed, to_cents(source.amount), "{}", source.name);
            }
        }
    }

    #[test]
    fn every_seeded_expense_seals_the_id_it_sends_in_clear() {
        use shared::entity::{read, Slot};
        let key = generate_key();
        let expenses = expense_creatables(&key, &FR, Uuid::nil(), &[10, 11, 12, 13, 14], Date::from_ymd_opt(2026, 10, 4).unwrap()).unwrap();
        let mut ids = std::collections::HashSet::new();
        for created in &expenses {
            let eid = created.client_op_id.expect("an id in clear");
            assert_eq!(read(&crate::crypto::decrypt(&key, &created.payload).unwrap(), Slot::Expense), Ok(Some(eid)));
            assert!(ids.insert(eid), "one id per row");
        }
    }

    #[test]
    fn both_languages_share_one_shape() {
        assert_eq!(FR.expenses.len(), EN.expenses.len());
        for (fr, en) in FR.expenses.iter().zip(EN.expenses) {
            assert_eq!((fr.amount, fr.category, fr.payer, fr.days_ago), (en.amount, en.category, en.payer, en.days_ago));
        }
    }
}
