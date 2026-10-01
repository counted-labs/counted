use shared::to_cents;

use super::model::Side;
use crate::expenses::helpers::expense_form_helpers::UserEntry;

/// The amounts one occurrence writes. Weighted sides use largest remainder like
/// `distribute_by_shares`, but ties go to a different participant each occurrence, so over a year
/// of €1 450 / 3 nobody pays the extra cent every month.
pub fn side_amounts(side: &Side, total: f64, rotation: u32) -> Vec<(i32, f64)> {
    if side.exact {
        return side.entries.clone();
    }
    let weights: Vec<(i32, f64)> = side.entries.iter().copied().filter(|(_, w)| *w > 0.0).collect();
    let sum: f64 = weights.iter().map(|(_, w)| w).sum();
    if weights.is_empty() || sum <= 0.0 {
        return Vec::new();
    }
    let total_cents = to_cents(total);
    let exact: Vec<f64> = weights.iter().map(|(_, w)| w / sum * total_cents as f64).collect();
    let mut cents: Vec<i64> = exact.iter().map(|x| x.floor() as i64).collect();
    let leftover = total_cents - cents.iter().sum::<i64>();
    let n = weights.len();
    let start = rotation as usize % n;
    let mut order: Vec<usize> = (0..n).collect();
    order.sort_by(|&a, &b| {
        let ra = exact[a] - exact[a].floor();
        let rb = exact[b] - exact[b].floor();
        rb.partial_cmp(&ra)
            .unwrap_or(std::cmp::Ordering::Equal)
            .then(((a + n - start) % n).cmp(&((b + n - start) % n)))
    });
    for &i in order.iter().take(leftover.max(0) as usize) {
        cents[i] += 1;
    }
    weights.iter().zip(cents).map(|((id, _), c)| (*id, c as f64 / 100.0)).collect()
}

/// A form side as the template stores it: its shares in share mode, equal weights when the amounts
/// are an even split, the amounts themselves when they were typed by hand.
pub fn side_from_entries(entries: &[UserEntry], share_mode: bool) -> Side {
    let checked: Vec<&UserEntry> = entries.iter().filter(|e| e.checked && e.amount > 0.0).collect();
    if share_mode {
        return Side { exact: false, entries: checked.iter().map(|e| (e.user.id, e.shares)).collect() };
    }
    let cents: Vec<i64> = checked.iter().map(|e| to_cents(e.amount)).collect();
    let even = match (cents.iter().min(), cents.iter().max()) {
        (Some(lo), Some(hi)) => hi - lo <= 1,
        _ => true,
    };
    if even {
        return Side { exact: false, entries: checked.iter().map(|e| (e.user.id, 1.0)).collect() };
    }
    Side { exact: true, entries: checked.iter().map(|e| (e.user.id, e.amount)).collect() }
}

#[cfg(test)]
mod tests {
    use super::*;
    use shared::{sums_to_total, User};

    fn equal(ids: &[i32]) -> Side {
        Side { exact: false, entries: ids.iter().map(|id| (*id, 1.0)).collect() }
    }

    fn entry(id: i32, amount: f64, shares: f64) -> UserEntry {
        UserEntry {
            user: User { id, ..Default::default() },
            display_name: String::new(),
            checked: true,
            amount,
            shares,
        }
    }

    #[test]
    fn the_extra_cent_moves_to_someone_else_each_occurrence() {
        let side = equal(&[1, 2, 3]);
        let got: Vec<Vec<f64>> = (0..3)
            .map(|k| side_amounts(&side, 1450.0, k).iter().map(|(_, a)| *a).collect())
            .collect();
        assert_eq!(got[0], vec![483.34, 483.33, 483.33]);
        assert_eq!(got[1], vec![483.33, 483.34, 483.33]);
        assert_eq!(got[2], vec![483.33, 483.33, 483.34]);
    }

    #[test]
    fn every_occurrence_adds_up_to_the_total() {
        let side = Side { exact: false, entries: vec![(1, 2.0), (2, 1.0), (3, 1.0)] };
        for k in 0..12 {
            let amounts = side_amounts(&side, 64.21, k);
            assert!(sums_to_total(64.21, amounts.iter().map(|(_, a)| *a)), "{amounts:?}");
        }
    }

    #[test]
    fn exact_amounts_are_written_as_they_are() {
        let side = Side { exact: true, entries: vec![(1, 1000.0), (2, 450.0)] };
        assert_eq!(side_amounts(&side, 1450.0, 5), vec![(1, 1000.0), (2, 450.0)]);
    }

    #[test]
    fn an_even_split_is_stored_as_equal_weights() {
        let side = side_from_entries(&[entry(1, 483.34, 1.0), entry(2, 483.33, 1.0)], false);
        assert_eq!(side, equal(&[1, 2]));
    }

    #[test]
    fn a_hand_typed_split_is_stored_exactly() {
        let side = side_from_entries(&[entry(1, 1000.0, 1.0), entry(2, 450.0, 1.0)], false);
        assert_eq!(side, Side { exact: true, entries: vec![(1, 1000.0), (2, 450.0)] });
    }

    #[test]
    fn share_mode_keeps_the_shares() {
        let side = side_from_entries(&[entry(1, 966.67, 2.0), entry(2, 483.33, 1.0)], true);
        assert_eq!(side, Side { exact: false, entries: vec![(1, 2.0), (2, 1.0)] });
    }

    #[test]
    fn unchecked_participants_are_left_out() {
        let mut out = entry(3, 0.0, 0.0);
        out.checked = false;
        let side = side_from_entries(&[entry(1, 10.0, 1.0), out], false);
        assert_eq!(side.entries.len(), 1);
    }
}
