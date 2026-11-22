use dioxus::prelude::{Signal, WritableExt};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use shared::time::DateTime;
use shared::{
    CreatableExpense, DeleteExpenseRequest, EditableExpense, EditableProject, EncryptedPair, Expense,
    HistoryContext, ProjectDto, ProjectPayload, ProjectStatus,
};

use crate::crypto::{decrypt_json, encrypt_json, encrypt_padded_json};
use std::collections::HashSet;
use uuid::Uuid;

use super::local_storage::{key_of, project_key, read_from_ls};
use super::queue_store::{device_store, erase_device, forget, in_one_step, leave, write_change, Snapshot, Store, OLD_KEY};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueuedOp {
    pub id: Uuid,
    /// Only in queues an older build wrote, which kept the name in clear.
    #[serde(default)]
    pub label: String,
    /// The expense or project name, sealed under the project key like the op itself: the queue is
    /// a plain file or localStorage entry that outlives logout.
    #[serde(default)]
    pub sealed_label: Option<EncryptedPair>,
    pub op: OpKind,
    /// Replays only if that op lands: a move's delete names the add it follows, so an add the
    /// server drops never leaves the expense in neither project.
    #[serde(default)]
    pub after: Option<Uuid>,
    /// Seconds since the epoch. An op that lands long after it was queued waited for the network,
    /// which is what the Hors ligne badge counts (every write is queued, online too).
    #[serde(default)]
    pub queued_at: Option<i64>,
}

/// How long an op must have waited before its landing counts as a change made offline.
pub const WAITED_SECS: i64 = 60;

impl QueuedOp {
    /// Every op is queued from a page that holds its project's key, so the key is in the store.
    pub fn new(id: Uuid, label: &str, op: OpKind) -> Self {
        let sealed_label = project_key(op.project_id()).and_then(|k| encrypt_json(&k, &label).ok());
        Self { id, label: String::new(), sealed_label, op, after: None, queued_at: Some(DateTime::now_utc().unix_secs()) }
    }

    /// Empty when the key is gone from this device since the op was queued.
    pub fn label(&self) -> String {
        match &self.sealed_label {
            Some(sealed) => project_key(self.op.project_id())
                .and_then(|k| decrypt_json::<String>(&k, sealed).ok())
                .unwrap_or_default(),
            None => self.label.clone(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum OpKind {
    AddExpense(CreatableExpense),
    EditExpense(EditableExpense),
    DeleteExpense(DeleteExpenseRequest),
    /// Only in queues a build before 3.2.5 wrote. A status alone still replays; a whole payload is
    /// dropped (`queue_replay::is_legacy_overwrite`), since the server refuses it without `expected_iv`.
    UpdateProject(EditableProject),
    EditProject(ProjectEdit),
}

impl OpKind {
    pub fn project_id(&self) -> Uuid {
        match self {
            OpKind::AddExpense(p) => p.project_id,
            OpKind::EditExpense(p) => p.project_id,
            OpKind::DeleteExpense(p) => p.project_id,
            OpKind::UpdateProject(p) => p.id,
            OpKind::EditProject(e) => e.project_id,
        }
    }

    fn row(&self) -> Option<i64> {
        match self {
            OpKind::EditExpense(p) => Some(p.id.into()),
            OpKind::DeleteExpense(p) => Some(p.id.into()),
            _ => None,
        }
    }
}

/// A project edit queued offline, kept as what the user changed rather than as a payload, so the
/// replay applies it to whatever the project holds by then instead of overwriting it.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProjectEdit {
    pub project_id: Uuid,
    /// Name and description travel together: the form edits both, and an `Option<Option<_>>`
    /// would lose "description cleared" through JSON. Sealed under the project key, since the queue
    /// is a plain file or localStorage entry that outlives logout.
    #[serde(default)]
    pub name_description: Option<EncryptedPair>,
    #[serde(default)]
    pub status: Option<ProjectStatus>,
    #[serde(default)]
    pub history: Option<HistoryContext>,
    /// `Some` only when the emoji was changed, sealed like the name: `Option<String>`, `None` for
    /// back to the initial.
    #[serde(default)]
    pub emoji: Option<EncryptedPair>,
}

impl ProjectEdit {
    /// The write that applies this edit to `stored`, as the online paths build it: a status goes
    /// in the payload - see `status_edit`.
    pub fn apply(&self, key: &[u8; 32], stored: &ProjectDto) -> Result<EditableProject, String> {
        let mut payload: ProjectPayload = decrypt_json(key, &stored.payload)?;
        let mut changed = false;
        if let Some(sealed) = &self.name_description {
            let (name, description): (String, Option<String>) = decrypt_json(key, sealed)?;
            payload.name = name;
            payload.description = description;
            changed = true;
        }
        if let Some(status) = &self.status {
            payload.status = Some(status.clone());
            changed = true;
        }
        if let Some(sealed) = &self.emoji {
            let emoji: Option<String> = decrypt_json(key, sealed)?;
            payload.set_emoji(emoji.as_deref());
            changed = true;
        }
        Ok(EditableProject {
            id: self.project_id,
            payload: if changed { Some(encrypt_padded_json(key, &payload)?) } else { None },
            status: (self.status.is_some() && stored.status != ProjectStatus::Ongoing)
                .then_some(ProjectStatus::Ongoing),
            history: self.history.clone(),
            expected_iv: None,
            notify_closed: self.status == Some(ProjectStatus::Closed),
        })
    }
}

/// What a read of the queue makes of a live entry (SPEC §15.9.5 Q1, Q3).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    Ready,
    /// Not readable by this build, or a version only 3.4.1 writes: kept, counted, never sent.
    Held,
    /// Names in `after` an entry still live.
    Waits,
    /// Names in `after` an id of which nothing is stored, or sits in a cycle: listed (Q9).
    Stuck,
    /// Names in `after` an entry that left without landing: it leaves too, whenever a build reads it.
    Leaves,
}

/// One stored entry, kept as the JSON it was read as: unknown keys at any depth and entries this
/// build cannot read are never rewritten (Q1, Q7). `op` is set only on an entry this build replays;
/// what it sends is built from `op`, so no unknown key is sent.
#[derive(Debug, Clone)]
pub struct Entry {
    raw: Value,
    pub id: Option<Uuid>,
    pub after: Option<Uuid>,
    /// `None` when it cannot be read from the entry, which then stops every entry behind it (Q4).
    pub project: Option<Uuid>,
    row: Option<i64>,
    /// Creates a row (Q4): an `add_expense`, or a 3.4.1 `write` of an expense with no `row` whose
    /// `eid` is its own `id`.
    creates: bool,
    pub op: Option<QueuedOp>,
    /// The storage key of this version, its generation and its place in the queue.
    pub key: String,
    pub gen: i64,
    pub seq: i64,
    pub status: Status,
}

impl PartialEq for Entry {
    fn eq(&self, other: &Self) -> bool {
        self.raw == other.raw && self.key == other.key && self.status == other.status
    }
}

pub type Queue = Vec<Entry>;

impl Entry {
    pub fn new(op: QueuedOp) -> Self {
        Self::read(serde_json::to_value(&op).unwrap_or(Value::Null))
    }

    /// Held, `op` unset: an entry with a `state` (3.4.1 parks with `needs_choice`), of a type this
    /// build does not replay (`write` and `remove` included), or a known type that does not parse,
    /// has a number outside i64, a ciphertext that is not exactly `{ct, iv}`, a `seq` that is not
    /// an integer or a `gen` other than 0. Without a readable `seq` it sorts last.
    pub fn read(raw: Value) -> Self {
        let in_op = |key: &str| raw.get("op").and_then(|op| op.get(key));
        let kind = in_op("type").and_then(Value::as_str);
        let readable = raw.get("state").is_none()
            && raw.get("seq").is_none_or(Value::is_i64)
            && raw.get("gen").is_none_or(|g| g.as_i64() == Some(0))
            && well_formed(&raw);
        let op = readable.then(|| serde_json::from_value::<QueuedOp>(raw.clone()).ok()).flatten();
        let project_field = match kind {
            Some("add_expense" | "edit_expense" | "delete_expense") => "projectId",
            Some("update_project") => "id",
            _ => "project_id",
        };
        let row = match (&op, kind) {
            (Some(queued), _) => queued.op.row(),
            (None, Some("edit_expense" | "delete_expense")) => in_op("id").and_then(Value::as_i64),
            (None, Some("write" | "remove")) if in_op("slot").and_then(Value::as_str) == Some("expense") => {
                in_op("row").and_then(Value::as_i64)
            }
            _ => None,
        };
        let creates = match kind {
            Some("add_expense") => true,
            Some("write") => {
                in_op("slot").and_then(Value::as_str) == Some("expense")
                    && in_op("row").is_none_or(Value::is_null)
                    && uuid_in(in_op("eid")).is_some_and(|eid| Some(eid) == uuid_in(raw.get("id")))
            }
            _ => false,
        };
        Self {
            id: uuid_in(raw.get("id")),
            after: uuid_in(raw.get("after")),
            project: uuid_in(in_op(project_field)),
            row,
            creates,
            status: if op.is_some() { Status::Ready } else { Status::Held },
            op,
            key: String::new(),
            gen: 0,
            seq: raw.get("seq").and_then(Value::as_i64).unwrap_or(i64::MAX),
            raw,
        }
    }

    pub fn raw(&self) -> &Value {
        &self.raw
    }

    pub fn row(&self) -> Option<i64> {
        self.row
    }

    /// Q4: two entries are about different rows only if both name a row and the rows differ, or one
    /// creates a row and the other names one.
    pub fn about_another_row_than(&self, other: &Entry) -> bool {
        match (self.row, other.row) {
            (Some(a), Some(b)) => a != b,
            (Some(_), None) => other.creates,
            (None, Some(_)) => self.creates,
            (None, None) => false,
        }
    }

    /// The name the app shows for it, opened under any of `keys`; empty when none opens it.
    pub fn name_under<'a>(&self, mut keys: impl Iterator<Item = &'a [u8; 32]>) -> String {
        match self.raw.get("sealed_label").filter(|v| !v.is_null()) {
            Some(sealed) => serde_json::from_value::<EncryptedPair>(sealed.clone())
                .ok()
                .and_then(|sealed| keys.find_map(|k| decrypt_json::<String>(k, &sealed).ok()))
                .unwrap_or_default(),
            None => self.raw.get("label").and_then(Value::as_str).unwrap_or_default().to_string(),
        }
    }
}

fn uuid_in(value: Option<&Value>) -> Option<Uuid> {
    value.and_then(Value::as_str).and_then(|s| Uuid::parse_str(s).ok())
}

fn well_formed(value: &Value) -> bool {
    match value {
        Value::Number(n) => n.is_i64(),
        Value::Object(map) if map.contains_key("ct") || map.contains_key("iv") => {
            map.len() == 2 && map.get("ct").is_some_and(Value::is_string) && map.get("iv").is_some_and(Value::is_string)
        }
        Value::Object(map) => map.values().all(well_formed),
        Value::Array(items) => items.iter().all(well_formed),
        _ => true,
    }
}

/// The id an expense queued offline is listed under until its replay lands. No server row has it:
/// `expenses.id` is a SERIAL, so it is never negative. Taken from the entry's `id`, so a row keeps
/// its id whatever another tab adds to the queue; two entries whose ids fall on one value take the
/// next free one, in queue order, so no two rows share a render key.
pub fn is_pending_expense(id: i32) -> bool {
    id < 0
}

fn pending_id(entry: &Entry, taken: &mut HashSet<i32>) -> i32 {
    let bits = entry.id.map_or(0, |id| id.as_u64_pair().1);
    let mut id = -((bits & 0x3fff_ffff) as i32) - 1;
    while !taken.insert(id) {
        id = if id == i32::MIN { -1 } else { id - 1 };
    }
    id
}

fn queued_rows(queue: &[Entry], project_id: Uuid, now: DateTime) -> Vec<(Option<Uuid>, Expense)> {
    let mut taken = HashSet::new();
    queue
        .iter()
        .filter_map(|entry| match entry.op.as_ref().map(|q| &q.op) {
            Some(OpKind::AddExpense(c)) if c.project_id == project_id => Some((
                entry.id,
                Expense {
                    id: pending_id(entry, &mut taken),
                    author_id: c.author_id,
                    project_id,
                    created_at: DateTime::from_unix_millis(entry.seq).filter(|_| entry.seq < i64::MAX).unwrap_or(now),
                    payload: c.payload.clone(),
                },
            )),
            _ => None,
        })
        .collect()
}

/// The expenses queued offline for `project_id`, as rows the page lists like any other: the queued
/// ciphertext carries its shares, so totals and balances count them before the replay. One leaves
/// this list when its entry lands, and the refetch that follows brings the server's row. Each is
/// dated by its `seq`, so the later add sorts first.
pub fn queued_expenses(queue: &[Entry], project_id: Uuid, now: DateTime) -> Vec<Expense> {
    queued_rows(queue, project_id, now).into_iter().map(|(_, row)| row).collect()
}

/// The row `queued_expenses` lists the add `op_id` under.
pub fn queued_expense(queue: &[Entry], op_id: Uuid, now: DateTime) -> Option<Expense> {
    let project = queue.iter().find(|entry| entry.id == Some(op_id))?.project?;
    queued_rows(queue, project, now).into_iter().find(|(id, _)| *id == Some(op_id)).map(|(_, row)| row)
}

/// The rows of `project_id` whose queued edit has not landed yet: listed with the edit applied
/// (the modal patches the store), flagged as pending until the replay answers.
pub fn queued_edit_ids(queue: &[Entry], project_id: Uuid) -> HashSet<i32> {
    queue
        .iter()
        .filter_map(|entry| match entry.op.as_ref().map(|q| &q.op) {
            Some(OpKind::EditExpense(e)) if e.project_id == project_id => Some(e.id),
            _ => None,
        })
        .collect()
}

/// `entry` as the replay may send it with `current`'s token: resealed from the project's previous
/// keys in what is sent, the stored entry left as it is (Q5). `None` when a pair opens under none
/// of them: it would write a row no member's key opens.
pub fn outgoing(entry: &Entry, current: &[u8; 32], previous: &[[u8; 32]]) -> Option<QueuedOp> {
    let mut value = entry.raw.clone();
    if !crate::rotation::rekey_to_current(&mut value, current, previous) {
        return None;
    }
    Entry::read(value).op
}

/// The queue to show, read from storage, the old key's entries not moved yet last. Writes nothing,
/// so a render may call it.
pub fn read_queue() -> Queue {
    let mut store = device_store();
    let snap = Snapshot::read(&mut store);
    let mut queue = snap.queue();
    queue.extend(snap.not_moved(store.get(OLD_KEY), store.now_ms()));
    queue
}

#[derive(Debug, PartialEq)]
enum Written {
    Made,
    /// The stored device state holds no key for one of the projects (Q3), as after an erase.
    NoKey,
    /// The storage write itself failed.
    Failed,
}

/// Q3: an entry is made only while the stored device state holds its project's key, read in the
/// same step as its write; the entries of one handler are written together, once it has moved the
/// old key's entries it sees (Q2d).
fn write_ops(ops: &[QueuedOp]) -> Written {
    let state = read_from_ls();
    let held = |project: Uuid| state.projects.iter().any(|p| p.project_id == project && key_of(p).is_some());
    if !ops.iter().all(|op| held(op.op.project_id())) {
        return Written::NoKey;
    }
    let Ok(raws) = ops.iter().map(serde_json::to_value).collect::<Result<Vec<_>, _>>() else { return Written::Failed };
    match in_one_step(|store| write_change(store, &raws)) {
        Some(true) => Written::Made,
        _ => Written::Failed,
    }
}

/// Q2c: in storage before this returns, or not made at all. A write that throws frees the caches
/// the server can give back and is tried once more; a refusal for a missing key frees nothing.
pub fn enqueue_ops(ops: &[QueuedOp]) -> bool {
    match write_ops(ops) {
        Written::Made => true,
        Written::NoKey => false,
        Written::Failed => {
            super::local_storage::free_caches();
            super::fx_cache::clear_fx();
            write_ops(ops) == Written::Made
        }
    }
}

/// [`enqueue_ops`], then the queue shown again. False when nothing was made: the user is told here,
/// and the caller shows nothing as saved.
pub fn enqueue_all(mut pending: Signal<Queue>, ops: Vec<QueuedOp>) -> bool {
    let made = enqueue_ops(&ops);
    if !made {
        if let Some(mut flash) = dioxus::prelude::try_consume_context::<Signal<Option<super::Flash>>>() {
            flash.set(Some(super::Flash::err(crate::tid!("queue-not-saved"))));
        }
    }
    pending.set(read_queue());
    made
}

pub fn enqueue(pending: Signal<Queue>, op: QueuedOp) -> bool {
    enqueue_all(pending, vec![op])
}

fn discard_stored(entry: &Entry) -> bool {
    in_one_step(|store| {
        let snap = Snapshot::read(store);
        leave(store, &snap, entry)
    }) == Some(true)
}

/// Its user's choice (Q3): the version they were shown gets its `x` key; what waits on it leaves by
/// rule. A marker that cannot be written discards nothing, and the user is told.
pub fn discard(mut pending: Signal<Queue>, entry: &Entry) {
    if !discard_stored(entry) {
        if let Some(mut flash) = dioxus::prelude::try_consume_context::<Signal<Option<super::Flash>>>() {
            flash.set(Some(super::Flash::err(crate::tid!("unsent-discard-failed"))));
        }
    }
    pending.set(read_queue());
}

fn forget_stored(project_id: Uuid) -> bool {
    in_one_step(|store| {
        let snap = Snapshot::read(store);
        forget(store, &snap, project_id)
    }) == Some(true)
}

/// A project taken off this device takes its entries with it (D15), and the entries that wait on
/// them: a move into it leaves the expense where it was.
pub fn forget_project(mut pending: Signal<Queue>, project_id: Uuid) {
    forget_stored(project_id);
    pending.set(read_queue());
}

/// What [`forget_project`] would take, counted in storage rather than in what this tab shows.
pub fn dropped_with_project(project_id: Uuid) -> usize {
    Snapshot::read(&mut device_store()).dropped_with(project_id)
}

/// An erase of the device's data (Q3), once the device state holds no project key.
pub fn erase_queue() {
    erase_device();
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use shared::time::Date;
    use shared::{EncryptedPair, ProjectStatus};

    const P: Uuid = Uuid::from_u128(0x50);

    fn ep() -> EncryptedPair {
        EncryptedPair { ct: "ct".into(), iv: "iv".into() }
    }

    fn queued(op: OpKind) -> QueuedOp {
        QueuedOp { id: Uuid::new_v4(), label: String::new(), sealed_label: None, after: None, queued_at: None, op }
    }

    fn add_expense_op(label: &str) -> QueuedOp {
        let id = Uuid::new_v4();
        QueuedOp {
            id,
            label: label.to_string(),
            sealed_label: None,
            after: None,
            queued_at: None,
            op: OpKind::AddExpense(shared::CreatableExpense {
                project_id: Uuid::nil(),
                author_id: Some(1),
                payload: ep(),
                payers: vec![],
                debtors: vec![],
                history: None,
                client_op_id: Some(id),
            }),
        }
    }

    fn edit_expense_op() -> QueuedOp {
        QueuedOp {
            label: "edit".into(),
            ..queued(OpKind::EditExpense(shared::EditableExpense {
                id: 42,
                project_id: Uuid::nil(),
                author_id: Some(1),
                payload: ep(),
                payers: vec![],
                debtors: vec![],
                history: None,
            }))
        }
    }

    fn delete_expense_op() -> QueuedOp {
        QueuedOp { label: "delete".into(), ..queued(OpKind::DeleteExpense(shared::DeleteExpenseRequest { id: 7, project_id: Uuid::nil(), history: None })) }
    }

    fn update_project_op() -> QueuedOp {
        QueuedOp {
            label: "project".into(),
            ..queued(OpKind::UpdateProject(shared::EditableProject {
                id: Uuid::nil(),
                payload: None,
                status: Some(ProjectStatus::Closed),
                history: None,
                expected_iv: None,
                notify_closed: false,
            }))
        }
    }

    fn at(op: QueuedOp, seq: i64) -> Entry {
        let mut raw = serde_json::to_value(op).unwrap();
        raw["seq"] = json!(seq);
        Entry::read(raw)
    }

    /// The idempotency key must survive the round trip through storage, or a replay after a
    /// restart would be the duplicate it exists to prevent.
    #[test]
    fn a_queued_add_keeps_its_client_op_id_across_persistence() {
        let op = add_expense_op("a");
        let entry = Entry::new(op.clone());
        match Entry::read(serde_json::from_str(&entry.raw.to_string()).unwrap()).op.unwrap().op {
            OpKind::AddExpense(p) => assert_eq!(p.client_op_id, Some(op.id)),
            _ => panic!("not an add"),
        }
    }

    /// A pending row is named by its entry, not by its place: another tab's entries entering the
    /// queue ahead of it change nothing, and a later add sorts first.
    #[test]
    fn only_this_projects_queued_adds_are_listed_under_ids_their_entries_give() {
        let other = Uuid::new_v4();
        let mut elsewhere = add_expense_op("elsewhere");
        if let OpKind::AddExpense(c) = &mut elsewhere.op {
            c.project_id = other;
        }
        let queue = vec![at(add_expense_op("a"), 100), at(edit_expense_op(), 101), at(elsewhere, 102), at(add_expense_op("b"), 103)];
        let now = Date::from_ymd_opt(2026, 10, 17).unwrap().and_hms_opt(18, 42, 0).unwrap();

        let rows = queued_expenses(&queue, Uuid::nil(), now);
        let ids: Vec<i32> = rows.iter().map(|e| e.id).collect();
        assert_eq!(ids.len(), 2);
        assert!(rows.iter().all(|e| is_pending_expense(e.id) && e.project_id == Uuid::nil()));
        assert!(rows[0].created_at < rows[1].created_at, "dated by seq, so the later add sorts first");
        assert_eq!(rows[0].payload, ep());
        assert_eq!(queued_expenses(&queue, other, now).len(), 1);
        assert_eq!(queued_expense(&queue, queue[3].id.unwrap(), now).map(|e| e.id), Some(ids[1]), "found by its op");
        assert_eq!(queued_expense(&queue, queue[1].id.unwrap(), now), None, "an edit lists no row");

        let mut with_another_tabs = vec![at(add_expense_op("theirs"), 50)];
        with_another_tabs.extend(queue.iter().cloned());
        assert_eq!(queued_expenses(&with_another_tabs, Uuid::nil(), now).iter().map(|e| e.id).skip(1).collect::<Vec<_>>(), ids);
    }

    /// R1, R2: a queued add is read like a served row, so a payload that names another slot, or one
    /// key without the other, is left out as one that does not open.
    #[test]
    fn a_queued_add_that_fails_r1_or_r2_is_read_as_one_that_does_not_open() {
        use crate::common::test_fixtures::test_key;
        let key = test_key();
        let eid = Uuid::new_v4();
        let queued_with = |keys: String| {
            let mut op = add_expense_op("a");
            if let OpKind::AddExpense(c) = &mut op.op {
                c.payload = crate::crypto::encrypt(
                    &key,
                    &format!(r#"{{"name":"Taxi","amount":12.5,"expenseType":"expense","date":"2025-01-01"{keys}}}"#),
                )
                .unwrap();
            }
            Entry::new(op)
        };
        let queue = vec![
            queued_with(format!(r#","eid":"{eid}","slot":"expense""#)),
            queued_with(format!(r#","eid":"{eid}","slot":"rule""#)),
            queued_with(format!(r#","eid":"{eid}""#)),
            queued_with(String::new()),
        ];
        let rows = queued_expenses(&queue, Uuid::nil(), DateTime::now_utc());
        assert_eq!(rows.len(), 4);
        let (read, _) = crate::decrypted::decrypt_ledger(&key, &rows, &[]);
        assert_eq!(read.iter().map(|e| e.id).collect::<Vec<_>>(), [rows[0].id, rows[3].id]);
    }

    #[test]
    fn only_this_projects_queued_edits_flag_their_rows() {
        let mut elsewhere = edit_expense_op();
        if let OpKind::EditExpense(e) = &mut elsewhere.op {
            e.project_id = Uuid::new_v4();
            e.id = 9;
        }
        let queue: Queue = [add_expense_op("a"), edit_expense_op(), elsewhere, delete_expense_op()].into_iter().map(Entry::new).collect();
        assert_eq!(queued_edit_ids(&queue, Uuid::nil()), [42].into());
    }


    /// Two adds written in the same millisecond, by two tabs, get two render keys.
    #[test]
    fn pending_rows_of_one_millisecond_get_distinct_ids() {
        let now = Date::from_ymd_opt(2026, 10, 17).unwrap().and_hms_opt(18, 42, 0).unwrap();
        let queue = vec![at(add_expense_op("a"), 100), at(add_expense_op("b"), 100), at(add_expense_op("c"), 100 + (1 << 30))];
        let ids: HashSet<i32> = queued_expenses(&queue, Uuid::nil(), now).iter().map(|e| e.id).collect();
        assert_eq!(ids.len(), 3);
    }
    #[test]
    fn a_queue_written_before_the_dependency_still_reads() {
        let old = r#"{"id":"00000000-0000-0000-0000-000000000001","op":{"type":"delete_expense","id":3,"projectId":"00000000-0000-0000-0000-000000000002"}}"#;
        let entry = Entry::read(serde_json::from_str(old).unwrap());
        assert_eq!(entry.after, None);
        assert_eq!(entry.seq, i64::MAX, "it takes a seq when moved");
        assert!(entry.op.is_some_and(|q| q.after.is_none() && q.queued_at.is_none()));
    }

    /// Q1: an entry this build cannot read is held and the others still read, the 3.4.1 shapes and
    /// `needs_choice` included.
    #[test]
    fn an_entry_this_build_cannot_read_is_held_and_the_others_still_read() {
        let p = P.to_string();
        let stored: Vec<Value> = serde_json::from_str(&format!(
            r#"[
            {{"id":"00000000-0000-0000-0000-000000000001","label":"","op":{{"type":"delete_expense","id":3,"projectId":"{p}"}}}},
            {{"id":"00000000-0000-0000-0000-000000000002","label":"","op":{{"type":"remove","project_id":"{p}","slot":"expense","eid":null,"row":4,"base_rev":null,"base":null}}}},
            {{"id":"00000000-0000-0000-0000-000000000003","label":"","queued_at":1,"state":"needs_choice","op":{{"type":"write","project_id":"{p}","slot":"expense","eid":null,"row":9,"base_rev":null,"base":null,"intent":{{"ct":"c","iv":"i"}}}}}},
            {{"id":"00000000-0000-0000-0000-000000000004","label":"","op":{{"type":"write","project_id":"{p}","slot":"project","eid":null,"row":null,"base_rev":2,"base":{{"ct":"a","iv":"b"}},"intent":{{"ct":"c","iv":"d"}}}}}},
            {{"id":"00000000-0000-0000-0000-000000000005","label":"","op":{{"type":"split_bill","project_id":"{p}"}}}},
            {{"id":"00000000-0000-0000-0000-000000000006","label":"","op":{{"type":"delete_expense","id":3.5,"projectId":"{p}"}}}},
            {{"id":"00000000-0000-0000-0000-000000000007","label":"","op":{{"type":"delete_expense","id":3,"projectId":"{p}","history":{{"payload":{{"ct":"a","iv":"b","tag":"c"}}}}}}}},
            {{"id":"00000000-0000-0000-0000-000000000008","label":"","state":"needs_choice","op":{{"type":"delete_expense","id":3,"projectId":"{p}"}}}},
            {{"id":"00000000-0000-0000-0000-000000000009","label":"","op":{{"type":"edit_project","project_id":"{p}","status":"someday"}}}},
            {{"id":"00000000-0000-0000-0000-00000000000a","label":"","op":{{"type":"delete_expense","id":9223372036854775808,"projectId":"{p}"}}}},
            {{"id":"00000000-0000-0000-0000-00000000000c","label":"","seq":"soon","op":{{"type":"delete_expense","id":3,"projectId":"{p}"}}}},
            {{"id":"00000000-0000-0000-0000-00000000000d","label":"","gen":1,"op":{{"type":"delete_expense","id":3,"projectId":"{p}"}}}},
            {{"id":"00000000-0000-0000-0000-00000000000e","label":"","state":null,"op":{{"type":"delete_expense","id":3,"projectId":"{p}"}}}},
            "not an entry",
            {{"id":"00000000-0000-0000-0000-00000000000b","label":"","seq":7,"gen":0,"op":{{"type":"add_expense","projectId":"{p}","payload":{{"ct":"x","iv":"y"}}}}}}
        ]"#
        ))
        .unwrap();
        let queue: Queue = stored.iter().cloned().map(Entry::read).collect();
        let ready: Vec<usize> = (0..queue.len()).filter(|&i| queue[i].op.is_some()).collect();
        assert_eq!(ready, vec![0, 14]);
        assert!(queue.iter().take(13).all(|e| e.project == Some(P)), "each one's project is read, held ones included");
        assert_eq!(queue.iter().map(|e| e.row).collect::<Vec<_>>()[..4], [Some(3), Some(4), Some(9), None]);
        assert_eq!((queue[13].id, queue[13].project), (None, None));
        assert_eq!(queue.iter().map(|e| e.raw.clone()).collect::<Vec<_>>(), stored, "each held as the same value");
        assert_eq!(queue[14].seq, 7);
    }

    /// Q7: keys this build does not know stay in what is stored and never go out.
    #[test]
    fn unknown_keys_are_kept_and_never_sent() {
        let k = crate::crypto::generate_key();
        let payload = serde_json::to_value(encrypt_json(&k, &"x").unwrap()).unwrap();
        let entry = Entry::read(json!({
            "id": Uuid::new_v4(), "label": "", "later": { "n": 1 },
            "op": { "type": "delete_expense", "id": 3, "projectId": P, "also": 2, "history": { "payload": payload } }
        }));
        assert!(entry.op.is_some(), "an unknown key does not hold a known type");
        assert_eq!((entry.raw["later"].clone(), entry.raw["op"]["also"].clone()), (json!({ "n": 1 }), json!(2)));
        let sent = serde_json::to_value(outgoing(&entry, &k, &[]).unwrap()).unwrap();
        assert!(sent.get("later").is_none() && sent["op"].get("also").is_none(), "{sent}");
    }

    /// A queue written before the label was sealed must still load and still name its ops.
    #[test]
    fn a_queue_from_an_older_build_keeps_its_clear_label() {
        let old = r#"{"id":"00000000-0000-0000-0000-000000000001","label":"Café","op":{"type":"delete_expense","id":3,"projectId":"00000000-0000-0000-0000-000000000002"}}"#;
        let entry = Entry::read(serde_json::from_str(old).unwrap());
        assert_eq!(entry.op.as_ref().unwrap().label, "Café");
        assert_eq!(entry.name_under(std::iter::empty()), "Café");
    }

    #[test]
    fn test_opkind_json_tag_format() {
        let ops = [add_expense_op("a"), edit_expense_op(), delete_expense_op(), update_project_op()];
        let expected_types = ["add_expense", "edit_expense", "delete_expense", "update_project"];
        for (op, expected) in ops.iter().zip(expected_types.iter()) {
            assert_eq!(Entry::new(op.clone()).raw["op"]["type"].as_str().unwrap(), *expected, "wrong type tag for {expected}");
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    mod native {
        use super::*;
        use crate::common::env_lock;
        use crate::common::persist::file_for;
        use crate::common::queue_store::{entry_key, Store, OLD_KEY};
        use crate::common::test_fixtures::with_data_dir;

        // with_data_dir takes the process-wide env_lock and starts from an empty dir.
        fn with_tmp_dir<F: FnOnce()>(suffix: &str, f: F) {
            with_data_dir(&format!("queue_{suffix}"), |_| f())
        }

        fn holding(projects: &[(Uuid, [u8; 32])]) {
            let mut state = crate::common::LocalStorageState::default();
            for (project_id, key) in projects {
                crate::common::upsert_project(&mut state, *project_id, None);
                crate::common::upsert_project_key(&mut state, *project_id, crate::crypto::key_to_fragment(key));
            }
            crate::common::write_to_ls(&state);
        }

        fn on_disk() -> serde_json::Map<String, Value> {
            serde_json::from_slice(&std::fs::read(file_for("counted_q")).unwrap()).unwrap()
        }

        fn push(op: QueuedOp) -> bool {
            enqueue_ops(&[op])
        }

        fn held_nil() {
            holding(&[(Uuid::nil(), crate::crypto::generate_key())]);
        }

        #[test]
        fn a_queued_name_is_sealed_under_the_project_key() {
            with_tmp_dir("sealed_label", || {
                let project_id = Uuid::from_u128(7);
                let key = crate::crypto::generate_key();
                holding(&[(project_id, key)]);
                let edit = ProjectEdit { project_id, name_description: None, status: Some(ProjectStatus::Closed), history: None, emoji: None };
                let op = QueuedOp::new(Uuid::new_v4(), "Corse", OpKind::EditProject(edit));

                assert!(!serde_json::to_string(&op).unwrap().contains("Corse"), "the queue holds no plaintext name");
                assert_eq!(op.label(), "Corse", "and the conflict message still names it");
                assert_eq!(Entry::new(op).name_under([key].iter()), "Corse");
            });
        }

        /// I1, I14: one file holding the keys and values as one JSON object, the entry in it before
        /// the call returns, each value the string the web stores.
        #[test]
        fn an_entry_is_in_the_file_before_the_call_returns() {
            with_tmp_dir("one_file", || {
                held_nil();
                let op = add_expense_op("Café");
                assert!(push(op.clone()));
                let stored = on_disk();
                let value: Value = serde_json::from_str(stored[&entry_key(op.id)].as_str().unwrap()).unwrap();
                assert_eq!(value["id"], json!(op.id));
                assert!(value["seq"].is_i64());
                assert_eq!(read_queue().len(), 1);
            });
        }

        /// Q3: no entry is made for a project whose key the stored device state does not hold, as
        /// after an erase in another tab.
        #[test]
        fn no_entry_is_made_without_its_project_key_in_the_device_state() {
            with_tmp_dir("no_key", || {
                holding(&[]);
                assert!(!push(add_expense_op("Café")));
                assert!(read_queue().is_empty());
            });
        }

        /// Q2c: a store that cannot be written makes nothing, and says so to the caller.
        #[test]
        fn a_write_that_fails_makes_nothing() {
            with_tmp_dir("unwritable", || {
                held_nil();
                std::fs::create_dir_all(file_for("counted_q")).unwrap();
                assert!(!push(add_expense_op("Café")), "shown as saved by the first build");
                std::fs::remove_dir_all(file_for("counted_q")).unwrap();
                assert!(read_queue().is_empty());
            });
        }

        /// Q5: a switch writes nothing to the queue; the replay reseals what it sends from the
        /// previous keys, the stored entry left as it is. Replaces the first build's reseal at the
        /// switch, which wrote every entry again.
        #[test]
        fn a_key_switch_writes_nothing_to_the_queue_and_the_replay_reseals() {
            with_tmp_dir("switch", || {
                use crate::common::local_storage::{key_of, previous_keys_of, read_from_ls};
                let project_id = Uuid::from_u128(9);
                let (k1, k2, k3, stranger) = (crate::crypto::generate_key(), crate::crypto::generate_key(), crate::crypto::generate_key(), crate::crypto::generate_key());
                holding(&[(project_id, k1)]);
                let add_under = |key: &[u8; 32], name: &str| {
                    QueuedOp::new(
                        Uuid::new_v4(),
                        name,
                        OpKind::AddExpense(shared::CreatableExpense {
                            project_id,
                            author_id: None,
                            payload: encrypt_json(key, &name).unwrap(),
                            payers: vec![],
                            debtors: vec![shared::EncryptedUserAmount { user_id: 1, payload: encrypt_json(key, &12.5).unwrap() }],
                            history: Some(shared::HistoryContext { actor_user_id: None, payload: encrypt_json(key, &"added").unwrap(), eid: None }),
                            client_op_id: None,
                        }),
                    )
                };
                let opens_under = |key: &[u8; 32], op: &QueuedOp| {
                    let OpKind::AddExpense(c) = &op.op else { return false };
                    decrypt_json::<String>(key, &c.payload).is_ok()
                        && decrypt_json::<f64>(key, &c.debtors[0].payload).is_ok()
                        && decrypt_json::<String>(key, &c.history.as_ref().unwrap().payload).is_ok()
                        && decrypt_json::<String>(key, op.sealed_label.as_ref().unwrap()).is_ok()
                };
                let keys = || {
                    let state = read_from_ls();
                    let entry = state.projects.iter().find(|p| p.project_id == project_id).unwrap().clone();
                    (key_of(&entry).unwrap(), previous_keys_of(&entry))
                };
                let switch = |from: &[u8; 32], to: &[u8; 32], epoch: i32| {
                    let mut state = read_from_ls();
                    crate::rotation_flow::switch_local(&mut state, project_id, from, to, epoch, epoch as usize);
                    crate::common::write_to_ls(&state);
                };

                assert!(push(add_under(&k1, "Hôtel")));
                let file = std::fs::read(file_for("counted_q")).unwrap();
                switch(&k1, &k2, 1);
                assert_eq!(std::fs::read(file_for("counted_q")).unwrap(), file, "nothing written");

                let shown = read_queue();
                let (current, previous) = keys();
                let sent = outgoing(&shown[0], &current, &previous).expect("an op the device can reseal is sent");
                assert!(opens_under(&k2, &sent), "what the replay sends opens under the key whose token goes with it");
                assert_eq!(sent.label(), "Hôtel");

                switch(&k2, &k3, 2);
                let (current, previous) = keys();
                assert!(opens_under(&k3, &outgoing(&read_queue()[0], &current, &previous).unwrap()), "across two switches");
                let foreign = Entry::new(add_under(&stranger, "Train"));
                assert!(outgoing(&foreign, &current, &previous).is_none(), "a pair no key held here opens is never sent");
            });
        }

        /// Q1, Q7: an entry this build cannot read is never rewritten, whatever else changes.
        #[test]
        fn a_held_entry_stays_as_it_was() {
            with_tmp_dir("held_kept", || {
                held_nil();
                let id = Uuid::new_v4();
                let held = json!({ "op": { "type": "write", "project_id": P, "slot": "expense", "intent": { "ct": "c", "iv": "i" } }, "id": id, "seq": 1, "state": "needs_choice", "x": [1, { "y": null }] }).to_string();
                let mut store = crate::common::queue_store::device_store();
                assert!(store.set(&entry_key(id), &held));
                assert!(push(add_expense_op("x")));
                assert_eq!(read_queue().len(), 2);
                assert_eq!(on_disk()[&entry_key(id)], json!(held));
            });
        }

        #[test]
        fn an_entry_queued_now_has_its_queued_at_and_an_empty_clear_label() {
            with_tmp_dir("queued_at", || {
                let fresh = Entry::new(QueuedOp::new(Uuid::new_v4(), "Café", OpKind::DeleteExpense(shared::DeleteExpenseRequest { id: 1, project_id: P, history: None })));
                assert!(fresh.raw["queued_at"].is_i64());
                assert_eq!(fresh.raw["label"], json!(""));
            });
        }

        /// Q0 on the phone: the old file's entries move to the one file, and the old file is
        /// emptied.
        #[test]
        fn the_old_file_moves_into_the_one_file() {
            with_tmp_dir("old_file", || {
                let old = json!([{ "id": Uuid::from_u128(3), "label": "", "op": { "type": "delete_expense", "id": 3, "projectId": P } }]);
                assert!(crate::common::persist::write_json(OLD_KEY, "old", &old));
                let mut store = crate::common::queue_store::device_store();
                let snap = crate::common::queue_store::Snapshot::read(&mut store);
                assert_eq!(crate::common::queue_store::move_old(&mut store, &snap), crate::common::queue_store::Moved::Whole);
                assert_eq!(crate::common::persist::read_json::<Value>(OLD_KEY), json!([]));
                assert_eq!(read_queue().iter().map(|e| e.id).collect::<Vec<_>>(), vec![Some(Uuid::from_u128(3))]);
                store.set_kept(Some(old.to_string()));
                erase_queue();
                assert_eq!(store.kept(), None, "the erase drops the kept text of a storage event");
                assert!(!file_for(OLD_KEY).exists());
                assert!(read_queue().is_empty());
                assert!(!file_for("counted_q").exists(), "on the phone the erase removes the whole file");
            });
        }


        /// Q1 on the phone: a value that is not a string is kept and held, never a reason to set the
        /// whole file aside.
        #[test]
        fn a_value_that_is_not_a_string_is_held_and_the_file_kept() {
            with_tmp_dir("not_a_string", || {
                held_nil();
                let (readable, odd) = (Uuid::from_u128(1), Uuid::from_u128(2));
                let mut raw = serde_json::to_value(QueuedOp { id: readable, ..delete_expense_op() }).unwrap();
                raw["seq"] = json!(5);
                let odd_value = json!({ "id": odd, "seq": 6, "op": { "type": "delete_expense", "id": 3, "projectId": Uuid::nil() } });
                let mut file = serde_json::Map::new();
                file.insert(entry_key(readable), json!(raw.to_string()));
                file.insert(entry_key(odd), odd_value.clone());
                assert!(crate::common::persist::write_json("counted_q", "test", &file));
                let shown = read_queue();
                assert_eq!(shown.iter().map(|e| (e.id, e.op.is_some())).collect::<Vec<_>>(), vec![(Some(readable), true), (Some(odd), false)]);
                assert!(push(add_expense_op("x")));
                assert_eq!(on_disk()[&entry_key(odd)], odd_value, "kept as it was");
            });
        }

        /// Q2c: a refusal for a missing key is not a full store, and frees no cache.
        #[test]
        fn a_refusal_for_a_missing_key_frees_no_cache() {
            with_tmp_dir("no_key_keeps_cache", || {
                let held = Uuid::from_u128(7);
                holding(&[(held, crate::crypto::generate_key())]);
                let mut state = crate::common::read_from_ls();
                state.projects[0].cached_data_version = Some(3);
                crate::common::write_to_ls(&state);
                assert!(!push(add_expense_op("for a project with no key")));
                assert_eq!(crate::common::read_from_ls().projects[0].cached_data_version, Some(3));
            });
        }

        /// On the phone a change of several keys is one read and one write of the file, and an
        /// erase removes the file.
        #[test]
        fn a_change_of_several_keys_is_one_write_of_the_file() {
            with_tmp_dir("one_write", || {
                use crate::common::queue_store::FILE_WRITES;
                held_nil();
                for n in 0..5 {
                    assert!(push(QueuedOp { id: Uuid::from_u128(n + 1), ..add_expense_op("x") }));
                }
                let count = |f: &dyn Fn()| {
                    FILE_WRITES.with(|w| w.set(0));
                    f();
                    FILE_WRITES.with(|w| w.get())
                };
                assert_eq!(count(&|| assert!(push_all(&[add_expense_op("a"), add_expense_op("b")]))), 1, "two entries of one handler");
                let old = json!([{ "id": Uuid::from_u128(30), "label": "", "op": { "type": "delete_expense", "id": 3, "projectId": Uuid::nil() } }, { "id": Uuid::from_u128(31), "label": "", "op": { "type": "delete_expense", "id": 4, "projectId": Uuid::nil() } }]);
                assert!(crate::common::persist::write_json(OLD_KEY, "old", &old));
                assert_eq!(count(&|| {
                    crate::common::queue_store::in_one_step(|store| {
                        let snap = Snapshot::read(store);
                        crate::common::queue_store::move_old(store, &snap)
                    });
                }), 1, "the old key's two entries, their o keys");
                assert_eq!(crate::common::persist::read_json::<Value>(OLD_KEY), json!([]));
                assert_eq!(read_queue().len(), 9);
                assert!(discard_stored(&read_queue()[0]));
                assert_eq!(count(&|| assert!(forget_stored(Uuid::nil()))), 1, "a p key and an x key for each entry");
                assert!(read_queue().is_empty());
                assert_eq!(count(&erase_queue), 0);
                assert!(!file_for("counted_q").exists());
            });
        }

        fn push_all(ops: &[QueuedOp]) -> bool {
            enqueue_ops(ops)
        }

        /// Q2d: a handler moves the old key's entries it sees before it writes its own, so an older
        /// build's edit of a row is stamped below the edit made here.
        #[test]
        fn a_handler_stamps_an_old_entry_of_the_row_below_its_own_edit() {
            with_tmp_dir("handler_moves", || {
                held_nil();
                let old_edit = Uuid::from_u128(30);
                let old = json!([{ "id": old_edit, "label": "", "op": { "type": "edit_expense", "id": 42, "projectId": Uuid::nil(), "payload": { "ct": "a", "iv": "b" }, "payers": [], "debtors": [] } }]);
                assert!(crate::common::persist::write_json(OLD_KEY, "old", &old));
                let mine = edit_expense_op();
                assert!(push(mine.clone()));
                let shown: Vec<(Option<Uuid>, String)> = read_queue().iter().map(|e| (e.id, e.key.clone())).collect();
                assert_eq!(shown.iter().map(|(id, _)| *id).collect::<Vec<_>>(), vec![Some(old_edit), Some(mine.id)]);
                assert!(shown.iter().all(|(_, key)| key != OLD_KEY), "both stored under the prefix: {shown:?}");
                assert_eq!(crate::common::persist::read_json::<Value>(OLD_KEY), json!([]));
            });
        }
        #[test]
        fn test_read_queue_empty_when_no_file() {
            with_tmp_dir("no_file", || {
                assert!(read_queue().is_empty());
            });
        }

        #[test]
        fn test_write_read_preserves_fifo_order() {
            with_tmp_dir("fifo_order", || {
                held_nil();
                for label in ["first", "second", "third"] {
                    assert!(push(add_expense_op(label)));
                }
                let labels: Vec<String> = read_queue().iter().map(|e| e.op.as_ref().unwrap().label.clone()).collect();
                assert_eq!(labels, vec!["first", "second", "third"]);
            });
        }

        #[test]
        fn test_write_creates_missing_dir() {
            let _guard = env_lock();
            let base = std::env::temp_dir().join("counted_queue_missing_dir");
            std::fs::remove_dir_all(&base).ok();
            let nested = base.join("x").join("y");
            std::env::set_var("COUNTED_DATA_DIR", nested.to_str().unwrap());

            held_nil();
            let made = push(add_expense_op("Café"));
            let loaded = read_queue();
            let file_exists = nested.join("counted_q.json").exists();

            std::env::remove_var("COUNTED_DATA_DIR");
            std::fs::remove_dir_all(&base).ok();

            assert!(made && file_exists, "a change should create missing parent dirs");
            assert_eq!(loaded.len(), 1);
        }

        #[test]
        fn test_write_atomic_no_tmp_left() {
            with_tmp_dir("atomic", || {
                held_nil();
                assert!(push(add_expense_op("x")));
                assert!(!file_for("counted_q").with_extension("json.tmp").exists(), "temp file must not remain after atomic write");
                assert_eq!(read_queue().len(), 1);
            });
        }

        #[test]
        fn test_corrupt_json_returns_empty() {
            with_tmp_dir("corrupt", || {
                std::fs::write(file_for("counted_q"), b"this is not json {{{").unwrap();
                assert!(read_queue().is_empty());
            });
        }

        #[test]
        fn test_all_opkind_variants_roundtrip() {
            with_tmp_dir("all_variants", || {
                held_nil();
                for op in [add_expense_op("add"), edit_expense_op(), delete_expense_op(), update_project_op()] {
                    assert!(push(op));
                }
                let loaded = read_queue();
                let kinds: Vec<&OpKind> = loaded.iter().map(|e| &e.op.as_ref().unwrap().op).collect();
                assert!(matches!(kinds[..], [OpKind::AddExpense(_), OpKind::EditExpense(_), OpKind::DeleteExpense(_), OpKind::UpdateProject(_)]));
            });
        }

        #[test]
        fn test_uuid_preserved_after_roundtrip() {
            with_tmp_dir("uuid_preserved", || {
                held_nil();
                let id = Uuid::parse_str("550e8400-e29b-41d4-a716-446655440099").unwrap();
                assert!(push(QueuedOp { id, ..delete_expense_op() }));
                assert_eq!(read_queue().first().unwrap().id, Some(id));
            });
        }

        #[test]
        fn a_landed_entry_leaves_and_the_rest_stays() {
            with_tmp_dir("landed", || {
                held_nil();
                assert!(push(add_expense_op("to remove")));
                assert!(push(add_expense_op("to keep")));
                let mut store = crate::common::queue_store::device_store();
                let snap = crate::common::queue_store::Snapshot::read(&mut store);
                assert!(crate::common::queue_store::land(&mut store, &snap, &snap.queue()[0], Some(1)));
                let loaded = read_queue();
                assert_eq!(loaded.len(), 1);
                assert_eq!(loaded[0].op.as_ref().unwrap().label, "to keep");
            });
        }
    }
}


#[cfg(test)]
mod project_edit_tests {
    use super::*;
    use crate::common::test_fixtures::{make_project, test_key};
    use crate::crypto::encrypt_json;

    fn stored_with(key: &[u8; 32], f: impl FnOnce(&mut ProjectPayload)) -> ProjectDto {
        let mut p = make_project(key, Uuid::nil(), "Corsica", "EUR");
        let mut payload: ProjectPayload = decrypt_json(key, &p.payload).unwrap();
        f(&mut payload);
        p.payload = encrypt_json(key, &payload).unwrap();
        p
    }

    fn payload_of(key: &[u8; 32], e: &EditableProject) -> ProjectPayload {
        decrypt_json(key, e.payload.as_ref().unwrap()).unwrap()
    }

    fn rename(name: &str, description: Option<&str>) -> ProjectEdit {
        let sealed = encrypt_json(&test_key(), &(name, description)).unwrap();
        ProjectEdit {
            project_id: Uuid::nil(),
            name_description: Some(sealed),
            status: None,
            history: None,
            emoji: None,
        }
    }

    /// The point of queueing the intent: a field another device wrote while this one was offline
    /// is still there after the replay.
    #[test]
    fn a_replayed_rename_keeps_what_was_written_meanwhile() {
        let k = test_key();
        let stored = stored_with(&k, |p| {
            p.extra.insert("kind".into(), "personal".into());
            p.extra.insert("finance".into(), serde_json::json!({ "budgets": [1] }));
        });
        let written = rename("Corse", Some("été")).apply(&k, &stored).unwrap();
        let after = payload_of(&k, &written);
        assert_eq!(after.name, "Corse");
        assert_eq!(after.description.as_deref(), Some("été"));
        assert!(after.is_personal());
        assert_eq!(after.extra["finance"], serde_json::json!({ "budgets": [1] }));
        assert_eq!(after.currency, "EUR");
    }

    #[test]
    fn a_replayed_emoji_change_touches_only_the_emoji() {
        let k = test_key();
        let stored = stored_with(&k, |p| {
            p.name = "Written meanwhile".into();
            p.extra.insert("finance".into(), serde_json::json!({ "budgets": [1] }));
        });
        let pick = |emoji: Option<&str>| ProjectEdit {
            project_id: Uuid::nil(),
            name_description: None,
            status: None,
            history: None,
            emoji: Some(encrypt_json(&k, &emoji).unwrap()),
        };
        let back: ProjectEdit = serde_json::from_str(&serde_json::to_string(&pick(Some("🎿"))).unwrap()).unwrap();
        let after = payload_of(&k, &back.apply(&k, &stored).unwrap());
        assert_eq!(after.emoji().as_deref(), Some("🎿"));
        assert_eq!(after.name, "Written meanwhile");
        assert_eq!(after.extra["finance"], serde_json::json!({ "budgets": [1] }));

        let with_emoji = ProjectDto { payload: encrypt_json(&k, &after).unwrap(), ..stored };
        assert_eq!(payload_of(&k, &pick(None).apply(&k, &with_emoji).unwrap()).emoji(), None);
    }

    #[test]
    fn a_queue_written_before_the_emoji_still_reads() {
        let old = serde_json::json!({ "project_id": Uuid::nil(), "status": "closed" });
        let edit: ProjectEdit = serde_json::from_value(old).unwrap();
        assert_eq!(edit.emoji, None);
    }

    #[test]
    fn a_cleared_description_survives_the_queue_on_disk() {
        let edit = rename("Corse", None);
        let back: ProjectEdit = serde_json::from_str(&serde_json::to_string(&edit).unwrap()).unwrap();
        assert_eq!(back, edit);
        let k = test_key();
        let stored = stored_with(&k, |p| p.description = Some("old".into()));
        assert_eq!(payload_of(&k, &back.apply(&k, &stored).unwrap()).description, None);
    }

    #[test]
    fn a_replayed_status_goes_in_the_payload_and_clears_the_column() {
        let k = test_key();
        let close = ProjectEdit {
            project_id: Uuid::nil(),
            name_description: None,
            status: Some(ProjectStatus::Closed),
            history: None,
            emoji: None,
        };
        let mut legacy = stored_with(&k, |p| p.status = None);
        legacy.status = ProjectStatus::Archived;
        let written = close.apply(&k, &legacy).unwrap();
        assert_eq!(payload_of(&k, &written).status, Some(ProjectStatus::Closed));
        assert_eq!(written.status, Some(ProjectStatus::Ongoing));
        assert!(written.notify_closed);

        let renamed = rename("Corse", None).apply(&k, &legacy).unwrap();
        assert_eq!(renamed.status, None, "an edit without a status leaves the column to conversion");
        assert!(!renamed.notify_closed);
    }

    /// The history entry rides in the stored request with the id drawn when it was made: every
    /// replay sends that one, and the server leaves it out once taken.
    #[test]
    fn every_replay_of_a_project_edit_sends_the_history_id_drawn_when_it_was_queued() {
        let k = test_key();
        let history = crate::entity_ids::sealed_history(
            &k,
            shared::HistoryPayload { summary: "renamed".into(), actor_user_id: Some(1), action: None, entity: None, entity_id: None, extra: Default::default() },
        )
        .unwrap();
        let drawn = history.eid.unwrap();
        let edit = ProjectEdit { history: Some(history), ..rename("Corse", None) };
        let stored: ProjectEdit = serde_json::from_str(&serde_json::to_string(&edit).unwrap()).unwrap();
        let project = stored_with(&k, |_| {});
        for _ in 0..2 {
            let sent = stored.apply(&k, &project).unwrap().history.unwrap();
            assert_eq!(sent.eid, Some(drawn));
            let plain = crate::crypto::decrypt(&k, &sent.payload).unwrap();
            assert_eq!(shared::entity::read(&plain, shared::entity::Slot::History), Ok(Some(drawn)));
        }
    }

    #[test]
    fn a_queued_intent_round_trips_as_an_op() {
        let edit = rename("Corse", Some("Sous la tente"));
        let op = QueuedOp { id: Uuid::new_v4(), label: "Corse".into(), sealed_label: None, op: OpKind::EditProject(edit.clone()), after: None, queued_at: None };
        let stored = serde_json::to_string(&op).unwrap();
        let back: QueuedOp = serde_json::from_str(&stored).unwrap();
        assert!(matches!(back.op, OpKind::EditProject(e) if e == edit));
        assert!(!stored.contains("Sous la tente"), "the queue holds no plaintext description");
    }
}
