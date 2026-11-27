use super::*;
use crate::common::offline_queue::{OpKind, ProjectEdit, QueuedOp};
use std::cell::RefCell;

/// The I2 fake: any test fails in which a key read as present is written again, `o` keys and the
/// old key aside (Q0, Q2). `writes_left` cuts a run after that many changes, the later ones lost as
/// in a crash; `refuse` makes every write of a key starting with it throw, as a full store does,
/// only the first `refusals` times when that is set.
#[derive(Default)]
pub struct MemStore {
    pub map: BTreeMap<String, String>,
    pub now: i64,
    seen: i64,
    read_present: RefCell<HashSet<String>>,
    pub writes_left: Option<usize>,
    pub refuse: Option<String>,
    pub refusals: Option<usize>,
    pub changes: usize,
    pub kept: Option<String>,
}

impl MemStore {
    pub fn new() -> Self {
        Self { now: 1_000_000, ..Default::default() }
    }

    fn alive(&mut self) -> bool {
        self.changes += 1;
        match &mut self.writes_left {
            Some(0) => false,
            Some(n) => {
                *n -= 1;
                true
            }
            None => true,
        }
    }

    pub fn keys(&self) -> Vec<String> {
        self.map.keys().cloned().collect()
    }
}

impl Store for MemStore {
    fn get(&self, key: &str) -> Option<String> {
        let value = self.map.get(key).cloned();
        if value.is_some() {
            self.read_present.borrow_mut().insert(key.to_string());
        }
        value
    }

    fn set(&mut self, key: &str, value: &str) -> bool {
        let exempt = key == OLD_KEY || matches!(parse_key(key), Some(Key::Moved(_)));
        assert!(exempt || !self.read_present.borrow().contains(key), "I2: {key} was read as present and is written again");
        if self.refuse.as_ref().is_some_and(|r| key.starts_with(r.as_str())) && self.refusals != Some(0) {
            if let Some(n) = self.refusals.as_mut() {
                *n -= 1;
            }
            return false;
        }
        if self.alive() {
            self.map.insert(key.to_string(), value.to_string());
        }
        true
    }

    fn remove(&mut self, key: &str) {
        if self.alive() {
            self.map.remove(key);
        }
    }

    fn all(&self) -> Vec<(String, String)> {
        let all: Vec<(String, String)> =
            self.map.iter().filter(|(k, _)| k.starts_with(PREFIX)).map(|(k, v)| (k.clone(), v.clone())).collect();
        self.read_present.borrow_mut().extend(all.iter().map(|(k, _)| k.clone()));
        all
    }

    fn now_ms(&self) -> i64 {
        self.now
    }

    fn seen_seq(&self) -> i64 {
        self.seen
    }

    fn saw_seq(&mut self, seq: i64) {
        self.seen = self.seen.max(seq);
    }

    fn kept(&self) -> Option<String> {
        self.kept.clone()
    }

    fn set_kept(&mut self, text: Option<String>) {
        self.kept = text;
    }
}

const P: Uuid = Uuid::from_u128(0x50);
const Q: Uuid = Uuid::from_u128(0x51);

pub fn id(n: u128) -> Uuid {
    Uuid::from_u128(0x1000 + n)
}

fn op(n: u128, kind: OpKind, after: Option<Uuid>) -> Value {
    serde_json::to_value(QueuedOp { id: id(n), label: String::new(), sealed_label: None, op: kind, after, queued_at: Some(1) }).unwrap()
}

fn pair() -> shared::EncryptedPair {
    shared::EncryptedPair { ct: "ct".into(), iv: "iv".into() }
}

pub fn add(n: u128, project: Uuid) -> Value {
    op(
        n,
        OpKind::AddExpense(shared::CreatableExpense {
            project_id: project,
            author_id: None,
            payload: pair(),
            payers: vec![],
            debtors: vec![],
            history: None,
            client_op_id: Some(id(n)),
        }),
        None,
    )
}

pub fn edit(n: u128, project: Uuid, row: i32) -> Value {
    op(
        n,
        OpKind::EditExpense(shared::EditableExpense { id: row, project_id: project, author_id: None, payload: pair(), payers: vec![], debtors: vec![], history: None, eid: None, expected_rev: None }),
        None,
    )
}

pub fn delete(n: u128, project: Uuid, row: i32, after: Option<Uuid>) -> Value {
    op(n, OpKind::DeleteExpense(shared::DeleteExpenseRequest { id: row, project_id: project, history: None, eid: None, expected_rev: None }), after)
}

fn remove_op(n: u128, project: Uuid, row: Option<i64>) -> Value {
    json!({ "id": id(n), "label": "", "op": { "type": "remove", "project_id": project, "slot": "expense", "eid": null, "row": row, "base_rev": null, "base": null } })
}

pub fn with(entries: &[Value]) -> MemStore {
    let mut store = MemStore::new();
    for raw in entries {
        assert!(write_entries(&mut store, std::slice::from_ref(raw)));
    }
    store
}

fn put(store: &mut MemStore, key: String, value: Value) {
    store.map.insert(key, value.to_string());
}

fn queue(store: &mut MemStore) -> Vec<Entry> {
    Snapshot::read(store).queue()
}

fn statuses(store: &mut MemStore) -> Vec<(Uuid, Status)> {
    Snapshot::read(store).entries().iter().map(|e| (e.id.unwrap(), e.status)).collect()
}

fn ids(store: &mut MemStore) -> Vec<Uuid> {
    queue(store).iter().map(|e| e.id.unwrap()).collect()
}

fn next(store: &mut MemStore) -> Option<Uuid> {
    let queue = queue(store);
    next_to_send(&queue, |_| false).map(|i| queue[i].id.unwrap())
}

#[test]
fn a_key_form_this_build_does_not_know_is_kept() {
    let mut store = with(&[add(1, P)]);
    let odd = ["counted_q.e.NOT-A-UUID", "counted_q.w.1", "counted_q.l.00000000-0000-0000-0000-000000001001.-1", "counted_q.z.01"];
    for key in odd {
        put(&mut store, key.into(), json!({}));
    }
    let snap = Snapshot::read(&mut store);
    tidy(&mut store, &snap);
    assert!(odd.iter().all(|k| store.map.contains_key(*k)));
    assert_eq!(queue(&mut store).len(), 1);
}

/// I3, Q2a: a key is live unless a marker covers its generation or a `z` or `p` key covers its
/// `seq`; the live version is the one of greatest `gen`, then `seq`, then key.
#[test]
fn liveness_and_the_live_version_follow_generations_and_seqs() {
    let a = id(1);
    let version = |gen: i64, seq: i64| json!({ "id": a, "gen": gen, "seq": seq, "op": { "type": "write", "project_id": P } });
    let v1 = format!("{PREFIX}v.{a}.{}", Uuid::from_u128(1));
    let v2 = format!("{PREFIX}v.{a}.{}", Uuid::from_u128(2));
    let mark = |kind: &str, g: i64| marker_key(kind, a, Some(g));
    type Case = (&'static str, Vec<(String, Value)>, Option<(i64, &'static str)>);
    let cases: Vec<Case> = vec![
        ("alone", vec![], Some((0, "e"))),
        ("landed", vec![(mark("l", 0), json!({ "seq": 9 }))], None),
        ("left", vec![(mark("x", 0), json!({ "seq": 9 }))], None),
        ("replaced", vec![(mark("s", 0), json!({ "seq": 9, "by": id(9) }))], None),
        ("a later version outlives a landing of 0", vec![(v1.clone(), version(1, 1_000_001)), (mark("l", 0), json!({ "seq": 9 }))], Some((1, "v"))),
        ("a marker of 1 covers it", vec![(v1.clone(), version(1, 1_000_001)), (mark("x", 1), json!({ "seq": 9 }))], None),
        ("a marker of 3 covers 0 and 1", vec![(v1.clone(), version(1, 1_000_001)), (mark("l", 3), json!({ "seq": 9 }))], None),
        ("erased at its seq", vec![(format!("{PREFIX}z.1000000"), json!({}))], None),
        ("erased before it", vec![(format!("{PREFIX}z.999999"), json!({}))], Some((0, "e"))),
        ("its project forgotten at its seq", vec![(format!("{PREFIX}p.{P}.1000000"), json!({}))], None),
        ("its project forgotten before it", vec![(format!("{PREFIX}p.{P}.999999"), json!({}))], Some((0, "e"))),
        ("another project forgotten", vec![(format!("{PREFIX}p.{Q}.9000000"), json!({}))], Some((0, "e"))),
        ("the greater seq at one gen", vec![(v1.clone(), version(2, 7)), (v2.clone(), version(2, 8))], Some((2, "v2"))),
        ("the greater key at one gen and seq", vec![(v1.clone(), version(2, 8)), (v2.clone(), version(2, 8))], Some((2, "v2"))),
    ];
    for (name, keys, expected) in cases {
        let mut store = with(&[add(1, P)]);
        for (key, value) in keys {
            put(&mut store, key, value);
        }
        let snap = Snapshot::read(&mut store);
        let live = snap.live(a).map(|v| (v.gen, if v.key == v2 { "v2" } else if v.key.contains(".v.") { "v" } else { "e" }));
        assert_eq!(live, expected, "{name}");
    }
}

#[test]
fn a_version_is_held_and_an_unreadable_project_ignores_p_keys() {
    let a = id(1);
    let mut store = MemStore::new();
    let edit_op = json!({ "type": "edit_expense", "id": 3, "projectId": P, "payload": { "ct": "a", "iv": "b" } });
    put(&mut store, format!("{PREFIX}v.{a}.{}", Uuid::from_u128(1)), json!({ "id": a, "gen": 1, "seq": 5, "op": edit_op }));
    put(&mut store, entry_key(id(2)), json!({ "id": id(2), "seq": 6, "op": { "type": "mystery" } }));
    put(&mut store, format!("{PREFIX}p.{P}.1"), json!({}));
    assert_eq!(statuses(&mut store), vec![(a, Status::Held), (id(2), Status::Held)], "a v key is held whatever its type");
    put(&mut store, entry_key(id(3)), json!({ "id": id(4), "seq": 7, "op": { "type": "delete_expense", "id": 3, "projectId": P } }));
    assert_eq!(statuses(&mut store)[2], (id(3), Status::Held), "an e key whose value names another id");
}

/// I4, Q3: an entry naming another in `after` is resolved by the markers of the greatest generation
/// of the named id: s before x before l at one generation, i last.
#[test]
fn after_is_resolved_by_the_markers_of_the_greatest_generation() {
    let t = id(1);
    let n = id(9);
    let mark = |kind: &str, g: Option<i64>| marker_key(kind, t, g);
    let seq = || json!({ "seq": 1 });
    type Case = (&'static str, bool, Vec<(String, Value)>, Status);
    let cases: Vec<Case> = vec![
        ("the named entry is live", true, vec![], Status::Waits),
        ("it landed", false, vec![(mark("l", Some(0)), seq())], Status::Ready),
        ("it left", false, vec![(mark("x", Some(0)), seq())], Status::Leaves),
        ("nothing of it is stored", false, vec![], Status::Stuck),
        ("landed before 3.4.0", false, vec![(mark("i", None), seq())], Status::Ready),
        ("i comes last", false, vec![(mark("i", None), seq()), (mark("x", Some(0)), seq())], Status::Leaves),
        ("an i key never ends a live entry", true, vec![(mark("i", None), seq())], Status::Waits),
        ("x at 1 over l at 0", false, vec![(mark("l", Some(0)), seq()), (mark("x", Some(1)), seq())], Status::Leaves),
        ("l at 1 over x at 0", false, vec![(mark("x", Some(0)), seq()), (mark("l", Some(1)), seq())], Status::Ready),
        ("l before x at one generation", false, vec![(mark("l", Some(2)), seq()), (mark("x", Some(2)), seq())], Status::Ready),
        ("s before x, its replacement unknown", false, vec![(mark("x", Some(0)), seq()), (mark("s", Some(0)), json!({ "seq": 1, "by": n }))], Status::Stuck),
        ("s before l, its replacement unknown", false, vec![(mark("l", Some(0)), seq()), (mark("s", Some(0)), json!({ "seq": 1, "by": n }))], Status::Stuck),
        ("s, its replacement landed", false, vec![(mark("s", Some(0)), json!({ "seq": 1, "by": n })), (marker_key("l", n, Some(0)), seq())], Status::Ready),
        ("s, its replacement left", false, vec![(mark("s", Some(0)), json!({ "seq": 1, "by": n })), (marker_key("x", n, Some(0)), seq())], Status::Leaves),
        ("s with no readable by", false, vec![(mark("s", Some(0)), seq())], Status::Stuck),
    ];
    for (name, target_live, keys, expected) in cases {
        let mut store = if target_live { with(&[add(1, Q), delete(2, P, 7, Some(t))]) } else { with(&[delete(2, P, 7, Some(t))]) };
        for (key, value) in keys {
            put(&mut store, key, value);
        }
        let found = statuses(&mut store).into_iter().find(|(i, _)| *i == id(2)).map(|(_, s)| s);
        assert_eq!(found, Some(expected), "{name}");
    }
}

#[test]
fn after_follows_a_live_replacement_and_a_dropped_or_leaving_target() {
    let t = id(1);
    let mut store = with(&[add(9, Q), delete(2, P, 7, Some(t))]);
    put(&mut store, marker_key("s", t, Some(0)), json!({ "seq": 1, "by": id(9) }));
    assert_eq!(statuses(&mut store)[1].1, Status::Waits, "waits on the creation named in by");

    let mut store = with(&[add(1, Q), delete(2, P, 7, Some(t))]);
    put(&mut store, format!("{PREFIX}p.{Q}.1000001"), json!({}));
    assert_eq!(statuses(&mut store), vec![(id(2), Status::Leaves)], "a target no longer live through a p key counts as left");

    let mut store = with(&[add(1, Q), delete(2, P, 7, Some(t))]);
    put(&mut store, format!("{PREFIX}z.1000000"), json!({}));
    assert_eq!(statuses(&mut store), vec![(id(2), Status::Leaves)], "and through a z key");

    let mut store = with(&[delete(3, Q, 1, Some(id(8))), delete(1, Q, 2, Some(id(3))), delete(2, P, 7, Some(t))]);
    put(&mut store, marker_key("x", id(8), Some(0)), json!({ "seq": 1 }));
    assert!(statuses(&mut store).iter().all(|(_, s)| *s == Status::Leaves), "down a chain, whatever the order");

    let mut store = with(&[delete(1, P, 1, Some(id(2))), delete(2, P, 2, Some(id(1)))]);
    let found: Vec<Status> = statuses(&mut store).into_iter().map(|(_, s)| s).collect();
    assert_eq!(found, vec![Status::Stuck, Status::Stuck], "a cycle waits, each of it listed");
}

/// Q3 step 2: a key of the named id that a `p` or `z` key ended counts as an `x` at its generation,
/// so an older landing does not replay the dependent, and a landing at that same generation does.
#[test]
fn a_named_version_a_p_key_ended_counts_as_left_at_its_generation() {
    let t = id(1);
    let later = |seq: i64| json!({ "id": t, "gen": 1, "seq": seq, "op": { "type": "write", "project_id": Q } });
    type Case = (&'static str, Vec<(String, Value)>, Status);
    let cases: Vec<Case> = vec![
        ("its gen 1 ended, gen 0 landed", vec![(format!("{PREFIX}v.{t}.{}", Uuid::from_u128(1)), later(5)), (marker_key("l", t, Some(0)), json!({ "seq": 1 }))], Status::Leaves),
        ("its gen 0 ended and landed", vec![(entry_key(t), { let mut a = add(1, Q); a["seq"] = json!(5); a }), (marker_key("l", t, Some(0)), json!({ "seq": 1 }))], Status::Ready),
        ("its gen 0 ended, nothing else", vec![(entry_key(t), { let mut a = add(1, Q); a["seq"] = json!(5); a })], Status::Leaves),
    ];
    for (name, keys, expected) in cases {
        let mut store = with(&[delete(2, P, 7, Some(t))]);
        put(&mut store, format!("{PREFIX}p.{Q}.10"), json!({}));
        for (key, value) in keys {
            put(&mut store, key, value);
        }
        assert_eq!(statuses(&mut store).into_iter().find(|(i, _)| *i == id(2)).map(|(_, s)| s), Some(expected), "{name}");
    }
}

fn status_edit(n: u128, project: Uuid) -> Value {
    op(n, OpKind::EditProject(ProjectEdit { project_id: project, name_description: None, status: Some(shared::ProjectStatus::Closed), history: None, emoji: None }), None)
}

/// Drains `store` against a server holding `rows` (row id to project and the expense it is), each
/// add copying the expense `copies` names, until nothing can be sent.
fn drain_against(store: &mut MemStore, rows: &mut BTreeMap<i64, (Uuid, &'static str)>, copies: &HashMap<Uuid, &'static str>) {
    for _ in 0..20 {
        let queue = queue(store);
        let Some(entry) = next_to_send(&queue, |_| false).map(|i| queue[i].clone()) else { return };
        let row = match entry.op.as_ref().map(|q| &q.op) {
            Some(OpKind::AddExpense(c)) => {
                let n = rows.keys().max().copied().unwrap_or(0) + 1;
                rows.insert(n, (c.project_id, copies[&entry.id.unwrap()]));
                Some(n)
            }
            Some(OpKind::DeleteExpense(d)) => {
                rows.remove(&i64::from(d.id));
                None
            }
            _ => None,
        };
        let snap = Snapshot::read(store);
        assert!(land(store, &snap, &entry, row));
    }
    panic!("the drain did not end");
}

/// Q4, Option B: an add passes a waiting delete, so two moves made at once in two tabs, in opposite
/// directions, both complete, and each expense ends in exactly one project; a status change of each
/// project queued between them changes nothing. Replaces the test that listed them as stuck.
#[test]
fn two_crossing_moves_both_complete_with_each_expense_in_one_project() {
    for edits in [false, true] {
        let mut entries = vec![delete(1, P, 7, Some(id(4))), delete(2, Q, 8, Some(id(3)))];
        if edits {
            entries.extend([status_edit(5, P), status_edit(6, Q)]);
        }
        entries.extend([add(3, P), add(4, Q)]);
        let mut store = with(&entries);
        let mut rows: BTreeMap<i64, (Uuid, &'static str)> = [(7, (P, "A")), (8, (Q, "B"))].into();
        let copies: HashMap<Uuid, &'static str> = [(id(3), "B"), (id(4), "A")].into();
        drain_against(&mut store, &mut rows, &copies);
        assert!(queue(&mut store).is_empty(), "edits {edits}: {:?}", statuses(&mut store));
        for (name, project) in [("A", Q), ("B", P)] {
            let found: Vec<Uuid> = rows.values().filter(|(_, n)| *n == name).map(|(p, _)| *p).collect();
            assert_eq!(found, vec![project], "edits {edits}: {name}");
        }
    }
}

/// Q4: a held entry stops the entries behind it for its project only, row by row.
#[test]
fn a_held_entry_stops_its_project_and_the_others_drain() {
    assert_eq!(next(&mut with(&[remove_op(1, P, None), add(2, P), edit(3, P, 3), add(4, Q)])), Some(id(4)));
    assert_eq!(next(&mut with(&[add(1, P), remove_op(2, P, None)])), Some(id(1)), "nothing ahead of it waits");
}

#[test]
fn a_held_entry_lets_a_later_one_about_another_row_pass() {
    assert_eq!(next(&mut with(&[remove_op(1, P, Some(4)), edit(2, P, 5)])), Some(id(2)));
    assert_eq!(next(&mut with(&[remove_op(1, P, Some(4)), add(2, P)])), Some(id(2)), "a creation and an entry that names a row");
    for behind in [edit(2, P, 4), delete(2, P, 4, None), status_edit(2, P)] {
        assert_eq!(next(&mut with(&[remove_op(1, P, Some(4)), behind])), None);
    }
    assert_eq!(next(&mut with(&[remove_op(1, P, None), edit(2, P, 5)])), None, "a held entry that names no row stops every one behind it");
}

#[test]
fn an_entry_waits_on_the_one_it_names_in_after_and_stops_its_project() {
    assert_eq!(next(&mut with(&[remove_op(1, Q, Some(1)), delete(2, P, 7, Some(id(1))), edit(3, P, 8), edit(4, P, 7)])), Some(id(3)));
    let mut store = with(&[add(1, Q), delete(2, P, 7, Some(id(1)))]);
    assert_eq!(next(&mut store), Some(id(1)));
    let queue = queue(&mut store);
    assert_eq!(next_to_send(&queue, |e| e.id == Some(id(1))), None, "the add held now holds its delete too");
}

/// Q4: only an entry stopped for its own reason stops others. A status change stopped behind a
/// waiting delete does not stop an add behind it, which is about another row than the delete.
#[test]
fn an_entry_stopped_by_another_stops_nothing_itself() {
    assert_eq!(next(&mut with(&[add(1, Q), delete(2, P, 7, Some(id(1))), status_edit(3, P), add(4, P)])), Some(id(1)));
    let queue = queue(&mut with(&[add(1, Q), delete(2, P, 7, Some(id(1))), status_edit(3, P), add(4, P)]));
    assert_eq!(next_to_send(&queue, |e| e.id == Some(id(1))).map(|i| queue[i].id), Some(Some(id(4))));
}

#[test]
fn an_entry_without_a_readable_project_stops_every_one_behind_it() {
    let blind = json!({ "id": id(1), "op": { "type": "mystery" } });
    assert_eq!(next(&mut with(&[blind.clone(), add(2, Q), add(3, P)])), None);
    assert_eq!(next(&mut with(&[add(2, Q), blind])), Some(id(2)));
}

#[test]
fn an_entry_held_at_the_replay_stops_only_its_project() {
    let mut store = with(&[add(1, P), add(2, P), add(3, Q)]);
    let queue = queue(&mut store);
    assert_eq!(next_to_send(&queue, |e| e.id == Some(id(1))).map(|i| queue[i].id), Some(Some(id(3))));
}

#[test]
fn the_queue_is_in_the_order_of_seq_then_id_and_seq_only_grows() {
    let mut store = MemStore::new();
    for (n, seq) in [(3, 5), (1, 9), (2, 5)] {
        let mut raw = add(n, P);
        raw["seq"] = json!(seq);
        put(&mut store, entry_key(id(n)), raw);
    }
    store.now = 0;
    assert_eq!(ids(&mut store), vec![id(2), id(3), id(1)]);
    assert_eq!(next_seq(&mut store), 10, "one more than the greatest seq read");
    store.now = 50;
    assert_eq!(next_seq(&mut store), 50, "or the time, whichever is greater");
}

/// Q2b: a landing marks first and removes after; the row of a creation is in its `l` key.
#[test]
fn a_landing_writes_its_marker_then_removes_the_keys() {
    let mut store = with(&[add(1, P), delete(2, P, 7, Some(id(1)))]);
    let snap = Snapshot::read(&mut store);
    assert!(land(&mut store, &snap, &snap.queue()[0], Some(42)));
    assert!(!store.map.contains_key(&entry_key(id(1))));
    let snap = Snapshot::read(&mut store);
    assert_eq!(snap.landed_row(id(1)), Some(42));
    assert_eq!(snap.queue().iter().map(|e| (e.id.unwrap(), e.status)).collect::<Vec<_>>(), vec![(id(2), Status::Ready)], "and what waits on it is replayed");
}

/// Q2c: a marker that cannot be written leaves the keys as they are.
#[test]
fn a_marker_that_throws_changes_nothing() {
    let mut store = with(&[add(1, P)]);
    store.refuse = Some(format!("{PREFIX}l."));
    let snap = Snapshot::read(&mut store);
    assert!(!land(&mut store, &snap, &snap.queue()[0], None));
    assert_eq!(queue(&mut store).len(), 1, "sent again later");
}

/// Q2d: the entries of one handler are written together, or none is.
#[test]
fn a_later_write_that_throws_removes_what_the_handler_wrote() {
    let mut store = MemStore::new();
    store.refuse = Some(entry_key(id(2)));
    assert!(!write_entries(&mut store, &[delete(1, P, 7, Some(id(2))), add(2, Q)]));
    assert!(store.map.is_empty());
    store.refuse = None;
    assert!(write_entries(&mut store, &[delete(1, P, 7, Some(id(2))), add(2, Q)]));
    assert_eq!(statuses(&mut store), vec![(id(1), Status::Waits), (id(2), Status::Ready)]);
}

/// Q3: a discard takes what waits on it, by rule, and the tidy writes their `x` keys.
#[test]
fn a_discarded_entry_takes_what_waits_on_it() {
    let mut store = with(&[add(1, Q), delete(2, P, 7, Some(id(1))), delete(3, P, 8, Some(id(2))), add(4, P)]);
    let snap = Snapshot::read(&mut store);
    assert!(leave(&mut store, &snap, &snap.queue()[0]));
    assert_eq!(ids(&mut store), vec![id(4)]);
    let snap = Snapshot::read(&mut store);
    tidy(&mut store, &snap);
    for n in 2..=3 {
        assert!(store.map.contains_key(&marker_key("x", id(n), Some(0))) && !store.map.contains_key(&entry_key(id(n))));
    }
}

/// Q3: a device that forgets a project writes its `p` key; a move into it leaves the expense where
/// it was, and an entry made afterwards for that project is live.
#[test]
fn a_forgotten_project_takes_its_entries_and_what_waits_on_them() {
    let mut store = with(&[add(1, Q), delete(2, P, 7, Some(id(1))), edit(3, Q, 4), edit(4, P, 9)]);
    assert_eq!(Snapshot::read(&mut store).dropped_with(Q), 3);
    let snap = Snapshot::read(&mut store);
    assert!(forget(&mut store, &snap, Q));
    assert_eq!(ids(&mut store), vec![id(4)]);
    assert!(write_entries(&mut store, &[add(5, Q)]));
    assert_eq!(ids(&mut store), vec![id(4), id(5)]);
}

/// Q3: the erase leaves its `z` key and the `o` keys; an entry a stale tab made before it is not
/// live, one made after it is.
#[test]
fn an_erase_keeps_only_the_o_keys_and_its_z_key() {
    let mut store = with(&[add(1, P)]);
    put(&mut store, marker_key("o", id(7), None), json!({ "seq": 1 }));
    put(&mut store, marker_key("l", id(8), Some(0)), json!({ "seq": 1 }));
    put(&mut store, OLD_KEY.into(), json!([]));
    erase(&mut store);
    let left = store.keys();
    assert_eq!(left.len(), 2, "{left:?}");
    assert!(left.iter().any(|k| k.starts_with(&format!("{PREFIX}o."))) && left.iter().any(|k| k.starts_with(&format!("{PREFIX}z."))));
    let mut late = add(2, P);
    late["seq"] = json!(1);
    put(&mut store, entry_key(id(2)), late);
    assert!(queue(&mut store).is_empty());
    assert!(write_entries(&mut store, &[add(3, P)]));
    assert_eq!(queue(&mut store).len(), 1);
}

/// Q2, Q3: an erase reads the `p` and `z` keys stored before it draws its `seq`, so it never names
/// a `z` key another erase wrote in the same millisecond; and when its `z` key cannot be written at
/// first, the one it writes at the end takes a new `seq`. Before, the second case wrote again a key
/// it had read as present.
#[test]
fn an_erase_never_writes_a_z_key_that_is_stored() {
    let mut store = with(&[add(1, P)]);
    store.map.insert(format!("{PREFIX}z.{}", store.now + 5), "{}".into());
    erase(&mut store);
    let above = format!("{PREFIX}z.{}", store.now + 6);
    assert_eq!(store.keys(), vec![above], "a z key above the one stored, which goes with the rest");

    let mut store = with(&[add(1, P)]);
    store.refuse = Some(format!("{PREFIX}z."));
    store.refusals = Some(1);
    let stored = next_seq(&mut store) + 1;
    store.map.insert(format!("{PREFIX}z.{stored}"), "{}".into());
    erase(&mut store);
    assert_eq!(store.keys(), vec![format!("{PREFIX}z.{}", stored + 2)], "refused once: a new seq, above the one stored");
}

/// Q2a, Q2b: the tidy removes what is not live, and markers older than 30 days nothing names.
#[test]
fn the_tidy_removes_what_is_not_live_and_markers_older_than_30_days() {
    let mut store = with(&[add(1, P), delete(2, P, 7, Some(id(5)))]);
    put(&mut store, marker_key("l", id(1), Some(0)), json!({ "seq": 1 }));
    put(&mut store, marker_key("l", id(4), Some(0)), json!({ "seq": 1 }));
    put(&mut store, marker_key("l", id(5), Some(0)), json!({ "seq": 1 }));
    let recent = store.now + MARKERS_KEPT_MS;
    put(&mut store, marker_key("x", id(6), Some(0)), json!({ "seq": recent }));
    store.now += MARKERS_KEPT_MS + 10;
    let snap = Snapshot::read(&mut store);
    tidy(&mut store, &snap);
    assert!(!store.map.contains_key(&entry_key(id(1))), "a landed entry's key");
    assert!(store.map.contains_key(&marker_key("l", id(1), Some(0))), "its marker goes only once nothing of its id is stored");
    assert!(!store.map.contains_key(&marker_key("l", id(4), Some(0))));
    assert!(store.map.contains_key(&marker_key("l", id(5), Some(0))), "a stored entry names it");
    assert!(store.map.contains_key(&marker_key("x", id(6), Some(0))), "30 days are not over");
}

/// Q2b: a marker reached through an `s` key's `by` is kept while an entry names the replaced id.
#[test]
fn the_tidy_keeps_a_marker_an_entry_reaches_through_by() {
    let mut store = with(&[delete(2, P, 7, Some(id(1)))]);
    put(&mut store, marker_key("s", id(1), Some(0)), json!({ "seq": 1, "by": id(9) }));
    put(&mut store, marker_key("s", id(9), Some(0)), json!({ "seq": 1, "by": id(8) }));
    put(&mut store, marker_key("l", id(8), Some(0)), json!({ "seq": 1, "row": 4 }));
    store.now += MARKERS_KEPT_MS + 10;
    let snap = Snapshot::read(&mut store);
    tidy(&mut store, &snap);
    for kept in [marker_key("s", id(1), Some(0)), marker_key("s", id(9), Some(0)), marker_key("l", id(8), Some(0))] {
        assert!(store.map.contains_key(&kept), "{kept}");
    }
    assert_eq!(statuses(&mut store), vec![(id(2), Status::Ready)]);
}

fn old_entry(n: u128, after: Option<Uuid>) -> Value {
    json!({ "id": id(n), "label": "", "sealed_label": null, "after": after, "op": { "type": "delete_expense", "id": n, "projectId": P } })
}

/// I6, Q0: each entry of the old key moves once, however often an older build writes it back.
#[test]
fn entries_under_the_old_key_move_once_each() {
    let mut store = with(&[add(9, Q)]);
    put(&mut store, OLD_KEY.into(), json!([old_entry(1, None), old_entry(2, None)]));
    let snap = Snapshot::read(&mut store);
    assert_eq!(move_old(&mut store, &snap), Moved::Whole);
    assert_eq!(store.map[OLD_KEY], EMPTIED);
    assert_eq!(ids(&mut store), vec![id(9), id(1), id(2)], "after what was queued");

    let snap = Snapshot::read(&mut store);
    land(&mut store, &snap, &snap.queue()[1], None);
    put(&mut store, OLD_KEY.into(), json!([old_entry(1, None), old_entry(2, None), old_entry(3, None)]));
    let snap = Snapshot::read(&mut store);
    move_old(&mut store, &snap);
    assert_eq!(ids(&mut store), vec![id(9), id(2), id(3)], "a landed entry is not moved again");
    assert_eq!(store.map[OLD_KEY], EMPTIED);
}

/// I6, Q0: a dependent whose target is in neither the old key nor the prefix gets an `i` key for it;
/// one whose target is still in the old key does not.
#[test]
fn a_moved_dependent_of_an_unknown_target_is_replayed_as_landed_before() {
    let mut store = MemStore::new();
    put(&mut store, OLD_KEY.into(), json!([old_entry(2, Some(id(1))), old_entry(4, Some(id(3))), old_entry(3, None)]));
    let snap = Snapshot::read(&mut store);
    move_old(&mut store, &snap);
    assert!(store.map.contains_key(&marker_key("i", id(1), None)));
    assert!(!store.map.contains_key(&marker_key("i", id(3), None)));
    assert_eq!(statuses(&mut store), vec![(id(2), Status::Ready), (id(4), Status::Waits), (id(3), Status::Ready)]);
}

/// I6: the old key is not emptied while an `o` key is missing; the next move writes it without
/// moving the entry twice.
#[test]
fn the_old_key_waits_for_every_o_key() {
    let mut store = MemStore::new();
    let old = json!([old_entry(1, None), old_entry(2, None)]);
    put(&mut store, OLD_KEY.into(), old.clone());
    store.refuse = Some(marker_key("o", id(2), None));
    let snap = Snapshot::read(&mut store);
    move_old(&mut store, &snap);
    assert_eq!(store.map[OLD_KEY], old.to_string(), "not emptied");
    store.refuse = None;
    let snap = Snapshot::read(&mut store);
    move_old(&mut store, &snap);
    assert_eq!(store.map[OLD_KEY], EMPTIED);
    assert_eq!(queue(&mut store).len(), 2);
}

struct Racing(MemStore);

impl Store for Racing {
    fn get(&self, key: &str) -> Option<String> {
        self.0.get(key)
    }
    fn set(&mut self, key: &str, value: &str) -> bool {
        let done = self.0.set(key, value);
        if key.starts_with(&format!("{PREFIX}o.")) {
            self.0.map.insert(OLD_KEY.into(), json!([old_entry(1, None), old_entry(5, None)]).to_string());
        }
        done
    }
    fn remove(&mut self, key: &str) {
        self.0.remove(key)
    }
    fn all(&self) -> Vec<(String, String)> {
        self.0.all()
    }
    fn now_ms(&self) -> i64 {
        self.0.now_ms()
    }
    fn seen_seq(&self) -> i64 {
        self.0.seen_seq()
    }
    fn saw_seq(&mut self, seq: i64) {
        self.0.saw_seq(seq)
    }
    fn kept(&self) -> Option<String> {
        self.0.kept()
    }
    fn set_kept(&mut self, text: Option<String>) {
        self.0.set_kept(text)
    }
}

/// Q0: an older build that writes its copy back while the move runs keeps the old key as it wrote
/// it; the entry it added moves on the next run.
#[test]
fn the_old_key_is_left_when_it_changed_since_it_was_read() {
    let mut store = Racing(MemStore::new());
    put(&mut store.0, OLD_KEY.into(), json!([old_entry(1, None)]));
    let snap = Snapshot::read(&mut store);
    move_old(&mut store, &snap);
    assert!(store.0.map[OLD_KEY].contains(&id(5).to_string()), "kept");
    let snap = Snapshot::read(&mut store);
    move_old(&mut store, &snap);
    assert_eq!(Snapshot::read(&mut store).queue().len(), 2);
}

/// I5: every operation cut after each storage call leaves the state before or the state after. A
/// move cut between its two writes leaves its delete waiting on nothing: listed, never sent.
#[test]
fn an_operation_cut_after_any_storage_call_leaves_before_or_after() {
    type Op = fn(&mut MemStore);
    let ops: Vec<(&str, Vec<Value>, Op)> = vec![
        ("enqueue", vec![add(1, P)], |s| {
            write_entries(s, &[add(2, P)]);
        }),
        ("move", vec![add(1, P)], |s| {
            write_entries(s, &[delete(3, P, 7, Some(id(2))), add(2, Q)]);
        }),
        ("landing", vec![add(1, P), delete(2, P, 7, Some(id(1)))], |s| {
            let snap = Snapshot::read(s);
            land(s, &snap, &snap.queue()[0], Some(5));
        }),
        ("discard", vec![add(1, P), delete(2, P, 7, Some(id(1)))], |s| {
            let snap = Snapshot::read(s);
            leave(s, &snap, &snap.queue()[0]);
        }),
        ("project drop", vec![add(1, P), add(2, Q), edit(3, P, 1)], |s| {
            let snap = Snapshot::read(s);
            forget(s, &snap, P);
        }),
        ("erase", vec![add(1, P), add(2, Q)], |s| erase(s)),
        ("tidy", vec![add(1, P), delete(2, P, 7, Some(id(9)))], |s| {
            let snap = Snapshot::read(s);
            tidy(s, &snap);
        }),
    ];
    for (name, setup, op) in ops {
        let prepare = || {
            let mut store = with(&setup);
            if name == "tidy" {
                put(&mut store, marker_key("x", id(9), Some(0)), json!({ "seq": 1 }));
            }
            store
        };
        let mut full = prepare();
        let before = statuses(&mut full);
        let start = full.changes;
        op(&mut full);
        let after = statuses(&mut full);
        for cut in 0..=(full.changes - start) {
            let mut store = prepare();
            store.writes_left = Some(cut);
            op(&mut store);
            store.writes_left = None;
            let state = statuses(&mut store);
            let half_move = name == "move" && state == vec![(id(1), Status::Ready), (id(3), Status::Stuck)];
            assert!(state == before || state == after || half_move, "{name} cut after {cut} calls: {state:?}");
        }
    }
}

/// I5 for Q0: a move cut anywhere, run again, ends with each entry once and the old key empty.
#[test]
fn an_old_key_move_cut_anywhere_ends_whole_on_the_next_run() {
    let old = json!([old_entry(2, Some(id(1))), old_entry(3, None)]);
    let mut full = MemStore::new();
    put(&mut full, OLD_KEY.into(), old.clone());
    let snap = Snapshot::read(&mut full);
    move_old(&mut full, &snap);
    let whole = statuses(&mut full);
    for cut in 0..full.changes {
        let mut store = MemStore::new();
        put(&mut store, OLD_KEY.into(), old.clone());
        store.writes_left = Some(cut);
        let snap = Snapshot::read(&mut store);
        move_old(&mut store, &snap);
        store.writes_left = None;
        let snap = Snapshot::read(&mut store);
        move_old(&mut store, &snap);
        assert_eq!(statuses(&mut store), whole, "cut after {cut}");
        assert_eq!(store.map[OLD_KEY], EMPTIED);
    }
}

#[test]
fn a_key_read_as_present_and_written_again_fails_the_fake() {
    let result = std::panic::catch_unwind(|| {
        let mut store = with(&[add(1, P)]);
        Snapshot::read(&mut store);
        store.set(&entry_key(id(1)), "{}");
    });
    assert!(result.is_err());
}

/// N2: the old key's entries not moved yet carry no `seq`; each gets one from its place in the old
/// array, so their rows sort in that order and none shares a date with another.
#[test]
fn entries_not_moved_yet_take_a_seq_from_their_place() {
    let mut store = MemStore::new();
    let old = json!([old_entry(1, None), old_entry(2, None), old_entry(3, None)]).to_string();
    let seqs: Vec<i64> = Snapshot::read(&mut store).not_moved(Some(old), 5_000).iter().map(|e| e.seq).collect();
    assert!(seqs.windows(2).all(|w| w[0] < w[1]) && seqs.iter().all(|s| *s < i64::MAX), "{seqs:?}");
}

fn stamp_key(n: u128, seq: i64) -> String {
    format!("{PREFIX}t.{}.{seq}", id(n))
}

/// Q0, Q2a: an entry moved twice with two stamps takes the smaller as its `seq`, in the order of the
/// queue and against a `p` key, whichever `seq` its `e` key holds.
#[test]
fn an_entry_with_two_stamps_takes_the_smaller_whichever_e_value_is_stored() {
    for e_seq in [100, 300] {
        let mut store = MemStore::new();
        let mut moved = old_entry(1, None);
        moved["seq"] = json!(e_seq);
        put(&mut store, entry_key(id(1)), moved);
        put(&mut store, stamp_key(1, 300), json!({}));
        put(&mut store, stamp_key(1, 100), json!({}));
        put(&mut store, marker_key("o", id(1), None), json!({ "seq": 301 }));
        let mut later = edit(2, P, 1);
        later["seq"] = json!(200);
        put(&mut store, entry_key(id(2)), later);
        assert_eq!(ids(&mut store), vec![id(1), id(2)], "e holds {e_seq}");
        assert_eq!(queue(&mut store)[0].seq, 100);
        put(&mut store, format!("{PREFIX}p.{P}.150"), json!({}));
        assert_eq!(ids(&mut store), vec![id(2)], "a p key at 150 ends it, e holds {e_seq}");
    }
}

/// Q0: a move that reads a stamp of the entry but no `e` key, its first move cut between the two,
/// writes the `e` key with that stamp and no stamp of its own.
#[test]
fn a_move_that_reads_a_stamp_writes_e_with_it_and_no_new_stamp() {
    let mut store = MemStore::new();
    put(&mut store, OLD_KEY.into(), json!([old_entry(1, None)]));
    put(&mut store, stamp_key(1, 500), json!({}));
    store.now = 9_000;
    let snap = Snapshot::read(&mut store);
    move_old(&mut store, &snap);
    let e: Value = serde_json::from_str(&store.map[&entry_key(id(1))]).unwrap();
    assert_eq!(e["seq"], json!(500));
    let stamps: Vec<String> = store.keys().into_iter().filter(|k| k.starts_with(&format!("{PREFIX}t."))).collect();
    assert_eq!(stamps, vec![stamp_key(1, 500)]);
    assert_eq!(store.map[OLD_KEY], EMPTIED);
}

/// Q0: the stamps of an id go once none of its `e` or `v` keys is stored and its `o` key is; an
/// erase removes them with the rest and keeps the `o` keys.
#[test]
fn stamps_go_once_no_entry_key_is_stored_and_o_is() {
    let mut store = MemStore::new();
    put(&mut store, OLD_KEY.into(), json!([old_entry(1, None)]));
    let snap = Snapshot::read(&mut store);
    move_old(&mut store, &snap);
    put(&mut store, stamp_key(2, 7), json!({}));
    let stamps = |store: &MemStore| store.keys().into_iter().filter(|k| k.starts_with(&format!("{PREFIX}t."))).count();
    assert_eq!(stamps(&store), 2);
    let snap = Snapshot::read(&mut store);
    tidy(&mut store, &snap);
    assert_eq!(stamps(&store), 2, "an e key of 1 is stored, and 2 has no o key");
    let snap = Snapshot::read(&mut store);
    land(&mut store, &snap, &snap.queue()[0], None);
    let snap = Snapshot::read(&mut store);
    tidy(&mut store, &snap);
    assert_eq!(store.keys().into_iter().filter(|k| k.starts_with(&format!("{PREFIX}t."))).collect::<Vec<_>>(), vec![stamp_key(2, 7)]);
    erase(&mut store);
    assert_eq!(stamps(&store), 0);
    assert!(store.map.contains_key(&marker_key("o", id(1), None)));
}

/// Q0: an entry the old key no longer holds, its write replaced by another tab's `[ ]`, moves from
/// the new value of the `storage` event this tab received; the text is kept until it moved whole.
#[test]
fn an_entry_only_in_a_received_event_moves() {
    let mut store = MemStore::new();
    store.map.insert(OLD_KEY.into(), EMPTIED.into());
    store.kept = Some(json!([old_entry(1, None)]).to_string());
    store.refuse = Some(format!("{PREFIX}o."));
    let snap = Snapshot::read(&mut store);
    assert_eq!(move_old(&mut store, &snap), Moved::Partly);
    assert!(store.kept.is_some(), "kept while its o key is missing");
    store.refuse = None;
    let snap = Snapshot::read(&mut store);
    assert_eq!(move_old(&mut store, &snap), Moved::Whole);
    assert_eq!(ids(&mut store), vec![id(1)]);
    assert!(store.kept.is_none());
    assert_eq!(store.map[OLD_KEY], EMPTIED);
}

fn has_key_of(store: &MemStore, kind: &str, n: u128) -> bool {
    store.keys().iter().any(|k| k.starts_with(&format!("{PREFIX}{kind}.{}", id(n))))
}

/// Q0: a kept entry is moved whatever the key holds by then: a later write of the older build does
/// not stand for it when that build was opened again from the emptied key.
#[test]
fn a_kept_entry_is_moved_whatever_the_key_holds_now() {
    for in_key in [EMPTIED.to_string(), "[]".to_string(), json!([old_entry(2, None)]).to_string()] {
        let mut store = MemStore::new();
        store.map.insert(OLD_KEY.into(), in_key.clone());
        store.kept = Some(json!([old_entry(1, None)]).to_string());
        let snap = Snapshot::read(&mut store);
        move_old(&mut store, &snap);
        assert!(has_key_of(&store, "e", 1), "the key holds {in_key}");
        assert!(store.kept.is_none(), "the key holds {in_key}");
        assert_eq!(has_key_of(&store, "e", 2), in_key.contains(&id(2).to_string()), "the key holds {in_key}");
    }
}

/// Q0: an entry a move read, from the key or from an event, and could not finish is kept: another
/// tab's `[ ]` can replace the key's text before the next move.
#[test]
fn an_entry_a_move_could_not_finish_is_kept_though_the_key_held_it() {
    for received in [true, false] {
        for refused in [entry_key(id(1)), format!("{PREFIX}t."), marker_key("o", id(1), None)] {
            let mut store = MemStore::new();
            let text = json!([old_entry(1, None)]).to_string();
            store.map.insert(OLD_KEY.into(), text.clone());
            if received {
                receive_old_text(&mut store, Some(text));
            }
            store.refuse = Some(refused.clone());
            let snap = Snapshot::read(&mut store);
            assert_ne!(move_old(&mut store, &snap), Moved::Whole, "{refused}");
            store.refuse = None;
            store.map.insert(OLD_KEY.into(), EMPTIED.into());
            let snap = Snapshot::read(&mut store);
            assert_eq!(move_old(&mut store, &snap), Moved::Whole, "{refused}, received: {received}");
            assert_eq!(ids(&mut store), vec![id(1)], "{refused}, received: {received}");
            assert!(store.kept.is_none(), "{refused}");
        }
    }
}

/// Q0: only an erase removes the old key, in any build, so the kept entries go with it.
#[test]
fn the_kept_entries_go_when_the_old_key_is_removed() {
    let mut store = MemStore::new();
    store.kept = Some(json!([old_entry(1, None)]).to_string());
    receive_old_text(&mut store, None);
    assert!(store.kept.is_none());
    let snap = Snapshot::read(&mut store);
    assert_eq!(move_old(&mut store, &snap), Moved::Nothing);
}

/// Q2b: a marker older than 30 days stays while a stamp of its id is stored, or an entry of the old
/// key whose `o` key is missing would be moved again.
#[test]
fn the_tidy_keeps_a_marker_while_a_stamp_of_its_id_is_stored() {
    let mut store = MemStore::new();
    put(&mut store, marker_key("l", id(4), Some(0)), json!({ "seq": 1 }));
    put(&mut store, stamp_key(4, 1), json!({}));
    store.now += MARKERS_KEPT_MS + 10;
    let snap = Snapshot::read(&mut store);
    tidy(&mut store, &snap);
    assert!(store.map.contains_key(&marker_key("l", id(4), Some(0))));
}

/// Q0: the entries of every text received are kept, once each, the text the key holds too.
#[test]
fn the_entries_of_every_received_text_are_kept_once_each() {
    let mut store = MemStore::new();
    store.map.insert(OLD_KEY.into(), EMPTIED.into());
    receive_old_text(&mut store, Some(json!([old_entry(1, None), old_entry(2, None)]).to_string()));
    receive_old_text(&mut store, Some(json!([old_entry(2, None), old_entry(3, None)]).to_string()));
    receive_old_text(&mut store, Some(EMPTIED.into()));
    receive_old_text(&mut store, Some("[]".into()));
    receive_old_text(&mut store, Some(json!([null, { "op": {} }]).to_string()));
    let held = json!([old_entry(4, None)]).to_string();
    store.map.insert(OLD_KEY.into(), held.clone());
    receive_old_text(&mut store, Some(held));
    let kept: Vec<Value> = serde_json::from_str(store.kept.as_deref().unwrap()).unwrap();
    assert_eq!(kept, vec![old_entry(1, None), old_entry(2, None), old_entry(3, None), old_entry(4, None)]);
    let snap = Snapshot::read(&mut store);
    assert_eq!(move_old(&mut store, &snap), Moved::Whole);
    assert_eq!(ids(&mut store), vec![id(1), id(2), id(3), id(4)]);
    assert!(store.kept.is_none());
    assert_eq!(store.map[OLD_KEY], EMPTIED);
}

#[test]
fn an_erase_drops_the_kept_text() {
    let mut store = MemStore::new();
    store.kept = Some(json!([old_entry(1, None)]).to_string());
    erase(&mut store);
    assert!(store.kept.is_none());
}

fn add_old(n: u128) -> Value {
    json!({ "id": id(n), "label": "", "op": { "type": "add_expense", "projectId": P, "payload": { "ct": "a", "iv": "b" }, "payers": [], "debtors": [] } })
}

/// Q0: the `i` key is decided from all the move reads. A delete in the key whose add is only among
/// the kept entries waits on that add, which the same move writes first.
#[test]
fn an_i_key_is_decided_from_all_the_move_reads() {
    let mut store = MemStore::new();
    store.map.insert(OLD_KEY.into(), json!([old_entry(2, Some(id(1)))]).to_string());
    store.kept = Some(json!([add_old(1)]).to_string());
    let snap = Snapshot::read(&mut store);
    assert_eq!(move_old(&mut store, &snap), Moved::Whole);
    assert!(!store.map.contains_key(&marker_key("i", id(1), None)));
    assert_eq!(ids(&mut store), vec![id(1), id(2)]);
}

/// Q0: an entry is not written while the one it names in `after` could not be, so that it never
/// waits on an add no stored key holds.
#[test]
fn an_entry_waits_for_the_next_move_when_the_one_it_names_could_not_be_written() {
    let mut store = MemStore::new();
    store.map.insert(OLD_KEY.into(), json!([old_entry(2, Some(id(1)))]).to_string());
    store.kept = Some(json!([add_old(1), old_entry(2, Some(id(1)))]).to_string());
    store.refuse = Some(entry_key(id(1)));
    let snap = Snapshot::read(&mut store);
    assert_eq!(move_old(&mut store, &snap), Moved::Partly);
    assert!(!has_key_of(&store, "e", 2));
    assert!(store.kept.is_some());
    store.refuse = None;
    let snap = Snapshot::read(&mut store);
    assert_eq!(move_old(&mut store, &snap), Moved::Whole);
    assert_eq!(ids(&mut store), vec![id(1), id(2)]);
}

/// Q0: a move that cannot write an entry's stamp stamps none of the entries behind it, so the next
/// move keeps them in the old key's order.
#[test]
fn a_move_stops_at_the_first_entry_it_cannot_stamp() {
    let mut store = MemStore::new();
    put(&mut store, OLD_KEY.into(), json!([old_entry(1, None), old_entry(2, None)]));
    store.refuse = Some(format!("{PREFIX}t.{}.", id(1)));
    let snap = Snapshot::read(&mut store);
    assert_eq!(move_old(&mut store, &snap), Moved::Unstamped);
    assert!(!has_key_of(&store, "t", 2), "{:?}", store.keys());
    store.refuse = None;
    let snap = Snapshot::read(&mut store);
    move_old(&mut store, &snap);
    assert_eq!(ids(&mut store), vec![id(1), id(2)]);
}

/// Q0: the stamps first, every one of them, before any `e` key.
#[test]
fn a_move_writes_every_stamp_before_any_e_key() {
    let mut store = MemStore::new();
    put(&mut store, OLD_KEY.into(), json!([old_entry(1, None), old_entry(2, None)]));
    store.writes_left = Some(2);
    let snap = Snapshot::read(&mut store);
    move_old(&mut store, &snap);
    assert!(has_key_of(&store, "t", 1) && has_key_of(&store, "t", 2), "{:?}", store.keys());
    assert!(!has_key_of(&store, "e", 1), "{:?}", store.keys());
}

/// Q0: a failed `i`, `e` or `o` write leaves that entry for the next move and stops nothing; its
/// stamp keeps its place.
#[test]
fn a_failed_write_after_the_stamps_leaves_that_entry_and_stops_nothing() {
    let o = marker_key("o", id(1), None);
    for refused in [entry_key(id(1)), marker_key("i", id(9), None), o.clone()] {
        let mut store = MemStore::new();
        let old = json!([old_entry(1, Some(id(9))), old_entry(2, None)]);
        put(&mut store, OLD_KEY.into(), old.clone());
        store.refuse = Some(refused.clone());
        let snap = Snapshot::read(&mut store);
        assert_eq!(move_old(&mut store, &snap), Moved::Partly, "{refused}");
        assert!(has_key_of(&store, "e", 2) && has_key_of(&store, "o", 2), "{refused}: {:?}", store.keys());
        assert_eq!(has_key_of(&store, "e", 1), refused == o, "{refused}");
        assert_eq!(store.map[OLD_KEY], old.to_string(), "{refused}: not emptied");
        store.refuse = None;
        let snap = Snapshot::read(&mut store);
        assert_eq!(move_old(&mut store, &snap), Moved::Whole, "{refused}");
        assert_eq!(ids(&mut store), vec![id(1), id(2)], "{refused}");
        assert_eq!(store.map[OLD_KEY], EMPTIED);
    }
}

/// Q0: an element of the old key without a readable `id` stays there and stops nothing: the
/// entries beside it move, and the key is not emptied while it holds that element.
#[test]
fn an_element_without_a_readable_id_stays_and_stops_nothing() {
    let mut store = MemStore::new();
    let old = json!([null, { "label": "" }, { "id": "" }, old_entry(1, None)]);
    put(&mut store, OLD_KEY.into(), old.clone());
    let snap = Snapshot::read(&mut store);
    assert_eq!(move_old(&mut store, &snap), Moved::Partly);
    assert_eq!(ids(&mut store), vec![id(1)]);
    assert_eq!(store.map[OLD_KEY], old.to_string());
}

/// Q2d: only a stamp the move cannot write refuses the handler's change. An unreadable element, or
/// an `e` key the move cannot write, never does.
#[test]
fn only_a_stamp_the_move_cannot_write_refuses_a_handler() {
    let mut store = MemStore::new();
    put(&mut store, OLD_KEY.into(), json!([null]));
    assert!(write_change(&mut store, &[add(5, P)]), "an unreadable element");
    assert_eq!(ids(&mut store), vec![id(5)]);

    let mut store = MemStore::new();
    put(&mut store, OLD_KEY.into(), json!([old_entry(1, None)]));
    store.refuse = Some(entry_key(id(1)));
    assert!(write_change(&mut store, &[add(5, P)]), "an e key of the move");
    assert!(has_key_of(&store, "e", 5));
}

/// Q2d, Q2c: a stamp the move cannot write is handled as a write that throws: once more, then the
/// change is not made, and the stamps already written stay.
#[test]
fn a_stamp_refused_twice_refuses_the_change_and_keeps_the_stamps_written() {
    let handler = |store: &mut MemStore| write_change(store, &[add(5, P)]) || write_change(store, &[add(5, P)]);
    let mut store = MemStore::new();
    put(&mut store, OLD_KEY.into(), json!([old_entry(1, None), old_entry(2, None)]));
    store.refuse = Some(format!("{PREFIX}t.{}.", id(2)));
    store.refusals = Some(1);
    assert!(handler(&mut store), "refused once, then accepted");
    assert_eq!(ids(&mut store), vec![id(1), id(2), id(5)]);

    let mut store = MemStore::new();
    put(&mut store, OLD_KEY.into(), json!([old_entry(1, None), old_entry(2, None)]));
    store.refuse = Some(format!("{PREFIX}t.{}.", id(2)));
    assert!(!handler(&mut store), "refused twice");
    assert!(!has_key_of(&store, "e", 5));
    assert!(has_key_of(&store, "t", 1) && !has_key_of(&store, "t", 2));
}

/// Q0: a view that holds the `o` key but neither the `e` key nor any `l`, `x` or `s` key of an id may
/// lag behind a live `e` key, so its stamps stay; once a marker of the id is read they go.
#[test]
fn stamps_stay_while_no_marker_of_the_id_is_read() {
    let mut store = MemStore::new();
    put(&mut store, stamp_key(1, 7), json!({}));
    put(&mut store, marker_key("o", id(1), None), json!({ "seq": 8 }));
    let snap = Snapshot::read(&mut store);
    tidy(&mut store, &snap);
    assert!(store.map.contains_key(&stamp_key(1, 7)), "no marker of the id read");
    put(&mut store, marker_key("x", id(1), Some(0)), json!({ "seq": 9 }));
    let snap = Snapshot::read(&mut store);
    tidy(&mut store, &snap);
    assert!(!store.map.contains_key(&stamp_key(1, 7)));
}
