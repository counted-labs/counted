use shared::RecurringExpense;
use shared::time::{Date, DateTime};
use uuid::Uuid;

use super::model::{decrypt_rules, DecryptedRule};
use super::view::{state, RuleState};
use crate::expenses::helpers::project_data::ProjectData;

/// The rules and the days from the last successful sync, tagged with their project like
/// `LiveData`. Not cached offline: an offline device neither materializes nor edits rules.
#[derive(Debug, Clone, PartialEq)]
pub struct RecurringSnapshot {
    pub project_id: Uuid,
    pub rules: Vec<RecurringExpense>,
    pub server_date: Option<Date>,
    pub member_day: Option<Date>,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct RecurringData {
    pub project_id: Option<Uuid>,
    pub rules: Vec<DecryptedRule>,
    pub server_date: Option<Date>,
    pub member_day: Option<Date>,
}

impl RecurringData {
    pub fn rule(&self, id: Uuid) -> Option<&DecryptedRule> {
        self.rules.iter().find(|r| r.id == id)
    }

    /// Only display uses the device-clock fallback: the materializer requires the server's.
    pub fn today(&self) -> Date {
        self.member_day.unwrap_or_else(crate::common::local_today)
    }
}

/// The member's calendar day, read off the server's clock so a wrong device clock writes nothing
/// early. A server too old to send its time leaves the UTC date.
pub fn member_day(
    server_time: Option<DateTime>,
    server_date: Option<Date>,
    offset_at: impl Fn(DateTime) -> i32,
) -> Option<Date> {
    server_time.map(|utc| utc.checked_add_seconds(offset_at(utc) as i64).unwrap_or(utc).date()).or(server_date)
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
        member_day: s.member_day,
    }
}

#[cfg(test)]
mod tests {

    use shared::time::{Date, DateTime};

    use super::member_day;

    fn instant(utc: &str) -> DateTime {
        let (day, time) = utc.split_once(' ').unwrap();
        let (hour, minute) = time.split_once(':').unwrap();
        Date::parse_ymd(day).unwrap().and_hms_opt(hour.parse().unwrap(), minute.parse().unwrap(), 0).unwrap()
    }

    fn day(s: &str) -> Date {
        Date::parse_ymd(s).unwrap()
    }

    fn day_at(utc: &str, offset: i32) -> Option<Date> {
        let t = instant(utc);
        member_day(Some(t), Some(t.date()), |_| offset)
    }

    #[test]
    fn the_day_is_the_members_own_on_the_servers_clock() {
        assert_eq!(day_at("2026-11-04 23:30", 3600), Some(day("2026-11-05")));
        assert_eq!(day_at("2026-11-04 23:30", 14 * 3600), Some(day("2026-11-05")));
        assert_eq!(day_at("2026-11-05 03:00", -11 * 3600), Some(day("2026-11-04")));
        assert_eq!(day_at("2026-11-05 03:00", 0), Some(day("2026-11-05")));
    }

    #[test]
    fn a_server_without_its_time_leaves_the_utc_date() {
        let east = |_| 14 * 3600;
        assert_eq!(member_day(None, Some(day("2026-11-04")), east), Some(day("2026-11-04")));
        assert_eq!(member_day(None, None, east), None);
    }
}
