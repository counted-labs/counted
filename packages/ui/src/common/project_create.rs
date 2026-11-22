use api::auth::auth_controller::me;
use api::projects::projects_controller::{add_project, get_project};
use dioxus::fullstack::Json;
use dioxus::prelude::*;
use shared::{Account, CreatableProject, EditableProject, EncryptedPair, ProjectDto};
use std::cell::RefCell;
use std::collections::BTreeSet;
use std::sync::Mutex;
use uuid::Uuid;

use super::creator_proof::keep_creator_secret;
use super::error_utils::{error_message, is_id_taken_error, is_offline_error, is_project_gone_error};
use super::local_storage::{key_of, read_from_ls, update_ls, upsert_project, upsert_project_key, write_to_ls, LocalStorageState, PendingCreation};
use crate::crypto::{decrypt_bytes, key_from_fragment, key_to_fragment};
use crate::decrypted::decrypt_project;

pub enum Unsent {
    Sealing(String),
    Refused(ServerFnError),
    /// The creation may have landed: its pending record stays until a read of `project_id` says.
    Unknown { project_id: Uuid, error: Option<ServerFnError> },
    /// Nothing was sent: the pending record could not be written.
    Unsaved,
    /// Nothing was sent: settling the pending records adopted this project, created earlier.
    Recovered(String),
    /// Nothing was sent: an earlier creation this account could adopt may still commit.
    Confirming,
}

impl Unsent {
    pub fn message(self) -> String {
        match self {
            Unsent::Sealing(e) => e,
            Unsent::Refused(e) | Unsent::Unknown { error: Some(e), .. } => error_message(&e),
            Unsent::Unknown { error: None, .. } => crate::tid!("error-network"),
            Unsent::Unsaved => crate::tid!("error-generic"),
            Unsent::Recovered(name) => crate::tid!("add-project-recovered", name: name),
            Unsent::Confirming => crate::tid!("project-still-confirming"),
        }
    }
}

pub struct Created {
    pub project: ProjectDto,
    pub key: [u8; 32],
}

const READS: usize = 3;
const READ_PAUSE_MS: u32 = 1_000;

/// How long after its last send a creation can still commit. Once the client has given up, nothing
/// more reaches nginx: it reads a body for at most 15 s (`client_body_timeout`) and connects
/// upstream within 10 s (`proxy_connect_timeout`); the backend takes a connection within 5 s and
/// runs each of the creation's ten or so statements within 5 s (`db.rs`). About 80 s, rounded up.
const SETTLED_AFTER_MS: i64 = 120_000;

/// A send of this record may still commit, so a 404 for it says nothing yet.
pub fn still_confirming(record: &PendingCreation, now_ms: i64) -> bool {
    now_ms - record.sent_at < SETTLED_AFTER_MS
}

pub fn now_ms() -> i64 {
    shared::time::DateTime::now_utc().unix_millis()
}

/// The ids this process is sending or settling right now: no other settlement here touches them.
static BUSY: Mutex<BTreeSet<Uuid>> = Mutex::new(BTreeSet::new());

struct Busy(Uuid);

impl Busy {
    fn take(id: Uuid) -> Option<Busy> {
        BUSY.lock().unwrap_or_else(|e| e.into_inner()).insert(id).then(|| Busy(id))
    }
}

impl Drop for Busy {
    fn drop(&mut self) {
        BUSY.lock().unwrap_or_else(|e| e.into_inner()).remove(&self.0);
    }
}

/// Creates a project under an id drawn here, before `seal` seals anything for it (SPEC 15.9.1),
/// with a pending record kept from before the first send until the outcome is known. `unsent` holds
/// the id an earlier press of the same form left unknown: when it landed, this press carries on with
/// it under its own key, its payload resealed from what `seal` builds now.
pub async fn add_project_fresh(
    ls_ctx: Signal<LocalStorageState>,
    unsent: Option<Signal<Option<Uuid>>>,
    key: &[u8; 32],
    creator_secret: Option<String>,
    seal: impl Fn(Uuid, &[u8; 32]) -> Result<CreatableProject, String>,
) -> Result<Created, Unsent> {
    let world = Live { ls_ctx, account: RefCell::new(None) };
    let before = unsent.and_then(|u| *u.peek());
    let result = create(&world, before, key, creator_secret, seal).await;
    if let Some(mut unsent) = unsent {
        unsent.set(left_unsent(before, &result));
    }
    result
}

/// Settles every pending creation a read can (`settle_by`). A creation is never sent again from here.
pub async fn resolve_pending(ls_ctx: Signal<LocalStorageState>) {
    resolve(&Live { ls_ctx, account: RefCell::new(None) }).await;
}

fn left_unsent(before: Option<Uuid>, result: &Result<Created, Unsent>) -> Option<Uuid> {
    match result {
        Ok(_) => None,
        Err(Unsent::Unknown { project_id, .. }) => Some(*project_id),
        Err(_) => before,
    }
}

trait World {
    fn store(&self) -> LocalStorageState;
    fn write(&self, f: impl FnOnce(&mut LocalStorageState));
    /// True once the record is in storage, not only in memory.
    fn keep(&self, record: PendingCreation) -> bool;
    fn now_ms(&self) -> i64;
    /// The account the session is, from a `me()` that answered: never guessed from the store.
    async fn signed_in(&self) -> Result<Option<Uuid>, ServerFnError>;
    async fn send(&self, creatable: &CreatableProject) -> Result<ProjectDto, ServerFnError>;
    async fn read(&self, id: Uuid, key: &[u8; 32]) -> Result<ProjectDto, ServerFnError>;
    async fn reseal(&self, id: Uuid, key: [u8; 32], payload: EncryptedPair) -> Result<ProjectDto, ServerFnError>;
    /// The membership the creation's later steps would have written, under `signed_in`.
    async fn join(&self, id: Uuid);
    async fn pause(&self);
}

async fn create<W: World>(
    w: &W,
    unsent: Option<Uuid>,
    key: &[u8; 32],
    creator_secret: Option<String>,
    seal: impl Fn(Uuid, &[u8; 32]) -> Result<CreatableProject, String>,
) -> Result<Created, Unsent> {
    let adopted = resolve(w).await;
    if let Some(id) = unsent {
        let state = w.store();
        if let Some(held) = state.projects.iter().find(|p| p.project_id == id).and_then(key_of) {
            let payload = seal(id, &held).map_err(Unsent::Sealing)?.payload;
            let project = w.reseal(id, held, payload).await.map_err(Unsent::Refused)?;
            return Ok(Created { project, key: held });
        }
        if state.pending_creations.iter().any(|p| p.project_id == id) {
            return Err(Unsent::Unknown { project_id: id, error: None });
        }
    }
    if let Some((_, name)) = adopted.into_iter().find(|(id, _)| Some(*id) != unsent) {
        return Err(Unsent::Recovered(name));
    }

    let id = Uuid::new_v4();
    let creatable = seal(id, key).map_err(Unsent::Sealing)?;
    let kept = !creatable.demo;
    let _busy = Busy::take(id);
    if kept {
        let account = w.signed_in().await.map_err(Unsent::Refused)?;
        let now = w.now_ms();
        if w.store().pending_creations.iter().any(|p| (p.account.is_none() || p.account == account) && still_confirming(p, now)) {
            return Err(Unsent::Confirming);
        }
        let record = PendingCreation { project_id: id, key: key_to_fragment(key), creator_secret: creator_secret.clone(), account, sent_at: w.now_ms() };
        if !w.keep(record) {
            return Err(Unsent::Unsaved);
        }
    }
    match delivered(w, &creatable, key, kept).await {
        Outcome::Landed(project) => {
            if kept {
                w.write(|state| settle(state, id, Some((key, creator_secret))));
            }
            Ok(Created { project, key: *key })
        }
        Outcome::Refused(e) => {
            w.write(|state| settle(state, id, None));
            Err(Unsent::Refused(e))
        }
        Outcome::Unknown(e) => Err(Unsent::Unknown { project_id: id, error: Some(e) }),
    }
}

/// The projects adopted, with their names.
async fn resolve<W: World>(w: &W) -> Vec<(Uuid, String)> {
    let mut adopted = vec![];
    let mut signed_in = None;
    for record in w.store().pending_creations {
        let id = record.project_id;
        let Some(_busy) = Busy::take(id) else { continue };
        if !w.store().pending_creations.iter().any(|p| p.project_id == id) {
            continue;
        }
        let Ok(key) = key_from_fragment(&record.key) else {
            w.write(|state| settle(state, id, None));
            continue;
        };
        let read_at = w.now_ms();
        if read_at < record.sent_at {
            w.write(|state| stamp(state, id, read_at));
        }
        let read = read_back(w, id, &key, 1).await;
        if matches!(read, Read::Ours(_)) && signed_in.is_none() {
            signed_in = Some(w.signed_in().await.ok());
        }
        match settle_by(&read, record.account, signed_in.flatten(), (read_at - record.sent_at).max(0)) {
            Settle::Adopt => {
                w.write(|state| settle(state, id, Some((&key, record.creator_secret.clone()))));
                w.join(id).await;
                let name = match &read {
                    Read::Ours(row) => decrypt_project(&key, row).map(|p| p.name).unwrap_or_default(),
                    _ => String::new(),
                };
                adopted.push((id, name));
            }
            Settle::Drop => w.write(|state| settle(state, id, None)),
            Settle::Wait => {}
        }
    }
    adopted
}

#[derive(Debug, PartialEq)]
enum Settle {
    Adopt,
    Drop,
    Wait,
}

/// A project that opens is adopted once a `me()` has answered: under the account it was created by,
/// or, made without one, under whoever is signed in now, as `account_sync` treats an anonymous
/// project. An absence is believed only once no send can still commit.
fn settle_by<T>(read: &Read<T>, creator: Option<Uuid>, signed_in: Option<Option<Uuid>>, age_ms: i64) -> Settle {
    match read {
        Read::Ours(_) if signed_in.is_some_and(|now| creator.is_none() || now == creator) => Settle::Adopt,
        Read::Gone | Read::NotOurs if age_ms >= SETTLED_AFTER_MS => Settle::Drop,
        _ => Settle::Wait,
    }
}

fn settle(state: &mut LocalStorageState, id: Uuid, landed: Option<(&[u8; 32], Option<String>)>) {
    state.pending_creations.retain(|p| p.project_id != id);
    if let Some((key, creator_secret)) = landed {
        upsert_project(state, id, None);
        upsert_project_key(state, id, key_to_fragment(key));
        keep_creator_secret(state, id, creator_secret);
    }
}

fn stamp(state: &mut LocalStorageState, id: Uuid, now_ms: i64) {
    if let Some(record) = state.pending_creations.iter_mut().find(|p| p.project_id == id) {
        record.sent_at = now_ms;
    }
}

fn opens(landed: &ProjectDto, id: Uuid, key: &[u8; 32]) -> bool {
    landed.id == id && decrypt_bytes(key, &landed.payload).is_ok()
}

fn lost_in_transit(e: &ServerFnError) -> bool {
    is_offline_error(e) || matches!(e, ServerFnError::ServerError { code: 502..=504, .. })
}

/// The server, or nginx before it, turned the first send down. After a resend no 4xx says this:
/// the first delivery may have landed before a deploy answered the resend with a 426 or a 400.
fn refused_first(e: &ServerFnError) -> bool {
    matches!(e, ServerFnError::ServerError { code, .. } if (400..500).contains(code) && *code != 408)
}

fn not_ours(e: &ServerFnError) -> bool {
    matches!(e, ServerFnError::ServerError { code: 403, message, .. } if message == shared::errors::PROJECT_KEY_REQUIRED)
}

enum Outcome {
    Landed(ProjectDto),
    Refused(ServerFnError),
    Unknown(ServerFnError),
}

enum Read<T> {
    Ours(T),
    Gone,
    NotOurs,
    Unknown,
}

async fn delivered<W: World>(w: &W, creatable: &CreatableProject, key: &[u8; 32], kept: bool) -> Outcome {
    let id = creatable.id.expect("drawn before sealing");
    let (answer, resent) = match w.send(creatable).await {
        Err(e) if lost_in_transit(&e) => {
            if kept {
                w.write(|state| stamp(state, id, w.now_ms()));
            }
            (w.send(creatable).await, true)
        }
        first => (first, false),
    };
    match answer {
        Ok(landed) => Outcome::Landed(landed),
        Err(e) if is_id_taken_error(&e) => match read_back(w, id, key, READS).await {
            Read::Ours(landed) => Outcome::Landed(landed),
            Read::Gone | Read::NotOurs => Outcome::Refused(e),
            Read::Unknown => Outcome::Unknown(e),
        },
        Err(e) if !resent && refused_first(&e) => Outcome::Refused(e),
        Err(e) => Outcome::Unknown(e),
    }
}

async fn read_back<W: World>(w: &W, id: Uuid, key: &[u8; 32], reads: usize) -> Read<ProjectDto> {
    for attempt in 0..reads {
        if attempt > 0 {
            w.pause().await;
        }
        match w.read(id, key).await {
            Ok(row) if opens(&row, id, key) => return Read::Ours(row),
            Ok(_) => return Read::NotOurs,
            Err(e) if is_project_gone_error(&e) => return Read::Gone,
            Err(e) if not_ours(&e) => return Read::NotOurs,
            Err(_) => {}
        }
    }
    Read::Unknown
}

struct Live {
    ls_ctx: Signal<LocalStorageState>,
    account: RefCell<Option<Option<Account>>>,
}

impl World for Live {
    fn store(&self) -> LocalStorageState {
        read_from_ls()
    }

    fn write(&self, f: impl FnOnce(&mut LocalStorageState)) {
        update_ls(self.ls_ctx, f);
    }

    fn keep(&self, record: PendingCreation) -> bool {
        let mut state = read_from_ls();
        state.pending_creations.push(record);
        let kept = write_to_ls(&state);
        if kept {
            let mut ls_ctx = self.ls_ctx;
            ls_ctx.set(state);
        }
        kept
    }

    fn now_ms(&self) -> i64 {
        now_ms()
    }

    async fn signed_in(&self) -> Result<Option<Uuid>, ServerFnError> {
        let known = self.account.borrow().clone();
        let account = match known {
            Some(account) => account,
            None if super::persist::is_demo() => None,
            None if try_consume_context::<Signal<super::MeAnswered>>().is_some_and(|m| m.peek().0) => {
                try_consume_context::<Signal<Option<Account>>>().and_then(|a| a.peek().clone())
            }
            None => me().await?,
        };
        *self.account.borrow_mut() = Some(account.clone());
        Ok(account.map(|a| a.id))
    }

    async fn send(&self, creatable: &CreatableProject) -> Result<ProjectDto, ServerFnError> {
        add_project(Json(creatable.clone())).await
    }

    async fn read(&self, id: Uuid, key: &[u8; 32]) -> Result<ProjectDto, ServerFnError> {
        super::project_token::scope_key(Some(key));
        get_project(id).await
    }

    async fn reseal(&self, id: Uuid, key: [u8; 32], payload: EncryptedPair) -> Result<ProjectDto, ServerFnError> {
        let edit = |_: &ProjectDto| {
            Ok(EditableProject { id, payload: Some(payload.clone()), status: None, history: None, expected_iv: None, notify_closed: false })
        };
        super::project_write::update_project_fresh_under(id, key, edit).await
    }

    async fn join(&self, id: Uuid) {
        let account = self.account.borrow().clone().flatten();
        let owner = read_from_ls().account_key_owner;
        let account_key = account
            .as_ref()
            .filter(|a| owner.is_none_or(|o| o == a.id))
            .and_then(|_| try_consume_context::<Signal<Option<[u8; 32]>>>().and_then(|k| *k.peek()));
        super::ensure_membership(self.ls_ctx, id, account, account_key).await;
        // The account's pull may have listed the project keyless before this: the list re-syncs.
        if let Some(mut resource_version) = try_consume_context::<Signal<u64>>() {
            resource_version += 1;
        }
    }

    async fn pause(&self) {
        super::sleep::sleep(READ_PAUSE_MS).await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::test_fixtures::make_project;
    use crate::crypto::encrypt_padded_json;
    use dioxus::fullstack::RequestError;
    use shared::ProjectPayload;
    use std::cell::Cell;
    use std::collections::{HashMap, VecDeque};
    use std::future::Future;
    use std::pin::pin;
    use std::task::{Context, Poll, Waker};

    fn now<T>(f: impl Future<Output = T>) -> T {
        match pin!(f).poll(&mut Context::from_waker(Waker::noop())) {
            Poll::Ready(v) => v,
            Poll::Pending => panic!("the mock answers at once"),
        }
    }

    fn server(message: &str, code: u16) -> ServerFnError {
        ServerFnError::ServerError { message: message.into(), code, details: None }
    }

    fn taken() -> ServerFnError {
        server(shared::errors::ID_TAKEN, 409)
    }

    fn timeout() -> ServerFnError {
        ServerFnError::Request(RequestError::Timeout("after 30s".into()))
    }

    fn rate_limited() -> ServerFnError {
        server("HTTP 429: <html><head><title>429 Too Many Requests</title></head></html>", 429)
    }

    fn gone() -> ServerFnError {
        server(shared::PROJECT_NOT_FOUND, 404)
    }

    fn key_required() -> ServerFnError {
        server(shared::errors::PROJECT_KEY_REQUIRED, 403)
    }

    /// What a send meets: `Pass` lands and answers, `Lose` lands and the answer is lost, `Fail`
    /// never reaches the server and comes back as the error, `Foreign` finds another key's project.
    #[derive(Clone)]
    enum Fate {
        Pass,
        Lose,
        Fail(ServerFnError),
        Foreign,
    }

    const ALICE: Uuid = Uuid::from_u128(0xa11ce);
    const BOB: Uuid = Uuid::from_u128(0xb0b);

    /// A server holding real rows, and a store that can refuse writes.
    struct Mock {
        state: RefCell<LocalStorageState>,
        stale: RefCell<Option<LocalStorageState>>,
        storable: Cell<bool>,
        clock: Cell<i64>,
        signed_in: RefCell<Result<Option<Uuid>, ServerFnError>>,
        rows: RefCell<HashMap<Uuid, ProjectDto>>,
        fates: RefCell<VecDeque<Fate>>,
        reads_fail: Cell<usize>,
        reseal_fails: Cell<bool>,
        log: RefCell<Vec<String>>,
    }

    impl Mock {
        fn new(signed_in: Option<Uuid>) -> Mock {
            Mock {
                state: RefCell::new(LocalStorageState::default()),
                stale: RefCell::new(None),
                storable: Cell::new(true),
                clock: Cell::new(1_000_000),
                signed_in: RefCell::new(Ok(signed_in)),
                rows: RefCell::new(HashMap::new()),
                fates: RefCell::new(VecDeque::new()),
                reads_fail: Cell::new(0),
                reseal_fails: Cell::new(false),
                log: RefCell::new(vec![]),
            }
        }

        fn fates(&self, fates: &[Fate]) {
            self.fates.borrow_mut().extend(fates.iter().cloned());
        }

        fn later(&self, ms: i64) {
            self.clock.set(self.clock.get() + ms);
        }

        fn count(&self, what: &str) -> usize {
            self.log.borrow().iter().filter(|l| l.starts_with(what)).count()
        }

        fn pending(&self) -> Vec<Uuid> {
            self.state.borrow().pending_creations.iter().map(|p| p.project_id).collect()
        }

        fn held(&self, id: Uuid) -> Option<[u8; 32]> {
            self.state.borrow().projects.iter().find(|p| p.project_id == id).and_then(key_of)
        }

        fn name_of(&self, id: Uuid, key: &[u8; 32]) -> String {
            decrypt_project(key, &self.rows.borrow()[&id]).unwrap().name
        }
    }

    impl World for Mock {
        fn store(&self) -> LocalStorageState {
            self.stale.take().unwrap_or_else(|| self.state.borrow().clone())
        }

        fn write(&self, f: impl FnOnce(&mut LocalStorageState)) {
            f(&mut self.state.borrow_mut());
        }

        fn keep(&self, record: PendingCreation) -> bool {
            if self.storable.get() {
                self.state.borrow_mut().pending_creations.push(record);
            }
            self.storable.get()
        }

        fn now_ms(&self) -> i64 {
            self.clock.get()
        }

        async fn signed_in(&self) -> Result<Option<Uuid>, ServerFnError> {
            self.signed_in.borrow().clone()
        }

        async fn send(&self, creatable: &CreatableProject) -> Result<ProjectDto, ServerFnError> {
            let id = creatable.id.unwrap();
            self.log.borrow_mut().push(format!("send {id}"));
            let fate = self.fates.borrow_mut().pop_front().unwrap_or(Fate::Pass);
            self.later(1_000);
            match fate {
                Fate::Fail(e) => return Err(e),
                Fate::Foreign => {
                    self.rows.borrow_mut().insert(id, make_project(&[9; 32], id, "x", "EUR"));
                    return Err(taken());
                }
                _ => {}
            }
            if self.rows.borrow().contains_key(&id) {
                return Err(match fate {
                    Fate::Lose => timeout(),
                    _ => taken(),
                });
            }
            let row = ProjectDto { payload: creatable.payload.clone(), ..make_project(&[0; 32], id, "", "EUR") };
            self.rows.borrow_mut().insert(id, row.clone());
            match fate {
                Fate::Lose => Err(timeout()),
                _ => Ok(row),
            }
        }

        async fn read(&self, id: Uuid, key: &[u8; 32]) -> Result<ProjectDto, ServerFnError> {
            self.log.borrow_mut().push(format!("read {id}"));
            if self.reads_fail.get() > 0 {
                self.reads_fail.set(self.reads_fail.get() - 1);
                return Err(timeout());
            }
            match self.rows.borrow().get(&id) {
                None => Err(gone()),
                Some(row) if decrypt_bytes(key, &row.payload).is_err() => Err(key_required()),
                Some(row) => Ok(row.clone()),
            }
        }

        async fn reseal(&self, id: Uuid, _: [u8; 32], payload: EncryptedPair) -> Result<ProjectDto, ServerFnError> {
            self.log.borrow_mut().push(format!("reseal {id}"));
            if self.reseal_fails.get() {
                return Err(server("x", 500));
            }
            let mut rows = self.rows.borrow_mut();
            let row = rows.get_mut(&id).ok_or_else(gone)?;
            row.payload = payload;
            Ok(row.clone())
        }

        async fn join(&self, id: Uuid) {
            let as_who = self.signed_in.borrow().clone().ok().flatten().map_or("anonymous".to_string(), |a| a.to_string());
            self.log.borrow_mut().push(format!("join {id} {as_who}"));
        }

        async fn pause(&self) {
            self.log.borrow_mut().push("pause".into());
        }
    }

    fn sealer(name: &'static str, demo: bool) -> impl Fn(Uuid, &[u8; 32]) -> Result<CreatableProject, String> {
        move |id, key| {
            Ok(CreatableProject {
                id: Some(id),
                payload: encrypt_padded_json(key, &ProjectPayload { name: name.into(), currency: "EUR".into(), description: None, status: None, extra: Default::default() })?,
                claim_verifier: None,
                demo,
                creator_verifier: None,
            })
        }
    }

    const KEY: [u8; 32] = [7; 32];

    /// One press of a form named `name`, `unsent` carried from the press before as the form does.
    fn press(w: &Mock, unsent: &mut Option<Uuid>, name: &'static str) -> Result<Created, Unsent> {
        let result = now(create(w, *unsent, &KEY, Some("secret".into()), sealer(name, false)));
        *unsent = left_unsent(*unsent, &result);
        result
    }

    fn unknown_id(result: Result<Created, Unsent>) -> Uuid {
        match result {
            Err(Unsent::Unknown { project_id, .. }) => project_id,
            _ => panic!("expected an unknown outcome"),
        }
    }

    fn is(result: &Result<Created, Unsent>) -> &'static str {
        match result {
            Ok(_) => "landed",
            Err(Unsent::Refused(_)) => "refused",
            Err(Unsent::Unknown { .. }) => "unknown",
            Err(Unsent::Unsaved) => "unsaved",
            Err(Unsent::Recovered(_)) => "recovered",
            Err(Unsent::Confirming) => "confirming",
            Err(Unsent::Sealing(_)) => "sealing",
        }
    }

    /// A first press that ends unknown with its project landed (both answers lost).
    fn landed_unknown(w: &Mock, unsent: &mut Option<Uuid>, name: &'static str) -> Uuid {
        w.fates(&[Fate::Lose, Fate::Lose]);
        unknown_id(press(w, unsent, name))
    }

    #[test]
    fn a_creation_that_lands_is_sent_once_nothing_is_read_and_its_record_settles() {
        let w = Mock::new(None);
        let made = press(&w, &mut None, "Ski").ok().expect("landed");
        assert_eq!((w.count("send"), w.count("read")), (1, 0));
        assert!(w.pending().is_empty());
        assert_eq!(w.held(made.project.id), Some(KEY));
        let entry = w.store().projects.into_iter().find(|p| p.project_id == made.project.id).unwrap();
        assert_eq!(entry.creator_secret.as_deref(), Some("secret"));
    }

    #[test]
    fn a_demo_keeps_no_record_and_asks_no_account() {
        let w = Mock::new(None);
        *w.signed_in.borrow_mut() = Err(timeout());
        w.fates(&[Fate::Lose, Fate::Lose]);
        let result = now(create(&w, None, &KEY, None, sealer("Demo", true)));
        assert_eq!(is(&result), "unknown");
        assert!(w.pending().is_empty());
        assert!(w.store().projects.is_empty());

        let made = now(create(&w, None, &KEY, None, sealer("Demo", true)));
        assert_eq!(is(&made), "landed");
        assert!(w.store().projects.is_empty(), "the demo stores its project itself, once seeded");
    }

    #[test]
    fn a_duplicate_delivery_is_adopted_when_the_landed_row_opens_under_the_key() {
        let w = Mock::new(None);
        w.fates(&[Fate::Lose, Fate::Pass]);
        let made = press(&w, &mut None, "Ski").ok().expect("adopted");
        assert_eq!((w.count("send"), w.count("read")), (2, 1));
        assert_eq!(w.held(made.project.id), Some(KEY));
        assert!(w.pending().is_empty());
    }

    #[test]
    fn a_taken_id_whose_row_does_not_open_or_is_gone_is_refused_and_its_record_dropped() {
        for taken_by in [Fate::Foreign, Fate::Fail(taken())] {
            let w = Mock::new(None);
            w.fates(&[Fate::Fail(timeout()), taken_by]);
            assert_eq!(is(&press(&w, &mut None, "Ski")), "refused");
            assert_eq!((w.count("read"), w.pending()), (1, vec![]));
        }
    }

    /// Case C: the read after a taken id is tried a few times before the outcome is unknown.
    #[test]
    fn a_taken_id_whose_read_keeps_failing_is_unknown_after_a_few_tries() {
        let w = Mock::new(None);
        w.fates(&[Fate::Lose, Fate::Pass]);
        w.reads_fail.set(READS);
        let id = unknown_id(press(&w, &mut None, "Ski"));
        assert_eq!((w.count("read"), w.count("pause")), (READS, READS - 1));
        assert_eq!(w.pending(), vec![id]);
    }

    #[test]
    fn a_lost_answer_is_sent_once_more_and_its_answer_kept() {
        for lost in [timeout(), server("Bad Gateway", 502), server("Gateway Timeout", 504)] {
            let w = Mock::new(None);
            w.fates(&[Fate::Fail(lost), Fate::Pass]);
            assert_eq!(is(&press(&w, &mut None, "Ski")), "landed");
            assert_eq!(w.count("send"), 2);
        }
    }

    /// Point 2 of the third review: after a lost answer only a 200, or a taken id and a read, says
    /// anything about the first delivery. A deploy can answer the resend with a 426 or a 400.
    #[test]
    fn no_answer_to_the_resend_but_a_200_or_a_taken_id_says_nothing_landed() {
        for resend in [rate_limited(), server("x", 500), server("Service Unavailable", 503), server("outdated", 426), server(shared::errors::INVALID_PAYLOAD, 400), server("x", 413)] {
            let w = Mock::new(None);
            w.fates(&[Fate::Lose, Fate::Fail(resend)]);
            let id = unknown_id(press(&w, &mut None, "Ski"));
            assert_eq!((w.count("send"), w.pending()), (2, vec![id]));
        }
    }

    /// Case B.
    #[test]
    fn two_lost_answers_are_unknown_without_a_third_send() {
        let w = Mock::new(None);
        let id = landed_unknown(&w, &mut None, "Ski");
        assert_eq!((w.count("send"), w.pending()), (2, vec![id]));
    }

    #[test]
    fn a_first_send_refused_before_the_server_or_by_it_is_final() {
        for refused in [rate_limited(), server(shared::errors::INVALID_PAYLOAD, 400), server(shared::errors::IDENTITY_TAKEN, 409)] {
            let w = Mock::new(None);
            w.fates(&[Fate::Fail(refused)]);
            assert_eq!(is(&press(&w, &mut None, "Ski")), "refused");
            assert_eq!((w.count("send"), w.count("read")), (1, 0));
            assert!(w.pending().is_empty());
        }
    }

    #[test]
    fn a_server_error_on_the_first_send_is_unknown() {
        let w = Mock::new(None);
        w.fates(&[Fate::Fail(server(shared::errors::ID_TAKEN, 500))]);
        assert_eq!(is(&press(&w, &mut None, "Ski")), "unknown");
        assert_eq!(w.count("send"), 1);
    }

    #[test]
    fn a_pending_record_is_adopted_under_its_account_dropped_once_settled_and_kept_otherwise() {
        let ours: Read<()> = Read::Ours(());
        let cases = [
            (&ours, None, Some(None), 0, Settle::Adopt),
            (&ours, Some(ALICE), Some(Some(ALICE)), 0, Settle::Adopt),
            (&ours, Some(ALICE), Some(None), 0, Settle::Wait),
            (&ours, Some(ALICE), Some(Some(BOB)), 0, Settle::Wait),
            (&ours, None, Some(Some(ALICE)), 0, Settle::Adopt),
            (&ours, None, None, SETTLED_AFTER_MS, Settle::Wait),
            (&Read::Gone, None, Some(None), SETTLED_AFTER_MS - 1, Settle::Wait),
            (&Read::Gone, None, Some(None), SETTLED_AFTER_MS, Settle::Drop),
            (&Read::NotOurs, Some(ALICE), None, SETTLED_AFTER_MS - 1, Settle::Wait),
            (&Read::NotOurs, Some(ALICE), None, SETTLED_AFTER_MS, Settle::Drop),
            (&Read::Unknown, None, Some(None), i64::MAX, Settle::Wait),
        ];
        for (read, creator, signed_in, age, expected) in cases {
            assert_eq!(settle_by(read, creator, signed_in, age), expected, "{creator:?} {signed_in:?} {age}");
        }
    }

    /// Point 1: a 404 read while the POST may still commit drops nothing.
    #[test]
    fn an_absence_read_before_any_send_could_commit_keeps_the_record() {
        let w = Mock::new(None);
        let t0 = w.now_ms();
        w.fates(&[Fate::Fail(timeout()), Fate::Fail(timeout())]);
        let id = unknown_id(press(&w, &mut None, "Ski"));
        assert_eq!(w.state.borrow().pending_creations[0].sent_at, t0 + 1_000, "stamped again before the resend");

        now(resolve(&w));
        assert_eq!(w.pending(), vec![id], "read 404 at once");
        w.later(SETTLED_AFTER_MS - 1_001);
        now(resolve(&w));
        assert_eq!(w.pending(), vec![id], "read 404 just before");
        w.later(1);
        now(resolve(&w));
        assert!(w.pending().is_empty());
        assert!(w.store().projects.is_empty());
    }

    #[test]
    fn a_key_refused_read_before_any_send_could_commit_keeps_the_record() {
        let w = Mock::new(None);
        w.fates(&[Fate::Fail(timeout()), Fate::Fail(timeout())]);
        let id = unknown_id(press(&w, &mut None, "Ski"));
        w.rows.borrow_mut().insert(id, make_project(&[9; 32], id, "x", "EUR"));
        now(resolve(&w));
        assert_eq!(w.pending(), vec![id]);
        w.later(SETTLED_AFTER_MS);
        now(resolve(&w));
        assert!(w.pending().is_empty());
    }

    /// Points 1 and 7: an id this process is sending or settling is not touched by another settlement.
    #[test]
    fn a_record_this_process_is_busy_with_is_neither_read_nor_settled() {
        let w = Mock::new(None);
        let id = landed_unknown(&w, &mut None, "Ski");
        w.later(SETTLED_AFTER_MS);
        let busy = Busy::take(id).expect("free");
        now(resolve(&w));
        assert_eq!((w.count("read"), w.pending()), (0, vec![id]));
        drop(busy);
        now(resolve(&w));
        assert_eq!((w.count("join"), w.held(id)), (1, Some(KEY)));
        assert!(Busy::take(id).is_some(), "released");
    }

    #[test]
    fn a_record_settled_since_the_list_was_read_is_not_joined_twice() {
        let w = Mock::new(None);
        landed_unknown(&w, &mut None, "Ski");
        let listed = w.store();
        now(resolve(&w));
        *w.stale.borrow_mut() = Some(listed);
        now(resolve(&w));
        assert_eq!((w.count("read"), w.count("join")), (1, 1));
    }

    /// Points 5 and 6.
    #[test]
    fn an_adoption_waits_for_a_me_that_answered_with_the_creator() {
        let w = Mock::new(Some(ALICE));
        let id = landed_unknown(&w, &mut None, "Ski");
        assert_eq!(w.state.borrow().pending_creations[0].account, Some(ALICE));

        for now_signed_in in [Ok(None), Ok(Some(BOB)), Err(timeout())] {
            *w.signed_in.borrow_mut() = now_signed_in;
            now(resolve(&w));
            assert_eq!((w.pending(), w.count("join"), w.held(id)), (vec![id], 0, None));
        }
        *w.signed_in.borrow_mut() = Ok(Some(ALICE));
        now(resolve(&w));
        assert_eq!(w.log.borrow().last().unwrap(), &format!("join {id} {ALICE}"));
        assert_eq!((w.pending(), w.held(id)), (vec![], Some(KEY)));
    }

    #[test]
    fn an_anonymous_creation_is_joined_only_once_a_me_answered_no_session() {
        let w = Mock::new(None);
        let id = landed_unknown(&w, &mut None, "Ski");
        *w.signed_in.borrow_mut() = Err(timeout());
        now(resolve(&w));
        assert_eq!((w.pending(), w.count("join")), (vec![id], 0));
        *w.signed_in.borrow_mut() = Ok(None);
        now(resolve(&w));
        assert_eq!(w.log.borrow().last().unwrap(), &format!("join {id} anonymous"));
    }

    /// B of the confirmation pass: an anonymous project follows whoever signs in (`account_sync`).
    #[test]
    fn an_anonymous_creation_is_adopted_under_the_account_signed_in_since() {
        let w = Mock::new(None);
        let id = landed_unknown(&w, &mut None, "Ski");
        *w.signed_in.borrow_mut() = Ok(Some(ALICE));
        now(resolve(&w));
        assert_eq!(w.log.borrow().last().unwrap(), &format!("join {id} {ALICE}"));
        assert_eq!((w.pending(), w.held(id)), (vec![], Some(KEY)));
    }

    /// C: a record stamped while the clock was ahead would otherwise never age.
    #[test]
    fn a_record_stamped_ahead_of_the_clock_ages_from_when_it_was_first_seen_so() {
        let w = Mock::new(None);
        w.fates(&[Fate::Fail(timeout()), Fate::Fail(timeout())]);
        let id = unknown_id(press(&w, &mut None, "Ski"));
        let ahead = w.now_ms() + 10 * 3_600_000;
        w.write(|state| stamp(state, id, ahead));

        now(resolve(&w));
        assert_eq!(w.state.borrow().pending_creations[0].sent_at, w.now_ms(), "restamped");
        w.later(SETTLED_AFTER_MS);
        now(resolve(&w));
        assert!(w.pending().is_empty());
    }

    /// D: a form closed and opened again while its creation may still commit.
    #[test]
    fn a_fresh_press_waits_while_a_record_this_account_could_adopt_may_still_commit() {
        let w = Mock::new(None);
        w.fates(&[Fate::Fail(timeout()), Fate::Fail(timeout())]);
        let id = unknown_id(press(&w, &mut None, "Premier"));

        let mut reopened = None;
        assert_eq!(is(&press(&w, &mut reopened, "Second")), "confirming");
        assert_eq!((w.count("send"), w.pending(), reopened), (2, vec![id], None));

        w.later(SETTLED_AFTER_MS);
        let made = press(&w, &mut reopened, "Second").ok().expect("created");
        assert_ne!(made.project.id, id);
        assert_eq!((w.count("send"), w.pending()), (3, vec![]));
    }

    #[test]
    fn a_record_waiting_on_another_account_never_holds_a_creation_back() {
        let w = Mock::new(Some(ALICE));
        let id = landed_unknown(&w, &mut None, "Alice");
        *w.signed_in.borrow_mut() = Ok(Some(BOB));
        let made = press(&w, &mut None, "Bob").ok().expect("created");
        assert_ne!(made.project.id, id);
        assert_eq!(w.pending(), vec![id]);
    }

    #[test]
    fn a_creation_whose_account_is_unknown_is_not_sent() {
        let w = Mock::new(None);
        *w.signed_in.borrow_mut() = Err(timeout());
        assert_eq!(is(&press(&w, &mut None, "Ski")), "refused");
        assert_eq!((w.count("send"), w.pending()), (0, vec![]));
    }

    /// Point 8.
    #[test]
    fn a_record_that_cannot_be_stored_sends_nothing() {
        let w = Mock::new(None);
        w.storable.set(false);
        assert_eq!(is(&press(&w, &mut None, "Ski")), "unsaved");
        assert_eq!(w.count("send"), 0);
    }

    /// Point 4: a form closed and opened again has no `unsent`.
    #[test]
    fn a_fresh_press_that_recovers_an_earlier_creation_creates_nothing_and_names_it() {
        let w = Mock::new(None);
        let id = landed_unknown(&w, &mut None, "Premier");

        let mut reopened = None;
        let result = press(&w, &mut reopened, "Second");
        match result {
            Err(Unsent::Recovered(name)) => assert_eq!(name, "Premier"),
            other => panic!("expected the recovery, got {}", is(&other)),
        }
        assert_eq!((w.count("send"), w.held(id), reopened), (2, Some(KEY), None));

        let made = press(&w, &mut reopened, "Second").ok().expect("created");
        assert_ne!(made.project.id, id);
        assert_eq!(w.count("send"), 3);
        assert_eq!(w.name_of(made.project.id, &KEY), "Second");
    }

    #[test]
    fn a_second_press_carries_on_with_the_project_that_landed_as_last_typed() {
        let w = Mock::new(None);
        let mut unsent = None;
        let id = landed_unknown(&w, &mut unsent, "Premier");
        assert_eq!(unsent, Some(id));

        let made = press(&w, &mut unsent, "Second").ok().expect("carried on");
        assert_eq!((made.project.id, unsent, w.count("send")), (id, None, 2));
        assert_eq!(w.name_of(id, &KEY), "Second");
    }

    #[test]
    fn a_second_press_while_the_outcome_is_still_unknown_creates_nothing() {
        let w = Mock::new(None);
        let mut unsent = None;
        let id = landed_unknown(&w, &mut unsent, "Premier");
        w.reads_fail.set(1);

        assert_eq!(unknown_id(press(&w, &mut unsent, "Second")), id);
        assert_eq!((w.count("send"), w.count("reseal"), unsent, w.pending()), (2, 0, Some(id), vec![id]));
    }

    #[test]
    fn a_second_press_whose_reseal_fails_keeps_the_project_for_the_next() {
        let w = Mock::new(None);
        let mut unsent = None;
        let id = landed_unknown(&w, &mut unsent, "Premier");
        w.reseal_fails.set(true);

        assert_eq!(is(&press(&w, &mut unsent, "Second")), "refused");
        assert_eq!((unsent, w.held(id), w.count("send")), (Some(id), Some(KEY), 2));

        w.reseal_fails.set(false);
        let made = press(&w, &mut unsent, "Third").ok().expect("carried on");
        assert_eq!((made.project.id, w.count("send")), (id, 2));
        assert_eq!(w.name_of(id, &KEY), "Third");
    }

    #[test]
    fn a_second_press_after_a_creation_that_never_landed_creates_once_settled() {
        let w = Mock::new(None);
        let mut unsent = None;
        w.fates(&[Fate::Fail(timeout()), Fate::Fail(timeout())]);
        let id = unknown_id(press(&w, &mut unsent, "Premier"));

        assert_eq!(unknown_id(press(&w, &mut unsent, "Premier")), id, "too soon to believe the 404");
        w.later(SETTLED_AFTER_MS);
        let made = press(&w, &mut unsent, "Premier").ok().expect("created");
        assert_ne!(made.project.id, id);
        assert!(w.pending().is_empty());
    }

    #[test]
    fn a_pending_record_survives_the_store_round_trip() {
        let mut state = LocalStorageState::default();
        state.pending_creations.push(PendingCreation {
            project_id: Uuid::new_v4(),
            key: key_to_fragment(&[3u8; 32]),
            creator_secret: Some("s".into()),
            account: Some(ALICE),
            sent_at: 42,
        });
        let read: LocalStorageState = serde_json::from_str(&serde_json::to_string(&state).unwrap()).unwrap();
        assert_eq!(read.pending_creations, state.pending_creations);
        assert!(!serde_json::to_string(&LocalStorageState::default()).unwrap().contains("pending_creations"));
    }
}
