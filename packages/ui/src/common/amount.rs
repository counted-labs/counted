use crate::i18n::current_lang;

/// "1 128,60 EUR" in French, "1 128.60 EUR" in English. Display only: exports keep the plain
/// `{:.2}` a spreadsheet reads back.
pub fn amount(value: f64, currency: &str) -> String {
    with_currency(number(value), currency)
}

/// [`amount`] with a "+" on a positive value.
pub fn signed_amount(value: f64, currency: &str) -> String {
    let body = amount(value, currency);
    if (value * 100.0).round() > 0.0 { format!("+{body}") } else { body }
}

/// [`amount`] without ",00" on a round value: a budget ceiling reads "1 200 EUR".
pub fn round_amount(value: f64, currency: &str) -> String {
    with_currency(drop_zero_cents(number(value)), currency)
}

fn drop_zero_cents(number: String) -> String {
    match number.strip_suffix(",00").or_else(|| number.strip_suffix(".00")) {
        Some(whole) => whole.to_string(),
        None => number,
    }
}

pub fn number(value: f64) -> String {
    format_number(value, decimal_comma(&current_lang()))
}

/// Whole units, for charts: "1 728 EUR".
pub fn whole_amount(value: f64, currency: &str) -> String {
    with_currency(group_thousands(value.abs().round() as u64), currency)
}

pub fn comma_language() -> bool {
    decimal_comma(&current_lang())
}

/// What an amount field shows for a number it did not get from the keyboard: "12,50" in French,
/// "210" for a whole amount. Cents always take two places, as a price is written - a scanned 37.2
/// read "37,2" and its shares "9,3".
pub fn plain_number(value: f64, comma: bool) -> String {
    let plain = if value.fract() == 0.0 { value.to_string() } else { format!("{value:.2}") };
    if comma { plain.replace('.', ",") } else { plain }
}

fn with_currency(number: String, currency: &str) -> String {
    if currency.is_empty() { number } else { format!("{number}\u{a0}{currency}") }
}

fn decimal_comma(lang: &str) -> bool {
    !matches!(lang, "en" | "ga" | "mt")
}

fn group_thousands(n: u64) -> String {
    let digits = n.to_string();
    let mut out = String::new();
    for (i, ch) in digits.chars().enumerate() {
        if i > 0 && (digits.len() - i).is_multiple_of(3) {
            out.push('\u{a0}');
        }
        out.push(ch);
    }
    out
}

fn format_number(value: f64, comma: bool) -> String {
    let cents = (value.abs() * 100.0).round() as u64;
    let whole = group_thousands(cents / 100);
    let sign = if value < 0.0 && cents > 0 { "-" } else { "" };
    let separator = if comma { ',' } else { '.' };
    format!("{sign}{whole}{separator}{:02}", cents % 100)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn groups_thousands_and_uses_the_language_separator() {
        assert_eq!(format_number(1128.6, true), "1\u{a0}128,60");
        assert_eq!(format_number(1128.6, false), "1\u{a0}128.60");
        assert_eq!(format_number(1234567.0, true), "1\u{a0}234\u{a0}567,00");
        assert_eq!(format_number(46.3, true), "46,30");
    }

    #[test]
    fn a_negative_keeps_its_sign_unless_it_rounds_to_zero() {
        assert_eq!(format_number(-48.0, true), "-48,00");
        assert_eq!(format_number(-0.001, true), "0,00");
        assert_eq!(format_number(0.005, false), "0.01");
    }

    #[test]
    fn english_irish_and_maltese_write_a_decimal_point() {
        assert!(!decimal_comma("en"));
        assert!(!decimal_comma("ga"));
        assert!(!decimal_comma("mt"));
        assert!(decimal_comma("fr"));
        assert!(decimal_comma("de"));
    }

    #[test]
    fn a_round_value_loses_its_zero_cents_only() {
        assert_eq!(drop_zero_cents("1\u{a0}200,00".into()), "1\u{a0}200");
        assert_eq!(drop_zero_cents("400.00".into()), "400");
        assert_eq!(drop_zero_cents("424,36".into()), "424,36");
        assert_eq!(drop_zero_cents("10,50".into()), "10,50");
    }

    #[test]
    fn whole_units_round_and_group() {
        assert_eq!(group_thousands(1728), "1\u{a0}728");
        assert_eq!(group_thousands(640), "640");
    }

    #[test]
    fn a_field_number_takes_the_language_separator() {
        assert_eq!(plain_number(12.5, true), "12,50");
        assert_eq!(plain_number(12.5, false), "12.50");
        assert_eq!(plain_number(83.33, false), "83.33");
        assert_eq!(plain_number(210.0, true), "210");
    }

    #[test]
    fn the_currency_follows_a_no_break_space() {
        assert_eq!(with_currency("46,30".into(), "EUR"), "46,30\u{a0}EUR");
        assert_eq!(with_currency("46,30".into(), ""), "46,30");
    }
}
