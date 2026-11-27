use super::*;
use crate::common::test_fixtures::TEST_KEY;
use crate::crypto::{decrypt, encrypt, encrypt_padded_json};
use proptest::prelude::*;
use proptest::test_runner::{Config, TestRunner};
use serde_json::json;
use shared::errors;
use shared::merge::Reason;
use shared::{HistoryAction, HistoryEntity};
use std::collections::VecDeque;
use std::future::ready;
use std::pin::pin;
use std::task::{Context, Poll, Waker};

const OTHER_KEY: [u8; 32] = [7u8; 32];
const PROJECT: Uuid = Uuid::from_u128(0x5e1f0000_0000_4000_8000_000000000001);
const ENTRY: Uuid = Uuid::from_u128(0x9a2b0000_0000_4000_8000_000000000002);
const A: &str = "0f8e2c4a-6b1d-4e3f-9a7b-5c2d1e0f3a4b";
const B: &str = "7d2c9b1e-3a4f-4c5d-8e6f-0a1b2c3d4e5f";
const ROW: i32 = 41;

fn id(s: &str) -> Uuid {
    Uuid::parse_str(s).unwrap()
}

fn sealer() -> KeySealer<'static> {
    KeySealer(&TEST_KEY)
}

fn now<F: Future>(f: F) -> F::Output {
    match pin!(f).poll(&mut Context::from_waker(Waker::noop())) {
        Poll::Ready(v) => v,
        Poll::Pending => panic!("the fake server never waits"),
    }
}

fn p(v: Value) -> Payload {
    match v {
        Value::Object(m) => m,
        other => panic!("not an object: {other}"),
    }
}

fn norm(v: Value) -> Payload {
    p(serde_json::to_value(serde_json::from_value::<ExpensePayload>(v).unwrap()).unwrap())
}

fn taxi() -> Payload {
    norm(json!({
        "name": "Taxi", "amount": 30.0, "expenseType": "expense", "date": "2026-11-23",
        "shares": [{"userId": 1, "amount": 30.0, "isDebt": false},
                   {"userId": 1, "amount": 15.0, "isDebt": true}, {"userId": 2, "amount": 15.0, "isDebt": true}],
        "authorId": 1, "eid": A, "slot": "expense", "later": {"k": 1}
    }))
}

fn edit(from: &Payload, set: Value) -> Payload {
    let mut out = from.clone();
    out.extend(p(set));
    norm(Value::Object(out))
}

fn without(from: &Payload, keys: &[&str]) -> Payload {
    let mut out = from.clone();
    for key in keys {
        out.remove(*key);
    }
    out
}

fn priced(amount: f64) -> Value {
    json!({"amount": amount, "shares": [{"userId": 1, "amount": amount, "isDebt": false}, {"userId": 2, "amount": amount, "isDebt": true}]})
}

fn seal(payload: &Payload) -> EncryptedPair {
    encrypt_padded_json(&TEST_KEY, payload).unwrap()
}

fn open(pair: &EncryptedPair) -> Payload {
    p(serde_json::from_str(&decrypt(&TEST_KEY, pair).unwrap()).unwrap())
}

struct Stored {
    pair: EncryptedPair,
    rev: i64,
    eid: Option<Uuid>,
}

enum Forced {
    Answer(Answer),
    /// Applied as an honest server would, then answered with nothing.
    Lose,
}

type Change = Box<dyn FnOnce(&mut Payload)>;

/// A send the server applied: the row it replaced, what it stored (`None` for a delete), the row
/// the client last read, whether another writer changed the row between that read and it, and how
/// many changes other writers had made before it.
struct Accepted {
    before: Payload,
    after: Option<Payload>,
    read: Option<Payload>,
    raced: bool,
    changes_before: usize,
}

/// A server that checks `expected_rev` and the clear id as `revisions::check` does, leaves out a
/// history entry under a taken id, and can be scripted: forced answers, and changes by other
/// writers applied between a read and the next send.
struct Fake {
    row: Option<Stored>,
    serves_rev: bool,
    reads: VecDeque<Read>,
    answers: VecDeque<Forced>,
    meanwhile: VecDeque<Option<Change>>,
    read_count: usize,
    writes: Vec<Outgoing>,
    deletes: Vec<Deletion>,
    accepted: Vec<Accepted>,
    history: Vec<Uuid>,
    last_read: Option<Payload>,
    raced: bool,
    /// Every change by another writer, as the row was before and after it.
    changes: Vec<(Payload, Payload)>,
}

impl Fake {
    fn new(payload: &Payload, rev: i64) -> Self {
        Self::with_pair(seal(payload), rev, Some(id(A)))
    }

    fn with_pair(pair: EncryptedPair, rev: i64, eid: Option<Uuid>) -> Self {
        Fake {
            row: Some(Stored { pair, rev, eid }),
            serves_rev: true,
            reads: VecDeque::new(),
            answers: VecDeque::new(),
            meanwhile: VecDeque::new(),
            read_count: 0,
            writes: Vec::new(),
            deletes: Vec::new(),
            accepted: Vec::new(),
            history: Vec::new(),
            last_read: None,
            raced: false,
            changes: Vec::new(),
        }
    }

    fn gone() -> Self {
        Fake { row: None, ..Self::new(&taxi(), 1) }
    }

    fn payload(&self) -> Payload {
        open(&self.row.as_ref().unwrap().pair)
    }

    fn rev(&self) -> i64 {
        self.row.as_ref().unwrap().rev
    }

    fn sent(&self) -> usize {
        self.writes.len() + self.deletes.len()
    }

    fn change(&mut self, f: impl FnOnce(&mut Payload)) {
        let was = self.payload();
        let mut payload = was.clone();
        f(&mut payload);
        self.changes.push((was, payload.clone()));
        let row = self.row.as_mut().unwrap();
        row.pair = seal(&payload);
        row.rev += 1;
    }

    fn before_send(&mut self) -> Option<Forced> {
        self.raced = false;
        if let Some(Some(change)) = self.meanwhile.pop_front() {
            if self.row.is_some() {
                self.change(change);
                self.raced = true;
                if self.payload().contains_key(GONE) {
                    self.row = None;
                }
            }
        }
        self.answers.pop_front()
    }

    fn accepted(&mut self, before: Payload, after: Option<Payload>) {
        self.accepted.push(Accepted { before, after, read: self.last_read.clone(), raced: self.raced, changes_before: self.changes.len() });
    }

    fn named(&self, expected_rev: Option<i64>, eid: Option<Uuid>) -> Result<(), Answer> {
        let Some(row) = &self.row else { return Err(Answer::Gone) };
        match (self.serves_rev, expected_rev) {
            (true, Some(rev)) if rev != row.rev || eid != row.eid => Err(Answer::Stale),
            _ => Ok(()),
        }
    }

    fn keep_history(&mut self, history: &Option<HistoryContext>) {
        if let Some(eid) = history.as_ref().and_then(|h| h.eid) {
            if !self.history.contains(&eid) {
                self.history.push(eid);
            }
        }
    }

    fn accept_write(&mut self, out: &Outgoing) -> Answer {
        if let Err(answer) = self.named(out.expected_rev, out.eid) {
            return answer;
        }
        let (before, after) = (self.payload(), open(&out.payload));
        let row = self.row.as_mut().unwrap();
        row.pair = out.payload.clone();
        row.rev += 1;
        let rev = row.rev;
        self.keep_history(&out.history);
        self.accepted(before, Some(after));
        Answer::Landed { rev: self.serves_rev.then_some(rev) }
    }

    fn accept_delete(&mut self, out: &Deletion) -> Answer {
        if let Err(answer) = self.named(out.expected_rev, out.eid) {
            return answer;
        }
        let before = self.payload();
        self.keep_history(&out.history);
        self.row = None;
        self.accepted(before, None);
        Answer::Landed { rev: None }
    }
}

impl Rows for Fake {
    fn read(&mut self, _: Address) -> impl Future<Output = Read> {
        self.read_count += 1;
        self.last_read = self.row.as_ref().and_then(|row| serde_json::from_str(&decrypt(&TEST_KEY, &row.pair).ok()?).ok());
        let read = self.reads.pop_front().unwrap_or_else(|| match &self.row {
            None => Read::Gone,
            Some(row) => Read::Row(Served {
                payload: row.pair.clone(),
                rev: self.serves_rev.then_some(row.rev),
                eid: row.eid.filter(|_| self.serves_rev),
            }),
        });
        ready(read)
    }

    fn write(&mut self, _: Address, out: Outgoing) -> impl Future<Output = Answer> {
        let forced = self.before_send();
        self.writes.push(out.clone());
        ready(match forced {
            Some(Forced::Answer(answer)) => answer,
            Some(Forced::Lose) => {
                self.accept_write(&out);
                Answer::Unreachable
            }
            None => self.accept_write(&out),
        })
    }

    fn delete(&mut self, _: Address, out: Deletion) -> impl Future<Output = Answer> {
        let forced = self.before_send();
        self.deletes.push(out.clone());
        ready(match forced {
            Some(Forced::Answer(answer)) => answer,
            Some(Forced::Lose) => {
                self.accept_delete(&out);
                Answer::Unreachable
            }
            None => self.accept_delete(&out),
        })
    }
}

fn write_entry(base: &Payload, base_rev: i64, intent: &Payload) -> Entry {
    Entry {
        id: ENTRY,
        gen: 0,
        project_id: PROJECT,
        slot: Slot::Expense,
        eid: Some(id(A)),
        row: Some(ROW),
        base_rev: Some(base_rev),
        base: Some(seal(base)),
        op: Op::Write { intent: seal(intent) },
    }
}

fn remove_entry(base: &Payload, base_rev: i64) -> Entry {
    Entry { op: Op::Remove, ..write_entry(base, base_rev, base) }
}

fn next(entry: &Entry, version: NextVersion) -> Entry {
    let op = match version.intent {
        Some(intent) => Op::Write { intent },
        None => Op::Remove,
    };
    Entry { gen: entry.gen + 1, base: Some(version.base), base_rev: version.base_rev, op, ..entry.clone() }
}

fn note(landing: Landing<'_>) -> Option<HistoryPayload> {
    let entity_id = match landing.at {
        Address::Expense { row, .. } => Some(row),
        Address::Project(_) => None,
    };
    Some(HistoryPayload {
        summary: "edited".into(),
        actor_user_id: Some(1),
        action: Some(HistoryAction::Updated),
        entity: Some(HistoryEntity::Expense),
        entity_id,
        extra: Default::default(),
    })
}

fn run(fake: &mut Fake, entry: &Entry) -> Outcome {
    now(write_fresh(fake, &sealer(), entry, expense_check, note))
}

fn parked(outcome: Outcome) -> Pending {
    match outcome {
        Outcome::Parked(Park::Conflict(pending)) => *pending,
        other => panic!("not parked on a conflict: {other:?}"),
    }
}

fn version(resolution: Resolution) -> NextVersion {
    match resolution {
        Resolution::Version(version) => version,
        other => panic!("no version: {other:?}"),
    }
}

#[test]
fn a_clean_disjoint_merge_lands_with_one_history_entry() {
    let base = taxi();
    let theirs = edit(&base, json!({"name": "Train"}));
    let mine = edit(&base, priced(42.0));
    let mut fake = Fake::new(&theirs, 4);
    let read_iv = fake.row.as_ref().unwrap().pair.iv.clone();

    assert_eq!(run(&mut fake, &write_entry(&base, 3, &mine)), Outcome::Landed { rev: Some(5), sent: true });
    assert_eq!(fake.writes.len(), 1);
    let out = &fake.writes[0];
    assert_eq!((out.expected_rev, out.eid, &out.read_iv), (Some(4), Some(id(A)), &read_iv));
    assert_eq!(out.changed, vec![Group::Known("money")]);
    assert_eq!(fake.payload(), edit(&mine, json!({"name": "Train"})));
    assert_eq!(fake.history, vec![history_id(ENTRY, 0)]);

    let entry = out.history.as_ref().unwrap();
    assert_eq!(entry.eid, Some(history_id(ENTRY, 0)));
    let plain = decrypt(&TEST_KEY, &entry.payload).unwrap();
    assert_eq!(entity::read(&plain, entity::Slot::History), Ok(Some(history_id(ENTRY, 0))));
    assert_eq!(plain.len(), 256, "padded");
}

#[test]
fn the_merge_runs_even_when_base_rev_equals_the_row_s_rev() {
    let base = taxi();
    let restored = edit(&base, json!({"name": "Train"}));
    let mine = edit(&base, priced(42.0));
    let mut fake = Fake::new(&restored, 3);
    assert_eq!(run(&mut fake, &write_entry(&base, 3, &mine)), Outcome::Landed { rev: Some(4), sent: true });
    assert_eq!(fake.payload(), edit(&mine, json!({"name": "Train"})));
}

#[test]
fn a_replay_after_a_lost_answer_sends_nothing_and_writes_no_second_history_entry() {
    let base = taxi();
    let mine = edit(&base, priced(42.0));
    let entry = write_entry(&base, 3, &mine);
    let mut fake = Fake::new(&base, 3);
    fake.answers.push_back(Forced::Lose);

    assert_eq!(run(&mut fake, &entry), Outcome::RetryLater(Later::Unreachable));
    assert_eq!(fake.rev(), 4, "the first attempt landed");
    assert_eq!(run(&mut fake, &entry), Outcome::Landed { rev: Some(4), sent: false });
    assert_eq!(fake.writes.len(), 1);
    assert_eq!(fake.history, vec![history_id(ENTRY, 0)]);
    assert_eq!(fake.payload(), mine);
}

#[test]
fn every_attempt_of_one_generation_carries_the_same_history_id_and_a_new_generation_another() {
    let base = taxi();
    let entry = write_entry(&base, 3, &edit(&base, json!({"name": "Bus"})));
    let mut fake = Fake::new(&base, 3);
    fake.answers.push_back(Forced::Answer(Answer::Stale));
    assert_eq!(run(&mut fake, &entry), Outcome::Landed { rev: Some(4), sent: true });
    let ids: Vec<_> = fake.writes.iter().map(|w| w.history.as_ref().unwrap().eid).collect();
    assert_eq!(ids, vec![Some(history_id(ENTRY, 0)); 2]);

    let later = Entry { gen: 1, ..write_entry(&fake.payload(), 4, &edit(&base, json!({"name": "Bus", "category": "Food"}))) };
    assert_eq!(run(&mut fake, &later), Outcome::Landed { rev: Some(5), sent: true });
    assert_eq!(fake.history, vec![history_id(ENTRY, 0), history_id(ENTRY, 1)]);
}

#[test]
fn a_history_id_is_derived_from_the_entry_and_its_generation_in_the_v4_layout() {
    let first = history_id(ENTRY, 0);
    assert_eq!(first, history_id(ENTRY, 0));
    assert_ne!(first, history_id(ENTRY, 1));
    assert_ne!(first, history_id(Uuid::from_u128(1), 0));
    for gen in [0, 1, u64::MAX] {
        let id = history_id(ENTRY, gen);
        assert_eq!(id.get_version_num(), 4);
        assert_eq!(id.get_variant(), uuid::Variant::RFC4122);
        assert_eq!(entity::entity_id(&id.hyphenated().to_string()), Some(id));
    }
}

#[test]
fn a_retry_after_a_lost_answer_that_sends_again_is_left_out_of_history() {
    let base = taxi();
    let entry = write_entry(&base, 3, &edit(&base, json!({"name": "Bus"})));
    let mut fake = Fake::new(&base, 3);
    fake.answers.push_back(Forced::Lose);
    assert_eq!(run(&mut fake, &entry), Outcome::RetryLater(Later::Unreachable));
    fake.change(|p| {
        p.insert("name".into(), json!("Taxi"));
    });

    assert_eq!(run(&mut fake, &entry), Outcome::Landed { rev: Some(6), sent: true });
    let ids: Vec<_> = fake.writes.iter().map(|w| w.history.as_ref().unwrap().eid).collect();
    assert_eq!(ids, vec![Some(history_id(ENTRY, 0)); 2]);
    assert_eq!(fake.history.len(), 1, "one entry for one generation");
}

#[test]
fn three_stale_answers_end_in_retry_later_after_exactly_three_attempts() {
    let base = taxi();
    let mut fake = Fake::new(&base, 3);
    for _ in 0..3 {
        fake.answers.push_back(Forced::Answer(Answer::Stale));
    }
    assert_eq!(run(&mut fake, &write_entry(&base, 3, &edit(&base, json!({"name": "Bus"})))), Outcome::RetryLater(Later::Stale));
    assert_eq!((fake.read_count, fake.writes.len()), (3, 3));
    assert!(fake.writes.iter().all(|w| w.expected_rev == Some(3)));
    assert!(fake.accepted.is_empty());
    assert_eq!((fake.payload(), fake.rev()), (base, 3));
}

#[test]
fn a_row_that_keeps_moving_ends_in_retry_later_with_only_the_other_writers_changes() {
    let base = taxi();
    let mut fake = Fake::new(&base, 3);
    for day in ["2026-11-24", "2026-11-25", "2026-11-26"] {
        fake.meanwhile.push_back(Some(Box::new(move |p: &mut Payload| {
            p.insert("date".into(), json!(day));
        })));
    }
    assert_eq!(run(&mut fake, &write_entry(&base, 3, &edit(&base, json!({"name": "Bus"})))), Outcome::RetryLater(Later::Stale));
    assert_eq!(fake.payload(), edit(&base, json!({"date": "2026-11-26"})));
    assert_eq!(fake.writes.iter().map(|w| w.expected_rev).collect::<Vec<_>>(), [Some(3), Some(4), Some(5)]);
}

#[test]
fn a_stale_write_reads_and_merges_again_from_the_same_base() {
    let base = taxi();
    let mut fake = Fake::new(&base, 3);
    fake.meanwhile.push_back(Some(Box::new(|p: &mut Payload| {
        p.insert("date".into(), json!("2026-11-24"));
    })));
    assert_eq!(run(&mut fake, &write_entry(&base, 3, &edit(&base, json!({"name": "Bus"})))), Outcome::Landed { rev: Some(5), sent: true });
    assert_eq!(fake.writes.iter().map(|w| w.expected_rev).collect::<Vec<_>>(), [Some(3), Some(4)]);
    assert_eq!(fake.payload(), edit(&base, json!({"name": "Bus", "date": "2026-11-24"})));
}

fn conflict() -> (Payload, Payload, Payload) {
    let base = taxi();
    let theirs = edit(&base, json!({"name": "Train", "date": "2026-11-24"}));
    let mine = edit(&base, json!({"name": "Bus", "category": "Transport"}));
    (base, theirs, mine)
}

#[test]
fn both_sides_changing_one_group_parks_with_both_values_and_sends_nothing() {
    let (base, theirs, mine) = conflict();
    let mut fake = Fake::new(&theirs, 4);
    let pending = parked(run(&mut fake, &write_entry(&base, 3, &mine)));
    let cards: Vec<_> = pending.choice.conflicts.iter().map(|c| (c.group.clone(), c.reason, c.theirs.clone(), c.mine.clone())).collect();
    assert_eq!(cards, vec![(Group::Known("name"), Reason::BothChanged, p(json!({"name": "Train"})), p(json!({"name": "Bus"})))]);
    assert_eq!((pending.seen.rev, pending.seen.eid, &pending.seen.payload), (Some(4), Some(id(A)), &theirs));
    assert_eq!((fake.sent(), fake.history.len()), (0, 0));
}

#[test]
fn resolving_a_conflict_each_way_sends_that_side_under_the_rev_it_was_computed_on() {
    let (base, theirs, mine) = conflict();
    for (side, name) in [(Side::Mine, "Bus"), (Side::Theirs, "Train")] {
        let entry = write_entry(&base, 3, &mine);
        let mut fake = Fake::new(&theirs, 4);
        let pending = parked(run(&mut fake, &entry));
        let chosen = version(resolve(&pending, |_| side, expense_check, &sealer()));
        assert_eq!(chosen.base_rev, Some(4));

        assert_eq!(run(&mut fake, &next(&entry, chosen)), Outcome::Landed { rev: Some(5), sent: true }, "{side:?}");
        assert_eq!(fake.writes.last().unwrap().expected_rev, Some(4));
        assert_eq!(fake.payload(), edit(&base, json!({"name": name, "date": "2026-11-24", "category": "Transport"})), "{side:?}");
        assert_eq!(fake.history, vec![history_id(ENTRY, 1)]);
    }
}

#[test]
fn a_row_changed_between_the_park_and_the_choice_is_merged_again() {
    let (base, theirs, mine) = conflict();
    let entry = write_entry(&base, 3, &mine);

    let mut fake = Fake::new(&theirs, 4);
    let chosen = version(resolve(&parked(run(&mut fake, &entry)), |_| Side::Mine, expense_check, &sealer()));
    fake.change(|p| {
        p.insert("description".into(), json!("Airport"));
    });
    assert_eq!(run(&mut fake, &next(&entry, chosen)), Outcome::Landed { rev: Some(6), sent: true });
    assert_eq!(fake.writes.last().unwrap().expected_rev, Some(5));
    assert_eq!(fake.payload(), edit(&base, json!({"name": "Bus", "date": "2026-11-24", "category": "Transport", "description": "Airport"})));

    let mut fake = Fake::new(&theirs, 4);
    let chosen = version(resolve(&parked(run(&mut fake, &entry)), |_| Side::Mine, expense_check, &sealer()));
    fake.change(|p| {
        p.insert("name".into(), json!("Tram"));
    });
    let again = parked(run(&mut fake, &next(&entry, chosen)));
    assert_eq!(again.choice.conflicts[0].theirs, p(json!({"name": "Tram"})));
    assert_eq!(again.choice.conflicts[0].mine, p(json!({"name": "Bus"})));
    assert_eq!(again.seen.rev, Some(5));
    assert_eq!(fake.sent(), 0);
}

#[test]
fn a_pick_the_form_refuses_asks_again() {
    let base = taxi();
    let theirs = edit(&base, json!({"name": "Train"}));
    let mine = edit(&base, json!({"name": "Bus"}));
    let refuse_bus = |p: &Payload| {
        let mut refused = expense_check(p);
        if p.get("name") == Some(&json!("Bus")) {
            refused.push(Group::Known("name"));
        }
        refused
    };
    let pending = Pending {
        choice: match merge::merge(Kind::Expense, &base, &theirs, &mine, refuse_bus) {
            merge::Outcome::Choose(choice) => choice,
            other => panic!("{other:?}"),
        },
        seen: Seen { pair: seal(&theirs), payload: theirs.clone(), rev: Some(4), eid: Some(id(A)) },
    };
    assert!(matches!(resolve(&pending, |_| Side::Mine, refuse_bus, &sealer()), Resolution::Again(_)));
    assert!(matches!(resolve(&pending, |_| Side::Theirs, refuse_bus, &sealer()), Resolution::Version(_)));
}

#[test]
fn a_gone_row_parks_a_write_and_lands_a_remove() {
    let base = taxi();
    let mine = edit(&base, json!({"name": "Bus"}));
    let mut fake = Fake::gone();
    assert_eq!(run(&mut fake, &write_entry(&base, 3, &mine)), Outcome::Parked(Park::Gone { mine: mine.clone() }));
    assert_eq!(run(&mut fake, &remove_entry(&base, 3)), Outcome::Landed { rev: None, sent: false });
    assert_eq!(fake.sent(), 0);

    let mut fake = Fake::new(&base, 3);
    fake.answers.push_back(Forced::Answer(Answer::Gone));
    assert_eq!(run(&mut fake, &write_entry(&base, 3, &mine)), Outcome::Parked(Park::Gone { mine }));
    let mut fake = Fake::new(&base, 3);
    fake.answers.push_back(Forced::Answer(Answer::Gone));
    assert_eq!(run(&mut fake, &remove_entry(&base, 3)), Outcome::Landed { rev: None, sent: true });
}

#[test]
fn a_served_id_other_than_the_entry_s_is_refused_and_never_taken_for_gone() {
    let base = taxi();
    let mine = edit(&base, json!({"name": "Bus"}));
    let mut fake = Fake::with_pair(seal(&base), 3, Some(id(B)));
    assert_eq!(run(&mut fake, &write_entry(&base, 3, &mine)), Outcome::Refused(Refusal::OtherRow));
    assert_eq!(run(&mut fake, &remove_entry(&base, 3)), Outcome::Refused(Refusal::OtherRow));
    assert_eq!(fake.sent(), 0);
    assert!(fake.row.is_some());

    let unknown = Entry { eid: None, ..write_entry(&base, 3, &mine) };
    let mut fake = Fake::new(&base, 3);
    assert_eq!(run(&mut fake, &unknown), Outcome::Landed { rev: Some(4), sent: true });
    assert_eq!(fake.writes[0].eid, Some(id(A)), "addressed by the id served, never the entry's");
}

#[test]
fn a_remove_deletes_naming_expected_rev_and_the_served_id_when_the_row_equals_base() {
    let base = taxi();
    let mut fake = Fake::new(&base, 3);
    assert_eq!(run(&mut fake, &remove_entry(&base, 3)), Outcome::Landed { rev: None, sent: true });
    let out = &fake.deletes[0];
    assert_eq!((out.expected_rev, out.eid), (Some(3), Some(id(A))));
    assert_eq!(out.history.as_ref().unwrap().eid, Some(history_id(ENTRY, 0)));
    assert!(fake.row.is_none());
    assert_eq!(fake.history.len(), 1);
}

#[test]
fn a_remove_of_a_row_changed_since_parks_and_deleting_anyway_takes_a_choice() {
    let base = taxi();
    let entry = remove_entry(&base, 3);
    let mut fake = Fake::new(&edit(&base, json!({"name": "Train"})), 4);
    let Outcome::Parked(Park::ChangedSinceRemoved(seen)) = run(&mut fake, &entry) else { panic!("not parked") };
    assert_eq!(seen.rev, Some(4));
    assert_eq!(fake.sent(), 0);

    let anyway = adopt(&seen, None, &sealer()).unwrap();
    assert_eq!((anyway.base_rev, anyway.intent.is_none()), (Some(4), true));
    assert_eq!(run(&mut fake, &next(&entry, anyway)), Outcome::Landed { rev: None, sent: true });
    assert!(fake.row.is_none());
}

#[test]
fn a_delete_answered_stale_reads_again_and_parks_when_the_row_changed() {
    let base = taxi();
    let mut fake = Fake::new(&base, 3);
    fake.meanwhile.push_back(Some(Box::new(|p: &mut Payload| {
        p.insert("name".into(), json!("Train"));
    })));
    assert!(matches!(run(&mut fake, &remove_entry(&base, 3)), Outcome::Parked(Park::ChangedSinceRemoved(_))));
    assert_eq!(fake.deletes.len(), 1);
    assert_eq!(fake.rev(), 4);
}

#[test]
fn rows_that_cannot_be_merged_onto_are_refused_and_never_written() {
    let base = taxi();
    let mine = edit(&base, json!({"name": "Bus"}));
    let raw = |set: Value, remove: &[&str]| {
        let mut plain = without(&base, remove);
        plain.extend(p(set));
        encrypt(&TEST_KEY, &serde_json::to_string(&plain).unwrap()).unwrap()
    };
    let cases = [
        ("under another key", encrypt_padded_json(&OTHER_KEY, &base).unwrap(), Refusal::RowDoesNotOpen),
        ("served in the wrong list", raw(json!({"slot": "rule"}), &[]), Refusal::RowUnreadable),
        ("eid without slot", raw(json!({}), &["slot"]), Refusal::RowUnreadable),
        ("not an expense this build reads", raw(json!({"amount": "30"}), &[]), Refusal::RowUnreadable),
        ("shares in the payments rows", seal(&without(&base, &["shares"])), Refusal::Legacy),
        ("another entity sealed", seal(&edit(&base, json!({"eid": B}))), Refusal::OtherRow),
    ];
    for (what, pair, refusal) in cases {
        let mut fake = Fake::with_pair(pair.clone(), 4, Some(id(A)));
        assert_eq!(run(&mut fake, &write_entry(&base, 3, &mine)), Outcome::Refused(refusal), "{what}");
        assert_eq!(fake.sent(), 0, "{what}");
        assert_eq!(fake.row.as_ref().unwrap().pair, pair, "{what}");
    }
}

#[test]
fn an_unfixable_merge_and_a_conflict_on_the_entity_group_are_refused() {
    let base = edit(&taxi(), json!({"category": "Food"}));
    let refuse_food = |p: &Payload| {
        let mut refused = expense_check(p);
        if p.get("category") == Some(&json!("Food")) {
            refused.push(Group::Known("category"));
        }
        refused
    };
    let mut fake = Fake::new(&edit(&base, json!({"name": "Train"})), 4);
    let entry = write_entry(&base, 3, &edit(&base, json!({"date": "2026-11-24"})));
    assert_eq!(now(write_fresh(&mut fake, &sealer(), &entry, refuse_food, note)), Outcome::Refused(Refusal::Unfixable(vec![Group::Known("category")])));
    assert_eq!(fake.sent(), 0);

    let unbound = without(&taxi(), &["eid", "slot"]);
    let theirs = edit(&unbound, json!({"eid": A, "slot": "expense"}));
    let mine = edit(&unbound, json!({"eid": B, "slot": "expense", "name": "Bus"}));
    let mut fake = Fake::new(&theirs, 4);
    assert_eq!(run(&mut fake, &write_entry(&unbound, 3, &mine)), Outcome::Refused(Refusal::EntityConflict));
    assert_eq!(fake.sent(), 0);
}

#[test]
fn an_entry_whose_pairs_do_not_open_or_read_is_refused_before_any_read() {
    let base = taxi();
    let mine = edit(&base, json!({"name": "Bus"}));
    let foreign = Entry { op: Op::Write { intent: encrypt_padded_json(&OTHER_KEY, &mine).unwrap() }, ..write_entry(&base, 3, &mine) };
    let unreadable = Entry { base: Some(seal(&edit(&base, json!({"slot": "rule"})))), ..write_entry(&base, 3, &mine) };
    for (entry, refusal) in [(foreign, Refusal::EntryDoesNotOpen), (unreadable, Refusal::EntryUnreadable)] {
        let mut fake = Fake::new(&base, 3);
        assert_eq!(run(&mut fake, &entry), Outcome::Refused(refusal));
        assert_eq!((fake.read_count, fake.sent()), (0, 0));
    }

    let legacy = without(&base, &["shares"]);
    for entry in [write_entry(&legacy, 3, &mine), write_entry(&base, 3, &without(&mine, &["shares"]))] {
        let mut fake = Fake::new(&base, 3);
        assert_eq!(run(&mut fake, &entry), Outcome::Refused(Refusal::Legacy));
        assert_eq!(fake.sent(), 0);
    }
}

#[test]
fn a_legacy_row_is_removed_only_at_the_rev_the_user_saw() {
    let legacy = without(&taxi(), &["shares"]);
    let mut fake = Fake::new(&legacy, 3);
    assert_eq!(run(&mut fake, &remove_entry(&legacy, 3)), Outcome::Landed { rev: None, sent: true });
    assert_eq!(fake.deletes[0].expected_rev, Some(3));

    let mut moved = Fake::new(&legacy, 4);
    let mut unknown = Fake::new(&legacy, 3);
    unknown.serves_rev = false;
    for fake in [&mut moved, &mut unknown] {
        assert_eq!(run(fake, &remove_entry(&legacy, 3)), Outcome::Refused(Refusal::Legacy));
        assert_eq!(fake.sent(), 0);
    }
}

#[test]
fn a_write_onto_a_legacy_row_lands_whole_only_while_the_row_is_what_the_user_saw() {
    let legacy = without(&taxi(), &["shares"]);
    let mine = edit(&taxi(), json!({"name": "Bus"}));
    let mut fake = Fake::new(&legacy, 3);
    assert_eq!(run(&mut fake, &write_entry(&legacy, 3, &mine)), Outcome::Landed { rev: Some(4), sent: true });
    assert_eq!(fake.writes[0].expected_rev, Some(3));
    assert_eq!(fake.writes[0].changed, vec![Group::Known("money"), Group::Known("name")]);
    assert_eq!(fake.payload(), mine);

    let mut moved = Fake::new(&legacy, 4);
    let mut unknown = Fake::new(&legacy, 3);
    unknown.serves_rev = false;
    let mut renamed = Fake::new(&edit(&legacy, json!({"name": "Train"})), 3);
    for (what, fake) in [("moved", &mut moved), ("no rev", &mut unknown), ("another payload", &mut renamed)] {
        assert_eq!(run(fake, &write_entry(&legacy, 3, &mine)), Outcome::Refused(Refusal::Legacy), "{what}");
        assert_eq!(fake.sent(), 0, "{what}");
    }

    let mut fake = Fake::new(&legacy, 3);
    let still_legacy = edit(&legacy, json!({"name": "Bus"}));
    assert_eq!(run(&mut fake, &write_entry(&legacy, 3, &still_legacy)), Outcome::Refused(Refusal::Legacy), "an intent without shares would drop the payments rows");
    assert_eq!(fake.sent(), 0);
}

#[test]
fn a_remove_of_a_row_bound_or_unbound_since_parks_and_one_naming_another_entity_is_refused() {
    let bound = taxi();
    let unbound = without(&bound, &["eid", "slot"]);
    for (what, base, row) in [("bound since", &unbound, &bound), ("unbound since", &bound, &unbound)] {
        let mut fake = Fake::new(row, 4);
        assert!(matches!(run(&mut fake, &remove_entry(base, 3)), Outcome::Parked(Park::ChangedSinceRemoved(_))), "{what}");
        assert_eq!(fake.sent(), 0, "{what}");
    }
    let mut fake = Fake::new(&edit(&bound, json!({"eid": B})), 4);
    assert_eq!(run(&mut fake, &remove_entry(&bound, 3)), Outcome::Refused(Refusal::OtherRow));
    assert_eq!(fake.sent(), 0);
}

type Logged = Vec<(Payload, Option<Payload>, Vec<Group>)>;

fn logging(log: &std::cell::RefCell<Logged>) -> impl FnMut(Landing<'_>) -> Option<HistoryPayload> + '_ {
    move |landing: Landing<'_>| {
        log.borrow_mut().push((landing.before.clone(), landing.after.cloned(), landing.changed.to_vec()));
        note(landing)
    }
}

#[test]
fn the_history_callback_is_given_the_row_merged_onto_what_is_stored_and_what_changed() {
    let base = taxi();
    let theirs = edit(&base, json!({"name": "Train"}));
    let mine = edit(&base, priced(42.0));
    let log = std::cell::RefCell::new(Logged::new());
    let mut fake = Fake::new(&theirs, 4);
    now(write_fresh(&mut fake, &sealer(), &write_entry(&base, 3, &mine), expense_check, logging(&log)));
    assert_eq!(log.take(), vec![(theirs, Some(fake.payload()), vec![Group::Known("money")])]);

    let mut fake = Fake::new(&base, 3);
    fake.meanwhile.push_back(Some(Box::new(|p: &mut Payload| {
        p.insert("date".into(), json!("2026-11-24"));
    })));
    let renamed = edit(&base, json!({"name": "Bus"}));
    now(write_fresh(&mut fake, &sealer(), &write_entry(&base, 3, &renamed), expense_check, logging(&log)));
    let moved = edit(&base, json!({"date": "2026-11-24"}));
    let name = vec![Group::Known("name")];
    assert_eq!(log.take(), vec![(base.clone(), Some(renamed), name.clone()), (moved, Some(fake.payload()), name)]);

    let mut fake = Fake::new(&base, 3);
    now(write_fresh(&mut fake, &sealer(), &remove_entry(&base, 3), expense_check, logging(&log)));
    assert_eq!(log.take(), vec![(base, None, Vec::new())]);
}

#[test]
fn rev_required_after_a_read_that_served_no_rev_reads_again_then_retries_later() {
    let base = taxi();
    let rev_required = || Forced::Answer(Answer::Refused { status: 409, code: errors::REV_REQUIRED.into() });
    for entry in [write_entry(&base, 3, &edit(&base, json!({"name": "Bus"}))), remove_entry(&base, 3)] {
        let mut fake = Fake::new(&base, 3);
        let pair = fake.row.as_ref().unwrap().pair.clone();
        fake.reads.push_back(Read::Row(Served { payload: pair, rev: None, eid: None }));
        fake.answers.push_back(rev_required());
        assert!(matches!(run(&mut fake, &entry), Outcome::Landed { sent: true, .. }));
        assert_eq!(fake.read_count, 2);
        assert_eq!(fake.sent(), 2);
        assert_eq!(fake.accepted.len(), 1);

        let mut fake = Fake::new(&base, 3);
        fake.serves_rev = false;
        for _ in 0..3 {
            fake.answers.push_back(rev_required());
        }
        assert_eq!(run(&mut fake, &entry), Outcome::RetryLater(Later::RevRequired));
        assert_eq!((fake.read_count, fake.sent(), fake.accepted.len()), (3, 3, 0));
    }
}

#[test]
fn rotating_unreachable_and_server_errors_retry_later_and_other_refusals_are_kept() {
    let refused = |status: u16, code: &str| Answer::Refused { status, code: code.into() };
    let kept = |status: u16, code: &str| Outcome::Refused(Refusal::Server { status, code: code.into() });
    let cases = [
        (Answer::Rotating, Outcome::RetryLater(Later::Rotating)),
        (Answer::Unreachable, Outcome::RetryLater(Later::Unreachable)),
        (refused(500, "x"), Outcome::RetryLater(Later::Server(500))),
        (refused(503, "x"), Outcome::RetryLater(Later::Server(503))),
        (refused(426, errors::CLIENT_OUTDATED), Outcome::RetryLater(Later::Server(426))),
        (refused(429, "x"), Outcome::RetryLater(Later::Server(429))),
        (refused(403, errors::PROJECT_KEY_ROTATED), kept(403, errors::PROJECT_KEY_ROTATED)),
        (refused(404, errors::USER_NOT_FOUND), kept(404, errors::USER_NOT_FOUND)),
        (refused(409, errors::REV_REQUIRED), kept(409, errors::REV_REQUIRED)),
        (refused(400, errors::INVALID_PAYLOAD), kept(400, errors::INVALID_PAYLOAD)),
    ];
    let base = taxi();
    let entries = [write_entry(&base, 3, &edit(&base, json!({"name": "Bus"}))), remove_entry(&base, 3)];
    for (answer, want) in cases {
        for entry in &entries {
            let mut fake = Fake::new(&base, 3);
            fake.answers.push_back(Forced::Answer(answer.clone()));
            assert_eq!(run(&mut fake, entry), want, "{answer:?} on a send");
            assert_eq!((fake.sent(), fake.accepted.len(), fake.row.is_some()), (1, 0, true));

            let read = match answer.clone() {
                Answer::Rotating => Read::Rotating,
                Answer::Unreachable => Read::Unreachable,
                Answer::Refused { status, code } => Read::Refused { status, code },
                other => panic!("{other:?}"),
            };
            let mut fake = Fake::new(&base, 3);
            fake.reads.push_back(read);
            assert_eq!(run(&mut fake, entry), want, "{answer:?} on a read");
            assert_eq!(fake.sent(), 0);
        }
    }
}

#[test]
fn a_creation_is_left_to_add_expense_and_an_unresolved_row_is_not_guessed() {
    let base = taxi();
    let creation = Entry { row: None, base: None, base_rev: None, ..write_entry(&base, 3, &base) };
    let unresolved = Entry { row: None, ..write_entry(&base, 1, &edit(&base, json!({"name": "Bus"}))) };
    let project_remove = Entry { slot: Slot::Project, row: None, eid: None, ..remove_entry(&base, 3) };
    let unresolved_remove = Entry { row: None, base: None, base_rev: None, ..remove_entry(&base, 3) };
    for (entry, refusal) in [
        (creation, Refusal::Creation),
        (unresolved, Refusal::Unaddressed),
        (project_remove, Refusal::Unaddressed),
        (unresolved_remove, Refusal::Unaddressed),
    ] {
        let mut fake = Fake::new(&base, 3);
        assert_eq!(run(&mut fake, &entry), Outcome::Refused(refusal));
        assert_eq!((fake.read_count, fake.sent()), (0, 0));
    }
}

#[test]
fn a_server_that_serves_no_rev_still_merges_from_a_fresh_read_and_sends_without_expected_rev() {
    let base = taxi();
    let mine = edit(&base, priced(42.0));
    let mut fake = Fake::new(&edit(&base, json!({"name": "Train"})), 4);
    fake.serves_rev = false;
    let read_iv = fake.row.as_ref().unwrap().pair.iv.clone();
    assert_eq!(run(&mut fake, &write_entry(&base, 3, &mine)), Outcome::Landed { rev: None, sent: true });
    assert_eq!((fake.writes[0].expected_rev, fake.writes[0].eid, &fake.writes[0].read_iv), (None, None, &read_iv));
    assert_eq!(fake.payload(), edit(&mine, json!({"name": "Train"})));

    let mut fake = Fake::new(&base, 3);
    fake.serves_rev = false;
    assert_eq!(run(&mut fake, &remove_entry(&base, 3)), Outcome::Landed { rev: None, sent: true });
    assert_eq!((fake.deletes[0].expected_rev, fake.deletes[0].eid), (None, None));
}

#[test]
fn an_entry_with_a_row_and_no_base_shows_the_whole_row_and_writes_what_the_user_adopts() {
    let base = taxi();
    let theirs = edit(&base, json!({"name": "Train"}));
    let mine = edit(&base, json!({"date": "2026-11-24"}));
    let entry = Entry { base: None, base_rev: None, ..write_entry(&base, 3, &mine) };
    let mut fake = Fake::new(&theirs, 4);
    let Outcome::Parked(Park::NoBase { seen, mine: Some(shown) }) = run(&mut fake, &entry) else { panic!("not parked") };
    assert_eq!((&seen.payload, seen.rev, &shown), (&theirs, Some(4), &mine));
    assert_eq!(fake.sent(), 0);

    let mine_over_it = adopt(&seen, Some(&mine), &sealer()).unwrap();
    assert_eq!(run(&mut fake, &next(&entry, mine_over_it)), Outcome::Landed { rev: Some(5), sent: true });
    assert_eq!(fake.payload(), mine);
}

#[test]
fn look_shows_a_parked_conflict_without_sending_and_says_when_nothing_is_left_to_choose() {
    let (base, theirs, mine) = conflict();
    let entry = write_entry(&base, 3, &mine);
    let mut fake = Fake::new(&theirs, 4);
    assert!(matches!(now(look(&mut fake, &sealer(), &entry, expense_check)), Looked::Now(Outcome::Parked(Park::Conflict(_)))));
    fake.change(|p| {
        p.insert("name".into(), json!("Bus"));
    });
    assert_eq!(now(look(&mut fake, &sealer(), &entry, expense_check)), Looked::WouldSend);
    assert_eq!(fake.sent(), 0);
}

fn project(v: Value) -> Payload {
    p(serde_json::to_value(serde_json::from_value::<ProjectPayload>(v).unwrap()).unwrap())
}

#[test]
fn a_project_write_merges_by_the_project_groups_and_asks_for_a_name() {
    let base = project(json!({"name": "Ski", "currency": "EUR", "description": "Tignes"}));
    let theirs = project(json!({"name": "Ski", "currency": "EUR", "description": "Tignes", "emoji": "\u{2600}"}));
    let mine = project(json!({"name": "Ski", "currency": "EUR", "description": "Tignes", "status": "closed"}));
    let entry = Entry { slot: Slot::Project, eid: None, row: None, ..write_entry(&base, 3, &mine) };
    let mut fake = Fake::with_pair(seal(&theirs), 4, None);
    assert_eq!(now(write_fresh(&mut fake, &sealer(), &entry, project_check, note)), Outcome::Landed { rev: Some(5), sent: true });
    assert_eq!((fake.writes[0].expected_rev, fake.writes[0].eid), (Some(4), None));
    assert_eq!(fake.payload(), project(json!({"name": "Ski", "currency": "EUR", "description": "Tignes", "emoji": "\u{2600}", "status": "closed"})));

    let blank = project(json!({"name": " ", "currency": "EUR", "description": "Tignes"}));
    let entry = Entry { slot: Slot::Project, eid: None, row: None, ..write_entry(&base, 3, &blank) };
    let mut fake = Fake::with_pair(seal(&theirs), 4, None);
    let pending = parked(now(write_fresh(&mut fake, &sealer(), &entry, check_for(Slot::Project), note)));
    assert_eq!(pending.choice.conflicts[0].reason, Reason::Invalid);
    assert_eq!(fake.sent(), 0);
}

#[test]
fn the_expense_check_refuses_what_the_form_refuses_by_group() {
    let base = taxi();
    let refused = |set: Value| expense_check(&edit(&base, set));
    assert!(expense_check(&base).is_empty());
    assert_eq!(refused(json!({"name": "  "})), vec![Group::Known("name")]);
    assert_eq!(refused(json!({"date": "23/11/2026"})), vec![Group::Known("date")]);
    for money in [
        json!({"amount": 0.0}),
        json!({"amount": -30.0}),
        json!({"shares": [{"userId": 1, "amount": 30.0, "isDebt": false}]}),
        json!({"shares": [{"userId": 1, "amount": 30.0, "isDebt": true}]}),
        json!({"shares": [{"userId": 1, "amount": 0.0, "isDebt": false}, {"userId": 2, "amount": 30.0, "isDebt": true}]}),
        json!({"sourceCurrency": "USD"}),
        json!({"sourceCurrency": "USD", "sourceAmount": 31.25, "rate": 0.9}),
        json!({"sourceCurrency": "USD", "sourceAmount": 31.25, "rate": 0.0}),
    ] {
        assert_eq!(refused(money.clone()), vec![Group::Known("money")], "{money}");
    }
    assert!(refused(json!({"sourceCurrency": "USD", "sourceAmount": 31.25, "rate": 0.96})).is_empty());
    assert_eq!(expense_check(&without(&base, &["shares"])), vec![Group::Known("money")]);
    assert_eq!(project_check(&project(json!({"name": "", "currency": "EUR"}))), vec![Group::Known("name")]);
    assert!(project_check(&project(json!({"name": "Ski", "currency": "EUR"}))).is_empty());
}

#[test]
fn the_key_sealer_opens_what_it_seals_padded() {
    let sealed = sealer().seal(&taxi()).unwrap();
    assert_eq!(decrypt(&TEST_KEY, &sealed).unwrap().len(), 512);
    assert_eq!(p(serde_json::from_str(&sealer().open(&sealed).unwrap()).unwrap()), taxi());
    assert!(KeySealer(&OTHER_KEY).open(&sealed).is_err());
}

const KEYS: [&[&str]; 5] = [&["amount", "expenseType", "shares"], &["name"], &["date"], &["category"], &["later"]];

fn pools() -> [[Value; 3]; 5] {
    [
        [
            json!({"amount": 30.0, "expenseType": "expense", "shares": [{"userId": 1, "amount": 30.0, "isDebt": false}, {"userId": 1, "amount": 15.0, "isDebt": true}, {"userId": 2, "amount": 15.0, "isDebt": true}]}),
            json!({"amount": 42.0, "expenseType": "expense", "shares": [{"userId": 1, "amount": 42.0, "isDebt": false}, {"userId": 2, "amount": 42.0, "isDebt": true}]}),
            json!({"amount": 30.0, "expenseType": "gain", "shares": [{"userId": 2, "amount": 30.0, "isDebt": false}, {"userId": 1, "amount": 30.0, "isDebt": true}]}),
        ],
        [json!({"name": "Taxi"}), json!({"name": "Train"}), json!({"name": "Bus"})],
        [json!({"date": "2026-11-23"}), json!({"date": "2026-11-24"}), json!({"date": "2026-11-25"})],
        [json!({}), json!({"category": "Food"}), json!({"category": "Transport"})],
        [json!({}), json!({"later": 1}), json!({"later": [2]})],
    ]
}

fn set_group(payload: &mut Payload, group: usize, value: usize) {
    for key in KEYS[group] {
        payload.remove(*key);
    }
    payload.extend(p(pools()[group][value].clone()));
}

fn built(pick: impl Fn(usize) -> usize) -> Payload {
    let mut out = p(json!({"authorId": 1, "eid": A, "slot": "expense"}));
    for group in 0..KEYS.len() {
        set_group(&mut out, group, pick(group));
    }
    norm(Value::Object(out))
}

fn rendered(payload: &Payload, group: usize) -> Vec<Option<Vec<u8>>> {
    KEYS[group].iter().map(|k| payload.get(*k).map(|v| serde_json::to_vec(v).unwrap())).collect()
}

/// Plan §3.1 on one write the server accepted, `before` being the row it landed on.
fn landed_fairly(base: &Payload, mine: &Payload, before: &Payload, after: &Payload) -> Result<(), TestCaseError> {
    for group in 0..KEYS.len() {
        let (b, m, t, r) = (rendered(base, group), rendered(mine, group), rendered(before, group), rendered(after, group));
        prop_assert!(r == t || r == m, "group {} is neither side's", group);
        if r != t {
            prop_assert_eq!(&t, &b, "group {}: overwrote a value its user had not seen", group);
        }
        if r != m {
            prop_assert_eq!(&m, &b, "group {}: the intent was dropped", group);
        }
    }
    for key in ["authorId", "eid", "slot"] {
        prop_assert_eq!(after.get(key), before.get(key));
    }
    Ok(())
}

fn intent_is_there(base: &Payload, mine: &Payload, row: &Payload) -> Result<(), TestCaseError> {
    for group in 0..KEYS.len() {
        if rendered(mine, group) != rendered(base, group) {
            prop_assert_eq!(rendered(row, group), rendered(mine, group), "group {}: landed without the intent", group);
        }
    }
    Ok(())
}

/// A side's index of 3 or more keeps the base's value, so each side changes a group a third of the time.
fn sides(picks: &[(usize, usize, usize)]) -> (Payload, Payload, Payload) {
    let side = |i: usize, b: usize| if i < 3 { i } else { b };
    (built(|g| picks[g].0), built(|g| side(picks[g].1, picks[g].0)), built(|g| side(picks[g].2, picks[g].0)))
}

const GONE: &str = "gone by another writer";

fn other_writer((group, value): (usize, usize)) -> Change {
    Box::new(move |p: &mut Payload| {
        if group < KEYS.len() {
            set_group(p, group, value);
        } else {
            p.insert(GONE.into(), json!(true));
        }
    })
}

fn pool_group(group: usize) -> Group {
    match group {
        0 => Group::Known("money"),
        1 => Group::Known("name"),
        2 => Group::Known("date"),
        3 => Group::Known("category"),
        _ => Group::Unknown("later".into()),
    }
}

fn moved_groups(from: &Payload, to: &Payload) -> Vec<usize> {
    (0..KEYS.len()).filter(|&g| rendered(from, g) != rendered(to, g)).collect()
}

/// Picks, other writers' changes before each send (a group past the last deletes the row), base_rev
/// equal to the row's, a lost answer, a change between that loss and the replay, a remove, a server
/// without `rev`, a row gone from the start.
type Case = (Vec<(usize, usize, usize)>, Vec<Option<(usize, usize)>>, bool, bool, Option<(usize, usize)>, bool, bool, bool);

fn cases() -> impl Strategy<Value = Case> {
    (
        prop::collection::vec((0..3usize, 0..6usize, 0..6usize), KEYS.len()),
        prop::collection::vec(prop::option::of((0..=KEYS.len(), 0..3usize)), 0..4),
        any::<bool>(),
        any::<bool>(),
        prop::option::of((0..KEYS.len(), 0..3usize)),
        prop::bool::weighted(0.3),
        prop::bool::weighted(0.25),
        prop::bool::weighted(0.1),
    )
}

/// One replay, and a second one after a lost answer, over a row other writers change between the
/// attempts. Every send the server applied was merged onto the row last read, keeps each group one
/// side's, overwrites only what its user saw, drops no intent and changes something; a delete
/// removes only the row its user saw; all carry the one history id of the entry's generation. What
/// is lost is only what the module documents: without `rev`, a change made between the read and
/// the send; after a lost answer that landed, each group another writer set back to its base value
/// since the landing, before the replay or inside it (sent again), and each group another writer
/// changed since (parked).
fn replays_fairly((picks, edits, same_rev, lose, between, remove, no_rev, gone): Case) -> Result<(), TestCaseError> {
    let (base, theirs, mine) = sides(&picks);
    let mut fake = if gone { Fake::gone() } else { Fake::new(&theirs, 5) };
    fake.serves_rev = !no_rev;
    for change in edits {
        fake.meanwhile.push_back(change.map(other_writer));
    }
    if lose {
        fake.answers.push_back(Forced::Lose);
    }
    let base_rev = if same_rev { 5 } else { 4 };
    let entry = if remove { remove_entry(&base, base_rev) } else { write_entry(&base, base_rev, &mine) };

    let mut outcome = run(&mut fake, &entry);
    let (mut earlier, mut landed_lost) = (0, false);
    if outcome == Outcome::RetryLater(Later::Unreachable) {
        prop_assert!(lose);
        earlier = fake.accepted.len();
        landed_lost = earlier > 0;
        if let (Some((group, value)), true) = (between, fake.row.is_some()) {
            fake.change(|p| set_group(p, group, value));
        }
        outcome = run(&mut fake, &entry);
    }

    for a in &fake.accepted {
        prop_assert!(!a.raced || no_rev, "applied over a change it never read");
        let Some(read) = &a.read else { return Err(TestCaseError::fail("applied with nothing read")) };
        if !a.raced {
            prop_assert_eq!(&a.before, read);
        }
        match &a.after {
            Some(after) => {
                prop_assert!(!remove);
                landed_fairly(&base, &mine, read, after)?;
                prop_assert!(!moved_groups(read, after).is_empty(), "a write that changed nothing was sent");
                for g in moved_groups(&a.before, after) {
                    prop_assert!(rendered(after, g) == rendered(&mine, g) || rendered(&a.before, g) != rendered(read, g), "group {} lost outside the race", g);
                }
            }
            None => {
                prop_assert!(remove);
                prop_assert!(differing_keys(&base, read).is_empty(), "deleted a row its user had not seen");
            }
        }
    }
    let history_ids = fake.writes.iter().map(|w| &w.history).chain(fake.deletes.iter().map(|d| &d.history));
    prop_assert!(history_ids.into_iter().all(|h| h.as_ref().and_then(|h| h.eid) == Some(history_id(ENTRY, 0))), "a history id other than the generation's");
    prop_assert!(fake.history.len() <= 1);
    let now = fake.accepted.len() - earlier;
    prop_assert!(now <= 1);
    let rev_now = || if no_rev { None } else { fake.row.as_ref().map(|r| r.rev) };

    if landed_lost {
        let since = &fake.changes[fake.accepted[earlier - 1].changes_before..];
        let reverted: BTreeSet<usize> = since.iter().flat_map(|(was, then)| {
            (0..KEYS.len()).filter(|&g| rendered(was, g) != rendered(&base, g) && rendered(then, g) == rendered(&base, g))
        }).collect();
        let touched: BTreeSet<Group> = since.iter().flat_map(|(was, then)| moved_groups(was, then)).map(pool_group).collect();
        let resent: BTreeSet<usize> = fake.accepted[earlier..].iter().filter_map(|a| Some(moved_groups(a.read.as_ref()?, a.after.as_ref()?))).flatten().collect();
        prop_assert!(resent.is_subset(&reverted), "sent again {:?} with only {:?} set back to base since the landing", resent, reverted);
        if let Outcome::Parked(Park::Conflict(pending)) = &outcome {
            prop_assert!(pending.choice.conflicts.iter().all(|c| touched.contains(&c.group)), "a landed write parked on {:?}, only {:?} changed since", pending.choice.conflicts, touched);
        }
    }

    match outcome {
        Outcome::Landed { rev, sent: true } if remove => {
            prop_assert!(fake.row.is_none());
            prop_assert_eq!(rev, None);
        }
        Outcome::Landed { rev, sent: true } => {
            prop_assert_eq!(now, 1);
            prop_assert_eq!(rev, rev_now());
        }
        Outcome::Landed { rev: None, sent: false } if remove => {
            prop_assert!(fake.row.is_none());
            prop_assert_eq!(now, 0);
        }
        Outcome::Landed { rev, sent: false } if !remove => {
            prop_assert_eq!(now, 0);
            prop_assert_eq!(rev, rev_now());
            intent_is_there(&base, &mine, &fake.payload())?;
        }
        Outcome::Parked(Park::Gone { mine: shown }) if !remove => {
            prop_assert!(fake.row.is_none());
            prop_assert_eq!(&shown, &mine);
        }
        Outcome::Parked(Park::ChangedSinceRemoved(seen)) if remove => {
            prop_assert_eq!(now, 0);
            prop_assert_eq!(&seen.payload, &fake.payload());
            prop_assert!(!differing_keys(&base, &seen.payload).is_empty());
        }
        Outcome::Parked(Park::Conflict(pending)) if !remove => {
            prop_assert_eq!(now, 0);
            prop_assert_eq!(&pending.seen.payload, &fake.payload());
            prop_assert_eq!(pending.seen.rev, rev_now());
            let both = (0..KEYS.len()).any(|g| {
                let (b, t, m) = (rendered(&base, g), rendered(&pending.seen.payload, g), rendered(&mine, g));
                b != t && b != m && t != m
            });
            prop_assert!(both, "parked with no group changed by both");
            for side in [Side::Theirs, Side::Mine] {
                let mut later = Fake::new(&pending.seen.payload, fake.rev());
                let chosen = match resolve(&pending, |_| side, expense_check, &sealer()) {
                    Resolution::Version(chosen) => chosen,
                    other => return Err(TestCaseError::fail(format!("{other:?}"))),
                };
                let replayed = run(&mut later, &next(&entry, chosen));
                prop_assert!(matches!(replayed, Outcome::Landed { .. }), "{:?}", replayed);
                let row = later.payload();
                for card in &pending.choice.conflicts {
                    let shown = if side == Side::Mine { &card.mine } else { &card.theirs };
                    let keys: BTreeSet<&String> = card.theirs.keys().chain(card.mine.keys()).collect();
                    let landed: Payload = keys.into_iter().filter_map(|k| row.get(k).map(|v| (k.clone(), v.clone()))).collect();
                    prop_assert_eq!(&landed, shown);
                }
            }
        }
        Outcome::RetryLater(Later::Stale) => {
            prop_assert!(!no_rev);
            prop_assert_eq!(now, 0);
        }
        other => prop_assert!(false, "{:?} for a remove: {}", other, remove),
    }
    Ok(())
}

/// Found by the property: after the landing, one revert between the lost answer and the replay and
/// another inside the replay's stale loop, both sent again.
#[test]
fn two_reverts_since_a_landing_at_two_moments_are_both_sent_again() {
    let seeds: [Case; 2] = [
        (vec![(0, 0, 1), (0, 3, 0), (1, 0, 0), (0, 0, 0), (0, 0, 0)], vec![None, Some((2, 1))], false, true, Some((0, 0)), false, false, false),
        (vec![(0, 0, 0), (0, 3, 1), (0, 0, 0), (1, 1, 2), (0, 0, 0)], vec![None, Some((1, 0))], false, true, Some((3, 1)), false, false, false),
    ];
    for case in seeds {
        let shown = format!("{case:?}");
        assert!(replays_fairly(case).is_ok(), "{shown}");
    }
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(1000))]

    #[test]
    fn every_replay_keeps_each_group_one_side_s_and_drops_no_intent(case in cases()) {
        replays_fairly(case)?;
    }
}

fn with_flaw<T>(flaw: Flaw, body: impl FnOnce() -> T) -> T {
    FLAW.with(|f| f.set(Some(flaw)));
    let out = body();
    FLAW.with(|f| f.set(None));
    out
}

fn caught_by_test(flaw: Flaw, test: fn()) -> bool {
    with_flaw(flaw, || std::panic::catch_unwind(test).is_err())
}

fn caught_by_property(flaw: Flaw) -> bool {
    let config = Config { cases: 1000, failure_persistence: None, max_shrink_iters: 0, ..Config::default() };
    with_flaw(flaw, || TestRunner::new(config).run(&cases(), replays_fairly).is_err())
}

#[test]
fn mine_wins_on_a_conflict_is_caught() {
    assert!(caught_by_test(Flaw::MineWins, both_sides_changing_one_group_parks_with_both_values_and_sends_nothing));
    assert!(caught_by_property(Flaw::MineWins));
}

#[test]
fn skipping_the_merge_when_base_rev_equals_the_row_s_rev_is_caught() {
    assert!(caught_by_test(Flaw::SkipMergeOnSameRev, the_merge_runs_even_when_base_rev_equals_the_row_s_rev));
    assert!(caught_by_property(Flaw::SkipMergeOnSameRev));
}

#[test]
fn a_fresh_history_id_per_attempt_is_caught() {
    assert!(caught_by_test(Flaw::FreshHistoryId, every_attempt_of_one_generation_carries_the_same_history_id_and_a_new_generation_another));
    assert!(caught_by_test(Flaw::FreshHistoryId, a_retry_after_a_lost_answer_that_sends_again_is_left_out_of_history));
    assert!(caught_by_property(Flaw::FreshHistoryId));
}

#[test]
fn sending_a_merge_that_changed_nothing_is_caught() {
    assert!(caught_by_test(Flaw::AlwaysSendOnNoChange, a_replay_after_a_lost_answer_sends_nothing_and_writes_no_second_history_entry));
    assert!(caught_by_property(Flaw::AlwaysSendOnNoChange));
}

#[test]
fn dropping_on_a_conflict_is_caught() {
    assert!(caught_by_test(Flaw::DropOnConflict, both_sides_changing_one_group_parks_with_both_values_and_sends_nothing));
    assert!(caught_by_property(Flaw::DropOnConflict));
}
