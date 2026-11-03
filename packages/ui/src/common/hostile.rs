//! What a key holder or the server can put in front of every member's app: plaintext under the
//! project key, and the ids and timestamps around it. The property tests feed these to each page.

use chrono::{Datelike, NaiveDate, NaiveDateTime};
use proptest::prelude::*;
use shared::{
    EncryptedPair, Expense, ExpensePayload, ExpenseShare, Payment, PaymentMethod, PaymentMethods, PaymentPayload, User, UserPayload,
};
use uuid::Uuid;

use crate::common::test_fixtures::TEST_KEY;
use crate::crypto::{encrypt, encrypt_json};
use crate::recurring::model::{Ends, Freq, RecurringPayload, Rule, Side, Template};

pub(crate) fn amount() -> impl Strategy<Value = f64> {
    prop_oneof![
        any::<f64>(),
        Just(shared::MAX_AMOUNT),
        Just(-shared::MAX_AMOUNT),
        Just(f64::MAX),
        Just(f64::MIN_POSITIVE),
        Just(-0.0),
        -1e6f64..1e6,
    ]
}

pub(crate) fn id() -> impl Strategy<Value = i32> {
    prop_oneof![any::<i32>(), Just(i32::MIN), Just(i32::MAX), Just(0), 1..4i32]
}

pub(crate) fn text() -> impl Strategy<Value = String> {
    prop_oneof![
        any::<String>(),
        Just(String::new()),
        Just("2025-02-30".into()),
        Just("2025-01-01".into()),
        Just("9999-12-31".into()),
        Just("0001-01-01".into()),
        Just("-262144-01-01".into()),
        Just("-262143-01-01".into()),
        Just("+262143-12-31".into()),
        Just("+262142-12-31".into()),
    ]
}

pub(crate) fn date() -> impl Strategy<Value = NaiveDate> {
    let days = NaiveDate::MIN.num_days_from_ce()..=NaiveDate::MAX.num_days_from_ce();
    prop_oneof![
        days.prop_map(|d| NaiveDate::from_num_days_from_ce_opt(d).unwrap_or_default()),
        Just(NaiveDate::MIN),
        Just(NaiveDate::MAX),
        (0..2000i32).prop_map(|d| NaiveDate::from_ymd_opt(2025, 1, 1).unwrap() + chrono::Duration::days(d as i64)),
    ]
}

pub(crate) fn timestamp() -> impl Strategy<Value = NaiveDateTime> {
    prop_oneof![Just(NaiveDateTime::MIN), Just(NaiveDateTime::MAX), date().prop_map(|d| d.and_time(Default::default()))]
}

fn expense_type() -> impl Strategy<Value = String> {
    prop_oneof![Just("expense".into()), Just("transfer".into()), Just("gain".into()), any::<String>()]
}

fn share() -> impl Strategy<Value = ExpenseShare> {
    (id(), amount(), any::<bool>()).prop_map(|(user_id, amount, is_debt)| ExpenseShare { user_id, amount, is_debt })
}

pub(crate) fn expense_payload() -> impl Strategy<Value = ExpensePayload> {
    (
        (text(), amount(), expense_type(), text()),
        (proptest::option::of(text()), proptest::option::of(amount()), proptest::option::of(amount())),
        (proptest::option::of(id()), proptest::option::of(prop::collection::vec(share(), 0..8)), any::<bool>()),
        proptest::option::of(any::<u128>()),
    )
        .prop_map(|((name, amount, expense_type, date), (category, source_amount, rate), (author_id, shares, estimate), recurring)| {
            ExpensePayload {
                name,
                amount,
                expense_type,
                date,
                description: None,
                category,
                source_currency: source_amount.map(|_| "USD".into()),
                source_amount,
                rate,
                recurring_id: recurring.map(|r| Uuid::from_u128(r % 3)),
                estimate,
                author_id,
                shares,
                extra: Default::default(),
            }
        })
}

/// Rows under [`TEST_KEY`], with the ids and times the server chose.
pub(crate) fn expenses() -> impl Strategy<Value = Vec<Expense>> {
    prop::collection::vec((id(), proptest::option::of(id()), timestamp(), expense_payload()), 1..12).prop_map(|rows| {
        rows.into_iter()
            .map(|(id, author_id, created_at, p)| Expense {
                id,
                author_id,
                project_id: Uuid::nil(),
                created_at,
                payload: encrypt_json(&TEST_KEY, &p).unwrap(),
            })
            .collect()
    })
}

pub(crate) fn payments() -> impl Strategy<Value = Vec<Payment>> {
    prop::collection::vec((id(), id(), id(), amount(), any::<bool>()), 0..12).prop_map(|rows| {
        rows.into_iter()
            .map(|(id, expense_id, user_id, amount, is_debt)| Payment {
                id,
                expense_id,
                user_id,
                payload: encrypt_json(&TEST_KEY, &PaymentPayload { amount, is_debt }).unwrap(),
                created_at: NaiveDateTime::default(),
            })
            .collect()
    })
}

fn payment_method() -> impl Strategy<Value = PaymentMethod> {
    let kind = prop_oneof![Just("iban".to_string()), Just("other".to_string()), any::<String>()];
    (kind, any::<String>(), proptest::option::of(any::<String>()), any::<bool>())
        .prop_map(|(kind, value, label, shared)| PaymentMethod { kind, value, label, shared })
}

/// A claim label or a payment-method copy: well-formed, or any plaintext at all.
fn project_blob<T: serde::Serialize + std::fmt::Debug>(valid: impl Strategy<Value = T>) -> impl Strategy<Value = EncryptedPair> {
    prop_oneof![
        valid.prop_map(|v| encrypt_json(&TEST_KEY, &v).unwrap()),
        any::<String>().prop_map(|plain| encrypt(&TEST_KEY, &plain).unwrap()),
    ]
}

pub(crate) fn users() -> impl Strategy<Value = Vec<User>> {
    let methods = prop::collection::vec(payment_method(), 0..4).prop_map(|methods| PaymentMethods { methods });
    let row = (
        id(),
        any::<String>(),
        any::<bool>(),
        any::<bool>(),
        proptest::option::of(project_blob(any::<String>())),
        proptest::option::of(project_blob(methods)),
    );
    prop::collection::vec(row, 0..5).prop_map(|rows| {
        rows.into_iter()
            .map(|(id, name, removed, claimed, claim_label, payment_methods)| User {
                id,
                payload: encrypt_json(&TEST_KEY, &UserPayload { name, removed }).unwrap(),
                claimed,
                claim_label,
                payment_methods,
                ..Default::default()
            })
            .collect()
    })
}

fn side() -> impl Strategy<Value = Side> {
    (any::<bool>(), prop::collection::vec((id(), amount()), 0..5)).prop_map(|(exact, entries)| Side { exact, entries })
}

fn rule() -> impl Strategy<Value = Rule> {
    let freq = prop_oneof![Just(Freq::Weekly), Just(Freq::Monthly), Just(Freq::Yearly)];
    let interval = prop_oneof![any::<u32>(), 0..14u32];
    let ends = prop_oneof![Just(Ends::Never), date().prop_map(Ends::On), any::<u32>().prop_map(Ends::After), (0..30u32).prop_map(Ends::After)];
    (freq, interval, date(), ends).prop_map(|(freq, interval, anchor, ends)| Rule { freq, interval, anchor, ends })
}

pub(crate) fn rule_payload() -> impl Strategy<Value = RecurringPayload> {
    (
        (text(), expense_type(), proptest::option::of(text()), amount(), side(), side()),
        (proptest::option::of(amount()), proptest::option::of(amount()), proptest::option::of(amount()), any::<bool>()),
        (rule(), prop_oneof![any::<u32>(), 0..40u32], any::<bool>(), any::<u128>(), proptest::option::of(id())),
    )
        .prop_map(
            |((name, expense_type, category, amount, payers, debtors), (source_amount, rate, rate_ack, variable), (rule, next, paused, seed, author_id))| {
                RecurringPayload {
                    template: Template {
                        name,
                        expense_type,
                        category,
                        amount,
                        payers,
                        debtors,
                        source_currency: source_amount.map(|_| "USD".into()),
                        source_amount,
                        rate,
                        rate_ack,
                        variable,
                        extra: Default::default(),
                    },
                    rule,
                    next,
                    paused,
                    seed: Uuid::from_u128(seed),
                    author_id,
                    extra: Default::default(),
                }
            },
        )
}
