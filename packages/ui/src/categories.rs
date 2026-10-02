use std::collections::BTreeMap;
use std::sync::LazyLock;

use crate::tid;

use crate::decrypted::DecryptedExpense;

/// A leaf of the taxonomy. `id` keys the keyword files and is never persisted; `parent` is.
pub struct Category {
    pub id: &'static str,
    pub parent: &'static str,
    pub emoji: &'static str,
}

pub const DEFAULT_CATEGORY: Category = Category { id: "other", parent: "Autres", emoji: "💵" };

/// Declaration order breaks ties: "Gâteau d'anniversaire" is a dessert because Desserts comes
/// before Anniversaire. The first leaf of each parent is its emoji (`parent_emoji`).
pub const CATEGORIES: &[Category] = &[
    Category { id: "restaurants", parent: "Nourriture", emoji: "🍽️" },
    Category { id: "cafe", parent: "Nourriture", emoji: "☕" },
    Category { id: "pizza", parent: "Nourriture", emoji: "🍕" },
    Category { id: "burger", parent: "Nourriture", emoji: "🍔" },
    Category { id: "kebab", parent: "Nourriture", emoji: "🥙" },
    Category { id: "sushi", parent: "Nourriture", emoji: "🍣" },
    Category { id: "bar", parent: "Nourriture", emoji: "🍺" },
    Category { id: "bakery", parent: "Nourriture", emoji: "🥖" },
    Category { id: "groceries", parent: "Nourriture", emoji: "🛒" },
    Category { id: "desserts", parent: "Nourriture", emoji: "🍦" },
    Category { id: "taxi", parent: "Transport", emoji: "🚕" },
    Category { id: "fuel", parent: "Transport", emoji: "⛽" },
    Category { id: "train", parent: "Transport", emoji: "🚆" },
    Category { id: "plane", parent: "Transport", emoji: "✈️" },
    Category { id: "bus", parent: "Transport", emoji: "🚌" },
    Category { id: "ferry", parent: "Transport", emoji: "⛴️" },
    Category { id: "car", parent: "Transport", emoji: "🚗" },
    Category { id: "toll", parent: "Transport", emoji: "🛣️" },
    Category { id: "bike", parent: "Transport", emoji: "🚲" },
    Category { id: "parking", parent: "Transport", emoji: "🅿️" },
    Category { id: "hotel", parent: "Hébergement", emoji: "🏨" },
    Category { id: "rent", parent: "Hébergement", emoji: "🏠" },
    Category { id: "cinema", parent: "Loisirs", emoji: "🎬" },
    Category { id: "music", parent: "Loisirs", emoji: "🎵" },
    Category { id: "museum", parent: "Loisirs", emoji: "🏛️" },
    Category { id: "gaming", parent: "Loisirs", emoji: "🎮" },
    Category { id: "ski", parent: "Loisirs", emoji: "🎿" },
    Category { id: "sport", parent: "Loisirs", emoji: "⚽" },
    Category { id: "tickets", parent: "Loisirs", emoji: "🎟️" },
    Category { id: "shopping", parent: "Shopping", emoji: "🛍️" },
    Category { id: "phone", parent: "Shopping", emoji: "📱" },
    Category { id: "computer", parent: "Shopping", emoji: "💻" },
    Category { id: "books", parent: "Shopping", emoji: "📚" },
    Category { id: "internet", parent: "Services", emoji: "📡" },
    Category { id: "electricity", parent: "Services", emoji: "⚡" },
    Category { id: "heating", parent: "Services", emoji: "🔥" },
    Category { id: "water", parent: "Services", emoji: "💧" },
    Category { id: "insurance", parent: "Services", emoji: "🛡️" },
    Category { id: "health", parent: "Services", emoji: "🏥" },
    Category { id: "pharmacy", parent: "Services", emoji: "💊" },
    Category { id: "hairdresser", parent: "Services", emoji: "💇" },
    Category { id: "spa", parent: "Services", emoji: "💆" },
    Category { id: "pets", parent: "Services", emoji: "🐾" },
    Category { id: "childcare", parent: "Services", emoji: "🧸" },
    Category { id: "cleaning", parent: "Services", emoji: "🧹" },
    Category { id: "birthday", parent: "Fêtes & Cadeaux", emoji: "🎂" },
    Category { id: "christmas", parent: "Fêtes & Cadeaux", emoji: "🎄" },
    Category { id: "gift", parent: "Fêtes & Cadeaux", emoji: "🎁" },
];

/// Every locale's keywords, embedded on every target and matched as **one** table. With the
/// category left on "auto", each device re-infers it from the name: a table keyed on the UI
/// language would show two members of the same project two different charts.
const KEYWORD_FILES: &[(&str, &str)] = &[
    ("bg", include_str!("../categories/bg.txt")),
    ("bs", include_str!("../categories/bs.txt")),
    ("cs", include_str!("../categories/cs.txt")),
    ("da", include_str!("../categories/da.txt")),
    ("de", include_str!("../categories/de.txt")),
    ("el", include_str!("../categories/el.txt")),
    ("en", include_str!("../categories/en.txt")),
    ("es", include_str!("../categories/es.txt")),
    ("et", include_str!("../categories/et.txt")),
    ("fi", include_str!("../categories/fi.txt")),
    ("fr", include_str!("../categories/fr.txt")),
    ("ga", include_str!("../categories/ga.txt")),
    ("hr", include_str!("../categories/hr.txt")),
    ("hu", include_str!("../categories/hu.txt")),
    ("is", include_str!("../categories/is.txt")),
    ("it", include_str!("../categories/it.txt")),
    ("lt", include_str!("../categories/lt.txt")),
    ("lv", include_str!("../categories/lv.txt")),
    ("mk", include_str!("../categories/mk.txt")),
    ("mt", include_str!("../categories/mt.txt")),
    ("nl", include_str!("../categories/nl.txt")),
    ("no", include_str!("../categories/no.txt")),
    ("pl", include_str!("../categories/pl.txt")),
    ("pt", include_str!("../categories/pt.txt")),
    ("ro", include_str!("../categories/ro.txt")),
    ("sk", include_str!("../categories/sk.txt")),
    ("sl", include_str!("../categories/sl.txt")),
    ("sq", include_str!("../categories/sq.txt")),
    ("sr", include_str!("../categories/sr.txt")),
    ("sv", include_str!("../categories/sv.txt")),
    ("tr", include_str!("../categories/tr.txt")),
    ("uk", include_str!("../categories/uk.txt")),
];

/// The `_stop` line of a keyword file. Never stripped at runtime — stripping every language's
/// articles would eat other languages' keywords — only held against the keywords by the tests.
const STOP_KEY: &str = "_stop";

/// Below this a truncation is a real word of its own: "super" would be groceries, "inter" internet.
const MIN_TRUNCATION_CHARS: usize = 6;

/// `key = word, word, …` lines. Lenient on purpose: this runs on the render path, where a panic
/// freezes the app, so a malformed line is skipped here and failed by the tests instead.
fn entries(file: &str) -> impl Iterator<Item = (&str, &str)> {
    file.lines()
        .map(str::trim)
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
        .filter_map(|line| line.split_once('='))
        .flat_map(|(key, words)| words.split(',').map(move |word| (key.trim(), word.trim())))
        .filter(|(_, word)| !word.is_empty())
}

struct Index {
    /// Single-word keywords, sorted so a truncation's completions are one contiguous range.
    words: Vec<(String, usize)>,
    phrases: Vec<(Vec<String>, usize)>,
}

static INDEX: LazyLock<Index> = LazyLock::new(|| {
    let mut words = BTreeMap::new();
    let mut phrases = Vec::new();
    for (_, file) in KEYWORD_FILES {
        for (key, keyword) in entries(file) {
            let Some(category) = CATEGORIES.iter().position(|c| c.id == key) else { continue };
            match normalize(keyword).as_slice() {
                [] => {}
                [word] => {
                    let leaf = words.entry(word.clone()).or_insert(category);
                    *leaf = (*leaf).min(category);
                }
                phrase => phrases.push((phrase.to_vec(), category)),
            }
        }
    }
    Index { words: words.into_iter().collect(), phrases }
});

impl Index {
    fn exact(&self, word: &str) -> Option<usize> {
        self.words.binary_search_by(|(k, _)| k.as_str().cmp(word)).ok().map(|i| self.words[i].1)
    }

    fn completions<'a>(&'a self, prefix: &'a str) -> impl Iterator<Item = usize> + 'a {
        let start = self.words.partition_point(|(k, _)| k.as_str() < prefix);
        self.words[start..].iter().take_while(move |(k, _)| k.starts_with(prefix)).map(|(_, c)| *c)
    }
}

/// The display label for a chart category.
///
/// **`CHART_CATEGORIES` are stable ids, not labels.** `ExpensePayload.category` persists one of
/// these strings *inside the encrypted payload*, and `get_expense_category` groups on them, so
/// translating them in place would orphan every expense already categorised and break the charts.
/// Only the label moves; the id stays French forever.
pub fn category_label(id: &str) -> String {
    match id {
        "Nourriture" => tid!("category-food"),
        "Transport" => tid!("category-transport"),
        "Hébergement" => tid!("category-accommodation"),
        "Loisirs" => tid!("category-leisure"),
        "Shopping" => tid!("category-shopping"),
        "Services" => tid!("category-services"),
        "Fêtes & Cadeaux" => tid!("category-parties-gifts"),
        _ => tid!("category-other"),
    }
}

pub const CHART_CATEGORIES: &[&str] = &[
    "Nourriture",
    "Transport",
    "Hébergement",
    "Loisirs",
    "Shopping",
    "Services",
    "Fêtes & Cadeaux",
    "Autres",
];

/// Text into whole words — lowercase, diacritics folded for every shipped script, split on
/// non-alphanumerics ("l'eau", "taxi-moto", "Uber (aéroport)"). Whole words, not `contains`, or
/// "cadeau" is a glass of water ("eau"). Plurals belong to `word_matches`: stripping the "s" here
/// would turn "repas" into "repa".
fn normalize(text: &str) -> Vec<String> {
    let folded: String = text
        .to_lowercase()
        .replace('œ', "oe")
        .replace('æ', "ae")
        .replace('ß', "ss")
        .replace('þ', "th")
        .chars()
        .filter_map(|c| match c {
            // Combining marks: "İ" lowercases to "i" + U+0307, which must not split the word.
            '\u{300}'..='\u{36f}' => None,
            'á' | 'à' | 'â' | 'ä' | 'ã' | 'å' | 'ā' | 'ą' | 'ă' => Some('a'),
            'ç' | 'ć' | 'č' | 'ċ' => Some('c'),
            'ď' | 'đ' | 'ð' => Some('d'),
            'é' | 'è' | 'ê' | 'ë' | 'ē' | 'ę' | 'ė' | 'ě' => Some('e'),
            'ğ' | 'ģ' | 'ġ' => Some('g'),
            'ħ' => Some('h'),
            'í' | 'ì' | 'î' | 'ï' | 'ī' | 'į' | 'ı' => Some('i'),
            'ķ' => Some('k'),
            'ł' | 'ļ' | 'ľ' | 'ĺ' => Some('l'),
            'ñ' | 'ń' | 'ň' | 'ņ' => Some('n'),
            'ó' | 'ò' | 'ô' | 'ö' | 'õ' | 'ő' | 'ø' => Some('o'),
            'ř' | 'ŕ' => Some('r'),
            'ś' | 'š' | 'ş' | 'ș' => Some('s'),
            'ť' | 'ţ' | 'ț' => Some('t'),
            'ú' | 'ù' | 'û' | 'ü' | 'ū' | 'ů' | 'ű' | 'ų' => Some('u'),
            'ý' | 'ÿ' => Some('y'),
            'ź' | 'ż' | 'ž' => Some('z'),
            'ά' => Some('α'),
            'έ' => Some('ε'),
            'ή' => Some('η'),
            'ί' | 'ϊ' | 'ΐ' => Some('ι'),
            'ό' => Some('ο'),
            'ύ' | 'ϋ' | 'ΰ' => Some('υ'),
            'ώ' => Some('ω'),
            'ς' => Some('σ'),
            'ё' | 'ѐ' => Some('е'),
            'ѝ' => Some('и'),
            c if c.is_alphanumeric() => Some(c),
            _ => Some(' '),
        })
        .collect();

    folded.split_whitespace().map(str::to_string).collect()
}

/// The token as typed, then de-pluralized ("cadeaux" is "cadeau"). Only the token is, so keywords
/// merely ending in "s" ("repas", "starbucks") stay whole words.
fn singulars(token: &str) -> impl Iterator<Item = &str> {
    [Some(token), token.strip_suffix('s'), token.strip_suffix('x')].into_iter().flatten()
}

/// Tolerates a plural and a word cut short ("restau" is "restaurant"). A short keyword never
/// swallows a longer word: only the token is truncated.
fn word_matches(token: &str, keyword: &str) -> bool {
    singulars(token).any(|t| {
        t == keyword || (t.chars().count() >= MIN_TRUNCATION_CHARS && keyword.starts_with(t))
    })
}

/// Phrases, then whole words, then truncations — each level only when the previous found nothing,
/// so "food shopping" is groceries although "food" alone is a restaurant. Within a level the leaf
/// declared first wins.
pub fn find_category(name: &str) -> &'static Category {
    let tokens = normalize(name);
    let index = &*INDEX;
    let phrase = || {
        index
            .phrases
            .iter()
            .filter(|(words, _)| {
                tokens
                    .windows(words.len())
                    .any(|w| w.iter().zip(words).all(|(t, k)| word_matches(t, k)))
            })
            .map(|(_, c)| *c)
            .min()
    };
    let exact = || tokens.iter().flat_map(|t| singulars(t)).filter_map(|t| index.exact(t)).min();
    let truncated = || {
        tokens
            .iter()
            .flat_map(|t| singulars(t))
            .filter(|t| t.chars().count() >= MIN_TRUNCATION_CHARS)
            .flat_map(|t| index.completions(t))
            .min()
    };
    phrase().or_else(exact).or_else(truncated).map_or(&DEFAULT_CATEGORY, |i| &CATEGORIES[i])
}

pub fn get_expense_emoji(name: &str) -> &'static str {
    find_category(name).emoji
}

pub fn infer_chart_category(name: &str) -> &'static str {
    find_category(name).parent
}

/// What the user picked while it is still in `CHART_CATEGORIES`, else what the name says.
pub fn get_expense_category(expense: &DecryptedExpense) -> &'static str {
    if let Some(ref cat) = expense.category {
        if let Some(&found) = CHART_CATEGORIES.iter().find(|&&c| c == cat.as_str()) {
            return found;
        }
    }
    infer_chart_category(&expense.name)
}

/// The first child of each parent is its most representative one, so no second table is needed.
pub fn parent_emoji(parent: &str) -> &'static str {
    CATEGORIES
        .iter()
        .find(|c| c.parent == parent)
        .map(|c| c.emoji)
        .unwrap_or(DEFAULT_CATEGORY.emoji)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::{BTreeSet, HashMap};

    fn keywords(file: &str) -> impl Iterator<Item = (&str, String)> {
        entries(file).filter(|(key, _)| *key != STOP_KEY).map(|(key, k)| (key, normalize(k).join(" ")))
    }

    fn stop_words() -> HashMap<String, &'static str> {
        KEYWORD_FILES
            .iter()
            .flat_map(|(lang, file)| {
                entries(file).filter(|(key, _)| *key == STOP_KEY).map(move |(_, w)| (normalize(w).join(" "), *lang))
            })
            .collect()
    }

    #[test]
    fn every_chart_category_has_an_emoji() {
        for cat in CHART_CATEGORIES {
            assert!(!parent_emoji(cat).is_empty(), "no emoji for {cat}");
        }
        assert_eq!(parent_emoji("Nourriture"), "🍽️");
        assert_eq!(parent_emoji("Transport"), "🚕");
        // "Autres" has no CATEGORIES entry — it falls back to the default.
        assert_eq!(parent_emoji("Autres"), DEFAULT_CATEGORY.emoji);
    }

    #[test]
    fn every_parent_is_a_chart_category_and_every_id_is_unique() {
        let mut ids = BTreeSet::new();
        for cat in CATEGORIES {
            assert!(CHART_CATEGORIES.contains(&cat.parent), "{} has parent {}", cat.id, cat.parent);
            assert!(ids.insert(cat.id), "duplicate id {}", cat.id);
        }
    }

    #[test]
    fn keyword_files_are_exactly_the_shipped_locales() {
        let files: BTreeSet<&str> = KEYWORD_FILES.iter().map(|(code, _)| *code).collect();
        let shipped: BTreeSet<&str> = crate::i18n::SUPPORTED.iter().map(|(code, _)| *code).collect();
        assert_eq!(files, shipped);
        assert_eq!(files.len(), KEYWORD_FILES.len(), "a locale is listed twice");
    }

    /// `entries` skips what it cannot read; this is where a typo is caught.
    #[test]
    fn every_line_is_a_known_key_with_words() {
        for (lang, file) in KEYWORD_FILES {
            for line in file.lines().map(str::trim).filter(|l| !l.is_empty() && !l.starts_with('#')) {
                let (key, words) = line.split_once('=').unwrap_or_else(|| panic!("{lang}: no '=' in {line:?}"));
                let key = key.trim();
                assert!(
                    key == STOP_KEY || CATEGORIES.iter().any(|c| c.id == key),
                    "{lang}: unknown key {key:?}"
                );
                for word in words.split(',') {
                    assert!(!normalize(word).is_empty(), "{lang}: empty word in {line:?}");
                }
            }
        }
    }

    #[test]
    fn every_leaf_has_keywords_in_every_locale() {
        for (lang, file) in KEYWORD_FILES {
            let keys: BTreeSet<&str> = keywords(file).map(|(key, _)| key).collect();
            for cat in CATEGORIES {
                assert!(keys.contains(cat.id), "{lang}: no keyword for {}", cat.id);
            }
        }
    }

    /// The table is the union of every language, so a word two languages file under different
    /// leaves would categorise by declaration order — silently wrong for one of them.
    #[test]
    fn a_keyword_belongs_to_one_leaf_across_every_language() {
        let mut owner: HashMap<String, (&str, &str)> = HashMap::new();
        let mut clashes = Vec::new();
        for (lang, file) in KEYWORD_FILES {
            for (key, keyword) in keywords(file) {
                match owner.get(&keyword) {
                    Some((other_key, other_lang)) if *other_key != key => {
                        clashes.push(format!("{keyword:?}: {other_lang} {other_key} / {lang} {key}"))
                    }
                    Some(_) => {}
                    None => {
                        owner.insert(keyword, (key, lang));
                    }
                }
            }
        }
        assert!(clashes.is_empty(), "keywords in two leaves:\n{}", clashes.join("\n"));
    }

    /// "thé" folds to "the": a keyword that is some language's article categorises every title
    /// written in that language.
    #[test]
    fn no_keyword_is_a_stop_word_of_any_language() {
        let stops = stop_words();
        let mut hits = Vec::new();
        for (lang, file) in KEYWORD_FILES {
            for (key, keyword) in keywords(file).filter(|(_, k)| !k.contains(' ')) {
                if let Some(stop_lang) = stops.get(&keyword) {
                    hits.push(format!("{lang} {key} {keyword:?} is a {stop_lang} stop word"));
                }
                for stop in stops.keys().filter(|s| s.chars().count() >= MIN_TRUNCATION_CHARS) {
                    if keyword.starts_with(stop.as_str()) {
                        hits.push(format!("{lang} {key} {keyword:?} completes the stop word {stop:?}"));
                    }
                }
            }
        }
        assert!(hits.is_empty(), "{}", hits.join("\n"));
    }

    #[test]
    fn normalize_folds_accents_and_case() {
        assert_eq!(normalize("CAFÉ"), ["cafe"]);
        assert_eq!(normalize("Hôtel"), ["hotel"]);
        assert_eq!(normalize("Bière"), ["biere"]);
        assert_eq!(normalize("Électricité"), ["electricite"]);
    }

    #[test]
    fn normalize_folds_every_shipped_script() {
        assert_eq!(normalize("Straße"), ["strasse"]);
        assert_eq!(normalize("Łódź"), ["lodz"]);
        assert_eq!(normalize("Șosea Țară"), ["sosea", "tara"]);
        assert_eq!(normalize("Kőbánya Ůžasný"), ["kobanya", "uzasny"]);
        assert_eq!(normalize("İSTANBUL ılık"), ["istanbul", "ilik"]);
        assert_eq!(normalize("ΤΑΒΈΡΝΑΣ"), ["ταβερνασ"]);
        assert_eq!(normalize("Ёлка"), ["елка"]);
        assert_eq!(normalize("Þingvellir"), ["thingvellir"]);
    }

    #[test]
    fn normalize_splits_on_punctuation() {
        assert_eq!(normalize("l'eau"), ["l", "eau"]);
        assert_eq!(normalize("taxi-moto"), ["taxi", "moto"]);
        assert_eq!(normalize("Uber (aéroport)"), ["uber", "aeroport"]);
        assert_eq!(normalize("Resto, bar"), ["resto", "bar"]);
    }

    #[test]
    fn the_english_article_is_not_tea() {
        assert_eq!(get_expense_emoji("The Ivy"), "💵");
        assert_eq!(get_expense_emoji("Hotel du Nord"), "🏨");
    }

    /// "repas" and "starbucks" are whole words, not plurals of "repa" and "starbuck".
    #[test]
    fn a_plural_matches_its_singular_keyword() {
        assert!(word_matches("cadeaux", "cadeau"));
        assert!(word_matches("tickets", "ticket"));
        assert!(word_matches("repas", "repas"));
        assert!(word_matches("starbucks", "starbucks"));
        assert!(!word_matches("business", "bus"));
        assert!(!word_matches("cadeau", "eau"));
    }

    #[test]
    fn a_short_truncation_is_not_a_match() {
        assert!(!word_matches("super", "supermarket"));
        assert!(!word_matches("inter", "internet"));
        assert!(!word_matches("training", "train"));
        assert!(!word_matches("marketing", "market"));
    }

    #[test]
    fn a_truncated_word_matches_its_keyword() {
        assert_eq!(get_expense_emoji("Restau"), "🍽️");
        assert_eq!(get_expense_emoji("restaus midi"), "🍽️");
        assert_eq!(get_expense_emoji("pharma"), "💊");
        assert_eq!(get_expense_emoji("supermarch"), "🛒");
    }

    #[test]
    fn a_whole_word_beats_a_truncation() {
        assert_eq!(get_expense_emoji("restaura cinéma"), "🎬");
    }

    /// Each of these once matched a keyword buried inside a longer word.
    #[test]
    fn a_keyword_inside_a_word_is_not_a_match() {
        assert_eq!(get_expense_emoji("cadeau"), "🎁"); // was 💧, via "eau"
        assert_ne!(get_expense_emoji("bateau"), "💧");
        assert_eq!(get_expense_emoji("transport"), "💵"); // was ⚽, via "sport"
        assert_eq!(get_expense_emoji("parent"), "💵"); // was 🏠, via "rent"
        assert_eq!(get_expense_emoji("carte SNCF"), "🚆"); // "carte" was 🚗, via "car"
        assert_eq!(get_expense_emoji("magasin"), "💵"); // was ⛽, via "gas"
        assert_eq!(get_expense_emoji("steak"), "💵"); // was ☕, via "tea"
        assert_eq!(get_expense_emoji("jeudi soir"), "💵"); // was 🎮, via "jeu"
        assert_eq!(get_expense_emoji("business"), "💵"); // was 🚌, via "bus"
        assert_eq!(get_expense_emoji("spaghetti"), "💵"); // was 💆, via "spa"
    }

    /// Words common in French and English titles that another language's keyword could claim.
    #[test]
    fn everyday_titles_stay_uncategorised() {
        for name in ["carte", "carte bleue", "total", "remboursement", "virement", "super", "divers"] {
            assert_eq!(get_expense_emoji(name), "💵", "{name}");
        }
    }

    #[test]
    fn a_phrase_beats_a_single_word() {
        assert_eq!(get_expense_emoji("food shopping"), "🛒");
        assert_eq!(get_expense_emoji("food"), "🍽️");
        assert_eq!(get_expense_emoji("ice cream"), "🍦");
        assert_eq!(get_expense_emoji("forfait de ski"), "🎿");
        assert_eq!(get_expense_emoji("salon de thé"), "☕");
        assert_eq!(get_expense_emoji("car park"), "🅿️");
    }

    #[test]
    fn accents_and_plurals_still_match() {
        assert_eq!(get_expense_emoji("Café"), "☕");
        assert_eq!(get_expense_emoji("cafe"), "☕");
        assert_eq!(get_expense_emoji("Cadeaux de Noël"), "🎄");
        assert_eq!(get_expense_emoji("2 tickets"), "🎟️");
        assert_eq!(get_expense_emoji("Courses Carrefour"), "🛒");
        assert_eq!(get_expense_emoji("Bière & frites"), "🍺");
    }

    /// Two or three real titles per shipped language.
    #[test]
    fn every_language_categorises_its_own_titles() {
        let samples: &[(&str, &str, &str)] = &[
            ("bg", "Ресторант", "🍽️"),
            ("bg", "Бензин", "⛽"),
            ("bs", "Restoran", "🍽️"),
            ("bs", "Gorivo", "⛽"),
            ("cs", "Kavárna", "☕"),
            ("cs", "Nájem", "🏠"),
            ("da", "Husleje", "🏠"),
            ("da", "Indkøb", "🛒"),
            ("de", "Tankstelle", "⛽"),
            ("de", "Miete", "🏠"),
            ("el", "Ταβέρνα", "🍽️"),
            ("el", "Σούπερ μάρκετ", "🛒"),
            ("en", "Gas bill", "🔥"),
            ("es", "Gasolina", "⛽"),
            ("es", "Alquiler", "🏠"),
            ("et", "Kohvik", "☕"),
            ("et", "Üür", "🏠"),
            ("fi", "Ravintola", "🍽️"),
            ("fi", "Vuokra", "🏠"),
            ("fr", "Péage autoroute", "🛣️"),
            ("ga", "Bialann", "🍽️"),
            ("hr", "Najam", "🏠"),
            ("hr", "Rođendan", "🎂"),
            ("hu", "Étterem", "🍽️"),
            ("hu", "Benzinkút", "⛽"),
            ("is", "Veitingastaður", "🍽️"),
            ("it", "Benzina", "⛽"),
            ("it", "Affitto", "🏠"),
            ("lt", "Kavinė", "☕"),
            ("lt", "Nuoma", "🏠"),
            ("lv", "Kafejnīca", "☕"),
            ("lv", "Degviela", "⛽"),
            ("mk", "Ресторан", "🍽️"),
            ("mk", "Кирија", "🏠"),
            ("mt", "Ristorant", "🍽️"),
            ("nl", "Boodschappen", "🛒"),
            ("nl", "Huur", "🏠"),
            ("no", "Husleie", "🏠"),
            ("no", "Dagligvarer", "🛒"),
            ("pl", "Kawiarnia", "☕"),
            ("pl", "Czynsz", "🏠"),
            ("pt", "Gasolina", "⛽"),
            ("pt", "Aluguel", "🏠"),
            ("ro", "Chirie", "🏠"),
            ("ro", "Benzinărie", "⛽"),
            ("sk", "Nájomné", "🏠"),
            ("sk", "Potraviny", "🛒"),
            ("sl", "Najemnina", "🏠"),
            ("sl", "Gostilna", "🍽️"),
            ("sq", "Qira", "🏠"),
            ("sq", "Karburant", "⛽"),
            ("sr", "Кирија", "🏠"),
            ("sr", "Kirija", "🏠"),
            ("sv", "Hyra", "🏠"),
            ("sv", "Matbutik", "🛒"),
            ("tr", "Kira", "🏠"),
            ("tr", "Benzin", "⛽"),
            ("uk", "Ресторан", "🍽️"),
            ("uk", "Оренда", "🏠"),
        ];
        let mut wrong = Vec::new();
        for (lang, name, emoji) in samples {
            let got = get_expense_emoji(name);
            if got != *emoji {
                wrong.push(format!("{lang} {name:?}: {got} instead of {emoji}"));
            }
        }
        assert!(wrong.is_empty(), "{}", wrong.join("\n"));
    }
}
