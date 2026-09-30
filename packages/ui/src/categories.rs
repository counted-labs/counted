use crate::tid;

use crate::decrypted::DecryptedExpense;

pub struct Category {
    pub name: &'static str,
    pub parent: &'static str,
    pub emoji: &'static str,
    pub keywords: &'static [&'static str],
}

pub const DEFAULT_CATEGORY: Category =
    Category { name: "Autres", parent: "Autres", emoji: "💵", keywords: &[] };

/// Skipped before matching: "thé" folds to "the", so the English article would be a coffee.
/// No keyword may appear here — `no_keyword_is_a_stop_token` enforces it.
const STOP_TOKENS: &[&str] = &[
    "le", "la", "les", "un", "une", "du", "de", "des", "d", "l", "et", "au", "aux", "pour", "chez",
    "avec", "en", "sur", "dans", "the", "a", "of", "and", "for", "to", "in",
];

/// Keywords must be stored **already normalized** (lowercase, accent-free): they are compared
/// untouched against `normalize`d tokens, so anything else never matches. Guard:
/// `keywords_are_already_normalized`. Plurals need no entry — `word_matches` handles them.
pub const CATEGORIES: &[Category] = &[
    Category {
        name: "Restaurants",
        parent: "Nourriture",
        emoji: "🍽️",
        keywords: &[
            "restaurant",
            "resto",
            "dinner",
            "diner",
            "repas",
            "bouffe",
            "dejeuner",
            "lunch",
            "brunch",
            "breakfast",
            "meal",
            "food",
            "eat",
        ],
    },
    Category {
        name: "Café",
        parent: "Nourriture",
        emoji: "☕",
        // "thé" folds to "the", which is a stop token — "tisane" and "infusion" cover it instead.
        keywords: &["coffee", "cafe", "starbucks", "tea", "tisane", "infusion", "expresso"],
    },
    Category { name: "Pizza", parent: "Nourriture", emoji: "🍕", keywords: &["pizza"] },
    Category {
        name: "Burger",
        parent: "Nourriture",
        emoji: "🍔",
        keywords: &["burger", "hamburger", "mcdo", "mcdonald"],
    },
    Category {
        name: "Sushi",
        parent: "Nourriture",
        emoji: "🍣",
        keywords: &["sushi", "jap", "japonais", "japanese", "ramen"],
    },
    Category {
        name: "Bar",
        parent: "Nourriture",
        emoji: "🍺",
        keywords: &[
            "beer", "bar", "pub", "biere", "drink", "wine", "vin", "apero", "cocktail", "alcohol",
        ],
    },
    Category {
        name: "Courses",
        parent: "Nourriture",
        emoji: "🛒",
        keywords: &[
            "grocery",
            "groceries",
            "supermarket",
            "market",
            "food shopping",
            "courses",
            "supermarche",
            "marche",
            "epicerie",
            "carrefour",
            "leclerc",
            "lidl",
            "auchan",
            "monoprix",
        ],
    },
    Category {
        name: "Desserts",
        parent: "Nourriture",
        emoji: "🍦",
        keywords: &["ice cream", "dessert", "glace", "gateau", "patisserie"],
    },
    Category {
        name: "Taxi",
        parent: "Transport",
        emoji: "🚕",
        keywords: &["uber", "taxi", "cab", "ride", "vtc"],
    },
    Category {
        name: "Carburant",
        parent: "Transport",
        emoji: "⛽",
        keywords: &["gas", "fuel", "essence", "petrol", "diesel", "gasoil"],
    },
    Category {
        name: "Train",
        parent: "Transport",
        emoji: "🚆",
        keywords: &["train", "railway", "sncf", "tgv"],
    },
    Category {
        name: "Avion",
        parent: "Transport",
        emoji: "✈️",
        keywords: &["plane", "flight", "airplane", "avion", "vol"],
    },
    Category {
        name: "Bus",
        parent: "Transport",
        emoji: "🚌",
        keywords: &["bus", "autobus", "metro", "tram", "tramway"],
    },
    Category {
        name: "Voiture",
        parent: "Transport",
        emoji: "🚗",
        keywords: &["car", "vehicle", "auto", "voiture"],
    },
    Category {
        name: "Vélo",
        parent: "Transport",
        emoji: "🚲",
        keywords: &["bike", "bicycle", "velo", "trottinette"],
    },
    Category { name: "Parking", parent: "Transport", emoji: "🅿️", keywords: &["parking"] },
    Category {
        name: "Hôtel",
        parent: "Hébergement",
        emoji: "🏨",
        keywords: &[
            "hotel",
            "airbnb",
            "accommodation",
            "lodging",
            "hebergement",
            "auberge",
            "camping",
            "gite",
        ],
    },
    Category {
        name: "Loyer",
        parent: "Hébergement",
        emoji: "🏠",
        keywords: &["rent", "loyer"],
    },
    Category {
        name: "Cinéma",
        parent: "Loisirs",
        emoji: "🎬",
        keywords: &["movie", "cinema", "film"],
    },
    Category {
        name: "Musique",
        parent: "Loisirs",
        emoji: "🎵",
        keywords: &["concert", "music", "musique", "festival"],
    },
    Category {
        name: "Gaming",
        parent: "Loisirs",
        emoji: "🎮",
        keywords: &["game", "gaming", "jeu", "console"],
    },
    Category {
        name: "Ski",
        parent: "Loisirs",
        emoji: "🎿",
        keywords: &["ski", "skiing", "snowboard"],
    },
    Category {
        name: "Sport",
        parent: "Loisirs",
        emoji: "⚽",
        keywords: &["sport", "gym", "fitness", "piscine", "tennis", "foot"],
    },
    Category {
        name: "Tickets",
        parent: "Loisirs",
        emoji: "🎟️",
        keywords: &["ticket", "billet"],
    },
    Category {
        name: "Shopping",
        parent: "Shopping",
        emoji: "🛍️",
        keywords: &["shop", "shopping", "clothes", "clothing", "fashion", "vetement"],
    },
    Category {
        name: "Téléphone",
        parent: "Shopping",
        emoji: "📱",
        keywords: &["phone", "mobile", "smartphone", "telephone", "forfait"],
    },
    Category {
        name: "Informatique",
        parent: "Shopping",
        emoji: "💻",
        keywords: &["computer", "laptop", "ordinateur", "pc"],
    },
    Category {
        name: "Livres",
        parent: "Shopping",
        emoji: "📚",
        keywords: &["book", "library", "livre", "librairie"],
    },
    Category {
        name: "Internet",
        parent: "Services",
        emoji: "📡",
        keywords: &["internet", "wifi", "fibre"],
    },
    Category {
        name: "Électricité",
        parent: "Services",
        emoji: "⚡",
        keywords: &["electricity", "electric", "electricite", "electrique", "edf"],
    },
    Category { name: "Eau", parent: "Services", emoji: "💧", keywords: &["water", "eau"] },
    Category {
        name: "Assurance",
        parent: "Services",
        emoji: "🛡️",
        keywords: &["insurance", "assurance", "mutuelle"],
    },
    Category {
        name: "Santé",
        parent: "Services",
        emoji: "🏥",
        keywords: &[
            "medical",
            "doctor",
            "hospital",
            "health",
            "pharmacy",
            "docteur",
            "medecin",
            "hopital",
            "pharmacie",
            "dentiste",
        ],
    },
    Category {
        name: "Coiffeur",
        parent: "Services",
        emoji: "💇",
        keywords: &["haircut", "salon", "coiffeur", "coiffure"],
    },
    Category {
        name: "Spa",
        parent: "Services",
        emoji: "💆",
        keywords: &["spa", "massage", "wellness"],
    },
    Category {
        name: "Anniversaire",
        parent: "Fêtes & Cadeaux",
        emoji: "🎂",
        keywords: &["birthday", "anniversaire"],
    },
    Category {
        name: "Noël",
        parent: "Fêtes & Cadeaux",
        emoji: "🎄",
        keywords: &["christmas", "noel"],
    },
    Category {
        name: "Cadeau",
        parent: "Fêtes & Cadeaux",
        emoji: "🎁",
        keywords: &["gift", "cadeau", "present"],
    },
];

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

/// Title into whole words — lowercase, accents folded, split on non-alphanumerics ("l'eau",
/// "taxi-moto", "Uber (aéroport)"), stop tokens dropped. Whole words, not `contains`, or "cadeau"
/// is a glass of water ("eau"). Plurals belong to `word_matches`: stripping the "s" here would
/// turn "repas" into "repa".
fn normalize(name: &str) -> Vec<String> {
    let folded: String = name
        .to_lowercase()
        .replace('œ', "oe")
        .replace('æ', "ae")
        .chars()
        .map(|c| match c {
            'á' | 'à' | 'â' | 'ä' | 'ã' | 'å' => 'a',
            'é' | 'è' | 'ê' | 'ë' => 'e',
            'í' | 'ì' | 'î' | 'ï' => 'i',
            'ó' | 'ò' | 'ô' | 'ö' | 'õ' => 'o',
            'ú' | 'ù' | 'û' | 'ü' => 'u',
            'ç' => 'c',
            'ñ' => 'n',
            'ý' | 'ÿ' => 'y',
            c if c.is_alphanumeric() => c,
            _ => ' ',
        })
        .collect();

    folded
        .split_whitespace()
        .map(str::to_string)
        .filter(|t| !STOP_TOKENS.contains(&t.as_str()))
        .collect()
}

/// Tolerates the plural the user typed ("cadeaux" is "cadeau"). Only the token is de-pluralized,
/// so keywords merely ending in "s" ("repas", "starbucks") stay whole words.
fn word_matches(token: &str, keyword: &str) -> bool {
    token == keyword
        || token.strip_suffix('s') == Some(keyword)
        || token.strip_suffix('x') == Some(keyword)
}

fn matches(tokens: &[String], keyword: &str) -> bool {
    if !keyword.contains(' ') {
        return tokens.iter().any(|t| word_matches(t, keyword));
    }
    let words: Vec<&str> = keyword.split(' ').collect();
    tokens
        .windows(words.len())
        .any(|w| w.iter().zip(&words).all(|(t, k)| word_matches(t, k)))
}

pub fn find_category(name: &str) -> &'static Category {
    let tokens = normalize(name);
    // Multi-word first: Restaurants owns "food" and is declared earlier, so a single pass in
    // table order would never reach Courses' "food shopping".
    for multi_word in [true, false] {
        let found = CATEGORIES.iter().find(|c| {
            c.keywords.iter().filter(|k| k.contains(' ') == multi_word).any(|k| matches(&tokens, k))
        });
        if let Some(c) = found {
            return c;
        }
    }
    &DEFAULT_CATEGORY
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

    /// A keyword that does not survive `normalize` unchanged could never match.
    #[test]
    fn keywords_are_already_normalized() {
        for cat in CATEGORIES {
            for k in cat.keywords {
                assert_eq!(
                    &normalize(k).join(" "),
                    k,
                    "keyword {k:?} of {} is not normalized",
                    cat.name
                );
            }
        }
    }

    #[test]
    fn no_keyword_is_a_stop_token() {
        for cat in CATEGORIES {
            for k in cat.keywords {
                assert!(!STOP_TOKENS.contains(k), "keyword {k:?} of {} is a stop token", cat.name);
            }
        }
    }

    #[test]
    fn normalize_folds_accents_and_case() {
        assert_eq!(normalize("CAFÉ"), ["cafe"]);
        assert_eq!(normalize("Hôtel"), ["hotel"]);
        assert_eq!(normalize("Bière"), ["biere"]);
        assert_eq!(normalize("Électricité"), ["electricite"]);
    }

    #[test]
    fn normalize_splits_on_punctuation() {
        assert_eq!(normalize("l'eau"), ["eau"]);
        assert_eq!(normalize("taxi-moto"), ["taxi", "moto"]);
        assert_eq!(normalize("Uber (aéroport)"), ["uber", "aeroport"]);
        assert_eq!(normalize("Resto, bar"), ["resto", "bar"]);
    }

    #[test]
    fn normalize_drops_stop_tokens() {
        assert_eq!(normalize("Hotel du Nord"), ["hotel", "nord"]);
        assert_eq!(normalize("The Ivy"), ["ivy"]);
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

    /// Each of these once matched a keyword buried inside a longer word.
    #[test]
    fn a_keyword_inside_a_word_is_not_a_match() {
        assert_eq!(get_expense_emoji("cadeau"), "🎁"); // was 💧, via "eau"
        assert_eq!(get_expense_emoji("bateau"), "💵"); // was 💧
        assert_eq!(get_expense_emoji("transport"), "💵"); // was ⚽, via "sport"
        assert_eq!(get_expense_emoji("parent"), "💵"); // was 🏠, via "rent"
        assert_eq!(get_expense_emoji("carte SNCF"), "🚆"); // "carte" was 🚗, via "car"
        assert_eq!(get_expense_emoji("magasin"), "💵"); // was ⛽, via "gas"
        assert_eq!(get_expense_emoji("steak"), "💵"); // was ☕, via "tea"
        assert_eq!(get_expense_emoji("jeudi soir"), "💵"); // was 🎮, via "jeu"
        assert_eq!(get_expense_emoji("business"), "💵"); // was 🚌, via "bus"
        assert_eq!(get_expense_emoji("spaghetti"), "💵"); // was 💆, via "spa"
    }

    /// Courses owns "food shopping"; Restaurants owns "food" and comes first in the table.
    #[test]
    fn a_multi_word_keyword_beats_a_single_word_one() {
        assert_eq!(get_expense_emoji("food shopping"), "🛒");
        assert_eq!(get_expense_emoji("food"), "🍽️");
        assert_eq!(get_expense_emoji("ice cream"), "🍦");
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
}
