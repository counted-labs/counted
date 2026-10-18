//! What the projects page (J1, J3, P4) shows of the money: my balance in each group, the people
//! I owe or who owe me, and the two totals. Everything comes from the ledgers Mes dépenses
//! already decrypts on the device; nothing here reaches the server.

use chrono::NaiveDate;
use shared::ProjectStatus;
use std::collections::HashMap;
use uuid::Uuid;

use crate::common::fx_cache::{cross_rate, CachedFx};
use crate::expenses::helpers::my_spending::GroupLedger;

const CENT: f64 = 0.005;

/// One group's row on the projects page.
#[derive(Clone, Debug, PartialEq)]
pub struct ProjectGlance {
    pub members: usize,
    /// Mine, positive when I am owed. `None` until I picked my participant there.
    pub balance: Option<f64>,
    pub currency: String,
    pub last_activity: Option<NaiveDate>,
}

/// One line of "Voir le détail": what one person and I owe each other in one group.
#[derive(Clone, Debug, PartialEq)]
pub struct Debt {
    pub project_id: Uuid,
    pub project_name: String,
    pub person: String,
    pub person_id: i32,
    pub amount: f64,
    pub currency: String,
    pub owed_to_me: bool,
}

/// "On vous doit" and "Vous devez", in one currency.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Totals {
    pub owed_to_me: f64,
    pub i_owe: f64,
    /// Some balance was converted, so the totals are approximate.
    pub converted: bool,
    /// Some balance had no known rate and is not in the totals.
    pub left_out: bool,
}

impl Totals {
    pub fn settled(&self) -> bool {
        self.owed_to_me < CENT && self.i_owe < CENT && !self.left_out
    }
}

fn open(ledger: &GroupLedger) -> bool {
    ledger.data.project_status != ProjectStatus::Archived
}

fn my_balance(ledger: &GroupLedger) -> Option<f64> {
    ledger.me.map(|me| ledger.data.summary.summary.get(&me).copied().unwrap_or(0.0))
}

pub fn glance(ledger: &GroupLedger) -> ProjectGlance {
    ProjectGlance {
        members: ledger.members,
        balance: my_balance(ledger),
        currency: ledger.data.currency.clone(),
        last_activity: ledger.data.expenses.iter().map(|r| r.date).max(),
    }
}

/// What I owe first, then what I am owed, largest first in each.
pub fn debts(ledgers: &[GroupLedger]) -> Vec<Debt> {
    let mut out = Vec::new();
    for ledger in ledgers.iter().filter(|l| open(l)) {
        let Some(me) = ledger.me else { continue };
        for s in &ledger.data.summary.reimbursement_suggestions {
            if s.amount < CENT {
                continue;
            }
            let (person_id, owed_to_me) = if s.user_id_debtor == me {
                (s.user_id_payer, false)
            } else if s.user_id_payer == me {
                (s.user_id_debtor, true)
            } else {
                continue;
            };
            out.push(Debt {
                project_id: ledger.project_id,
                project_name: ledger.name.clone(),
                person: ledger.data.user_names.get(&person_id).cloned().unwrap_or_else(|| "?".to_string()),
                person_id,
                amount: s.amount,
                currency: ledger.data.currency.clone(),
                owed_to_me,
            });
        }
    }
    out.sort_by(|a, b| {
        a.owed_to_me
            .cmp(&b.owed_to_me)
            .then(b.amount.partial_cmp(&a.amount).unwrap_or(std::cmp::Ordering::Equal))
    });
    out
}

/// My balances in the open groups, converted into `currency` with the InforEuro table.
pub fn totals(ledgers: &[GroupLedger], currency: &str, fx: Option<&CachedFx>) -> Totals {
    let mut totals = Totals::default();
    for ledger in ledgers.iter().filter(|l| open(l)) {
        let Some(balance) = my_balance(ledger) else { continue };
        if balance.abs() < CENT {
            continue;
        }
        let from = ledger.data.currency.as_str();
        let converted = if from == currency {
            balance
        } else {
            match fx.and_then(|t| cross_rate(t, from, currency)) {
                Some(rate) => {
                    totals.converted = true;
                    balance * rate
                }
                None => {
                    totals.left_out = true;
                    continue;
                }
            }
        };
        if converted > 0.0 {
            totals.owed_to_me += converted;
        } else {
            totals.i_owe -= converted;
        }
    }
    totals
}

/// The personal project's currency, otherwise the one most groups use.
pub fn home_currency(personal: Option<String>, ledgers: &[GroupLedger]) -> String {
    if let Some(currency) = personal {
        return currency;
    }
    let mut counts: HashMap<&str, usize> = HashMap::new();
    for ledger in ledgers.iter().filter(|l| open(l)) {
        *counts.entry(ledger.data.currency.as_str()).or_default() += 1;
    }
    counts
        .into_iter()
        .max_by(|a, b| a.1.cmp(&b.1).then(b.0.cmp(a.0)))
        .map(|(c, _)| c.to_string())
        .unwrap_or_else(|| "EUR".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::expenses::helpers::project_data::ProjectData;
    use shared::{ReimbursementSuggestion, UserSummary};

    fn ledger(n: u8, currency: &str, me: Option<i32>, balances: &[(i32, f64)], suggestions: &[(i32, i32, f64)]) -> GroupLedger {
        GroupLedger {
            project_id: Uuid::from_bytes([n; 16]),
            name: format!("Groupe {n}"),
            me,
            members: balances.len(),
            data: ProjectData {
                currency: currency.to_string(),
                user_names: [(1, "Moi".to_string()), (2, "Clara".to_string()), (3, "Malo".to_string())].into_iter().collect(),
                summary: UserSummary {
                    summary: balances.iter().copied().collect(),
                    reimbursement_suggestions: suggestions
                        .iter()
                        .map(|&(debtor, payer, amount)| ReimbursementSuggestion { user_id_debtor: debtor, user_id_payer: payer, amount })
                        .collect(),
                },
                ..ProjectData::default()
            },
        }
    }

    fn fx() -> CachedFx {
        CachedFx { rates: [("EUR".to_string(), 1.0), ("USD".to_string(), 2.0)].into_iter().collect(), ..Default::default() }
    }

    #[test]
    fn totals_split_what_i_am_owed_from_what_i_owe() {
        let ledgers = [
            ledger(1, "EUR", Some(1), &[(1, 30.0), (2, -30.0)], &[]),
            ledger(2, "EUR", Some(1), &[(1, -12.5), (3, 12.5)], &[]),
        ];
        let t = totals(&ledgers, "EUR", None);
        assert_eq!((t.owed_to_me, t.i_owe, t.converted, t.left_out), (30.0, 12.5, false, false));
    }

    #[test]
    fn another_currency_is_converted_or_left_out_without_a_rate() {
        let ledgers = [ledger(1, "USD", Some(1), &[(1, 20.0), (2, -20.0)], &[])];
        let t = totals(&ledgers, "EUR", Some(&fx()));
        assert_eq!((t.owed_to_me, t.converted), (10.0, true));
        let t = totals(&ledgers, "EUR", None);
        assert_eq!((t.owed_to_me, t.left_out), (0.0, true));
        assert!(!t.settled());
    }

    #[test]
    fn a_group_where_i_am_nobody_and_an_archived_group_do_not_count() {
        let mut archived = ledger(2, "EUR", Some(1), &[(1, 50.0), (2, -50.0)], &[(2, 1, 50.0)]);
        archived.data.project_status = ProjectStatus::Archived;
        let ledgers = [ledger(1, "EUR", None, &[(1, 30.0), (2, -30.0)], &[(2, 1, 30.0)]), archived];
        assert!(totals(&ledgers, "EUR", None).settled());
        assert!(debts(&ledgers).is_empty());
    }

    #[test]
    fn debts_name_the_other_person_and_put_mine_first() {
        let ledgers = [ledger(1, "EUR", Some(1), &[(1, 10.0), (2, -40.0), (3, 30.0)], &[(2, 3, 30.0), (2, 1, 10.0), (1, 3, 5.0)])];
        let d = debts(&ledgers);
        assert_eq!(d.len(), 2, "{d:?}");
        assert_eq!((d[0].person.as_str(), d[0].owed_to_me, d[0].amount), ("Malo", false, 5.0));
        assert_eq!((d[1].person.as_str(), d[1].owed_to_me, d[1].amount), ("Clara", true, 10.0));
    }

    #[test]
    fn a_row_shows_my_balance_or_none_before_i_picked_myself() {
        assert_eq!(glance(&ledger(1, "EUR", Some(2), &[(1, 5.0), (2, -5.0)], &[])).balance, Some(-5.0));
        assert_eq!(glance(&ledger(1, "EUR", Some(3), &[(1, 5.0), (2, -5.0)], &[])).balance, Some(0.0));
        assert_eq!(glance(&ledger(1, "EUR", None, &[], &[])).balance, None);
    }

    #[test]
    fn the_home_currency_is_the_personal_one_or_the_most_used() {
        let ledgers = [ledger(1, "USD", None, &[], &[]), ledger(2, "EUR", None, &[], &[]), ledger(3, "USD", None, &[], &[])];
        assert_eq!(home_currency(Some("CHF".to_string()), &ledgers), "CHF");
        assert_eq!(home_currency(None, &ledgers), "USD");
        assert_eq!(home_currency(None, &[]), "EUR");
    }
}
