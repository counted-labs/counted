use shared::User;
use uuid::Uuid;

use crate::common::LocalStorageState;
use crate::decrypted::user_name;

/// The participants no account holds as its identity, this device's own left out: the people who
/// most likely never received the project. A device that joined by link without an account claims
/// nothing either, so it is counted too; the card can be dismissed for that. `users` is the
/// pickable roster, removed participants already out.
pub fn unclaimed_guests(key: &[u8; 32], users: &[User], me: i32) -> Vec<String> {
    users
        .iter()
        .filter(|u| !u.claimed && u.id != me)
        .map(|u| user_name(key, u))
        .filter(|name| !name.is_empty())
        .collect()
}

/// Each name isolated, so one written right-to-left cannot reorder its neighbours or the commas.
pub fn guest_names_line(names: &[String]) -> String {
    names.iter().map(|n| format!("\u{2068}{n}\u{2069}")).collect::<Vec<_>>().join(", ")
}

pub fn guests_hint_dismissed(state: &LocalStorageState, project_id: Uuid) -> bool {
    state.projects.iter().any(|p| p.project_id == project_id && p.guests_hint_dismissed)
}

pub fn dismiss_guests_hint(state: &mut LocalStorageState, project_id: Uuid) {
    if let Some(p) = state.projects.iter_mut().find(|p| p.project_id == project_id) {
        p.guests_hint_dismissed = true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::test_fixtures::{make_user, test_key};
    use crate::common::LocalStorageProject;

    #[test]
    fn claimed_participants_and_me_are_left_out() {
        let key = test_key();
        let mut claimed = make_user(&key, 2, "Rayane");
        claimed.claimed = true;
        let users = vec![make_user(&key, 1, "Moi"), claimed, make_user(&key, 3, "Clément"), make_user(&key, 4, "Soph")];
        assert_eq!(unclaimed_guests(&key, &users, 1), vec!["Clément".to_string(), "Soph".to_string()]);
    }

    #[test]
    fn a_name_that_does_not_decrypt_is_not_listed() {
        let key = test_key();
        let foreign = make_user(&crate::crypto::generate_key(), 5, "Eve");
        assert!(unclaimed_guests(&key, &[foreign], 1).is_empty());
    }

    #[test]
    fn every_name_is_isolated_on_its_own() {
        let line = guest_names_line(&["Clément".into(), "\u{202e}Eve".into()]);
        assert_eq!(line, "\u{2068}Clément\u{2069}, \u{2068}\u{202e}Eve\u{2069}");
    }

    #[test]
    fn dismissing_is_per_project() {
        let (a, b) = (Uuid::new_v4(), Uuid::new_v4());
        let mut state = LocalStorageState {
            projects: vec![
                LocalStorageProject { project_id: a, ..Default::default() },
                LocalStorageProject { project_id: b, ..Default::default() },
            ],
            ..Default::default()
        };
        dismiss_guests_hint(&mut state, a);
        assert!(guests_hint_dismissed(&state, a));
        assert!(!guests_hint_dismissed(&state, b));
        assert!(!guests_hint_dismissed(&state, Uuid::new_v4()));
    }

    #[test]
    fn the_dismissal_survives_a_reload_and_is_absent_until_set() {
        let entry = LocalStorageProject { project_id: Uuid::nil(), ..Default::default() };
        assert!(!serde_json::to_string(&entry).unwrap().contains("guests_hint_dismissed"));
        let dismissed = LocalStorageProject { guests_hint_dismissed: true, ..entry };
        let back: LocalStorageProject = serde_json::from_str(&serde_json::to_string(&dismissed).unwrap()).unwrap();
        assert!(back.guests_hint_dismissed);
    }
}
