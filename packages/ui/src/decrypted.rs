use chrono::NaiveDateTime;
use shared::{
    EncryptedPair, Expense, ExpensePayload, ExpenseType, Payment, PaymentMethod, PaymentMethods, PaymentPayload,
    ProjectDto, ProjectPayload, ProjectStatus, User, UserPayload,
};
use uuid::Uuid;

use crate::crypto::{decrypt_json, encrypt_json};

#[derive(Debug, Clone, PartialEq)]
pub struct DecryptedProject {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub currency: String,
    pub status: ProjectStatus,
    pub created_at: NaiveDateTime,
    pub read_only: bool,
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
    /// Out of the pickers; still named on past expenses and still in balances.
    pub removed: bool,
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
        status: pp.status.unwrap_or_else(|| p.status.clone()),
        created_at: p.created_at,
        read_only: p.read_only,
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
        removed: up.removed,
    })
}

/// The participants a picker offers: everyone not removed, plus `keep` — the ones an existing
/// expense or rule already names, which editing must not silently drop. Without a key nothing
/// can be read, so everyone is offered.
pub fn pickable_users(key: Option<&[u8; 32]>, users: &[User], keep: &[i32]) -> Vec<User> {
    users
        .iter()
        .filter(|u| {
            keep.contains(&u.id)
                || !key.and_then(|k| decrypt_json::<UserPayload>(k, &u.payload).ok()).is_some_and(|p| p.removed)
        })
        .cloned()
        .collect()
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

/// The payload's status when it carries one, else the `projects.status` column's.
pub fn project_status(key: Option<&[u8; 32]>, p: &ProjectDto) -> ProjectStatus {
    key.and_then(|k| decrypt_json::<ProjectPayload>(k, &p.payload).ok())
        .and_then(|pp| pp.status)
        .unwrap_or_else(|| p.status.clone())
}

/// The payload re-encrypted with `status`, for a project whose payload already carries one — the
/// payload wins over the column, so a column-only write would not take. `None` for a payload that
/// does not carry one: writing it there while a client that only knows the column can still change
/// the status would freeze the status those clients see.
pub fn payload_with_status(key: &[u8; 32], p: &ProjectDto, status: &ProjectStatus) -> Option<EncryptedPair> {
    let mut pp = decrypt_json::<ProjectPayload>(key, &p.payload).ok().filter(|pp| pp.status.is_some())?;
    pp.status = Some(status.clone());
    encrypt_json(key, &pp).ok()
}

pub fn decrypt_expense(key: &[u8; 32], e: &Expense) -> Result<DecryptedExpense, String> {
    decrypt_expense_and_shares(key, e).map(|(d, _)| d)
}

/// The expense and, when its payload carries them, its shares as payments. `None` means they are
/// still `payments` rows — the format before docs/plans/participant-links-encryption.md.
fn decrypt_expense_and_shares(
    key: &[u8; 32],
    e: &Expense,
) -> Result<(DecryptedExpense, Option<Vec<DecryptedPayment>>), String> {
    let ep: ExpensePayload = decrypt_json(key, &e.payload)?;
    let expense_type = match ep.expense_type.as_str() {
        "expense" => ExpenseType::Expense,
        "transfer" => ExpenseType::Transfer,
        "gain" => ExpenseType::Gain,
        other => return Err(format!("invalid expense type: {other}")),
    };
    let shares = ep.shares.map(|shares| {
        shares
            .into_iter()
            .map(|s| DecryptedPayment {
                id: 0,
                expense_id: e.id,
                user_id: s.user_id,
                is_debt: s.is_debt,
                amount: s.amount,
                created_at: e.created_at,
            })
            .collect()
    });
    let expense = DecryptedExpense {
        id: e.id,
        author_id: ep.author_id.or(e.author_id),
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
    };
    Ok((expense, shares))
}

/// Every expense that opens, and its payments: from the payload's shares when it has them, from
/// its `payments` rows otherwise. Rows of an expense that does not open are dropped with it, and
/// rows of an expense that carries shares are ignored.
pub fn decrypt_ledger(
    key: &[u8; 32],
    expenses: &[Expense],
    payments: &[Payment],
) -> (Vec<DecryptedExpense>, Vec<DecryptedPayment>) {
    let mut decrypted = Vec::with_capacity(expenses.len());
    let mut ledger = Vec::with_capacity(payments.len());
    let mut from_rows = std::collections::HashSet::new();
    for e in expenses {
        let Ok((expense, shares)) = decrypt_expense_and_shares(key, e) else { continue };
        match shares {
            Some(shares) => ledger.extend(shares),
            None => {
                from_rows.insert(e.id);
            }
        }
        decrypted.push(expense);
    }
    ledger.extend(
        payments
            .iter()
            .filter(|p| from_rows.contains(&p.expense_id))
            .filter_map(|p| decrypt_payment(key, p).ok()),
    );
    (decrypted, ledger)
}

/// The payments of one expense, either format.
pub fn decrypt_expense_payments(key: &[u8; 32], e: &Expense, payments: &[Payment]) -> Vec<DecryptedPayment> {
    decrypt_ledger(key, std::slice::from_ref(e), payments).1
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
                status: None,
            })
            .unwrap(),
            status: shared::ProjectStatus::Ongoing,
            created_at: chrono::NaiveDateTime::default(),
            owner_account_id: None,
            read_only: false,
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
                author_id: None,
                shares: None,
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

    /// The format of docs/plans/participant-links-encryption.md, padded the way it will be written.
    fn v2_expense(key: &[u8; 32], id: i32, author_id: i32, shares: Vec<shared::ExpenseShare>) -> Expense {
        let mut e = make_expense(key, id, "Pizza", 30.0, ExpenseType::Expense, "2026-10-01");
        let mut ep: ExpensePayload = decrypt_json(key, &e.payload).unwrap();
        ep.author_id = Some(author_id);
        ep.shares = Some(shares);
        e.payload = crate::crypto::encrypt(key, &shared::pad_json(serde_json::to_string(&ep).unwrap())).unwrap();
        e.author_id = None;
        e
    }

    fn share(user_id: i32, is_debt: bool, amount: f64) -> shared::ExpenseShare {
        shared::ExpenseShare { user_id, amount, is_debt }
    }

    #[test]
    fn a_ledger_reads_shares_from_the_payload_and_rows_from_the_table() {
        let key = test_key();
        let v1 = make_expense(&key, 1, "Taxi", 20.0, ExpenseType::Expense, "2026-10-01");
        let v2 = v2_expense(&key, 2, 3, vec![share(3, false, 30.0), share(4, true, 30.0)]);
        let rows = vec![
            make_payment(&key, 10, 1, 1, false, 20.0),
            make_payment(&key, 11, 1, 2, true, 20.0),
            make_payment(&key, 12, 2, 9, true, 99.0),
        ];

        let (expenses, payments) = decrypt_ledger(&key, &[v1, v2], &rows);

        assert_eq!(expenses.len(), 2);
        let of = |id: i32| -> Vec<(i32, bool, f64)> {
            payments.iter().filter(|p| p.expense_id == id).map(|p| (p.user_id, p.is_debt, p.amount)).collect()
        };
        assert_eq!(of(1), vec![(1, false, 20.0), (2, true, 20.0)]);
        assert_eq!(of(2), vec![(3, false, 30.0), (4, true, 30.0)], "a row next to shares is ignored");
    }

    #[test]
    fn a_ledger_drops_the_rows_of_an_expense_that_does_not_open() {
        let key = test_key();
        let e = make_expense(&generate_key(), 1, "Taxi", 20.0, ExpenseType::Expense, "2026-10-01");
        let (expenses, payments) = decrypt_ledger(&key, &[e], &[make_payment(&key, 10, 1, 1, false, 20.0)]);
        assert!(expenses.is_empty());
        assert!(payments.is_empty());
    }

    #[test]
    fn the_payload_author_wins_over_the_column() {
        let key = test_key();
        let v2 = v2_expense(&key, 2, 3, vec![]);
        assert_eq!(decrypt_expense(&key, &v2).unwrap().author_id, Some(3));
        let v1 = make_expense(&key, 1, "Taxi", 20.0, ExpenseType::Expense, "2026-10-01");
        assert_eq!(decrypt_expense(&key, &v1).unwrap().author_id, Some(1));
    }

    #[test]
    fn a_padded_payload_still_parses() {
        let key = test_key();
        let v2 = v2_expense(&key, 2, 3, vec![share(3, false, 30.0)]);
        assert_eq!(crate::crypto::decrypt(&key, &v2.payload).unwrap().len(), 256);
        assert_eq!(decrypt_expense_payments(&key, &v2, &[]).len(), 1);
    }

    fn project_with(key: &[u8; 32], status: Option<ProjectStatus>, column: ProjectStatus) -> ProjectDto {
        let mut p = crate::common::test_fixtures::make_project(key, Uuid::nil(), "Trip", "EUR");
        let mut pp: ProjectPayload = decrypt_json(key, &p.payload).unwrap();
        pp.status = status;
        p.payload = encrypt_json(key, &pp).unwrap();
        p.status = column;
        p
    }

    #[test]
    fn the_payload_status_wins_over_the_column() {
        let key = test_key();
        let converted = project_with(&key, Some(ProjectStatus::Closed), ProjectStatus::Ongoing);
        assert_eq!(project_status(Some(&key), &converted), ProjectStatus::Closed);
        assert_eq!(decrypt_project(&key, &converted).unwrap().status, ProjectStatus::Closed);
        assert_eq!(project_status(None, &converted), ProjectStatus::Ongoing, "without a key, the column");

        let legacy = project_with(&key, None, ProjectStatus::Archived);
        assert_eq!(project_status(Some(&key), &legacy), ProjectStatus::Archived);
    }

    /// A status change on a project whose payload carries the status must land in the payload, or
    /// the payload would keep winning; one whose payload does not must stay column-only.
    #[test]
    fn a_status_change_follows_where_the_status_lives() {
        let key = test_key();
        let converted = project_with(&key, Some(ProjectStatus::Ongoing), ProjectStatus::Ongoing);
        let payload = payload_with_status(&key, &converted, &ProjectStatus::Archived).unwrap();
        let pp: ProjectPayload = decrypt_json(&key, &payload).unwrap();
        assert_eq!(pp.status, Some(ProjectStatus::Archived));
        assert_eq!(pp.name, "Trip");

        let legacy = project_with(&key, None, ProjectStatus::Ongoing);
        assert!(payload_with_status(&key, &legacy, &ProjectStatus::Archived).is_none());
    }

    #[test]
    fn a_removed_participant_leaves_the_pickers_unless_kept() {
        let key = test_key();
        let mut removed = make_user(&key, 2, "Bob");
        removed.payload = encrypt_json(&key, &UserPayload { name: "Bob".into(), removed: true }).unwrap();
        let users = vec![make_user(&key, 1, "Alice"), removed];

        let ids = |us: Vec<User>| us.iter().map(|u| u.id).collect::<Vec<_>>();
        assert_eq!(ids(pickable_users(Some(&key), &users, &[])), vec![1]);
        assert_eq!(ids(pickable_users(Some(&key), &users, &[2])), vec![1, 2], "an expense naming Bob keeps him");
        assert_eq!(ids(pickable_users(None, &users, &[])), vec![1, 2], "no key, nothing to read");
        assert!(decrypt_user(&key, &users[1]).unwrap().removed);
    }
}
