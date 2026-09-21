use crate::{round_currency, ReimbursementSuggestion, UserBalance, UserBalanceComputation};
use std::cmp::Ordering;
use std::collections::HashMap;
use std::ops::Sub;

pub fn get_reimbursement_suggestions(
    mut balances: Vec<UserBalance>,
) -> Vec<ReimbursementSuggestion> {
    let mut result: Vec<ReimbursementSuggestion> = Vec::new();

    if balances.is_empty() {
        return result;
    }

    balances.sort_by(|a, b| f64::total_cmp(&b.amount.abs(), &a.amount.abs()));

    let (mut unsolved_positive_balances_by_user, mut unsolved_negative_balances_by_user) =
        get_unresolved_balances_by_user(&mut balances);

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
    unsolved_positive_balances_by_user: &mut HashMap<i32, UserBalanceComputation>,
    unsolved_negative_balances_by_user: &mut HashMap<i32, UserBalanceComputation>,
) -> Vec<ReimbursementSuggestion> {
    let mut result: Vec<ReimbursementSuggestion> = Vec::new();

    while !unsolved_positive_balances_by_user.is_empty()
        && !unsolved_negative_balances_by_user.is_empty()
    {
        let previous_lengths =
            unsolved_positive_balances_by_user.len() + unsolved_negative_balances_by_user.len();

        let MaxBalance { is_debt, max_balance, opposite_balances: min_balances } = get_max_balance(
            unsolved_positive_balances_by_user.clone(),
            unsolved_negative_balances_by_user.clone(),
        );
        let (opposite_balances_used, remainder) =
            solve_max_balance(max_balance.clone(), min_balances);

        opposite_balances_used.iter().for_each(|(user_id, balance)| {
            if is_debt {
                result.push(ReimbursementSuggestion {
                    amount: round_currency(balance.remaining_amount.abs()),
                    user_id_debtor: max_balance.0,
                    user_id_payer: *user_id,
                });
                unsolved_positive_balances_by_user.remove(user_id);
            } else {
                result.push(ReimbursementSuggestion {
                    amount: round_currency(balance.remaining_amount.abs()),
                    user_id_debtor: *user_id,
                    user_id_payer: max_balance.0,
                });
                unsolved_negative_balances_by_user.remove(user_id);
            }
        });

        let has_remainder = remainder.0 != 0;
        if has_remainder {
            let original_remainder_balance = if is_debt {
                unsolved_positive_balances_by_user.get(&remainder.0).unwrap().remaining_amount.abs()
            } else {
                unsolved_negative_balances_by_user.get(&remainder.0).unwrap().remaining_amount.abs()
            };
            let amount_used = original_remainder_balance - remainder.1.remaining_amount.abs();

            if is_debt {
                result.push(ReimbursementSuggestion {
                    amount: round_currency(amount_used),
                    user_id_debtor: max_balance.0,
                    user_id_payer: remainder.0,
                });
            } else {
                result.push(ReimbursementSuggestion {
                    amount: round_currency(amount_used),
                    user_id_debtor: remainder.0,
                    user_id_payer: max_balance.0,
                });
            }

            if is_debt {
                let value = unsolved_positive_balances_by_user.get_mut(&remainder.0);
                value.unwrap().remaining_amount = remainder.1.remaining_amount;
            } else {
                let value = unsolved_negative_balances_by_user.get_mut(&remainder.0);
                value.unwrap().remaining_amount = remainder.1.remaining_amount;
            }
            if is_debt {
                unsolved_negative_balances_by_user.remove(&max_balance.0);
            } else {
                unsolved_positive_balances_by_user.remove(&max_balance.0);
            }
        } else if is_debt {
            unsolved_negative_balances_by_user.remove(&max_balance.0);
        } else {
            unsolved_positive_balances_by_user.remove(&max_balance.0);
        }

        let current_lengths =
            unsolved_positive_balances_by_user.len() + unsolved_negative_balances_by_user.len();

        if current_lengths == 0 {
            break;
        }

        if previous_lengths == current_lengths {
            break;
        }
    }

    result
}

fn solve_max_balance(
    max_balance: (i32, UserBalanceComputation),
    min_balances: HashMap<i32, UserBalanceComputation>,
) -> (HashMap<i32, UserBalanceComputation>, (i32, UserBalanceComputation)) {
    let mut fully_compensated_balances = HashMap::new();
    let mut remainder: (i32, UserBalanceComputation) =
        (0, UserBalanceComputation { remaining_amount: 0.0, amount: 0.0 });

    let mut max_balance_amount = max_balance.1.remaining_amount.abs();
    let mut sorted_min_balances: Vec<_> = min_balances.into_iter().collect();
    sorted_min_balances.sort_by(|(u1, b1), (u2, b2)| {
        f64::total_cmp(&b2.remaining_amount.abs(), &b1.remaining_amount.abs()).then(u1.cmp(u2))
    });
    for min_balance in sorted_min_balances {
        if max_balance_amount.total_cmp(&0.0) != Ordering::Greater {
            break;
        }

        let min_balance_amount = min_balance.1.remaining_amount.abs();

        if max_balance_amount.total_cmp(&min_balance_amount) == Ordering::Greater {
            fully_compensated_balances.insert(min_balance.0, min_balance.1.clone());
            max_balance_amount = max_balance_amount.sub(min_balance_amount);
            continue;
        }

        if max_balance_amount.total_cmp(&min_balance_amount) == Ordering::Equal {
            fully_compensated_balances.insert(min_balance.0, min_balance.1.clone());
            max_balance_amount = 0.0;
            continue;
        }

        if max_balance_amount.total_cmp(&min_balance_amount) == Ordering::Less {
            let new_remaining_amount = if min_balance.1.remaining_amount < 0.0 {
                min_balance.1.remaining_amount + max_balance_amount
            } else {
                min_balance.1.remaining_amount - max_balance_amount
            };

            remainder = (
                min_balance.0,
                UserBalanceComputation {
                    remaining_amount: round_currency(new_remaining_amount),
                    amount: round_currency(min_balance.1.amount),
                },
            );
            break;
        }
    }

    (fully_compensated_balances, remainder)
}

fn get_max_balance(
    sorted_unsolved_positive_balances_by_user: HashMap<i32, UserBalanceComputation>,
    sorted_unsolved_negative_balances_by_user: HashMap<i32, UserBalanceComputation>,
) -> MaxBalance {
    let positive_max: Option<(i32, UserBalanceComputation)> =
        sorted_unsolved_positive_balances_by_user
            .iter()
            .max_by(|(u1, b1), (u2, b2)| {
                f64::total_cmp(&b2.remaining_amount.abs(), &b1.remaining_amount.abs())
                    .then(u2.cmp(u1))
            })
            .map(|(a, b)| (*a, b.clone()));

    let negative_max: Option<(i32, UserBalanceComputation)> =
        sorted_unsolved_negative_balances_by_user
            .iter()
            .max_by(|(u1, b1), (u2, b2)| {
                f64::total_cmp(&b2.remaining_amount.abs(), &b1.remaining_amount.abs())
                    .then(u2.cmp(u1))
            })
            .map(|(a, b)| (*a, b.clone()));

    if positive_max.is_none() || negative_max.is_none() {
        return MaxBalance {
            is_debt: false,
            max_balance: (0, UserBalanceComputation { remaining_amount: 0.0, amount: 0.0 }),
            opposite_balances: HashMap::new(),
        };
    }

    let positive_max_value = positive_max.unwrap();
    let negative_max_value = negative_max.unwrap();

    if positive_max_value.1.remaining_amount.abs() >= negative_max_value.1.remaining_amount.abs() {
        MaxBalance {
            is_debt: false,
            max_balance: positive_max_value,
            opposite_balances: sorted_unsolved_negative_balances_by_user,
        }
    } else {
        MaxBalance {
            is_debt: true,
            max_balance: negative_max_value,
            opposite_balances: sorted_unsolved_positive_balances_by_user,
        }
    }
}

fn resolve_equally_opposed_balances(
    unsolved_positive_balances_by_user: &mut HashMap<i32, UserBalanceComputation>,
    unsolved_negative_balances_by_user: &mut HashMap<i32, UserBalanceComputation>,
) -> Vec<ReimbursementSuggestion> {
    let mut resolved_users: Vec<(i32, i32)> = Vec::new();
    // Mirrors the debtor ids in `resolved_users`. The membership test below sits two loops deep,
    // so scanning a Vec there made this O(n³) — and it runs in WASM on every balance render.
    let mut resolved_debtors: std::collections::HashSet<i32> = std::collections::HashSet::new();
    let mut result: Vec<ReimbursementSuggestion> = Vec::new();

    let sorted_positive_ids: Vec<i32> = {
        let mut entries: Vec<(i32, f64)> = unsolved_positive_balances_by_user
            .iter()
            .map(|(id, b)| (*id, b.remaining_amount))
            .collect();
        entries.sort_by(|(u1, a1), (u2, a2)| f64::total_cmp(&a2.abs(), &a1.abs()).then(u1.cmp(u2)));
        entries.into_iter().map(|(id, _)| id).collect()
    };

    for positive_user_id in sorted_positive_ids {
        let balance_amount_val =
            unsolved_positive_balances_by_user[&positive_user_id].remaining_amount;

        let matching_debtor_id = unsolved_negative_balances_by_user
            .iter()
            .filter(|(debtor_id, _)| !resolved_debtors.contains(debtor_id))
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

            unsolved_positive_balances_by_user
                .get_mut(&positive_user_id)
                .unwrap()
                .remaining_amount = 0.0;
            unsolved_negative_balances_by_user
                .get_mut(&resolved_user_id_debtor)
                .unwrap()
                .remaining_amount = 0.0;

            resolved_users.push((positive_user_id, resolved_user_id_debtor));
            resolved_debtors.insert(resolved_user_id_debtor);
        }
    }

    resolved_users.iter().for_each(|(payer_id, debtor_id)| {
        unsolved_positive_balances_by_user.remove(payer_id);
        unsolved_negative_balances_by_user.remove(debtor_id);
    });

    result
}

fn get_unresolved_balances_by_user(
    balances: &mut [UserBalance],
) -> (HashMap<i32, UserBalanceComputation>, HashMap<i32, UserBalanceComputation>) {
    let mut unsolved_positive: HashMap<i32, UserBalanceComputation> = Default::default();
    let mut unsolved_negative: HashMap<i32, UserBalanceComputation> = Default::default();

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
    max_balance: (i32, UserBalanceComputation),
    opposite_balances: HashMap<i32, UserBalanceComputation>,
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
}
