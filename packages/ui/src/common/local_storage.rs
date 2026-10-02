use dioxus::prelude::{ReadableExt, Signal, WritableExt};
use serde::{Deserialize, Serialize};
use shared::{Expense, Payment, ProjectDto, User};
use uuid::Uuid;

use super::persist::{read_json, write_json};
use crate::crypto::key_from_fragment;

const LS_KEY: &str = "counted_local_storage";

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct LocalStorageState {
    pub projects: Vec<LocalStorageProject>,
    #[serde(default)]
    pub onboarding_seen: bool,
    /// Invisible to SSR like `onboarding_seen` — read it post-mount or hydration breaks.
    #[serde(default)]
    pub show_archived: bool,
    #[serde(default)]
    pub cached_projects_list: Option<Vec<ProjectDto>>,
    /// Base64url account key derived from the password at login. Persisted because `me()` restores
    /// the session from the cookie without the password, and reloads still decrypt the display name.
    #[serde(default)]
    pub account_key: Option<String>,
    /// Chosen UI language, or `None` to follow `Accept-Language`. Invisible to SSR like
    /// `onboarding_seen` — the *cookie* is what the server reads; this is the native store and the
    /// recovery path when the cookie is cleared. Applied post-mount, never on the first render.
    #[serde(default)]
    pub language: Option<String>,
}

/// False server-side (non-wasm, non-mobile native).
pub fn is_mobile() -> bool {
    #[cfg(target_arch = "wasm32")]
    {
        return web_sys::window()
            .and_then(|w| w.navigator().user_agent().ok())
            .map(|ua| {
                let ua = ua.to_lowercase();
                ua.contains("android")
                    || ua.contains("iphone")
                    || ua.contains("ipad")
                    || ua.contains("mobile")
            })
            .unwrap_or(false);
    }
    #[cfg(any(target_os = "android", target_os = "ios"))]
    return true;
    #[cfg(not(any(target_arch = "wasm32", target_os = "android", target_os = "ios")))]
    return false;
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct LocalStorageProject {
    pub project_id: Uuid,
    pub user_id: Option<i32>,
    /// Anonymous membership, set when this device joined without an account. Random and
    /// per-project: a reused id would let the server group a person's projects together. Only ever
    /// sent in a request body — never a URL, which nginx logs.
    #[serde(default)]
    pub anon_member_id: Option<Uuid>,
    #[serde(default)]
    pub encryption_key: Option<String>,
    #[serde(default)]
    pub cached_project: Option<ProjectDto>,
    #[serde(default)]
    pub cached_users: Option<Vec<User>>,
    #[serde(default)]
    pub cached_expenses: Option<Vec<Expense>>,
    #[serde(default)]
    pub cached_payments: Option<Vec<Payment>>,
    /// The `data_version` the rows above were written at, sent back on the next open so the server
    /// can answer "unchanged". Absent means "no usable cache", which a truncated cache also
    /// reports — see [`set_project_cache`].
    #[serde(default)]
    pub cached_data_version: Option<i64>,
    /// The account whose `account_projects` row this entry was last seen in. What lets
    /// reconciliation tell "the account never had this" (push it) from "the account left it on
    /// another device" (forget it) — see `account_sync::left_elsewhere`. Keyed by account rather
    /// than a flag so a different account signing in here still inherits the list.
    #[serde(default)]
    pub synced_account: Option<Uuid>,
}

/// How many expenses are cached per project. **Web is the only target with a ceiling.**
///
/// localStorage is a hard ~5 MB per origin charged in UTF-16 units, so ASCII JSON costs two bytes
/// per character: a 2000-expense project is ~2.8 MB of JSON, ~5.6 MB of quota — over the wall, and
/// unraisable (IndexedDB/Cache/OPFS have a different quota, unreachable from here). So the web
/// cache is bounded by construction: 300 expenses and their payments is ~450 KB and cannot grow
/// with the project. Native writes a plain file, no quota, caches everything.
#[cfg(target_arch = "wasm32")]
const MAX_CACHED_EXPENSES: usize = 300;
#[cfg(not(target_arch = "wasm32"))]
const MAX_CACHED_EXPENSES: usize = usize::MAX;

/// Only the tests need the resolved path; the read/write pair goes through `persist` by key.
#[cfg(all(test, not(target_arch = "wasm32")))]
fn storage_file() -> std::path::PathBuf {
    super::persist::file_for(LS_KEY)
}

pub fn read_from_ls() -> LocalStorageState {
    read_json(LS_KEY)
}

/// True when everything but the caches reached storage.
///
/// There is no bound across projects, so enough of them at the per-project ceiling fill the web
/// quota, and a failed write used to lose whatever it carried — a newly joined project's key
/// lived in memory only and was gone on reload. The retry drops every cached row: keys, user ids
/// and anonymous memberships are a few hundred bytes a project, and the caches refill on the next
/// sync.
pub fn write_to_ls(state: &LocalStorageState) -> bool {
    if write_json(LS_KEY, "local storage", state) {
        return true;
    }
    let mut lean = state.clone();
    drop_caches(&mut lean);
    write_json(LS_KEY, "local storage without caches", &lean)
}

fn drop_caches(state: &mut LocalStorageState) {
    state.cached_projects_list = None;
    for p in &mut state.projects {
        p.cached_project = None;
        p.cached_users = None;
        p.cached_expenses = None;
        p.cached_payments = None;
        p.cached_data_version = None;
    }
}

/// Only reachable when even the caches-free store did not fit, or storage is gone altogether.
fn report_unsaved() {
    use crate::common::Flash;
    if let Some(mut flash) = dioxus::prelude::try_consume_context::<Signal<Option<Flash>>>() {
        flash.set(Some(Flash::err(crate::tid!("error-storage-full"))));
    }
}

/// Every store mutation goes through here.
///
/// The base is `read_from_ls()`, never the `Signal`: the signal is seeded once at mount
/// (`app_contexts::use_app_contexts`) and never re-read, so any writer that serialised it back to
/// disk overwrote whatever another writer had put there since. That is how project keys and
/// `anon_member_id`s went missing — `use_project_store` wrote a key resolved from a share link to
/// disk alone, and the next signal-based write erased it.
pub fn update_ls(mut ls_ctx: Signal<LocalStorageState>, f: impl FnOnce(&mut LocalStorageState)) {
    let mut state = read_from_ls();
    f(&mut state);
    if !write_to_ls(&state) {
        report_unsaved();
    }
    ls_ctx.set(state);
}

/// [`update_ls`], but writes nothing when the mutation changed nothing.
///
/// Only for the hot paths: serialising the store and notifying every `ls_ctx` reader is expensive
/// at scale, and an unchanged revisit is the common case. `peek` on purpose — subscribing here
/// would re-run the caller's effect on its own write.
pub fn update_ls_if_changed(
    mut ls_ctx: Signal<LocalStorageState>,
    f: impl FnOnce(&mut LocalStorageState),
) {
    let mut state = read_from_ls();
    f(&mut state);
    if state != *ls_ctx.peek() {
        if !write_to_ls(&state) {
            report_unsaved();
        }
        ls_ctx.set(state);
    }
}

/// The entry for `project_id`, inserted with defaults if this device has never seen it.
fn entry_mut(state: &mut LocalStorageState, project_id: Uuid) -> &mut LocalStorageProject {
    if let Some(i) = state.projects.iter().position(|p| p.project_id == project_id) {
        return &mut state.projects[i];
    }
    state.projects.push(LocalStorageProject { project_id, ..Default::default() });
    state.projects.last_mut().expect("just pushed")
}

/// Upsert a project entry. Does not overwrite an existing user_id with None.
pub fn upsert_project(state: &mut LocalStorageState, project_id: Uuid, user_id: Option<i32>) {
    let entry = entry_mut(state, project_id);
    if user_id.is_some() {
        entry.user_id = user_id;
    }
}

/// Forget which participant this device is. The one thing [`upsert_project`] deliberately cannot
/// do, and the only way to reopen the "who am I?" picker: `ExpensesPage` shows it exactly when the
/// stored `user_id` is `None`.
///
/// Called when the server refuses a claim because another account holds that participant. Nothing
/// else is touched — the key, the cache and the anonymous membership all stay.
pub fn clear_user_id(state: &mut LocalStorageState, project_id: Uuid) {
    if let Some(entry) = state.projects.iter_mut().find(|p| p.project_id == project_id) {
        entry.user_id = None;
    }
}

/// Record the anonymous membership the server accepted for this device. Creates the entry if absent.
pub fn set_anon_member_id(state: &mut LocalStorageState, project_id: Uuid, member_id: Uuid) {
    entry_mut(state, project_id).anon_member_id = Some(member_id);
}

/// Drops the entry (encryption key included) and its row in the cached list. Without it a stale
/// entry survives and re-adding the share link resurrects a dangling id.
pub fn remove_project(state: &mut LocalStorageState, project_id: Uuid) {
    state.projects.retain(|p| p.project_id != project_id);
    if let Some(ref mut list) = state.cached_projects_list {
        list.retain(|p| p.id != project_id);
    }
}

/// Records that `account_id`'s `account_projects` row for each of these projects exists — at the
/// time of writing. Only touches entries this device holds; it never creates one.
pub fn mark_synced(state: &mut LocalStorageState, project_ids: &[Uuid], account_id: Uuid) {
    for entry in state.projects.iter_mut().filter(|p| project_ids.contains(&p.project_id)) {
        entry.synced_account = Some(account_id);
    }
}

/// Store the base64url-encoded encryption key for a project. Creates the entry if absent.
pub fn upsert_project_key(state: &mut LocalStorageState, project_id: Uuid, key: String) {
    entry_mut(state, project_id).encryption_key = Some(key);
}

/// The decoded per-project encryption key held on this device, if any.
pub fn key_of(entry: &LocalStorageProject) -> Option<[u8; 32]> {
    entry.encryption_key.as_ref().and_then(|k| key_from_fragment(k).ok())
}

/// Same, looked up by id. Prefer [`key_of`] when the caller already holds a `read_from_ls()`
/// result — this reads the whole store again.
pub fn project_key(project_id: Uuid) -> Option<[u8; 32]> {
    read_from_ls().projects.iter().find(|p| p.project_id == project_id).and_then(key_of)
}

/// Cache the last-fetched projects list (used for cold-start offline rendering).
pub fn set_cached_projects_list(state: &mut LocalStorageState, list: Vec<ProjectDto>) {
    state.cached_projects_list = Some(list);
}

/// The most recent `max` expenses and their payments. Recency is by id — `expenses.id` is a
/// `SERIAL`, so a higher id is later whatever the user-supplied date says. `None` on truncation,
/// which is how the caller knows the cache no longer covers the whole project.
fn most_recent(
    expenses: Vec<Expense>,
    payments: Vec<Payment>,
    max: usize,
) -> (Vec<Expense>, Vec<Payment>, bool) {
    if expenses.len() <= max {
        return (expenses, payments, true);
    }
    let mut kept = expenses;
    kept.sort_by_key(|e| std::cmp::Reverse(e.id));
    kept.truncate(max);
    let ids: std::collections::HashSet<i32> = kept.iter().map(|e| e.id).collect();
    let payments = payments.into_iter().filter(|p| ids.contains(&p.expense_id)).collect();
    (kept, payments, false)
}

/// No-op if the project entry does not exist.
///
/// `data_version` is the client's claim to hold the whole project, stored only when true: callers
/// pass `None` for patched or cached rows, and truncation drops it here. A false claim gets
/// "unchanged" back forever and never receives the missing rows.
pub fn set_project_cache(
    state: &mut LocalStorageState,
    project_id: Uuid,
    project: ProjectDto,
    users: Vec<User>,
    expenses: Vec<Expense>,
    payments: Vec<Payment>,
    data_version: Option<i64>,
) {
    let (expenses, payments, complete) = most_recent(expenses, payments, MAX_CACHED_EXPENSES);

    if let Some(e) = state.projects.iter_mut().find(|p| p.project_id == project_id) {
        e.cached_project = Some(project);
        e.cached_users = Some(users);
        e.cached_expenses = Some(expenses);
        e.cached_payments = Some(payments);
        e.cached_data_version = data_version.filter(|_| complete);
    }
}

pub fn user_color_class(user_id: i32) -> &'static str {
    const COLORS: &[&str] = &[
        "bg-primary",
        "bg-secondary",
        "bg-accent",
        "bg-info",
        "bg-success",
        "bg-warning",
        "bg-error",
    ];
    COLORS[(user_id.unsigned_abs() as usize) % COLORS.len()]
}

/// Up to 2 uppercase initials (first + last word); a single-word name yields one.
pub fn initials(name: &str) -> String {
    let mut parts = name.split_whitespace();
    match (parts.next(), parts.next()) {
        (Some(first), Some(last)) => format!(
            "{}{}",
            first.chars().next().unwrap_or_default().to_uppercase(),
            last.chars().next().unwrap_or_default().to_uppercase()
        ),
        (Some(first), None) => first
            .chars()
            .next()
            .map(|c| c.to_uppercase().to_string())
            .unwrap_or_default(),
        _ => "?".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // What survives a full store is exactly what cannot be fetched again.
    #[test]
    fn dropping_caches_keeps_keys_and_memberships() {
        let id = Uuid::new_v4();
        let mut state = LocalStorageState {
            projects: vec![LocalStorageProject {
                project_id: id,
                user_id: Some(3),
                anon_member_id: Some(Uuid::nil()),
                encryption_key: Some("key".into()),
                cached_expenses: Some(vec![]),
                cached_payments: Some(vec![]),
                cached_users: Some(vec![]),
                cached_data_version: Some(7),
                ..Default::default()
            }],
            cached_projects_list: Some(vec![]),
            ..Default::default()
        };
        drop_caches(&mut state);
        assert_eq!(
            state.projects[0],
            LocalStorageProject {
                project_id: id,
                user_id: Some(3),
                anon_member_id: Some(Uuid::nil()),
                encryption_key: Some("key".into()),
                ..Default::default()
            }
        );
        assert!(state.cached_projects_list.is_none());
    }

    #[cfg(not(target_arch = "wasm32"))]
    mod native {
        use super::*;
        use shared::{EncryptedPair, ProjectStatus};

        // with_data_dir takes the process-wide env_lock — offline_queue and export set the same
        // COUNTED_DATA_DIR, so a module-local lock would not keep them apart.
        use crate::common::env_lock;
        use crate::common::test_fixtures::with_data_dir;

        fn make_project_dto(id: Uuid) -> ProjectDto {
            ProjectDto {
                id,
                payload: EncryptedPair { ct: "ct".into(), iv: "iv".into() },
                status: ProjectStatus::Ongoing,
                created_at: chrono::DateTime::UNIX_EPOCH.naive_utc(),
                owner_account_id: None,
                read_only: false,
            }
        }

        // The bound is what keeps the web cache away from localStorage's ~5 MB wall however large
        // the project gets. It is `most_recent` that has to be right, not the quota.

        fn ex(id: i32) -> Expense {
            Expense {
                id,
                author_id: Some(1),
                project_id: Uuid::nil(),
                created_at: chrono::DateTime::UNIX_EPOCH.naive_utc(),
                payload: EncryptedPair { ct: "ct".into(), iv: "iv".into() },
            }
        }

        fn pm(id: i32, expense_id: i32) -> Payment {
            Payment {
                id,
                expense_id,
                user_id: 1,
                payload: EncryptedPair { ct: "ct".into(), iv: "iv".into() },
                created_at: chrono::DateTime::UNIX_EPOCH.naive_utc(),
            }
        }

        #[test]
        fn a_project_under_the_bound_is_cached_whole_and_stays_complete() {
            let expenses: Vec<Expense> = (1..=10).map(ex).collect();
            let payments: Vec<Payment> = (1..=10).map(|i| pm(i, i)).collect();
            let (e, p, complete) = most_recent(expenses, payments, 300);
            assert_eq!(e.len(), 10);
            assert_eq!(p.len(), 10);
            assert!(complete, "nothing was dropped, so the version may be claimed");
        }

        #[test]
        fn over_the_bound_only_the_newest_survive() {
            let expenses: Vec<Expense> = (1..=500).map(ex).collect();
            let payments: Vec<Payment> = (1..=500).map(|i| pm(i, i)).collect();
            let (e, _p, complete) = most_recent(expenses, payments, 300);

            assert_eq!(e.len(), 300);
            assert!(!complete, "a truncated cache must not claim to be the whole project");
            assert_eq!(e.first().unwrap().id, 500, "newest first");
            assert_eq!(e.last().unwrap().id, 201);
            assert!(e.iter().all(|x| x.id > 200));
        }

        /// A payment whose expense was dropped would be an orphan: it would still count toward the
        /// balances while the row it belongs to is invisible.
        #[test]
        fn payments_of_dropped_expenses_go_with_them() {
            let expenses: Vec<Expense> = (1..=500).map(ex).collect();
            let payments: Vec<Payment> =
                (1..=500).flat_map(|i| [pm(i * 2, i), pm(i * 2 + 1, i)]).collect();
            let (e, p, _) = most_recent(expenses, payments, 300);

            let kept: std::collections::HashSet<i32> = e.iter().map(|x| x.id).collect();
            assert_eq!(p.len(), 600, "two payments per surviving expense");
            assert!(p.iter().all(|x| kept.contains(&x.expense_id)), "no orphaned payments");
        }

        /// Exactly at the bound is not truncation — the off-by-one here would silently disable the
        /// version check for every project of exactly that size.
        #[test]
        fn exactly_at_the_bound_is_still_complete() {
            let expenses: Vec<Expense> = (1..=300).map(ex).collect();
            let (e, _, complete) = most_recent(expenses, vec![], 300);
            assert_eq!(e.len(), 300);
            assert!(complete);
        }

        // `cached_data_version` is a claim: "this device holds every row as of version N". The
        // server takes it at its word and answers "unchanged", sending nothing. Storing it when it
        // is not true is the one failure here that persists across restarts.

        fn entry(state: &LocalStorageState, id: Uuid) -> &LocalStorageProject {
            state.projects.iter().find(|p| p.project_id == id).unwrap()
        }

        fn state_with(id: Uuid) -> LocalStorageState {
            let mut s = LocalStorageState::default();
            upsert_project(&mut s, id, Some(1));
            s
        }

        #[test]
        fn a_complete_cache_claims_its_version() {
            let id = Uuid::from_u128(9);
            let mut s = state_with(id);
            set_project_cache(&mut s, id, make_project_dto(id), vec![], vec![ex(1)], vec![], Some(7));
            assert_eq!(entry(&s, id).cached_data_version, Some(7));
        }

        /// Rows carrying a local patch match no server version. Claiming the pre-patch one would
        /// have the server answer "unchanged" and this device would never see its own write again.
        #[test]
        fn rows_with_no_version_claim_none() {
            let id = Uuid::from_u128(9);
            let mut s = state_with(id);
            set_project_cache(&mut s, id, make_project_dto(id), vec![], vec![ex(1)], vec![], None);
            assert_eq!(entry(&s, id).cached_data_version, None);
            assert!(entry(&s, id).cached_expenses.is_some(), "the rows are still cached");
        }

        /// The web bound and the version claim interact: a truncated cache is not the whole project,
        /// so it must withdraw the claim even though the server did give it a version.
        #[cfg(target_arch = "wasm32")]
        #[test]
        fn a_truncated_cache_withdraws_the_claim() {
            let id = Uuid::from_u128(9);
            let mut s = state_with(id);
            let expenses: Vec<Expense> = (1..=MAX_CACHED_EXPENSES as i32 + 1).map(ex).collect();
            set_project_cache(&mut s, id, make_project_dto(id), vec![], expenses, vec![], Some(7));
            assert_eq!(entry(&s, id).cached_data_version, None);
            assert_eq!(entry(&s, id).cached_expenses.as_ref().unwrap().len(), MAX_CACHED_EXPENSES);
        }

        /// Native has no quota, so it caches whole projects and keeps the claim.
        #[cfg(not(target_arch = "wasm32"))]
        #[test]
        fn native_caches_everything_and_keeps_the_claim() {
            let id = Uuid::from_u128(9);
            let mut s = state_with(id);
            let expenses: Vec<Expense> = (1..=5_000).map(ex).collect();
            set_project_cache(&mut s, id, make_project_dto(id), vec![], expenses, vec![], Some(7));
            assert_eq!(entry(&s, id).cached_expenses.as_ref().unwrap().len(), 5_000);
            assert_eq!(entry(&s, id).cached_data_version, Some(7));
        }

        /// The regression that lost project keys.
        ///
        /// `ls_ctx` is seeded once at mount and never re-read, so a writer that serialised the
        /// signal back to disk overwrote whatever another writer had put there since. The key
        /// resolved from a share link was written to disk alone (`use_project_store`), and the
        /// account-projects sync — which runs on every projects-page load while signed in — then
        /// wrote its stale snapshot over it.
        ///
        /// A `VirtualDom` because `update_ls` takes a `Signal`, which needs a runtime to exist.
        #[test]
        fn update_ls_does_not_clobber_a_write_made_behind_the_signal() {
            use dioxus::prelude::*;

            let project = Uuid::parse_str("11111111-2222-3333-4444-555555555555").unwrap();

            let survived = with_data_dir("clobber", |_| {
                #[component]
                fn Harness(project: Uuid, out: std::rc::Rc<std::cell::Cell<bool>>) -> Element {
                    let ls_ctx = use_context_provider(|| Signal::new(read_from_ls()));

                    // Another code path persists a key straight to disk. The signal, captured at
                    // mount, knows nothing about it.
                    let mut on_disk = read_from_ls();
                    upsert_project_key(&mut on_disk, project, "AAAA".to_string());
                    write_to_ls(&on_disk);

                    // Now an unrelated mutation goes through the store, as the account sync does.
                    update_ls(ls_ctx, |state| state.onboarding_seen = true);

                    let after = read_from_ls();
                    out.set(
                        after.onboarding_seen
                            && after
                                .projects
                                .iter()
                                .any(|p| p.project_id == project
                                    && p.encryption_key.as_deref() == Some("AAAA")),
                    );
                    rsx! {}
                }

                write_to_ls(&LocalStorageState::default());
                let out = std::rc::Rc::new(std::cell::Cell::new(false));
                let mut dom = VirtualDom::new_with_props(
                    Harness,
                    HarnessProps { project, out: out.clone() },
                );
                dom.rebuild_in_place();
                out.get()
            });

            assert!(survived, "the key written behind the signal must survive an unrelated write");
        }

        #[test]
        fn test_storage_file_prefers_counted_data_dir() {
            let (path, dir) = with_data_dir("dir_a", |dir| (storage_file(), dir.to_path_buf()));
            assert_eq!(path, dir.join("counted_local_storage.json"));
        }

        #[test]
        fn test_storage_file_falls_back_to_home() {
            let _guard = env_lock();
            std::env::remove_var("COUNTED_DATA_DIR");
            let tmp = std::env::temp_dir().join("counted_test_dir_b");
            std::env::set_var("HOME", tmp.to_str().unwrap());
            let path = storage_file();
            assert_eq!(path, tmp.join("counted_local_storage.json"));
        }

        #[test]
        fn test_native_write_read_roundtrip() {
            let id = Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000").unwrap();
            let loaded = with_data_dir("rw", |_| {
                let state = LocalStorageState {
                    projects: vec![LocalStorageProject {
                        project_id: id,
                        user_id: Some(7),
                        ..Default::default()
                    }],
                    onboarding_seen: true,
                    ..Default::default()
                };
                write_to_ls(&state);
                read_from_ls()
            });

            assert_eq!(loaded.projects.len(), 1);
            assert_eq!(loaded.projects[0].project_id, id);
            assert_eq!(loaded.projects[0].user_id, Some(7));
            assert!(loaded.onboarding_seen);
        }

        #[test]
        fn test_write_creates_missing_dir() {
            // Robustness: write_to_ls must create missing parent dirs (fresh install,
            // data dir not yet created). Without create_dir_all the write silently fails.
            // Points COUNTED_DATA_DIR at a nested path that does not exist yet, so it cannot
            // use with_data_dir (which creates the dir up front).
            let _guard = env_lock();
            let base = std::env::temp_dir().join("counted_missing_dir_test");
            std::fs::remove_dir_all(&base).ok();
            let nested = base.join("a").join("b");
            std::env::set_var("COUNTED_DATA_DIR", nested.to_str().unwrap());

            let state = LocalStorageState { onboarding_seen: true, ..Default::default() };
            write_to_ls(&state);
            let loaded = read_from_ls();
            let file_exists = nested.join("counted_local_storage.json").exists();

            std::env::remove_var("COUNTED_DATA_DIR");
            std::fs::remove_dir_all(&base).ok();

            assert!(file_exists, "write_to_ls should create missing parent dirs");
            assert!(loaded.onboarding_seen);
        }

        #[test]
        fn test_write_atomic_no_tmp_left() {
            // Robustness: atomic write (tmp + rename) must not leave a .json.tmp behind.
            let (tmp_left, loaded) = with_data_dir("atomic", |dir| {
                let state = LocalStorageState { onboarding_seen: true, ..Default::default() };
                write_to_ls(&state);
                (dir.join("counted_local_storage.json.tmp").exists(), read_from_ls())
            });

            assert!(!tmp_left, "temp file must not remain after atomic write");
            assert!(loaded.onboarding_seen);
        }

        #[test]
        fn test_set_cached_projects_list_roundtrip() {
            let project_id = Uuid::parse_str("550e8400-e29b-41d4-a716-446655440001").unwrap();
            let loaded = with_data_dir("cache_list", |_| {
                let mut state = LocalStorageState::default();
                set_cached_projects_list(&mut state, vec![make_project_dto(project_id)]);
                write_to_ls(&state);
                read_from_ls()
            });

            let cached = loaded.cached_projects_list.unwrap();
            assert_eq!(cached.len(), 1);
            assert_eq!(cached[0].id, project_id);
        }

        #[test]
        fn test_set_project_cache_roundtrip() {
            let project_id = Uuid::parse_str("550e8400-e29b-41d4-a716-446655440002").unwrap();
            let loaded = with_data_dir("cache_project", |_| {
                let mut state = LocalStorageState::default();
                upsert_project(&mut state, project_id, None);
                let dto = make_project_dto(project_id);
                set_project_cache(&mut state, project_id, dto, vec![], vec![], vec![], Some(0));
                write_to_ls(&state);
                read_from_ls()
            });

            let entry = loaded.projects.iter().find(|p| p.project_id == project_id).unwrap();
            assert!(entry.cached_project.is_some());
            assert_eq!(entry.cached_project.as_ref().unwrap().id, project_id);
            assert!(entry.cached_users.as_ref().unwrap().is_empty());
        }

        #[test]
        fn test_remove_project_drops_entry_and_cached_row() {
            // The bug this fixes: after deleting a project the entry survived, so re-opening the
            // share link re-registered a dangling id and the detail page hit a missing row.
            let id_a = Uuid::parse_str("550e8400-e29b-41d4-a716-446655440009").unwrap();
            let id_b = Uuid::parse_str("550e8400-e29b-41d4-a716-44665544000a").unwrap();

            let mut state = LocalStorageState::default();
            upsert_project(&mut state, id_a, None);
            upsert_project_key(&mut state, id_a, "key-a".into());
            upsert_project(&mut state, id_b, None);
            set_cached_projects_list(
                &mut state,
                vec![make_project_dto(id_a), make_project_dto(id_b)],
            );

            remove_project(&mut state, id_a);

            assert!(state.projects.iter().all(|p| p.project_id != id_a));
            assert!(state.projects.iter().any(|p| p.project_id == id_b));
            let list = state.cached_projects_list.as_ref().unwrap();
            assert_eq!(list.len(), 1);
            assert_eq!(list[0].id, id_b);
        }

        #[test]
        fn test_remove_project_is_noop_for_unknown_id() {
            let mut state = LocalStorageState::default();
            let known = Uuid::parse_str("550e8400-e29b-41d4-a716-44665544000b").unwrap();
            let unknown = Uuid::parse_str("550e8400-e29b-41d4-a716-44665544000c").unwrap();
            upsert_project(&mut state, known, None);

            remove_project(&mut state, unknown);

            assert_eq!(state.projects.len(), 1);
        }

        #[test]
        fn test_set_anon_member_id_creates_entry_and_is_scoped_per_project() {
            let a = Uuid::parse_str("550e8400-e29b-41d4-a716-44665544000d").unwrap();
            let b = Uuid::parse_str("550e8400-e29b-41d4-a716-44665544000e").unwrap();
            let member_a = Uuid::new_v4();
            let member_b = Uuid::new_v4();

            let mut state = LocalStorageState::default();
            set_anon_member_id(&mut state, a, member_a);
            set_anon_member_id(&mut state, b, member_b);

            let by = |id: Uuid| {
                state.projects.iter().find(|p| p.project_id == id).unwrap().anon_member_id
            };
            assert_eq!(by(a), Some(member_a));
            assert_eq!(by(b), Some(member_b));
            assert_ne!(by(a), by(b));
        }

        #[test]
        fn test_anon_member_id_defaults_none_for_pre_existing_entries() {
            // Entries written before the field existed must read as "never joined anonymously":
            // a fabricated id would be one the server never recorded, so leaving would delete
            // nothing and the project could never be cleaned up.
            let json = r#"{"projects":[{"project_id":"550e8400-e29b-41d4-a716-44665544000f","user_id":3,"encryption_key":"k"}],"onboarding_seen":true}"#;
            let state: LocalStorageState = serde_json::from_str(json).unwrap();
            assert!(state.projects[0].anon_member_id.is_none());
        }

        #[test]
        fn test_set_project_cache_noop_if_no_entry() {
            let project_id = Uuid::parse_str("550e8400-e29b-41d4-a716-446655440003").unwrap();
            let mut state = LocalStorageState::default();
            set_project_cache(
                &mut state,
                project_id,
                make_project_dto(project_id),
                vec![],
                vec![],
                vec![],
                Some(0),
            );
            assert!(state.projects.is_empty());
        }

        #[test]
        fn test_old_json_without_cache_fields_deserialises() {
            let json = r#"{"projects":[{"project_id":"550e8400-e29b-41d4-a716-446655440004","user_id":null}],"onboarding_seen":false}"#;
            let state: LocalStorageState = serde_json::from_str(json).unwrap();
            assert_eq!(state.projects.len(), 1);
            assert!(state.projects[0].cached_project.is_none());
            assert!(state.cached_projects_list.is_none());
            assert!(state.account_key.is_none());
        }

        #[test]
        fn test_account_key_survives_write_read_cycle() {
            let loaded = with_data_dir("account_key", |_| {
                let mut state = read_from_ls();
                state.account_key = Some("dGVzdC1rZXk".to_string());
                write_to_ls(&state);
                read_from_ls()
            });

            assert_eq!(loaded.account_key.as_deref(), Some("dGVzdC1rZXk"));
        }

        // The fix for the reactive loop: reading state via read_from_ls() instead of the ls_ctx
        // signal breaks the reactive dependency chain. These tests verify that repeated
        // read_from_ls() → mutate → write_to_ls() cycles are stable and idempotent.

        #[test]
        fn test_projects_page_cache_write_via_read_from_ls_is_idempotent() {
            // Simulates what the projects_page resource does after the fix:
            // read_from_ls() → set_cached_projects_list → write_to_ls (no ls_ctx.set).
            // Running this 3 times must not corrupt data or grow the list.
            let id = Uuid::parse_str("550e8400-e29b-41d4-a716-446655440005").unwrap();
            let loaded = with_data_dir("idempotent_list", |_| {
                let dto = make_project_dto(id);
                for _ in 0..3 {
                    let mut state = read_from_ls();
                    set_cached_projects_list(&mut state, vec![dto.clone()]);
                    write_to_ls(&state);
                }
                read_from_ls()
            });

            let list = loaded.cached_projects_list.unwrap();
            assert_eq!(list.len(), 1);
            assert_eq!(list[0].id, id);
        }

        #[test]
        fn test_project_detail_cache_write_via_read_from_ls_preserves_other_projects() {
            // Simulates the expenses_page use_effect after the fix:
            // read_from_ls() → set_project_cache → write_to_ls → ls_ctx.set.
            // Verifies that writing cache for project A does not erase project B's data.
            let id_a = Uuid::parse_str("550e8400-e29b-41d4-a716-446655440006").unwrap();
            let id_b = Uuid::parse_str("550e8400-e29b-41d4-a716-446655440007").unwrap();

            let loaded = with_data_dir("preserve_other", |_| {
                // Both projects are in localStorage (e.g. from account_projects sync)
                let mut initial = LocalStorageState::default();
                upsert_project(&mut initial, id_a, None);
                upsert_project(&mut initial, id_b, None);
                write_to_ls(&initial);

                // Project A's detail page loads: writes cache for A via read_from_ls.
                // Twice, simulating a reactive re-run — it must be idempotent.
                for _ in 0..2 {
                    let mut state = read_from_ls();
                    set_project_cache(
                        &mut state,
                        id_a,
                        make_project_dto(id_a),
                        vec![],
                        vec![],
                        vec![],
                        Some(0),
                    );
                    write_to_ls(&state);
                }
                read_from_ls()
            });

            // A has cache, B still exists and is untouched
            let a = loaded.projects.iter().find(|p| p.project_id == id_a).unwrap();
            let b = loaded.projects.iter().find(|p| p.project_id == id_b).unwrap();
            assert!(a.cached_project.is_some());
            assert!(b.cached_project.is_none());
        }

        #[test]
        fn test_cached_projects_list_visible_to_next_read_from_ls() {
            // After the fix, the projects_page resource writes cache only to disk (no signal update).
            // This test verifies that a subsequent cold-start read_from_ls() sees the cached data,
            // which is the whole point of the cache.
            let id = Uuid::parse_str("550e8400-e29b-41d4-a716-446655440008").unwrap();
            let cold_state = with_data_dir("cold_start", |_| {
                // Session 1: resource succeeds, writes cache to disk
                let mut state = read_from_ls();
                set_cached_projects_list(&mut state, vec![make_project_dto(id)]);
                write_to_ls(&state);

                // Session 2 (cold start): read_from_ls initialises ls_ctx
                read_from_ls()
            });

            assert!(cold_state.cached_projects_list.is_some());
            assert_eq!(cold_state.cached_projects_list.unwrap()[0].id, id);
        }
    }

    #[test]
    fn test_initials_two_words() {
        assert_eq!(initials("Jean Dupont"), "JD");
        assert_eq!(initials("alice bob"), "AB");
    }

    #[test]
    fn test_initials_single_word() {
        assert_eq!(initials("Jean"), "J");
        assert_eq!(initials("j"), "J");
    }

    #[test]
    fn test_initials_empty() {
        assert_eq!(initials(""), "?");
    }

    #[test]
    fn test_initials_ignores_extra_words() {
        // Only first and second word are used
        assert_eq!(initials("Jean Pierre Dupont"), "JP");
    }

    #[test]
    fn test_user_color_class_deterministic() {
        assert_eq!(user_color_class(1), user_color_class(1));
        assert_eq!(user_color_class(42), user_color_class(42));
    }

    #[test]
    fn test_user_color_class_valid_values() {
        const VALID: &[&str] = &[
            "bg-primary", "bg-secondary", "bg-accent", "bg-info",
            "bg-success", "bg-warning", "bg-error",
        ];
        for id in 0..20i32 {
            assert!(VALID.contains(&user_color_class(id)), "unexpected class for id {id}");
        }
    }

    #[test]
    fn test_user_color_class_all_reachable() {
        // 7 colors, IDs 0-6 should each hit a different one
        let classes: std::collections::HashSet<_> = (0..7).map(user_color_class).collect();
        assert_eq!(classes.len(), 7);
    }
}
