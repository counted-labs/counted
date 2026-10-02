//! Every decision the project page makes, as plain functions.
//!
//! Nothing here touches a `Signal`. The page's effects read their resource guards, call in here,
//! and act on the answer — which is what makes the offline and cache fallbacks testable at all.
//! They used to live inside `use_effect` closures, where no test could reach them.

use dioxus::fullstack::ServerFnError;
use shared::{
    Expense, Payment, ProjectDto, ProjectStatus, ProjectSync, ReimbursementSuggestion, User,
    NO_CACHED_VERSION,
};
use uuid::Uuid;

use crate::common::{
    error_key, error_message, is_offline_error, is_project_gone_error, LocalStorageProject,
    LocalStorageState,
};
use crate::crypto::key_from_fragment;
use crate::decrypted::{project_name, user_name};

/// A fetch result paired with the project id it was fetched for.
///
/// `use_resource` never clears `value` on a dependency change
/// (`dioxus-hooks/src/use_resource.rs`), and the router gives `ExpensesPage` no `key:` — so
/// without the pairing a deep link between projects renders A's rows under B and poisons B's
/// cache entry against B's `data_version` permanently.
pub type For<T> = (Uuid, T);

/// `export::download_json` / `download_csv` — same shape, picked per menu entry.
pub type DownloadFn = fn(&[u8; 32], &ProjectDto, &[User], &[Expense], &[Payment]);

/// A resolved fetch, or `None` while it is pending — or while no project is open at all, which is
/// the store's resting state on `/projects`, `/login` and every other non-project route.
pub type Resolved<'a, T> = Option<&'a Result<For<T>, ServerFnError>>;

/// The resolved value, but only if it belongs to `project_id`.
fn ok_for<T>(guard: Resolved<'_, T>, project_id: Uuid) -> Option<&T> {
    guard
        .and_then(|r| r.as_ref().ok())
        .filter(|(id, _)| *id == project_id)
        .map(|(_, v)| v)
}

/// Unreachable server — the one failure the cold-start cache may paper over.
pub fn went_offline<T>(guard: Resolved<'_, T>) -> bool {
    guard.and_then(|r| r.as_ref().err()).map(is_offline_error).unwrap_or(false)
}

/// The sync's failure as (message to display, whether the server was unreachable). The second flag
/// is what lets the page fall back to cached rows for a connectivity error but not for a real one.
pub fn sync_error<T>(guard: Resolved<'_, T>) -> Option<(String, bool)> {
    guard.and_then(|r| r.as_ref().err()).map(|e| (error_message(e), is_offline_error(e)))
}

/// The `data_version` this device last cached for `project_id`, or [`NO_CACHED_VERSION`] when it
/// has no complete cache to claim.
pub fn cached_data_version(state: &LocalStorageState, project_id: Uuid) -> i64 {
    state
        .projects
        .iter()
        .find(|p| p.project_id == project_id)
        .and_then(|p| p.cached_data_version)
        .unwrap_or(NO_CACHED_VERSION)
}

/// Decided while the resource guards are held, acted on after they are dropped.
#[derive(Debug, PartialEq)]
pub enum Outcome {
    /// Not enough resolved yet, or a failure the cache may not paper over.
    Wait,
    Load {
        users: Vec<User>,
        project: Option<ProjectDto>,
        expenses: Vec<Expense>,
        payments: Vec<Payment>,
        /// The version the rows are complete at, or `None` when they are not known to be.
        version: Option<i64>,
    },
    /// "Unchanged", but the rows it meant are gone. Withdraw the version claim and ask again.
    Refetch,
    /// The response is fine but names no participants, and there is no cache to fill the gap — a
    /// server predating the merged sync, met by a device that has never opened this project.
    /// Nothing arrives on its own from here, so the page offers a retry instead of a skeleton it
    /// would sit on for ever.
    Unusable,
}

/// What the one body resource adds up to, resolved against the offline cache.
pub fn decide(
    sync: Resolved<'_, ProjectSync>,
    cached: Option<&LocalStorageProject>,
    project_id: Uuid,
) -> Outcome {
    let cached_rows = || {
        cached
            .and_then(|c| c.cached_expenses.clone())
            .zip(cached.and_then(|c| c.cached_payments.clone()))
    };
    let cached_users = || cached.and_then(|c| c.cached_users.clone());
    let cached_project = || cached.and_then(|c| c.cached_project.clone());

    let Some(s) = ok_for(sync, project_id) else {
        // Connectivity failures only — a genuine server error must surface, not be papered over
        // with stale rows. Offline renders what was cached, under the version it was cached at: the
        // cache effect writes these rows straight back, and a `None` there withdrew the claim, so
        // the next online open re-downloaded the whole project.
        if !went_offline(sync) {
            return Outcome::Wait;
        }
        return match (cached_users(), cached_rows()) {
            (Some(users), Some((expenses, payments))) => Outcome::Load {
                users,
                project: cached_project(),
                expenses,
                payments,
                version: cached.and_then(|c| c.cached_data_version),
            },
            _ => Outcome::Wait,
        };
    };

    // A server predating the merged sync omits both. Falling back to the cache is what keeps a
    // rollback from rendering a project with no participants — and when there is no cache either,
    // the page says so rather than waiting on a response that will never carry them.
    let Some(users) = s.users.clone().or_else(cached_users) else { return Outcome::Unusable };
    let project = s.project.clone().or_else(cached_project);

    match (s.expenses.clone(), s.payments.clone()) {
        (Some(expenses), Some(payments)) => {
            Outcome::Load { users, project, expenses, payments, version: Some(s.data_version) }
        }
        // "Unchanged": the held rows are current — unless the cache was cleared since the claim,
        // in which case believing the server strands the page on its skeleton.
        _ => match cached_rows() {
            Some((expenses, payments)) => {
                Outcome::Load { users, project, expenses, payments, version: Some(s.data_version) }
            }
            None => Outcome::Refetch,
        },
    }
}

/// The project key: the URL fragment first, then whatever this device stored for the project.
///
/// The fragment wins because it is how a share link hands the key over — the stored one may be
/// absent on a first open, and a link is never opened for a project it does not name.
pub fn key_for_project(
    fragment: Option<&str>,
    state: &LocalStorageState,
    project_id: Uuid,
) -> Option<[u8; 32]> {
    let from_fragment =
        fragment.filter(|f| !f.is_empty()).and_then(|f| key_from_fragment(f).ok());
    if from_fragment.is_some() {
        return from_fragment;
    }
    state
        .projects
        .iter()
        .find(|p| p.project_id == project_id)
        .and_then(|p| p.encryption_key.as_deref())
        .and_then(|k| key_from_fragment(k).ok())
}

/// What the page header should show.
#[derive(Debug, Clone, PartialEq)]
pub enum HeaderState {
    Loading,
    /// The project was deleted server-side — the only exit from a dangling share link.
    Gone,
    /// A translation key, not a sentence: the lookup belongs on the render path, and keeping it
    /// a key is what lets `header_state` be unit-tested without a Dioxus runtime.
    Error(&'static str),
    /// Offline with nothing but the cached project row: the name renders, the actions do not.
    /// Every dropdown entry needs the live project.
    Cached { title: String },
    Ready { title: String, status: ProjectStatus, read_only: bool },
}

pub fn header_state(
    sync: Resolved<'_, ProjectSync>,
    cached_project: Option<&ProjectDto>,
    key: &[u8; 32],
    project_id: Uuid,
) -> HeaderState {
    match sync {
        None => HeaderState::Loading,
        Some(Err(e)) if is_project_gone_error(e) => HeaderState::Gone,
        Some(Err(e)) => match (is_offline_error(e), cached_project) {
            (true, Some(p)) => HeaderState::Cached { title: project_name(key, p) },
            _ => HeaderState::Error(error_key(e)),
        },
        // `id == project_id`: the resource may still serve the previous project, and a wrongly
        // titled header is the most visible form of that.
        Some(Ok((id, s))) if *id == project_id => match &s.project {
            Some(p) => HeaderState::Ready {
                title: project_name(key, p),
                status: p.status.clone(),
                read_only: p.read_only,
            },
            // A server predating the merged sync; the cached row still names the project, but
            // every dropdown action needs the live one. With no cached row there is nothing to
            // title the header with and nothing further to wait for — a spinner here never stops.
            None => match cached_project {
                Some(p) => HeaderState::Cached { title: project_name(key, p) },
                None => HeaderState::Error("error-generic"),
            },
        },
        // A result for a project this page has already navigated away from.
        Some(Ok(_)) => HeaderState::Loading,
    }
}

/// (name, amount, payer id, debtor id) for the transfer modal a reimbursement suggestion opens.
/// `None` when either participant is no longer in the project.
pub fn transfer_preset(
    key: &[u8; 32],
    users: &[User],
    s: &ReimbursementSuggestion,
) -> Option<(String, f64, i32, i32)> {
    let debtor = users.iter().find(|u| u.id == s.user_id_debtor)?;
    let payer = users.iter().find(|u| u.id == s.user_id_payer)?;
    let name = format!("Remboursement {} vers {}", user_name(key, debtor), user_name(key, payer));
    Some((name, s.amount, s.user_id_debtor, s.user_id_payer))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::test_fixtures::{make_expense, make_payment, make_project, make_user, test_key};
    use crate::crypto::key_to_fragment;
    use dioxus::fullstack::RequestError;
    use shared::ExpenseType;

    fn pid() -> Uuid {
        Uuid::from_u128(1)
    }

    fn other_pid() -> Uuid {
        Uuid::from_u128(2)
    }

    fn offline() -> ServerFnError {
        ServerFnError::Request(RequestError::Timeout("after 30s".into()))
    }

    fn boom() -> ServerFnError {
        ServerFnError::new("boom")
    }

    fn gone() -> ServerFnError {
        ServerFnError::ServerError {
            message: shared::PROJECT_NOT_FOUND.into(),
            code: 404,
            details: None,
        }
    }

    fn sync_ok(id: Uuid, version: i64, rows: bool) -> Option<Result<For<ProjectSync>, ServerFnError>> {
        let key = test_key();
        let (expenses, payments) = match rows {
            true => (
                Some(vec![make_expense(&key, 1, "Pizza", 10.0, ExpenseType::Expense, "2025-01-01")]),
                Some(vec![make_payment(&key, 1, 1, 1, false, 10.0)]),
            ),
            false => (None, None),
        };
        Some(Ok((
            id,
            ProjectSync {
                data_version: version,
                project: Some(make_project(&key, id, "Live", "EUR")),
                users: Some(vec![make_user(&key, 1, "Alice")]),
                expenses,
                payments,
                recurring: None,
                server_date: None,
            },
        )))
    }

    /// What a server predating the merged sync sends: the rows, and neither the project nor the
    /// participants.
    fn legacy_sync(id: Uuid, version: i64) -> Option<Result<For<ProjectSync>, ServerFnError>> {
        let Some(Ok((id, mut s))) = sync_ok(id, version, true) else { unreachable!() };
        s.project = None;
        s.users = None;
        Some(Ok((id, s)))
    }

    /// A cache entry holding rows, users and the project row.
    fn full_cache() -> LocalStorageProject {
        let key = test_key();
        LocalStorageProject {
            project_id: pid(),
            cached_project: Some(make_project(&key, pid(), "Trip", "EUR")),
            cached_users: Some(vec![make_user(&key, 9, "Cached")]),
            cached_expenses: Some(vec![make_expense(
                &key,
                7,
                "Cached expense",
                5.0,
                ExpenseType::Expense,
                "2025-01-01",
            )]),
            cached_payments: Some(vec![make_payment(&key, 7, 7, 9, false, 5.0)]),
            cached_data_version: Some(42),
            ..Default::default()
        }
    }

    #[test]
    fn nothing_resolved_waits() {
        assert_eq!(decide(None, None, pid()), Outcome::Wait);
    }

    // The whole point of the merged sync: one response carries the project and its participants,
    // so nothing else has to be fetched for the page to load.
    #[test]
    fn sync_carrying_rows_loads_them_and_claims_the_version() {
        let out = decide(sync_ok(pid(), 7, true).as_ref(), None, pid());
        let Outcome::Load { users, project, expenses, payments, version } = out else {
            panic!("expected Load, got {out:?}")
        };
        assert_eq!(version, Some(7));
        assert_eq!(users[0].id, 1);
        assert!(project.is_some());
        assert_eq!(expenses.len(), 1);
        assert_eq!(payments.len(), 1);
    }

    // "Unchanged" withholds the rows but never the project or the participants — that is what lets
    // a rename or an added participant land without a full expense re-download.
    #[test]
    fn unchanged_with_a_cache_loads_the_cached_rows_and_still_claims_the_version() {
        let cache = full_cache();
        let out = decide(sync_ok(pid(), 42, false).as_ref(), Some(&cache), pid());
        let Outcome::Load { users, project, expenses, version, .. } = out else {
            panic!("expected Load")
        };
        assert_eq!(version, Some(42));
        assert_eq!(expenses[0].id, 7, "rows come from the cache");
        assert_eq!(users[0].id, 1, "participants come from the response, not the cache");
        assert!(project.is_some());
    }

    // Believing "unchanged" with no rows to show strands the page on its skeleton forever.
    #[test]
    fn unchanged_with_a_cleared_cache_refetches() {
        let out = decide(sync_ok(pid(), 42, false).as_ref(), None, pid());
        assert_eq!(out, Outcome::Refetch);
    }

    #[test]
    fn offline_with_a_cache_loads_it_and_keeps_its_version() {
        let cache = LocalStorageProject { cached_data_version: Some(42), ..full_cache() };
        let out = decide(Some(&Err(offline())), Some(&cache), pid());
        let Outcome::Load { users, expenses, version, .. } = out else { panic!("expected Load") };
        // Written back as it was read: `None` here withdrew the claim, and the next online open
        // re-downloaded the whole project.
        assert_eq!(version, Some(42));
        assert_eq!(users[0].id, 9);
        assert_eq!(expenses[0].id, 7);
    }

    #[test]
    fn offline_with_no_cache_waits() {
        let out = decide(Some(&Err(offline())), None, pid());
        assert_eq!(out, Outcome::Wait);
    }

    // A genuine server error must surface as an error, never as yesterday's rows.
    #[test]
    fn a_server_error_never_falls_back_to_the_cache() {
        let cache = full_cache();
        let out = decide(Some(&Err(boom())), Some(&cache), pid());
        assert_eq!(out, Outcome::Wait);
    }

    #[test]
    fn a_result_for_another_project_is_ignored() {
        let out = decide(sync_ok(other_pid(), 7, true).as_ref(), None, pid());
        assert_eq!(out, Outcome::Wait);
    }

    // Rolled back to a server that predates the merged sync: the rows arrive, the participants do
    // not, and the cache is what keeps the page whole.
    #[test]
    fn an_old_server_omitting_users_falls_back_to_the_cache() {
        let cache = full_cache();
        let out = decide(legacy_sync(pid(), 7).as_ref(), Some(&cache), pid());
        let Outcome::Load { users, project, expenses, .. } = out else { panic!("expected Load") };
        assert_eq!(users[0].id, 9, "participants come from the cache");
        assert_eq!(project.map(|p| p.id), Some(pid()));
        assert_eq!(expenses[0].id, 1, "rows still come from the response");
    }

    // Rendering a project with no participants at all would be worse than waiting: every expense
    // would show an unknown payer. Waiting *for ever* is worse than both — the response is final,
    // so the page has to say so and offer a retry.
    #[test]
    fn an_old_server_omitting_users_with_no_cache_is_unusable() {
        assert_eq!(decide(legacy_sync(pid(), 7).as_ref(), None, pid()), Outcome::Unusable);
    }

    // Only a *resolved* response is unusable. Pending, offline and another project's result are
    // all still worth waiting on, and must never surface an error.
    #[test]
    fn nothing_resolved_is_never_unusable() {
        assert_eq!(decide(None, None, pid()), Outcome::Wait);
        assert_eq!(decide(Some(&Err(offline())), None, pid()), Outcome::Wait);
        assert_eq!(decide(legacy_sync(other_pid(), 7).as_ref(), None, pid()), Outcome::Wait);
    }

    #[test]
    fn cached_data_version_defaults_to_no_cached_version() {
        let state = LocalStorageState::default();
        assert_eq!(cached_data_version(&state, pid()), NO_CACHED_VERSION);
    }

    #[test]
    fn cached_data_version_reads_the_matching_project() {
        let state = LocalStorageState { projects: vec![full_cache()], ..Default::default() };
        assert_eq!(cached_data_version(&state, pid()), 42);
        assert_eq!(cached_data_version(&state, other_pid()), NO_CACHED_VERSION);
    }

    #[test]
    fn the_fragment_key_wins_over_the_stored_one() {
        let stored = [7u8; 32];
        let state = LocalStorageState {
            projects: vec![LocalStorageProject {
                project_id: pid(),
                encryption_key: Some(key_to_fragment(&stored)),
                ..Default::default()
            }],
            ..Default::default()
        };
        let fragment = key_to_fragment(&test_key());
        assert_eq!(key_for_project(Some(&fragment), &state, pid()), Some(test_key()));
    }

    #[test]
    fn a_malformed_fragment_falls_through_to_the_stored_key() {
        let state = LocalStorageState {
            projects: vec![LocalStorageProject {
                project_id: pid(),
                encryption_key: Some(key_to_fragment(&test_key())),
                ..Default::default()
            }],
            ..Default::default()
        };
        assert_eq!(key_for_project(Some("not-a-key"), &state, pid()), Some(test_key()));
        assert_eq!(key_for_project(Some(""), &state, pid()), Some(test_key()));
        assert_eq!(key_for_project(None, &state, pid()), Some(test_key()));
    }

    #[test]
    fn no_fragment_and_no_stored_key_is_none() {
        let state = LocalStorageState::default();
        assert_eq!(key_for_project(None, &state, pid()), None);
    }

    #[test]
    fn a_deleted_project_gets_the_gone_recovery() {
        let out = header_state(Some(&Err(gone())), None, &test_key(), pid());
        assert_eq!(out, HeaderState::Gone);
    }

    #[test]
    fn offline_shows_the_cached_name_without_the_actions() {
        let key = test_key();
        let cached = make_project(&key, pid(), "Trip", "EUR");
        let out = header_state(Some(&Err(offline())), Some(&cached), &key, pid());
        assert_eq!(out, HeaderState::Cached { title: "Trip".to_string() });
    }

    #[test]
    fn offline_with_no_cached_project_shows_the_error() {
        let out = header_state(Some(&Err(offline())), None, &test_key(), pid());
        assert!(matches!(out, HeaderState::Error(_)));
    }

    // The cache must not paper over a genuine failure here either.
    #[test]
    fn a_server_error_shows_the_error_even_with_a_cached_project() {
        let key = test_key();
        let cached = make_project(&key, pid(), "Trip", "EUR");
        let out = header_state(Some(&Err(boom())), Some(&cached), &key, pid());
        assert!(matches!(out, HeaderState::Error(_)));
    }

    #[test]
    fn a_loaded_project_titles_the_header() {
        let out = header_state(sync_ok(pid(), 7, true).as_ref(), None, &test_key(), pid());
        assert_eq!(
            out,
            HeaderState::Ready { title: "Live".to_string(), status: ProjectStatus::Ongoing, read_only: false }
        );
    }

    // The header moved onto the sync resource, so the dangling-share-link recovery has to come
    // through it now — `get_data_version` 404s with the same `PROJECT_NOT_FOUND` as `get_project`.
    #[test]
    fn a_project_gone_error_survives_the_move_to_the_sync_resource() {
        assert_eq!(header_state(Some(&Err(gone())), None, &test_key(), pid()), HeaderState::Gone);
    }

    // An old server sends no project row; the cached name beats a spinner, and the actions stay
    // hidden because every one of them needs the live row.
    #[test]
    fn an_old_server_omitting_the_project_shows_the_cached_name() {
        let key = test_key();
        let cached = make_project(&key, pid(), "Trip", "EUR");
        let out = header_state(legacy_sync(pid(), 7).as_ref(), Some(&cached), &key, pid());
        assert_eq!(out, HeaderState::Cached { title: "Trip".to_string() });
    }

    // Nothing names the project and nothing more is coming: a spinner here spins for ever.
    #[test]
    fn an_old_server_omitting_the_project_with_no_cache_shows_the_error() {
        let out = header_state(legacy_sync(pid(), 7).as_ref(), None, &test_key(), pid());
        assert!(matches!(out, HeaderState::Error(_)));
    }

    #[test]
    fn a_result_for_another_project_keeps_the_header_loading() {
        let out = header_state(sync_ok(other_pid(), 7, true).as_ref(), None, &test_key(), pid());
        assert_eq!(out, HeaderState::Loading);
    }

    #[test]
    fn a_transfer_preset_names_both_participants() {
        let key = test_key();
        let users = vec![make_user(&key, 1, "Alice"), make_user(&key, 2, "Bob")];
        let s = ReimbursementSuggestion { amount: 12.5, user_id_debtor: 1, user_id_payer: 2 };
        let (name, amount, payer, debtor) = transfer_preset(&key, &users, &s).unwrap();
        assert_eq!(name, "Remboursement Alice vers Bob");
        assert_eq!(amount, 12.5);
        assert_eq!((payer, debtor), (1, 2));
    }

    #[test]
    fn a_transfer_preset_needs_both_participants() {
        let key = test_key();
        let users = vec![make_user(&key, 1, "Alice")];
        let s = ReimbursementSuggestion { amount: 12.5, user_id_debtor: 1, user_id_payer: 2 };
        assert_eq!(transfer_preset(&key, &users, &s), None);
    }
}
