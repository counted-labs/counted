use shared::User;
use uuid::Uuid;

use crate::common::LocalStorageState;
use crate::expenses::group_security::AdminAccess;

/// SPEC 14.3: a project key sent in a classical invitation falls to a quantum adversary holding the
/// database or a backup. An accepted invitation leaves no row, so every project still on its first
/// key with another account member is asked to change it, once per admin device.
pub fn first_key_prompt(access: Option<&AdminAccess>, key_epoch: i32, users: &[User], me: Option<i32>, dismissed: bool) -> bool {
    access == Some(&AdminAccess::Admin) && key_epoch == 0 && !dismissed && users.iter().any(|u| u.claimed && Some(u.id) != me)
}

pub fn first_key_hint_dismissed(state: &LocalStorageState, project_id: Uuid) -> bool {
    state.projects.iter().any(|p| p.project_id == project_id && p.first_key_hint_dismissed)
}

pub fn dismiss_first_key_hint(state: &mut LocalStorageState, project_id: Uuid) {
    if let Some(p) = state.projects.iter_mut().find(|p| p.project_id == project_id) {
        p.first_key_hint_dismissed = true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::test_fixtures::{make_user, test_key};
    use crate::common::LocalStorageProject;

    fn roster() -> Vec<User> {
        let key = test_key();
        let mut friend = make_user(&key, 2, "Rayane");
        friend.claimed = true;
        let mut me = make_user(&key, 1, "Moi");
        me.claimed = true;
        vec![me, friend, make_user(&key, 3, "Clément")]
    }

    #[test]
    fn an_admin_of_a_first_key_project_with_another_account_is_asked() {
        assert!(first_key_prompt(Some(&AdminAccess::Admin), 0, &roster(), Some(1), false));
    }

    #[test]
    fn a_changed_key_a_dismissal_or_another_role_is_not_asked() {
        let users = roster();
        assert!(!first_key_prompt(Some(&AdminAccess::Admin), 1, &users, Some(1), false), "key changed");
        assert!(!first_key_prompt(Some(&AdminAccess::Admin), 0, &users, Some(1), true), "dismissed");
        assert!(!first_key_prompt(Some(&AdminAccess::AskAdmins(vec![])), 0, &users, Some(1), false), "not admin");
        assert!(!first_key_prompt(None, 0, &users, Some(1), false), "below the floor");
    }

    #[test]
    fn a_project_where_only_this_account_holds_a_participant_is_not_asked() {
        let users: Vec<User> = roster().into_iter().filter(|u| u.id != 2).collect();
        assert!(!first_key_prompt(Some(&AdminAccess::Admin), 0, &users, Some(1), false));
    }

    #[test]
    fn later_is_kept_per_project() {
        let (asked, other) = (Uuid::new_v4(), Uuid::new_v4());
        let mut state = LocalStorageState {
            projects: vec![
                LocalStorageProject { project_id: asked, ..Default::default() },
                LocalStorageProject { project_id: other, ..Default::default() },
            ],
            ..Default::default()
        };
        dismiss_first_key_hint(&mut state, asked);
        assert!(first_key_hint_dismissed(&state, asked));
        assert!(!first_key_hint_dismissed(&state, other));
    }
}
