use api::expenses::expenses_controller::{add_expense, delete_expense, edit_expense};
use shared::time::DateTime;
use api::projects::projects_controller::update_project_by_id;
use dioxus::fullstack::Json;
use dioxus::prelude::*;
use shared::ExpenseWithPayments;
use std::collections::{BTreeMap, HashSet};
use uuid::Uuid;

use super::error_utils::{error_message, is_rate_limited, RATE_LIMIT_RETRY_MS, REQUEST_PACE_MS};
use super::sleep;
use super::local_storage::{key_of, previous_keys_of, read_from_ls, LocalStorageState};
use super::offline_queue::{outgoing, read_queue, Entry, OpKind, Queue, QueuedOp, Status};
use super::project_token::scope_key;
use super::project_write::update_project_fresh_legacy;
use super::queue_store::{in_one_step, land, leave, move_old, next_to_send, tidy, Moved, Snapshot, Store, OLD_KEY};

/// Why a replay could not finish, as data rather than a sentence.
///
/// `replay_queue` runs inside a spawned task, where `tid!` would have to reach the i18n context
/// through `consume_context`. Handing the variant back and translating in `AppLayout` keeps the
/// lookup on the render path, and keeps every key a literal the `i18n` scanner can check.
#[derive(Clone, PartialEq, Debug)]
pub enum SyncFailure {
    /// The queued edit/other targets something the server no longer has. Carries the label.
    ConflictEdit(String),
    ConflictOther(String),
    /// A sealed pair of the entry opens under no key this device holds for its project: sent, it
    /// would be a row nobody can read, so it stays and holds its project's entries behind it.
    /// Carries the label, empty when it does not open either.
    Held(String),
    /// Refused, or not sent: the entry is kept. Carries the server's message.
    Error(String),
    /// A marker could not be written (SPEC §15.9.5 Q2c): the entry stays and is sent again later.
    Storage,
}

/// What the server wrote for an expense op, handed to `on_landed` before the op leaves the queue,
/// so the row listed under a pending id is replaced in the same render instead of vanishing until
/// a refetch.
#[derive(Clone, PartialEq)]
pub enum Landed {
    Added(ExpenseWithPayments),
    Edited(ExpenseWithPayments),
    Deleted(i32),
}

#[derive(Clone, PartialEq, Debug)]
pub struct ReplayEnd {
    /// The keys held when the drain stopped with entries still queued, `None` once it is empty.
    pub blocked: Option<HeldKeys>,
    /// A project op landed or an op was dropped: only a refetch settles what the page shows.
    pub refetch: bool,
    /// Stopped on a failure that is not a refusal (Q4): the web sees no connectivity change to
    /// retry on, so the caller tries again after a while.
    pub transient: bool,
}

/// The entries the server refused for good (Q4) in this tab, with its message: listed as unsent
/// (Q9) and tried again by the next drain.
pub static REFUSED: GlobalSignal<BTreeMap<Uuid, String>> = Signal::global(BTreeMap::new);

thread_local! {
    static FLASHED: std::cell::RefCell<HashSet<Uuid>> = std::cell::RefCell::new(HashSet::new());
    static LANDED_HERE: std::cell::RefCell<HashSet<Uuid>> = std::cell::RefCell::new(HashSet::new());
}

/// Whether this tab's replay landed `id`: a pending row another tab landed is brought by a refetch.
pub fn landed_here(id: Uuid) -> bool {
    LANDED_HERE.with_borrow(|l| l.contains(&id))
}

/// Who may send (Q2e): on the web the tab that holds the Web Lock `counted-drain`, or where there
/// is none, a visible tab; the demo alone in its sessionStorage; the one replay of a native process.
#[cfg(target_arch = "wasm32")]
enum DrainLock {
    /// The lock, and the epoch it was taken under: a `pagehide` lets it go and moves the epoch on.
    Held(#[allow(dead_code)] super::web_dom::LockRelease, u64),
    WhileVisible,
    Alone,
}

#[cfg(target_arch = "wasm32")]
impl DrainLock {
    async fn take() -> Option<Self> {
        use super::web_dom::{acquire_lock, lock_epoch, page_gone, page_visible, release_on_page_hide};
        if super::persist::is_demo() {
            return Some(DrainLock::Alone);
        }
        if page_gone() {
            return None;
        }
        let epoch = lock_epoch();
        match acquire_lock(DRAIN_LOCK).await {
            Some(release) if epoch == lock_epoch() && !page_gone() => {
                release_on_page_hide(&release);
                Some(DrainLock::Held(release, epoch))
            }
            Some(_) => None,
            None => page_visible().then_some(DrainLock::WhileVisible),
        }
    }

    /// False once the lock was let go on `pagehide`, even after a `pageshow`: that session ends, and
    /// the next replay takes the lock again.
    fn may_send(&self) -> bool {
        use super::web_dom::{lock_epoch, page_gone, page_visible};
        match self {
            DrainLock::Held(_, epoch) => *epoch == lock_epoch() && !page_gone(),
            DrainLock::WhileVisible => !page_gone() && page_visible(),
            DrainLock::Alone => true,
        }
    }

    /// A hidden tab lets the lock go after each entry and asks for it again, so that a visible tab
    /// waiting for it, queued first by the Web Locks, takes its turn.
    fn yields(&self) -> bool {
        matches!(self, DrainLock::Held(..)) && !super::web_dom::page_visible()
    }
}

#[cfg(target_arch = "wasm32")]
const DRAIN_LOCK: &str = "counted-drain";

#[cfg(not(target_arch = "wasm32"))]
struct DrainLock;

#[cfg(not(target_arch = "wasm32"))]
impl DrainLock {
    async fn take() -> Option<Self> {
        Some(DrainLock)
    }

    fn may_send(&self) -> bool {
        true
    }

    fn yields(&self) -> bool {
        false
    }
}

/// How a session under the lock ended.
enum Session {
    /// The keys held when it stopped with entries still queued, `None` once the queue is empty.
    Done(Option<HeldKeys>),
    /// Rate limited: waited out with the lock let go.
    RateLimited(String),
    /// A hidden tab let the lock go after an entry, to take it again.
    Yielded,
    /// Stopped on a failure that is not a refusal: sent again later.
    Failed(HeldKeys),
}

/// Drains the queue one entry at a time (SPEC §15.9.5 Q2e), each read from storage: an entry held,
/// waiting or refused for good stops its project's entries behind it and the others go on (Q4); a
/// 404 drops the entry with a notice, or lands it for a delete (Q3); a failure to reach the server
/// stops the drain.
/// Q0: the old key moves at start and on a `storage` event for it, online or not and under no lock,
/// so that an entry an older build queued sorts before any change made since. Handlers and the drain
/// move it too.
pub fn move_old_key(mut pending: Signal<Queue>) {
    let moved = in_one_step(|store| {
        let snap = Snapshot::read(store);
        move_old(store, &snap)
    });
    if moved.is_some_and(|m| m != Moved::Nothing) {
        pending.set(read_queue());
    }
}

pub async fn replay_queue(
    pending: Signal<Queue>,
    mut conflict_msg: Signal<Option<SyncFailure>>,
    on_landed: Callback<Landed>,
) -> ReplayEnd {
    let mut refetch = false;
    let mut refused_now = HashSet::new();
    let mut retried = false;
    loop {
        let end = {
            let Some(lock) = DrainLock::take().await else {
                return ReplayEnd { blocked: Some(held_keys(&read_from_ls())), refetch, transient: false };
            };
            sleep(0).await;
            drain(pending, conflict_msg, on_landed, &lock, &mut refetch, &mut refused_now).await
        };
        match end {
            Session::Done(blocked) => return ReplayEnd { blocked, refetch, transient: false },
            Session::Failed(blocked) => return ReplayEnd { blocked: Some(blocked), refetch, transient: true },
            Session::Yielded => {}
            Session::RateLimited(_) if !retried => {
                retried = true;
                sleep(RATE_LIMIT_RETRY_MS).await;
            }
            Session::RateLimited(message) => {
                conflict_msg.set(Some(SyncFailure::Error(message)));
                return ReplayEnd { blocked: Some(held_keys(&read_from_ls())), refetch, transient: true };
            }
        }
    }
}

/// One session under the lock. Each step reads storage, moves the old key (Q0) and tidies as one
/// step, which on the phone is one write of the file.
async fn drain(
    mut pending: Signal<Queue>,
    mut conflict_msg: Signal<Option<SyncFailure>>,
    on_landed: Callback<Landed>,
    lock: &DrainLock,
    refetch: &mut bool,
    refused_now: &mut HashSet<Uuid>,
) -> Session {
    let mut first = true;
    loop {
        if !first {
            sleep(REQUEST_PACE_MS).await;
        }
        first = false;
        let state = read_from_ls();
        if !lock.may_send() {
            return Session::Done(Some(held_keys(&state)));
        }
        let read = in_one_step(|store| {
            let mut snap = Snapshot::read(store);
            if move_old(store, &snap) != Moved::Nothing {
                snap = Snapshot::read(store);
            }
            tidy(store, &snap);
            let mut shown = snap.queue();
            shown.extend(snap.not_moved(store.get(OLD_KEY), store.now_ms()));
            (snap, shown)
        });
        let Some((snap, shown)) = read else {
            conflict_msg.set(Some(SyncFailure::Storage));
            return Session::Done(Some(held_keys(&state)));
        };
        let queue = snap.queue();
        if *pending.peek() != shown {
            pending.set(shown.clone());
        }
        let mut held: Vec<(Uuid, String)> = Vec::new();
        let next = next_to_send(&queue, |entry| match (entry.id, prepare(entry, &state)) {
            (Some(id), _) if refused_now.contains(&id) => true,
            (Some(id), Err(SyncFailure::Held(name))) => {
                held.push((id, name));
                true
            }
            (_, result) => result.is_err(),
        });
        if let Some((_, name)) = held.into_iter().find(|(id, _)| FLASHED.with_borrow_mut(|f| f.insert(*id))) {
            conflict_msg.set(Some(SyncFailure::Held(name)));
        }
        let Some((entry, id, (sealed, key))) =
            next.and_then(|i| Some((&queue[i], queue[i].id?, prepare(&queue[i], &state).ok()?)))
        else {
            return Session::Done((!shown.is_empty()).then(|| held_keys(&state)));
        };
        let marked = |mark: &dyn Fn(&mut dyn Store) -> bool| in_one_step(|store| mark(store)) == Some(true);
        if is_legacy_overwrite(&sealed.op) {
            conflict_msg.set(Some(SyncFailure::ConflictOther(sealed.label())));
            if !marked(&|store| leave(store, &snap, entry)) {
                conflict_msg.set(Some(SyncFailure::Storage));
                return Session::Done(Some(held_keys(&state)));
            }
            *refetch = true;
            continue;
        }
        let result = replay(&sealed.op, key).await;
        match answer(&sealed, result) {
            Answer::Landed(landed) => {
                if sealed.queued_at.is_some_and(|at| DateTime::now_utc().unix_secs().saturating_sub(at) >= super::offline_queue::WAITED_SECS) {
                    crate::badges::award(crate::badges::HORS_LIGNE);
                }
                let row = match &landed {
                    Some(Landed::Added(c)) => Some(i64::from(c.expense.id)),
                    _ => None,
                };
                if !marked(&|store| land(store, &snap, entry, row)) {
                    conflict_msg.set(Some(SyncFailure::Storage));
                    return Session::Done(Some(held_keys(&state)));
                }
                REFUSED.write().remove(&id);
                LANDED_HERE.with_borrow_mut(|l| l.insert(id));
                match landed {
                    Some(landed) => on_landed.call(landed),
                    None => *refetch = true,
                }
            }
            Answer::Dropped(notice) => {
                conflict_msg.set(Some(notice));
                REFUSED.write().remove(&id);
                if !marked(&|store| leave(store, &snap, entry)) {
                    conflict_msg.set(Some(SyncFailure::Storage));
                    return Session::Done(Some(held_keys(&state)));
                }
                *refetch = true;
            }
            Answer::Retry(e) if is_rate_limited(&e) => return Session::RateLimited(error_message(&e)),
            Answer::Retry(e) => {
                conflict_msg.set(Some(SyncFailure::Error(error_message(&e))));
                return Session::Failed(held_keys(&state));
            }
            Answer::Refused(e) => {
                let message = error_message(&e);
                conflict_msg.set(Some(SyncFailure::Error(message.clone())));
                refused_now.insert(id);
                REFUSED.write().insert(id, message);
            }
        }
        let shown = read_queue();
        if *pending.peek() != shown {
            pending.set(shown);
        }
        if lock.yields() {
            return Session::Yielded;
        }
    }
}

pub type HeldKeys = Vec<(uuid::Uuid, Option<String>)>;

pub fn held_keys(state: &LocalStorageState) -> HeldKeys {
    state.projects.iter().map(|p| (p.project_id, p.encryption_key.clone())).collect()
}

/// A replay blocked under these keys tries again once any of them changes: a switch is what lets a
/// refused or held op through, and the web never sees a connectivity change to retry on.
pub fn still_blocked(blocked_at: Option<&HeldKeys>, state: &LocalStorageState) -> bool {
    blocked_at.is_some_and(|at| *at == held_keys(state))
}

/// The op as it may be sent, every pair under the project's current key, and that key, whose
/// token goes with it. Held when no key is held or a pair opens under none of them (Q5).
pub fn prepare(entry: &Entry, state: &LocalStorageState) -> Result<(QueuedOp, [u8; 32]), SyncFailure> {
    let held = state.projects.iter().find(|p| Some(p.project_id) == entry.project);
    let current = held.and_then(key_of);
    let previous = held.map(previous_keys_of).unwrap_or_default();
    current
        .and_then(|key| Some((outgoing(entry, &key, &previous)?, key)))
        .ok_or_else(|| SyncFailure::Held(entry.name_under(current.iter().chain(&previous))))
}

/// Q9, exactly: held ones, those no key held here opens, those that wait on an id with no `e`, `v`,
/// `l`, `x`, `s` or `i` key or in a cycle (`Stuck`), and those the server refused for good; each with
/// its name, what its user may discard.
pub fn unsent(queue: &[Entry], state: &LocalStorageState, refused: &BTreeMap<Uuid, String>) -> Vec<(Entry, String)> {
    queue
        .iter()
        .filter_map(|entry| {
            let held = state.projects.iter().find(|p| Some(p.project_id) == entry.project);
            let keys: Vec<[u8; 32]> = held.and_then(key_of).into_iter().chain(held.map(previous_keys_of).unwrap_or_default()).collect();
            let listed = matches!(entry.status, Status::Held | Status::Stuck)
                || entry.id.is_some_and(|id| refused.contains_key(&id))
                || prepare(entry, state).is_err();
            listed.then(|| (entry.clone(), entry.name_under(keys.iter())))
        })
        .collect()
}

async fn replay(op: &OpKind, key: [u8; 32]) -> Result<Option<Landed>, ServerFnError> {
    match op.clone() {
        OpKind::AddExpense(p) => {
            scope_key(Some(&key));
            add_expense(Json(p)).await.map(|c| Some(Landed::Added(c)))
        }
        OpKind::EditExpense(p) => {
            scope_key(Some(&key));
            edit_expense(Json(p)).await.map(|c| Some(Landed::Edited(c)))
        }
        OpKind::DeleteExpense(p) => {
            scope_key(Some(&key));
            let id = p.id;
            delete_expense(Json(p)).await.map(|_| Some(Landed::Deleted(id)))
        }
        OpKind::UpdateProject(p) => {
            scope_key(Some(&key));
            update_project_by_id(Json(p)).await.map(|_| None)
        }
        // Rebuilt on the payload the server holds now, so a field written meanwhile - by
        // another device or a newer build - survives the replay.
        OpKind::EditProject(edit) => {
            update_project_fresh_legacy(edit.project_id, key, |stored| edit.apply(&key, stored)).await.map(|_| None)
        }
    }
}

/// A whole payload queued by a build before 3.2.5, the last to write `UpdateProject`. It carries no
/// `expected_iv`, which the server refuses with a payload, and was built on a payload long replaced.
fn is_legacy_overwrite(op: &OpKind) -> bool {
    matches!(op, OpKind::UpdateProject(p) if p.payload.is_some())
}

enum Answer {
    Landed(Option<Landed>),
    /// A 404 for any type but a delete: dropped with a notice, as builds before 3.4.1 do.
    Dropped(SyncFailure),
    /// Not refused: kept, and the drain stops.
    Retry(ServerFnError),
    /// Refused for good (Q4): kept, listed, and its project stops.
    Refused(ServerFnError),
}

/// Q3, Q4 for the types up to 3.4.0. Refused for good is any refusal but a 404, a failure to reach
/// the server, 5xx, 429 and 409 `PROJECT_ROTATING`; 426 is kept out of it too, since the update it
/// asks for sends the entry.
fn answer(queued: &QueuedOp, result: Result<Option<Landed>, ServerFnError>) -> Answer {
    let err = match result {
        Ok(landed) => return Answer::Landed(landed),
        Err(err) => err,
    };
    match &err {
        _ if is_conflict(&err) => match &queued.op {
            OpKind::DeleteExpense(delete) => Answer::Landed(Some(Landed::Deleted(delete.id))),
            OpKind::EditExpense(_) => Answer::Dropped(SyncFailure::ConflictEdit(queued.label())),
            _ => Answer::Dropped(SyncFailure::ConflictOther(queued.label())),
        },
        ServerFnError::ServerError { code, message, .. }
            if *code >= 500 || *code == 429 || *code == 426 || (*code == 409 && message == shared::errors::PROJECT_ROTATING) =>
        {
            Answer::Retry(err)
        }
        ServerFnError::ServerError { .. } => Answer::Refused(err),
        _ => Answer::Retry(err),
    }
}

/// Matched on the variant, never the text: `error_message` returns translated copy, which no
/// substring check can recognise in every locale.
fn is_conflict(err: &ServerFnError) -> bool {
    matches!(
        err,
        ServerFnError::ServerError { message, code: 404, .. }
            if message == shared::errors::EXPENSE_NOT_FOUND || message == shared::PROJECT_NOT_FOUND
    )
}

#[cfg(test)]
mod tests {
    use super::{is_conflict, is_legacy_overwrite, OpKind};
    use dioxus::prelude::ServerFnError;
    use shared::{EditableProject, EncryptedPair, ProjectStatus};
    use uuid::Uuid;

    fn update_project(payload: Option<EncryptedPair>) -> OpKind {
        OpKind::UpdateProject(EditableProject {
            id: Uuid::nil(),
            payload,
            status: Some(ProjectStatus::Closed),
            history: None,
            expected_iv: None,
            notify_closed: false,
            expected_rev: None,
        })
    }

    #[test]
    fn a_legacy_whole_payload_is_dropped() {
        assert!(is_legacy_overwrite(&update_project(Some(EncryptedPair { ct: "ct".into(), iv: "iv".into() }))));
    }

    #[test]
    fn a_legacy_status_alone_still_replays() {
        assert!(!is_legacy_overwrite(&update_project(None)));
    }

    fn server_error(message: &str, code: u16) -> ServerFnError {
        ServerFnError::ServerError { message: message.into(), code, details: None }
    }

    #[test]
    fn test_conflict_on_expense_not_found() {
        assert!(is_conflict(&server_error(shared::errors::EXPENSE_NOT_FOUND, 404)));
    }

    #[test]
    fn test_conflict_on_project_not_found() {
        assert!(is_conflict(&server_error(shared::PROJECT_NOT_FOUND, 404)));
    }

    #[test]
    fn test_no_conflict_on_not_found_text_without_404() {
        assert!(!is_conflict(&server_error(shared::errors::EXPENSE_NOT_FOUND, 500)));
    }

    #[test]
    fn test_no_conflict_on_unrelated_404() {
        assert!(!is_conflict(&server_error(shared::errors::USER_NOT_FOUND, 404)));
    }

    #[test]
    fn test_no_conflict_on_other_server_error() {
        assert!(!is_conflict(&ServerFnError::new("boom")));
    }

    #[test]
    fn test_no_conflict_on_stream_error() {
        assert!(!is_conflict(&ServerFnError::StreamError("reset".into())));
    }
}

#[cfg(test)]
#[path = "queue_replay_tests.rs"]
mod prepare_tests;
