//! Everything the participant block decides, with no rendering in it: the name a friend starts
//! with, which names clash, what the combobox suggests and what the chips offer. See
//! `docs/plans/friends.md` §11.

use shared::Friend;
use uuid::Uuid;

pub const QUICK_ADD_COUNT: usize = 4;
const MAX_SUGGESTIONS: usize = 5;

/// A participant not saved yet. `friend` is set when picked from the friends list: they get an
/// invitation once the participant exists.
#[derive(Debug, Clone, PartialEq)]
pub struct DraftParticipant {
    pub name: String,
    pub friend: Option<Friend>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum NameError {
    Empty,
    Taken(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Pick {
    Friend(Friend),
    Guest(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Suggestion {
    pub friend: Friend,
    /// `false` for a friend with no public key yet: shown, never picked.
    pub ready: bool,
    pub before: String,
    pub matched: String,
    pub after: String,
}

pub fn is_invitable(friend: &Friend) -> bool {
    friend.public_key.is_some()
}

pub fn email_local_part(email: &str) -> &str {
    email.split('@').next().unwrap_or(email)
}

/// `tom.bernard@x.io` → `Tom`: the friends list holds emails only (friends.md §4), so this is the
/// best starting point there is. The person renames it if it is wrong.
pub fn name_from_email(email: &str) -> String {
    let local = email_local_part(email);
    let first = local.split(['.', '_', '-', '+']).find(|s| !s.is_empty()).unwrap_or(local);
    capitalise(first)
}

fn capitalise(word: &str) -> String {
    let mut chars = word.chars();
    match chars.next() {
        Some(c) => c.to_uppercase().chain(chars).collect(),
        None => String::new(),
    }
}

pub fn same_name(a: &str, b: &str) -> bool {
    a.trim().to_lowercase() == b.trim().to_lowercase()
}

/// Refused because "who is me" is still resolved by name among freshly created participants.
pub fn check_name(name: &str, taken: &[String]) -> Result<String, NameError> {
    let name = name.trim();
    if name.is_empty() {
        return Err(NameError::Empty);
    }
    if taken.iter().any(|t| same_name(t, name)) {
        return Err(NameError::Taken(name.to_string()));
    }
    Ok(name.to_string())
}

/// Every name in use, yours first: what a new or renamed draft must not repeat.
pub fn taken_names(me: &str, saved: &[String], drafts: &[DraftParticipant]) -> Vec<String> {
    std::iter::once(me.trim().to_string())
        .filter(|n| !n.is_empty())
        .chain(saved.iter().cloned())
        .chain(drafts.iter().map(|d| d.name.clone()))
        .collect()
}

/// A friend's derived name that clashes is made unique instead of refused: the person did not type
/// it. First the whole local part, then a number.
fn unique_friend_name(email: &str, taken: &[String]) -> String {
    let short = name_from_email(email);
    if check_name(&short, taken).is_ok() {
        return short;
    }
    let long = capitalise(email_local_part(email));
    if check_name(&long, taken).is_ok() {
        return long;
    }
    (2..).map(|n| format!("{short} {n}")).find(|n| check_name(n, taken).is_ok()).unwrap_or(short)
}

pub fn added_friend_ids(drafts: &[DraftParticipant]) -> Vec<Uuid> {
    drafts.iter().filter_map(|d| d.friend.as_ref().map(|f| f.account_id)).collect()
}

pub fn invite_count(drafts: &[DraftParticipant]) -> usize {
    drafts.iter().filter(|d| d.friend.is_some()).count()
}

pub fn can_create(me: &str, drafts: &[DraftParticipant]) -> bool {
    !me.trim().is_empty() && !drafts.is_empty()
}

/// Adds what was picked. A guest's name is what the person typed, so a clash is theirs to fix.
pub fn apply_pick(drafts: &mut Vec<DraftParticipant>, pick: Pick, taken: &[String]) -> Result<(), NameError> {
    match pick {
        Pick::Guest(text) => {
            let name = check_name(&text, taken)?;
            drafts.push(DraftParticipant { name, friend: None });
        }
        Pick::Friend(friend) => {
            if drafts.iter().any(|d| d.friend.as_ref().map(|f| f.account_id) == Some(friend.account_id)) {
                return Ok(());
            }
            let name = unique_friend_name(&friend.email, taken);
            drafts.push(DraftParticipant { name, friend: Some(friend) });
        }
    }
    Ok(())
}

/// `taken` must not include the draft being renamed.
pub fn rename(drafts: &mut [DraftParticipant], index: usize, name: &str, taken: &[String]) -> Result<(), NameError> {
    let name = check_name(name, taken)?;
    if let Some(d) = drafts.get_mut(index) {
        d.name = name;
    }
    Ok(())
}

/// Friends whose email contains the query, case-insensitive, not yet added.
pub fn suggestions(query: &str, friends: &[Friend], added: &[Uuid]) -> Vec<Suggestion> {
    let q = query.trim().to_lowercase();
    if q.is_empty() {
        return vec![];
    }
    friends
        .iter()
        .filter(|f| !added.contains(&f.account_id))
        .filter_map(|f| {
            let lower = f.email.to_lowercase();
            let at = lower.find(&q)?;
            // Lowercasing kept the byte offsets (ASCII, as emails nearly always are): cut there.
            let (before, matched, after) = if lower.len() == f.email.len() && f.email.is_char_boundary(at) && f.email.is_char_boundary(at + q.len()) {
                (f.email[..at].to_string(), f.email[at..at + q.len()].to_string(), f.email[at + q.len()..].to_string())
            } else {
                (f.email.clone(), String::new(), String::new())
            };
            Some(Suggestion { friend: f.clone(), ready: is_invitable(f), before, matched, after })
        })
        .take(MAX_SUGGESTIONS)
        .collect()
}

/// What Enter and the arrow keys walk through: the ready friends, then "add as a guest".
pub fn picks(query: &str, suggestions: &[Suggestion]) -> Vec<Pick> {
    suggestions
        .iter()
        .filter(|s| s.ready)
        .map(|s| Pick::Friend(s.friend.clone()))
        .chain((!query.trim().is_empty()).then(|| Pick::Guest(query.trim().to_string())))
        .collect()
}

pub fn quick_add(friends: &[Friend], added: &[Uuid]) -> Vec<Friend> {
    friends
        .iter()
        .filter(|f| is_invitable(f) && !added.contains(&f.account_id))
        .take(QUICK_ADD_COUNT)
        .cloned()
        .collect()
}

pub fn initial(name: &str) -> String {
    name.trim().chars().next().map(|c| c.to_uppercase().collect()).unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn friend(email: &str, ready: bool) -> Friend {
        Friend {
            id: Uuid::new_v4(),
            account_id: Uuid::new_v4(),
            email: email.into(),
            public_key: ready.then(|| vec![1; 32]),
        }
    }

    fn names(v: &[&str]) -> Vec<String> {
        v.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn a_friend_starts_with_the_first_word_of_their_address() {
        assert_eq!(name_from_email("tom.bernard@gmail.com"), "Tom");
        assert_eq!(name_from_email("sarah_l@outlook.fr"), "Sarah");
        assert_eq!(name_from_email("jean-pierre@x.io"), "Jean");
        assert_eq!(name_from_email("hugo+counted@fastmail.com"), "Hugo");
        assert_eq!(name_from_email("élodie@x.fr"), "Élodie");
        assert_eq!(name_from_email(".x@y.z"), "X");
    }

    #[test]
    fn names_clash_case_insensitively_and_blank_is_refused() {
        let taken = names(&["Alex", "Julien"]);
        assert_eq!(check_name(" julien ", &taken), Err(NameError::Taken("julien".into())));
        assert_eq!(check_name("   ", &taken), Err(NameError::Empty));
        assert_eq!(check_name(" Marie ", &taken), Ok("Marie".into()));
    }

    #[test]
    fn a_guest_that_clashes_is_refused_but_a_friend_is_renamed() {
        let mut drafts = vec![];
        let taken = names(&["Tom"]);
        assert_eq!(apply_pick(&mut drafts, Pick::Guest("tom".into()), &taken), Err(NameError::Taken("tom".into())));
        assert!(drafts.is_empty());

        apply_pick(&mut drafts, Pick::Friend(friend("tom.bernard@x.io", true)), &taken).unwrap();
        assert_eq!(drafts[0].name, "Tom.bernard");

        let taken = names(&["Tom", "Tom.bernard"]);
        apply_pick(&mut drafts, Pick::Friend(friend("tom.bernard@y.io", true)), &taken).unwrap();
        assert_eq!(drafts[1].name, "Tom 2");
    }

    #[test]
    fn the_same_friend_is_added_once() {
        let f = friend("marie@x.io", true);
        let mut drafts = vec![];
        apply_pick(&mut drafts, Pick::Friend(f.clone()), &[]).unwrap();
        apply_pick(&mut drafts, Pick::Friend(f), &names(&["Marie"])).unwrap();
        assert_eq!(drafts.len(), 1);
        assert_eq!(invite_count(&drafts), 1);
    }

    #[test]
    fn rename_refuses_a_name_in_use() {
        let mut drafts = vec![DraftParticipant { name: "Tom".into(), friend: None }];
        assert_eq!(rename(&mut drafts, 0, "Alex", &names(&["Alex"])), Err(NameError::Taken("Alex".into())));
        rename(&mut drafts, 0, " Tommy ", &names(&["Alex"])).unwrap();
        assert_eq!(drafts[0].name, "Tommy");
    }

    #[test]
    fn suggestions_match_the_email_and_skip_added_friends() {
        let marie = friend("Marie.Dupont@proton.me", true);
        let lea = friend("lea.martin@gmail.com", false);
        let tom = friend("tom@x.io", true);
        let all = vec![marie.clone(), lea.clone(), tom];

        let found = suggestions("MA", &all, &[]);
        assert_eq!(found.len(), 2);
        assert_eq!((found[0].before.as_str(), found[0].matched.as_str(), found[0].after.as_str()), ("", "Ma", "rie.Dupont@proton.me"));
        assert_eq!(found[1].matched, "ma");
        assert!(!found[1].ready);

        assert_eq!(suggestions("ma", &all, &[marie.account_id]).len(), 1);
        assert!(suggestions("  ", &all, &[]).is_empty());
    }

    #[test]
    fn enter_walks_ready_friends_then_the_guest_option() {
        let all = vec![friend("marie@x.io", true), friend("mathis@x.io", false)];
        let found = suggestions("ma", &all, &[]);
        let p = picks("ma", &found);
        assert_eq!(p.len(), 2);
        assert!(matches!(p[0], Pick::Friend(_)));
        assert_eq!(p[1], Pick::Guest("ma".into()));
    }

    #[test]
    fn chips_offer_invitable_friends_not_yet_added() {
        let all: Vec<Friend> = (0..6).map(|i| friend(&format!("f{i}@x.io"), i != 1)).collect();
        let chips = quick_add(&all, &[all[0].account_id]);
        assert_eq!(chips.len(), QUICK_ADD_COUNT);
        assert!(chips.iter().all(|f| f.account_id != all[0].account_id && f.account_id != all[1].account_id));
    }

    #[test]
    fn create_needs_your_name_and_someone_else() {
        let other = vec![DraftParticipant { name: "Julien".into(), friend: None }];
        assert!(can_create("Alex", &other));
        assert!(!can_create(" ", &other));
        assert!(!can_create("Alex", &[]));
    }

    #[test]
    fn taken_names_lead_with_yours_and_skip_a_blank_one() {
        let drafts = vec![DraftParticipant { name: "Julien".into(), friend: None }];
        assert_eq!(taken_names(" Alex ", &names(&["Marie"]), &drafts), names(&["Alex", "Marie", "Julien"]));
        assert_eq!(taken_names("", &[], &drafts), names(&["Julien"]));
    }
}
