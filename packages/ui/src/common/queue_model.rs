//! I7: the model check of the write queue (SPEC §15.9.5), native, with no browser. Tabs each keep
//! their own view of storage; a write reaches storage in its tab's order and every other view after
//! a random delay, in any order across keys; a tab can die between any two storage calls, run
//! another tab's whole action between two of its own, or find its write refused. Clocks tie and step
//! back. A tab can die with its request at the server, which applies it later, after other sends. A
//! tab of a build before 3.4.0 keeps its queue in memory and writes it back whole under the old key;
//! each write of that key reaches every other living tab of 3.4.0 as a `storage` event carrying the
//! text written, in the order written, once that tab's view holds that write or a later one. The tab
//! keeps the entries of each text until they have their `o` key, and moves on each event, its view
//! of the key at that event's text or at storage's. A tab of 3.4.0 moves where the app does: at
//! start, on those events, in a handler before it writes, and in the drain. A page opened at an
//! index after another closed reads what that page wrote. The server keeps rows, and which expense
//! each row is. Every schedule is seeded; a failure prints its seed.
//!
//! The properties, each failure tagged with its letter:
//! - (a) every change made is landed, discarded, dropped with its project, erased, or still queued
//!   and explained; explained is judged here from storage and the server, not by the code's own
//!   statuses: listed by Q9's list, or behind a listed entry by Q4, or waiting on an explained one.
//!   The code's own list must be exactly the one judged here. One loss is excused, the one Q0 names:
//!   a change of the 3.3.x tab whose last write was replaced by a `[ ]`, or by an erase's removal
//!   of the old key, whose writer had not read it, no living 3.4.0 tab receiving that write's event,
//!   and the 3.3.x tab closing before it wrote again.
//! - (b) no entry leaves storage without a marker.
//! - (c) no send after the entry's `x` reached the sending tab.
//! - (d) every applied entry is marked landed, left for a logged reason, or ended by a `p` or `z`
//!   key as (a) accepts; every `l` marker matches
//!   a landing the server saw.
//! - (e) a move's delete never runs before its add landed.
//! - (f) the changes to one row first reach the server in the order they were made.
//! - (g) a moved expense ends in exactly one project.

use super::offline_queue::{Entry, OpKind, QueuedOp, Status};
use super::queue_store::{erase, forget, land, leave, move_old, next_to_send, receive_old_text, tidy, write_change, write_entries, Moved, Snapshot, Store, EMPTIED, OLD_KEY, PREFIX};
use serde_json::{json, Value};
use std::cell::{Cell, RefCell};
use std::collections::{BTreeMap, HashMap, HashSet};
use uuid::Uuid;

const DEV: &str = "dev";
const PROJECTS: [Uuid; 3] = [Uuid::from_u128(0xa1), Uuid::from_u128(0xa2), Uuid::from_u128(0xa3)];

struct Rng(u64);

impl Rng {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9e37_79b9_7f4a_7c15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
        z ^ (z >> 31)
    }

    fn below(&mut self, n: usize) -> usize {
        (self.next() % n.max(1) as u64) as usize
    }

    fn chance(&mut self, per_mille: u64) -> bool {
        self.next() % 1000 < per_mille
    }
}

/// A rule broken on purpose, to show the check catches what it is for.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Broken {
    None,
    /// (b): a landing removes the entry's keys without its `l` key.
    LandWithoutMarker,
    /// (e): the drain sends whatever is first, waiting on nothing named in `after`.
    IgnoreAfter,
    /// (a): Q4 as it stood before Option B, an add stopped behind a waiting delete and every
    /// stopped entry stopping the ones behind it.
    NoOptionB,
    /// (f): the drain sends the last entry it may send, not the first.
    Reverse,
    /// (g): a landed add takes the entries that wait on it with it.
    LandingTakesDependents,
    /// (f): the old key moves only inside the drain, which runs only online: an entry an older build
    /// queued offline is stamped after a change made since.
    MoveOnlyInDrain,
    /// (f): no reader sees a stamp, so an entry takes the `seq` of its `e` key, which a tab that
    /// missed the first move writes later.
    StampsUnread,
    /// (a): a `storage` event for the old key moves what the key holds, never the event's new value.
    MoveFromKeyOnly,
    /// (f): a tab stamps an old entry below every entry whose `e` key it has read.
    StampBelowRead,
}

#[derive(Clone, Copy, Debug)]
pub struct Config {
    pub tabs: usize,
    pub steps: usize,
    /// The Web Lock `counted-drain`; without it every tab sends.
    pub locks: bool,
    /// The phone: one process, one view, every change one step.
    pub one_view: bool,
    pub old_tab: bool,
    /// One project the server refuses for good.
    pub refusing: bool,
    /// Mostly moves between two projects, made inside one another's handlers.
    pub crossing: bool,
    /// Every change an edit of one row, so that edits of the row made in different tabs cross.
    pub one_row: bool,
    /// The 3.3.x tab closes often, and a page of 3.3.x opens in its place, reading the old key.
    pub old_reopens: bool,
    pub broken: Broken,
}

struct Tab {
    view: BTreeMap<String, String>,
    alive: bool,
    seen: i64,
    read_present: HashSet<String>,
    in_task: bool,
    old: Option<Vec<Value>>,
    refused: HashSet<Uuid>,
    clock_offset: i64,
    /// `storage` events for the old key that reached the page and wait for its next task: the
    /// 3.3.x tab's write each comes from, and the text written.
    inbox: Vec<(Option<usize>, Option<String>)>,
    kept: Option<String>,
    /// The page made its move at start (Q0).
    started: bool,
}

impl Tab {
    fn fresh(view: BTreeMap<String, String>, old: bool) -> Tab {
        let memory = old.then(|| view.get(OLD_KEY).and_then(|t| serde_json::from_str::<Vec<Value>>(t).ok()).unwrap_or_default());
        Tab { view, alive: true, seen: 0, read_present: HashSet::new(), in_task: false, old: memory, refused: HashSet::new(), clock_offset: 0, inbox: Vec::new(), kept: None, started: false }
    }
}

struct Row {
    project: Uuid,
    client: Option<Uuid>,
    alive: bool,
    expense: u64,
}

struct Made {
    id: Uuid,
    step: usize,
    project: Uuid,
    after: Option<Uuid>,
}

struct Edit {
    row: i32,
    order: usize,
    /// `None` for an edit of the 3.3.x tab, stamped only when moved.
    seq: Option<i64>,
}

struct Move {
    delete: Uuid,
    add: Uuid,
    expense: u64,
    source: Uuid,
    to: Uuid,
    step: usize,
}

/// A write on its way to storage. For the old key: the text the writing tab of 3.4.0 held there, or
/// the number of the 3.3.x tab's write.
struct Commit {
    tab: usize,
    key: String,
    value: Option<String>,
    read: Option<String>,
    old_write: Option<usize>,
}

enum Outcome {
    Landed(Option<i64>),
    Dropped,
    Retry,
    Refused,
}

struct World {
    rng: Rng,
    cfg: Config,
    quiet: bool,
    now: i64,
    step: usize,
    depth: usize,
    truth: BTreeMap<String, String>,
    commits: Vec<Commit>,
    deliveries: Vec<(usize, String)>,
    events: Vec<(usize, Option<usize>, Option<String>)>,
    tabs: Vec<Tab>,
    lock: Option<usize>,
    rows: BTreeMap<i32, Row>,
    applied: HashMap<Uuid, usize>,
    sent_by_new: HashMap<Uuid, usize>,
    sent_by_old: HashSet<Uuid>,
    made: Vec<Made>,
    left: HashMap<Uuid, usize>,
    forgets: Vec<(Uuid, usize)>,
    erases: Vec<usize>,
    stored: HashMap<Uuid, (i64, Option<Uuid>, usize)>,
    rolled_back: HashSet<Uuid>,
    in_flight: Vec<Entry>,
    late: HashSet<Uuid>,
    edits: HashMap<Uuid, Edit>,
    made_order: usize,
    row_log: HashMap<i32, Vec<Uuid>>,
    stale_pairs: HashSet<(Uuid, Uuid)>,
    late_pairs: HashSet<(Uuid, Uuid)>,
    stamp_pairs: HashSet<(Uuid, Uuid)>,
    unseen_at_send: HashMap<Uuid, (Vec<Uuid>, Vec<Uuid>)>,
    /// The smallest stamp any tab has written of each entry, reached storage or not.
    first_stamp: HashMap<Uuid, i64>,
    /// The `p` and `z` keys in the views of the tabs that wrote each stamp, at that write.
    stamp_covers: HashMap<(Uuid, i64), BTreeMap<String, String>>,
    /// Every entry whose `e` key a tab has written, reached storage or not.
    e_written: HashSet<Uuid>,
    /// `(entry, other, seq)`: a stamp of `entry` at `seq`, written after `other`'s `e` key by a tab
    /// whose view did not hold that key.
    unread_stamps: HashSet<(Uuid, Uuid, i64)>,
    /// The smallest stamp of an entry its sender's view held when it sent the request that first
    /// reached the server; for a request applied late, taken when it was sent.
    sender_stamp: HashMap<Uuid, i64>,
    stamp_at_send: HashMap<Uuid, i64>,
    /// Which excuses this schedule used: a stale sender, a stale sender of a request applied late,
    /// a sender without the first stamp, a stamp by a tab that had not read the other edit, and the
    /// loss Q0 names.
    uses: [bool; 5],
    moves: Vec<Move>,
    copies: HashMap<Uuid, u64>,
    deleted_expenses: HashSet<u64>,
    refused_ever: HashSet<Uuid>,
    online: bool,
    old_writes: HashMap<usize, HashSet<Uuid>>,
    old_in_storage: Option<usize>,
    old_last_write: Option<usize>,
    old_closings: HashSet<usize>,
    replaced_unread: HashSet<usize>,
    received_writes: HashSet<usize>,
    covers_seen: HashMap<Uuid, Vec<i64>>,
    violations: Vec<String>,
}

impl World {
    fn new(seed: u64, cfg: Config) -> World {
        let mut world = World {
            rng: Rng(seed),
            cfg,
            quiet: false,
            now: 1_000,
            step: 0,
            depth: 0,
            truth: BTreeMap::new(),
            commits: Vec::new(),
            deliveries: Vec::new(),
            events: Vec::new(),
            tabs: Vec::new(),
            lock: None,
            rows: BTreeMap::new(),
            applied: HashMap::new(),
            sent_by_new: HashMap::new(),
            sent_by_old: HashSet::new(),
            made: Vec::new(),
            left: HashMap::new(),
            forgets: Vec::new(),
            erases: Vec::new(),
            stored: HashMap::new(),
            rolled_back: HashSet::new(),
            in_flight: Vec::new(),
            late: HashSet::new(),
            edits: HashMap::new(),
            made_order: 0,
            row_log: HashMap::new(),
            stale_pairs: HashSet::new(),
            late_pairs: HashSet::new(),
            stamp_pairs: HashSet::new(),
            unseen_at_send: HashMap::new(),
            first_stamp: HashMap::new(),
            stamp_covers: HashMap::new(),
            e_written: HashSet::new(),
            unread_stamps: HashSet::new(),
            sender_stamp: HashMap::new(),
            stamp_at_send: HashMap::new(),
            uses: [false; 5],
            moves: Vec::new(),
            copies: HashMap::new(),
            deleted_expenses: HashSet::new(),
            refused_ever: HashSet::new(),
            online: true,
            old_writes: HashMap::new(),
            old_in_storage: None,
            old_last_write: None,
            old_closings: HashSet::new(),
            replaced_unread: HashSet::new(),
            received_writes: HashSet::new(),
            covers_seen: HashMap::new(),
            violations: Vec::new(),
        };
        world.truth.insert(DEV.into(), json!(PROJECTS).to_string());
        for (n, project) in PROJECTS.iter().cycle().take(9).enumerate() {
            world.rows.insert(n as i32 + 1, Row { project: *project, client: None, alive: true, expense: n as u64 + 1 });
        }
        for t in 0..cfg.tabs {
            let old = cfg.old_tab && t == cfg.tabs - 1;
            world.tabs.push(Tab::fresh(world.truth.clone(), old));
        }
        world
    }

    fn uuid(&mut self) -> Uuid {
        Uuid::from_u64_pair(self.rng.next(), self.rng.next())
    }

    fn write(&mut self, t: usize, key: &str, value: Option<String>) {
        if self.cfg.one_view {
            self.apply_truth(key, value.clone());
            for tab in &mut self.tabs {
                match &value {
                    Some(v) => tab.view.insert(key.to_string(), v.clone()),
                    None => tab.view.remove(key),
                };
            }
            return;
        }
        if let (Some((id, seq)), Some(_)) = (key.strip_prefix(&format!("{PREFIX}t.")).and_then(|rest| rest.split_once('.')), &value) {
            if let (Ok(id), Ok(seq)) = (Uuid::parse_str(id), seq.parse::<i64>()) {
                let at = self.first_stamp.entry(id).or_insert(seq);
                *at = (*at).min(seq);
                let view = &self.tabs[t].view;
                let covers = view.iter().filter(|(k, _)| k.starts_with(&format!("{PREFIX}p.")) || k.starts_with(&format!("{PREFIX}z.")));
                self.stamp_covers.entry((id, seq)).or_default().extend(covers.map(|(k, v)| (k.clone(), v.clone())));
                let unread: Vec<Uuid> = self.e_written.iter().filter(|other| !view.contains_key(&format!("{PREFIX}e.{other}"))).copied().collect();
                self.unread_stamps.extend(unread.into_iter().map(|other| (id, other, seq)));
            }
        }
        if let (Some(Ok(id)), Some(_)) = (key.strip_prefix(&format!("{PREFIX}e.")).map(Uuid::parse_str), &value) {
            self.e_written.insert(id);
        }
        let (mut read, mut old_write) = (None, None);
        if key == OLD_KEY {
            match self.tabs[t].old.is_some() {
                true => {
                    let n = self.old_last_write.map_or(1, |n| n + 1);
                    self.old_last_write = Some(n);
                    old_write = Some(n);
                }
                false => read = self.tabs[t].view.get(OLD_KEY).cloned(),
            }
        }
        match &value {
            Some(v) => self.tabs[t].view.insert(key.to_string(), v.clone()),
            None => self.tabs[t].view.remove(key),
        };
        self.commits.push(Commit { tab: t, key: key.to_string(), value, read, old_write });
    }

    /// A write reaches storage, and every view after a delay; a write of the old key also reaches
    /// every other living tab of 3.4.0 as a `storage` event carrying its text. Records a `[ ]`, or an
    /// erase's removal of the key, that replaces a write of the 3.3.x tab its writer had not read,
    /// for the one excused loss of (a).
    fn apply_commit(&mut self, at: usize) {
        let c = self.commits.remove(at);
        if c.key == OLD_KEY {
            if let (Some(n), None) = (self.old_in_storage, c.old_write) {
                let empties = c.value.as_deref() == Some(EMPTIED) || c.value.is_none();
                if empties && c.read != self.truth.get(OLD_KEY).cloned() {
                    self.replaced_unread.insert(n);
                }
            }
            self.old_in_storage = c.old_write;
            if let (Some(n), Some(text)) = (c.old_write, &c.value) {
                let ids = serde_json::from_str::<Vec<Value>>(text).unwrap_or_default().iter().filter_map(|e| e["id"].as_str().and_then(|s| Uuid::parse_str(s).ok())).collect();
                self.old_writes.insert(n, ids);
            }
            for u in 0..self.tabs.len() {
                if u != c.tab && self.tabs[u].alive && self.tabs[u].old.is_none() {
                    self.events.push((u, c.old_write, c.value.clone()));
                }
            }
        }
        self.apply_truth(&c.key, c.value);
        for u in 0..self.tabs.len() {
            self.deliveries.push((u, c.key.clone()));
        }
    }

    fn apply_truth(&mut self, key: &str, value: Option<String>) {
        if let (Some(rest), Some(v)) = (key.strip_prefix(&format!("{PREFIX}e.")), &value) {
            if let Ok(id) = Uuid::parse_str(rest) {
                let entry = Entry::read(serde_json::from_str(v).unwrap_or(Value::Null));
                let step = self.step;
                self.stored.entry(id).or_insert((entry.seq, entry.project, step));
            }
        }
        match value {
            Some(v) => self.truth.insert(key.to_string(), v),
            None => self.truth.remove(key),
        };
    }

    /// One event on its way: the oldest pending write of a tab reaches storage, or storage reaches
    /// a view, or the server applies a request a dead tab left at it. A view in the middle of a
    /// synchronous task takes nothing.
    fn deliver_one(&mut self) -> bool {
        if !self.in_flight.is_empty() && self.rng.chance(100) {
            let at = self.rng.below(self.in_flight.len());
            let entry = self.in_flight.remove(at);
            self.apply(&entry, None);
            return true;
        }
        let total = self.commits.len() + self.deliveries.len() + self.events.len();
        if total == 0 {
            return false;
        }
        let pick = self.rng.below(total);
        if pick < self.commits.len() {
            let t = self.commits[pick].tab;
            let first = self.commits.iter().position(|c| c.tab == t).unwrap_or(pick);
            self.apply_commit(first);
            return true;
        }
        if pick >= self.commits.len() + self.deliveries.len() {
            let u = self.events[pick - self.commits.len() - self.deliveries.len()].0;
            if self.tabs[u].in_task {
                return false;
            }
            let at = self.events.iter().position(|e| e.0 == u).unwrap_or_default();
            let (_, write, text) = self.events.remove(at);
            self.tabs[u].inbox.push((write, text));
            return true;
        }
        let at = pick - self.commits.len();
        let (u, key) = self.deliveries[at].clone();
        if self.tabs[u].in_task {
            return false;
        }
        self.deliveries.remove(at);
        match self.truth.get(&key).cloned() {
            Some(v) => self.tabs[u].view.insert(key, v),
            None => self.tabs[u].view.remove(&key),
        };
        true
    }

    fn settle(&mut self) {
        for tab in &mut self.tabs {
            tab.in_task = false;
        }
        while let Some(entry) = self.in_flight.pop() {
            self.apply(&entry, None);
        }
        let mut guard = 0;
        while (!self.commits.is_empty() || !self.deliveries.is_empty() || !self.events.is_empty()) && guard < 1_000_000 {
            self.deliver_one();
            guard += 1;
        }
    }

    fn held(&self, t: usize) -> Vec<Uuid> {
        self.tabs[t].view.get(DEV).and_then(|v| serde_json::from_str(v).ok()).unwrap_or_default()
    }

    fn fail(&mut self, what: String) {
        if self.violations.len() < 20 {
            self.violations.push(format!("step {}: {what}", self.step));
        }
    }

    /// A request through the network: lost before the server, applied, or applied with its answer
    /// lost.
    fn send(&mut self, t: usize, entry: &Entry) -> Outcome {
        let Some(queued) = entry.op.clone() else { return Outcome::Retry };
        if !self.quiet && self.rng.chance(60) {
            return Outcome::Retry;
        }
        if self.cfg.refusing && queued.op.project_id() == PROJECTS[2] {
            return Outcome::Refused;
        }
        match self.tabs[t].old.is_none() {
            true => {
                self.sent_by_new.entry(queued.id).or_insert(self.step);
            }
            false => {
                self.sent_by_old.insert(queued.id);
            }
        }
        let outcome = self.apply(entry, Some(t));
        if !self.quiet && self.rng.chance(40) {
            return Outcome::Retry;
        }
        outcome
    }

    /// The server, with per-row state. A move's delete applied before its add is the loss the
    /// queue exists to prevent. `from` is the sending tab, `None` for a request applied late.
    fn apply(&mut self, entry: &Entry, from: Option<usize>) -> Outcome {
        let Some(queued) = entry.op.clone() else { return Outcome::Retry };
        let id = queued.id;
        let project = queued.op.project_id();
        if from.is_none() {
            self.late.insert(id);
        }
        let outcome = match &queued.op {
            OpKind::AddExpense(_) => {
                let found = self.rows.iter().find(|(_, r)| r.alive && r.project == project && r.client == Some(id)).map(|(n, _)| *n);
                let row = match found {
                    Some(n) => n,
                    None => {
                        let n = self.rows.keys().next_back().copied().unwrap_or(0) + 1;
                        let expense = self.copies.get(&id).copied().unwrap_or(1_000 + n as u64);
                        self.rows.insert(n, Row { project, client: Some(id), alive: true, expense });
                        n
                    }
                };
                Outcome::Landed(Some(i64::from(row)))
            }
            OpKind::EditExpense(e) => match self.rows.get(&e.id) {
                Some(r) if r.alive && r.project == project => {
                    if !self.applied.contains_key(&id) {
                        self.edit_reached(e.id, id, from.map(|t| (t, entry.seq)));
                    }
                    Outcome::Landed(None)
                }
                _ => Outcome::Dropped,
            },
            OpKind::DeleteExpense(d) => {
                if let Some(add) = entry.after.filter(|a| !self.applied.contains_key(a)) {
                    self.fail(format!("(e) the delete {id} of a move ran before its add {add} landed"));
                }
                if let Some(r) = self.rows.get_mut(&d.id).filter(|r| r.alive && r.project == project) {
                    r.alive = false;
                    let expense = r.expense;
                    if entry.after.is_none() {
                        self.deleted_expenses.insert(expense);
                    }
                }
                Outcome::Landed(None)
            }
            _ => Outcome::Landed(None),
        };
        if matches!(outcome, Outcome::Landed(_)) {
            *self.applied.entry(id).or_insert(0) += 1;
        }
        outcome
    }

    /// (f): an edit first reaches the server; any edit of that row made earlier that has not, and
    /// that the sender's view held, will reach it later out of order. Pairs the sender could not see
    /// are recorded as stale, which (f) accepts; for a request applied late, as its tab saw them when
    /// it sent it.
    fn edit_reached(&mut self, row: i32, id: Uuid, from: Option<(usize, i64)>) {
        self.row_log.entry(row).or_default().push(id);
        match from {
            Some((t, seq)) => {
                let (unseen, unstamped) = self.unseen_at(t, id, seq);
                self.stale_pairs.extend(unseen.into_iter().map(|other| (id, other)));
                self.stamp_pairs.extend(unstamped.into_iter().map(|other| (id, other)));
                if let Some(stamp) = self.stamp_read(t, id) {
                    self.sender_stamp.insert(id, stamp);
                }
            }
            None => {
                let (unseen, unstamped) = self.unseen_at_send.remove(&id).unwrap_or_default();
                self.late_pairs.extend(unseen.into_iter().map(|other| (id, other)));
                self.stamp_pairs.extend(unstamped.into_iter().map(|other| (id, other)));
                if let Some(stamp) = self.stamp_at_send.remove(&id) {
                    self.sender_stamp.insert(id, stamp);
                }
            }
        }
    }

    /// The smallest stamp of `id` tab `t`'s view holds.
    fn stamp_read(&self, t: usize, id: Uuid) -> Option<i64> {
        let stamps = format!("{PREFIX}t.{id}.");
        self.tabs[t].view.keys().filter_map(|k| k.strip_prefix(&stamps)).filter_map(|s| s.parse::<i64>().ok()).min()
    }

    /// The earlier edits of `id`'s row not applied yet that tab `t`'s view does not hold when it
    /// sends `id` ordered at `seq`; then the moved entries of the 3.3.x tab it holds with no stamp
    /// below `seq` though a tab had written one, ordered by the stamps it has (Q0). For a request
    /// applied late, taken when its tab sent it.
    fn unseen_at(&self, t: usize, id: Uuid, seq: i64) -> (Vec<Uuid>, Vec<Uuid>) {
        let Some(edit) = self.edits.get(&id) else { return Default::default() };
        let view = &self.tabs[t].view;
        let older = self.edits.iter().filter(|(other, e)| e.row == edit.row && e.order < edit.order && !self.applied.contains_key(*other));
        let (held, unseen): (Vec<_>, Vec<_>) = older.partition(|(other, _)| view.contains_key(&format!("{PREFIX}e.{other}")));
        let stamped_below = |other: &Uuid| {
            let stamps = format!("{PREFIX}t.{other}.");
            view.keys().filter_map(|k| k.strip_prefix(&stamps)).filter_map(|s| s.parse::<i64>().ok()).any(|at| at < seq)
        };
        let written_below = |other: &Uuid| self.first_stamp.get(other).is_some_and(|at| *at < seq);
        let unstamped = held.into_iter().filter(|(other, e)| e.seq.is_none() && written_below(other) && !stamped_below(other)).map(|(other, _)| *other).collect();
        (unseen.into_iter().map(|(other, _)| *other).collect(), unstamped)
    }
}

struct TabStore<'a> {
    world: &'a RefCell<World>,
    tab: usize,
}

/// A storage call is where another process gets its turn: writes reach storage and views, another
/// tab runs a whole action, or this tab dies.
fn interleave(cell: &RefCell<World>, t: usize) {
    let (events, crash, nested) = {
        let mut w = cell.borrow_mut();
        if w.quiet {
            return;
        }
        let events = w.rng.below(4);
        let crash = w.tabs[t].old.is_none() && w.rng.chance(4);
        let nested_chance = if w.cfg.crossing { 200 } else { 60 };
        let nested = w.depth == 0 && !(w.cfg.one_view && w.tabs[t].in_task) && w.rng.chance(nested_chance);
        (events, crash, nested)
    };
    for _ in 0..events {
        cell.borrow_mut().deliver_one();
        handle_events(cell);
    }
    if crash {
        die(&mut cell.borrow_mut(), t);
    }
    if nested {
        let other = {
            let mut w = cell.borrow_mut();
            let others: Vec<usize> = (0..w.tabs.len()).filter(|&u| u != t && w.tabs[u].alive && !w.tabs[u].in_task).collect();
            (!others.is_empty()).then(|| others[w.rng.below(others.len())])
        };
        if let Some(u) = other {
            cell.borrow_mut().depth += 1;
            action(cell, u);
            cell.borrow_mut().depth -= 1;
        }
    }
}

fn die(w: &mut World, t: usize) {
    let dying: Vec<usize> = if w.cfg.one_view { (0..w.tabs.len()).collect() } else { vec![t] };
    for u in dying {
        w.tabs[u].alive = false;
        if w.lock == Some(u) {
            w.lock = None;
        }
    }
}

/// Each tab of 3.4.0 not in a task takes the `storage` events for the old key that reached it, one
/// at a time as the app does: it keeps the entries of the event's text, then moves in a task of its
/// own (Q0). An event is dispatched once the tab's storage holds its write or a later one, so the
/// view of the old key is brought to that event's text or to storage, either one, unless the tab's
/// own write of it is still on its way.
fn handle_events(cell: &RefCell<World>) {
    loop {
        let next = {
            let w = cell.borrow();
            if w.depth > 1 {
                return;
            }
            (0..w.tabs.len()).find(|&u| w.tabs[u].alive && !w.tabs[u].in_task && !w.tabs[u].inbox.is_empty())
        };
        let Some(u) = next else { return };
        let (broken, text) = {
            let mut w = cell.borrow_mut();
            let (write, text) = w.tabs[u].inbox.remove(0);
            if let Some(n) = write {
                w.received_writes.insert(n);
            }
            if !w.commits.iter().any(|c| c.tab == u && c.key == OLD_KEY) {
                let shown = if w.rng.chance(500) { text.clone() } else { w.truth.get(OLD_KEY).cloned() };
                match shown {
                    Some(v) => w.tabs[u].view.insert(OLD_KEY.to_string(), v),
                    None => w.tabs[u].view.remove(OLD_KEY),
                };
            }
            (w.cfg.broken, text)
        };
        if broken != Broken::MoveFromKeyOnly {
            receive_old_text(&mut TabStore { world: cell, tab: u }, text);
        }
        if broken != Broken::MoveOnlyInDrain {
            move_task(cell, u);
        }
    }
}

/// A move as a task of its own, so what it reads does not count as read by an operation of the tab
/// that awaits meanwhile (I2).
fn move_task(cell: &RefCell<World>, u: usize) {
    let outer_reads = {
        let mut w = cell.borrow_mut();
        w.tabs[u].in_task = true;
        w.depth += 1;
        std::mem::take(&mut w.tabs[u].read_present)
    };
    let mut store = TabStore { world: cell, tab: u };
    let snap = Snapshot::read(&mut store);
    move_old(&mut store, &snap);
    let mut w = cell.borrow_mut();
    w.depth -= 1;
    w.tabs[u].in_task = false;
    w.tabs[u].read_present = outer_reads;
}

/// Q0 at start: each page of 3.4.0 opened since the last step moves what it sees, online or not and
/// under no lock.
fn start_pages(cell: &RefCell<World>) {
    let starting: Vec<usize> = {
        let mut w = cell.borrow_mut();
        let starting = (0..w.tabs.len()).filter(|&u| w.tabs[u].alive && w.tabs[u].old.is_none() && !w.tabs[u].started).collect::<Vec<_>>();
        starting.iter().for_each(|&u| w.tabs[u].started = true);
        starting
    };
    if cell.borrow().cfg.broken == Broken::MoveOnlyInDrain {
        return;
    }
    for u in starting {
        if cell.borrow().tabs[u].alive {
            move_task(cell, u);
        }
    }
}

/// An `await`: this tab's view takes what reached storage meanwhile.
fn await_point(cell: &RefCell<World>, t: usize) {
    cell.borrow_mut().tabs[t].in_task = false;
    interleave(cell, t);
    let extra = cell.borrow_mut().rng.below(6);
    for _ in 0..extra {
        cell.borrow_mut().deliver_one();
        handle_events(cell);
    }
    cell.borrow_mut().tabs[t].in_task = true;
}

impl Store for TabStore<'_> {
    fn get(&self, key: &str) -> Option<String> {
        interleave(self.world, self.tab);
        let mut w = self.world.borrow_mut();
        let tab = &mut w.tabs[self.tab];
        let value = tab.view.get(key).cloned();
        match value.is_some() {
            true => tab.read_present.insert(key.to_string()),
            false => tab.read_present.remove(key),
        };
        value
    }

    fn set(&mut self, key: &str, value: &str) -> bool {
        interleave(self.world, self.tab);
        let mut w = self.world.borrow_mut();
        if !w.tabs[self.tab].alive {
            return true;
        }
        let exempt = !key.starts_with(PREFIX) || key.starts_with(&format!("{PREFIX}o."));
        if !exempt && w.tabs[self.tab].read_present.contains(key) {
            let t = self.tab;
            w.fail(format!("I2: tab {t} wrote {key} again after reading it present"));
        }
        if key.starts_with(PREFIX) && !w.quiet && w.rng.chance(15) {
            return false;
        }
        if w.cfg.broken == Broken::LandWithoutMarker && key.starts_with(&format!("{PREFIX}l.")) {
            return true;
        }
        let below_read = match key.strip_prefix(&format!("{PREFIX}t.")).and_then(|rest| rest.split_once('.')) {
            Some((id, _)) if w.cfg.broken == Broken::StampBelowRead => w.tabs[self.tab]
                .view
                .iter()
                .filter(|(k, _)| k.starts_with(&format!("{PREFIX}e.")))
                .filter_map(|(_, v)| serde_json::from_str::<Value>(v).ok()?["seq"].as_i64())
                .min()
                .map(|seq| format!("{PREFIX}t.{id}.{}", seq - 1)),
            _ => None,
        };
        w.write(self.tab, below_read.as_deref().unwrap_or(key), Some(value.to_string()));
        true
    }

    fn remove(&mut self, key: &str) {
        interleave(self.world, self.tab);
        let mut w = self.world.borrow_mut();
        if w.tabs[self.tab].alive {
            w.write(self.tab, key, None);
        }
    }

    fn all(&self) -> Vec<(String, String)> {
        interleave(self.world, self.tab);
        let mut w = self.world.borrow_mut();
        let stamps_unread = w.cfg.broken == Broken::StampsUnread;
        let tab = &mut w.tabs[self.tab];
        let all: Vec<(String, String)> = tab
            .view
            .iter()
            .filter(|(k, _)| k.starts_with(PREFIX) && !(stamps_unread && k.starts_with(&format!("{PREFIX}t."))))
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();
        let view = &tab.view;
        tab.read_present.retain(|k| view.contains_key(k));
        tab.read_present.extend(all.iter().map(|(k, _)| k.clone()));
        all
    }

    /// Each tab's clock: tied with another tab's now and then, and stepped back now and then.
    fn now_ms(&self) -> i64 {
        let mut w = self.world.borrow_mut();
        if w.quiet || !w.rng.chance(300) {
            w.now += 1;
        }
        if !w.quiet && w.rng.chance(10) {
            let back = 1 + w.rng.below(5) as i64;
            w.tabs[self.tab].clock_offset -= back;
        }
        w.now + w.tabs[self.tab].clock_offset
    }

    fn seen_seq(&self) -> i64 {
        self.world.borrow().tabs[self.tab].seen
    }

    fn saw_seq(&mut self, seq: i64) {
        let mut w = self.world.borrow_mut();
        let tab = &mut w.tabs[self.tab];
        tab.seen = tab.seen.max(seq);
    }

    fn kept(&self) -> Option<String> {
        self.world.borrow().tabs[self.tab].kept.clone()
    }

    fn set_kept(&mut self, text: Option<String>) {
        self.world.borrow_mut().tabs[self.tab].kept = text;
    }
}

fn pair() -> shared::EncryptedPair {
    shared::EncryptedPair { ct: "ct".into(), iv: "iv".into() }
}

fn raw(id: Uuid, op: OpKind, after: Option<Uuid>) -> Value {
    serde_json::to_value(QueuedOp { id, label: String::new(), sealed_label: None, op, after, queued_at: Some(1) }).unwrap()
}

fn add_op(id: Uuid, project: Uuid) -> Value {
    let add = shared::CreatableExpense { project_id: project, author_id: None, payload: pair(), payers: vec![], debtors: vec![], history: None, client_op_id: Some(id) };
    raw(id, OpKind::AddExpense(add), None)
}

fn edit_op(id: Uuid, project: Uuid, row: i32) -> Value {
    let edit = shared::EditableExpense { id: row, project_id: project, author_id: None, payload: pair(), payers: vec![], debtors: vec![], history: None };
    raw(id, OpKind::EditExpense(edit), None)
}

fn delete_op(id: Uuid, project: Uuid, row: i32, after: Option<Uuid>) -> Value {
    raw(id, OpKind::DeleteExpense(shared::DeleteExpenseRequest { id: row, project_id: project, history: None }), after)
}

/// What the user does in a tab: a list of entries the handler writes, those naming others first,
/// and the move it makes, if any.
type Changes = (Vec<(Value, Uuid, Option<Uuid>)>, Option<Move>);

fn changes(cell: &RefCell<World>, t: usize) -> Changes {
    let mut w = cell.borrow_mut();
    let mut held = w.held(t);
    if w.cfg.crossing {
        held.retain(|p| PROJECTS[..2].contains(p));
    }
    if held.is_empty() {
        return (Vec::new(), None);
    }
    if w.cfg.one_row && held.contains(&PROJECTS[0]) {
        let id = w.uuid();
        return (vec![(edit_op(id, PROJECTS[0], 1), PROJECTS[0], None)], None);
    }
    let project = held[w.rng.below(held.len())];
    let rows: Vec<i32> = w.rows.iter().filter(|(_, r)| r.project == project).map(|(n, _)| *n).collect();
    let id = w.uuid();
    let pick = if w.cfg.crossing { 7 + w.rng.below(3) } else { w.rng.below(10) };
    match pick {
        0..=3 => (vec![(add_op(id, project), project, None)], None),
        4..=5 if !rows.is_empty() => {
            let row = rows[w.rng.below(rows.len())];
            (vec![(edit_op(id, project, row), project, None)], None)
        }
        6 if !rows.is_empty() => {
            let row = rows[w.rng.below(rows.len())];
            (vec![(delete_op(id, project, row, None), project, None)], None)
        }
        _ => {
            let others: Vec<Uuid> = held.iter().copied().filter(|p| *p != project).collect();
            let alive: Vec<i32> = w.rows.iter().filter(|(_, r)| r.project == project && r.alive).map(|(n, _)| *n).collect();
            if others.is_empty() || alive.is_empty() {
                return (vec![(add_op(id, project), project, None)], None);
            }
            let to = others[w.rng.below(others.len())];
            let row = alive[w.rng.below(alive.len())];
            let add = w.uuid();
            let expense = w.rows[&row].expense;
            let step = w.step;
            let entries = vec![(delete_op(id, project, row, Some(add)), project, Some(add)), (add_op(add, to), to, None)];
            (entries, Some(Move { delete: id, add, expense, source: project, to, step }))
        }
    }
}

fn action(cell: &RefCell<World>, t: usize) {
    if !cell.borrow().tabs[t].alive {
        return;
    }
    if cell.borrow().tabs[t].old.is_some() {
        return old_action(cell, t);
    }
    let pick = cell.borrow_mut().rng.below(100);
    cell.borrow_mut().tabs[t].in_task = true;
    let mut store = TabStore { world: cell, tab: t };
    let broken = cell.borrow().cfg.broken;
    match pick {
        0..=39 => {
            let (made, moved) = changes(cell, t);
            if made.is_empty() {
                cell.borrow_mut().tabs[t].in_task = false;
                return;
            }
            let held = {
                let device: Vec<Uuid> = store.get(DEV).and_then(|v| serde_json::from_str(&v).ok()).unwrap_or_default();
                made.iter().all(|(_, p, _)| device.contains(p))
            };
            let raws: Vec<Value> = made.iter().map(|(r, _, _)| r.clone()).collect();
            let ok = held
                && match broken {
                    Broken::MoveOnlyInDrain => write_entries(&mut store, &raws),
                    _ => write_change(&mut store, &raws),
                };
            let mut w = cell.borrow_mut();
            let step = w.step;
            for (r, project, after) in &made {
                let id = Uuid::parse_str(r["id"].as_str().unwrap_or_default()).unwrap_or_default();
                if ok && w.tabs[t].alive {
                    w.made.push(Made { id, step, project: *project, after: *after });
                    let covers = covers_in_view(&w.tabs[t].view, *project);
                    w.covers_seen.insert(id, covers);
                    w.made_order += 1;
                    let order = w.made_order;
                    if let Some(row) = r["op"]["type"].as_str().filter(|k| *k == "edit_expense").and_then(|_| r["op"]["id"].as_i64()) {
                        let seq = w.tabs[t].view.get(&format!("{PREFIX}e.{id}")).and_then(|v| serde_json::from_str::<Value>(v).ok()).and_then(|v| v["seq"].as_i64()).unwrap_or(0);
                        w.edits.insert(id, Edit { row: row as i32, order, seq: Some(seq) });
                        unseen_older_edits(&mut w, t, id, row as i32, order);
                    }
                } else if !ok {
                    w.rolled_back.insert(id);
                }
            }
            if let Some(moved) = moved.filter(|_| ok && w.tabs[t].alive) {
                w.copies.insert(moved.add, moved.expense);
                w.moves.push(moved);
            }
        }
        40..=47 => {
            let snap = Snapshot::read(&mut store);
            let queue = snap.queue();
            if !queue.is_empty() {
                let entry = queue[cell.borrow_mut().rng.below(queue.len())].clone();
                let step = cell.borrow().step;
                cell.borrow_mut().left.insert(entry.id.unwrap_or_default(), step);
                leave(&mut store, &snap, &entry);
            }
        }
        48..=49 => {
            let held = cell.borrow().held(t);
            if !held.is_empty() {
                let project = held[cell.borrow_mut().rng.below(held.len())];
                let step = cell.borrow().step;
                cell.borrow_mut().forgets.push((project, step));
                let kept: Vec<Uuid> = held.into_iter().filter(|p| *p != project).collect();
                store.set(DEV, &json!(kept).to_string());
                let snap = Snapshot::read(&mut store);
                forget(&mut store, &snap, project);
            }
        }
        50..=52 => {
            let mut held = cell.borrow().held(t);
            let missing: Vec<Uuid> = PROJECTS.iter().copied().filter(|p| !held.contains(p)).collect();
            if !missing.is_empty() {
                held.push(missing[cell.borrow_mut().rng.below(missing.len())]);
                store.set(DEV, &json!(held).to_string());
            }
        }
        53 if cell.borrow_mut().rng.chance(300) => {
            let step = cell.borrow().step;
            cell.borrow_mut().erases.push(step);
            store.set(DEV, "[]");
            erase(&mut store);
        }
        _ => drain(cell, t, 8),
    }
    cell.borrow_mut().tabs[t].in_task = false;
}

/// Q4 as it stood at 07ab6bf0, for `Broken::NoOptionB`.
fn next_before_option_b(queue: &[Entry], mut blocked: impl FnMut(&Entry) -> bool) -> Option<usize> {
    let mut stops: Vec<(Uuid, Option<i64>)> = Vec::new();
    for (position, entry) in queue.iter().enumerate() {
        let project = entry.project?;
        let behind = stops.iter().any(|&(p, row)| p == project && !(row.is_some() && entry.row().is_some() && row != entry.row()));
        if entry.status == Status::Ready && !behind && !blocked(entry) {
            return Some(position);
        }
        stops.push((project, entry.row()));
    }
    None
}

/// The replay of SPEC §15.9.5 Q2e, step for step as `queue_replay::drain` takes them.
fn drain(cell: &RefCell<World>, t: usize, budget: usize) {
    {
        let mut w = cell.borrow_mut();
        if !w.online {
            return;
        }
        if w.cfg.locks {
            if w.lock.is_some() {
                return;
            }
            w.lock = Some(t);
        }
    }
    await_point(cell, t);
    let mut store = TabStore { world: cell, tab: t };
    for _ in 0..budget {
        if !cell.borrow().tabs[t].alive {
            break;
        }
        let snap = Snapshot::read(&mut store);
        let snap = match move_old(&mut store, &snap) {
            Moved::Nothing => snap,
            _ => Snapshot::read(&mut store),
        };
        tidy(&mut store, &snap);
        let held = cell.borrow().held(t);
        let queue = snap.queue();
        let blocked = |e: &Entry| cell.borrow().tabs[t].refused.contains(&e.id.unwrap_or_default()) || !e.project.is_some_and(|p| held.contains(&p));
        let next = match cell.borrow().cfg.broken {
            Broken::IgnoreAfter => queue.iter().position(|e| e.op.is_some() && !blocked(e)),
            Broken::NoOptionB => next_before_option_b(&queue, blocked),
            Broken::Reverse => queue.iter().rposition(|e| e.status == Status::Ready && e.project.is_some() && !blocked(e)),
            _ => next_to_send(&queue, blocked),
        };
        let Some(i) = next else { break };
        let entry = queue[i].clone();
        let id = entry.id.unwrap_or_default();
        {
            let mut w = cell.borrow_mut();
            if !w.tabs[t].alive {
                break;
            }
            let left = format!("{PREFIX}x.{id}.");
            if w.tabs[t].view.keys().any(|k| k.starts_with(&left)) {
                w.fail(format!("(c) the discarded entry {id} was sent by tab {t} after its x key reached it"));
            }
            if !w.quiet && w.rng.chance(15) {
                let step = w.step;
                w.sent_by_new.entry(id).or_insert(step);
                let unseen = w.unseen_at(t, id, entry.seq);
                w.unseen_at_send.insert(id, unseen);
                if let Some(stamp) = w.stamp_read(t, id) {
                    w.stamp_at_send.insert(id, stamp);
                }
                w.in_flight.push(entry.clone());
                die(&mut w, t);
                break;
            }
        }
        let outcome = cell.borrow_mut().send(t, &entry);
        await_point(cell, t);
        if !cell.borrow().tabs[t].alive {
            break;
        }
        match outcome {
            Outcome::Landed(row) => {
                land(&mut store, &snap, &entry, row);
                if cell.borrow().cfg.broken == Broken::LandingTakesDependents {
                    let snap = Snapshot::read(&mut store);
                    for dependent in snap.queue().into_iter().filter(|e| e.after == Some(id)) {
                        leave(&mut store, &snap, &dependent);
                    }
                }
            }
            Outcome::Dropped => {
                let step = cell.borrow().step;
                cell.borrow_mut().left.insert(id, step);
                leave(&mut store, &snap, &entry);
            }
            Outcome::Retry => break,
            Outcome::Refused => {
                cell.borrow_mut().tabs[t].refused.insert(id);
                cell.borrow_mut().refused_ever.insert(id);
            }
        }
        await_point(cell, t);
    }
    let mut w = cell.borrow_mut();
    if w.lock == Some(t) {
        w.lock = None;
    }
}

/// A tab of a build before 3.4.0: its queue in memory, written back whole under the old key, sent
/// first to last with no lock, `after` used only to drop dependents with a 404.
fn old_action(cell: &RefCell<World>, t: usize) {
    let mut store = TabStore { world: cell, tab: t };
    let replay = cell.borrow_mut().rng.chance(400);
    if !replay {
        let (made, moved) = changes(cell, t);
        let mut w = cell.borrow_mut();
        let step = w.step;
        for (r, project, after) in made.into_iter().rev() {
            let id = Uuid::parse_str(r["id"].as_str().unwrap_or_default()).unwrap_or_default();
            w.made.push(Made { id, step, project, after });
            if let Some(row) = r["op"]["type"].as_str().filter(|k| *k == "edit_expense").and_then(|_| r["op"]["id"].as_i64()) {
                w.made_order += 1;
                let order = w.made_order;
                w.edits.insert(id, Edit { row: row as i32, order, seq: None });
            }
            if let Some(memory) = w.tabs[t].old.as_mut() {
                memory.push(r);
            }
        }
        if let Some(moved) = moved {
            w.copies.insert(moved.add, moved.expense);
            w.moves.push(moved);
        }
    } else if !old_replay_one(cell, t) {
        return;
    }
    let memory = cell.borrow().tabs[t].old.clone().unwrap_or_default();
    store.set(OLD_KEY, &Value::Array(memory).to_string());
    let mut w = cell.borrow_mut();
    let closes = if w.cfg.old_reopens { 200 } else { 50 };
    if !w.quiet && w.rng.chance(closes) {
        w.tabs[t].alive = false;
        if let Some(last) = w.old_last_write {
            w.old_closings.insert(last);
        }
    }
}

/// Sends the first entry of an old tab's memory, a 404 dropping it with what names it in `after`.
/// False when it stays.
fn old_replay_one(cell: &RefCell<World>, t: usize) -> bool {
    let Some(first) = cell.borrow().tabs[t].old.as_ref().and_then(|m| m.first().cloned()) else { return false };
    let entry = Entry::read(first);
    let outcome = cell.borrow_mut().send(t, &entry);
    let mut w = cell.borrow_mut();
    let step = w.step;
    let memory = w.tabs[t].old.take().unwrap_or_default();
    let (kept, progress) = match outcome {
        Outcome::Landed(_) => (memory[1..].to_vec(), true),
        Outcome::Dropped => {
            let (dropped, kept): (Vec<Value>, Vec<Value>) = memory.into_iter().partition(|e| {
                let read = Entry::read(e.clone());
                read.id == entry.id || read.after == entry.id
            });
            for gone in dropped.into_iter().filter_map(|d| Entry::read(d).id) {
                w.left.insert(gone, step);
            }
            (kept, true)
        }
        _ => (memory, false),
    };
    w.tabs[t].old = Some(kept);
    progress
}

/// A dead tab is replaced by a fresh one reading storage as it is, once the writes the dead page made
/// reached storage.
fn respawn(w: &mut World) {
    for t in 0..w.tabs.len() {
        if !w.tabs[t].alive {
            while let Some(at) = w.commits.iter().position(|c| c.tab == t) {
                w.apply_commit(at);
            }
            w.events.retain(|e| e.0 != t);
            let old = w.cfg.old_reopens && w.cfg.old_tab && t == w.tabs.len() - 1;
            let fresh = Tab::fresh(w.truth.clone(), old);
            w.tabs[t] = fresh;
        }
    }
}

/// (b): an entry that was in storage and no longer is has a marker, or a `p` or `z` key covers it,
/// or it was made before an erase or before its project was forgotten. The last case is an entry of
/// the old key moved by two tabs with two `seq`, one of them before the erase or the `p` key: the
/// value storage keeps decides, and either outcome is one the user's own erase or leave allows.
fn check_markers(w: &mut World) {
    let mut missing = Vec::new();
    for (id, (seq, project, at)) in &w.stored {
        let has = |kind: &str| w.truth.keys().any(|k| k.starts_with(&format!("{PREFIX}{kind}.{id}")));
        if has("e") || has("v") || has("l") || has("x") || has("s") || has("i") || w.rolled_back.contains(id) {
            continue;
        }
        let covered_by = |kind: String| {
            w.truth.keys().filter_map(|k| k.strip_prefix(&kind)).filter_map(|s| s.parse::<i64>().ok()).any(|at| at >= *seq)
        };
        let made = w.made.iter().find(|m| m.id == *id).map_or(*at, |m| m.step.min(*at));
        let dropped = covered_by(format!("{PREFIX}z.")) || project.is_some_and(|p| covered_by(format!("{PREFIX}p.{p}.")))
            || w.erases.iter().any(|e| *e >= made)
            || w.forgets.iter().any(|&(p, f)| Some(p) == *project && f >= made);
        if !dropped {
            missing.push(format!("{id} (seq {seq}, project {project:?}, stored at step {at})"));
        }
    }
    for what in missing {
        w.fail(format!("(b) the entry {what} left storage without a marker"));
    }
}

/// An entry a `p` or `z` key ended by its `seq` (Q2a), made in a tab whose view did not hold that
/// key: the tab had not yet seen that forget or erase, and wrote in the same millisecond or across a
/// clock step back. For an entry moved from the old key, the tab is each one that wrote its smallest
/// stamp. Accepted with (a).
fn dropped_by_key(w: &World, id: Uuid) -> bool {
    let Some((seq, Some(project), _)) = w.stored.get(&id) else { return false };
    let stamp = w.first_stamp.get(&id).and_then(|at| Some((*at, covers_in_view(w.stamp_covers.get(&(id, *at))?, *project))));
    let Some((seq, seen)) = w.covers_seen.get(&id).map(|seen| (*seq, seen.clone())).or(stamp) else { return false };
    covers_in_view(&w.truth, *project).into_iter().any(|at| at >= seq && !seen.contains(&at))
}

/// The `seq` of every `z` key and `p` key of `project` in `view`.
fn covers_in_view(view: &BTreeMap<String, String>, project: Uuid) -> Vec<i64> {
    let (z, p) = (format!("{PREFIX}z."), format!("{PREFIX}p.{project}."));
    view.keys().filter_map(|k| k.strip_prefix(&z).or_else(|| k.strip_prefix(&p))).filter_map(|s| s.parse().ok()).collect()
}

/// (f): an earlier edit of the row that the tab making a new one could not see, neither under its
/// own key, nor in the old key, nor in a `storage` event it received, may be stamped after it.
fn unseen_older_edits(w: &mut World, t: usize, id: Uuid, row: i32, order: usize) {
    let tab = &w.tabs[t];
    let old: Vec<Value> = tab.view.get(OLD_KEY).into_iter().chain(&tab.kept).flat_map(|v| serde_json::from_str::<Vec<Value>>(v).unwrap_or_default()).collect();
    let in_old = |other: &Uuid| old.iter().any(|e| e["id"].as_str() == Some(&other.to_string()));
    let older: Vec<Uuid> = w.edits.iter().filter(|(other, e)| **other != id && e.row == row && e.order < order).map(|(other, _)| *other).collect();
    for other in older {
        let seen = w.tabs[t].view.contains_key(&format!("{PREFIX}e.{other}")) || in_old(&other);
        if !seen {
            w.stale_pairs.insert((id, other));
        }
    }
}

/// The one loss (a) excuses, the one Q0 names, from what was recorded as it happened: the change was
/// in a 3.3.x tab's last write of the old key before that tab closed, a `[ ]` or an erase's removal
/// whose writer had not read that write replaced it, and no living tab of 3.4.0 received its
/// `storage` event.
fn replaced_unseen(w: &World, id: Uuid) -> bool {
    w.old_closings.iter().any(|last| {
        w.old_writes.get(last).is_some_and(|ids| ids.contains(&id)) && w.replaced_unread.contains(last) && !w.received_writes.contains(last)
    })
}

fn reason(w: &World, id: Uuid, made: usize, project: Uuid, after: Option<Uuid>, depth: usize) -> bool {
    if w.left.contains_key(&id) || w.forgets.iter().any(|&(p, at)| p == project && at >= made) || w.erases.iter().any(|&at| at >= made) || dropped_by_key(w, id) {
        return true;
    }
    let Some(target) = after.filter(|_| depth < 8) else { return false };
    match w.made.iter().find(|m| m.id == target) {
        Some(m) => reason(w, m.id, m.step, m.project, m.after, depth + 1),
        None => false,
    }
}

/// What an entry's stored value says, read here rather than through the code's reader.
fn row_of(raw: &Value) -> (Option<i64>, bool) {
    match raw["op"]["type"].as_str() {
        Some("add_expense") => (None, true),
        Some("edit_expense" | "delete_expense") => (raw["op"]["id"].as_i64(), false),
        _ => (None, false),
    }
}

/// (a), judged from the final storage: Q9's list (held, no key held, refused, waiting on an id with
/// no `e`, `v`, `l`, `x`, `s` or `i` key, in a cycle), then what stands behind a listed or explained
/// entry of its project and is not about another row (Q4), or waits on an explained entry.
fn judge_listed(w: &World, queue: &[Entry], held: &[Uuid], refused: &HashSet<Uuid>) -> HashSet<Uuid> {
    let has_key = |id: Uuid| ["e", "v", "l", "x", "s", "i"].iter().any(|k| w.truth.keys().any(|key| key.starts_with(&format!("{PREFIX}{k}.{id}"))));
    let afters: HashMap<Uuid, Uuid> = queue.iter().filter_map(|e| Some((e.id?, e.raw().get("after").and_then(Value::as_str).and_then(|a| Uuid::parse_str(a).ok())?))).collect();
    let in_cycle = |id: Uuid| {
        let mut at = id;
        for _ in 0..=queue.len() {
            match afters.get(&at) {
                Some(next) if *next == id => return true,
                Some(next) => at = *next,
                None => return false,
            }
        }
        false
    };
    queue
        .iter()
        .filter(|e| {
            let id = e.id.unwrap_or_default();
            let project = e.raw()["op"]["projectId"].as_str().and_then(|p| Uuid::parse_str(p).ok());
            e.op.is_none()
                || !project.is_some_and(|p| held.contains(&p))
                || refused.contains(&id)
                || afters.get(&id).is_some_and(|a| !has_key(*a))
                || in_cycle(id)
        })
        .filter_map(|e| e.id)
        .collect()
}

fn judge_explained(queue: &[Entry], listed: &HashSet<Uuid>) -> HashSet<Uuid> {
    let mut explained = listed.clone();
    loop {
        let before = explained.len();
        for (i, e) in queue.iter().enumerate() {
            let id = e.id.unwrap_or_default();
            let (row, creates) = row_of(e.raw());
            let project = e.raw()["op"]["projectId"].as_str().map(str::to_string);
            let behind = queue[..i].iter().any(|d| {
                let (d_row, d_creates) = row_of(d.raw());
                let another_row = match (d_row, row) {
                    (Some(a), Some(b)) => a != b,
                    (Some(_), None) => creates,
                    (None, Some(_)) => d_creates,
                    (None, None) => false,
                };
                explained.contains(&d.id.unwrap_or_default()) && d.raw()["op"]["projectId"].as_str().map(str::to_string) == project && !another_row
            });
            let waits = e.raw().get("after").and_then(Value::as_str).and_then(|a| Uuid::parse_str(a).ok()).is_some_and(|a| explained.contains(&a));
            if behind || waits {
                explained.insert(id);
            }
        }
        if explained.len() == before {
            return explained;
        }
    }
}

/// (f), with the accepted exceptions of docs/plans/format-v2-f2a.md "Step 1a amended": an older
/// edit that first reaches the server after a newer one of the same row is a violation, unless
/// - the older one was applied late, its tab dead with the request at the server;
/// - either was sent by the 3.3.x tab, which replays its memory with no lock;
/// - there are no Web Locks, so two tabs send at once;
/// - the older one's `seq` is not below the newer one's: two tabs in one millisecond, or a clock
///   step back, and the order falls to `id`;
/// - the sending tab's view did not hold the older one when it sent the newer (the model delays a
///   write to a view without bound; browsers deliver it within milliseconds), judged when it sent
///   it for a request applied late;
/// - the older one is a moved entry of the 3.3.x tab with a stamp below the newer one's `seq`
///   already written, of which the sending tab's view held none when it sent the newer one, so it
///   ordered the older one by a later stamp (owner's decision: one tab's writes to two keys
///   reach another tab in order in the browsers measured, the model delivers them in any order);
/// - the newer one is a moved entry of the 3.3.x tab whose smallest stamp its sender's view held,
///   at or below the older one's `seq`, was written after the older one's `e` key by a tab whose
///   view did not hold it: that `seq` ran ahead of the clock (several writes in one millisecond, or
///   a clock step back), as above.
///
/// A re-send of an edit that already landed is not judged here: only first applications are.
fn check_row_order(w: &mut World) {
    let mut out_of_order = Vec::new();
    let mut uses = [false; 4];
    for (row, log) in &w.row_log {
        for (later_at, newer) in log.iter().enumerate() {
            let Some(n) = w.edits.get(newer) else { continue };
            for older in &log[later_at + 1..] {
                let Some(o) = w.edits.get(older) else { continue };
                if o.order > n.order {
                    continue;
                }
                let pair = (*newer, *older);
                let excused = w.late.contains(older)
                    || w.sent_by_old.contains(older)
                    || w.sent_by_old.contains(newer)
                    || !w.cfg.locks
                    || o.seq.zip(n.seq).is_some_and(|(o, n)| o >= n);
                let unread_stamp = n.seq.is_none()
                    && o.seq.zip(w.sender_stamp.get(newer)).is_some_and(|(at, &stamp)| stamp <= at && w.unread_stamps.contains(&(*newer, *older, stamp)));
                let stale = [&w.stale_pairs, &w.late_pairs, &w.stamp_pairs].iter().position(|pairs| pairs.contains(&pair)).or(unread_stamp.then_some(3));
                if let (false, Some(excuse)) = (excused, stale) {
                    uses[excuse] = true;
                }
                if !excused && stale.is_none() {
                    out_of_order.push(format!("(f) row {row}: the edit {older} (seq {:?}) first reached the server after the later {newer} (seq {:?})", o.seq, n.seq));
                }
            }
        }
    }
    for what in out_of_order {
        w.fail(what);
    }
    for (excuse, used) in uses.into_iter().enumerate() {
        w.uses[excuse] |= used;
    }
}

/// (g), with the accepted exceptions: a moved expense is in exactly one project at the end, unless
/// - either entry of the move left by its user's choice or was dropped with a notice;
/// - its source or destination project was forgotten, or the device erased, after the move;
/// - another change deleted a row of that expense, or another move moved the same expense;
/// - the 3.3.x tab made the move or sent one of its entries;
/// - one of its entries is still queued, or was refused for good: listed, its user decides;
/// - one of its entries was ended by a `p` or `z` key it was made after, in a tab that had not seen
///   that forget or erase (same millisecond or a clock step back), as (a) accepts.
fn check_moves(w: &mut World, live: &HashSet<Uuid>) {
    let mut wrong = Vec::new();
    for m in &w.moves {
        let in_projects: Vec<Uuid> = w.rows.values().filter(|r| r.alive && r.expense == m.expense).map(|r| r.project).collect();
        if in_projects.len() == 1 {
            continue;
        }
        let excused = w.left.contains_key(&m.delete)
            || w.left.contains_key(&m.add)
            || w.forgets.iter().any(|&(p, at)| (p == m.source || p == m.to) && at >= m.step)
            || w.erases.iter().any(|&at| at >= m.step)
            || w.deleted_expenses.contains(&m.expense)
            || dropped_by_key(w, m.delete)
            || dropped_by_key(w, m.add)
            || live.contains(&m.delete)
            || live.contains(&m.add)
            || w.refused_ever.contains(&m.delete)
            || w.refused_ever.contains(&m.add)
            || w.moves.iter().filter(|o| o.expense == m.expense).count() > 1
            || w.sent_by_old.contains(&m.delete)
            || w.sent_by_old.contains(&m.add)
            || w.tabs.iter().filter_map(|t| t.old.as_ref()).flatten().any(|e| e["id"].as_str() == Some(&m.delete.to_string()));
        if !excused {
            let facts = |id: Uuid| format!("late {} dropped by key {} applied {:?} keys {:?}", w.late.contains(&id), dropped_by_key(w, id), w.applied.get(&id), w.truth.keys().filter(|k| k.contains(&id.to_string())).collect::<Vec<_>>());
            wrong.push(format!("(g) the expense {} moved by {} ends in {} projects: {in_projects:?}; delete {}; add {}; forgets {:?} erases {:?} step {}", m.expense, m.delete, in_projects.len(), facts(m.delete), facts(m.add), w.forgets, w.erases, m.step));
        }
    }
    for what in wrong {
        w.fail(what);
    }
}

/// Every write, view and event arrives, and each tab takes the events that reached it.
fn settle(cell: &RefCell<World>) {
    for _ in 0..100 {
        cell.borrow_mut().settle();
        if cell.borrow().tabs.iter().all(|t| t.inbox.is_empty()) {
            return;
        }
        handle_events(cell);
    }
}

thread_local! {
    /// Per excuse (`World::uses`), the schedules this thread ran that used it.
    static EXCUSED: Cell<[usize; 5]> = const { Cell::new([0; 5]) };
}

/// Runs one schedule, then lets every write arrive and drains until nothing moves.
pub fn run(seed: u64, cfg: Config) -> Result<(), String> {
    let cell = RefCell::new(World::new(seed, cfg));
    for step in 0..cfg.steps {
        {
            let mut w = cell.borrow_mut();
            w.step = step;
            if w.rng.chance(300) {
                w.deliver_one();
            }
            if w.rng.chance(40) {
                w.online = !w.online;
            }
        }
        start_pages(&cell);
        handle_events(&cell);
        let t = cell.borrow_mut().rng.below(cfg.tabs);
        action(&cell, t);
        handle_events(&cell);
        respawn(&mut cell.borrow_mut());
        if step % 25 == 0 {
            check_markers(&mut cell.borrow_mut());
        }
    }
    {
        let mut w = cell.borrow_mut();
        w.quiet = true;
        w.step = cfg.steps;
        w.online = true;
        w.lock = None;
    }
    settle(&cell);
    let new_tab = (0..cfg.tabs).find(|&t| cell.borrow().tabs[t].old.is_none()).unwrap_or(0);
    for _ in 0..3 {
        let old: Vec<usize> = (0..cfg.tabs).filter(|&t| cell.borrow().tabs[t].old.is_some()).collect();
        for t in old {
            for _ in 0..40 {
                if !old_replay_one(&cell, t) {
                    break;
                }
            }
            let memory = cell.borrow().tabs[t].old.clone().unwrap_or_default();
            let mut store = TabStore { world: &cell, tab: t };
            store.set(OLD_KEY, &Value::Array(memory).to_string());
        }
        settle(&cell);
        for _ in 0..40 {
            drain(&cell, new_tab, 50);
            settle(&cell);
        }
    }

    let mut w = cell.into_inner();
    check_markers(&mut w);
    let mut view = TabStore { world: &RefCell::new(World::new(0, Config { tabs: 1, ..cfg })), tab: 0 };
    {
        let mut inner = view.world.borrow_mut();
        inner.quiet = true;
        inner.tabs[0].view = w.truth.clone();
    }
    let snap = Snapshot::read(&mut view);
    let queue = snap.queue();
    let held: Vec<Uuid> = w.truth.get(DEV).and_then(|v| serde_json::from_str(v).ok()).unwrap_or_default();
    let refused = w.tabs[new_tab].refused.clone();
    let live: HashSet<Uuid> = queue.iter().filter_map(|e| e.id).collect();
    let blocked = |e: &Entry| refused.contains(&e.id.unwrap_or_default()) || !e.project.is_some_and(|p| held.contains(&p));
    if cfg.broken == Broken::None && next_to_send(&queue, &blocked).is_some() {
        w.fail("(a) the drain stopped with an entry it could send".into());
    }
    let judged = judge_listed(&w, &queue, &held, &refused);
    let by_code: HashSet<Uuid> = queue.iter().filter(|e| matches!(e.status, Status::Held | Status::Stuck) || blocked(e)).filter_map(|e| e.id).collect();
    if judged != by_code {
        w.fail(format!("(a) Q9 lists {by_code:?}, the judge {judged:?}"));
    }
    let explained = judge_explained(&queue, &judged);
    let invisible: Vec<String> = queue
        .iter()
        .filter(|e| !explained.contains(&e.id.unwrap_or_default()))
        .map(|e| format!("{} {:?} after {:?} seq {} project {:?}", e.id.unwrap_or_default(), e.status, e.after, e.seq, e.project))
        .collect();
    for what in invisible {
        w.fail(format!("(a) the entry {what} stays queued with nothing in the way and nothing listed"));
    }
    let made: Vec<(Uuid, usize, Uuid, Option<Uuid>)> = w.made.iter().map(|m| (m.id, m.step, m.project, m.after)).collect();
    for (id, step, project, after) in made {
        if live.contains(&id) || w.applied.contains_key(&id) || reason(&w, id, step, project, after, 0) {
            continue;
        }
        if replaced_unseen(&w, id) {
            w.uses[4] = true;
            continue;
        }
        let keys: Vec<&String> = w.truth.keys().filter(|k| k.contains(&id.to_string())).collect();
        let what = format!("(a) the change {id} made at step {step} is lost: not landed, discarded, dropped with its project, erased or queued; project {project} after {after:?} stored {:?} keys {keys:?} sent {:?} late {} in flight {} forgets {:?} erases {:?}", w.stored.get(&id), w.sent_by_new.get(&id), w.late.contains(&id), w.in_flight.iter().any(|e| e.id == Some(id)), w.forgets, w.erases);
        w.fail(what);
    }
    let sent: Vec<Uuid> = w.sent_by_new.keys().copied().collect();
    for id in sent {
        let landed = w.truth.keys().any(|k| k.starts_with(&format!("{PREFIX}l.{id}.")));
        let left = match w.made.iter().find(|m| m.id == id) {
            Some(m) => reason(&w, m.id, m.step, m.project, m.after, 0),
            None => w.stored.get(&id).is_some_and(|&(_, project, at)| {
                w.left.contains_key(&id) || w.erases.iter().any(|&e| e >= at) || w.forgets.iter().any(|&(p, f)| Some(p) == project && f >= at)
            }),
        };
        if !landed && !left && !dropped_by_key(&w, id) && w.applied.contains_key(&id) && !live.contains(&id) {
            let keys: Vec<&String> = w.truth.keys().filter(|k| k.contains(&id.to_string())).collect();
            let what = format!("(d) the entry {id} was applied but is neither marked landed nor left for a reason; late {} stored {:?} keys {keys:?} forgets {:?} erases {:?}", w.late.contains(&id), w.stored.get(&id), w.forgets, w.erases);
            w.fail(what);
        }
    }
    for key in w.truth.keys().filter(|k| k.starts_with(&format!("{PREFIX}l."))).cloned().collect::<Vec<_>>() {
        let id = key.split('.').nth(2).and_then(|s| Uuid::parse_str(s).ok()).unwrap_or_default();
        if !w.applied.contains_key(&id) {
            w.fail(format!("(d) {key} marks a landing the server never saw"));
        }
    }
    check_row_order(&mut w);
    check_moves(&mut w, &live);
    EXCUSED.set(std::array::from_fn(|i| EXCUSED.get()[i] + usize::from(w.uses[i])));
    match w.violations.is_empty() {
        true => Ok(()),
        false => Err(format!("seed {seed:#x} {cfg:?}:\n  {}", w.violations.join("\n  "))),
    }
}

fn schedules_from(base: u64, count: u64, cfg: Config) -> Vec<String> {
    (0..count).filter_map(|n| run(base + n, cfg).err()).collect()
}

/// 25 schedules per configuration by default; `make preflight-build` and the CI job `model-check`
/// run 500 (`QUEUE_MODEL_RUNS`). `QUEUE_MODEL_SEED` (hex) moves the first seed, so CI explores new
/// schedules on each pipeline.
fn count() -> u64 {
    std::env::var("QUEUE_MODEL_RUNS").ok().and_then(|s| s.parse().ok()).unwrap_or(25)
}

const WEB: Config = Config { tabs: 3, steps: 120, locks: true, one_view: false, old_tab: false, refusing: false, crossing: false, one_row: false, old_reopens: false, broken: Broken::None };
const CROSSING: Config = Config { steps: 60, crossing: true, ..WEB };

const OLD_ROW: Config = Config { old_tab: true, one_row: true, ..WEB };
const OLD_REOPENS: Config = Config { old_tab: true, old_reopens: true, ..WEB };

/// Also prints how many schedules use the one loss (a) excuses.
fn assert_none_fail(cfg: Config) {
    let base: u64 = std::env::var("QUEUE_MODEL_SEED").ok().and_then(|s| u64::from_str_radix(s.trim_start_matches("0x"), 16).ok()).unwrap_or(0x5eed_0000);
    let mut failures = Vec::new();
    EXCUSED.set([0; 5]);
    for n in 0..count() {
        if let Err(failure) = run(base + n, cfg) {
            failures.push(failure);
        }
    }
    let [stale, late, stamp, unread, lost] = EXCUSED.get();
    println!("{cfg:?}: of {} schedules, (f) excuses a stale sender in {stale}, a stale sender of a late request in {late}, a sender without the first stamp in {stamp}, a stamp by a tab that had not read the other edit in {unread}; (a) excuses the loss Q0 names in {lost}", count());
    assert!(failures.is_empty(), "{} failing schedules, first:\n{}", failures.len(), failures[0]);
}

#[test]
fn the_model_check_finds_no_lost_change_with_the_lock() {
    assert_none_fail(WEB);
}

#[test]
fn the_model_check_finds_no_lost_change_without_web_locks() {
    assert_none_fail(Config { locks: false, ..WEB });
}

#[test]
fn the_model_check_finds_no_lost_change_beside_an_older_build_and_a_refusing_server() {
    assert_none_fail(Config { tabs: 4, old_tab: true, refusing: true, ..WEB });
}

/// I14: the phone, one process and one view, every change one step.
#[test]
fn the_model_check_finds_no_lost_change_on_the_phone() {
    assert_none_fail(Config { tabs: 2, one_view: true, ..WEB });
}

/// Moves between two projects made inside one another's handlers: crossing moves form often.
#[test]
fn the_model_check_finds_no_lost_change_with_crossing_moves() {
    assert_none_fail(CROSSING);
}

/// Every change an edit of one row, beside the 3.3.x tab: old and new edits of the row cross.
#[test]
fn the_model_check_finds_no_lost_change_when_edits_of_one_row_cross() {
    assert_none_fail(OLD_ROW);
}

/// The 3.3.x tab closes often and opens again: the loss Q0 names, and its exact excuse, occur often.
#[test]
fn the_model_check_finds_no_lost_change_when_the_old_tab_reopens() {
    assert_none_fail(OLD_REOPENS);
}

/// The check catches what it is for: each rule broken on purpose fails some of `count` schedules on
/// its own property. Fixed seeds, so the demonstration does not depend on the pipeline's.
fn catches(broken: Broken, cfg: Config, property: &str) {
    catches_from(0x5eed_0000, 50, broken, cfg, property);
}

fn catches_from(first: u64, count: u64, broken: Broken, cfg: Config, property: &str) {
    let failures = schedules_from(first, count, Config { broken, ..cfg });
    let on_its_property: Vec<&String> = failures.iter().filter(|f| f.contains(property)).collect();
    assert!(!on_its_property.is_empty(), "{broken:?} went unnoticed by {property}: {} failures, first: {:?}", failures.len(), failures.first());
    println!("{broken:?}: {} of {count} schedules fail on {property}, first:\n{}", on_its_property.len(), on_its_property[0]);
}

#[test]
fn the_model_check_catches_a_landing_without_its_marker() {
    catches(Broken::LandWithoutMarker, WEB, "(b)");
}

#[test]
fn the_model_check_catches_a_drain_that_ignores_after() {
    catches(Broken::IgnoreAfter, WEB, "(e)");
}

#[test]
fn the_model_check_catches_crossing_moves_that_stop_each_other() {
    catches(Broken::NoOptionB, CROSSING, "(a)");
}

#[test]
fn the_model_check_catches_changes_to_a_row_sent_out_of_order() {
    catches(Broken::Reverse, Config { one_row: true, ..WEB }, "(f)");
}

#[test]
fn the_model_check_catches_a_moved_expense_left_in_two_projects() {
    catches(Broken::LandingTakesDependents, WEB, "(g)");
}

#[test]
fn the_model_check_catches_an_old_entry_stamped_after_a_newer_edit() {
    catches(Broken::MoveOnlyInDrain, OLD_ROW, "(f)");
}

#[test]
fn the_model_check_catches_an_entry_ordered_by_its_e_key_rather_than_its_first_stamp() {
    catches(Broken::StampsUnread, OLD_ROW, "(f)");
}

/// Caught in 9 of the 400 schedules from 0x5eed0000: the text is lost only when the tab's view has
/// gone past it, so the seeds start near one that shows it.
#[test]
fn the_model_check_catches_a_move_that_skips_the_storage_event_s_text() {
    catches_from(0x5eed_0060, 10, Broken::MoveFromKeyOnly, OLD_REOPENS, "(a)");
}

/// The narrowed excuse of (f): a tab that had read the older edit and still stamps below it is
/// caught, whatever stamps other tabs wrote unaware.
#[test]
fn the_model_check_catches_a_stamp_below_an_edit_its_tab_had_read() {
    catches(Broken::StampBelowRead, OLD_ROW, "(f)");
}

