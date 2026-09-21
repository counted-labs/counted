use super::*;

fn day(y: i32, m: u32, d: u32) -> NaiveDate {
    NaiveDate::from_ymd_opt(y, m, d).unwrap()
}

fn today() -> NaiveDate {
    day(2026, 9, 4)
}

/// One line at a vertical position. `x`/`h`/`confidence` are the boring case; the tests that care
/// about them build their own.
fn l(text: &str, y: f32) -> TextLine {
    TextLine { text: text.into(), x: 0.1, y, h: 0.02, confidence: 0.9 }
}

/// A whole supermarket ticket, in the shape the recogniser hands over.
fn supermarket() -> Vec<TextLine> {
    vec![
        l("CARREFOUR MARKET", 0.04),
        l("12 RUE DE LA PAIX", 0.09),
        l("75002 PARIS", 0.12),
        l("TEL 01 42 33 44 55", 0.15),
        l("PAIN COMPLET  2,40", 0.40),
        l("LAIT DEMI 1L  1,15", 0.44),
        l("SOUS-TOTAL   12,05", 0.70),
        l("TVA 5,5%      0,85", 0.74),
        l("TOTAL        12,90", 0.78),
        l("ESPECES      20,00", 0.83),
        l("RENDU         7,10", 0.86),
        l("04/09/2026 14:32", 0.93),
    ]
}

fn values(s: &str) -> Vec<f64> {
    money_tokens(s).iter().map(|m| m.value).collect()
}

// ─── amount ──────────────────────────────────────────────────────────────────

#[test]
fn total_wins_over_subtotal() {
    assert_eq!(find_amount(&supermarket()), Some((12.90, true)));
}

#[test]
fn tva_line_is_never_the_total() {
    let lines = vec![l("TVA 5,5%      0,85", 0.74), l("TOTAL        12,90", 0.78)];
    assert_eq!(find_amount(&lines), Some((12.90, true)));
}

/// The classic misread: the cash tendered is larger than the total and the change is printed last,
/// so both a "largest number" and a "last number" heuristic get it wrong.
#[test]
fn rendu_and_especes_are_rejected() {
    let lines = vec![
        l("TOTAL        12,90", 0.78),
        l("ESPECES      20,00", 0.83),
        l("RENDU         7,10", 0.86),
    ];
    assert_eq!(find_amount(&lines), Some((12.90, true)));
}

#[test]
fn amount_on_the_line_below_its_anchor() {
    let lines = vec![l("NET A PAYER", 0.70), l("42,00 EUR", 0.73)];
    assert_eq!(find_amount(&lines), Some((42.0, true)));
}

#[test]
fn an_anchor_reaches_no_further_than_its_own_line_height() {
    let lines = vec![l("TOTAL", 0.20), l("99,00", 0.90)];
    assert_eq!(find_amount(&lines), Some((99.0, false)), "must fall back, not reach down 70%");
}

#[test]
fn the_lowest_total_anchor_wins() {
    let lines = vec![l("TOTAL ARTICLES 7", 0.30), l("TOTAL 31,45", 0.80)];
    assert_eq!(find_amount(&lines), Some((31.45, true)));
}

#[test]
fn summe_gesamt_totale_importe_totaal_are_anchors() {
    for (text, expected) in [
        ("SUMME 12,90", 12.90),
        ("GESAMTBETRAG 8,40", 8.40),
        ("TOTALE 15,00", 15.00),
        ("IMPORTE TOTAL 9,99", 9.99),
        ("TOTAAL 4,50", 4.50),
        ("TE BETALEN 6,25", 6.25),
        ("TOTAL A PAGAR 3,10", 3.10),
    ] {
        assert_eq!(find_amount(&[l(text, 0.8)]), Some((expected, true)), "{text}");
    }
}

#[test]
fn zwischensumme_and_subtotaal_are_rejected() {
    let lines = vec![l("ZWISCHENSUMME 10,00", 0.70), l("SUMME 12,90", 0.78)];
    assert_eq!(find_amount(&lines), Some((12.90, true)));
}

#[test]
fn no_anchor_falls_back_to_the_bottom_third_and_is_not_confident() {
    let lines = vec![l("PAIN 2,40", 0.20), l("18,60", 0.88)];
    assert_eq!(find_amount(&lines), Some((18.60, false)));
}

/// Without the date/time guard the fallback returns the year off the timestamp line.
#[test]
fn the_fallback_never_returns_a_year_or_a_clock_time() {
    let lines = vec![l("04/09/2026 14:32", 0.93), l("12,90", 0.80)];
    assert_eq!(find_amount(&lines), Some((12.90, false)));
}

/// The tail of the Hyper U fixture, as the recogniser hands it over: the amount paid sits under
/// "RESTE A PAYER" (misread "PAVER"), and three other "TOTAL" lines compete with it — the article
/// total before discount, the discount line, and the VAT table's last row printed *below* it.
fn hyper_u_tail() -> Vec<TextLine> {
    vec![
        l("TOTAL [156] Articles 709,91 €", 0.623),
        l("SOUS-TOTAL 670,12 €", 0.640),
        l("TOTAL TUA 39.79 €", 0.650),
        l("TOTAL REDUCTIONS INNEDIATES -2.00 e", 0.698),
        l("RESTE A PAVER", 0.714),
        l("CB 707,91 €", 0.722),
        l("CB SANS CONTACT 350.00 €", 0.731),
        l("TOTAL 39,79 € 670,12 € 709,91 €", 0.785),
    ]
}

#[test]
fn reste_a_payer_wins_over_the_vat_table_total() {
    assert_eq!(find_amount(&hyper_u_tail()), Some((707.91, true)));
}

#[test]
fn a_total_line_with_three_prices_is_the_vat_table() {
    let lines = vec![l("TOTAL 12,90", 0.70), l("TOTAL 0,85 € 12,05 € 12,90 €", 0.80)];
    assert_eq!(find_amount(&lines), Some((12.90, true)));
    assert_eq!(find_amount(&lines[1..]), Some((12.90, false)), "the table alone is only a fallback");
}

#[test]
fn a_discount_total_and_a_misread_tva_are_rejected() {
    let lines = vec![
        l("TOTAL 12,90", 0.70),
        l("TOTAL TUA 0,85", 0.75),
        l("TOTAL REDUCTIONS IMMEDIATES -2,00", 0.80),
    ];
    assert_eq!(find_amount(&lines), Some((12.90, true)));
}

#[test]
fn an_empty_receipt_yields_all_none() {
    assert_eq!(parse_receipt(&[], today()), Receipt::default());
}

#[test]
fn a_receipt_with_no_date_still_yields_the_amount() {
    let r = parse_receipt(&[l("BOULANGERIE DUPONT", 0.05), l("TOTAL 7,80", 0.80)], today());
    assert_eq!(r.amount, Some(7.80));
    assert!(r.amount_confident);
    assert_eq!(r.date, None);
}

// ─── money tokens ────────────────────────────────────────────────────────────

#[test]
fn french_thousands_are_a_space_and_a_comma() {
    assert_eq!(values("TOTAL 1 234,56"), vec![1234.56]);
}

#[test]
fn english_thousands_are_a_comma_and_a_dot() {
    assert_eq!(values("TOTAL 1,234.56"), vec![1234.56]);
}

#[test]
fn german_thousands_are_a_dot_and_a_comma() {
    assert_eq!(values("SUMME 1.234,56"), vec![1234.56]);
}

#[test]
fn swiss_thousands_are_an_apostrophe() {
    assert_eq!(values("TOTAL 1\u{27}234.55"), vec![1234.55]);
}

/// The space rule must not swallow the next price. Two figures on one line stay two figures.
#[test]
fn two_prices_on_one_line_are_two_numbers() {
    assert_eq!(values("12,90   20,00"), vec![12.90, 20.00]);
    assert_eq!(values("12,90   200,00"), vec![12.90, 200.00]);
}

#[test]
fn a_vat_percentage_is_not_money() {
    assert_eq!(values("TVA 5,5% 0,85"), vec![0.85]);
    assert_eq!(values("TVA 20 % 1,60"), vec![1.60]);
}

#[test]
fn a_quantity_multiplier_is_not_money() {
    assert_eq!(values("2 x 4,50"), vec![4.50]);
}

#[test]
fn a_date_or_a_time_is_not_money() {
    assert!(values("04/09/2026 14:32").is_empty());
    assert!(values("2026-09-04").is_empty());
}

#[test]
fn a_lone_group_separator_is_not_a_decimal_point() {
    assert_eq!(values("1.234"), vec![1234.0]);
    assert_eq!(values("1,234"), vec![1234.0]);
}

#[test]
fn a_single_decimal_digit_still_parses() {
    assert_eq!(values("TOTAL 12,5"), vec![12.5]);
}

#[test]
fn implausible_values_are_dropped() {
    assert!(values("0,00").is_empty());
    assert!(values("999999,99").is_empty());
}

#[test]
fn only_priced_tokens_reach_the_fallback() {
    let lines = vec![l("ARTICLES 7", 0.80), l("18,60", 0.85)];
    assert_eq!(find_amount(&lines), Some((18.60, false)));
}

// ─── date ────────────────────────────────────────────────────────────────────

#[test]
fn date_slash_dot_dash_and_iso() {
    for text in ["04/09/2026", "04.09.2026", "04-09-2026", "2026-09-04"] {
        assert_eq!(find_date(&[l(text, 0.9)], today()), Some(day(2026, 9, 4)), "{text}");
    }
}

#[test]
fn a_two_digit_year_is_this_century() {
    assert_eq!(find_date(&[l("04/09/26", 0.9)], today()), Some(day(2026, 9, 4)));
}

#[test]
fn an_ambiguous_date_is_read_day_first() {
    assert_eq!(find_date(&[l("03/04/2026", 0.9)], today()), Some(day(2026, 4, 3)));
}

#[test]
fn a_day_over_twelve_forces_month_second() {
    assert_eq!(find_date(&[l("25/03/2026", 0.9)], today()), Some(day(2026, 3, 25)));
}

/// A US receipt prints month first, and the only thing that can reveal it is a second field above
/// twelve.
#[test]
fn a_second_field_over_twelve_is_an_american_receipt() {
    assert_eq!(find_date(&[l("03/25/2026", 0.9)], today()), Some(day(2026, 3, 25)));
}

#[test]
fn month_names_in_all_seven_locales() {
    for text in [
        "4 septembre 2026",
        "4 September 2026",
        "4 septiembre 2026",
        "4 settembre 2026",
        "4 setembro 2026",
        "4 september 2026",
    ] {
        assert_eq!(find_date(&[l(text, 0.9)], today()), Some(day(2026, 9, 4)), "{text}");
    }
}

#[test]
fn a_month_first_name_form_parses() {
    assert_eq!(find_date(&[l("September 4, 2026", 0.9)], today()), Some(day(2026, 9, 4)));
}

#[test]
fn an_abbreviated_month_parses() {
    assert_eq!(find_date(&[l("4 sept 2026", 0.9)], today()), Some(day(2026, 9, 4)));
}

#[test]
fn a_future_date_is_rejected() {
    assert_eq!(find_date(&[l("04/09/2027", 0.9)], today()), None);
}

#[test]
fn a_date_older_than_two_years_is_rejected() {
    assert_eq!(find_date(&[l("04/09/2020", 0.9)], today()), None);
}

#[test]
fn an_impossible_date_is_rejected() {
    assert_eq!(find_date(&[l("32/13/2026", 0.9)], today()), None);
}

#[test]
fn a_time_is_not_a_date() {
    assert_eq!(find_date(&[l("14:32:07", 0.9)], today()), None);
}

// ─── title ───────────────────────────────────────────────────────────────────

#[test]
fn title_is_the_merchant_not_the_address() {
    assert_eq!(find_title(&supermarket()).as_deref(), Some("Carrefour Market"));
}

#[test]
fn title_is_not_a_phone_or_a_siret() {
    let lines = vec![
        l("TEL 01 42 33 44 55", 0.05),
        l("SIRET 39525589700018", 0.08),
        l("BOULANGERIE DUPONT", 0.11),
    ];
    assert_eq!(find_title(&lines).as_deref(), Some("Boulangerie Dupont"));
}

#[test]
fn title_drops_the_legal_form() {
    assert_eq!(find_title(&[l("SARL LE PETIT CAFE", 0.05)]).as_deref(), Some("Le Petit Cafe"));
    assert_eq!(find_title(&[l("PIZZERIA ROMA GMBH", 0.05)]).as_deref(), Some("Pizzeria Roma"));
}

#[test]
fn a_mixed_case_title_is_left_alone() {
    assert_eq!(find_title(&[l("Le Pain Quotidien", 0.05)]).as_deref(), Some("Le Pain Quotidien"));
}

/// The header is set larger than the body; that is what separates it from the address.
#[test]
fn the_larger_header_line_wins() {
    let lines = vec![
        TextLine { text: "MONOPRIX".into(), x: 0.1, y: 0.04, h: 0.05, confidence: 0.9 },
        TextLine { text: "OUVERT DIMANCHE".into(), x: 0.1, y: 0.10, h: 0.02, confidence: 0.9 },
    ];
    assert_eq!(find_title(&lines).as_deref(), Some("Monoprix"));
}

#[test]
fn a_low_confidence_line_is_never_the_title() {
    let lines = vec![
        TextLine { text: "RRBBFF".into(), x: 0.1, y: 0.03, h: 0.09, confidence: 0.2 },
        TextLine { text: "INTERMARCHE".into(), x: 0.1, y: 0.08, h: 0.02, confidence: 0.9 },
    ];
    assert_eq!(find_title(&lines).as_deref(), Some("Intermarche"));
}

#[test]
fn a_line_below_the_header_zone_is_never_the_title() {
    assert_eq!(find_title(&[l("TOTAL 12,90", 0.80)]), None);
}

/// The bug this whole change exists for, measured in `tests/framing.rs`: pad the fixture with 30%
/// background above the receipt and the header is still detected and recognised perfectly — at
/// confidence 1.00 — but lands at y=0.280 and was discarded for sitting 0.03 outside a window
/// measured against the *image*. Every y here is past the old 0.25 cutoff; the header is still the
/// top of the receipt's own text, which is the only thing that should matter.
#[test]
fn a_header_pushed_down_by_background_is_still_the_title() {
    let lines = vec![
        TextLine { text: "CARREFOUR MARKET".into(), x: 0.1, y: 0.28, h: 0.029, confidence: 1.0 },
        l("12 RUE DE LA PAIX", 0.33),
        l("75002 PARIS", 0.35),
        l("PAIN COMPLET 2,40", 0.43),
        l("TOTAL 12,90", 0.62),
    ];
    assert_eq!(find_title(&lines).as_deref(), Some("Carrefour Market"));
}

/// The widened second pass must not let a priced line outrank the header — the reason
/// `is_title_candidate` rejects anything carrying a decimal amount.
#[test]
fn an_item_line_never_outranks_the_header() {
    let lines = vec![
        TextLine { text: "SPAR".into(), x: 0.1, y: 0.30, h: 0.02, confidence: 0.9 },
        // Deliberately the largest line on the page, so only the money rule can exclude it.
        TextLine { text: "CAFE MOULU 250G 4,80".into(), x: 0.1, y: 0.34, h: 0.09, confidence: 1.0 },
    ];
    assert_eq!(find_title(&lines).as_deref(), Some("Spar"));
}

/// Both zones empty — a logo block, then the name far down the receipt. Reading order is the SROIE
/// baseline for the company field, and a plausible title beats the blank the user saw before.
#[test]
fn a_name_below_both_zones_is_still_preferred_to_nothing() {
    let lines = vec![
        l("TICKET DE CAISSE", 0.10),
        l("TEL 01 42 33 44 55", 0.20),
        l("BOULANGERIE MARTIN", 0.70),
    ];
    assert_eq!(find_title(&lines).as_deref(), Some("Boulangerie Martin"));
}

/// Digits in a merchant name are not a price. `has_cents` is the discriminator, so an integer in
/// the name survives while "2,40" does not.
#[test]
fn digits_in_a_merchant_name_are_not_a_price() {
    assert_eq!(find_title(&[l("MAGASIN 24", 0.05)]).as_deref(), Some("Magasin 24"));
}

#[test]
fn a_welcome_banner_is_not_a_merchant() {
    let lines = vec![l("BIENVENUE CHEZ VOUS", 0.03), l("FRANPRIX", 0.09)];
    assert_eq!(find_title(&lines).as_deref(), Some("Franprix"));
}

// ─── folding ─────────────────────────────────────────────────────────────────

#[test]
fn folding_strips_accents_and_punctuation() {
    assert_eq!(fold("SOUS-TOTAL"), "sous total");
    assert_eq!(fold("NET \u{c0} PAYER"), "net a payer");
    assert_eq!(fold("Caf\u{e9}  Cr\u{e8}me!"), "cafe creme");
    assert_eq!(fold("Stra\u{df}e"), "strasse");
}

#[test]
fn word_matching_does_not_fire_inside_a_longer_word() {
    assert!(has_word("total ttc", "total"));
    assert!(!has_word("totalisateur", "total"));
}

// ─── end to end ──────────────────────────────────────────────────────────────

#[test]
fn a_whole_supermarket_ticket_parses() {
    let r = parse_receipt(&supermarket(), today());
    assert_eq!(r.title.as_deref(), Some("Carrefour Market"));
    assert_eq!(r.amount, Some(12.90));
    assert_eq!(r.date, Some(day(2026, 9, 4)));
    assert!(r.amount_confident);
}

/// The recogniser drops the space between a date and the time printed beside it, so the year runs
/// straight into the hour. Observed on the very first end-to-end fixture; before this, every
/// timestamped receipt lost its date.
#[test]
fn a_time_merged_onto_the_year_still_parses() {
    assert_eq!(find_date(&[l("04/09/202614:32", 0.9)], today()), Some(day(2026, 9, 4)));
}

/// The other half of that relaxation: the day and month fields stay strict, so a run of digits
/// cannot masquerade as a date.
#[test]
fn a_long_digit_run_is_still_not_a_date() {
    assert_eq!(find_date(&[l("123/45/6789", 0.9)], today()), None);
}
