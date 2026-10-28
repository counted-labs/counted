use crate::{round_currency, ReimbursementSuggestion, UserBalance, UserBalanceComputation};
use std::cmp::Ordering;
use std::collections::BTreeMap;

pub fn get_reimbursement_suggestions(
    mut balances: Vec<UserBalance>,
) -> Vec<ReimbursementSuggestion> {
    let mut result: Vec<ReimbursementSuggestion> = Vec::new();

    if balances.is_empty() {
        return result;
    }

    balances.sort_by(|a, b| f64::total_cmp(&b.amount.abs(), &a.amount.abs()));

    let (mut unsolved_positive_balances_by_user, mut unsolved_negative_balances_by_user) =
        get_unresolved_balances_by_user(&balances);

    let equally_opposed = resolve_equally_opposed_balances(
        &mut unsolved_positive_balances_by_user,
        &mut unsolved_negative_balances_by_user,
    );
    result.extend(equally_opposed);

    let remaining = resolve_remaining_balances(
        &mut unsolved_positive_balances_by_user,
        &mut unsolved_negative_balances_by_user,
    );
    result.extend(remaining);

    result.retain(|s| s.amount != 0.0);
    result
}

fn resolve_remaining_balances(
    unsolved_positive_balances_by_user: &mut BTreeMap<i32, UserBalanceComputation>,
    unsolved_negative_balances_by_user: &mut BTreeMap<i32, UserBalanceComputation>,
) -> Vec<ReimbursementSuggestion> {
    let mut result: Vec<ReimbursementSuggestion> = Vec::new();

    while let Some(MaxBalance { is_debt, user_id: max_user_id, remaining_amount }) =
        get_max_balance(unsolved_positive_balances_by_user, unsolved_negative_balances_by_user)
    {
        let previous_lengths =
            unsolved_positive_balances_by_user.len() + unsolved_negative_balances_by_user.len();

        let (own_balances, opposite_balances) = if is_debt {
            (&mut *unsolved_negative_balances_by_user, &mut *unsolved_positive_balances_by_user)
        } else {
            (&mut *unsolved_positive_balances_by_user, &mut *unsolved_negative_balances_by_user)
        };
        let suggestion = |other_user_id: i32, amount: f64| {
            let (user_id_debtor, user_id_payer) =
                if is_debt { (max_user_id, other_user_id) } else { (other_user_id, max_user_id) };
            ReimbursementSuggestion { amount: round_currency(amount), user_id_debtor, user_id_payer }
        };

        let (fully_compensated, remainder) = solve_max_balance(remaining_amount, opposite_balances);

        for (user_id, remaining) in fully_compensated {
            result.push(suggestion(user_id, remaining.abs()));
            opposite_balances.remove(&user_id);
        }

        if let Some((user_id, new_remaining)) = remainder {
            if let Some(balance) = opposite_balances.get_mut(&user_id) {
                result.push(suggestion(user_id, balance.remaining_amount.abs() - new_remaining.abs()));
                balance.remaining_amount = new_remaining;
            }
        }

        own_balances.remove(&max_user_id);

        debug_assert!(
            unsolved_positive_balances_by_user.len() + unsolved_negative_balances_by_user.len()
                < previous_lengths
        );
    }

    result
}

/// The opposite balances the max one pays off whole, by user, and the one it pays off in part
/// with what that user has left.
fn solve_max_balance(
    max_remaining_amount: f64,
    opposite_balances: &BTreeMap<i32, UserBalanceComputation>,
) -> (BTreeMap<i32, f64>, Option<(i32, f64)>) {
    let mut fully_compensated = BTreeMap::new();

    let mut max_balance_amount = max_remaining_amount.abs();
    let mut sorted_opposite: Vec<(i32, f64)> =
        opposite_balances.iter().map(|(id, b)| (*id, b.remaining_amount)).collect();
    sorted_opposite
        .sort_by(|(u1, a1), (u2, a2)| f64::total_cmp(&a2.abs(), &a1.abs()).then(u1.cmp(u2)));

    for (user_id, remaining_amount) in sorted_opposite {
        if max_balance_amount.total_cmp(&0.0) != Ordering::Greater {
            break;
        }

        let min_balance_amount = remaining_amount.abs();

        match max_balance_amount.total_cmp(&min_balance_amount) {
            Ordering::Greater => {
                fully_compensated.insert(user_id, remaining_amount);
                max_balance_amount -= min_balance_amount;
            }
            Ordering::Equal => {
                fully_compensated.insert(user_id, remaining_amount);
                max_balance_amount = 0.0;
            }
            Ordering::Less => {
                let new_remaining_amount = if remaining_amount < 0.0 {
                    remaining_amount + max_balance_amount
                } else {
                    remaining_amount - max_balance_amount
                };
                return (fully_compensated, Some((user_id, round_currency(new_remaining_amount))));
            }
        }
    }

    (fully_compensated, None)
}

fn get_max_balance(
    unsolved_positive_balances_by_user: &BTreeMap<i32, UserBalanceComputation>,
    unsolved_negative_balances_by_user: &BTreeMap<i32, UserBalanceComputation>,
) -> Option<MaxBalance> {
    let pick = |balances: &BTreeMap<i32, UserBalanceComputation>| {
        balances
            .iter()
            .max_by(|(u1, b1), (u2, b2)| {
                f64::total_cmp(&b2.remaining_amount.abs(), &b1.remaining_amount.abs())
                    .then(u2.cmp(u1))
            })
            .map(|(id, b)| (*id, b.remaining_amount))
    };

    let (positive_user_id, positive_amount) = pick(unsolved_positive_balances_by_user)?;
    let (negative_user_id, negative_amount) = pick(unsolved_negative_balances_by_user)?;

    Some(if positive_amount.abs() >= negative_amount.abs() {
        MaxBalance { is_debt: false, user_id: positive_user_id, remaining_amount: positive_amount }
    } else {
        MaxBalance { is_debt: true, user_id: negative_user_id, remaining_amount: negative_amount }
    })
}

fn resolve_equally_opposed_balances(
    unsolved_positive_balances_by_user: &mut BTreeMap<i32, UserBalanceComputation>,
    unsolved_negative_balances_by_user: &mut BTreeMap<i32, UserBalanceComputation>,
) -> Vec<ReimbursementSuggestion> {
    let mut result: Vec<ReimbursementSuggestion> = Vec::new();

    let mut sorted_positive: Vec<(i32, f64)> = unsolved_positive_balances_by_user
        .iter()
        .map(|(id, b)| (*id, b.remaining_amount))
        .collect();
    sorted_positive.sort_by(|(u1, a1), (u2, a2)| f64::total_cmp(&a2.abs(), &a1.abs()).then(u1.cmp(u2)));

    for (positive_user_id, balance_amount_val) in sorted_positive {
        let matching_debtor_id = unsolved_negative_balances_by_user
            .iter()
            .find(|(_, b_amount)| {
                b_amount.remaining_amount.abs().total_cmp(&balance_amount_val) == Ordering::Equal
            })
            .map(|(id, _)| *id);

        if let Some(resolved_user_id_debtor) = matching_debtor_id {
            result.push(ReimbursementSuggestion {
                amount: round_currency(balance_amount_val),
                user_id_debtor: resolved_user_id_debtor,
                user_id_payer: positive_user_id,
            });
            unsolved_positive_balances_by_user.remove(&positive_user_id);
            unsolved_negative_balances_by_user.remove(&resolved_user_id_debtor);
        }
    }

    result
}

fn get_unresolved_balances_by_user(
    balances: &[UserBalance],
) -> (BTreeMap<i32, UserBalanceComputation>, BTreeMap<i32, UserBalanceComputation>) {
    let mut unsolved_positive: BTreeMap<i32, UserBalanceComputation> = Default::default();
    let mut unsolved_negative: BTreeMap<i32, UserBalanceComputation> = Default::default();

    for user_balance in balances.iter() {
        if user_balance.amount.is_sign_positive() {
            unsolved_positive.insert(
                user_balance.user_id,
                UserBalanceComputation {
                    remaining_amount: round_currency(user_balance.amount),
                    amount: round_currency(user_balance.amount),
                },
            );
        } else {
            unsolved_negative.insert(
                user_balance.user_id,
                UserBalanceComputation {
                    remaining_amount: round_currency(user_balance.amount),
                    amount: round_currency(user_balance.amount),
                },
            );
        }
    }
    (unsolved_positive, unsolved_negative)
}

struct MaxBalance {
    is_debt: bool,
    user_id: i32,
    remaining_amount: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_balances() {
        assert!(get_reimbursement_suggestions(vec![]).is_empty());
    }

    #[test]
    fn test_single_equally_opposed_balance() {
        let suggestions = get_reimbursement_suggestions(vec![
            UserBalance { amount: 50.0, user_id: 1 },
            UserBalance { amount: -50.0, user_id: 2 },
        ]);
        assert_eq!(suggestions.len(), 1);
        assert_eq!(suggestions[0].amount, 50.0);
        assert_eq!(suggestions[0].user_id_payer, 1);
        assert_eq!(suggestions[0].user_id_debtor, 2);
    }

    // Every device must suggest the same transfers. Fresh HashMaps get a fresh RandomState, so
    // an order-dependent pick shows up across repeated runs.
    #[test]
    fn test_exact_match_pairing_is_deterministic() {
        let run = || {
            get_reimbursement_suggestions(vec![
                UserBalance { amount: 20.0, user_id: 1 },
                UserBalance { amount: -20.0, user_id: 2 },
                UserBalance { amount: -20.0, user_id: 3 },
                UserBalance { amount: -20.0, user_id: 4 },
                UserBalance { amount: 40.0, user_id: 5 },
            ])
        };
        let first = run();
        for _ in 0..200 {
            assert_eq!(run(), first);
        }
    }

    #[test]
    fn test_multiple_equally_opposed_balances() {
        let suggestions = get_reimbursement_suggestions(vec![
            UserBalance { amount: 50.0, user_id: 1 },
            UserBalance { amount: -50.0, user_id: 2 },
            UserBalance { amount: 30.0, user_id: 3 },
            UserBalance { amount: -30.0, user_id: 4 },
        ]);
        assert_eq!(suggestions.len(), 2);
        let total: f64 = suggestions.iter().map(|s| s.amount).sum();
        assert_eq!(total, 80.0);
    }

    #[test]
    fn test_one_person_owes_multiple() {
        let suggestions = get_reimbursement_suggestions(vec![
            UserBalance { amount: 30.0, user_id: 1 },
            UserBalance { amount: 20.0, user_id: 2 },
            UserBalance { amount: -50.0, user_id: 3 },
        ]);
        assert_eq!(suggestions.len(), 2);
        let charlie_payments: Vec<_> =
            suggestions.iter().filter(|s| s.user_id_debtor == 3).collect();
        assert_eq!(charlie_payments.len(), 2);
        let total: f64 = charlie_payments.iter().map(|s| s.amount).sum();
        assert_eq!(total, 50.0);
    }

    #[test]
    fn test_multiple_people_owe_one() {
        let suggestions = get_reimbursement_suggestions(vec![
            UserBalance { amount: 50.0, user_id: 1 },
            UserBalance { amount: -30.0, user_id: 2 },
            UserBalance { amount: -20.0, user_id: 3 },
        ]);
        assert_eq!(suggestions.len(), 2);
        let alice_receives: Vec<_> = suggestions.iter().filter(|s| s.user_id_payer == 1).collect();
        assert_eq!(alice_receives.len(), 2);
        let total: f64 = alice_receives.iter().map(|s| s.amount).sum();
        assert_eq!(total, 50.0);
    }

    #[test]
    fn test_complex_scenario() {
        let suggestions = get_reimbursement_suggestions(vec![
            UserBalance { amount: 100.0, user_id: 1 },
            UserBalance { amount: 50.0, user_id: 2 },
            UserBalance { amount: -60.0, user_id: 3 },
            UserBalance { amount: -90.0, user_id: 4 },
        ]);
        let total: f64 = suggestions.iter().map(|s| s.amount).sum();
        assert!((total - 150.0).abs() < 0.01);
        for s in &suggestions {
            assert!(s.amount > 0.0);
        }
    }

    #[test]
    fn test_partial_compensation() {
        let suggestions = get_reimbursement_suggestions(vec![
            UserBalance { amount: 100.0, user_id: 1 },
            UserBalance { amount: -25.0, user_id: 2 },
            UserBalance { amount: -25.0, user_id: 3 },
            UserBalance { amount: -50.0, user_id: 4 },
        ]);
        assert_eq!(suggestions.len(), 3);
        let alice_receives: Vec<_> = suggestions.iter().filter(|s| s.user_id_payer == 1).collect();
        assert_eq!(alice_receives.len(), 3);
        let total: f64 = alice_receives.iter().map(|s| s.amount).sum();
        assert_eq!(total, 100.0);
    }

    #[test]
    fn test_balanced_group() {
        let suggestions = get_reimbursement_suggestions(vec![
            UserBalance { amount: 0.0, user_id: 1 },
            UserBalance { amount: 0.0, user_id: 2 },
        ]);
        assert!(suggestions.is_empty() || suggestions.iter().all(|s| s.amount == 0.0));
    }

    #[test]
    fn test_three_way_split() {
        let suggestions = get_reimbursement_suggestions(vec![
            UserBalance { amount: 60.0, user_id: 1 },
            UserBalance { amount: -30.0, user_id: 2 },
            UserBalance { amount: -30.0, user_id: 3 },
        ]);
        let total: f64 = suggestions.iter().map(|s| s.amount).sum();
        assert_eq!(total, 60.0);
        assert!(suggestions.iter().all(|s| s.user_id_payer == 1));
    }

    #[test]
    fn test_small_amounts() {
        let suggestions = get_reimbursement_suggestions(vec![
            UserBalance { amount: 0.50, user_id: 1 },
            UserBalance { amount: -0.50, user_id: 2 },
        ]);
        assert_eq!(suggestions.len(), 1);
        assert_eq!(suggestions[0].amount, 0.50);
    }

    #[test]
    fn test_large_group() {
        let suggestions = get_reimbursement_suggestions(vec![
            UserBalance { amount: 100.0, user_id: 1 },
            UserBalance { amount: 50.0, user_id: 2 },
            UserBalance { amount: 25.0, user_id: 3 },
            UserBalance { amount: -40.0, user_id: 4 },
            UserBalance { amount: -60.0, user_id: 5 },
            UserBalance { amount: -75.0, user_id: 6 },
        ]);
        let total: f64 = suggestions.iter().map(|s| s.amount).sum();
        assert!((total - 175.0).abs() < 0.01);
        for s in &suggestions {
            assert!(s.amount > 0.0);
        }
    }

    #[test]
    fn round_currency_two_decimal_places() {
        assert_eq!(round_currency(3.333333333), 3.33);
        assert_eq!(round_currency(3.335), 3.34);
        assert_eq!(round_currency(42.5), 42.5);
    }

    #[test]
    fn a_user_with_id_0_pays_their_share() {
        let balances = vec![
            UserBalance { amount: -147.61, user_id: 0 },
            UserBalance { amount: -448.30, user_id: 1 },
            UserBalance { amount: 595.91, user_id: 2 },
        ];
        let suggestions = get_reimbursement_suggestions(balances);
        let paid_by_0: f64 =
            suggestions.iter().filter(|s| s.user_id_debtor == 0).map(|s| s.amount).sum();
        assert_eq!(paid_by_0, 147.61);
    }

    mod generated {
        use super::*;
        use crate::{balances_v1, to_cents};
        use proptest::prelude::*;

        fn closed_ledger(ids_from: i32, cents: Vec<i64>) -> Vec<UserBalance> {
            let closing = -cents.iter().sum::<i64>();
            cents
                .into_iter()
                .chain([closing])
                .zip(ids_from..)
                .map(|(c, user_id)| UserBalance { amount: c as f64 / 100.0, user_id })
                .collect()
        }

        fn wide_ledger(ids_from: i32) -> impl Strategy<Value = Vec<UserBalance>> {
            prop::collection::vec(-1_000_000_000i64..=1_000_000_000, 1..12)
                .prop_map(move |cents| closed_ledger(ids_from, cents))
        }

        fn tied_ledger(ids_from: i32) -> impl Strategy<Value = Vec<UserBalance>> {
            let amounts = vec![-300i64, -150, -100, -1, 0, 1, 100, 150, 300];
            prop::collection::vec(prop::sample::select(amounts), 1..12)
                .prop_map(move |cents| closed_ledger(ids_from, cents))
        }

        /// Balances summed in `f64` from cent-exact equal splits, as `summary_from_payments` does.
        fn accumulated_ledger(ids_from: i32) -> impl Strategy<Value = Vec<UserBalance>> {
            (2usize..10).prop_flat_map(move |users| {
                prop::collection::vec((0..users, 1i64..10_000_000, 1u32..(1u32 << users)), 1..200)
                    .prop_map(move |expenses| {
                        let mut totals = vec![0.0f64; users];
                        for (payer, cents, mask) in expenses {
                            let debtors: Vec<usize> =
                                (0..users).filter(|u| mask & (1 << u) != 0).collect();
                            let n = debtors.len() as i64;
                            totals[payer] += cents as f64 / 100.0;
                            for (pos, debtor) in debtors.into_iter().enumerate() {
                                let share = cents / n + i64::from((pos as i64) < cents % n);
                                totals[debtor] -= share as f64 / 100.0;
                            }
                        }
                        totals
                            .into_iter()
                            .zip(ids_from..)
                            .map(|(amount, user_id)| UserBalance { amount, user_id })
                            .collect()
                    })
            })
        }

        fn any_ledger(ids_from: i32) -> impl Strategy<Value = Vec<UserBalance>> {
            prop_oneof![wide_ledger(ids_from), tied_ledger(ids_from), accumulated_ledger(ids_from)]
        }

        fn hostile_amount() -> impl Strategy<Value = f64> {
            prop_oneof![
                Just(f64::NAN),
                Just(f64::INFINITY),
                Just(f64::NEG_INFINITY),
                Just(1e300),
                Just(-1e300),
                Just(-0.0),
                Just(0.0),
                Just(0.004),
                Just(-0.004),
                -1e6f64..1e6,
            ]
        }

        fn assert_settles(balances: &[UserBalance]) -> Result<(), TestCaseError> {
            let suggestions = get_reimbursement_suggestions(balances.to_vec());

            let non_zero = balances.iter().filter(|b| to_cents(b.amount) != 0).count();
            prop_assert!(suggestions.len() <= non_zero.saturating_sub(1));
            for s in &suggestions {
                prop_assert!(s.amount > 0.0 && round_currency(s.amount) == s.amount, "{:?}", s);
                prop_assert_ne!(s.user_id_debtor, s.user_id_payer);
            }

            let cents_where = |keep: &dyn Fn(&ReimbursementSuggestion) -> bool| -> i64 {
                suggestions.iter().filter(|s| keep(s)).map(|s| to_cents(s.amount)).sum()
            };
            for b in balances {
                let received = cents_where(&|s| s.user_id_payer == b.user_id);
                let paid = cents_where(&|s| s.user_id_debtor == b.user_id);
                prop_assert_eq!(received - paid, to_cents(b.amount), "user {}", b.user_id);
            }

            let owed: i64 = balances.iter().map(|b| to_cents(b.amount)).filter(|c| *c > 0).sum();
            prop_assert_eq!(cents_where(&|_| true), owed);
            Ok(())
        }

        proptest! {
            #![proptest_config(ProptestConfig::with_cases(2000))]

            #[test]
            fn every_user_settles_to_the_cent(balances in any_ledger(1)) {
                assert_settles(&balances)?;
            }

            #[test]
            fn every_user_settles_to_the_cent_with_ids_from_0(balances in any_ledger(0)) {
                assert_settles(&balances)?;
            }

            #[test]
            fn the_order_of_the_balances_does_not_matter(
                (balances, shuffled) in any_ledger(1).prop_flat_map(|b| (Just(b.clone()), Just(b).prop_shuffle()))
            ) {
                prop_assert_eq!(get_reimbursement_suggestions(balances), get_reimbursement_suggestions(shuffled));
            }

            #[test]
            fn same_transfers_as_the_previous_version(balances in any_ledger(1)) {
                prop_assert_eq!(
                    get_reimbursement_suggestions(balances.clone()),
                    balances_v1::get_reimbursement_suggestions(balances)
                );
            }

            #[test]
            fn same_transfers_as_the_previous_version_on_hostile_input(
                amounts in prop::collection::vec(hostile_amount(), 1..8)
            ) {
                let balances: Vec<UserBalance> =
                    amounts.into_iter().zip(1..).map(|(amount, user_id)| UserBalance { amount, user_id }).collect();
                prop_assert_eq!(
                    format!("{:?}", get_reimbursement_suggestions(balances.clone())),
                    format!("{:?}", balances_v1::get_reimbursement_suggestions(balances))
                );
            }
        }
    }
}
