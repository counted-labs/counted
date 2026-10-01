use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};
use shared::{ExpenseType, RecurringExpense};
use uuid::Uuid;

use crate::crypto::decrypt_json;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Freq {
    Weekly,
    Monthly,
    Yearly,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "camelCase", tag = "kind", content = "value")]
pub enum Ends {
    Never,
    On(NaiveDate),
    After(u32),
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Rule {
    pub freq: Freq,
    pub interval: u32,
    pub anchor: NaiveDate,
    pub ends: Ends,
}

/// One side of the split. `exact` amounts are written as they are; otherwise the values are
/// weights, re-split on every occurrence so the leftover cent rotates between participants.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Side {
    pub exact: bool,
    pub entries: Vec<(i32, f64)>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Template {
    pub name: String,
    pub expense_type: String,
    #[serde(default)]
    pub category: Option<String>,
    /// In the project currency, frozen when the rule is set up.
    pub amount: f64,
    pub payers: Side,
    pub debtors: Side,
    #[serde(default)]
    pub source_currency: Option<String>,
    #[serde(default)]
    pub source_amount: Option<f64>,
    #[serde(default)]
    pub rate: Option<f64>,
    /// The last rate a member saw and chose to keep; the drift warning compares against it.
    #[serde(default)]
    pub rate_ack: Option<f64>,
    /// "Amount changes each time": occurrences are written as estimates to confirm.
    #[serde(default)]
    pub variable: bool,
}

impl Template {
    pub fn expense_type(&self) -> ExpenseType {
        match self.expense_type.as_str() {
            "transfer" => ExpenseType::Transfer,
            "gain" => ExpenseType::Gain,
            _ => ExpenseType::Expense,
        }
    }

    pub fn conversion(&self) -> Option<(f64, &str, f64)> {
        let currency = self.source_currency.as_deref()?;
        let amount = self.source_amount?;
        let rate = self.rate?;
        (amount.is_finite() && shared::is_valid_rate(rate)).then_some((amount, currency, rate))
    }

    pub fn participant_ids(&self) -> Vec<i32> {
        let mut ids: Vec<i32> =
            self.payers.entries.iter().chain(&self.debtors.entries).map(|(id, _)| *id).collect();
        ids.sort_unstable();
        ids.dedup();
        ids
    }

    pub fn involves(&self, user_id: i32) -> bool {
        self.participant_ids().contains(&user_id)
    }
}

/// The whole rule, encrypted under the project key. `next` is the index of the first occurrence
/// not yet written; `seed` keys every occurrence's `client_op_id` and never leaves the ciphertext.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RecurringPayload {
    pub template: Template,
    pub rule: Rule,
    pub next: u32,
    #[serde(default)]
    pub paused: bool,
    pub seed: Uuid,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DecryptedRule {
    pub id: Uuid,
    pub version: i64,
    pub author_id: Option<i32>,
    pub created_at: NaiveDateTime,
    pub payload: RecurringPayload,
}

/// Rules whose ciphertext does not open are dropped, like expenses.
pub fn decrypt_rules(key: &[u8; 32], rules: &[RecurringExpense]) -> Vec<DecryptedRule> {
    rules
        .iter()
        .filter_map(|r| {
            let payload = decrypt_json::<RecurringPayload>(key, &r.payload).ok()?;
            Some(DecryptedRule {
                id: r.id,
                version: r.version,
                author_id: r.author_id,
                created_at: r.created_at,
                payload,
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::test_fixtures::test_key;
    use crate::crypto::encrypt_json;

    pub(crate) fn template() -> Template {
        Template {
            name: "Rent".to_string(),
            expense_type: "expense".to_string(),
            category: None,
            amount: 1450.0,
            payers: Side { exact: true, entries: vec![(1, 1450.0)] },
            debtors: Side { exact: false, entries: vec![(1, 1.0), (2, 1.0), (3, 1.0)] },
            source_currency: None,
            source_amount: None,
            rate: None,
            rate_ack: None,
            variable: false,
        }
    }

    #[test]
    fn a_rule_round_trips_through_the_ciphertext() {
        let key = test_key();
        let payload = RecurringPayload {
            template: template(),
            rule: Rule {
                freq: Freq::Monthly,
                interval: 1,
                anchor: NaiveDate::from_ymd_opt(2026, 8, 1).unwrap(),
                ends: Ends::After(12),
            },
            next: 2,
            paused: false,
            seed: Uuid::new_v4(),
        };
        let row = RecurringExpense {
            id: Uuid::new_v4(),
            project_id: Uuid::nil(),
            author_id: Some(1),
            payload: encrypt_json(&key, &payload).unwrap(),
            version: 3,
            created_at: NaiveDateTime::default(),
        };
        let rules = decrypt_rules(&key, &[row]);
        assert_eq!(rules.len(), 1);
        assert_eq!(rules[0].payload, payload);
        assert_eq!(rules[0].version, 3);
    }

    #[test]
    fn a_rule_under_another_key_is_dropped() {
        let payload = encrypt_json(&[9u8; 32], &"x").unwrap();
        let row = RecurringExpense {
            id: Uuid::new_v4(),
            project_id: Uuid::nil(),
            author_id: None,
            payload,
            version: 0,
            created_at: NaiveDateTime::default(),
        };
        assert!(decrypt_rules(&test_key(), &[row]).is_empty());
    }

    #[test]
    fn participants_are_both_sides_without_duplicates() {
        assert_eq!(template().participant_ids(), vec![1, 2, 3]);
        assert!(template().involves(2));
        assert!(!template().involves(4));
    }
}
