use chrono::NaiveDateTime;
use shared::{
    Expense, ExpensePayload, ExpenseType, Payment, PaymentMethod, PaymentMethods, PaymentPayload,
    ProjectDto, ProjectPayload, ProjectStatus, User, UserPayload,
};
use uuid::Uuid;

use crate::crypto::decrypt_json;

#[derive(Debug, Clone, PartialEq)]
pub struct DecryptedProject {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub currency: String,
    pub status: ProjectStatus,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DecryptedUser {
    pub id: i32,
    pub name: String,
    pub created_at: Option<NaiveDateTime>,
    /// An account holds this participant as its identity in the project.
    pub claimed: bool,
    /// That account's display name. `None` when the claim carries no label, or when it does not
    /// decrypt — the padlock still shows, only the name is missing.
    pub claim_name: Option<String>,
    /// The payment methods that account chose to share with the project. Empty when it shares
    /// nothing, when there is no claim, or when the copy does not decrypt.
    pub payment_methods: Vec<PaymentMethod>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DecryptedExpense {
    pub id: i32,
    pub author_id: Option<i32>,
    pub project_id: Uuid,
    pub created_at: NaiveDateTime,
    pub name: String,
    pub description: Option<String>,
    /// Always in the project's currency — see `shared::ExpensePayload`. Every reader sums this and
    /// only this; the three fields below are for display and for prefilling the edit form.
    pub amount: f64,
    pub expense_type: ExpenseType,
    pub date: String,
    pub category: Option<String>,
    pub source_currency: Option<String>,
    pub source_amount: Option<f64>,
    pub rate: Option<f64>,
    pub recurring_id: Option<Uuid>,
    pub estimate: bool,
}

impl DecryptedExpense {
    /// The original amount, its currency and the rate it was booked at — `None` unless all three
    /// are present and the rate is usable, so a half-written payload renders as a plain
    /// project-currency expense instead of a broken conversion line.
    pub fn conversion(&self) -> Option<(f64, &str, f64)> {
        let currency = self.source_currency.as_deref()?;
        let amount = self.source_amount?;
        let rate = self.rate?;
        (amount.is_finite() && shared::is_valid_rate(rate)).then_some((amount, currency, rate))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DecryptedPayment {
    pub id: i32,
    pub expense_id: i32,
    pub user_id: i32,
    pub is_debt: bool,
    pub amount: f64,
    pub created_at: NaiveDateTime,
}

pub fn decrypt_project(key: &[u8; 32], p: &ProjectDto) -> Result<DecryptedProject, String> {
    let pp: ProjectPayload = decrypt_json(key, &p.payload)?;
    Ok(DecryptedProject {
        id: p.id,
        name: pp.name,
        description: pp.description,
        currency: pp.currency,
        status: p.status.clone(),
        created_at: p.created_at,
    })
}

pub fn decrypt_user(key: &[u8; 32], u: &User) -> Result<DecryptedUser, String> {
    let up: UserPayload = decrypt_json(key, &u.payload)?;
    Ok(DecryptedUser {
        id: u.id,
        name: up.name,
        created_at: u.created_at,
        claimed: u.claimed,
        claim_name: u.claim_label.as_ref().and_then(|p| decrypt_json::<String>(key, p).ok()),
        payment_methods: u
            .payment_methods
            .as_ref()
            .and_then(|p| decrypt_json::<PaymentMethods>(key, p).ok())
            .map(|p| p.methods)
            .unwrap_or_default(),
    })
}

/// Empty when the payload cannot be decrypted: call sites render it directly, so a failure
/// degrades to a blank name rather than an error.
pub fn user_name(key: &[u8; 32], u: &User) -> String {
    decrypt_json::<UserPayload>(key, &u.payload).map(|p| p.name).unwrap_or_default()
}

/// Same, for the call sites that only hold an optional key (the key context before it resolves).
pub fn user_name_opt(key: Option<&[u8; 32]>, u: &User) -> String {
    key.map(|k| user_name(k, u)).unwrap_or_default()
}

/// Positionally matches `users`. Without a key every name is blank, but the length is kept so
/// callers can still zip against `users`.
pub fn user_names(key: Option<&[u8; 32]>, users: &[User]) -> Vec<String> {
    users.iter().map(|u| user_name_opt(key, u)).collect()
}

pub fn project_name(key: &[u8; 32], p: &ProjectDto) -> String {
    decrypt_json::<ProjectPayload>(key, &p.payload).map(|pp| pp.name).unwrap_or_default()
}

pub fn project_currency(key: &[u8; 32], p: &ProjectDto) -> String {
    decrypt_json::<ProjectPayload>(key, &p.payload).map(|pp| pp.currency).unwrap_or_default()
}

pub fn decrypt_expense(key: &[u8; 32], e: &Expense) -> Result<DecryptedExpense, String> {
    let ep: ExpensePayload = decrypt_json(key, &e.payload)?;
    let expense_type = match ep.expense_type.as_str() {
        "expense" => ExpenseType::Expense,
        "transfer" => ExpenseType::Transfer,
        "gain" => ExpenseType::Gain,
        other => return Err(format!("invalid expense type: {other}")),
    };
    Ok(DecryptedExpense {
        id: e.id,
        author_id: e.author_id,
        project_id: e.project_id,
        created_at: e.created_at,
        name: ep.name,
        description: ep.description,
        amount: ep.amount,
        expense_type,
        date: ep.date,
        category: ep.category,
        source_currency: ep.source_currency,
        source_amount: ep.source_amount,
        rate: ep.rate,
        recurring_id: ep.recurring_id,
        estimate: ep.estimate,
    })
}

pub fn decrypt_payment(key: &[u8; 32], p: &Payment) -> Result<DecryptedPayment, String> {
    let pp: PaymentPayload = decrypt_json(key, &p.payload)?;
    Ok(DecryptedPayment {
        id: p.id,
        expense_id: p.expense_id,
        user_id: p.user_id,
        is_debt: pp.is_debt,
        amount: pp.amount,
        created_at: p.created_at,
    })
}

/// True when an expense's payments no longer add up on either side. The server cannot check this
/// on ciphertext, so honest clients detect it on read — see DOCUMENTATION.md §8, "Why the server
/// does not validate that an expense balances".
///
/// Empty `payments` means the key could not open them — a missing key, not an inconsistency.
pub fn payments_are_inconsistent(total: f64, payments: &[DecryptedPayment]) -> bool {
    if payments.is_empty() {
        return false;
    }
    let side = |is_debt: bool| {
        payments.iter().filter(move |p| p.is_debt == is_debt).map(|p| p.amount)
    };
    !shared::sums_to_total(total, side(false)) || !shared::sums_to_total(total, side(true))
}

#[cfg(test)]
mod tests {
    use super::*;
    use base64::{engine::general_purpose::STANDARD, Engine as _};

    use crate::common::test_fixtures::{make_expense, make_payment, make_user, test_key};
    use crate::crypto::{encrypt_json, generate_key};

    fn dp(expense_id: i32, is_debt: bool, amount: f64) -> DecryptedPayment {
        DecryptedPayment {
            id: 0,
            expense_id,
            user_id: 0,
            is_debt,
            amount,
            created_at: NaiveDateTime::default(),
        }
    }

    /// `decrypt_user` is what the padlock reads: the flag comes from the server, the name from the
    /// project key, and a label that does not decrypt still leaves the lock visible.
    #[test]
    fn decrypt_user_carries_the_claim_through() {
        let key = test_key();
        let mut u = make_user(&key, 7, "Alice");
        u.claimed = true;
        u.claim_label = Some(encrypt_json(&key, &"Jonathan".to_string()).unwrap());

        let du = decrypt_user(&key, &u).unwrap();

        assert!(du.claimed);
        assert_eq!(du.claim_name.as_deref(), Some("Jonathan"));

        u.claim_label = Some(encrypt_json(&generate_key(), &"Jonathan".to_string()).unwrap());
        let du = decrypt_user(&key, &u).unwrap();
        assert!(du.claimed, "the lock survives a label it cannot read");
        assert_eq!(du.claim_name, None);
    }

    #[test]
    fn a_balanced_expense_is_consistent() {
        let pmts = [dp(1, false, 100.0), dp(1, true, 33.33), dp(1, true, 33.33), dp(1, true, 33.34)];
        assert!(!payments_are_inconsistent(100.0, &pmts));
    }

    #[test]
    fn the_reported_scenario_is_flagged() {
        // 100 expense, 60 + 60 paid, 70 + 70 owed.
        let pmts = [
            dp(1, false, 60.0),
            dp(1, false, 60.0),
            dp(1, true, 70.0),
            dp(1, true, 70.0),
        ];
        assert!(payments_are_inconsistent(100.0, &pmts));
    }

    #[test]
    fn either_side_alone_is_enough_to_flag() {
        assert!(payments_are_inconsistent(100.0, &[dp(1, false, 120.0), dp(1, true, 100.0)]));
        assert!(payments_are_inconsistent(100.0, &[dp(1, false, 100.0), dp(1, true, 90.0)]));
    }

    #[test]
    fn a_one_cent_gap_is_flagged() {
        assert!(payments_are_inconsistent(100.0, &[dp(1, false, 100.0), dp(1, true, 99.99)]));
    }

    #[test]
    fn an_undecryptable_expense_is_not_an_inconsistency() {
        // No payment decrypted — the key is wrong, the data is not necessarily broken.
        assert!(!payments_are_inconsistent(100.0, &[]));
    }

    #[test]
    fn a_nan_amount_is_flagged_without_panicking() {
        let pmts = [dp(1, false, f64::NAN), dp(1, true, 100.0)];
        assert!(payments_are_inconsistent(100.0, &pmts));
    }

    fn make_project(
        key: &[u8; 32],
        name: &str,
        description: Option<&str>,
        currency: &str,
    ) -> ProjectDto {
        ProjectDto {
            id: Uuid::nil(),
            payload: encrypt_json(key, &ProjectPayload {
                name: name.to_string(),
                currency: currency.to_string(),
                description: description.map(|s| s.to_string()),
            })
            .unwrap(),
            status: shared::ProjectStatus::Ongoing,
            created_at: chrono::NaiveDateTime::default(),
            owner_account_id: None,
        }
    }

    #[test]
    fn decrypt_user_roundtrip() {
        let key = test_key();
        let user = make_user(&key, 7, "Alice");
        let du = decrypt_user(&key, &user).unwrap();
        assert_eq!(du.id, 7);
        assert_eq!(du.name, "Alice");
    }

    #[test]
    fn decrypt_user_wrong_key_fails() {
        let key = test_key();
        let user = make_user(&key, 1, "Bob");
        let mut bad = key;
        bad[0] ^= 0xFF;
        assert!(decrypt_user(&bad, &user).is_err());
    }

    #[test]
    fn decrypt_user_unicode_name() {
        let key = test_key();
        let user = make_user(&key, 2, "日本語 🍜");
        assert_eq!(decrypt_user(&key, &user).unwrap().name, "日本語 🍜");
    }

    #[test]
    fn decrypt_project_roundtrip() {
        let key = test_key();
        let p = make_project(&key, "Vacances", None, "EUR");
        let dp = decrypt_project(&key, &p).unwrap();
        assert_eq!(dp.name, "Vacances");
        assert_eq!(dp.currency, "EUR");
        assert!(dp.description.is_none());
    }

    #[test]
    fn decrypt_project_with_description() {
        let key = test_key();
        let p = make_project(&key, "Road trip", Some("Paris → Lyon"), "EUR");
        let dp = decrypt_project(&key, &p).unwrap();
        assert_eq!(dp.name, "Road trip");
        assert_eq!(dp.description.unwrap(), "Paris → Lyon");
    }

    #[test]
    fn decrypt_project_wrong_key_fails() {
        let key = test_key();
        let p = make_project(&key, "Secret", None, "EUR");
        let mut bad = key;
        bad[3] ^= 0xAB;
        assert!(decrypt_project(&bad, &p).is_err());
    }

    #[test]
    fn decrypt_expense_roundtrip() {
        let key = test_key();
        let e = make_expense(&key, 42, "Dîner", 45.50, ExpenseType::Expense, "2025-06-15");
        let de = decrypt_expense(&key, &e).unwrap();
        assert_eq!(de.id, 42);
        assert_eq!(de.name, "Dîner");
        assert!((de.amount - 45.50).abs() < 1e-9);
        assert_eq!(de.expense_type, ExpenseType::Expense);
        assert_eq!(de.date, "2025-06-15");
        assert!(de.description.is_none());
    }

    #[test]
    fn decrypt_expense_all_types() {
        let key = test_key();
        for t in [ExpenseType::Expense, ExpenseType::Transfer, ExpenseType::Gain] {
            let e = make_expense(&key, 1, "x", 10.0, t.clone(), "2025-01-01");
            assert_eq!(decrypt_expense(&key, &e).unwrap().expense_type, t);
        }
    }

    #[test]
    fn decrypt_expense_with_description() {
        let key = test_key();
        let e = Expense {
            id: 1,
            author_id: Some(1),
            project_id: Uuid::nil(),
            created_at: chrono::NaiveDateTime::default(),
            payload: encrypt_json(&key, &ExpensePayload {
                name: "Lunch".to_string(),
                amount: 20.0,
                expense_type: "expense".to_string(),
                date: "2025-01-01".to_string(),
                description: Some("Mediterranean restaurant".to_string()),
                category: None,
                source_currency: None,
                source_amount: None,
                rate: None,
                recurring_id: None,
                estimate: false,
            })
            .unwrap(),
        };
        let de = decrypt_expense(&key, &e).unwrap();
        assert_eq!(de.description.unwrap(), "Mediterranean restaurant");
    }

    #[test]
    fn decrypt_expense_wrong_key_fails() {
        let key = test_key();
        let e = make_expense(&key, 1, "secret", 100.0, ExpenseType::Expense, "2025-01-01");
        let mut bad = key;
        bad[0] ^= 0xFF;
        assert!(decrypt_expense(&bad, &e).is_err());
    }

    #[test]
    fn decrypt_expense_tampered_payload_fails() {
        let key = test_key();
        let mut e = make_expense(&key, 1, "test", 50.0, ExpenseType::Expense, "2025-01-01");
        let mut ct = STANDARD.decode(&e.payload.ct).unwrap();
        ct[0] ^= 0x01;
        e.payload.ct = STANDARD.encode(&ct);
        assert!(decrypt_expense(&key, &e).is_err());
    }

    #[test]
    fn decrypt_payment_payer_roundtrip() {
        let key = test_key();
        let p = make_payment(&key, 1, 10, 5, false, 30.0);
        let dp = decrypt_payment(&key, &p).unwrap();
        assert_eq!(dp.user_id, 5);
        assert!(!dp.is_debt);
        assert!((dp.amount - 30.0).abs() < 1e-9);
    }

    #[test]
    fn decrypt_payment_debtor_roundtrip() {
        let key = test_key();
        let p = make_payment(&key, 2, 10, 7, true, 15.0);
        let dp = decrypt_payment(&key, &p).unwrap();
        assert_eq!(dp.user_id, 7);
        assert!(dp.is_debt);
        assert!((dp.amount - 15.0).abs() < 1e-9);
    }

    #[test]
    fn decrypt_payment_wrong_key_fails() {
        let key = test_key();
        let p = make_payment(&key, 1, 1, 1, false, 10.0);
        let mut bad = key;
        bad[1] ^= 0x55;
        assert!(decrypt_payment(&bad, &p).is_err());
    }

    #[test]
    fn decrypt_payment_tampered_payload_fails() {
        let key = test_key();
        let mut p = make_payment(&key, 1, 1, 1, true, 10.0);
        let mut ct = STANDARD.decode(&p.payload.ct).unwrap();
        ct[0] ^= 0x01;
        p.payload.ct = STANDARD.encode(&ct);
        assert!(decrypt_payment(&key, &p).is_err());
    }
}
