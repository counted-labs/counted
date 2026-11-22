use std::fmt::Write;
use std::path::PathBuf;

use shared::time::{Date, DateTime};
use uuid::Uuid;

use crate::common::LocalStorageState;
use crate::expenses::helpers::my_spending::{first_of_month, in_month, shift_month};
use crate::recurring::model::{Ends, Freq, Rule};
use crate::recurring::schedule::{due, is_finished, last_date, nth_date, occurrence_id, resume_index, total, upcoming, weekday};
use crate::recurring::store::member_day;

pub(crate) struct Rng(u64);

impl Rng {
    pub(crate) fn new(seed: u64) -> Self {
        Rng(seed)
    }

    pub(crate) fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }

    pub(crate) fn range(&mut self, lo: i64, hi: i64) -> i64 {
        lo + (self.next() % (hi - lo + 1) as u64) as i64
    }

    pub(crate) fn uuid(&mut self) -> Uuid {
        Uuid::from_u64_pair(self.next(), self.next())
    }
}

pub(crate) fn date_text(d: Date) -> String {
    if !(-9999..=9999).contains(&d.year()) {
        return "none".into();
    }
    format!("{}/{}/{}", d.year(), d.month(), d.day())
}

pub(crate) fn dates_text(dates: impl IntoIterator<Item = Date>) -> String {
    dates.into_iter().map(date_text).collect::<Vec<_>>().join(",")
}

/// Years 1..=`max_year`, half of them around today, with the month ends overrepresented.
pub(crate) fn app_date(rng: &mut Rng, max_year: i64) -> Date {
    loop {
        let year = if rng.next().is_multiple_of(2) { rng.range(2000, 2040) } else { rng.range(1, max_year) };
        let day = if rng.next().is_multiple_of(3) { rng.range(28, 31) } else { rng.range(1, 31) };
        if let Some(d) = Date::from_ymd_opt(year as i32, rng.range(1, 12) as u32, day as u32) {
            return d;
        }
    }
}

const TODAYS: [(i32, u32, u32); 5] = [(2024, 2, 29), (2026, 1, 31), (2026, 11, 19), (2027, 3, 1), (2030, 12, 31)];

fn todays() -> Vec<Date> {
    TODAYS.iter().map(|(y, m, d)| Date::from_ymd_opt(*y, *m, *d).unwrap()).collect()
}

fn random_rule(rng: &mut Rng) -> Rule {
    let freq = [Freq::Weekly, Freq::Monthly, Freq::Yearly][rng.next() as usize % 3];
    let interval = rng.range(0, 12) as u32;
    let anchor = app_date(rng, 9600);
    let ends = match rng.next() % 4 {
        0 => Ends::Never,
        1 => Ends::After(rng.range(0, 40) as u32),
        _ => Ends::On(anchor.checked_add_days(rng.range(-30, 3000)).unwrap()),
    };
    Rule { freq, interval, anchor, ends }
}

fn section_schedule(out: &mut String, rng: &mut Rng) {
    for _ in 0..800 {
        let rule = random_rule(rng);
        let seed = rng.uuid();
        let next = rng.range(0, 6) as u32;
        let coming = upcoming(&rule, next, 24);
        let ids: Vec<String> = coming.iter().take(6).map(|d| occurrence_id(seed, *d).to_string()).collect();
        write!(
            out,
            "rule\t{}\t{seed}\t{next}\t{}\t{}\t{:?}\t{}\t{}\t{}",
            serde_json::to_string(&rule).unwrap(),
            dates_text(coming.iter().copied()),
            ids.join(","),
            total(&rule),
            last_date(&rule).map(date_text).unwrap_or_else(|| "none".into()),
            is_finished(&rule, next),
            weekday(rule.anchor).to_monday_zero_offset(),
        )
        .unwrap();
        for n in [0, 1, 11, 12, 59, 120, 600] {
            write!(out, "\t{n}:{}", nth_date(&rule, n).map(date_text).unwrap_or_else(|| "none".into())).unwrap();
        }
        for today in todays() {
            let written = due(&rule, next, today, 50);
            write!(
                out,
                "\t{}={}|{}",
                date_text(today),
                written.iter().map(|(n, d)| format!("{n}@{}", date_text(*d))).collect::<Vec<_>>().join(","),
                resume_index(&rule, next, today),
            )
            .unwrap();
        }
        out.push('\n');
    }
}

fn section_months(out: &mut String, rng: &mut Rng) {
    for _ in 0..2000 {
        let d = app_date(rng, 9990);
        let other = app_date(rng, 9990);
        let by = rng.range(-40, 40) as i32;
        writeln!(
            out,
            "month\t{}\t{}\t{by}\t{}\t{}\t{}",
            date_text(d),
            date_text(other),
            date_text(first_of_month(d)),
            date_text(shift_month(d, by)),
            in_month(d, other) || in_month(d, first_of_month(d)),
        )
        .unwrap();
    }
}

fn section_member_day(out: &mut String, rng: &mut Rng) {
    let offsets: Vec<i32> = (-48..=56).map(|q| q * 900).collect();
    for i in 0..1000u32 {
        let day = app_date(rng, 9990);
        let (hour, minute) = if i.is_multiple_of(2) { (rng.range(0, 23), rng.range(0, 59)) } else { (*[0, 11, 12, 13, 23].get(rng.range(0, 4) as usize).unwrap(), rng.range(0, 59)) };
        let utc = day.and_hms_nano_opt(hour as u32, minute as u32, rng.range(0, 59) as u32, rng.range(0, 999_999) as u32 * 1000).unwrap();
        let fallback = app_date(rng, 9990);
        let shifts: String = offsets
            .iter()
            .map(|zone| {
                let local = member_day(Some(utc), Some(fallback), |_| *zone).unwrap();
                match local.days_since(utc.date()) {
                    -1 => '-',
                    0 => '0',
                    1 => '+',
                    _ => '?',
                }
            })
            .collect();
        writeln!(
            out,
            "member-day\t{} {}:{}:{}.{:09}\t{shifts}\t{}",
            date_text(utc.date()),
            utc.hour(),
            utc.minute(),
            utc.second(),
            utc.nanosecond(),
            member_day(None, Some(fallback), |_| offsets[0]).map(date_text).unwrap_or_else(|| "none".into()),
        )
        .unwrap();
    }
    writeln!(out, "member-day\tnone\t{:?}", member_day(None, None, |_| offsets[0])).unwrap();
}

const RULE_INPUTS: &[&str] = &[
    r#"{"freq":"monthly","interval":1,"anchor":"2026-01-31","ends":{"kind":"never"}}"#,
    r#"{"freq":"weekly","interval":2,"anchor":"2026-11-19","ends":{"kind":"after","value":10}}"#,
    r#"{"freq":"yearly","interval":1,"anchor":"2024-02-29","ends":{"kind":"on","value":"2034-02-28"}}"#,
    r#"{"freq":"monthly","interval":3,"anchor":"0001-01-01","ends":{"kind":"on","value":"9999-12-31"}}"#,
    r#"{"freq":"monthly","interval":1,"anchor":"2026-1-31","ends":{"kind":"on","value":" 2026-12-31"}}"#,
    r#"{"freq":"monthly","interval":1,"anchor":"+2026-01-31","ends":{"kind":"never"}}"#,
    r#"{"freq":"monthly","interval":1,"anchor":"+10000-01-01","ends":{"kind":"never"}}"#,
    r#"{"freq":"monthly","interval":1,"anchor":"2026-01-31","ends":{"kind":"on","value":"-262143-01-01"}}"#,
    r#"{"freq":"monthly","interval":1,"anchor":"2026-01-31T00:00:00","ends":{"kind":"never"}}"#,
    r#"{"freq":"monthly","interval":1,"anchor":"2026-02-30","ends":{"kind":"never"}}"#,
    r#"{"freq":"monthly","interval":1,"anchor":"20260131","ends":{"kind":"never"}}"#,
    r#"{"freq":"monthly","interval":1,"anchor":null,"ends":{"kind":"never"}}"#,
];

fn saturated(d: Date) -> String {
    match d.year() {
        y if y < -9999 => "-9999/1/1".into(),
        y if y > 9999 => "9999/12/31".into(),
        _ => date_text(d),
    }
}

fn section_rule_payload(out: &mut String) {
    for input in RULE_INPUTS {
        let result = match serde_json::from_str::<Rule>(input) {
            Ok(rule) => {
                let end = match rule.ends {
                    Ends::On(d) => saturated(d),
                    other => format!("{other:?}"),
                };
                let canonical = !input.contains("+10000") && !input.contains("-262143");
                let bytes = if canonical { serde_json::to_string(&rule).unwrap() } else { "saturated".into() };
                format!("ok\t{}\t{end}\t{bytes}", saturated(rule.anchor))
            }
            Err(_) => "err".into(),
        };
        writeln!(out, "rule-payload\t{input}\t{result}").unwrap();
    }
}

const PAIR: &str = r#"{"ct":"Y3Q","iv":"aXY"}"#;
const ID: &str = "\"7d4b2c1e-0f3a-4b5c-9d6e-7f8091a2b3c4\"";

fn store_input(at: [&str; 5]) -> String {
    format!(
        r#"{{"projects":[{{"project_id":{ID},"user_id":2,"encryption_key":"a2V5","cached_project":{{"id":{ID},"payload":{PAIR},"status":"ongoing","createdAt":{}}},"cached_users":[{{"id":2,"payload":{PAIR},"createdAt":{}}},{{"id":3,"payload":{PAIR},"createdAt":null}}],"cached_expenses":[{{"id":4,"projectId":{ID},"createdAt":{},"payload":{PAIR}}}],"cached_payments":[{{"id":5,"expenseId":4,"userId":2,"payload":{PAIR},"createdAt":{}}}],"cached_data_version":7}}],"cached_projects_list":[{{"id":{ID},"payload":{PAIR},"status":"archived","createdAt":{}}}],"account_key":"YWNjb3VudA"}}"#,
        at[0], at[1], at[2], at[3], at[4]
    )
}

fn store_created_ats(state: &LocalStorageState) -> Vec<Option<DateTime>> {
    let mut out = vec![];
    for p in &state.projects {
        out.push(p.cached_project.as_ref().map(|d| d.created_at));
        out.extend(p.cached_users.iter().flatten().map(|u| u.created_at));
        out.extend(p.cached_expenses.iter().flatten().map(|e| Some(e.created_at)));
        out.extend(p.cached_payments.iter().flatten().map(|e| Some(e.created_at)));
    }
    out.extend(state.cached_projects_list.iter().flatten().map(|d| Some(d.created_at)));
    out
}

fn saturated_dt(t: DateTime) -> String {
    match t.date().year() {
        y if y < -9999 => "-9999/1/1 0:0:0.000000000".into(),
        y if y > 9999 => "9999/12/31 23:59:59.999999999".into(),
        _ if t.nanosecond() >= 1_000_000_000 => format!("{} {}:{}:59.999999999", date_text(t.date()), t.hour(), t.minute()),
        _ => format!("{} {}:{}:{}.{:09}", date_text(t.date()), t.hour(), t.minute(), t.second(), t.nanosecond()),
    }
}

fn section_local_store(out: &mut String) {
    let canonical = [
        "\"2026-11-10T07:08:09\"",
        "\"2026-11-11T08:08:09.5\"",
        "\"2026-11-12T09:08:09.123400\"",
        "\"2026-11-13T10:08:09.123456\"",
        "\"2026-11-14T11:08:09.123456789\"",
    ];
    let hostile = [
        "\"+10000-01-01T00:00:00\"",
        "\" 2026-11-11T08:08:09\"",
        "\"2026-11-12T23:59:60\"",
        "\"-262143-01-01T00:00:00\"",
        "\"2026-11-14T7:8:9 \"",
    ];
    for (name, at, check_bytes) in [("canonical", canonical, true), ("hostile", hostile, false)] {
        let input = store_input(at);
        let result = match serde_json::from_str::<LocalStorageState>(&input) {
            Ok(state) => {
                let ats = store_created_ats(&state)
                    .into_iter()
                    .map(|t| t.map(saturated_dt).unwrap_or_else(|| "null".into()))
                    .collect::<Vec<_>>()
                    .join(",");
                let keys = state.projects.iter().filter(|p| p.encryption_key.is_some()).count();
                let bytes = if check_bytes { serde_json::to_string(&state).unwrap() } else { "-".into() };
                format!("ok\tkeys={keys}\t{ats}\t{bytes}")
            }
            Err(e) => format!("err {e}"),
        };
        writeln!(out, "store\t{name}\t{result}").unwrap();
    }
}

pub(crate) fn app_transcript() -> String {
    let mut out = String::new();
    let mut rng = Rng::new(0x5EED_C4A0_0000_0002);
    section_schedule(&mut out, &mut rng);
    section_months(&mut out, &mut rng);
    section_member_day(&mut out, &mut rng);
    section_rule_payload(&mut out);
    section_local_store(&mut out);
    out
}

fn fixture_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests").join("fixtures").join("time").join(name)
}

/// Compares a transcript with its committed fixture, which chrono printed before it was replaced
/// (docs/plans/chrono-to-jiff.md §5.1). Frozen: a difference is a change of behaviour, never a fixture to rewrite.
pub(crate) fn check_golden(name: &str, actual: &str) {
    let path = fixture_path(name);
    let expected = std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
    let expected = expected.replace("\r\n", "\n");
    if expected == actual {
        return;
    }
    let mut report = String::new();
    let (mut shown, mut differing) = (0, 0);
    let expected_lines: Vec<&str> = expected.lines().collect();
    let actual_lines: Vec<&str> = actual.lines().collect();
    for i in 0..expected_lines.len().max(actual_lines.len()) {
        let (e, a) = (expected_lines.get(i).copied().unwrap_or("<missing>"), actual_lines.get(i).copied().unwrap_or("<missing>"));
        if e != a {
            differing += 1;
            if shown < 20 {
                shown += 1;
                writeln!(report, "line {}\n  expected {e}\n  actual   {a}", i + 1).unwrap();
            }
        }
    }
    panic!("{name}: {differing} lines differ from {}\n{report}", path.display());
}

#[test]
fn app_date_behaviour_matches_golden() {
    check_golden("app.tsv", &app_transcript());
}

#[cfg(target_os = "linux")]
fn instant_text(t: DateTime) -> String {
    format!("{} {}:{}:{}", date_text(t.date()), t.hour(), t.minute(), t.second())
}

/// The device zone as the native app reads it, one process per zone because the zone is the
/// process's `TZ`. Linux only: run by CI's `test` job (docs/plans/chrono-to-jiff.md §5.1 T1).
#[cfg(target_os = "linux")]
#[test]
#[ignore]
fn local_zone_matches_golden() {
    use crate::common::local_time::{offset_at, to_local};
    let zone = std::env::var("TZ").expect("run with TZ=<IANA zone>");
    let local_day = |utc: DateTime| date_text(to_local(utc).date());
    let mut out = String::new();
    for year in [2026, 2027] {
        for (month, day, days) in [(3, 1, 46), (9, 25, 47)] {
            let start = Date::from_ymd_opt(year, month, day).unwrap().at_midnight();
            let end = start.checked_add_days(days).unwrap();
            let mut previous = offset_at(start);
            writeln!(out, "start\t{}\t{previous}\t{}", instant_text(start), local_day(start)).unwrap();
            let mut t = start;
            while t < end {
                let next = t.checked_add_seconds(900).unwrap();
                let offset = offset_at(next);
                if offset != previous {
                    let (mut before, mut after) = (t, next);
                    while after.seconds_since(before) > 1 {
                        let mid = before.checked_add_seconds(after.seconds_since(before) / 2).unwrap();
                        if offset_at(mid) == previous {
                            before = mid;
                        } else {
                            after = mid;
                        }
                    }
                    writeln!(out, "change\t{}\t{previous}\t{offset}\t{}\t{}", instant_text(after), local_day(before), local_day(after)).unwrap();
                    previous = offset;
                }
                if next.hour() == 0 && next.minute() == 0 {
                    write!(out, "day\t{}", date_text(next.date())).unwrap();
                    for hour in [0, 9, 10, 11, 12, 13, 14, 23] {
                        let at = next.checked_add_seconds(hour * 3600 + 420).unwrap();
                        write!(out, "\t{} {}", local_day(at), to_local(at).hm()).unwrap();
                    }
                    out.push('\n');
                }
                t = next;
            }
        }
    }
    check_golden(&format!("tz-{}.tsv", zone.replace('/', "-")), &out);
}
