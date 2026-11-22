//! The write queue as storage keys (SPEC §15.9.5): one key per entry, written once and never
//! rewritten, markers for what landed or left, read whole and resolved in memory. Nothing here reads
//! a key and writes it back, so a stale read in one tab can delay or repeat a send, never lose one.

use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use uuid::Uuid;

use super::offline_queue::{Entry, Status};

pub const PREFIX: &str = "counted_q.";
/// Where builds before 3.4.0 keep theirs, one array written back whole (Q0).
pub const OLD_KEY: &str = "counted_offline_queue";
/// The empty array 3.4.0 empties the old key with, one space inside, which no older build writes (Q0).
pub const EMPTIED: &str = "[ ]";
const MARKERS_KEPT_MS: i64 = 30 * 24 * 3600 * 1000;

/// The one seam between the queue and where it is stored, so that the tests can stand in for it.
pub trait Store {
    fn get(&self, key: &str) -> Option<String>;
    fn set(&mut self, key: &str, value: &str) -> bool;
    fn remove(&mut self, key: &str);
    /// Every key under the prefix, with its value.
    fn all(&self) -> Vec<(String, String)>;
    fn now_ms(&self) -> i64;
    fn seen_seq(&self) -> i64;
    fn saw_seq(&mut self, seq: i64);
    /// The entries of the old key this tab received or read that have no `o` key yet, as one array,
    /// in this tab's memory (Q0).
    fn kept(&self) -> Option<String>;
    fn set_kept(&mut self, text: Option<String>);
}

#[derive(Debug, Clone, PartialEq)]
enum Key {
    Entry(Uuid),
    Version(Uuid),
    Landed(Uuid, i64),
    Left(Uuid, i64),
    Replaced(Uuid, i64),
    LandedBefore(Uuid),
    Moved(Uuid),
    Stamp(Uuid, i64),
    Forgotten(Uuid, i64),
    Erased(i64),
}

fn uuid_part(s: &str) -> Option<Uuid> {
    Uuid::parse_str(s).ok().filter(|id| id.to_string() == s)
}

fn int_part(s: &str) -> Option<i64> {
    s.parse::<i64>().ok().filter(|n| n.to_string() == s)
}

fn generation(s: &str) -> Option<i64> {
    int_part(s).filter(|g| *g >= 0)
}

fn parse_key(key: &str) -> Option<Key> {
    let parts: Vec<&str> = key.strip_prefix(PREFIX)?.split('.').collect();
    Some(match parts[..] {
        ["e", id] => Key::Entry(uuid_part(id)?),
        ["v", id, rid] => {
            uuid_part(rid)?;
            Key::Version(uuid_part(id)?)
        }
        ["l", id, g] => Key::Landed(uuid_part(id)?, generation(g)?),
        ["x", id, g] => Key::Left(uuid_part(id)?, generation(g)?),
        ["s", id, g] => Key::Replaced(uuid_part(id)?, generation(g)?),
        ["i", id] => Key::LandedBefore(uuid_part(id)?),
        ["o", id] => Key::Moved(uuid_part(id)?),
        ["t", id, seq] => Key::Stamp(uuid_part(id)?, int_part(seq)?),
        ["p", project, seq] => Key::Forgotten(uuid_part(project)?, int_part(seq)?),
        ["z", seq] => Key::Erased(int_part(seq)?),
        _ => return None,
    })
}

pub fn entry_key(id: Uuid) -> String {
    format!("{PREFIX}e.{id}")
}

fn marker_key(kind: &str, id: Uuid, gen: Option<i64>) -> String {
    match gen {
        Some(g) => format!("{PREFIX}{kind}.{id}.{g}"),
        None => format!("{PREFIX}{kind}.{id}"),
    }
}

fn uuid_in(value: Option<&Value>) -> Option<Uuid> {
    value.and_then(Value::as_str).and_then(|s| Uuid::parse_str(s).ok())
}

/// The `seq` of what this build writes next (Q2): the milliseconds since the epoch, or one more
/// than the greatest `seq` it has read or written.
fn next_seq(store: &mut dyn Store) -> i64 {
    let seq = store.now_ms().max(store.seen_seq().saturating_add(1));
    store.saw_seq(seq);
    seq
}

fn seq_value(store: &mut dyn Store) -> String {
    json!({ "seq": next_seq(store) }).to_string()
}

#[derive(Debug, Clone)]
struct Marker {
    key: String,
    id: Uuid,
    seq: Option<i64>,
}

/// Every key under the prefix as one read saw them.
#[derive(Debug, Clone, Default)]
pub struct Snapshot {
    versions: BTreeMap<Uuid, Vec<Entry>>,
    landed: HashMap<Uuid, BTreeMap<i64, Option<i64>>>,
    left: HashMap<Uuid, BTreeSet<i64>>,
    replaced: HashMap<Uuid, BTreeMap<i64, Option<Uuid>>>,
    landed_before: HashSet<Uuid>,
    moved: HashSet<Uuid>,
    stamps: HashMap<Uuid, i64>,
    stamp_keys: Vec<(Uuid, String)>,
    forgotten: HashMap<Uuid, i64>,
    erased: Option<i64>,
    markers: Vec<Marker>,
}

impl Snapshot {
    pub fn read(store: &mut dyn Store) -> Snapshot {
        let mut snap = Snapshot::default();
        let mut max_seq = i64::MIN;
        for (key, value) in store.all() {
            let parsed: Option<Value> = serde_json::from_str(&value).ok();
            let marker_seq = parsed.as_ref().and_then(|v| v.get("seq")).and_then(Value::as_i64);
            let marker = |snap: &mut Snapshot, id: Uuid| snap.markers.push(Marker { key: key.clone(), id, seq: marker_seq });
            match parse_key(&key) {
                Some(Key::Entry(id)) => snap.add_version(id, &key, parsed.unwrap_or(Value::String(value)), false),
                Some(Key::Version(id)) => snap.add_version(id, &key, parsed.unwrap_or(Value::String(value)), true),
                Some(Key::Landed(id, g)) => {
                    let row = parsed.as_ref().and_then(|v| v.get("row")).and_then(Value::as_i64);
                    snap.landed.entry(id).or_default().insert(g, row);
                    marker(&mut snap, id);
                }
                Some(Key::Left(id, g)) => {
                    snap.left.entry(id).or_default().insert(g);
                    marker(&mut snap, id);
                }
                Some(Key::Replaced(id, g)) => {
                    let by = uuid_in(parsed.as_ref().and_then(|v| v.get("by")));
                    snap.replaced.entry(id).or_default().insert(g, by);
                    marker(&mut snap, id);
                }
                Some(Key::LandedBefore(id)) => {
                    snap.landed_before.insert(id);
                    marker(&mut snap, id);
                }
                Some(Key::Moved(id)) => {
                    snap.moved.insert(id);
                }
                Some(Key::Stamp(id, seq)) => {
                    let at = snap.stamps.entry(id).or_insert(seq);
                    *at = (*at).min(seq);
                    snap.stamp_keys.push((id, key.clone()));
                    max_seq = max_seq.max(seq);
                }
                Some(Key::Forgotten(project, seq)) => {
                    let at = snap.forgotten.entry(project).or_insert(seq);
                    *at = (*at).max(seq);
                    max_seq = max_seq.max(seq);
                }
                Some(Key::Erased(seq)) => {
                    snap.erased = Some(snap.erased.map_or(seq, |z| z.max(seq)));
                    max_seq = max_seq.max(seq);
                }
                None => {}
            }
            max_seq = max_seq.max(marker_seq.unwrap_or(i64::MIN));
        }
        if max_seq > i64::MIN {
            store.saw_seq(max_seq.max(store.seen_seq()));
        }
        for (id, seq) in &snap.stamps {
            snap.versions.get_mut(id).into_iter().flatten().for_each(|v| v.seq = *seq);
        }
        snap
    }

    /// A version 3.4.0 holds (a `v` key is written from 3.4.1, which it replays none of), or an `e`
    /// key whose value names another id or a generation: held, kept, listed.
    fn add_version(&mut self, id: Uuid, key: &str, raw: Value, later: bool) {
        let mut entry = Entry::read(raw);
        let gen = match later {
            true => entry.raw().get("gen").and_then(Value::as_i64).filter(|g| *g >= 1).unwrap_or(0),
            false => 0,
        };
        if later || entry.id != Some(id) {
            entry.op = None;
        }
        entry.id = Some(id);
        entry.key = key.to_string();
        entry.gen = gen;
        self.versions.entry(id).or_default().push(entry);
    }

    fn covered(&self, id: Uuid, gen: i64) -> bool {
        self.landed.get(&id).is_some_and(|m| m.range(gen..).next().is_some())
            || self.left.get(&id).is_some_and(|m| m.range(gen..).next().is_some())
            || self.replaced.get(&id).is_some_and(|m| m.range(gen..).next().is_some())
    }

    fn dropped(&self, version: &Entry) -> bool {
        self.erased.is_some_and(|z| version.seq <= z)
            || version.project.and_then(|p| self.forgotten.get(&p)).is_some_and(|p| version.seq <= *p)
    }

    /// Q2a.
    fn is_live(&self, version: &Entry) -> bool {
        version.id.is_some_and(|id| !self.covered(id, version.gen)) && !self.dropped(version)
    }

    /// Q2a: the live version of greatest `gen`, then `seq`, then key.
    fn live(&self, id: Uuid) -> Option<&Entry> {
        self.versions
            .get(&id)?
            .iter()
            .filter(|v| self.is_live(v))
            .max_by(|a, b| (a.gen, a.seq, &a.key).cmp(&(b.gen, b.seq, &b.key)))
    }

    fn knows(&self, id: Uuid) -> bool {
        self.versions.contains_key(&id)
            || self.landed.contains_key(&id)
            || self.left.contains_key(&id)
            || self.replaced.contains_key(&id)
            || self.landed_before.contains(&id)
            || self.moved.contains(&id)
    }

    /// Q3, its four steps, for an entry that names `target` in `after`. A cycle, or a target with no
    /// `e`, `v`, `l`, `x`, `s` or `i` key, waits and is listed (Q9).
    fn after(&self, target: Uuid, visiting: &mut Vec<Uuid>) -> Status {
        if visiting.contains(&target) {
            return Status::Stuck;
        }
        if let Some(live) = self.live(target) {
            return match self.status(live, visiting) {
                Status::Leaves => Status::Leaves,
                _ => Status::Waits,
            };
        }
        let ended = self.versions.get(&target).into_iter().flatten().filter(|v| self.dropped(v)).map(|v| v.gen).max();
        let top = [
            self.landed.get(&target).and_then(|m| m.keys().next_back()).copied(),
            self.left.get(&target).and_then(|m| m.iter().next_back()).copied(),
            self.replaced.get(&target).and_then(|m| m.keys().next_back()).copied(),
            ended,
        ]
        .into_iter()
        .flatten()
        .max();
        if let Some(g) = top {
            if let Some(by) = self.replaced.get(&target).and_then(|m| m.get(&g)) {
                let Some(by) = *by else { return Status::Stuck };
                visiting.push(target);
                let status = self.after(by, visiting);
                visiting.pop();
                return status;
            }
            if self.landed.get(&target).is_some_and(|m| m.contains_key(&g)) {
                return Status::Ready;
            }
            return Status::Leaves;
        }
        if self.landed_before.contains(&target) {
            return Status::Ready;
        }
        Status::Stuck
    }

    fn status(&self, entry: &Entry, visiting: &mut Vec<Uuid>) -> Status {
        let Some(id) = entry.id else { return Status::Held };
        visiting.push(id);
        let after = entry.after.map_or(Status::Ready, |a| self.after(a, visiting));
        visiting.pop();
        match after {
            Status::Leaves => Status::Leaves,
            _ if entry.op.is_none() => Status::Held,
            _ if self.in_cycle(id) => Status::Stuck,
            other => other,
        }
    }

    fn in_cycle(&self, id: Uuid) -> bool {
        let mut seen = vec![id];
        let mut at = id;
        while let Some(next) = self.live(at).and_then(|e| e.after) {
            if next == id {
                return true;
            }
            if seen.contains(&next) {
                return false;
            }
            seen.push(next);
            at = next;
        }
        false
    }

    /// Every live entry with its status, in the order of `seq`, then `id`; those that leave by rule
    /// (Q3) included.
    pub fn entries(&self) -> Vec<Entry> {
        let mut out: Vec<Entry> = self
            .versions
            .keys()
            .filter_map(|id| self.live(*id))
            .map(|live| {
                let mut entry = live.clone();
                entry.status = self.status(live, &mut Vec::new());
                entry
            })
            .collect();
        out.sort_by_key(|e| (e.seq, e.id));
        out
    }

    /// The queue (Q2): the live entries that do not leave by rule.
    pub fn queue(&self) -> Vec<Entry> {
        self.entries().into_iter().filter(|e| e.status != Status::Leaves).collect()
    }

    /// The old key's entries not moved yet (Q0), last and in their order, each stamped `now_ms` plus
    /// its place, as the screen shows them until the replay moves them: never sent from here, and a
    /// discard of one writes its `x` key, after which the move skips it.
    pub fn not_moved(&self, old: Option<String>, now_ms: i64) -> Vec<Entry> {
        let Some(Value::Array(old)) = old.and_then(|text| serde_json::from_str(&text).ok()) else { return Vec::new() };
        let mut seen: HashSet<Uuid> = HashSet::new();
        old.into_iter()
            .enumerate()
            .filter_map(|(place, raw)| {
                let id = uuid_in(raw.get("id"))?;
                if self.knows(id) || !seen.insert(id) {
                    return None;
                }
                let mut entry = Entry::read(raw);
                entry.key = OLD_KEY.to_string();
                entry.seq = now_ms.saturating_add(place as i64);
                if entry.op.is_some() && entry.after.is_some_and(|a| self.live(a).is_some() || seen.contains(&a)) {
                    entry.status = Status::Waits;
                }
                Some(entry)
            })
            .collect()
    }

    /// The row a creation landed as, written in its `l` key.
    pub fn landed_row(&self, id: Uuid) -> Option<i64> {
        self.landed.get(&id)?.values().flatten().next_back().copied()
    }

    /// What would leave with `project` (Q3), dependents in other projects included.
    pub fn dropped_with(&self, project: Uuid) -> usize {
        let mut without = self.clone();
        without.forgotten.insert(project, i64::MAX);
        self.queue().len() - without.queue().len()
    }

    fn version_keys_up_to(&self, id: Uuid, gen: i64) -> Vec<String> {
        self.versions.get(&id).into_iter().flatten().filter(|v| v.gen <= gen).map(|v| v.key.clone()).collect()
    }
}

/// Q2: every `p` and `z` key stored counts as read before a `seq` is drawn.
fn see_ends(store: &mut dyn Store) {
    let ends = store.all().into_iter().filter_map(|(key, _)| match parse_key(&key) {
        Some(Key::Erased(seq) | Key::Forgotten(_, seq)) => Some(seq),
        _ => None,
    });
    if let Some(seq) = ends.max() {
        store.saw_seq(seq);
    }
}

/// Q2d: written in the order given, those that name others in `after` first; on a throw, what was
/// written is removed again and nothing is made. Every `p` and `z` key stored is read first (Q2), so
/// that no entry is stamped at or below a forget or an erase this tab can see.
pub fn write_entries(store: &mut dyn Store, entries: &[Value]) -> bool {
    see_ends(store);
    let mut written: Vec<String> = Vec::new();
    for raw in entries {
        let (Some(id), Value::Object(fields)) = (uuid_in(raw.get("id")), raw) else {
            written.iter().for_each(|key| store.remove(key));
            return false;
        };
        let mut fields = fields.clone();
        fields.insert("seq".into(), json!(next_seq(store)));
        let key = entry_key(id);
        if !store.set(&key, &Value::Object(fields).to_string()) {
            written.iter().for_each(|key| store.remove(key));
            return false;
        }
        written.push(key);
    }
    true
}

fn mark_then_remove(store: &mut dyn Store, snap: &Snapshot, marker: String, value: String, id: Uuid, gen: i64) -> bool {
    if !store.set(&marker, &value) {
        return false;
    }
    snap.version_keys_up_to(id, gen).iter().for_each(|key| store.remove(key));
    true
}

/// Q2b: `l` for the generation sent, as `row` for a creation, then the keys up to it.
pub fn land(store: &mut dyn Store, snap: &Snapshot, entry: &Entry, row: Option<i64>) -> bool {
    let Some(id) = entry.id else { return false };
    let mut value = json!({ "seq": next_seq(store) });
    if let Some(row) = row {
        value["row"] = json!(row);
    }
    mark_then_remove(store, snap, marker_key("l", id, Some(entry.gen)), value.to_string(), id, entry.gen)
}

/// Q3: an entry that leaves for any reason but its landing gets its `x` key; what names it in
/// `after` leaves by rule.
pub fn leave(store: &mut dyn Store, snap: &Snapshot, entry: &Entry) -> bool {
    let Some(id) = entry.id else { return false };
    let value = seq_value(store);
    mark_then_remove(store, snap, marker_key("x", id, Some(entry.gen)), value, id, entry.gen)
}

/// Q3: a device that forgets a project writes its `p` key, which ends every entry of it made before,
/// seen here or not; those seen here leave with their `x` key, so that what names them in `after`
/// still learns it once their keys are gone.
pub fn forget(store: &mut dyn Store, snap: &Snapshot, project: Uuid) -> bool {
    let seq = next_seq(store);
    if !store.set(&format!("{PREFIX}p.{project}.{seq}"), "{}") {
        return false;
    }
    let gone: Vec<Entry> = snap.versions.keys().filter_map(|id| snap.live(*id)).filter(|v| v.project == Some(project)).cloned().collect();
    gone.iter().all(|entry| leave(store, snap, entry))
}

/// Q3, after the device state was written without the project keys: a `z` key, then every key
/// under the prefix but the `o` keys and that `z` key, and the old key, and the kept entries (Q0).
/// A full store does not stop an erase: a `z` key is written once the rest is gone, under a new
/// `seq`, since another erase may have stored the first name meanwhile. The `p` and `z` keys
/// stored are read first (Q2), so the `seq` is above every one of them.
pub fn erase(store: &mut dyn Store) {
    store.set_kept(None);
    see_ends(store);
    let z = format!("{PREFIX}z.{}", next_seq(store));
    let marked = store.set(&z, "{}");
    for (key, _) in store.all() {
        if key != z && !matches!(parse_key(&key), Some(Key::Moved(_))) {
            store.remove(&key);
        }
    }
    store.remove(OLD_KEY);
    if !marked {
        let z = format!("{PREFIX}z.{}", next_seq(store));
        store.set(&z, "{}");
    }
}

/// What a move did (Q0).
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Moved {
    Nothing,
    /// Every entry of the text read has its `o` key.
    Whole,
    /// A write after the stamps failed, or an element without a readable `id` stays: the next move
    /// goes on from there.
    Partly,
    /// A stamp could not be written, so no entry behind it was stamped (Q2d).
    Unstamped,
}

/// Q0: the entries of the new value of a `storage` event for the old key are kept: another tab's
/// write can replace that text before this tab has moved them. An event without a new value tells
/// that the key was removed, which only an erase does in any build, so the kept entries go.
pub fn receive_old_text(store: &mut dyn Store, text: Option<String>) {
    let Some(text) = text else { return store.set_kept(None) };
    let Ok(Value::Array(arrived)) = serde_json::from_str::<Value>(&text) else { return };
    let mut kept = kept_entries(store);
    let mut ids: HashSet<Uuid> = kept.iter().filter_map(|raw| uuid_in(raw.get("id"))).collect();
    for raw in arrived {
        if uuid_in(raw.get("id")).is_some_and(|id| ids.insert(id)) {
            kept.push(raw);
        }
    }
    store.set_kept((!kept.is_empty()).then(|| Value::Array(kept).to_string()));
}

fn kept_entries(store: &dyn Store) -> Vec<Value> {
    match store.kept().as_deref().map(serde_json::from_str::<Value>) {
        Some(Ok(Value::Array(kept))) => kept,
        _ => Vec::new(),
    }
}

/// Q0: the move reads the kept entries, then those of the old key. First the stamps, in that order,
/// up to the first that cannot be written; then, for each entry that has one, its `i` key, its `e`
/// key with the smallest stamp, its `o` key, a failed write leaving that entry, and those behind it
/// that name it in `after`, for the next move. Every entry read that has no `o` key yet is kept, the
/// others dropped. Once every element read is an entry with its `o` key, the key is emptied with
/// `[ ]` if it still holds what was read.
pub fn move_old(store: &mut dyn Store, snap: &Snapshot) -> Moved {
    let in_key = store.get(OLD_KEY);
    let held = match in_key.as_deref().map(serde_json::from_str::<Value>) {
        Some(Ok(Value::Array(held))) => held,
        _ => Vec::new(),
    };
    let kept = kept_entries(store);
    let old: Vec<&Value> = kept.iter().chain(&held).collect();
    if old.is_empty() {
        return Moved::Nothing;
    }
    let in_old: HashSet<Uuid> = old.iter().filter_map(|raw| uuid_in(raw.get("id"))).collect();
    let mut stamps: HashMap<Uuid, i64> = snap.stamps.clone();
    let mut unstamped = false;
    for id in old.iter().filter_map(|raw| uuid_in(raw.get("id"))) {
        if snap.knows(id) || stamps.contains_key(&id) {
            continue;
        }
        let seq = next_seq(store);
        if !store.set(&format!("{PREFIX}t.{id}.{seq}"), "{}") {
            unstamped = true;
            break;
        }
        stamps.insert(id, seq);
    }
    let mut every_one_moved = !unstamped;
    let mut done: HashSet<Uuid> = HashSet::new();
    let mut landed_before: HashSet<Uuid> = HashSet::new();
    let mut marked: HashSet<Uuid> = HashSet::new();
    let mut left: HashSet<Uuid> = HashSet::new();
    for raw in old.iter().copied() {
        let Some(id) = uuid_in(raw.get("id")) else {
            every_one_moved = false;
            continue;
        };
        if !done.insert(id) {
            continue;
        }
        if snap.knows(id) {
            let value = seq_value(store);
            if snap.moved.contains(&id) || store.set(&marker_key("o", id, None), &value) {
                marked.insert(id);
            } else {
                every_one_moved = false;
            }
            continue;
        }
        let after = uuid_in(raw.get("after"));
        let (Some(seq), Value::Object(fields), false) = (stamps.get(&id), raw, after.is_some_and(|a| left.contains(&a))) else {
            left.insert(id);
            every_one_moved = false;
            continue;
        };
        if let Some(after) = after.filter(|a| !in_old.contains(a) && !snap.knows(*a) && !landed_before.contains(a)) {
            let value = seq_value(store);
            if !store.set(&marker_key("i", after, None), &value) {
                left.insert(id);
                every_one_moved = false;
                continue;
            }
            landed_before.insert(after);
        }
        let mut fields = fields.clone();
        fields.insert("seq".into(), json!(seq));
        if !store.set(&entry_key(id), &Value::Object(fields).to_string()) {
            left.insert(id);
            every_one_moved = false;
            continue;
        }
        let value = seq_value(store);
        if store.set(&marker_key("o", id, None), &value) {
            marked.insert(id);
        } else {
            every_one_moved = false;
        }
    }
    let mut unmoved: Vec<Value> = Vec::new();
    for raw in &old {
        if uuid_in(raw.get("id")).is_some_and(|id| !marked.contains(&id) && done.remove(&id)) {
            unmoved.push((*raw).clone());
        }
    }
    store.set_kept((!unmoved.is_empty()).then(|| Value::Array(unmoved).to_string()));
    if unstamped {
        return Moved::Unstamped;
    }
    if !every_one_moved {
        return Moved::Partly;
    }
    if !held.is_empty() && store.get(OLD_KEY) == in_key {
        store.set(OLD_KEY, EMPTIED);
    }
    Moved::Whole
}

/// Q2d, Q2c: a handler moves the old key's entries it sees, then writes its own. Nothing of its own
/// is written only when the move could not write a stamp; no other write of the move stops it.
pub fn write_change(store: &mut dyn Store, entries: &[Value]) -> bool {
    let snap = Snapshot::read(store);
    move_old(store, &snap) != Moved::Unstamped && write_entries(store, entries)
}

/// What any build may tidy (Q2a, Q2b, Q3): the `x` key of an entry that leaves by rule, then its
/// keys; every key that is not live or not the live version, an entry another tab's `p` key ended
/// getting its `x` key first; the stamps of an id once none of its `e` or `v` keys is stored, its
/// `o` key is, and one of its `l`, `x` or `s` keys is (Q0); and a marker older than 30 days once
/// no `e`, `v` or `t` key of its id is stored and no stored entry names it.
pub fn tidy(store: &mut dyn Store, snap: &Snapshot) {
    for entry in snap.entries().into_iter().filter(|e| e.status == Status::Leaves) {
        leave(store, snap, &entry);
    }
    for (id, versions) in &snap.versions {
        let live = snap.live(*id).map(|v| v.key.clone());
        let ended_by_p = versions
            .iter()
            .filter(|v| live.is_none() && !snap.covered(*id, v.gen) && !snap.erased.is_some_and(|z| v.seq <= z))
            .map(|v| v.gen)
            .max();
        if let Some(gen) = ended_by_p {
            let value = seq_value(store);
            if !store.set(&marker_key("x", *id, Some(gen)), &value) {
                continue;
            }
        }
        for version in versions.iter().filter(|v| Some(&v.key) != live.as_ref()) {
            store.remove(&version.key);
        }
    }
    for (id, key) in &snap.stamp_keys {
        let marked = snap.landed.contains_key(id) || snap.left.contains_key(id) || snap.replaced.contains_key(id);
        if !snap.versions.contains_key(id) && snap.moved.contains(id) && marked {
            store.remove(key);
        }
    }
    let mut named: HashSet<Uuid> = snap.versions.values().flatten().filter_map(|v| v.after).collect();
    let mut reached: Vec<Uuid> = named.iter().copied().collect();
    while let Some(id) = reached.pop() {
        for by in snap.replaced.get(&id).into_iter().flat_map(|m| m.values().flatten()) {
            if named.insert(*by) {
                reached.push(*by);
            }
        }
    }
    let old = store.now_ms().saturating_sub(MARKERS_KEPT_MS);
    for marker in &snap.markers {
        let stored = snap.versions.contains_key(&marker.id) || snap.stamps.contains_key(&marker.id);
        if marker.seq.is_some_and(|seq| seq < old) && !stored && !named.contains(&marker.id) {
            store.remove(&marker.key);
        }
    }
}

/// Q4: the first entry the replay may send. An entry not sent for a reason of its own (held, parked,
/// waiting, or one `blocked` names) stops the entries behind it for its project that are not about
/// another row; one whose project cannot be read stops every one behind it. An entry stopped only
/// by another stops nothing itself, so the stops never form a knot that no listed entry explains.
pub fn next_to_send(queue: &[Entry], mut blocked: impl FnMut(&Entry) -> bool) -> Option<usize> {
    let mut stops: Vec<&Entry> = Vec::new();
    for (position, entry) in queue.iter().enumerate() {
        let behind = stops.iter().any(|s| s.project.is_none() || (s.project == entry.project && !s.about_another_row_than(entry)));
        let own_reason = entry.status != Status::Ready || entry.project.is_none() || blocked(entry);
        if !own_reason && !behind {
            return Some(position);
        }
        if own_reason {
            stops.push(entry);
        }
    }
    None
}

/// localStorage, or sessionStorage in the demo, under the keys of the prefix.
#[cfg(target_arch = "wasm32")]
pub struct DeviceStore(Option<web_sys::Storage>);

#[cfg(target_arch = "wasm32")]
impl Store for DeviceStore {
    fn get(&self, key: &str) -> Option<String> {
        self.0.as_ref()?.get_item(key).ok().flatten()
    }

    fn set(&mut self, key: &str, value: &str) -> bool {
        let Some(storage) = &self.0 else { return false };
        match storage.set_item(key, value) {
            Ok(()) => true,
            Err(e) => {
                dioxus::logger::tracing::error!("counted: write {key} failed ({} chars): {e:?}", value.len());
                false
            }
        }
    }

    fn remove(&mut self, key: &str) {
        if let Some(storage) = &self.0 {
            let _ = storage.remove_item(key);
        }
    }

    fn all(&self) -> Vec<(String, String)> {
        let Some(storage) = &self.0 else { return Vec::new() };
        let count = storage.length().unwrap_or(0);
        let keys: Vec<String> = (0..count).filter_map(|i| storage.key(i).ok().flatten()).filter(|k| k.starts_with(PREFIX)).collect();
        keys.into_iter().filter_map(|k| Some((k.clone(), storage.get_item(&k).ok().flatten()?))).collect()
    }

    fn now_ms(&self) -> i64 {
        js_sys::Date::now() as i64
    }

    fn seen_seq(&self) -> i64 {
        SEEN.with(|s| s.get())
    }

    fn saw_seq(&mut self, seq: i64) {
        SEEN.with(|s| s.set(s.get().max(seq)));
    }

    fn kept(&self) -> Option<String> {
        KEPT.with_borrow(Clone::clone)
    }

    fn set_kept(&mut self, text: Option<String>) {
        KEPT.set(text);
    }
}

thread_local! {
    static SEEN: std::cell::Cell<i64> = const { std::cell::Cell::new(0) };
    static KEPT: std::cell::RefCell<Option<String>> = const { std::cell::RefCell::new(None) };
}

#[cfg(target_arch = "wasm32")]
pub fn device_store() -> DeviceStore {
    DeviceStore(super::persist::web_storage())
}

/// Runs `step` on the device store as one step. On the phone that is one read of the file and at
/// most one write of it, made at the end, or before the old key's file is written so that its
/// entries are stored before it is emptied (Q0). `None` when that write failed: nothing it did is
/// stored.
#[cfg(target_arch = "wasm32")]
pub fn in_one_step<T>(step: impl FnOnce(&mut dyn Store) -> T) -> Option<T> {
    Some(step(&mut device_store()))
}

#[cfg(not(target_arch = "wasm32"))]
pub fn in_one_step<T>(step: impl FnOnce(&mut dyn Store) -> T) -> Option<T> {
    let _one_step = ONE_STEP.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
    let mut store = DeviceStore { staged: Some(DeviceStore::map()), dirty: false };
    let out = step(&mut store);
    store.flush().then_some(out)
}

/// On the phone: one file of the sealed store holding the same keys and values as one JSON object,
/// every change one synchronous step under `ONE_STEP`, the old key in its own file as before. A value
/// that is not a string is kept, and read as the JSON string of its text, which no build reads as an
/// entry: held (Q1).
#[cfg(not(target_arch = "wasm32"))]
pub struct DeviceStore {
    staged: Option<BTreeMap<String, Value>>,
    dirty: bool,
}

#[cfg(not(target_arch = "wasm32"))]
const QUEUE_FILE: &str = "counted_q";

#[cfg(not(target_arch = "wasm32"))]
static ONE_STEP: std::sync::Mutex<()> = std::sync::Mutex::new(());

#[cfg(all(test, not(target_arch = "wasm32")))]
thread_local! {
    pub static FILE_WRITES: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

#[cfg(not(target_arch = "wasm32"))]
fn as_stored(value: &Value) -> String {
    match value {
        Value::String(s) => s.clone(),
        other => Value::String(other.to_string()).to_string(),
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn write_file(map: &BTreeMap<String, Value>) -> bool {
    #[cfg(test)]
    FILE_WRITES.with(|w| w.set(w.get() + 1));
    super::persist::write_json(QUEUE_FILE, "offline queue", map)
}

#[cfg(not(target_arch = "wasm32"))]
impl DeviceStore {
    fn map() -> BTreeMap<String, Value> {
        super::persist::read_json(QUEUE_FILE)
    }

    fn change(&mut self, change: impl FnOnce(&mut BTreeMap<String, Value>)) -> bool {
        if let Some(staged) = self.staged.as_mut() {
            change(staged);
            self.dirty = true;
            return true;
        }
        let _one_step = ONE_STEP.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        let mut map = Self::map();
        change(&mut map);
        write_file(&map)
    }

    fn flush(&mut self) -> bool {
        let Some(staged) = self.staged.as_ref().filter(|_| self.dirty) else { return true };
        let written = write_file(staged);
        self.dirty &= !written;
        written
    }

    fn read(&self) -> BTreeMap<String, Value> {
        self.staged.clone().unwrap_or_else(Self::map)
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl Store for DeviceStore {
    fn get(&self, key: &str) -> Option<String> {
        if key == OLD_KEY {
            return Some(super::persist::read_json::<Value>(OLD_KEY)).filter(|v| !v.is_null()).map(|v| v.to_string());
        }
        self.read().get(key).map(as_stored)
    }

    fn set(&mut self, key: &str, value: &str) -> bool {
        if key == OLD_KEY {
            let Ok(value) = serde_json::from_str::<Value>(value) else { return false };
            return self.flush() && super::persist::write_json(OLD_KEY, "offline queue before 3.4.0", &value);
        }
        self.change(|map| {
            map.insert(key.to_string(), Value::String(value.to_string()));
        })
    }

    fn remove(&mut self, key: &str) {
        if key == OLD_KEY {
            if self.flush() {
                super::persist::remove_store(OLD_KEY);
            }
            return;
        }
        if self.staged.is_none() && !Self::map().contains_key(key) {
            return;
        }
        self.change(|map| {
            map.remove(key);
        });
    }

    fn all(&self) -> Vec<(String, String)> {
        self.read().iter().filter(|(k, _)| k.starts_with(PREFIX)).map(|(k, v)| (k.clone(), as_stored(v))).collect()
    }

    fn now_ms(&self) -> i64 {
        shared::time::DateTime::now_utc().unix_millis()
    }

    fn seen_seq(&self) -> i64 {
        SEEN.with(|s| s.get())
    }

    fn saw_seq(&mut self, seq: i64) {
        SEEN.with(|s| s.set(s.get().max(seq)));
    }

    fn kept(&self) -> Option<String> {
        KEPT.with_borrow(Clone::clone)
    }

    fn set_kept(&mut self, text: Option<String>) {
        KEPT.set(text);
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub fn device_store() -> DeviceStore {
    DeviceStore { staged: None, dirty: false }
}

/// Q3 on the phone: the erase removes the whole file, and the old key's.
#[cfg(not(target_arch = "wasm32"))]
pub fn erase_device() {
    let _one_step = ONE_STEP.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
    KEPT.set(None);
    super::persist::remove_store(QUEUE_FILE);
    super::persist::remove_store(OLD_KEY);
}

/// Q3 on the web; the old key's quarantined copy goes with it, which shipped builds made when that
/// key did not parse.
#[cfg(target_arch = "wasm32")]
pub fn erase_device() {
    let mut store = device_store();
    erase(&mut store);
    super::persist::remove_store(OLD_KEY);
}

#[cfg(test)]
#[path = "queue_store_tests.rs"]
pub mod tests;
