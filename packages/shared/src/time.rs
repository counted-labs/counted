//! Calendar dates and naive UTC date-times with chrono 0.4's observable contract, over jiff:
//! the same serde bytes, the same accepted inputs, `Default` at 1970-01-01, the same Postgres
//! types. Values chrono holds and jiff cannot (years past ±9999, leap seconds) saturate on
//! decode instead of failing (docs/plans/chrono-to-jiff.md §3.1, D4).

use std::fmt;

use jiff::civil;
use jiff::{SignedDuration, Span};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

pub use jiff::civil::Weekday;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Date(civil::Date);

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DateTime(civil::DateTime);

const CHRONO_YEARS: std::ops::RangeInclusive<i64> = -262_143..=262_142;
const YEARS: std::ops::RangeInclusive<i64> = -9999..=9999;
const NANOS_PER_SECOND: i64 = 1_000_000_000;

impl Date {
    pub const MIN: Date = Date(civil::Date::MIN);
    pub const MAX: Date = Date(civil::Date::MAX);
    pub const EPOCH: Date = Date(civil::date(1970, 1, 1));

    pub fn from_ymd_opt(year: i32, month: u32, day: u32) -> Option<Date> {
        let (year, month, day) = (i16::try_from(year).ok()?, i8::try_from(month).ok()?, i8::try_from(day).ok()?);
        civil::Date::new(year, month, day).ok().map(Date)
    }

    /// chrono's `NaiveDate::parse_from_str(s, "%Y-%m-%d")`, with `None` for a year it would read
    /// past ±9999: the callers' existing "unparseable" path.
    pub fn parse_ymd(s: &str) -> Option<Date> {
        let (rest, year, month, day) = scan_date(s, false)?;
        if !rest.is_empty() || !YEARS.contains(&year) {
            return None;
        }
        Date::from_ymd_opt(year as i32, month, day)
    }

    pub fn year(self) -> i32 {
        self.0.year() as i32
    }

    pub fn month(self) -> u32 {
        self.0.month() as u32
    }

    pub fn month0(self) -> u32 {
        self.month() - 1
    }

    pub fn day(self) -> u32 {
        self.0.day() as u32
    }

    pub fn weekday(self) -> Weekday {
        self.0.weekday()
    }

    pub fn with_day(self, day: u32) -> Option<Date> {
        Date::from_ymd_opt(self.year(), self.month(), day)
    }

    pub fn checked_add_days(self, days: i64) -> Option<Date> {
        self.0.checked_add(Span::new().try_days(days).ok()?).ok().map(Date)
    }

    pub fn checked_sub_days(self, days: i64) -> Option<Date> {
        self.checked_add_days(days.checked_neg()?)
    }

    /// Clamped to the last day of a shorter month, as chrono's `checked_add_months`. Counted on
    /// the month index, not through a jiff `Span`, which stops at 239 976 months.
    pub fn checked_add_months(self, months: u32) -> Option<Date> {
        self.shift_months(months as i64)
    }

    pub fn checked_sub_months(self, months: u32) -> Option<Date> {
        self.shift_months(-(months as i64))
    }

    fn shift_months(self, months: i64) -> Option<Date> {
        let index = self.year() as i64 * 12 + self.month0() as i64 + months;
        let (year, month) = (index.div_euclid(12), index.rem_euclid(12) as u32 + 1);
        if !YEARS.contains(&year) {
            return None;
        }
        Date::from_ymd_opt(year as i32, month, self.day().min(days_in_month(year, month)))
    }

    /// chrono's `(self - other).num_days()`.
    pub fn days_since(self, other: Date) -> i64 {
        self.num_days_from_ce() as i64 - other.num_days_from_ce() as i64
    }

    /// 0001-01-01 is day 1, as chrono counts.
    pub fn num_days_from_ce(self) -> i32 {
        (self.0 - civil::date(1, 1, 1)).get_days() + 1
    }

    pub fn from_num_days_from_ce_opt(days: i32) -> Option<Date> {
        Date(civil::date(1, 1, 1)).checked_add_days(days as i64 - 1)
    }

    pub fn and_hms_opt(self, hour: u32, minute: u32, second: u32) -> Option<DateTime> {
        self.and_hms_nano_opt(hour, minute, second, 0)
    }

    pub fn and_hms_nano_opt(self, hour: u32, minute: u32, second: u32, nano: u32) -> Option<DateTime> {
        let time = civil::Time::new(
            i8::try_from(hour).ok()?,
            i8::try_from(minute).ok()?,
            i8::try_from(second).ok()?,
            i32::try_from(nano).ok()?,
        )
        .ok()?;
        Some(DateTime(self.0.to_datetime(time)))
    }

    pub fn at_midnight(self) -> DateTime {
        DateTime(self.0.to_datetime(civil::Time::midnight()))
    }

    /// `%Y-%m-%d`: four digits, a minus sign before a negative year.
    pub fn ymd(self) -> String {
        self.to_string()
    }
}

impl DateTime {
    pub const MIN: DateTime = DateTime(civil::DateTime::MIN);
    pub const MAX: DateTime = DateTime(civil::DateTime::MAX);
    pub const EPOCH: DateTime = DateTime(civil::date(1970, 1, 1).at(0, 0, 0, 0));

    /// chrono's `Utc::now().naive_utc()`.
    pub fn now_utc() -> DateTime {
        DateTime(jiff::tz::Offset::UTC.to_datetime(jiff::Timestamp::now()))
    }

    /// The instant this naive UTC date-time names, in Unix milliseconds, floored like chrono's
    /// `timestamp_millis`.
    pub fn unix_millis(self) -> i64 {
        self.0.duration_since(DateTime::EPOCH.0).as_nanos().div_euclid(1_000_000) as i64
    }

    /// In Unix seconds, floored like chrono's `timestamp`.
    pub fn unix_secs(self) -> i64 {
        self.0.duration_since(DateTime::EPOCH.0).as_nanos().div_euclid(1_000_000_000) as i64
    }

    pub fn from_unix_millis(millis: i64) -> Option<DateTime> {
        DateTime::EPOCH.0.checked_add(SignedDuration::from_millis(millis)).ok().map(DateTime)
    }

    pub fn date(self) -> Date {
        Date(self.0.date())
    }

    pub fn hour(self) -> u32 {
        self.0.hour() as u32
    }

    pub fn minute(self) -> u32 {
        self.0.minute() as u32
    }

    pub fn second(self) -> u32 {
        self.0.second() as u32
    }

    pub fn nanosecond(self) -> u32 {
        self.0.subsec_nanosecond() as u32
    }

    pub fn checked_add_nanos(self, nanos: i64) -> Option<DateTime> {
        self.0.checked_add(SignedDuration::from_nanos(nanos)).ok().map(DateTime)
    }

    pub fn checked_add_seconds(self, seconds: i64) -> Option<DateTime> {
        self.0.checked_add(SignedDuration::from_secs(seconds)).ok().map(DateTime)
    }

    pub fn checked_add_hours(self, hours: i64) -> Option<DateTime> {
        self.checked_add_seconds(hours.checked_mul(3600)?)
    }

    pub fn checked_add_days(self, days: i64) -> Option<DateTime> {
        self.checked_add_seconds(days.checked_mul(86_400)?)
    }

    /// chrono's `(self - other).num_seconds()`: whole seconds, truncated toward zero.
    pub fn seconds_since(self, other: DateTime) -> i64 {
        self.0.duration_since(other.0).as_secs()
    }

    /// `%H:%M`.
    pub fn hm(self) -> String {
        format!("{:02}:{:02}", self.hour(), self.minute())
    }

    /// `%Y-%m-%d %H:%M`.
    pub fn ymd_hm(self) -> String {
        format!("{} {}", self.date(), self.hm())
    }

    fn write_time(self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}:{:02}", self.hour(), self.minute(), self.second())?;
        match self.nanosecond() {
            0 => Ok(()),
            n if n % 1_000_000 == 0 => write!(f, ".{:03}", n / 1_000_000),
            n if n % 1_000 == 0 => write!(f, ".{:06}", n / 1_000),
            n => write!(f, ".{n:09}"),
        }
    }
}

impl Default for Date {
    fn default() -> Self {
        Date::EPOCH
    }
}

impl Default for DateTime {
    fn default() -> Self {
        DateTime::EPOCH
    }
}

impl fmt::Display for Date {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let year = self.year();
        if year < 0 {
            write!(f, "-{:04}", -year)?;
        } else {
            write!(f, "{year:04}")?;
        }
        write!(f, "-{:02}-{:02}", self.month(), self.day())
    }
}

impl fmt::Debug for Date {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

/// chrono's Display: a space between the date and the time.
impl fmt::Display for DateTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ", self.date())?;
        self.write_time(f)
    }
}

/// chrono's Debug and serde form: a `T` between the date and the time.
impl fmt::Debug for DateTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}T", self.date())?;
        self.write_time(f)
    }
}

impl Serialize for Date {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_str(self)
    }
}

impl Serialize for DateTime {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_str(&format_args!("{self:?}"))
    }
}

impl<'de> Deserialize<'de> for Date {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let text = std::borrow::Cow::<str>::deserialize(deserializer)?;
        decode_date(&text).ok_or_else(|| serde::de::Error::custom("invalid date"))
    }
}

impl<'de> Deserialize<'de> for DateTime {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let text = std::borrow::Cow::<str>::deserialize(deserializer)?;
        decode_datetime(&text).ok_or_else(|| serde::de::Error::custom("invalid date-time"))
    }
}

/// chrono's `scan::number`: 1 to `max` ASCII digits, after any leading whitespace.
fn number(s: &str, max: usize) -> Option<(&str, i64)> {
    let s = s.trim_start();
    let digits = s.bytes().take(max).take_while(u8::is_ascii_digit).count();
    if digits == 0 {
        return None;
    }
    let mut value = 0i64;
    for b in s[..digits].bytes() {
        value = value.checked_mul(10)?.checked_add((b - b'0') as i64)?;
    }
    Some((&s[digits..], value))
}

/// chrono's year item: a sign and any number of digits, or one to four digits without one.
fn year(s: &str) -> Option<(&str, i64)> {
    let s = s.trim_start();
    if let Some(rest) = s.strip_prefix('-') {
        let (rest, value) = number_unsigned_unbounded(rest)?;
        return Some((rest, value.checked_neg()?));
    }
    if let Some(rest) = s.strip_prefix('+') {
        return number_unsigned_unbounded(rest);
    }
    number(s, 4)
}

fn number_unsigned_unbounded(s: &str) -> Option<(&str, i64)> {
    let digits = s.bytes().take_while(u8::is_ascii_digit).count();
    if digits == 0 {
        return None;
    }
    let mut value = 0i64;
    for b in s[..digits].bytes() {
        value = value.checked_mul(10)?.checked_add((b - b'0') as i64)?;
    }
    Some((&s[digits..], value))
}

fn space(s: &str, spaces: bool) -> &str {
    if spaces {
        s.trim_start()
    } else {
        s
    }
}

/// `YYYY-MM-DD` as chrono reads it, with its whitespace items when `spaces` (serde's `FromStr`)
/// and without them (`parse_from_str` with `%Y-%m-%d`). The year is not range-checked here.
fn scan_date(s: &str, spaces: bool) -> Option<(&str, i64, u32, u32)> {
    let (s, year) = year(s)?;
    i32::try_from(year).ok()?;
    let s = space(s, spaces).strip_prefix('-')?;
    let (s, month) = number(s, 2)?;
    if !(1..=12).contains(&month) {
        return None;
    }
    let s = space(s, spaces).strip_prefix('-')?;
    let (s, day) = number(s, 2)?;
    if !(1..=31).contains(&day) {
        return None;
    }
    if day as u32 > days_in_month(year, month as u32) {
        return None;
    }
    Some((space(s, spaces), year, month as u32, day as u32))
}

fn days_in_month(year: i64, month: u32) -> u32 {
    match month {
        2 if year.rem_euclid(4) == 0 && (year.rem_euclid(100) != 0 || year.rem_euclid(400) == 0) => 29,
        2 => 28,
        4 | 6 | 9 | 11 => 30,
        _ => 31,
    }
}

fn saturated_date(year: i64, month: u32, day: u32) -> Option<Date> {
    if !CHRONO_YEARS.contains(&year) {
        return None;
    }
    Some(match year {
        y if y < *YEARS.start() => Date::MIN,
        y if y > *YEARS.end() => Date::MAX,
        _ => Date::from_ymd_opt(year as i32, month, day)?,
    })
}

fn decode_date(s: &str) -> Option<Date> {
    let (rest, year, month, day) = scan_date(s, true)?;
    if !rest.is_empty() {
        return None;
    }
    saturated_date(year, month, day)
}

fn decode_datetime(s: &str) -> Option<DateTime> {
    let (s, year, month, day) = scan_date(s, true)?;
    let s = s.strip_prefix('T')?;
    let (s, hour) = number(s, 2)?;
    let s = s.trim_start().strip_prefix(':')?;
    let (s, minute) = number(s, 2)?;
    let s = s.trim_start().strip_prefix(':')?;
    let (s, second) = number(s, 2)?;
    let (s, nano) = match s.strip_prefix('.') {
        Some(fraction) => {
            let digits = fraction.bytes().take(9).take_while(u8::is_ascii_digit).count();
            if digits == 0 {
                return None;
            }
            let value: i64 = fraction[..digits].parse().ok()?;
            let nano = value * 10i64.pow(9 - digits as u32);
            (fraction[digits..].trim_start_matches(|c: char| c.is_ascii_digit()), nano)
        }
        None => (s, 0),
    };
    if !s.trim_start().is_empty() || hour > 23 || minute > 59 || second > 60 || nano >= NANOS_PER_SECOND {
        return None;
    }
    let date = saturated_date(year, month, day)?;
    if !YEARS.contains(&year) {
        return Some(if year < 0 { DateTime::MIN } else { DateTime::MAX });
    }
    if second == 60 {
        return date.and_hms_nano_opt(hour as u32, minute as u32, 59, 999_999_999);
    }
    date.and_hms_nano_opt(hour as u32, minute as u32, second as u32, nano as u32)
}

#[cfg(feature = "server")]
mod postgres {
    use jiff_sqlx::ToSqlx;
    use sqlx::encode::IsNull;
    use sqlx::error::BoxDynError;
    use sqlx::postgres::{PgArgumentBuffer, PgTypeInfo, PgValueRef};
    use sqlx::{Decode, Encode, Postgres, Type};

    use super::{Date, DateTime};

    impl Type<Postgres> for DateTime {
        fn type_info() -> PgTypeInfo {
            <jiff_sqlx::DateTime as Type<Postgres>>::type_info()
        }
    }

    impl Encode<'_, Postgres> for DateTime {
        fn encode_by_ref(&self, buf: &mut PgArgumentBuffer) -> Result<IsNull, BoxDynError> {
            Encode::<Postgres>::encode(self.0.to_sqlx(), buf)
        }
    }

    impl<'r> Decode<'r, Postgres> for DateTime {
        fn decode(value: PgValueRef<'r>) -> Result<Self, BoxDynError> {
            Ok(DateTime(<jiff_sqlx::DateTime as Decode<Postgres>>::decode(value)?.to_jiff()))
        }
    }

    impl Type<Postgres> for Date {
        fn type_info() -> PgTypeInfo {
            <jiff_sqlx::Date as Type<Postgres>>::type_info()
        }
    }

    impl Encode<'_, Postgres> for Date {
        fn encode_by_ref(&self, buf: &mut PgArgumentBuffer) -> Result<IsNull, BoxDynError> {
            Encode::<Postgres>::encode(self.0.to_sqlx(), buf)
        }
    }

    impl<'r> Decode<'r, Postgres> for Date {
        fn decode(value: PgValueRef<'r>) -> Result<Self, BoxDynError> {
            Ok(Date(<jiff_sqlx::Date as Decode<Postgres>>::decode(value)?.to_jiff()))
        }
    }
}
