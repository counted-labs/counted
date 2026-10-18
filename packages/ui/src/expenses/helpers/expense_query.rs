//! Search over the expenses tab. Runs on the page's one decrypted list, so the server never sees a
//! query: it is kept in the app-wide store, not the URL, because a reload sends the URL to the server.

use std::collections::{HashMap, HashSet};

use crate::categories::{normalize, CHART_CATEGORIES};
use crate::expenses::helpers::project_data::{ProjectData, RowExpense};

#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub enum ExpenseFilter {
    #[default]
    All,
    MyPayments,
    MyDebts,
}

#[derive(Clone, PartialEq, Debug, Default)]
pub struct ExpenseQuery {
    pub scope: ExpenseFilter,
    /// The search field is open.
    pub searching: bool,
    pub text: String,
}

impl ExpenseQuery {
    /// The typed words, folded as [`normalize`] folds them. Empty while the field is closed.
    pub fn words(&self) -> Vec<String> {
        if self.searching { normalize(&self.text) } else { Vec::new() }
    }

    /// `user_id` is the viewer's participant, `None` before they picked one, when the scope chips
    /// filter nothing. `label` translates a category id, which the search also matches.
    pub fn select<'a>(
        &self,
        data: &'a ProjectData,
        user_id: Option<i32>,
        label: impl Fn(&str) -> String,
    ) -> Vec<&'a RowExpense> {
        let scoped: Option<HashSet<i32>> = match (self.scope, user_id) {
            (ExpenseFilter::All, _) | (_, None) => None,
            (scope, Some(uid)) => {
                let want_debt = scope == ExpenseFilter::MyDebts;
                Some(
                    data.payments
                        .iter()
                        .filter(|p| p.user_id == uid && p.is_debt == want_debt)
                        .map(|p| p.expense_id)
                        .collect(),
                )
            }
        };
        self.search(
            data.expenses.iter().filter(|r| scoped.as_ref().is_none_or(|ids| ids.contains(&r.expense.id))),
            label,
        )
    }

    /// The rows the typed words match, whatever project they come from.
    pub fn search<'a>(
        &self,
        rows: impl Iterator<Item = &'a RowExpense>,
        label: impl Fn(&str) -> String,
    ) -> Vec<&'a RowExpense> {
        let words = self.words();
        let labels: HashMap<&str, Vec<String>> = if words.is_empty() {
            HashMap::new()
        } else {
            CHART_CATEGORIES.iter().map(|c| (*c, normalize(&label(c)))).collect()
        };
        rows.filter(|r| {
                words.is_empty() || {
                    let mut haystack = normalize(&r.expense.name);
                    if let Some(d) = &r.expense.description {
                        haystack.extend(normalize(d));
                    }
                    haystack.extend(labels.get(r.category).into_iter().flatten().cloned());
                    words.iter().all(|w| haystack.iter().any(|h| h.starts_with(w.as_str())))
                }
            })
            .collect()
    }
}

/// `text` cut into runs, `true` for the start of a word that a typed word matches: "piz" marks
/// "Piz" in "Pizza", and "aero" marks "aéro" in "Taxi aéroport". The original characters are kept.
pub fn mark_segments(text: &str, words: &[String]) -> Vec<(String, bool)> {
    let mut segments: Vec<(String, bool)> = Vec::new();
    let mut push = |s: String, marked: bool| {
        if s.is_empty() {
            return;
        }
        match segments.last_mut() {
            Some((last, m)) if *m == marked => last.push_str(&s),
            _ => segments.push((s, marked)),
        }
    };
    let chars: Vec<char> = text.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        if !chars[i].is_alphanumeric() {
            push(chars[i].to_string(), false);
            i += 1;
            continue;
        }
        let end = (i..chars.len()).find(|&j| !chars[j].is_alphanumeric()).unwrap_or(chars.len());
        let word = &chars[i..end];
        let folded: Vec<String> = word.iter().map(|c| normalize(&c.to_string()).concat()).collect();
        let whole = folded.concat();
        let marked_chars = words
            .iter()
            .filter(|w| whole.starts_with(w.as_str()))
            .map(|w| {
                let mut len = 0;
                folded.iter().take_while(|f| {
                    let before = len;
                    len += f.len();
                    before < w.len()
                }).count()
            })
            .max()
            .unwrap_or(0);
        push(word[..marked_chars].iter().collect(), true);
        push(word[marked_chars..].iter().collect(), false);
        i = end;
    }
    segments
}

pub fn mark_segments_hit(text: &str, words: &[String]) -> bool {
    mark_segments(text, words).iter().any(|(_, marked)| *marked)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::test_fixtures::{make_expense, make_payment, make_user, test_key};
    use crate::expenses::helpers::project_data::{build, LiveData};
    use shared::{Expense, ExpenseType, Payment};

    fn data(expenses: Vec<Expense>, payments: Vec<Payment>) -> ProjectData {
        let k = test_key();
        build(
            Some(k),
            Some(&LiveData {
                project_id: uuid::Uuid::nil(),
                project: None,
                users: vec![make_user(&k, 7, "Alice"), make_user(&k, 8, "Bob")],
                expenses,
                payments,
                version: None,
            }),
        )
    }

    fn sample() -> ProjectData {
        let k = test_key();
        data(
            vec![
                make_expense(&k, 1, "Pizza margherita", 24.0, ExpenseType::Expense, "2025-03-01"),
                make_expense(&k, 2, "Taxi aéroport", 60.0, ExpenseType::Expense, "2025-03-02"),
                make_expense(&k, 3, "Remboursement", 30.0, ExpenseType::Transfer, "2025-03-03"),
                make_expense(&k, 4, "Cadeau", 15.5, ExpenseType::Expense, "2025-03-04"),
            ],
            vec![
                make_payment(&k, 1, 1, 7, false, 24.0),
                make_payment(&k, 2, 1, 8, true, 24.0),
                make_payment(&k, 3, 2, 8, false, 60.0),
                make_payment(&k, 4, 2, 7, true, 60.0),
                make_payment(&k, 5, 3, 8, false, 30.0),
                make_payment(&k, 6, 3, 7, true, 30.0),
                make_payment(&k, 7, 4, 7, false, 15.5),
                make_payment(&k, 8, 4, 8, true, 15.5),
            ],
        )
    }

    fn english(id: &str) -> String {
        match id {
            "Transport" => "Transport".to_string(),
            "Nourriture" => "Food".to_string(),
            other => other.to_string(),
        }
    }

    fn ids(q: &ExpenseQuery, user_id: Option<i32>) -> Vec<i32> {
        let d = sample();
        let mut ids: Vec<i32> = q.select(&d, user_id, english).iter().map(|r| r.expense.id).collect();
        ids.sort();
        ids
    }

    fn search(t: &str) -> ExpenseQuery {
        ExpenseQuery { searching: true, text: t.to_string(), ..Default::default() }
    }

    #[test]
    fn an_empty_query_keeps_everything() {
        assert_eq!(ids(&ExpenseQuery::default(), Some(7)), vec![1, 2, 3, 4]);
        assert!(search("  ").words().is_empty(), "blank text is no search");
    }

    #[test]
    fn a_closed_field_searches_nothing() {
        let closed = ExpenseQuery { text: "taxi".into(), ..Default::default() };
        assert_eq!(ids(&closed, None), vec![1, 2, 3, 4]);
        assert!(closed.words().is_empty());
    }

    #[test]
    fn text_matches_word_starts_ignoring_case_and_accents() {
        assert_eq!(ids(&search("PIZ"), None), vec![1]);
        assert_eq!(ids(&search("aero"), None), vec![2]);
        assert_eq!(ids(&search("taxi aér"), None), vec![2], "every word must match");
        assert!(ids(&search("eau"), None).is_empty(), "a word's middle is no match");
    }

    #[test]
    fn text_matches_the_category_in_the_viewers_language() {
        assert_eq!(ids(&search("transp"), None), vec![2]);
        assert_eq!(ids(&search("food"), None), vec![1]);
    }

    #[test]
    fn text_reads_the_description_too() {
        let k = test_key();
        let mut e = make_expense(&k, 1, "Courses", 10.0, ExpenseType::Expense, "2025-01-01");
        let mut p: shared::ExpensePayload = crate::crypto::decrypt_json(&k, &e.payload).unwrap();
        p.description = Some("lait et œufs".to_string());
        e.payload = crate::crypto::encrypt_json(&k, &p).unwrap();
        let d = data(vec![e], vec![]);
        assert_eq!(search("oeuf").select(&d, None, english).len(), 1);
    }

    #[test]
    fn the_scope_chips_follow_the_viewer() {
        let mine = ExpenseQuery { scope: ExpenseFilter::MyPayments, ..Default::default() };
        assert_eq!(ids(&mine, Some(7)), vec![1, 4]);
        let owed = ExpenseQuery { scope: ExpenseFilter::MyDebts, ..Default::default() };
        assert_eq!(ids(&owed, Some(7)), vec![2, 3]);
        assert_eq!(ids(&owed, None), vec![1, 2, 3, 4], "no viewer, nothing to scope to");
    }

    #[test]
    fn search_and_scope_combine() {
        let q = ExpenseQuery { scope: ExpenseFilter::MyPayments, ..search("cad") };
        assert_eq!(ids(&q, Some(7)), vec![4]);
    }

    fn marked(text: &str, q: &str) -> Vec<(String, bool)> {
        mark_segments(text, &normalize(q))
    }

    #[test]
    fn marks_the_matched_start_of_each_word_keeping_the_original_letters() {
        assert_eq!(
            marked("Taxi aéroport", "aero"),
            vec![("Taxi ".into(), false), ("aéro".into(), true), ("port".into(), false)]
        );
        assert_eq!(marked("Resto U Fanale", "resto"), vec![("Resto".into(), true), (" U Fanale".into(), false)]);
    }

    #[test]
    fn marks_every_word_a_typed_word_starts() {
        assert_eq!(
            marked("Petit-déj resto gare", "resto g"),
            vec![
                ("Petit-déj ".into(), false),
                ("resto".into(), true),
                (" ".into(), false),
                ("g".into(), true),
                ("are".into(), false),
            ]
        );
    }

    #[test]
    fn a_ligature_counts_as_the_letters_it_folds_to() {
        assert_eq!(marked("Œufs", "oeu"), vec![("Œu".into(), true), ("fs".into(), false)]);
    }

    #[test]
    fn no_words_marks_nothing() {
        assert_eq!(mark_segments("Pizza", &[]), vec![("Pizza".into(), false)]);
        assert_eq!(marked("cadeau", "eau"), vec![("cadeau".into(), false)]);
        assert!(!mark_segments_hit("Dîner du port", &normalize("food")));
        assert!(mark_segments_hit("Food", &normalize("foo")));
    }
}
