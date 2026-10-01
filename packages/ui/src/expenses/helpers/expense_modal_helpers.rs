//! Shared between `AddExpenseModal` and `EditExpenseModal`: the two differ only in which
//! server function they call and what history summary they write.

use chrono::NaiveDate;
use crate::tid;
use shared::{
    sums_to_total, EncryptedPair, EncryptedUserAmount, ExpensePayload, ExpenseType, PaymentPayload,
};

use super::expense_form_helpers::UserEntry;
use crate::crypto::encrypt_json;

/// A submitted form that passed validation.
#[derive(Debug)]
pub struct ValidatedExpense {
    pub name: String,
    pub total: f64,
    pub date: String,
    pub payers: Vec<(i32, f64)>,
    pub debtors: Vec<(i32, f64)>,
}

/// Why a submitted form was rejected.
///
/// Data, not a sentence: the wording is a translation, and keeping this pure is what lets the whole
/// validator be unit-tested without a Dioxus runtime. Render it with [`FormError::message`].
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FormError {
    NameRequired,
    AmountNotPositive,
    NoPayer,
    NoDebtor,
    InvalidDate,
    /// One side does not add up. `sum` is what it adds up to, `total` what it should.
    PayersMismatch { sum: f64, total: f64 },
    DebtorsMismatch { sum: f64, total: f64 },
    /// A hand-typed exchange rate that is not a number, not positive, or large enough that
    /// `amount * rate` would stop being finite.
    RateInvalid,
    /// A foreign currency with the rate field left blank and no rate table to fall back on —
    /// offline, or the server could not reach InforEuro. The user can still type one.
    RateUnavailable,
}

impl FormError {
    /// The message to show. Needs an i18n context, so call it from a component or one of its tasks.
    ///
    /// The two mismatch variants are separate messages rather than one with an interpolated noun:
    /// French needs "le total des payeurs" / "des débiteurs", and other languages inflect the
    /// surrounding words differently again.
    pub fn message(self) -> String {
        match self {
            Self::NameRequired => tid!("expense-name-required"),
            Self::AmountNotPositive => tid!("expense-amount-not-positive"),
            Self::NoPayer => tid!("expense-no-payer"),
            Self::NoDebtor => tid!("expense-no-debtor"),
            Self::InvalidDate => tid!("expense-invalid-date"),
            Self::PayersMismatch { sum, total } => {
                tid!("expense-payers-mismatch", sum: format!("{sum:.2}"), total: format!("{total:.2}"))
            }
            Self::DebtorsMismatch { sum, total } => {
                tid!("expense-debtors-mismatch", sum: format!("{sum:.2}"), total: format!("{total:.2}"))
            }
            Self::RateInvalid => tid!("expense-rate-invalid"),
            Self::RateUnavailable => tid!("expense-rate-unavailable"),
        }
    }
}

/// Resolves the rate field into a [`Conversion`], or says why it cannot.
///
/// `typed` is the rate field's raw contents; blank means "use the InforEuro rate", which is what
/// `auto_rate` carries when one is available. Split out of the form so it is testable without a
/// Dioxus runtime, like every other validator here.
pub fn resolve_conversion(
    source_currency: &str,
    project_currency: &str,
    source_amount: f64,
    typed: &str,
    auto_rate: Option<f64>,
) -> Result<Option<Conversion>, FormError> {
    if source_currency == project_currency {
        return Ok(None);
    }
    let rate = match typed.trim() {
        "" => auto_rate.ok_or(FormError::RateUnavailable)?,
        raw => super::expense_form_helpers::parse_amount(raw).ok_or(FormError::RateInvalid)?,
    };
    Conversion::new(source_currency, project_currency, source_amount, rate)
        .ok_or(FormError::RateInvalid)
        .map(Some)
}

/// Refuses a payload whose recorded total does not equal its own `source_amount * rate`.
///
/// `validated_total` is the figure the participant split was checked against; `conversion` is what
/// the payload will carry. If those two ever disagree the expense is self-contradictory the moment
/// it is written, and under E2EE nobody downstream can catch it — the server sees ciphertext, and
/// every reader trusts `amount`. Cheap to check, so it is checked.
pub fn conversion_matches_total(
    validated_total: f64,
    conversion: Option<&Conversion>,
) -> Result<(), FormError> {
    let Some(c) = conversion else { return Ok(()) };
    if shared::to_cents(c.converted_total()) == shared::to_cents(validated_total) {
        return Ok(());
    }
    Err(FormError::RateInvalid)
}

/// Validates a submitted expense form.
pub fn validate_expense_form(
    name: &str,
    total: f64,
    date: &str,
    payers: &[UserEntry],
    debtors: &[UserEntry],
) -> Result<ValidatedExpense, FormError> {
    let name = name.trim().to_string();
    if name.is_empty() {
        return Err(FormError::NameRequired);
    }
    if total <= 0.0 {
        return Err(FormError::AmountNotPositive);
    }
    let payers = active_amounts(payers);
    let debtors = active_amounts(debtors);
    if payers.is_empty() {
        return Err(FormError::NoPayer);
    }
    if debtors.is_empty() {
        return Err(FormError::NoDebtor);
    }
    if NaiveDate::parse_from_str(date, "%Y-%m-%d").is_err() {
        return Err(FormError::InvalidDate);
    }
    // Both sides must add up to the total, or the balances they produce never net to zero and the
    // reimbursement suggestions can't settle. The server cannot check this — the amounts reach it
    // encrypted — so this is the only place it is enforced at write time.
    if let Some(sum) = side_mismatch(total, &payers) {
        return Err(FormError::PayersMismatch { sum, total });
    }
    if let Some(sum) = side_mismatch(total, &debtors) {
        return Err(FormError::DebtorsMismatch { sum, total });
    }
    Ok(ValidatedExpense { name, total, date: date.to_string(), payers, debtors })
}

/// `None` when the side adds up to `total`, otherwise what it actually adds up to.
fn side_mismatch(total: f64, entries: &[(i32, f64)]) -> Option<f64> {
    if sums_to_total(total, entries.iter().map(|(_, a)| *a)) {
        return None;
    }
    Some(entries.iter().map(|(_, a)| a).sum())
}

fn active_amounts(entries: &[UserEntry]) -> Vec<(i32, f64)> {
    entries
        .iter()
        .filter(|e| e.checked && e.amount > 0.0)
        .map(|e| (e.user.id, e.amount))
        .collect()
}

pub fn encrypt_user_amounts(
    key: &[u8; 32],
    entries: &[(i32, f64)],
    is_debt: bool,
) -> Result<Vec<EncryptedUserAmount>, String> {
    entries
        .iter()
        .map(|(user_id, amount)| {
            Ok(EncryptedUserAmount {
                user_id: *user_id,
                payload: encrypt_json(key, &PaymentPayload { amount: *amount, is_debt })?,
            })
        })
        .collect()
}

/// What an expense entered in another currency records alongside the converted amount.
///
/// Built by [`Conversion::new`], which is the only way one is made: it refuses anything that would
/// not convert, so a `Conversion` in hand is always usable.
#[derive(Debug, Clone, PartialEq)]
pub struct Conversion {
    pub source_currency: String,
    pub source_amount: f64,
    pub rate: f64,
}

impl Conversion {
    /// `None` when the currency is the project's own (nothing to record), or when the amount or the
    /// rate could not produce a finite converted total.
    pub fn new(
        source_currency: &str,
        project_currency: &str,
        source_amount: f64,
        rate: f64,
    ) -> Option<Self> {
        if source_currency == project_currency
            || !source_amount.is_finite()
            || !shared::is_valid_rate(rate)
        {
            return None;
        }
        Some(Self {
            source_currency: source_currency.to_string(),
            source_amount,
            rate,
        })
    }

    /// The project-currency total this expense is booked at.
    ///
    /// **The only place the multiplication happens.** Converting the total once and letting
    /// `distribute` split the result into cents is what keeps `sum(payers) == total ==
    /// sum(debtors)` exact; converting each share separately would round each one and drift.
    pub fn converted_total(&self) -> f64 {
        shared::convert_to_project(self.source_amount, self.rate)
    }
}

/// `amount` is the total **in the project's currency** — already converted when `conversion` is
/// `Some`. Never the amount as typed: every reader of an expense sums one currency.
pub fn encrypt_expense_payload(
    key: &[u8; 32],
    name: &str,
    amount: f64,
    date_str: &str,
    expense_type: &ExpenseType,
    category: Option<String>,
    conversion: Option<&Conversion>,
) -> Result<EncryptedPair, String> {
    encrypt_json(key, &expense_payload(name, amount, date_str, expense_type, category, conversion))
}

pub fn expense_payload(
    name: &str,
    amount: f64,
    date_str: &str,
    expense_type: &ExpenseType,
    category: Option<String>,
    conversion: Option<&Conversion>,
) -> ExpensePayload {
    ExpensePayload {
        name: name.to_string(),
        amount,
        expense_type: expense_type.as_str().to_string(),
        date: date_str.to_string(),
        description: None,
        category,
        source_currency: conversion.map(|c| c.source_currency.clone()),
        source_amount: conversion.map(|c| c.source_amount),
        rate: conversion.map(|c| c.rate),
        recurring_id: None,
        estimate: false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypto::decrypt_json;
    use shared::{EncryptedPair, User};

    use crate::common::test_fixtures::test_key;

    fn entry(id: i32, checked: bool, amount: f64) -> UserEntry {
        UserEntry {
            user: User { id, payload: EncryptedPair::default(), ..Default::default() },
            display_name: String::new(),
            checked,
            amount,
            shares: 1.0,
        }
    }

    // ---- encrypt_user_amounts ----

    #[test]
    fn payer_is_debt_false() {
        let key = test_key();
        let result = encrypt_user_amounts(&key, &[(1, 30.0)], false).unwrap();
        assert_eq!(result[0].user_id, 1);
        let pp: PaymentPayload = decrypt_json(&key, &result[0].payload).unwrap();
        assert!(!pp.is_debt);
        assert!((pp.amount - 30.0).abs() < 1e-9);
    }

    #[test]
    fn debtor_is_debt_true() {
        let key = test_key();
        let result = encrypt_user_amounts(&key, &[(2, 15.0)], true).unwrap();
        let pp: PaymentPayload = decrypt_json(&key, &result[0].payload).unwrap();
        assert!(pp.is_debt);
        assert!((pp.amount - 15.0).abs() < 1e-9);
    }

    #[test]
    fn multiple_entries_all_encrypted() {
        let key = test_key();
        let entries = [(1, 10.0), (2, 20.0), (3, 30.0)];
        let result = encrypt_user_amounts(&key, &entries, false).unwrap();
        assert_eq!(result.len(), 3);
        for (i, (user_id, amount)) in entries.iter().enumerate() {
            assert_eq!(result[i].user_id, *user_id);
            let pp: PaymentPayload = decrypt_json(&key, &result[i].payload).unwrap();
            assert!((pp.amount - amount).abs() < 1e-9);
        }
    }

    #[test]
    fn wrong_key_fails_decrypt() {
        let key = test_key();
        let result = encrypt_user_amounts(&key, &[(1, 50.0)], false).unwrap();
        let mut bad = key;
        bad[0] ^= 0xFF;
        assert!(decrypt_json::<PaymentPayload>(&bad, &result[0].payload).is_err());
    }

    // ---- validate_expense_form ----

    #[test]
    fn validate_rejects_empty_name() {
        let err = validate_expense_form("   ", 10.0, "2025-01-01", &[entry(1, true, 10.0)], &[entry(2, true, 10.0)])
            .unwrap_err();
        assert_eq!(err, FormError::NameRequired);
    }

    #[test]
    fn validate_rejects_non_positive_amount() {
        let err = validate_expense_form("x", 0.0, "2025-01-01", &[entry(1, true, 10.0)], &[entry(2, true, 10.0)])
            .unwrap_err();
        assert_eq!(err, FormError::AmountNotPositive);
    }

    #[test]
    fn validate_rejects_no_payer() {
        let err = validate_expense_form("x", 10.0, "2025-01-01", &[entry(1, false, 10.0)], &[entry(2, true, 10.0)])
            .unwrap_err();
        assert_eq!(err, FormError::NoPayer);
    }

    #[test]
    fn validate_rejects_no_debtor() {
        let err = validate_expense_form("x", 10.0, "2025-01-01", &[entry(1, true, 10.0)], &[entry(2, true, 0.0)])
            .unwrap_err();
        assert_eq!(err, FormError::NoDebtor);
    }

    #[test]
    fn validate_rejects_bad_date() {
        let err = validate_expense_form("x", 10.0, "not-a-date", &[entry(1, true, 10.0)], &[entry(2, true, 10.0)])
            .unwrap_err();
        assert_eq!(err, FormError::InvalidDate);
    }

    // ---- validate_expense_form: the sums must balance ----

    /// The reported bug: a 100 expense saved with 120 paid and 140 owed.
    #[test]
    fn validate_rejects_the_reported_scenario() {
        let err = validate_expense_form(
            "Dîner",
            100.0,
            "2025-01-01",
            &[entry(1, true, 60.0), entry(2, true, 60.0)],
            &[entry(1, true, 70.0), entry(2, true, 70.0)],
        )
        .unwrap_err();
        assert_eq!(err, FormError::PayersMismatch { sum: 120.0, total: 100.0 });
    }

    #[test]
    fn validate_rejects_payers_over_and_short() {
        let over = validate_expense_form(
            "x",
            100.0,
            "2025-01-01",
            &[entry(1, true, 150.0)],
            &[entry(2, true, 100.0)],
        )
        .unwrap_err();
        assert_eq!(over, FormError::PayersMismatch { sum: 150.0, total: 100.0 });

        let short = validate_expense_form(
            "x",
            100.0,
            "2025-01-01",
            &[entry(1, true, 40.0)],
            &[entry(2, true, 100.0)],
        )
        .unwrap_err();
        assert_eq!(short, FormError::PayersMismatch { sum: 40.0, total: 100.0 });
    }

    #[test]
    fn validate_rejects_debtors_over_and_short() {
        let over = validate_expense_form(
            "x",
            100.0,
            "2025-01-01",
            &[entry(1, true, 100.0)],
            &[entry(2, true, 60.0), entry(3, true, 60.0)],
        )
        .unwrap_err();
        assert_eq!(over, FormError::DebtorsMismatch { sum: 120.0, total: 100.0 });

        let short = validate_expense_form(
            "x",
            100.0,
            "2025-01-01",
            &[entry(1, true, 100.0)],
            &[entry(2, true, 30.0), entry(3, true, 30.0)],
        )
        .unwrap_err();
        assert_eq!(short, FormError::DebtorsMismatch { sum: 60.0, total: 100.0 });
    }

    #[test]
    fn validate_rejects_a_single_cent_gap() {
        // The boundary that must not be tolerated: one cent is a real imbalance, not float noise.
        assert!(validate_expense_form(
            "x",
            100.0,
            "2025-01-01",
            &[entry(1, true, 99.99)],
            &[entry(2, true, 100.0)],
        )
        .is_err());
        assert!(validate_expense_form(
            "x",
            100.0,
            "2025-01-01",
            &[entry(1, true, 100.0)],
            &[entry(2, true, 100.01)],
        )
        .is_err());
    }

    #[test]
    fn validate_accepts_a_cent_rounded_split() {
        let v = validate_expense_form(
            "x",
            100.0,
            "2025-01-01",
            &[entry(1, true, 100.0)],
            &[entry(1, true, 33.33), entry(2, true, 33.33), entry(3, true, 33.34)],
        )
        .unwrap();
        assert_eq!(v.debtors.len(), 3);
    }

    #[test]
    fn validate_accepts_a_payer_who_is_also_a_debtor() {
        // Paying and owing on the same expense is the normal case, not a duplicate.
        let v = validate_expense_form(
            "x",
            100.0,
            "2025-01-01",
            &[entry(1, true, 100.0)],
            &[entry(1, true, 50.0), entry(2, true, 50.0)],
        )
        .unwrap();
        assert_eq!(v.payers, vec![(1, 100.0)]);
        assert_eq!(v.debtors, vec![(1, 50.0), (2, 50.0)]);
    }

    #[test]
    fn validate_accepts_what_the_splitter_produced_for_a_sub_cent_total() {
        // 100.005 quantises to 10000 cents, so `distribute` hands back 100.00 — the validator must
        // agree with the splitter instead of rejecting its own output.
        use super::super::expense_form_helpers::distribute;
        let total = 100.005;
        let mut payers = vec![entry(1, true, 0.0)];
        let mut debtors = vec![entry(2, true, 0.0), entry(3, true, 0.0), entry(4, true, 0.0)];
        distribute(total, &mut payers);
        distribute(total, &mut debtors);
        assert!(validate_expense_form("x", total, "2025-01-01", &payers, &debtors).is_ok());
    }

    #[test]
    fn validate_ignores_unchecked_rows_carrying_stale_amounts() {
        // `redistribute` leaves an unchecked row's amount alone when nothing is checked, so a stale
        // 50 can survive there — it must not count against the total.
        let v = validate_expense_form(
            "x",
            100.0,
            "2025-01-01",
            &[entry(1, true, 100.0), entry(9, false, 50.0)],
            &[entry(2, true, 100.0), entry(8, false, 50.0)],
        )
        .unwrap();
        assert_eq!(v.payers, vec![(1, 100.0)]);
    }

    #[test]
    fn validate_reports_the_empty_side_before_the_mismatch() {
        // A side with nothing selected is the more actionable message of the two.
        let err = validate_expense_form(
            "x",
            100.0,
            "2025-01-01",
            &[entry(1, true, 0.0)],
            &[entry(2, true, 70.0)],
        )
        .unwrap_err();
        assert_eq!(err, FormError::NoPayer);
    }

    #[test]
    fn validate_precedence_is_name_total_sides_date_then_sums() {
        let bad = |name: &str, total: f64, date: &str| {
            validate_expense_form(
                name,
                total,
                date,
                &[entry(1, true, 1.0)], // never matches a 100 total
                &[entry(2, true, 2.0)],
            )
            .unwrap_err()
        };
        assert_eq!(bad("", 100.0, "2025-01-01"), FormError::NameRequired);
        assert_eq!(bad("x", 0.0, "2025-01-01"), FormError::AmountNotPositive);
        assert_eq!(bad("x", 100.0, "nope"), FormError::InvalidDate);
        // …and with everything else valid, the payers mismatch is reported before the debtors one.
        assert!(matches!(bad("x", 100.0, "2025-01-01"), FormError::PayersMismatch { .. }));
    }

    #[test]
    fn validate_accepts_a_hundred_participants_per_side() {
        // No accidental client-side cap: the server's cap is 100 per side, the form must reach it.
        let payers: Vec<UserEntry> = (1..=100).map(|id| entry(id, true, 1.0)).collect();
        let debtors: Vec<UserEntry> = (101..=200).map(|id| entry(id, true, 1.0)).collect();
        let v = validate_expense_form("x", 100.0, "2025-01-01", &payers, &debtors).unwrap();
        assert_eq!(v.payers.len(), 100);
        assert_eq!(v.debtors.len(), 100);
    }

    #[test]
    fn validate_trims_name_and_collects_amounts() {
        let v = validate_expense_form(
            "  Dîner  ",
            30.0,
            "2025-06-15",
            &[entry(1, true, 30.0), entry(9, false, 5.0)],
            &[entry(2, true, 15.0), entry(3, true, 15.0)],
        )
        .unwrap();
        assert_eq!(v.name, "Dîner");
        assert_eq!(v.payers, vec![(1, 30.0)]);
        assert_eq!(v.debtors, vec![(2, 15.0), (3, 15.0)]);
        assert_eq!(v.date, "2025-06-15");
    }

    // ---- encrypt_expense_payload ----

    #[test]
    fn payload_roundtrips_all_fields() {
        let key = test_key();
        let pair = encrypt_expense_payload(
            &key,
            "Dîner",
            45.0,
            "2025-06-15",
            &ExpenseType::Expense,
            Some("food".to_string()),
            None,
        )
        .unwrap();
        let ep: ExpensePayload = decrypt_json(&key, &pair).unwrap();
        assert_eq!(ep.name, "Dîner");
        assert!((ep.amount - 45.0).abs() < 1e-9);
        assert_eq!(ep.date, "2025-06-15");
        assert_eq!(ep.expense_type, ExpenseType::Expense.as_str());
        assert_eq!(ep.category.unwrap(), "food");
        assert!(ep.description.is_none());
        assert!(ep.source_currency.is_none());
        assert!(ep.rate.is_none());
    }

    #[test]
    fn payload_expense_type_all_variants() {
        let key = test_key();
        for t in [ExpenseType::Expense, ExpenseType::Transfer, ExpenseType::Gain] {
            let pair =
                encrypt_expense_payload(&key, "x", 10.0, "2025-01-01", &t, None, None).unwrap();
            let ep: ExpensePayload = decrypt_json(&key, &pair).unwrap();
            assert_eq!(ep.expense_type, t.as_str());
        }
    }

    #[test]
    fn payload_wrong_key_fails() {
        let key = test_key();
        let pair = encrypt_expense_payload(
            &key,
            "secret",
            10.0,
            "2025-01-01",
            &ExpenseType::Expense,
            None,
            None,
        )
        .unwrap();
        let mut bad = key;
        bad[0] ^= 0xFF;
        assert!(decrypt_json::<ExpensePayload>(&bad, &pair).is_err());
    }

    // ---- conversion ----

    /// `amount` is the converted total, and the source figures ride alongside it. This is the
    /// contract every other reader depends on.
    #[test]
    fn payload_records_the_converted_total_and_the_source() {
        let key = test_key();
        let c = Conversion::new("USD", "EUR", 135.0, 0.860437).unwrap();
        let pair = encrypt_expense_payload(
            &key,
            "Diner",
            c.converted_total(),
            "2026-09-04",
            &ExpenseType::Expense,
            None,
            Some(&c),
        )
        .unwrap();
        let ep: ExpensePayload = decrypt_json(&key, &pair).unwrap();
        assert_eq!(ep.amount, 116.16, "amount must be the EUR total, not the USD one");
        assert_eq!(ep.source_currency.unwrap(), "USD");
        assert_eq!(ep.source_amount.unwrap(), 135.0);
        assert_eq!(ep.rate.unwrap(), 0.860437);
    }

    #[test]
    fn conversion_is_none_in_the_project_currency() {
        assert!(Conversion::new("EUR", "EUR", 10.0, 1.0).is_none());
    }

    #[test]
    fn conversion_rejects_unusable_inputs() {
        for rate in [0.0, -1.0, f64::NAN, f64::INFINITY, 1e12] {
            assert!(Conversion::new("USD", "EUR", 10.0, rate).is_none(), "accepted rate {rate}");
        }
        assert!(Conversion::new("USD", "EUR", f64::NAN, 1.0).is_none());
    }

    // ---- resolve_conversion ----

    #[test]
    fn resolve_returns_none_in_the_project_currency() {
        assert_eq!(resolve_conversion("EUR", "EUR", 10.0, "", None), Ok(None));
        // Even with a rate typed in — the field is not shown, so it cannot mean anything.
        assert_eq!(resolve_conversion("EUR", "EUR", 10.0, "2.0", None), Ok(None));
    }

    #[test]
    fn resolve_falls_back_to_the_auto_rate_when_blank() {
        let c = resolve_conversion("USD", "EUR", 135.0, "  ", Some(0.860437)).unwrap().unwrap();
        assert_eq!(c.rate, 0.860437);
        assert_eq!(c.converted_total(), 116.16);
    }

    /// A typed rate always wins, which is what makes the InforEuro rate a default rather than a rule.
    #[test]
    fn resolve_prefers_a_typed_rate() {
        let c = resolve_conversion("USD", "EUR", 10.0, "2", Some(0.86)).unwrap().unwrap();
        assert_eq!(c.rate, 2.0);
        assert_eq!(c.converted_total(), 20.0);
    }

    /// FR mobile keyboards produce a comma, and `parse_amount` is the one place that is handled.
    #[test]
    fn resolve_accepts_a_decimal_comma() {
        let c = resolve_conversion("USD", "EUR", 10.0, "0,5", None).unwrap().unwrap();
        assert_eq!(c.rate, 0.5);
    }

    #[test]
    fn resolve_reports_a_blank_rate_with_no_table() {
        assert_eq!(
            resolve_conversion("USD", "EUR", 10.0, "", None),
            Err(FormError::RateUnavailable)
        );
    }

    #[test]
    fn resolve_reports_an_unusable_typed_rate() {
        for typed in ["abc", "0", "-1", "1e12"] {
            assert_eq!(
                resolve_conversion("USD", "EUR", 10.0, typed, Some(0.86)),
                Err(FormError::RateInvalid),
                "accepted {typed}"
            );
        }
    }
}
