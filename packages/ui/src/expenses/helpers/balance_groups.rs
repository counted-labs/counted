use std::cmp::Ordering;
use std::collections::HashMap;

use shared::User;

/// Below half a cent a balance prints as 0.00, so it is float residue, not a debt.
const SETTLED_BELOW: f64 = 0.005;

#[derive(Clone, PartialEq, Debug)]
pub struct BalanceRow {
    pub user: User,
    pub balance: f64,
    /// Percent of the largest balance on either side, so bars compare across both groups.
    pub bar_percent: f64,
}

#[derive(Clone, PartialEq, Debug, Default)]
pub struct BalanceGroups {
    pub gets_back: Vec<BalanceRow>,
    pub owes: Vec<BalanceRow>,
    pub settled: Vec<User>,
    pub gets_back_total: f64,
    pub owes_total: f64,
}

impl BalanceGroups {
    pub fn all_settled(&self) -> bool {
        self.gets_back.is_empty() && self.owes.is_empty()
    }
}

pub fn balance_groups(users: &[User], summary: &HashMap<i32, f64>) -> BalanceGroups {
    let balances: Vec<(&User, f64)> =
        users.iter().map(|u| (u, summary.get(&u.id).copied().unwrap_or(0.0))).collect();
    let max_abs = balances.iter().map(|(_, b)| b.abs()).fold(0.0_f64, f64::max);
    let row = |(user, balance): &(&User, f64)| BalanceRow {
        user: (*user).clone(),
        balance: *balance,
        bar_percent: if max_abs > 0.0 { balance.abs() / max_abs * 100.0 } else { 0.0 },
    };
    let largest_first = |a: &BalanceRow, b: &BalanceRow| {
        b.balance.abs().partial_cmp(&a.balance.abs()).unwrap_or(Ordering::Equal).then(a.user.id.cmp(&b.user.id))
    };

    let mut gets_back: Vec<BalanceRow> =
        balances.iter().filter(|(_, b)| *b >= SETTLED_BELOW).map(row).collect();
    let mut owes: Vec<BalanceRow> =
        balances.iter().filter(|(_, b)| *b <= -SETTLED_BELOW).map(row).collect();
    gets_back.sort_by(largest_first);
    owes.sort_by(largest_first);
    let settled =
        balances.iter().filter(|(_, b)| b.abs() < SETTLED_BELOW).map(|(u, _)| (*u).clone()).collect();

    BalanceGroups {
        gets_back_total: gets_back.iter().map(|r| r.balance).sum(),
        owes_total: owes.iter().map(|r| -r.balance).sum(),
        gets_back,
        owes,
        settled,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn user(id: i32) -> User {
        User { id, ..Default::default() }
    }

    fn ids(rows: &[BalanceRow]) -> Vec<i32> {
        rows.iter().map(|r| r.user.id).collect()
    }

    fn groups(entries: &[(i32, f64)]) -> BalanceGroups {
        let users: Vec<User> = entries.iter().map(|(id, _)| user(*id)).collect();
        balance_groups(&users, &entries.iter().copied().collect())
    }

    #[test]
    fn splits_creditors_debtors_and_settled_largest_first() {
        let g = groups(&[(1, 86.40), (2, 42.10), (3, 0.0), (4, -23.75), (5, -104.75)]);
        assert_eq!(ids(&g.gets_back), vec![1, 2]);
        assert_eq!(ids(&g.owes), vec![5, 4]);
        assert_eq!(g.settled.iter().map(|u| u.id).collect::<Vec<_>>(), vec![3]);
    }

    #[test]
    fn group_totals_are_positive_and_match() {
        let g = groups(&[(1, 86.40), (2, 42.10), (3, -23.75), (4, -104.75)]);
        assert!((g.gets_back_total - 128.50).abs() < 1e-9);
        assert!((g.owes_total - 128.50).abs() < 1e-9);
    }

    #[test]
    fn bars_scale_to_the_largest_balance_on_either_side() {
        let g = groups(&[(1, 50.0), (2, -100.0), (3, 50.0)]);
        assert_eq!(g.owes[0].bar_percent, 100.0);
        assert_eq!(g.gets_back[0].bar_percent, 50.0);
    }

    #[test]
    fn float_residue_counts_as_settled() {
        let g = groups(&[(1, 0.004), (2, -0.004), (3, 0.005), (4, -0.005)]);
        assert_eq!(g.settled.iter().map(|u| u.id).collect::<Vec<_>>(), vec![1, 2]);
        assert_eq!(ids(&g.gets_back), vec![3]);
        assert_eq!(ids(&g.owes), vec![4]);
    }

    #[test]
    fn a_user_missing_from_the_summary_is_settled() {
        let g = balance_groups(&[user(1)], &HashMap::new());
        assert!(g.all_settled());
        assert_eq!(g.settled.len(), 1);
    }

    #[test]
    fn equal_balances_keep_a_stable_order() {
        let g = groups(&[(7, -10.0), (3, -10.0), (5, -10.0)]);
        assert_eq!(ids(&g.owes), vec![3, 5, 7]);
    }

}
