use chrono::NaiveDate;
use shared::RecurringExpense;
use uuid::Uuid;

use super::model::{decrypt_rules, DecryptedRule};
use super::view::{state, RuleState};
use crate::expenses::helpers::project_data::ProjectData;

/// The rules and the server's date from the last successful sync, tagged with their project like
/// `LiveData`. Not cached offline: an offline device neither materializes nor edits rules.
#[derive(Debug, Clone, PartialEq)]
pub struct RecurringSnapshot {
    pub project_id: Uuid,
    pub rules: Vec<RecurringExpense>,
    pub server_date: Option<NaiveDate>,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct RecurringData {
    pub project_id: Option<Uuid>,
    pub rules: Vec<DecryptedRule>,
    pub server_date: Option<NaiveDate>,
}

impl RecurringData {
    pub fn rule(&self, id: Uuid) -> Option<&DecryptedRule> {
        self.rules.iter().find(|r| r.id == id)
    }

    /// The server's date when known, else the device's. Only display uses the fallback: the
    /// materializer requires the server's.
    pub fn today(&self) -> NaiveDate {
        self.server_date.unwrap_or_else(|| chrono::Utc::now().date_naive())
    }
}

/// The rule an expense was written by, while that rule can still write more.
pub fn live_rule_of(data: &ProjectData, expense_id: i32, rules: Option<&RecurringData>) -> Option<DecryptedRule> {
    let id = data.expenses.iter().find(|r| r.expense.id == expense_id)?.expense.recurring_id?;
    let rule = rules?.rule(id)?;
    (state(&rule.payload) != RuleState::Finished).then(|| rule.clone())
}

pub fn build(key: Option<[u8; 32]>, snapshot: Option<&RecurringSnapshot>) -> RecurringData {
    let (Some(k), Some(s)) = (key, snapshot) else { return RecurringData::default() };
    RecurringData {
        project_id: Some(s.project_id),
        rules: decrypt_rules(&k, &s.rules),
        server_date: s.server_date,
    }
}
