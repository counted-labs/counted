//! Fast entry into the account's "Mes dépenses" project, from the projects page (designs P4, P5).
//! Nothing here renders unless the device holds an open personal project.

use api::users::users_controller::get_users_by_project_id;
use dioxus::prelude::*;
use shared::{ExpenseWithPayments, ProjectDto, ProjectPayload, ProjectStatus, User};
use uuid::Uuid;

use crate::common::{key_of, read_from_ls, Flash, LocalStorageState};
use crate::crypto::decrypt_json;
use crate::decrypted::project_status;
use crate::expenses::hooks::use_project_store::ProjectStore;
use crate::expenses::tabs::expenses_tab::ExpenseMutation;
use crate::expenses::AddExpenseModal;
use crate::tid;

#[derive(Clone, Debug, PartialEq)]
pub struct PersonalProject {
    pub project_id: Uuid,
    pub key: [u8; 32],
    pub currency: String,
    pub user_id: Option<i32>,
}

/// The account's open personal project among `list`, decrypted with the keys this device holds.
/// Which one it is comes from the account, never from the payload's `kind` (audit 2026-10-19
/// M3). A closed or archived one takes no new expense, so it offers no shortcut either.
pub fn personal_project(list: &[ProjectDto], state: &LocalStorageState) -> Option<PersonalProject> {
    list.iter().filter(|p| state.personal_project == Some(p.id)).find_map(|p| {
        let entry = state.projects.iter().find(|lp| lp.project_id == p.id)?;
        let key = key_of(entry)?;
        let payload: ProjectPayload = decrypt_json(&key, &p.payload).ok()?;
        (project_status(Some(&key), p) == ProjectStatus::Ongoing).then_some(PersonalProject {
            project_id: p.id,
            key,
            currency: payload.currency,
            user_id: entry.user_id,
        })
    })
}

/// The account is the project's only participant, so it pays and owes every expense.
fn me(users: &[User], stored: Option<i32>) -> Option<i32> {
    stored.or(match users {
        [only] => Some(only.id),
        _ => None,
    })
}

#[component]
pub fn PersonalExpenseSheet(project: PersonalProject, on_close: EventHandler<()>) -> Element {
    let store = use_context::<ProjectStore>();
    let mut flash = use_context::<Signal<Option<Flash>>>();
    let project_id = project.project_id;
    // Offline, the cached participants still let the expense queue.
    let users = use_resource(move || async move {
        crate::common::project_token::scope(project_id);
        match get_users_by_project_id(project_id).await {
            Ok(users) => users,
            Err(_) => read_from_ls()
                .projects
                .into_iter()
                .find(|p| p.project_id == project_id)
                .and_then(|p| p.cached_users)
                .unwrap_or_default(),
        }
    });

    let Some(users) = users() else { return rsx! {} };
    rsx! {
        AddExpenseModal {
            on_close,
            on_created: move |created: Option<ExpenseWithPayments>| {
                store.on_expenses_changed.call(match created {
                    Some(c) => ExpenseMutation::Added(c),
                    None => ExpenseMutation::Queued,
                });
                flash.set(Some(Flash::ok(tid!("personal-expense-banner"))));
                on_close.call(());
            },
            project_id,
            stored_user_id: me(&users, project.user_id),
            users,
            currency: project.currency.clone(),
            project_key: Some(project.key),
            personal: true,
            initial_name: None,
            initial_amount: None,
            initial_expense_type: None,
            initial_payer_id: None,
            initial_debtor_id: None,
            initial_date: None,
            initial_category: None,
            amount_hint: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::test_fixtures::{make_project, make_user, test_key};
    use crate::common::LocalStorageProject;
    use crate::crypto::encrypt_json;

    fn held(project: &ProjectDto, key: &[u8; 32], user_id: Option<i32>) -> LocalStorageProject {
        LocalStorageProject {
            project_id: project.id,
            user_id,
            encryption_key: Some(crate::crypto::key_to_fragment(key)),
            ..Default::default()
        }
    }

    fn mark(key: &[u8; 32], p: &mut ProjectDto, kind: &str, status: Option<ProjectStatus>) {
        let mut payload: ProjectPayload = decrypt_json(key, &p.payload).unwrap();
        payload.extra.insert("kind".into(), kind.into());
        payload.status = status;
        p.payload = encrypt_json(key, &payload).unwrap();
    }

    fn state(projects: Vec<LocalStorageProject>) -> LocalStorageState {
        LocalStorageState { projects, ..Default::default() }
    }

    fn mine_is(id: Uuid, projects: Vec<LocalStorageProject>) -> LocalStorageState {
        LocalStorageState { personal_project: Some(id), ..state(projects) }
    }

    #[test]
    fn no_personal_project_offers_nothing() {
        let k = test_key();
        let trip = make_project(&k, Uuid::new_v4(), "Corsica", "EUR");
        assert_eq!(personal_project(std::slice::from_ref(&trip), &state(vec![held(&trip, &k, Some(1))])), None);
        assert_eq!(personal_project(&[], &state(vec![])), None);
    }

    #[test]
    fn the_personal_project_is_found_with_its_key_currency_and_identity() {
        let k = test_key();
        let trip = make_project(&k, Uuid::new_v4(), "Corsica", "EUR");
        let mut mine = make_project(&k, Uuid::new_v4(), "Mes dépenses", "CHF");
        mark(&k, &mut mine, "personal", None);
        let s = mine_is(mine.id, vec![held(&trip, &k, Some(1)), held(&mine, &k, Some(9))]);
        assert_eq!(
            personal_project(&[trip, mine.clone()], &s),
            Some(PersonalProject { project_id: mine.id, key: k, currency: "CHF".into(), user_id: Some(9) })
        );
    }

    #[test]
    fn a_project_this_device_holds_no_key_for_is_skipped() {
        let k = test_key();
        let mut mine = make_project(&k, Uuid::new_v4(), "Mes dépenses", "EUR");
        mark(&k, &mut mine, "personal", None);
        assert_eq!(personal_project(&[mine.clone()], &mine_is(mine.id, vec![])), None);
    }

    /// Audit 2026-10-19 M3: a co-member who writes `kind: personal` into a shared project does not
    /// make it where this account's personal expenses go.
    #[test]
    fn a_shared_project_marked_personal_is_not_the_shortcut_target() {
        let k = test_key();
        let mine = make_project(&k, Uuid::new_v4(), "Mes dépenses", "EUR");
        let mut trip = make_project(&k, Uuid::new_v4(), "Corsica", "EUR");
        mark(&k, &mut trip, "personal", None);
        let held_both = vec![held(&mine, &k, Some(1)), held(&trip, &k, Some(2))];

        assert_eq!(personal_project(&[trip.clone()], &state(held_both.clone())), None);
        assert_eq!(personal_project(&[trip, mine.clone()], &mine_is(mine.id, held_both)).map(|p| p.project_id), Some(mine.id));
    }

    #[test]
    fn a_closed_or_archived_personal_project_takes_no_fast_entry() {
        let k = test_key();
        for status in [ProjectStatus::Closed, ProjectStatus::Archived] {
            let mut mine = make_project(&k, Uuid::new_v4(), "Mes dépenses", "EUR");
            mark(&k, &mut mine, "personal", Some(status));
            assert_eq!(personal_project(&[mine.clone()], &mine_is(mine.id, vec![held(&mine, &k, Some(1))])), None);
        }
    }

    #[test]
    fn another_kind_is_not_personal() {
        let k = test_key();
        let mut direct = make_project(&k, Uuid::new_v4(), "Alice", "EUR");
        mark(&k, &mut direct, "direct", None);
        assert_eq!(personal_project(&[direct.clone()], &state(vec![held(&direct, &k, Some(1))])), None);
    }

    #[test]
    fn the_only_participant_is_me_when_no_identity_is_stored() {
        let k = test_key();
        let alone = vec![make_user(&k, 4, "Moi")];
        assert_eq!(me(&alone, None), Some(4));
        assert_eq!(me(&alone, Some(7)), Some(7), "a stored identity wins");
        let two = vec![make_user(&k, 4, "Moi"), make_user(&k, 5, "Autre")];
        assert_eq!(me(&two, None), None);
    }
}
