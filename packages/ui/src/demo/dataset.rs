//! The `/demo` ledger, ported from `e2e/shots/src/dataset.ts` (the store screenshots' seed).
//! `category` is a CHART_CATEGORIES id, French in both languages.

pub struct DemoExpense {
    pub name: &'static str,
    pub amount: f64,
    pub category: &'static str,
    /// Index into `participants`; 0 is the visitor.
    pub payer: usize,
    pub days_ago: i64,
}

pub struct Dataset {
    pub project: &'static str,
    pub participants: [&'static str; 5],
    pub expenses: &'static [DemoExpense],
}

const fn e(name: &'static str, amount: f64, category: &'static str, payer: usize, days_ago: i64) -> DemoExpense {
    DemoExpense { name, amount, category, payer, days_ago }
}

pub const FR: Dataset = Dataset {
    project: "Week-end à Étretat",
    participants: ["Camille", "Ben", "Léa", "Romain", "Stéphanie"],
    expenses: &[
        e("Location de la maison", 480.00, "Hébergement", 1, 6),
        e("Plein d’essence", 62.00, "Transport", 3, 6),
        e("Péage A13", 18.60, "Transport", 3, 6),
        e("Courses du samedi", 86.40, "Nourriture", 0, 5),
        e("Restaurant Le Bel Ami", 134.50, "Nourriture", 2, 5),
        e("Billets falaises", 44.00, "Loisirs", 4, 5),
        e("Location de vélos", 72.00, "Loisirs", 1, 4),
        e("Petit-déjeuner", 27.30, "Nourriture", 0, 4),
        e("Cidre et calvados", 29.50, "Shopping", 4, 4),
        e("Marché du dimanche", 38.90, "Nourriture", 2, 3),
        e("Parking centre-ville", 12.00, "Transport", 0, 3),
        e("Musée des falaises", 33.00, "Loisirs", 2, 2),
        e("Cadeau pour Manon", 45.00, "Fêtes & Cadeaux", 1, 1),
        e("Dîner d’anniversaire", 156.00, "Nourriture", 3, 1),
    ],
};

pub const EN: Dataset = Dataset {
    project: "Weekend in Étretat",
    participants: ["Camille", "Ben", "Lea", "Romain", "Steph"],
    expenses: &[
        e("House rental", 480.00, "Hébergement", 1, 6),
        e("Petrol", 62.00, "Transport", 3, 6),
        e("A13 toll", 18.60, "Transport", 3, 6),
        e("Saturday groceries", 86.40, "Nourriture", 0, 5),
        e("Dinner at Le Bel Ami", 134.50, "Nourriture", 2, 5),
        e("Cliff walk tickets", 44.00, "Loisirs", 4, 5),
        e("Bike hire", 72.00, "Loisirs", 1, 4),
        e("Breakfast", 27.30, "Nourriture", 0, 4),
        e("Cider and calvados", 29.50, "Shopping", 4, 4),
        e("Sunday market", 38.90, "Nourriture", 2, 3),
        e("City parking", 12.00, "Transport", 0, 3),
        e("Cliff museum", 33.00, "Loisirs", 2, 2),
        e("Gift for Manon", 45.00, "Fêtes & Cadeaux", 1, 1),
        e("Birthday dinner", 156.00, "Nourriture", 3, 1),
    ],
};

pub fn dataset(lang: &str) -> &'static Dataset {
    match lang {
        "fr" => &FR,
        _ => &EN,
    }
}
