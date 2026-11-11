use crate::i18n::current_lang;

/// "1 128,60 €" in French, "€1,128.60" in English, "1 128,60 USD" for a currency whose symbol
/// stands for several. Display only: exports keep the plain `{:.2}` a spreadsheet reads back.
pub fn amount(value: f64, currency: &str) -> String {
    let lang = current_lang();
    with_currency(format_number(value, decimal_comma(&lang)), currency, &lang)
}

/// [`amount`] with a "+" on a positive value.
pub fn signed_amount(value: f64, currency: &str) -> String {
    let body = amount(value, currency);
    if (value * 100.0).round() > 0.0 { format!("+{body}") } else { body }
}

/// [`amount`] without ",00" on a round value: a budget ceiling reads "1 200 €".
pub fn round_amount(value: f64, currency: &str) -> String {
    let lang = current_lang();
    with_currency(drop_zero_cents(format_number(value, decimal_comma(&lang))), currency, &lang)
}

fn drop_zero_cents(number: String) -> String {
    match number.strip_suffix(",00").or_else(|| number.strip_suffix(".00")) {
        Some(whole) => whole.to_string(),
        None => number,
    }
}

/// Four significant digits, trailing zeros dropped: "0,002729", "1,083", "370,5".
pub fn rate(value: f64) -> String {
    format_rate(value, comma_language())
}

fn format_rate(value: f64, comma: bool) -> String {
    let decimals = (3.0 - value.abs().log10().floor()).clamp(0.0, 12.0) as usize;
    let fixed = format!("{value:.decimals$}");
    let trimmed = if fixed.contains('.') { fixed.trim_end_matches('0').trim_end_matches('.') } else { &fixed };
    if comma { trimmed.replace('.', ",") } else { trimmed.to_string() }
}

pub fn number(value: f64) -> String {
    format_number(value, decimal_comma(&current_lang()))
}

/// Whole units, for charts: "1 728 €".
pub fn whole_amount(value: f64, currency: &str) -> String {
    with_currency(group_thousands(value.abs().round() as u64), currency, &current_lang())
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

/// Only symbols that name one currency the world over. `$`, `¥`, `kr`, `Fr` and the like each
/// stand for several, and a project can mix currencies, so those keep the ISO code.
fn symbol(currency: &str) -> Option<&'static str> {
    match currency {
        "EUR" => Some("€"),
        "GBP" => Some("£"),
        "INR" => Some("₹"),
        "ILS" => Some("₪"),
        "KRW" => Some("₩"),
        "UAH" => Some("₴"),
        "TRY" => Some("₺"),
        "VND" => Some("₫"),
        "NGN" => Some("₦"),
        _ => None,
    }
}

enum Placement {
    Before,
    BeforeSpaced,
    After,
}

/// CLDR's currency pattern per shipped language: en, ga, mt and tr write "€46.30", nl "€ 46,30",
/// every other one "46,30 €". The sign always leads.
fn placement(lang: &str) -> Placement {
    match lang {
        "en" | "ga" | "mt" | "tr" => Placement::Before,
        "nl" => Placement::BeforeSpaced,
        _ => Placement::After,
    }
}

fn with_currency(number: String, currency: &str, lang: &str) -> String {
    if currency.is_empty() {
        return number;
    }
    let Some(symbol) = symbol(currency) else {
        return format!("{number}\u{a0}{currency}");
    };
    let (sign, digits) = match number.strip_prefix('-') {
        Some(digits) => ("-", digits),
        None => ("", number.as_str()),
    };
    match placement(lang) {
        Placement::Before => format!("{sign}{symbol}{digits}"),
        Placement::BeforeSpaced => format!("{sign}{symbol}\u{a0}{digits}"),
        Placement::After => format!("{number}\u{a0}{symbol}"),
    }
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
    fn a_rate_keeps_four_significant_digits_in_the_language_separator() {
        assert_eq!(format_rate(0.0027294066269992904, true), "0,002729");
        assert_eq!(format_rate(0.0027294066269992904, false), "0.002729");
        assert_eq!(format_rate(1.0834, true), "1,083");
        assert_eq!(format_rate(370.5, true), "370,5");
        assert_eq!(format_rate(2.0, true), "2");
    }

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
    fn the_symbol_follows_a_no_break_space_by_default() {
        assert_eq!(with_currency("46,30".into(), "EUR", "fr"), "46,30\u{a0}€");
        assert_eq!(with_currency("-46,30".into(), "EUR", "de"), "-46,30\u{a0}€");
        assert_eq!(with_currency("1\u{a0}200".into(), "GBP", "pl"), "1\u{a0}200\u{a0}£");
    }

    #[test]
    fn english_irish_maltese_and_turkish_put_the_symbol_first() {
        for lang in ["en", "ga", "mt", "tr"] {
            assert_eq!(with_currency("46.30".into(), "EUR", lang), "€46.30", "{lang}");
            assert_eq!(with_currency("-46.30".into(), "EUR", lang), "-€46.30", "{lang}");
        }
    }

    #[test]
    fn dutch_puts_the_symbol_first_with_a_space() {
        assert_eq!(with_currency("46,30".into(), "EUR", "nl"), "€\u{a0}46,30");
        assert_eq!(with_currency("-46,30".into(), "EUR", "nl"), "-€\u{a0}46,30");
    }

    #[test]
    fn every_shipped_language_has_a_placement() {
        for lang in shared::UI_LANGS {
            let placed = with_currency("1".into(), "EUR", lang);
            assert!(placed.contains('€'), "{lang}: {placed}");
        }
    }

    #[test]
    fn a_shared_symbol_keeps_the_iso_code_after_the_number() {
        for currency in ["USD", "CAD", "AUD", "JPY", "CNY", "SEK", "NOK", "DKK", "CHF"] {
            assert_eq!(symbol(currency), None, "{currency}");
            assert_eq!(with_currency("46.30".into(), currency, "en"), format!("46.30\u{a0}{currency}"));
            assert_eq!(with_currency("46,30".into(), currency, "fr"), format!("46,30\u{a0}{currency}"));
        }
    }

    #[test]
    fn no_currency_is_the_bare_number() {
        assert_eq!(with_currency("46,30".into(), "", "fr"), "46,30");
        assert_eq!(with_currency("46.30".into(), "", "en"), "46.30");
    }
}
