use std::fmt::Write;
use std::path::PathBuf;

use crate::time::{Date, DateTime};
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::{
    Account, Expense, HistoryEntry, IncomingFriendRequest, MaterializeRecurringRequest, OutgoingFriendRequest, Payment,
    ProjectDto, ProjectInvitation, ProjectSync, RecoveryStart, RecurringExpense, User,
};

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

    pub(crate) fn pick<'a, T>(&mut self, items: &'a [T]) -> &'a T {
        &items[self.next() as usize % items.len()]
    }
}

const YEARS: std::ops::RangeInclusive<i32> = -9999..=9999;

fn date_text(d: Date) -> String {
    if !YEARS.contains(&d.year()) {
        return "none".into();
    }
    format!("{}/{}/{}", d.year(), d.month(), d.day())
}

fn dt_text(t: DateTime) -> String {
    if !YEARS.contains(&t.date().year()) {
        return "none".into();
    }
    format!("{} {}:{}:{}.{:09}", date_text(t.date()), t.hour(), t.minute(), t.second(), t.nanosecond())
}

fn saturated_date(d: Date) -> String {
    match d.year() {
        y if y < *YEARS.start() => "-9999/1/1".into(),
        y if y > *YEARS.end() => "9999/12/31".into(),
        _ => date_text(d),
    }
}

fn saturated_dt(t: DateTime) -> String {
    match t.date().year() {
        y if y < *YEARS.start() => "-9999/1/1 0:0:0.000000000".into(),
        y if y > *YEARS.end() => "9999/12/31 23:59:59.999999999".into(),
        _ if t.nanosecond() >= 1_000_000_000 => {
            format!("{} {}:{}:59.999999999", date_text(t.date()), t.hour(), t.minute())
        }
        _ => dt_text(t),
    }
}

fn opt_date(d: Option<Date>) -> String {
    d.map(date_text).unwrap_or_else(|| "none".into())
}

fn opt_dt(t: Option<DateTime>) -> String {
    t.map(dt_text).unwrap_or_else(|| "none".into())
}

pub(crate) fn random_date(rng: &mut Rng) -> Date {
    loop {
        let year = if rng.next().is_multiple_of(2) { rng.range(1900, 2100) } else { rng.range(-9999, 9999) };
        if let Some(d) = Date::from_ymd_opt(year as i32, rng.range(1, 12) as u32, rng.range(1, 31) as u32) {
            return d;
        }
    }
}

pub(crate) fn random_nanos(rng: &mut Rng) -> u32 {
    match rng.next() % 4 {
        0 => 0,
        1 => rng.range(0, 999) as u32 * 1_000_000,
        2 => rng.range(0, 999_999) as u32 * 1_000,
        _ => rng.range(0, 999_999_999) as u32,
    }
}

pub(crate) fn random_dt(rng: &mut Rng) -> DateTime {
    random_date(rng)
        .and_hms_nano_opt(rng.range(0, 23) as u32, rng.range(0, 59) as u32, rng.range(0, 59) as u32, random_nanos(rng))
        .unwrap()
}

fn edge_dates() -> Vec<Date> {
    [
        (1970, 1, 1),
        (0, 1, 1),
        (1, 1, 1),
        (-1, 12, 31),
        (-9999, 1, 1),
        (9999, 12, 31),
        (999, 1, 1),
        (99, 3, 1),
        (2000, 2, 29),
        (1900, 2, 28),
        (2024, 2, 29),
        (2026, 1, 31),
        (2026, 4, 30),
        (2026, 11, 19),
        (2026, 12, 31),
    ]
    .iter()
    .map(|(y, m, d)| Date::from_ymd_opt(*y, *m, *d).unwrap())
    .collect()
}

fn edge_dts() -> Vec<DateTime> {
    let mut out = vec![];
    for d in edge_dates() {
        for (h, mi, s, ns) in
            [(0, 0, 0, 0), (23, 59, 59, 999_999_999), (7, 8, 9, 500_000_000), (7, 8, 9, 123_400_000), (12, 0, 0, 1), (12, 0, 0, 1_000), (12, 0, 0, 1_000_000)]
        {
            out.push(d.and_hms_nano_opt(h, mi, s, ns).unwrap());
        }
    }
    out
}

fn json<T: Serialize>(value: &T) -> String {
    serde_json::to_string(value).unwrap()
}

fn from_json<T: DeserializeOwned>(text: &str) -> Result<T, String> {
    serde_json::from_str(text).map_err(|_| "err".to_string())
}

fn section_date_serde(out: &mut String, rng: &mut Rng) {
    let mut dates = edge_dates();
    dates.extend((0..2000).map(|_| random_date(rng)));
    for d in dates {
        let text = json(&d);
        let back = from_json::<Date>(&text).map(date_text).unwrap_or_else(|e| e);
        writeln!(
            out,
            "date\t{}\t{text}\t{}\t{}\t{}\t{back}",
            date_text(d),
            d,
            d.ymd(),
            d.with_day(1).unwrap().ymd()
        )
        .unwrap();
    }
}

fn section_datetime_serde(out: &mut String, rng: &mut Rng) {
    let mut dts = edge_dts();
    dts.extend((0..2000).map(|_| random_dt(rng)));
    for t in dts {
        let text = json(&t);
        let back = from_json::<DateTime>(&text).map(dt_text).unwrap_or_else(|e| e);
        writeln!(
            out,
            "datetime\t{}\t{text}\t{}\t{}\t{} UTC\t{back}",
            dt_text(t),
            t,
            t.hm(),
            t.ymd_hm()
        )
        .unwrap();
    }
}

fn handwritten_inputs() -> Vec<String> {
    [
        "2024-03-05",
        "2024-3-5",
        "2024-03-5",
        " 2024-03-05",
        "2024-03-05 ",
        "\t2024-03-05",
        "2024-03-05x",
        "2024-02-29",
        "2023-02-29",
        "2024-02-30",
        "2024-13-01",
        "2024-00-10",
        "2024-01-00",
        "2024-01-32",
        "0000-01-01",
        "0001-01-01",
        "999-01-01",
        "99-01-01",
        "0999-01-01",
        "9999-12-31",
        "10000-01-01",
        "20000-01-01",
        "+2024-03-05",
        "+10000-01-01",
        "+262142-12-31",
        "+262143-01-01",
        "-0001-01-01",
        "-9999-01-01",
        "-10000-01-01",
        "-262143-01-01",
        "-262144-01-01",
        "2024/03/05",
        "2024.03.05",
        "20240305",
        "05/03/2024",
        "",
        " ",
        "2024",
        "2024-03",
        "２０２４-03-05",
        "2024-03-05T07:08:09",
        "2024-03-05T07:08:09.5",
        "2024-03-05T07:08:09.500",
        "2024-03-05T07:08:09.123400",
        "2024-03-05T07:08:09.123456789",
        "2024-03-05T07:08:09.1234567891",
        "2024-03-05T07:08:09.",
        "2024-03-05T07:08:09,5",
        "2024-03-05T07:08",
        "2024-03-05T07",
        "2024-03-05T",
        "2024-03-05 07:08:09",
        "2024-03-05t07:08:09",
        "2024-03-05T07:08:09Z",
        "2024-03-05T07:08:09+00:00",
        "2024-03-05T07:08:09+01:00",
        "2024-03-05T24:00:00",
        "2024-03-05T23:59:60",
        "2024-03-05T23:59:60.5",
        "2024-03-05T23:60:00",
        "2024-03-05T7:8:9",
        "2024-03-05T07:08:09 ",
        " 2024-03-05T07:08:09",
        "+10000-01-01T00:00:00",
        "-0001-01-01T00:00:00",
        "2024-03-05T00:00:00",
        "2026-11-04 23:30",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect()
}

pub(crate) fn generated_input(rng: &mut Rng) -> String {
    let mut s = String::new();
    s.push_str(rng.pick(&["", "", "", "", " ", "+", "-"]));
    let year = rng.range(0, 30_000);
    let width = *rng.pick(&[1usize, 2, 3, 4, 4, 4, 4, 5, 6]);
    write!(s, "{year:0width$}").unwrap();
    s.push_str(rng.pick(&["-", "-", "-", "-", "/", " "]));
    let month = rng.range(0, 13);
    if rng.next().is_multiple_of(3) { write!(s, "{month}") } else { write!(s, "{month:02}") }.unwrap();
    s.push('-');
    let day = rng.range(0, 32);
    if rng.next().is_multiple_of(3) { write!(s, "{day}") } else { write!(s, "{day:02}") }.unwrap();
    if rng.next().is_multiple_of(2) {
        s.push_str(rng.pick(&["T", "T", "T", " ", "t"]));
        write!(s, "{:02}:{:02}", rng.range(0, 24), rng.range(0, 60)).unwrap();
        if !rng.next().is_multiple_of(4) {
            write!(s, ":{:02}", rng.range(0, 60)).unwrap();
            if rng.next().is_multiple_of(2) {
                s.push('.');
                for _ in 0..rng.range(0, 10) {
                    s.push(char::from(b'0' + rng.range(0, 9) as u8));
                }
            }
        }
        s.push_str(rng.pick(&["", "", "", "", "Z", "+01:00", " "]));
    }
    s.push_str(rng.pick(&["", "", "", "", "", " ", "x"]));
    s
}

fn section_parse(out: &mut String, rng: &mut Rng) {
    let mut inputs = handwritten_inputs();
    inputs.extend((0..3000).map(|_| generated_input(rng)));
    for s in inputs {
        let as_json = json(&s);
        let date = from_json::<Date>(&as_json).map(saturated_date).unwrap_or_else(|e| e);
        let dt = from_json::<DateTime>(&as_json).map(saturated_dt).unwrap_or_else(|e| e);
        let ymd = Date::parse_ymd(&s).map(date_text).unwrap_or_else(|| "none".into());
        writeln!(out, "parse\t{as_json}\t{date}\t{dt}\t{ymd}").unwrap();
    }
    for text in ["123", "null", "true", "[2024,3,5]", "{\"year\":2024}", "\"\\u0032024-03-05\""] {
        let date = from_json::<Date>(text).map(saturated_date).unwrap_or_else(|e| e);
        let dt = from_json::<DateTime>(text).map(saturated_dt).unwrap_or_else(|e| e);
        writeln!(out, "json\t{text}\t{date}\t{dt}").unwrap();
    }
}

fn section_date_arithmetic(out: &mut String, rng: &mut Rng) {
    for i in 0..4000 {
        let d = if i < 15 { edge_dates()[i] } else { random_date(rng) };
        let other = random_date(rng);
        let n = match rng.next() % 4 {
            0 => rng.range(0, 31),
            1 => rng.range(0, 400),
            2 => rng.range(0, 20_000),
            _ => rng.range(0, 240_000),
        } as u32;
        let days = rng.range(-800, 800);
        let ce = rng.range(-4_000_000, 4_000_000) as i32;
        writeln!(
            out,
            "date-op\t{}\t{n}\t{}\t{}\t{}\t{days}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{ce}\t{}\t{:?}",
            date_text(d),
            opt_date(d.checked_add_days(n as i64)),
            opt_date(d.checked_add_months(n)),
            opt_date(d.checked_sub_months(n)),
            opt_date(d.checked_sub_days(days)),
            date_text(other),
            d.days_since(other),
            d.weekday().to_monday_zero_offset(),
            opt_date(d.with_day(1)),
            d.month0(),
            d.num_days_from_ce(),
            opt_date(Date::from_num_days_from_ce_opt(ce)),
            d.cmp(&other),
        )
        .unwrap();
    }
}

fn section_datetime_arithmetic(out: &mut String, rng: &mut Rng) {
    for _ in 0..3000 {
        let t = random_dt(rng);
        let near = match rng.next() % 3 {
            0 => t.checked_add_nanos(rng.range(-61_000_000_000, 61_000_000_000)).unwrap(),
            1 => t.checked_add_seconds(rng.range(-120, 120)).unwrap(),
            _ => random_dt(rng),
        };
        let hours = rng.range(-48, 48);
        let days = rng.range(-400, 400);
        let seconds = t.seconds_since(near);
        writeln!(
            out,
            "datetime-op\t{}\t{}\t{seconds}\t{}\t{hours}\t{}\t{days}\t{}\t{}\t{}\t{:?}",
            dt_text(t),
            dt_text(near),
            seconds.abs() < 60,
            opt_dt(t.checked_add_hours(hours)),
            opt_dt(t.checked_add_days(days)),
            date_text(t.date()),
            dt_text(t.date().at_midnight()),
            t.cmp(&near),
        )
        .unwrap();
    }
}

fn section_defaults(out: &mut String) {
    writeln!(out, "default\tdate\t{}", date_text(Date::default())).unwrap();
    writeln!(out, "default\tdatetime\t{}", dt_text(DateTime::default())).unwrap();
    writeln!(out, "default\tepoch\t{}", dt_text(DateTime::EPOCH)).unwrap();
}

pub(crate) fn library_transcript() -> String {
    let mut out = String::new();
    let mut rng = Rng::new(0x5EED_C4A0_0000_0001);
    section_date_serde(&mut out, &mut rng);
    section_datetime_serde(&mut out, &mut rng);
    section_parse(&mut out, &mut rng);
    section_date_arithmetic(&mut out, &mut rng);
    section_datetime_arithmetic(&mut out, &mut rng);
    section_defaults(&mut out);
    out
}

const PAIR: &str = r#"{"ct":"Y3Q","iv":"aXY"}"#;
const ID: &str = "\"7d4b2c1e-0f3a-4b5c-9d6e-7f8091a2b3c4\"";
const FRACTIONS: [&str; 6] = ["", ".5", ".500", ".123400", ".123456", ".123456789"];

fn wire_inputs() -> Vec<(&'static str, String)> {
    let mut out = vec![];
    for (i, f) in FRACTIONS.iter().enumerate() {
        let at = format!("\"2026-11-{:02}T{:02}:08:09{f}\"", 10 + i, 7 + i);
        let day = format!("\"2026-11-{:02}\"", 10 + i);
        out.push(("Expense", format!(r#"{{"id":4,"authorId":2,"projectId":{ID},"createdAt":{at},"payload":{PAIR}}}"#)));
        out.push(("Payment", format!(r#"{{"id":5,"expenseId":4,"userId":2,"payload":{PAIR},"createdAt":{at}}}"#)));
        out.push(("User", format!(r#"{{"id":2,"payload":{PAIR},"createdAt":{at}}}"#)));
        out.push(("User", format!(r#"{{"id":2,"payload":{PAIR},"createdAt":null}}"#)));
        out.push(("ProjectDto", format!(r#"{{"id":{ID},"payload":{PAIR},"status":"ongoing","createdAt":{at}}}"#)));
        out.push(("HistoryEntry", format!(r#"{{"id":9,"projectId":{ID},"actorUserId":2,"action":"updated","entity":"expense","entityId":4,"payload":{PAIR},"createdAt":{at}}}"#)));
        out.push(("RecurringExpense", format!(r#"{{"id":{ID},"projectId":{ID},"authorId":2,"payload":{PAIR},"version":3,"createdAt":{at}}}"#)));
        out.push(("IncomingFriendRequest", format!(r#"{{"id":{ID},"requesterEmail":"a@example.test","createdAt":{at}}}"#)));
        out.push(("OutgoingFriendRequest", format!(r#"{{"id":{ID},"label":{PAIR},"createdAt":{at}}}"#)));
        out.push(("ProjectInvitation", format!(r#"{{"id":{ID},"projectId":{ID},"fromEmail":"a@example.test","fromPublicKey":[1,2],"createdAt":{at},"userId":3}}"#)));
        out.push(("RecoveryStart", format!(r#"{{"accountId":{ID},"loginSalt":[1],"amkEpoch":0,"hasRecoveryKey":true,"pendingResetAt":{at}}}"#)));
        out.push(("RecoveryStart", format!(r#"{{"accountId":{ID},"loginSalt":[1],"amkEpoch":0,"hasRecoveryKey":false,"pendingResetAt":null}}"#)));
        out.push((
            "Account",
            format!(
                r#"{{"id":{ID},"email":"a@example.test","displayName":{PAIR},"createdAt":{at},"emailVerified":true,"kdfSalt":[1],"kdfVersion":2,"preferences":null,"paymentMethods":null,"recoveryPendingAt":{at},"pendingResetAt":{at}}}"#
            ),
        ));
        out.push((
            "MaterializeRecurringRequest",
            format!(r#"{{"id":{ID},"projectId":{ID},"expectedVersion":1,"payload":{PAIR},"dueThrough":{day},"occurrences":[]}}"#),
        ));
        out.push((
            "ProjectSync",
            format!(
                r#"{{"dataVersion":7,"project":{{"id":{ID},"payload":{PAIR},"status":"closed","createdAt":{at}}},"users":[{{"id":2,"payload":{PAIR},"createdAt":{at}}}],"expenses":[{{"id":4,"projectId":{ID},"createdAt":{at},"payload":{PAIR}}}],"payments":[],"recurring":[],"serverDate":{day},"serverTime":{at}}}"#
            ),
        ));
        out.push(("ProjectSync", r#"{"dataVersion":7,"expenses":null,"payments":null}"#.to_string()));
        out.push(("NaiveDateTime", at.clone()));
        out.push(("NaiveDate", day.clone()));
    }
    out
}

fn roundtrip<T: Serialize + DeserializeOwned>(input: &str) -> String {
    match serde_json::from_str::<T>(input) {
        Ok(value) => json(&value),
        Err(e) => format!("err {e}"),
    }
}

pub(crate) fn wire_transcript() -> String {
    let mut out = String::new();
    for (name, input) in wire_inputs() {
        let output = match name {
            "Expense" => roundtrip::<Expense>(&input),
            "Payment" => roundtrip::<Payment>(&input),
            "User" => roundtrip::<User>(&input),
            "ProjectDto" => roundtrip::<ProjectDto>(&input),
            "HistoryEntry" => roundtrip::<HistoryEntry>(&input),
            "RecurringExpense" => roundtrip::<RecurringExpense>(&input),
            "IncomingFriendRequest" => roundtrip::<IncomingFriendRequest>(&input),
            "OutgoingFriendRequest" => roundtrip::<OutgoingFriendRequest>(&input),
            "ProjectInvitation" => roundtrip::<ProjectInvitation>(&input),
            "RecoveryStart" => roundtrip::<RecoveryStart>(&input),
            "Account" => roundtrip::<Account>(&input),
            "MaterializeRecurringRequest" => roundtrip::<MaterializeRecurringRequest>(&input),
            "ProjectSync" => roundtrip::<ProjectSync>(&input),
            "NaiveDateTime" => roundtrip::<DateTime>(&input),
            "NaiveDate" => roundtrip::<Date>(&input),
            other => panic!("no wire shape {other}"),
        };
        writeln!(out, "{name}\t{input}\t{output}").unwrap();
    }
    out
}

fn fixture_path(crate_dir: &str, name: &str) -> PathBuf {
    PathBuf::from(crate_dir).join("tests").join("fixtures").join("time").join(name)
}

/// Compares a transcript with its committed fixture, which chrono printed before it was replaced
/// (docs/plans/chrono-to-jiff.md §5.1). Frozen: a difference is a change of behaviour, never a fixture to rewrite.
fn check_golden(crate_dir: &str, name: &str, actual: &str) {
    let path = fixture_path(crate_dir, name);
    let expected = std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
    let expected = expected.replace("\r\n", "\n");
    if expected == actual {
        return;
    }
    let mut report = String::new();
    let (mut shown, mut total) = (0, 0);
    let expected_lines: Vec<&str> = expected.lines().collect();
    let actual_lines: Vec<&str> = actual.lines().collect();
    for i in 0..expected_lines.len().max(actual_lines.len()) {
        let (e, a) = (expected_lines.get(i).copied().unwrap_or("<missing>"), actual_lines.get(i).copied().unwrap_or("<missing>"));
        if e != a {
            total += 1;
            if shown < 20 {
                shown += 1;
                writeln!(report, "line {}\n  expected {e}\n  actual   {a}", i + 1).unwrap();
            }
        }
    }
    panic!("{name}: {total} lines differ from {}\n{report}", path.display());
}

#[test]
fn library_behaviour_matches_golden() {
    check_golden(env!("CARGO_MANIFEST_DIR"), "library.tsv", &library_transcript());
}

#[test]
fn wire_bytes_match_golden() {
    check_golden(env!("CARGO_MANIFEST_DIR"), "wire.tsv", &wire_transcript());
}

/// What chrono 0.4.45 printed for whitespace at every place its grammar skips or refuses it, and
/// for month arithmetic up to the ends of the range (`tests/fixtures/time/edges.tsv`, generated by
/// `edges.generator.rs.txt` beside it, docs/plans/chrono-to-jiff.md §5.5).
#[test]
fn edges_match_what_chrono_printed() {
    let path = fixture_path(env!("CARGO_MANIFEST_DIR"), "edges.tsv");
    let text = std::fs::read_to_string(&path).unwrap().replace("\r\n", "\n");
    let mut differing = vec![];
    for line in text.lines() {
        let cols: Vec<&str> = line.split('\t').collect();
        let actual = match cols[0] {
            "space" => {
                let json = cols[1];
                let date = from_json::<Date>(json).map(date_text).unwrap_or_else(|e| e);
                let dt = from_json::<DateTime>(json).map(dt_text).unwrap_or_else(|e| e);
                let raw: String = serde_json::from_str(json).unwrap();
                let ymd = Date::parse_ymd(&raw).map(date_text).unwrap_or_else(|| "none".into());
                format!("space\t{json}\t{date}\t{dt}\t{ymd}")
            }
            _ => {
                let parts: Vec<i32> = cols[1].rsplitn(3, '/').map(|p| p.parse().unwrap()).collect();
                let anchor = Date::from_ymd_opt(parts[2], parts[1] as u32, parts[0] as u32).unwrap();
                let n: u32 = cols[2].parse().unwrap();
                format!("months\t{}\t{n}\t{}\t{}", cols[1], opt_date(anchor.checked_add_months(n)), opt_date(anchor.checked_sub_months(n)))
            }
        };
        if actual != line {
            differing.push(format!("  expected {line}\n  actual   {actual}"));
        }
    }
    assert!(differing.is_empty(), "{} of {} rows differ:\n{}", differing.len(), text.lines().count(), differing.join("\n"));
}
