//! Encrypted fixtures shared by the test modules. Every builder takes the key explicitly so a
//! test can encrypt with one key and assert that a different one fails.

use chrono::NaiveDateTime;
use shared::{
    Expense, ExpensePayload, ExpenseType, Payment, PaymentPayload, ProjectDto, ProjectPayload,
    ProjectStatus, User, UserPayload,
};
use uuid::Uuid;

use crate::crypto::encrypt_json;

pub(crate) const TEST_KEY: [u8; 32] = [0x42u8; 32];

pub(crate) fn test_key() -> [u8; 32] {
    TEST_KEY
}

pub(crate) fn make_user(key: &[u8; 32], id: i32, name: &str) -> User {
    User {
        id,
        payload: encrypt_json(key, &UserPayload { name: name.to_string() }).unwrap(),
        ..Default::default()
    }
}

pub(crate) fn make_project(key: &[u8; 32], id: Uuid, name: &str, currency: &str) -> ProjectDto {
    ProjectDto {
        id,
        payload: encrypt_json(
            key,
            &ProjectPayload {
                name: name.to_string(),
                currency: currency.to_string(),
                description: None,
            },
        )
        .unwrap(),
        status: ProjectStatus::Ongoing,
        created_at: NaiveDateTime::default(),
        owner_account_id: None,
    }
}

pub(crate) fn make_expense(
    key: &[u8; 32],
    id: i32,
    name: &str,
    amount: f64,
    expense_type: ExpenseType,
    date: &str,
) -> Expense {
    Expense {
        id,
        author_id: Some(1),
        project_id: Uuid::nil(),
        created_at: NaiveDateTime::default(),
        payload: encrypt_json(
            key,
            &ExpensePayload {
                name: name.to_string(),
                amount,
                expense_type: expense_type.as_str().to_string(),
                date: date.to_string(),
                description: None,
                category: None,
                source_currency: None,
                source_amount: None,
                rate: None,
            },
        )
        .unwrap(),
    }
}

/// Same row, with the category the user picked by hand — what `RowExpense` prefers over the name.
pub(crate) fn make_expense_with_category(
    key: &[u8; 32],
    id: i32,
    name: &str,
    category: &str,
) -> Expense {
    Expense {
        id,
        author_id: Some(1),
        project_id: Uuid::nil(),
        created_at: NaiveDateTime::default(),
        payload: encrypt_json(
            key,
            &ExpensePayload {
                name: name.to_string(),
                amount: 10.0,
                expense_type: ExpenseType::Expense.as_str().to_string(),
                date: "2025-01-01".to_string(),
                description: None,
                category: Some(category.to_string()),
                source_currency: None,
                source_amount: None,
                rate: None,
            },
        )
        .unwrap(),
    }
}

pub(crate) fn make_payment(
    key: &[u8; 32],
    id: i32,
    expense_id: i32,
    user_id: i32,
    is_debt: bool,
    amount: f64,
) -> Payment {
    Payment {
        id,
        expense_id,
        user_id,
        payload: encrypt_json(key, &PaymentPayload { amount, is_debt }).unwrap(),
        created_at: NaiveDateTime::default(),
    }
}

/// Runs `f` with `COUNTED_DATA_DIR` pointed at a fresh temp dir named after `suffix`, then
/// clears the var and removes the dir. Takes [`super::env_lock`], so tests using it are
/// serialised against every other test that touches those env vars.
///
/// The value is returned so assertions can run *after* cleanup — a failing assertion then
/// cannot leave the temp dir or the env var behind for the next test.
#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn with_data_dir<T>(suffix: &str, f: impl FnOnce(&std::path::Path) -> T) -> T {
    let _guard = super::env_lock();
    let dir = std::env::temp_dir().join(format!("counted_test_{suffix}"));
    std::fs::remove_dir_all(&dir).ok();
    std::fs::create_dir_all(&dir).unwrap();
    std::env::set_var("COUNTED_DATA_DIR", dir.to_str().unwrap());

    let out = f(&dir);

    std::env::remove_var("COUNTED_DATA_DIR");
    std::fs::remove_dir_all(&dir).ok();
    out
}

/// 10 users, 1 000 expenses, 5 000 payments (one payer + four debtors each).
pub(crate) struct LargeProject {
    pub users: Vec<User>,
    pub expenses: Vec<Expense>,
    pub payments: Vec<Payment>,
}

/// Built once per test binary: these encrypt for real, which costs seconds in a debug build.
pub(crate) fn large_project() -> &'static LargeProject {
    static CACHE: std::sync::OnceLock<LargeProject> = std::sync::OnceLock::new();
    CACHE.get_or_init(|| {
        let key = test_key();
        const USERS: i32 = 10;
        const EXPENSES: i32 = 1_000;

        let users = (1..=USERS).map(|id| make_user(&key, id, &format!("user {id}"))).collect();

        let mut expenses = Vec::with_capacity(EXPENSES as usize);
        let mut payments = Vec::with_capacity(EXPENSES as usize * 5);
        let mut payment_id = 0;
        for e in 1..=EXPENSES {
            // Spread across ~3 months so the date grouping has real work to do.
            let date = format!("2025-{:02}-{:02}", 1 + (e % 3), 1 + (e % 28));
            expenses.push(make_expense(
                &key,
                e,
                &format!("expense {e}"),
                100.0,
                ExpenseType::Expense,
                &date,
            ));

            let payer = 1 + (e % USERS);
            payment_id += 1;
            payments.push(make_payment(&key, payment_id, e, payer, false, 100.0));
            for d in 0..4 {
                let debtor = 1 + ((e + d) % USERS);
                payment_id += 1;
                payments.push(make_payment(&key, payment_id, e, debtor, true, 25.0));
            }
        }

        LargeProject { users, expenses, payments }
    })
}
