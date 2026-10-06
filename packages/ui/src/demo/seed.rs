use api::expenses::expenses_controller::batch_add_expenses;
use api::projects::projects_controller::add_project;
use api::users::users_controller::add_user;
use chrono::{Duration, Local, NaiveDate};
use dioxus::fullstack::Json;
use dioxus::prelude::*;
use shared::{
    to_cents, CreatableExpense, CreatableProject, CreatableUser, CreatableUserBatch,
    EncryptedUserAmount, ExpensePayload, ExpenseType, PaymentPayload, ProjectPayload, UserPayload,
};
use uuid::Uuid;

use super::dataset::Dataset;
use crate::common::{error_message, update_ls, upsert_project, upsert_project_key, LocalStorageState};
use crate::crypto::{claim_verifier, encrypt_json, generate_key, key_to_fragment};
use crate::tid;

/// The visitor's own copy, through the same three calls as a Tricount import, with them as its first
/// participant. Stored only once complete: a half-seeded project has no member and the sweep takes it.
pub async fn seed_demo(ls_ctx: Signal<LocalStorageState>, data: &'static Dataset) -> Result<Uuid, String> {
    let key = generate_key();
    let payload = encrypt_json(&key, &ProjectPayload {
        name: data.project.to_string(),
        currency: "EUR".to_string(),
        description: None,
        status: None,
    })?;
    let project = add_project(Json(CreatableProject {
        payload,
        claim_verifier: Some(claim_verifier(&key).to_vec()),
        demo: true,
    }))
    .await
    .map_err(|e| error_message(&e))?;

    let users = data
        .participants
        .iter()
        .map(|name| {
            Ok(CreatableUser {
                payload: encrypt_json(&key, &UserPayload { name: name.to_string(), removed: false })?,
                project_id: project.id,
            })
        })
        .collect::<Result<Vec<_>, String>>()?;
    let user_ids: Vec<i32> = add_user(Json(CreatableUserBatch::Multiple(users)))
        .await
        .map_err(|e| error_message(&e))?
        .iter()
        .map(|u| u.id)
        .collect();
    if user_ids.len() != data.participants.len() {
        return Err(tid!("demo-failed"));
    }

    let expenses = expense_creatables(&key, data, project.id, &user_ids, Local::now().date_naive())?;
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
    today: NaiveDate,
) -> Result<Vec<CreatableExpense>, String> {
    data.expenses
        .iter()
        .map(|e| {
            let payer = user_ids[e.payer];
            let payload = encrypt_json(key, &ExpensePayload {
                name: e.name.to_string(),
                amount: e.amount,
                expense_type: ExpenseType::Expense.as_str().to_string(),
                date: (today - Duration::days(e.days_ago)).format("%Y-%m-%d").to_string(),
                description: None,
                category: Some(e.category.to_string()),
                source_currency: None,
                source_amount: None,
                rate: None,
                recurring_id: None,
                estimate: false,
                author_id: None,
                shares: None,
            })?;
            let paid = encrypt_json(key, &PaymentPayload { amount: e.amount, is_debt: false })?;
            let debtors = user_ids
                .iter()
                .zip(equal_shares(e.amount, user_ids.len()))
                .map(|(id, amount)| {
                    Ok(EncryptedUserAmount {
                        user_id: *id,
                        payload: encrypt_json(key, &PaymentPayload { amount, is_debt: true })?,
                    })
                })
                .collect::<Result<Vec<_>, String>>()?;
            Ok(CreatableExpense {
                project_id,
                author_id: payer,
                payload,
                payers: vec![EncryptedUserAmount { user_id: payer, payload: paid }],
                debtors,
                history: None,
                client_op_id: None,
            })
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
        let today = NaiveDate::from_ymd_opt(2026, 10, 4).unwrap();
        for data in [&FR, &EN] {
            let ids = [10, 11, 12, 13, 14];
            let expenses = expense_creatables(&key, data, Uuid::nil(), &ids, today).unwrap();
            assert_eq!(expenses.len(), data.expenses.len());
            for (created, source) in expenses.iter().zip(data.expenses) {
                let payload: ExpensePayload = decrypt_json(&key, &created.payload).unwrap();
                assert_eq!(payload.date, (today - Duration::days(source.days_ago)).to_string());
                assert_eq!(created.author_id, ids[source.payer]);
                let owed: i64 = created
                    .debtors
                    .iter()
                    .map(|d| to_cents(decrypt_json::<PaymentPayload>(&key, &d.payload).unwrap().amount))
                    .sum();
                assert_eq!(owed, to_cents(source.amount), "{}", source.name);
            }
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
