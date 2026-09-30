use shared::{to_cents, ExpenseType, User};
use crate::tid;

#[derive(Clone, PartialEq)]
pub struct UserEntry {
    pub user: User,
    pub display_name: String,
    pub checked: bool,
    pub amount: f64,
    pub shares: f64,
}

/// `preselected` is checked with `init_amount`, everyone else zero. With no preselection
/// `check_rest` decides the rest — that is what makes debtors default to everyone, payers nobody.
pub fn init_entries(
    users: &[User],
    display_names: &[String],
    preselected: Option<i32>,
    init_amount: f64,
    check_rest: bool,
) -> Vec<UserEntry> {
    users
        .iter()
        .enumerate()
        .map(|(i, u)| {
            let is_pre = preselected == Some(u.id);
            let checked = is_pre || (preselected.is_none() && check_rest);
            UserEntry {
                display_name: display_names.get(i).cloned().unwrap_or_default(),
                user: u.clone(),
                checked,
                amount: if is_pre { init_amount } else { 0.0 },
                shares: if checked { 1.0 } else { 0.0 },
            }
        })
        .collect()
}

/// `init_entries`, then the same even split the first keystroke in the total field would apply.
///
/// `init_entries` gives the whole amount to the *preselected* participant and 0.00 to everyone
/// else. That is right for payers — one person paid — and wrong for debtors, where nothing is
/// preselected so every one of them is checked carrying 0.00. Typing the total calls `redistribute`
/// (see `expense_form.rs`), but an amount prefilled from a scanned receipt fires no `oninput`, so
/// the modal would open showing "0.00 / 13.90, Reste 13.90" and split nothing.
///
/// Routing the seed through `redistribute` keeps it consistent with the keystroke path by
/// construction rather than by a second copy of the same arithmetic.
pub fn seed_entries(
    users: &[User],
    display_names: &[String],
    preselected: Option<i32>,
    init_amount: f64,
    check_rest: bool,
) -> Vec<UserEntry> {
    let mut entries = init_entries(users, display_names, preselected, init_amount, check_rest);
    // `false` is `share_mode`, not the debtor flag: both share-mode signals start off, so the seed
    // must use the even split.
    if init_amount != 0.0 {
        redistribute(init_amount, &mut entries, false);
    }
    entries
}

/// Checked with the recorded amount when the user has a payment on the matching side.
pub fn init_entries_from_payments(
    users: &[User],
    display_names: &[String],
    payments: &[crate::decrypted::DecryptedPayment],
    is_debt: bool,
) -> Vec<UserEntry> {
    users
        .iter()
        .enumerate()
        .map(|(i, u)| {
            let payment = payments.iter().find(|p| p.is_debt == is_debt && p.user_id == u.id);
            UserEntry {
                display_name: display_names.get(i).cloned().unwrap_or_default(),
                user: u.clone(),
                checked: payment.is_some(),
                amount: payment.map(|p| p.amount).unwrap_or(0.0),
                shares: if payment.is_some() { 1.0 } else { 0.0 },
            }
        })
        .collect()
}

/// Accepts the decimal comma (FR mobile keyboards) and one `a op b` with `/`, `*`, `+` or `-`.
/// `None` means empty or mid-typing — callers must then touch nothing, or Dioxus rewrites `value`
/// and clobbers the keystroke in progress.
pub fn parse_amount(raw: &str) -> Option<f64> {
    let s = raw.trim().replace(',', ".");
    if s.is_empty() {
        return None;
    }
    parse_plain(&s).or_else(|| evaluate_expression(&s))
}

fn parse_plain(s: &str) -> Option<f64> {
    s.trim().parse::<f64>().ok().filter(|v| v.is_finite() && *v >= 0.0)
}

const OPERATORS: [char; 4] = ['/', '*', '+', '-'];

/// A leading `-` is an operator with an empty left side, so `-5` and `2*-3` fail here as they do
/// in `parse_plain`. Rounded to cents so the settled field reads `83.33`, not `83.333…`.
fn evaluate_expression(s: &str) -> Option<f64> {
    let mut ops = s.match_indices(OPERATORS);
    let (at, op) = ops.next()?;
    if ops.next().is_some() {
        return None;
    }
    let a = parse_plain(&s[..at])?;
    let b = parse_plain(&s[at + op.len()..])?;
    let result = match op {
        "/" => a / b,
        "*" => a * b,
        "+" => a + b,
        _ => a - b,
    };
    Some(shared::round_currency(result)).filter(|v| v.is_finite() && *v >= 0.0)
}

pub fn is_expression(s: &str) -> bool {
    s.contains(OPERATORS)
}

/// The field text after tapping an operator key: appended to a plain number, swapped for a
/// trailing operator, refused when there is nothing to operate on or the expression is complete.
pub fn with_operator(shown: &str, op: char) -> Option<String> {
    let base = shown.trim_end_matches(OPERATORS);
    parse_plain(&base.replace(',', ".")).map(|_| format!("{base}{op}"))
}

/// What an amount field displays: the keystrokes as typed while they still mean `value`, the
/// number otherwise. Dioxus rewrites `value` on every render and an `f64` has no trailing "12."
/// or "12,0", so binding the number directly erases the separator under a mobile keyboard. A
/// draft that does not parse (empty, mid-typing) is kept too — the number it fails to be is what
/// the field last held, and clobbering it would refuse a clear-and-retype.
pub fn amount_field_text(draft: Option<&str>, value: f64) -> String {
    match draft {
        Some(d) if parse_amount(d).is_none_or(|v| v == value) => d.to_string(),
        _ => value.to_string(),
    }
}

/// Blur swaps the text for the number unless it is a plain one: an expression settles to its
/// result, and a draft that does not parse ("", "-") shows the value it left behind.
pub fn keeps_draft_on_blur(draft: &str) -> bool {
    !is_expression(draft) && parse_amount(draft).is_some()
}

/// The rate the form is actually converting at: what the user typed, else the InforEuro one.
///
/// `None` means "cannot convert right now" — a blank field with no rate table, or a rate that does
/// not parse or is out of range. Callers leave the split untouched rather than showing a wrong one;
/// `resolve_conversion` is what turns the same situation into a message on submit.
pub fn effective_rate(typed: &str, auto_rate: Option<f64>) -> Option<f64> {
    match typed.trim() {
        "" => auto_rate.filter(|r| shared::is_valid_rate(*r)),
        raw => parse_amount(raw).filter(|r| shared::is_valid_rate(*r)),
    }
}

/// The total in the project's currency, which is what the participant split is computed on.
///
/// `None` when a conversion is needed but not yet possible. The one multiplication site alongside
/// `Conversion::converted_total`, and both go through `shared::convert_to_project`.
pub fn project_total(source_amount: f64, foreign: bool, rate: Option<f64>) -> Option<f64> {
    if !foreign {
        return Some(source_amount);
    }
    rate.map(|r| shared::convert_to_project(source_amount, r))
}

pub fn redistribute(total: f64, entries: &mut [UserEntry], share_mode: bool) {
    if share_mode {
        distribute_by_shares(total, entries);
    } else {
        distribute(total, entries);
    }
}

/// An unchecked participant owes nothing, whatever amount they carried before.
fn clear_unchecked(entries: &mut [UserEntry]) {
    for entry in entries.iter_mut() {
        if !entry.checked {
            entry.amount = 0.0;
        }
    }
}

pub fn distribute(total: f64, entries: &mut [UserEntry]) {
    let checked_indices: Vec<usize> =
        entries.iter().enumerate().filter(|(_, e)| e.checked).map(|(i, _)| i).collect();
    let n = checked_indices.len();
    if n == 0 {
        return;
    }
    let total_cents = to_cents(total);
    let base_cents = total_cents / n as i64;
    let remainder = (total_cents % n as i64) as usize;
    for (pos, &idx) in checked_indices.iter().enumerate() {
        let cents = if pos < remainder { base_cents + 1 } else { base_cents };
        entries[idx].amount = cents as f64 / 100.0;
    }
    clear_unchecked(entries);
}

pub fn distribute_by_shares(total: f64, entries: &mut [UserEntry]) {
    let total_shares: f64 =
        entries.iter().filter(|e| e.checked && e.shares > 0.0).map(|e| e.shares).sum();
    if total_shares == 0.0 {
        return;
    }
    let total_cents = to_cents(total);
    let checked_indices: Vec<usize> = entries
        .iter()
        .enumerate()
        .filter(|(_, e)| e.checked && e.shares > 0.0)
        .map(|(i, _)| i)
        .collect();
    // Largest remainder: floor every share, then hand the leftover cents one each to the rows
    // that rounded down the most. Rounding each share and dumping the difference on the last row
    // went negative when the half-ups overshot (4 equal shares of 0.02 gave 1, 1, 1, −1).
    let exact: Vec<f64> = checked_indices
        .iter()
        .map(|&idx| entries[idx].shares / total_shares * total_cents as f64)
        .collect();
    let mut cents: Vec<i64> = exact.iter().map(|x| x.floor() as i64).collect();
    let leftover = total_cents - cents.iter().sum::<i64>();
    let mut by_remainder: Vec<usize> = (0..cents.len()).collect();
    by_remainder.sort_by(|&a, &b| {
        let ra = exact[a] - exact[a].floor();
        let rb = exact[b] - exact[b].floor();
        rb.partial_cmp(&ra).unwrap_or(std::cmp::Ordering::Equal)
    });
    for &pos in by_remainder.iter().take(leftover.max(0) as usize) {
        cents[pos] += 1;
    }
    for (pos, &idx) in checked_indices.iter().enumerate() {
        entries[idx].amount = cents[pos] as f64 / 100.0;
    }
    clear_unchecked(entries);
}

/// Shown in the collapsed header, so the common case needs no expanding.
pub fn participants_summary(entries: &[UserEntry]) -> ParticipantsSummary {
    let checked: Vec<&UserEntry> = entries.iter().filter(|e| e.checked).collect();
    match checked.len() {
        0 => ParticipantsSummary::None,
        1 => ParticipantsSummary::One(checked[0].display_name.clone()),
        k if k == entries.len() => ParticipantsSummary::Everyone(k),
        k => ParticipantsSummary::Some { checked: k, total: entries.len() },
    }
}

/// Data, not a sentence — see `FormError`. Render it with [`ParticipantsSummary::label`].
#[derive(Debug, Clone, PartialEq)]
pub enum ParticipantsSummary {
    None,
    /// Exactly one, named: "Alice" reads better than "Everyone (1)".
    One(String),
    Everyone(usize),
    Some { checked: usize, total: usize },
}

impl ParticipantsSummary {
    pub fn label(&self) -> String {
        match self {
            Self::None => tid!("participants-none"),
            Self::One(name) => name.clone(),
            Self::Everyone(k) => tid!("participants-everyone", count: *k as i64),
            Self::Some { checked, total } => {
                tid!("participants-some", count: *checked as i64, total: *total as i64)
            }
        }
    }
}

/// Translation keys, not labels — see `expense_type_label`.
pub fn payers_label(t: &ExpenseType) -> &'static str {
    match t {
        ExpenseType::Gain => "participants-who-received",
        ExpenseType::Transfer => "participants-who-transfers",
        _ => "participants-who-paid",
    }
}

pub fn debtors_label(t: &ExpenseType) -> &'static str {
    match t {
        ExpenseType::Transfer => "participants-who-receives",
        _ => "participants-for-whom",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::categories::get_expense_emoji;
    use shared::EncryptedPair;

    fn make_entries(checked: &[bool], amounts: &[f64], shares: &[f64]) -> Vec<UserEntry> {
        checked
            .iter()
            .zip(amounts)
            .zip(shares)
            .map(|((&c, &a), &s)| UserEntry {
                user: User { id: 0, payload: EncryptedPair::default(), ..Default::default() },
                display_name: String::new(),
                checked: c,
                amount: a,
                shares: s,
            })
            .collect()
    }

    fn total_amounts(entries: &[UserEntry]) -> f64 {
        entries.iter().map(|e| e.amount).sum::<f64>()
    }

    #[test]
    fn distribute_equal_split_3() {
        let mut e = make_entries(&[true, true, true], &[0.0; 3], &[1.0; 3]);
        distribute(10.0, &mut e);
        let amounts: Vec<f64> = e.iter().map(|x| x.amount).collect();
        assert!((total_amounts(&e) - 10.0).abs() < 0.001);
        // Two should be 3.33, one 3.34
        assert!(amounts.contains(&3.34) || amounts.contains(&3.33));
    }

    #[test]
    fn distribute_single_user() {
        let mut e = make_entries(&[true], &[0.0], &[1.0]);
        distribute(42.50, &mut e);
        assert!((e[0].amount - 42.50).abs() < 0.001);
    }

    #[test]
    fn distribute_unchecked_gets_zero() {
        let mut e = make_entries(&[true, false], &[5.0, 5.0], &[1.0, 1.0]);
        distribute(10.0, &mut e);
        assert!((e[1].amount - 0.0).abs() < 0.001);
        assert!((e[0].amount - 10.0).abs() < 0.001);
    }

    #[test]
    fn distribute_rounding_invariant() {
        for n in 1..=20usize {
            let checked = vec![true; n];
            let mut e = make_entries(&checked, &vec![0.0; n], &vec![1.0; n]);
            distribute(99999.99, &mut e);
            assert!((total_amounts(&e) - 99999.99).abs() < 0.001, "failed for n={n}");
        }
    }

    /// The form refuses a side that does not add up, so its own splits must pass that check at
    /// any row count and any total, sub-cent digits included.
    #[test]
    fn distribute_always_satisfies_the_form_validation() {
        for total in [0.01, 0.03, 10.0, 33.33, 100.005, 99999.99] {
            for n in 1..=20usize {
                let checked = vec![true; n];
                let mut e = make_entries(&checked, &vec![0.0; n], &vec![1.0; n]);
                distribute(total, &mut e);
                assert!(
                    shared::sums_to_total(total, e.iter().map(|x| x.amount)),
                    "distribute({total}) over {n} rows does not add up"
                );
            }
        }
    }

    /// Adds up **and** never goes negative: the sum alone let the last row absorb an overshoot
    /// from the other rows' half-up rounding, which the form then dropped as "unchecked".
    #[test]
    fn distribute_by_shares_always_satisfies_the_form_validation() {
        let share_sets: [&[f64]; 6] = [
            &[1.0, 1.0, 1.0],
            &[2.0, 1.0],
            &[0.75, 0.25],
            &[1.5, 1.0, 0.5],
            &[1.0],
            &[1.0, 1.0, 1.0, 1.0],
        ];
        for total in [0.01, 0.02, 0.03, 10.0, 33.33, 100.005, 99999.99] {
            for shares in share_sets {
                let checked = vec![true; shares.len()];
                let mut e = make_entries(&checked, &vec![0.0; shares.len()], shares);
                distribute_by_shares(total, &mut e);
                assert!(
                    shared::sums_to_total(total, e.iter().map(|x| x.amount)),
                    "distribute_by_shares({total}) over {shares:?} does not add up"
                );
                assert!(
                    e.iter().all(|x| x.amount >= 0.0),
                    "distribute_by_shares({total}) over {shares:?} went negative"
                );
            }
        }
    }

    #[test]
    fn distribute_by_shares_sub_cent_shares_stay_non_negative() {
        let mut e = make_entries(&[true; 4], &[0.0; 4], &[1.0; 4]);
        distribute_by_shares(0.02, &mut e);
        let amounts: Vec<f64> = e.iter().map(|x| x.amount).collect();
        assert_eq!(amounts, vec![0.01, 0.01, 0.0, 0.0]);
    }

    #[test]
    fn distribute_with_unchecked_rows_still_adds_up() {
        // `clear_unchecked` zeroes the stale amount; the checked rows carry the whole total.
        let mut e = make_entries(&[true, false, true], &[0.0, 50.0, 0.0], &[1.0, 1.0, 1.0]);
        distribute(100.0, &mut e);
        assert!(shared::sums_to_total(100.0, e.iter().map(|x| x.amount)));
    }

    #[test]
    fn distribute_quantisation_is_unchanged_by_the_to_cents_refactor() {
        // Pins the exact output of the expression `to_cents` replaced.
        let mut e = make_entries(&[true; 7], &[0.0; 7], &[1.0; 7]);
        distribute(99999.99, &mut e);
        let amounts: Vec<f64> = e.iter().map(|x| x.amount).collect();
        // 9_999_999 cents / 7 = 1_428_571 remainder 2 — the first two rows take the extra cent.
        assert_eq!(
            amounts,
            vec![14285.72, 14285.72, 14285.71, 14285.71, 14285.71, 14285.71, 14285.71]
        );
    }

    #[test]
    fn distribute_no_checked_is_noop() {
        let mut e = make_entries(&[false, false], &[5.0, 5.0], &[1.0, 1.0]);
        distribute(10.0, &mut e);
        // amounts unchanged
        assert!((e[0].amount - 5.0).abs() < 0.001);
        assert!((e[1].amount - 5.0).abs() < 0.001);
    }

    #[test]
    fn distribute_by_shares_2_to_1() {
        let mut e = make_entries(&[true, true], &[0.0, 0.0], &[2.0, 1.0]);
        distribute_by_shares(9.0, &mut e);
        assert!((e[0].amount - 6.0).abs() < 0.001);
        assert!((e[1].amount - 3.0).abs() < 0.001);
    }

    #[test]
    fn distribute_by_shares_rounding_invariant() {
        let mut e = make_entries(&[true, true, true], &[0.0; 3], &[1.0, 1.0, 1.0]);
        distribute_by_shares(10.0, &mut e);
        assert!((total_amounts(&e) - 10.0).abs() < 0.001);
    }

    #[test]
    fn distribute_by_shares_zero_shares_is_noop() {
        let mut e = make_entries(&[true, true], &[5.0, 5.0], &[0.0, 0.0]);
        distribute_by_shares(10.0, &mut e);
        // no change — total_shares == 0
        assert!((e[0].amount - 5.0).abs() < 0.001);
    }

    #[test]
    fn distribute_by_shares_fractional_075() {
        // shares: [0.75, 0.25] on 10.0 → [7.50, 2.50]
        let mut e = make_entries(&[true, true], &[0.0, 0.0], &[0.75, 0.25]);
        distribute_by_shares(10.0, &mut e);
        assert!((e[0].amount - 7.50).abs() < 0.001);
        assert!((e[1].amount - 2.50).abs() < 0.001);
    }

    #[test]
    fn distribute_by_shares_fractional_rounding_invariant() {
        // [1.5, 1.0, 0.5] on 10.0 — the total must survive
        let mut e = make_entries(&[true, true, true], &[0.0; 3], &[1.5, 1.0, 0.5]);
        distribute_by_shares(10.0, &mut e);
        assert!((total_amounts(&e) - 10.0).abs() < 0.001);
        // 1.5/3 * 10 = 5.0
        assert!((e[0].amount - 5.0).abs() < 0.001);
    }

    #[test]
    fn distribute_by_shares_zero_float_is_noop() {
        let mut e = make_entries(&[true, true], &[5.0, 5.0], &[0.0, 0.0]);
        distribute_by_shares(10.0, &mut e);
        assert!((e[0].amount - 5.0).abs() < 0.001);
    }

    #[test]
    fn parse_amount_rejects_empty_and_garbage() {
        // None means "leave everything untouched" — a mid-typing value must not
        // rewrite the field and clobber the keystroke.
        assert_eq!(parse_amount(""), None);
        assert_eq!(parse_amount("   "), None);
        assert_eq!(parse_amount("abc"), None);
        assert_eq!(parse_amount("-5"), None);
    }

    #[test]
    fn parse_amount_accepts_decimal_comma() {
        // FR mobile keyboards emit a comma — this is what type="number" silently dropped.
        assert_eq!(parse_amount("10,5"), Some(10.5));
        assert_eq!(parse_amount("12,50"), Some(12.50));
    }

    #[test]
    fn parse_amount_accepts_plain_values() {
        assert_eq!(parse_amount("10"), Some(10.0));
        assert_eq!(parse_amount("10.5"), Some(10.5));
        assert_eq!(parse_amount("0"), Some(0.0));
        assert_eq!(parse_amount(" 7 "), Some(7.0));
    }

    #[test]
    fn parse_amount_evaluates_one_operator_between_two_numbers() {
        assert_eq!(parse_amount("250/3"), Some(83.33));
        assert_eq!(parse_amount("250 / 3"), Some(83.33));
        assert_eq!(parse_amount("12,5*2"), Some(25.0));
        assert_eq!(parse_amount("10 + 2.5"), Some(12.5));
        assert_eq!(parse_amount("0/5"), Some(0.0));
        assert_eq!(parse_amount("5-2"), Some(3.0));
        assert_eq!(parse_amount("10,5-0,5"), Some(10.0));
        assert_eq!(parse_amount("5-5"), Some(0.0));
    }

    /// A half-typed expression is `None` for the same reason "12." is kept: the handler must not
    /// touch the split, and the field must keep what was typed.
    #[test]
    fn parse_amount_rejects_incomplete_or_ambiguous_expressions() {
        assert_eq!(parse_amount("250/"), None);
        assert_eq!(parse_amount("/3"), None);
        assert_eq!(parse_amount("1+2+3"), None);
        assert_eq!(parse_amount("250/0"), None);
        assert_eq!(parse_amount("2*-3"), None);
        assert_eq!(parse_amount("abc/2"), None);
        assert_eq!(parse_amount("2-5"), None);
        assert_eq!(parse_amount("5-"), None);
        assert_eq!(parse_amount("-"), None);
    }

    #[test]
    fn is_expression_only_sees_the_four_operators() {
        assert!(is_expression("250/3"));
        assert!(is_expression("2*"));
        assert!(is_expression("-5"));
        assert!(!is_expression("12,50"));
    }

    #[test]
    fn with_operator_appends_to_a_plain_number() {
        assert_eq!(with_operator("250", '/'), Some("250/".into()));
        assert_eq!(with_operator("12,5", '-'), Some("12,5-".into()));
    }

    #[test]
    fn with_operator_swaps_a_trailing_operator() {
        assert_eq!(with_operator("250/", '*'), Some("250*".into()));
    }

    #[test]
    fn with_operator_refuses_without_a_left_operand_or_after_a_complete_expression() {
        assert_eq!(with_operator("", '+'), None);
        assert_eq!(with_operator("-", '+'), None);
        assert_eq!(with_operator("250/3", '+'), None);
    }

    #[test]
    fn amount_field_text_keeps_an_expression_that_evaluates_to_the_value() {
        assert_eq!(amount_field_text(Some("250/3"), 83.33), "250/3");
        assert_eq!(amount_field_text(Some("250/"), 83.33), "250/");
        assert_eq!(amount_field_text(None, 83.33), "83.33");
    }

    #[test]
    fn amount_field_text_keeps_a_draft_that_still_means_the_value() {
        assert_eq!(amount_field_text(Some("12."), 12.0), "12.");
        assert_eq!(amount_field_text(Some("12,"), 12.0), "12,");
        assert_eq!(amount_field_text(Some("12,0"), 12.0), "12,0");
        assert_eq!(amount_field_text(Some("12,50"), 12.5), "12,50");
    }

    #[test]
    fn amount_field_text_keeps_a_draft_that_does_not_parse() {
        assert_eq!(amount_field_text(Some(""), 12.0), "");
        assert_eq!(amount_field_text(Some("-"), 12.0), "-");
    }

    #[test]
    fn amount_field_text_shows_a_value_changed_from_outside() {
        assert_eq!(amount_field_text(None, 12.5), "12.5");
        assert_eq!(amount_field_text(Some("7."), 15.0), "15");
    }

    #[test]
    fn amount_field_text_shows_an_untouched_zero() {
        assert_eq!(amount_field_text(None, 0.0), "0");
        assert_eq!(amount_field_text(Some(""), 0.0), "");
    }

    #[test]
    fn keeps_draft_on_blur_only_for_a_plain_number() {
        assert!(keeps_draft_on_blur("12,50"));
        assert!(keeps_draft_on_blur("12."));
        assert!(!keeps_draft_on_blur(""));
        assert!(!keeps_draft_on_blur("-"));
        assert!(!keeps_draft_on_blur("250/3"));
    }

    #[test]
    fn redistribute_splits_equally_when_not_in_share_mode() {
        let mut e = make_entries(&[true, true], &[0.0, 0.0], &[2.0, 1.0]);
        redistribute(10.0, &mut e, false);
        assert!((e[0].amount - 5.0).abs() < 0.001);
        assert!((e[1].amount - 5.0).abs() < 0.001);
    }

    #[test]
    fn redistribute_uses_shares_in_share_mode() {
        let mut e = make_entries(&[true, true], &[0.0, 0.0], &[2.0, 1.0]);
        redistribute(9.0, &mut e, true);
        assert!((e[0].amount - 6.0).abs() < 0.001);
        assert!((e[1].amount - 3.0).abs() < 0.001);
    }

    #[test]
    fn redistribute_with_nothing_checked_is_noop() {
        let mut e = make_entries(&[false, false], &[5.0, 5.0], &[1.0, 1.0]);
        redistribute(10.0, &mut e, false);
        redistribute(10.0, &mut e, true);
        assert!((e[0].amount - 5.0).abs() < 0.001);
        assert!((e[1].amount - 5.0).abs() < 0.001);
    }

    #[test]
    fn typing_the_total_keeps_the_split_consistent_at_every_keystroke() {
        let mut e = make_entries(&[true, true, true], &[0.0; 3], &[1.0; 3]);
        for keystroke in ["1", "10", "100"] {
            let t = parse_amount(keystroke).unwrap();
            redistribute(t, &mut e, false);
            assert!((total_amounts(&e) - t).abs() < 0.001, "failed after typing {keystroke}");
        }
    }

    #[test]
    fn typing_the_total_in_share_mode_follows_every_keystroke() {
        let mut e = make_entries(&[true, true], &[0.0, 0.0], &[3.0, 1.0]);
        for keystroke in ["4", "40", "400"] {
            let t = parse_amount(keystroke).unwrap();
            redistribute(t, &mut e, true);
            assert!((total_amounts(&e) - t).abs() < 0.001, "failed after typing {keystroke}");
            assert!((e[0].amount - t * 0.75).abs() < 0.001);
        }
    }

    #[test]
    fn typing_a_leading_zero_unchecks_then_rechecks() {
        // The intermediate "0" unchecks the row; the last keystroke checks it back.
        let mut e = make_entries(&[true], &[0.0], &[1.0]);
        let states: Vec<(f64, bool)> = ["0", "0.", "0.5"]
            .iter()
            .map(|k| {
                let v = parse_amount(k).unwrap();
                e[0].amount = v;
                e[0].checked = v > 0.0;
                (e[0].amount, e[0].checked)
            })
            .collect();
        assert_eq!(states[0], (0.0, false));
        assert_eq!(states[1], (0.0, false), "\"0.\" parses to 0.0 — same as \"0\"");
        assert_eq!(states[2], (0.5, true));
    }

    fn named(names: &[&str], checked: &[bool]) -> Vec<UserEntry> {
        names
            .iter()
            .zip(checked)
            .map(|(n, &c)| UserEntry {
                user: User { id: 0, payload: EncryptedPair::default(), ..Default::default() },
                display_name: n.to_string(),
                checked: c,
                amount: 0.0,
                shares: 0.0,
            })
            .collect()
    }

    #[test]
    fn summary_names_the_only_participant() {
        assert_eq!(
            participants_summary(&named(&["Alice", "Bob"], &[true, false])),
            ParticipantsSummary::One("Alice".into())
        );
    }

    #[test]
    fn summary_calls_out_everyone() {
        assert_eq!(
            participants_summary(&named(&["Alice", "Bob", "Cid"], &[true, true, true])),
            ParticipantsSummary::Everyone(3)
        );
    }

    #[test]
    fn summary_counts_a_partial_selection() {
        assert_eq!(
            participants_summary(&named(&["Alice", "Bob", "Cid"], &[true, true, false])),
            ParticipantsSummary::Some { checked: 2, total: 3 }
        );
    }

    #[test]
    fn summary_handles_an_empty_side() {
        assert_eq!(
            participants_summary(&named(&["Alice", "Bob"], &[false, false])),
            ParticipantsSummary::None
        );
        // A single participant, checked, is "them" — not "Everyone (1)".
        assert_eq!(
            participants_summary(&named(&["Alice"], &[true])),
            ParticipantsSummary::One("Alice".into())
        );
    }

    #[test]
    fn emoji_food_categories() {
        assert_eq!(get_expense_emoji("restaurant"), "🍽️");
        assert_eq!(get_expense_emoji("coffee"), "☕");
        assert_eq!(get_expense_emoji("starbucks"), "☕");
        assert_eq!(get_expense_emoji("pizza"), "🍕");
        assert_eq!(get_expense_emoji("burger"), "🍔");
        assert_eq!(get_expense_emoji("sushi"), "🍣");
        assert_eq!(get_expense_emoji("beer"), "🍺");
        assert_eq!(get_expense_emoji("grocery"), "🛒");
    }

    #[test]
    fn emoji_transport() {
        assert_eq!(get_expense_emoji("uber"), "🚕");
        assert_eq!(get_expense_emoji("gas"), "⛽");
        assert_eq!(get_expense_emoji("train"), "🚆");
        assert_eq!(get_expense_emoji("plane"), "✈️");
        assert_eq!(get_expense_emoji("parking"), "🅿️");
    }

    #[test]
    fn emoji_case_insensitive() {
        assert_eq!(get_expense_emoji("PIZZA"), "🍕");
        assert_eq!(get_expense_emoji("Restaurant"), "🍽️");
    }

    #[test]
    fn emoji_default_fallback() {
        assert_eq!(get_expense_emoji("quelque chose"), "💵");
        assert_eq!(get_expense_emoji(""), "💵");
    }

    /// The substring matcher this replaced turned "cadeau" and "gâteau d'anniversaire" into 💧.
    #[test]
    fn emoji_on_real_titles() {
        assert_eq!(get_expense_emoji("Cadeau Marie"), "🎁");
        assert_eq!(get_expense_emoji("Gâteau d'anniversaire"), "🍦");
        assert_eq!(get_expense_emoji("Courses Carrefour"), "🛒");
        assert_eq!(get_expense_emoji("Uber (aéroport)"), "🚕");
        assert_eq!(get_expense_emoji("Resto, bar"), "🍽️");
        assert_eq!(get_expense_emoji("Essence retour"), "⛽");
        assert_eq!(get_expense_emoji("Transport en commun"), "💵");
    }

    fn make_user(id: i32) -> User {
        User {
            id,
            payload: EncryptedPair { ct: format!("user{id}"), iv: String::new() },
            ..Default::default()
        }
    }

    #[test]
    fn init_entries_selects_stored_user() {
        let users = vec![make_user(1), make_user(2), make_user(3)];
        let names = vec![String::new(); users.len()];
        let entries = init_entries(&users, &names, Some(2), 50.0, false);
        assert!(!entries[0].checked);
        assert!(entries[1].checked);
        assert!(!entries[2].checked);
        assert!((entries[1].amount - 50.0).abs() < 0.001);
        assert!((entries[1].shares - 1.0).abs() < 0.001);
        assert!((entries[0].amount - 0.0).abs() < 0.001);
    }

    /// The scanned-receipt bug: two participants, no preselected debtor, amount prefilled. Both are
    /// checked, and `init_entries` alone leaves both owing 0.00 — the form opened on
    /// "0.00 / 13.90, Reste 13.90" and split nothing, because no `oninput` ever fired.
    #[test]
    fn a_prefilled_amount_is_split_across_debtors() {
        let users = vec![make_user(1), make_user(2)];
        let names = vec![String::new(); users.len()];
        let entries = seed_entries(&users, &names, None, 13.90, true);
        assert!(entries.iter().all(|e| e.checked));
        assert_eq!(entries.iter().map(|e| e.amount).collect::<Vec<_>>(), vec![6.95, 6.95]);
    }

    /// The payer side must be untouched by the same seed: one person paid the whole thing, and
    /// only they are checked, so an even split over the checked set still gives them all of it.
    #[test]
    fn a_prefilled_amount_stays_whole_on_the_paying_side() {
        let users = vec![make_user(1), make_user(2)];
        let names = vec![String::new(); users.len()];
        let entries = seed_entries(&users, &names, Some(1), 13.90, false);
        assert_eq!(entries.iter().map(|e| e.amount).collect::<Vec<_>>(), vec![13.90, 0.0]);
    }

    /// An odd total must still reconcile to the cent — `distribute` hands the remainder out rather
    /// than leaving "Reste 0.01" on a scanned receipt.
    #[test]
    fn a_prefilled_odd_amount_loses_no_cent() {
        let users = vec![make_user(1), make_user(2), make_user(3)];
        let names = vec![String::new(); users.len()];
        let entries = seed_entries(&users, &names, None, 10.0, true);
        let amounts: Vec<f64> = entries.iter().map(|e| e.amount).collect();
        let sum: f64 = amounts.iter().sum();
        assert!((sum - 10.0).abs() < 0.0001, "split must reconcile: {amounts:?}");
    }

    /// Opening the modal from the plain FAB carries no amount, and must not mark anyone as owing.
    #[test]
    fn no_prefilled_amount_leaves_everyone_at_zero() {
        let users = vec![make_user(1), make_user(2)];
        let names = vec![String::new(); users.len()];
        let entries = seed_entries(&users, &names, None, 0.0, true);
        assert!(entries.iter().all(|e| e.checked));
        assert!(entries.iter().all(|e| e.amount == 0.0));
    }

    #[test]
    fn init_entries_no_default_all_unchecked() {
        let users = vec![make_user(1), make_user(2)];
        let names = vec![String::new(); users.len()];
        let entries = init_entries(&users, &names, None, 0.0, false);
        assert!(entries.iter().all(|e| !e.checked));
        assert!(entries.iter().all(|e| e.amount == 0.0));
        assert!(entries.iter().all(|e| e.shares == 0.0));
    }

    #[test]
    fn init_entries_unknown_id_all_unchecked() {
        let users = vec![make_user(1), make_user(2)];
        let names = vec![String::new(); users.len()];
        let entries = init_entries(&users, &names, Some(99), 0.0, false);
        assert!(entries.iter().all(|e| !e.checked));
    }

    #[test]
    fn init_entries_check_rest_checks_everyone_when_nothing_preselected() {
        // The debtors default: no preselection means "split with everyone".
        let users = vec![make_user(1), make_user(2)];
        let names = vec![String::new(); users.len()];
        let entries = init_entries(&users, &names, None, 0.0, true);
        assert!(entries.iter().all(|e| e.checked));
        assert!(entries.iter().all(|e| (e.shares - 1.0).abs() < 0.001));
    }

    #[test]
    fn init_entries_check_rest_ignored_when_one_is_preselected() {
        let users = vec![make_user(1), make_user(2)];
        let names = vec![String::new(); users.len()];
        let entries = init_entries(&users, &names, Some(2), 40.0, true);
        assert!(!entries[0].checked);
        assert!(entries[1].checked);
        assert!((entries[1].amount - 40.0).abs() < 0.001);
    }

    #[test]
    fn init_entries_from_payments_splits_by_side() {
        use crate::decrypted::DecryptedPayment;
        let payment = |user_id, is_debt, amount| DecryptedPayment {
            id: 0,
            expense_id: 0,
            user_id,
            is_debt,
            amount,
            created_at: chrono::NaiveDateTime::default(),
        };
        let users = vec![make_user(1), make_user(2)];
        let names = vec![String::new(); users.len()];
        let payments = vec![payment(1, false, 30.0), payment(2, true, 30.0)];

        let payers = init_entries_from_payments(&users, &names, &payments, false);
        assert!(payers[0].checked && (payers[0].amount - 30.0).abs() < 0.001);
        assert!(!payers[1].checked);

        let debtors = init_entries_from_payments(&users, &names, &payments, true);
        assert!(!debtors[0].checked);
        assert!(debtors[1].checked && (debtors[1].amount - 30.0).abs() < 0.001);
    }

    #[test]
    fn init_entries_uses_display_names() {
        let users = vec![make_user(1), make_user(2)];
        let names = vec!["Alice".to_string(), "Bob".to_string()];
        let entries = init_entries(&users, &names, None, 0.0, false);
        assert_eq!(entries[0].display_name, "Alice");
        assert_eq!(entries[1].display_name, "Bob");
    }

    #[test]
    fn init_entries_missing_name_defaults_to_empty() {
        let users = vec![make_user(1), make_user(2)];
        let names: Vec<String> = vec!["Only".to_string()]; // shorter than users
        let entries = init_entries(&users, &names, None, 0.0, false);
        assert_eq!(entries[0].display_name, "Only");
        assert_eq!(entries[1].display_name, ""); // unwrap_or_default
    }
}
