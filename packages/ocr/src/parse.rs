//! Recognised text lines into the four fields the expense form wants.
//!
//! Entirely pure and free of the models, so it compiles and tests under
//! `--no-default-features`. This is where the feature is won or lost: the OCR either reads a
//! receipt or it does not, but *which* of the twenty numbers on it is the total is decided here.

use chrono::NaiveDate;

/// One recognised line, with its geometry as **fractions of the source image** so the parser is
/// resolution-independent and its fixtures are hand-writable.
#[derive(Debug, Clone, PartialEq)]
pub struct TextLine {
    pub text: String,
    /// Left edge, 0.0 = left of the image.
    pub x: f32,
    /// Vertical centre, 0.0 = top.
    pub y: f32,
    /// Line height.
    pub h: f32,
    /// Mean per-character probability, 0.0..=1.0.
    pub confidence: f32,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Receipt {
    pub title: Option<String>,
    pub amount: Option<f64>,
    pub date: Option<NaiveDate>,
    /// False when `amount` came from the bottom-of-page fallback rather than a keyword anchor.
    /// The form surfaces this as "check the total".
    pub amount_confident: bool,
}

/// `today` is injected rather than read here so the plausibility window is testable.
pub fn parse_receipt(lines: &[TextLine], today: NaiveDate) -> Receipt {
    let (amount, amount_confident) = match find_amount(lines) {
        Some((value, confident)) => (Some(value), confident),
        None => (None, false),
    };
    Receipt { title: find_title(lines), amount, date: find_date(lines, today), amount_confident }
}

// ─── text folding ────────────────────────────────────────────────────────────

/// Lowercase, accents folded, every non-alphanumeric collapsed to a single space.
///
/// Deliberately not `ui::categories::normalize`: that one also drops stop tokens and returns a
/// token vector, and "de"/"a" are load-bearing here ("net a payer", "te betalen").
fn fold(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut pending_space = false;
    for c in s.to_lowercase().chars() {
        let mapped = match c {
            'á' | 'à' | 'â' | 'ä' | 'ã' | 'å' => 'a',
            'é' | 'è' | 'ê' | 'ë' => 'e',
            'í' | 'ì' | 'î' | 'ï' => 'i',
            'ó' | 'ò' | 'ô' | 'ö' | 'õ' => 'o',
            'ú' | 'ù' | 'û' | 'ü' => 'u',
            'ç' => 'c',
            'ñ' => 'n',
            'ý' | 'ÿ' => 'y',
            'æ' => {
                push_folded(&mut out, &mut pending_space, 'a');
                'e'
            }
            'œ' => {
                push_folded(&mut out, &mut pending_space, 'o');
                'e'
            }
            'ß' => {
                push_folded(&mut out, &mut pending_space, 's');
                's'
            }
            c if c.is_alphanumeric() => c,
            _ => {
                pending_space = !out.is_empty();
                continue;
            }
        };
        push_folded(&mut out, &mut pending_space, mapped);
    }
    out
}

fn push_folded(out: &mut String, pending_space: &mut bool, c: char) {
    if *pending_space {
        out.push(' ');
        *pending_space = false;
    }
    out.push(c);
}

/// Whole-word containment. Both sides are `fold`ed, so padding with spaces is a sufficient
/// boundary check and multi-word phrases ("sous total") work unchanged.
fn has_word(folded: &str, needle: &str) -> bool {
    format!(" {folded} ").contains(&format!(" {needle} "))
}

// ─── money ───────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq)]
struct Money {
    value: f64,
    /// The token carried a decimal part. Prices do; quantities, years and article counts do not.
    has_cents: bool,
}

const MAX_PLAUSIBLE: f64 = 100_000.0;

fn is_digit(b: u8) -> bool {
    b.is_ascii_digit()
}

/// Every number on the line that could be an amount of money, in reading order.
///
/// Hand-rolled rather than a `regex` dependency for one pattern. The rules that matter:
///
/// - a space is a group separator only before any decimal separator has been seen and only in
///   front of exactly three digits, so `1 234,56` is one number but `12,90 20,00` is two;
/// - a token touching `/`, `:` or `-` is a date or a time, not money — without this the
///   bottom-of-page fallback happily returns the year off `04/09/2026`;
/// - a token followed by `%` is a VAT rate, and one followed by `x` is a quantity.
fn money_tokens(s: &str) -> Vec<Money> {
    let b = s.as_bytes();
    let mut out = Vec::new();
    let mut i = 0;
    while i < b.len() {
        if !is_digit(b[i]) {
            i += 1;
            continue;
        }
        let start = i;
        let mut seen_point = false;
        while i < b.len() {
            let c = b[i];
            let next_is_digit = b.get(i + 1).is_some_and(|n| is_digit(*n));
            if is_digit(c) {
                i += 1;
            } else if matches!(c, b'.' | b',') && next_is_digit {
                seen_point = true;
                i += 1;
            } else if (c == b'\'' && next_is_digit)
                || (c == b' ' && !seen_point && group_of_three(b, i + 1))
            {
                i += 1;
            } else {
                break;
            }
        }
        let end = i;
        if !touches_date_punctuation(b, start, end) {
            if let Some(m) = classify(&s[start..end]) {
                if !followed_by_rate_or_quantity(b, end) {
                    out.push(m);
                }
            }
        }
    }
    out
}

/// Exactly three digits, then something that is not a digit.
fn group_of_three(b: &[u8], at: usize) -> bool {
    b.len() >= at + 3
        && b[at..at + 3].iter().all(|c| is_digit(*c))
        && b.get(at + 3).is_none_or(|c| !is_digit(*c))
}

fn touches_date_punctuation(b: &[u8], start: usize, end: usize) -> bool {
    let before = start.checked_sub(1).map(|i| b[i]);
    let after = b.get(end).copied();
    let is_date_punct = |c: Option<u8>| matches!(c, Some(b'/') | Some(b':') | Some(b'-'));
    is_date_punct(before) || is_date_punct(after)
}

/// `%` is a VAT rate; `x` is a quantity, and it is the token *before* the `x` that is rejected —
/// `2 x 4,50` must still yield 4.50.
fn followed_by_rate_or_quantity(b: &[u8], end: usize) -> bool {
    let mut i = end;
    while i < b.len() && b[i] == b' ' {
        i += 1;
    }
    matches!(b.get(i), Some(b'%')) || matches!(b.get(i), Some(b'x') | Some(b'X'))
}

/// Resolves group separators against decimal separators. With both present the **last** one is
/// the decimal point, which is what makes `1.234,56` and `1,234.56` both 1234.56.
fn classify(token: &str) -> Option<Money> {
    let cleaned: String = token.chars().filter(|c| *c != ' ' && *c != '\'').collect();
    let last_dot = cleaned.rfind('.');
    let last_comma = cleaned.rfind(',');

    let decimal_at = match (last_dot, last_comma) {
        (Some(d), Some(c)) => Some(d.max(c)),
        (Some(d), None) => single_separator(&cleaned, d, '.'),
        (None, Some(c)) => single_separator(&cleaned, c, ','),
        (None, None) => None,
    };

    let (digits, cents) = match decimal_at {
        Some(at) => {
            let whole: String = cleaned[..at].chars().filter(char::is_ascii_digit).collect();
            let frac: String = cleaned[at + 1..].chars().filter(char::is_ascii_digit).collect();
            (format!("{whole}.{frac}"), true)
        }
        None => (cleaned.chars().filter(char::is_ascii_digit).collect(), false),
    };

    let value: f64 = digits.parse().ok()?;
    if !value.is_finite() || value <= 0.0 || value > MAX_PLAUSIBLE {
        return None;
    }
    Some(Money { value, has_cents: cents })
}

/// One separator of a kind: three trailing digits make it a group separator, anything else makes
/// it the decimal point. A second occurrence of the same character always means groups.
fn single_separator(cleaned: &str, at: usize, sep: char) -> Option<usize> {
    if cleaned.matches(sep).count() > 1 {
        return None;
    }
    if cleaned[at + 1..].len() == 3 {
        return None;
    }
    Some(at)
}

// ─── amount ──────────────────────────────────────────────────────────────────

/// Checked first, and as whole words, so `sous-total` is out before `total` is ever tried.
const NEGATIVE: &[&str] = &[
    "sous total",
    "soustotal",
    "subtotal",
    "sub total",
    "zwischensumme",
    "subtotaal",
    "totale parziale",
    "total parcial",
    "tva",
    "t v a",
    // How the recogniser reads "TVA" in dot-matrix print: V and U are one glyph apart there, and
    // the Hyper U fixture misreads it on every line it appears.
    "tua",
    "vat",
    "mwst",
    "ust",
    "iva",
    "btw",
    "taxe",
    "taxes",
    "tax",
    "dont tva",
    "rendu",
    "monnaie",
    "change",
    "wechselgeld",
    "troco",
    "cambio",
    "resto",
    "especes",
    "cash",
    "contant",
    "carte",
    "card",
    "cb",
    "visa",
    "mastercard",
    "bancontact",
    "remise",
    "reduction",
    "reductions",
    "discount",
    "rabatt",
    "korting",
    "pourboire",
    "tip",
    "acompte",
    "avoir",
    "articles",
    "article",
    "points",
    "fidelite",
];

const POSITIVE: &[&str] = &[
    "total",
    "total ttc",
    "total ht",
    "montant",
    "montant du",
    "net a payer",
    "a payer",
    // "RESTE A PAYER" / "RESTE DU". The bare word, because "payer" comes back as "paver" often
    // enough that the two-word phrase alone missed the strongest anchor on the page.
    "reste",
    "grand total",
    "amount due",
    "balance due",
    "total due",
    "summe",
    "gesamt",
    "gesamtbetrag",
    "zu zahlen",
    "endbetrag",
    "totale",
    "totale complessivo",
    "importo",
    "importe",
    "importe total",
    "total a pagar",
    "total geral",
    "totaal",
    "te betalen",
    "bedrag",
];

/// Where the bottom-of-page fallback starts looking.
const FALLBACK_FROM_Y: f32 = 0.55;
/// How far below an anchor its figure may sit, in multiples of the anchor's own line height.
const ANCHOR_REACH: f32 = 2.5;
/// A "TOTAL" line carrying this many prices is the last row of the VAT breakdown table (tax, net,
/// gross), which French supermarkets print *below* the amount actually paid. A grand total line
/// never carries three.
const VAT_TABLE_PRICES: usize = 3;

/// `(value, confident)`. Confident means a keyword anchored it; otherwise it is the largest
/// price-shaped number in the bottom of the page, and the form says so.
fn find_amount(lines: &[TextLine]) -> Option<(f64, bool)> {
    let mut best: Option<(f32, f64)> = None;
    for (i, line) in lines.iter().enumerate() {
        let folded = fold(&line.text);
        if NEGATIVE.iter().any(|n| has_word(&folded, n)) {
            continue;
        }
        if !POSITIVE.iter().any(|p| has_word(&folded, p)) {
            continue;
        }
        let tokens = money_tokens(&line.text);
        if tokens.iter().filter(|m| m.has_cents).count() >= VAT_TABLE_PRICES {
            continue;
        }
        let value = tokens.last().map(|m| m.value).or_else(|| amount_below(lines, i));
        // A receipt prints its grand total last, so the lowest anchor on the page wins over a
        // "TOTAL ARTICLES" near the top.
        if let Some(value) = value {
            if best.is_none_or(|(y, _)| line.y > y) {
                best = Some((line.y, value));
            }
        }
    }
    if let Some((_, value)) = best {
        return Some((value, true));
    }

    lines
        .iter()
        .filter(|l| l.y > FALLBACK_FROM_Y)
        .flat_map(|l| money_tokens(&l.text))
        .filter(|m| m.has_cents)
        .map(|m| m.value)
        .max_by(|a, b| a.total_cmp(b))
        .map(|value| (value, false))
}

/// The rightmost figure on the nearest line below the anchor — a left-hand one is usually a
/// quantity or an article count.
fn amount_below(lines: &[TextLine], anchor: usize) -> Option<f64> {
    let a = &lines[anchor];
    lines
        .iter()
        .filter(|l| l.y > a.y && l.y - a.y <= ANCHOR_REACH * a.h)
        .min_by(|l, r| l.y.total_cmp(&r.y))
        .and_then(|l| money_tokens(&l.text).last().map(|m| m.value))
}

// ─── date ────────────────────────────────────────────────────────────────────

/// A receipt older than this is almost certainly a misread, not a very old purchase.
const MAX_AGE_DAYS: i64 = 730;

/// Folded month names and the unambiguous abbreviations, for the seven shipped locales.
const MONTHS: &[(&str, u32)] = &[
    ("janvier", 1), ("january", 1), ("enero", 1), ("januar", 1), ("gennaio", 1),
    ("janeiro", 1), ("januari", 1), ("jan", 1),
    ("fevrier", 2), ("february", 2), ("febrero", 2), ("februar", 2), ("febbraio", 2),
    ("fevereiro", 2), ("februari", 2), ("feb", 2), ("fev", 2),
    ("mars", 3), ("march", 3), ("marzo", 3), ("marz", 3), ("marco", 3), ("maart", 3), ("mar", 3),
    ("avril", 4), ("april", 4), ("abril", 4), ("aprile", 4), ("apr", 4), ("avr", 4),
    ("mai", 5), ("may", 5), ("mayo", 5), ("maggio", 5), ("maio", 5), ("mei", 5),
    ("juin", 6), ("june", 6), ("junio", 6), ("juni", 6), ("giugno", 6), ("junho", 6), ("jun", 6),
    ("juillet", 7), ("july", 7), ("julio", 7), ("juli", 7), ("luglio", 7), ("julho", 7), ("jul", 7),
    ("aout", 8), ("august", 8), ("agosto", 8), ("augustus", 8), ("aug", 8),
    ("septembre", 9), ("september", 9), ("septiembre", 9), ("settembre", 9), ("setembro", 9),
    ("sept", 9), ("sep", 9),
    ("octobre", 10), ("october", 10), ("octubre", 10), ("oktober", 10), ("ottobre", 10),
    ("outubro", 10), ("oct", 10), ("okt", 10),
    ("novembre", 11), ("november", 11), ("noviembre", 11), ("novembro", 11), ("nov", 11),
    ("decembre", 12), ("december", 12), ("diciembre", 12), ("dezember", 12), ("dicembre", 12),
    ("dezembro", 12), ("dec", 12), ("dez", 12), ("dic", 12),
];

fn find_date(lines: &[TextLine], today: NaiveDate) -> Option<NaiveDate> {
    lines.iter().find_map(|line| {
        numeric_date(&line.text)
            .or_else(|| named_date(&fold(&line.text)))
            .filter(|d| plausible(*d, today))
    })
}

fn plausible(date: NaiveDate, today: NaiveDate) -> bool {
    date <= today && (today - date).num_days() <= MAX_AGE_DAYS
}

/// Up to four digits. `bounded` rejects a run that continues into another digit, which is what
/// stops `123/45/6789` looking like a date.
fn take_digits(b: &[u8], at: usize, bounded: bool) -> Option<(u32, usize)> {
    let mut len = 0;
    while len < 4 && b.get(at + len).is_some_and(|c| is_digit(*c)) {
        len += 1;
    }
    if len == 0 || (bounded && b.get(at + len).is_some_and(|c| is_digit(*c))) {
        return None;
    }
    let value: u32 = core::str::from_utf8(&b[at..at + len]).ok()?.parse().ok()?;
    Some((value, len))
}

fn numeric_date(s: &str) -> Option<NaiveDate> {
    let b = s.as_bytes();
    (0..b.len())
        .filter(|i| is_digit(b[*i]) && i.checked_sub(1).is_none_or(|p| !is_digit(b[p])))
        .find_map(|i| numeric_date_at(b, i))
}

fn numeric_date_at(b: &[u8], i: usize) -> Option<NaiveDate> {
    let (first, flen) = take_digits(b, i, true)?;
    let sep = *b.get(i + flen)?;
    if !matches!(sep, b'/' | b'.' | b'-') {
        return None;
    }
    let (second, slen) = take_digits(b, i + flen + 1, true)?;
    if *b.get(i + flen + 1 + slen)? != sep {
        return None;
    }
    // Unbounded on purpose. The recogniser drops the space between a date and the time beside it —
    // "04/09/2026 14:32" comes back as "04/09/202614:32" — and requiring a non-digit after the year
    // threw away every timestamped receipt. Four digits are taken greedily and the plausibility
    // window below is what rejects nonsense.
    let (third, tlen) = take_digits(b, i + flen + 1 + slen + 1, false)?;

    // ISO: a four-digit leading field can only be a year.
    if flen == 4 {
        return NaiveDate::from_ymd_opt(first as i32, second, third);
    }
    let year = match tlen {
        4 => third as i32,
        2 => 2000 + third as i32,
        _ => return None,
    };
    // Day-first by default: five of the seven shipped locales write it that way and the bundle's
    // development region is fr_FR. A day over twelve settles it either way.
    let (day, month) = if first <= 12 && second > 12 { (second, first) } else { (first, second) };
    NaiveDate::from_ymd_opt(year, month, day)
}

fn month_number(token: &str) -> Option<u32> {
    MONTHS.iter().find(|(name, _)| *name == token).map(|(_, m)| *m)
}

fn named_date(folded: &str) -> Option<NaiveDate> {
    let tokens: Vec<&str> = folded.split_whitespace().collect();
    tokens.windows(3).find_map(|w| {
        let (day, month, year) = match (w[0].parse::<u32>(), month_number(w[1])) {
            // 4 septembre 2026
            (Ok(day), Some(month)) => (day, month, w[2].parse::<i32>().ok()?),
            // september 4 2026 — the comma is already gone, folding removed it
            _ => (w[1].parse::<u32>().ok()?, month_number(w[0])?, w[2].parse::<i32>().ok()?),
        };
        NaiveDate::from_ymd_opt(year, month, day)
    })
}

// ─── title ───────────────────────────────────────────────────────────────────

/// The merchant name is printed in the header, above the address block — as a fraction of the
/// **detected text**, not of the image.
///
/// `TextLine.y` is a fraction of image height (`detect::group_bounds`), and a hand-held photo puts
/// an arbitrary amount of background above the receipt. Measuring the zone against the image made
/// the title the only field whose accuracy depended on framing, and `tests/framing.rs` shows the
/// cost precisely: pad the fixture with 30% background and the header is still detected and
/// recognised perfectly — "CARREFOUR MARKET" at confidence 1.00 — but lands at `y = 0.280` and is
/// discarded for being 0.03 outside a window that has nothing to do with the receipt. 20% padding
/// still passes, 30% does not, and 30% is an ordinary photo.
const TITLE_ZONE: f32 = 0.25;
/// Second pass. A receipt with a logo block or a long header pushes the name down within its own
/// text span, and a merely imperfect title beats a blank one in a field the user is looking at.
const TITLE_ZONE_WIDE: f32 = 0.45;

/// Currently equal to `lib.rs`'s `MIN_LINE_CONFIDENCE`, so in the shipped pipeline this rejects
/// nothing. It is kept because `find_title` is a pure function with its own tests, and because
/// lowering the global floor to admit stylised header text is a live option — the day that happens,
/// this is the guard that stops a garbled but large line from outscoring the real name.
const MIN_CONFIDENCE: f32 = 0.5;

const TITLE_NOISE: &[&str] = &[
    "tel", "telephone", "phone", "fax", "siret", "siren", "rcs", "ape", "naf", "tva", "vat",
    "www", "http", "https", "ticket", "recu", "facture", "invoice", "caisse", "vendeur",
    "bienvenue", "welcome", "merci", "thank", "bonjour", "kassenbon", "beleg", "kassa",
];

const LEGAL_FORMS: &[&str] = &[
    "sarl", "sas", "sasu", "sa", "eurl", "snc", "sci", "scop", "gmbh", "ag", "ug", "kg", "ohg",
    "bv", "nv", "spa", "srl", "ltd", "ltda", "plc", "inc", "llc", "lda", "sl", "sau", "eirl",
];

fn find_title(lines: &[TextLine]) -> Option<String> {
    let median = median_height(lines)?;
    let top = lines.iter().map(|l| l.y).fold(f32::MAX, f32::min);
    let bottom = lines.iter().map(|l| l.y).fold(f32::MIN, f32::max);
    let span = (bottom - top).max(f32::EPSILON);

    let candidates = |limit: f32| -> Vec<&TextLine> {
        lines
            .iter()
            .filter(|l| l.y <= limit && l.confidence >= MIN_CONFIDENCE)
            .filter(|l| is_title_candidate(&l.text))
            .collect()
    };

    let best = [TITLE_ZONE, TITLE_ZONE_WIDE]
        .into_iter()
        .find_map(|cut| {
            candidates(top + span * cut).into_iter().max_by(|a, b| {
                title_score(a, median).total_cmp(&title_score(b, median)).then(b.y.total_cmp(&a.y))
            })
        })
        // Neither zone held a candidate. Fall back to the first one in reading order, which is what
        // the SROIE baselines use for the company field, rather than returning nothing.
        .or_else(|| candidates(f32::MAX).into_iter().min_by(|a, b| a.y.total_cmp(&b.y)))?;

    let cleaned = strip_legal_form(&best.text);
    if cleaned.is_empty() {
        return None;
    }
    Some(if is_all_caps(&cleaned) { title_case(&cleaned) } else { cleaned })
}

fn median_height(lines: &[TextLine]) -> Option<f32> {
    let mut heights: Vec<f32> = lines.iter().map(|l| l.h).collect();
    if heights.is_empty() {
        return None;
    }
    heights.sort_by(|a, b| a.total_cmp(b));
    Some(heights[heights.len() / 2].max(f32::EPSILON))
}

fn is_title_candidate(text: &str) -> bool {
    let letters = text.chars().filter(|c| c.is_alphabetic()).count();
    if text.chars().count() < 3 || letters < 2 {
        return false;
    }
    // A phone number, SIRET or VAT id. Nine is below the shortest of the three and above any
    // plausible run inside a name.
    if longest_digit_run(text) >= 9 {
        return false;
    }
    // A priced line — "PAIN COMPLET 2,40", "TOTAL 12,90". `has_cents` is the discriminator the
    // amount parser already relies on: prices carry a decimal part, while the digits that legally
    // appear in a merchant name ("MAGASIN 24", "CARREFOUR 2000") do not. Without this the widened
    // zone below lets item lines compete with the header, and a receipt whose only readable line is
    // its total would take that total as its name.
    if money_tokens(text).iter().any(|m| m.has_cents) {
        return false;
    }
    let folded = fold(text);
    if TITLE_NOISE.iter().any(|n| has_word(&folded, n)) {
        return false;
    }
    // An amount anchor is not a merchant name. On a skewed page `group_lines` can split "TOTAL
    // 12,90" into two lines, and the bare "TOTAL" then has no decimal part left to disqualify it —
    // `tests/framing.rs` caught exactly that at 5 degrees. Reusing the amount parser's own
    // vocabulary keeps the two from drifting apart. The known cost is the French fuel brand: a line
    // reading exactly "TOTAL" loses, though "TOTALENERGIES" survives because `has_word` matches
    // whole words only.
    if POSITIVE.iter().chain(NEGATIVE).any(|k| has_word(&folded, k)) {
        return false;
    }
    // An address line: "12 RUE DE LA PAIX", "75002 PARIS". A merchant name starting with a
    // number is rare enough to lose here.
    !folded.split_whitespace().next().is_some_and(|t| t.chars().all(|c| c.is_ascii_digit()))
}

fn longest_digit_run(text: &str) -> usize {
    let (mut best, mut run) = (0, 0);
    for c in text.chars() {
        run = if c.is_ascii_digit() { run + 1 } else { 0 };
        best = best.max(run);
    }
    best
}

/// Merchant names are printed larger and in capitals. That is the whole signal, and it is one
/// line of arithmetic against the alternatives.
fn title_score(line: &TextLine, median_h: f32) -> f32 {
    uppercase_fraction(&line.text) + line.h / median_h
}

fn uppercase_fraction(text: &str) -> f32 {
    let letters: Vec<char> = text.chars().filter(|c| c.is_alphabetic()).collect();
    if letters.is_empty() {
        return 0.0;
    }
    letters.iter().filter(|c| c.is_uppercase()).count() as f32 / letters.len() as f32
}

fn is_all_caps(text: &str) -> bool {
    uppercase_fraction(text) > 0.99
}

fn strip_legal_form(text: &str) -> String {
    let mut tokens: Vec<&str> =
        text.split_whitespace().map(|t| t.trim_matches(|c: char| !c.is_alphanumeric())).collect();
    tokens.retain(|t| !t.is_empty());
    let is_legal = |t: &&str| LEGAL_FORMS.contains(&fold(t).as_str());
    while tokens.first().is_some_and(is_legal) {
        tokens.remove(0);
    }
    while tokens.last().is_some_and(is_legal) {
        tokens.pop();
    }
    tokens.join(" ")
}

#[cfg(test)]
mod tests;

fn title_case(text: &str) -> String {
    text.split_whitespace()
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                Some(first) => {
                    first.to_uppercase().collect::<String>() + &chars.as_str().to_lowercase()
                }
                None => String::new(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}
