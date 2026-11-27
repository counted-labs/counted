//! `write_fresh`: a queued `write` or `remove` (mirror/SPEC.md 15.9.5) applied onto the row as it
//! is now (docs/plans/format-v2-f2a.md step 0a, §3.1). It reads the row, merges `intent` onto it
//! from `base` with `shared::merge`, seals at that moment and sends under 15.9.4. It only decides:
//! the caller writes the queue's markers and versions from the [`Outcome`].
//!
//! Known behaviour after a lost answer whose write had landed, when another writer changes the row
//! before the replay reads it. The replay cannot tell its own landing from the row, so:
//!
//! - a group the other writer set back to its `base` value is sent again, over that revert (the
//!   history entry is left out, its id being taken);
//! - a group the other writer changed to a third value parks as a conflict, though the user's
//!   change was already done;
//! - a row the other writer left failing the form's check parks or is refused, though the user's
//!   change was already done.
//!
//! Without a `rev` from the server (rule 10), a change another writer makes between the read and
//! the send is overwritten, as before 3.4.1.
//!
//! [`expense_check`] sees only the payload, so it cannot refuse what the form refuses from its own
//! context: a source currency equal to the project's, an expense type this build does not know,
//! a participant twice on one side, an extra share of zero or less. Only values already in the row
//! can carry those, since the form never writes them.

use std::collections::BTreeSet;
use std::future::Future;

use serde_json::Value;
use sha2::{Digest, Sha256};
use shared::entity::{self, EID, SLOT};
use shared::merge::{self, Choice, Group, Kind, Payload, Side};
use shared::time::Date;
use shared::{errors, EncryptedPair, ExpensePayload, HistoryContext, HistoryPayload, ProjectPayload};
use uuid::Uuid;

/// Reads, merges and sends at most this many times when the server answers `STALE_REV`.
pub const ATTEMPTS: usize = 3;

const ENTITY: Group = Group::Known("entity");

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Slot {
    Expense,
    Project,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Op {
    Write { intent: EncryptedPair },
    /// Slot expense only.
    Remove,
}

/// One queue entry at one generation, its pairs as stored: `base` and `intent` sealed under the
/// project key. For the project row `eid` and `row` are `None` and the row is `project_id`.
#[derive(Clone, Debug, PartialEq)]
pub struct Entry {
    pub id: Uuid,
    pub gen: u64,
    pub project_id: Uuid,
    pub slot: Slot,
    pub eid: Option<Uuid>,
    pub row: Option<i32>,
    pub base_rev: Option<i64>,
    pub base: Option<EncryptedPair>,
    pub op: Op,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Address {
    Expense { project_id: Uuid, row: i32 },
    Project(Uuid),
}

/// A row as the server serves it now. `rev` and `eid` are `None` from a server before 3.4.1.
#[derive(Clone, Debug, PartialEq)]
pub struct Served {
    pub payload: EncryptedPair,
    pub rev: Option<i64>,
    pub eid: Option<Uuid>,
}

/// `Gone` is a 404 with `EXPENSE_NOT_FOUND` or `PROJECT_NOT_FOUND` (SPEC 15.9.5 Q3); any other 404
/// is `Refused`. `Unreachable` is a request that got no answer.
#[derive(Clone, Debug, PartialEq)]
pub enum Read {
    Row(Served),
    Gone,
    Rotating,
    Unreachable,
    Refused { status: u16, code: String },
}

/// `Stale` is 409 `STALE_REV`, and `PROJECT_STALE` for a project write that named `expected_iv`.
/// `Rotating` is 409 `PROJECT_ROTATING`. `Gone` and `Unreachable` as for [`Read`].
#[derive(Clone, Debug, PartialEq)]
pub enum Answer {
    Landed { rev: Option<i64> },
    Stale,
    Gone,
    Rotating,
    Unreachable,
    Refused { status: u16, code: String },
}

/// A write of the row (SPEC 15.9.4): `expected_rev` and `eid` are those served with the row just
/// read, never the entry's. `read_iv` is the iv of that row's pair, for `expected_iv` on a project
/// write. `merged` and `changed` are what was sealed, for the request's clear fields.
#[derive(Clone, Debug, PartialEq)]
pub struct Outgoing {
    pub payload: EncryptedPair,
    pub expected_rev: Option<i64>,
    pub eid: Option<Uuid>,
    pub read_iv: String,
    pub history: Option<HistoryContext>,
    pub merged: Payload,
    pub changed: Vec<Group>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Deletion {
    pub expected_rev: Option<i64>,
    pub eid: Option<Uuid>,
    pub history: Option<HistoryContext>,
}

/// The server, as `write_fresh` needs it.
pub trait Rows {
    fn read(&mut self, at: Address) -> impl Future<Output = Read>;
    fn write(&mut self, at: Address, out: Outgoing) -> impl Future<Output = Answer>;
    fn delete(&mut self, at: Address, out: Deletion) -> impl Future<Output = Answer>;
}

/// `open` gives the plaintext of a pair under the project's current key or one of its previous
/// ones (Q5); `seal` seals under the current key, padded.
pub trait Sealer {
    fn open(&self, pair: &EncryptedPair) -> Result<String, String>;
    fn seal(&self, payload: &Payload) -> Result<EncryptedPair, String>;
}

pub struct KeySealer<'a>(pub &'a [u8; 32]);

impl Sealer for KeySealer<'_> {
    fn open(&self, pair: &EncryptedPair) -> Result<String, String> {
        crate::crypto::decrypt(self.0, pair)
    }

    fn seal(&self, payload: &Payload) -> Result<EncryptedPair, String> {
        crate::crypto::encrypt_padded_json(self.0, payload)
    }
}

/// What the history callback is given when a write or a delete is about to be sent. `id` is the
/// entry's id, bound and sealed by `write_fresh`; `after` is `None` for a delete.
pub struct Landing<'a> {
    pub id: Uuid,
    pub at: Address,
    pub before: &'a Payload,
    pub after: Option<&'a Payload>,
    pub changed: &'a [Group],
}

/// The row as it was read when an entry stopped on it.
#[derive(Clone, Debug, PartialEq)]
pub struct Seen {
    pub pair: EncryptedPair,
    pub payload: Payload,
    pub rev: Option<i64>,
    pub eid: Option<Uuid>,
}

/// A choice to put to the user: `choice.conflicts` are the sheet's pairs of cards, computed on
/// `seen`. Apply the picks with [`resolve`].
#[derive(Clone, Debug, PartialEq)]
pub struct Pending {
    pub choice: Choice,
    pub seen: Seen,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Outcome {
    /// `sent` is false when nothing needed sending: the row already holds the entry, or a remove's
    /// row is gone. `rev` is the row's after the write, when served.
    Landed { rev: Option<i64>, sent: bool },
    Parked(Park),
    RetryLater(Later),
    Refused(Refusal),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Park {
    /// The row and the user changed one group differently, or the merge failed the form's check.
    Conflict(Box<Pending>),
    /// A write whose row is gone: "Add back with my changes" (a creation with a new id) or discard.
    Gone { mine: Payload },
    /// A remove whose row changed since the user deleted it: delete anyway ([`adopt`]) or keep.
    ChangedSinceRemoved(Seen),
    /// An entry with a row and no `base` (a 3.4.0 entry parked by 3.4.1): the whole row against
    /// `mine`, which is `None` for a remove.
    NoBase { seen: Seen, mine: Option<Payload> },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Later {
    Stale,
    Rotating,
    Unreachable,
    Server(u16),
    /// `REV_REQUIRED` on every attempt from a server that served no `rev` when read.
    RevRequired,
}

/// Kept and listed for the user, never sent and never dropped here.
#[derive(Clone, Debug, PartialEq)]
pub enum Refusal {
    EntryDoesNotOpen,
    EntryUnreadable,
    RowDoesNotOpen,
    RowUnreadable,
    /// The row, `base` or `intent` keeps its shares in the `payments` rows.
    Legacy,
    /// The row is served under a clear id other than the entry's `eid`, or its payload names
    /// another entity than `base` or `intent`. Row ids are never reused, so this is never a row
    /// that is gone: only a 404 is.
    OtherRow,
    Unfixable(Vec<Group>),
    EntityConflict,
    Creation,
    Unaddressed,
    Seal(String),
    Server { status: u16, code: String },
}

/// The fields of the entry's next version once its user chose: `base` is the row the choice was
/// made on, `intent` what to write onto it (`None` for a remove). The queue writes it without
/// `state`, keeping the entry's `eid` and `row`, and the next drain replays it.
#[derive(Clone, Debug, PartialEq)]
pub struct NextVersion {
    pub base: EncryptedPair,
    pub base_rev: Option<i64>,
    pub intent: Option<EncryptedPair>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Resolution {
    Version(NextVersion),
    /// A pick the form refuses: the sheet asks again over these groups.
    Again(Box<Pending>),
    Refused(Refusal),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Looked {
    /// The replay would send now: the user has nothing to choose any more.
    WouldSend,
    Now(Outcome),
}

/// The history entry's id for `entry` at `gen`: the first 16 bytes of
/// SHA-256("counted.write_fresh.history.v1" || entry id || gen as 8 bytes big-endian), in the
/// UUID v4 layout. One entry, one generation, one id on every attempt, so a retry whose first
/// attempt landed is left out by the server (SPEC 15.9.1); a new generation is a new write and a
/// new id. The entry id of a write or a remove is never sent, so the server cannot link the two.
pub fn history_id(entry: Uuid, gen: u64) -> Uuid {
    let mut hasher = Sha256::new();
    hasher.update(b"counted.write_fresh.history.v1");
    hasher.update(entry.as_bytes());
    hasher.update(gen.to_be_bytes());
    let digest = hasher.finalize();
    let mut bytes = [0u8; 16];
    bytes.copy_from_slice(&digest[..16]);
    bytes[6] = (bytes[6] & 0x0f) | 0x40;
    bytes[8] = (bytes[8] & 0x3f) | 0x80;
    Uuid::from_bytes(bytes)
}

/// Applies `entry` onto the row as it is now, rules 1 to 10 of item 0a.
pub async fn write_fresh<R: Rows, S: Sealer>(
    rows: &mut R,
    sealer: &S,
    entry: &Entry,
    check: impl Fn(&Payload) -> Vec<Group>,
    mut history: impl FnMut(Landing<'_>) -> Option<HistoryPayload>,
) -> Outcome {
    let opened = match open_entry(sealer, entry) {
        Ok(opened) => opened,
        Err(refusal) => return Outcome::Refused(refusal),
    };
    let mut later = Later::Stale;
    for _ in 0..ATTEMPTS {
        let read = rows.read(opened.at).await;
        let answer = match decide(sealer, entry, &opened, read, &check) {
            Step::Done(outcome) => return outcome,
            Step::Write { seen, payload, changed } => {
                let sealed = match sealer.seal(&payload) {
                    Ok(sealed) => sealed,
                    Err(e) => return Outcome::Refused(Refusal::Seal(e)),
                };
                let landing = Landing { id: entry_history_id(entry), at: opened.at, before: &seen.payload, after: Some(&payload), changed: &changed };
                let history = match sealed_history(sealer, &mut history, landing) {
                    Ok(history) => history,
                    Err(e) => return Outcome::Refused(Refusal::Seal(e)),
                };
                let read_rev = seen.rev;
                let out = Outgoing { payload: sealed, expected_rev: seen.rev, eid: seen.eid, read_iv: seen.pair.iv, history, merged: payload, changed };
                match rows.write(opened.at, out).await {
                    Answer::Landed { rev } => return Outcome::Landed { rev, sent: true },
                    Answer::Gone => return gone(&opened),
                    other => (other, read_rev),
                }
            }
            Step::Delete(seen) => {
                let landing = Landing { id: entry_history_id(entry), at: opened.at, before: &seen.payload, after: None, changed: &[] };
                let history = match sealed_history(sealer, &mut history, landing) {
                    Ok(history) => history,
                    Err(e) => return Outcome::Refused(Refusal::Seal(e)),
                };
                match rows.delete(opened.at, Deletion { expected_rev: seen.rev, eid: seen.eid, history }).await {
                    Answer::Landed { .. } | Answer::Gone => return Outcome::Landed { rev: None, sent: true },
                    other => (other, seen.rev),
                }
            }
        };
        match answer {
            (Answer::Stale, _) => later = Later::Stale,
            (Answer::Refused { status: 409, code }, None) if code == errors::REV_REQUIRED => later = Later::RevRequired,
            (other, _) => return failed(other),
        }
    }
    Outcome::RetryLater(later)
}

/// Rule 8 for an answer that neither landed nor asks to read again.
fn failed(answer: Answer) -> Outcome {
    match answer {
        Answer::Rotating => Outcome::RetryLater(Later::Rotating),
        Answer::Refused { status, code } => refused_or_later(status, code),
        Answer::Unreachable | Answer::Stale | Answer::Landed { .. } | Answer::Gone => Outcome::RetryLater(Later::Unreachable),
    }
}

/// What a replay of `entry` would do now, without sending: the material of a parked entry's sheet
/// after a restart.
pub async fn look<R: Rows, S: Sealer>(rows: &mut R, sealer: &S, entry: &Entry, check: impl Fn(&Payload) -> Vec<Group>) -> Looked {
    let opened = match open_entry(sealer, entry) {
        Ok(opened) => opened,
        Err(refusal) => return Looked::Now(Outcome::Refused(refusal)),
    };
    match decide(sealer, entry, &opened, rows.read(opened.at).await, &check) {
        Step::Done(outcome) => Looked::Now(outcome),
        Step::Write { .. } | Step::Delete(_) => Looked::WouldSend,
    }
}

/// The user's picks applied to a parked conflict: the next version to queue, computed on the row
/// the sheet showed, which the replay merges again if that row has moved since.
pub fn resolve(pending: &Pending, pick: impl Fn(&Group) -> Side, check: impl Fn(&Payload) -> Vec<Group>, sealer: &impl Sealer) -> Resolution {
    match pending.choice.resolve(pick, &check) {
        merge::Outcome::Merged { payload, .. } => match adopt(&pending.seen, Some(&payload), sealer) {
            Ok(version) => Resolution::Version(version),
            Err(e) => Resolution::Refused(Refusal::Seal(e)),
        },
        merge::Outcome::Choose(choice) => match asked(choice, pending.seen.clone()) {
            Ok(pending) => Resolution::Again(Box::new(pending)),
            Err(refusal) => Resolution::Refused(refusal),
        },
        merge::Outcome::Unfixable(groups) => Resolution::Refused(Refusal::Unfixable(groups)),
        merge::Outcome::OtherRow => Resolution::Refused(Refusal::OtherRow),
    }
}

/// The next version for a user who chose, having seen `seen`, to write `intent` onto it, or to
/// delete it when `intent` is `None`.
pub fn adopt(seen: &Seen, intent: Option<&Payload>, sealer: &impl Sealer) -> Result<NextVersion, String> {
    Ok(NextVersion { base: seen.pair.clone(), base_rev: seen.rev, intent: intent.map(|p| sealer.seal(p)).transpose()? })
}

/// The form's whole validation of an expense payload (`expense_modal_helpers`), by group, for
/// `shared::merge`'s `check`: the name, the amount above zero, a payer and a debtor, the date, and
/// a conversion that matches the total. The merge adds that the shares add up.
pub fn expense_check(p: &Payload) -> Vec<Group> {
    let mut refused = Vec::new();
    if p.get("name").and_then(Value::as_str).is_none_or(|n| n.trim().is_empty()) {
        refused.push(Group::Known("name"));
    }
    if !money_passes(p) {
        refused.push(Group::Known("money"));
    }
    if p.get("date").and_then(Value::as_str).and_then(Date::parse_ymd).is_none() {
        refused.push(Group::Known("date"));
    }
    refused
}

/// The project form's validation: a name.
pub fn project_check(p: &Payload) -> Vec<Group> {
    match p.get("name").and_then(Value::as_str).is_none_or(|n| n.trim().is_empty()) {
        true => vec![Group::Known("name")],
        false => Vec::new(),
    }
}

pub fn check_for(slot: Slot) -> fn(&Payload) -> Vec<Group> {
    match slot {
        Slot::Expense => expense_check,
        Slot::Project => project_check,
    }
}

fn money_passes(p: &Payload) -> bool {
    let Some(total) = p.get("amount").and_then(Value::as_f64).filter(|a| *a > 0.0) else { return false };
    let Some(shares) = p.get("shares").and_then(Value::as_array) else { return false };
    let has_side = |is_debt: bool| {
        shares.iter().any(|s| s.get("isDebt").and_then(Value::as_bool) == Some(is_debt) && s.get("amount").and_then(Value::as_f64).is_some_and(|a| a > 0.0))
    };
    has_side(false) && has_side(true) && conversion_passes(p, total)
}

fn conversion_passes(p: &Payload, total: f64) -> bool {
    let (currency, amount, rate) = (p.get("sourceCurrency"), p.get("sourceAmount"), p.get("rate"));
    if currency.is_none() && amount.is_none() && rate.is_none() {
        return true;
    }
    match (currency.and_then(Value::as_str), amount.and_then(Value::as_f64), rate.and_then(Value::as_f64)) {
        (Some(_), Some(amount), Some(rate)) => {
            amount.is_finite() && shared::is_valid_rate(rate) && shared::to_cents(shared::convert_to_project(amount, rate)) == shared::to_cents(total)
        }
        _ => false,
    }
}

struct Opened {
    at: Address,
    kind: Kind,
    base: Option<Payload>,
    mine: Option<Payload>,
}

enum Step {
    Done(Outcome),
    Write { seen: Seen, payload: Payload, changed: Vec<Group> },
    Delete(Seen),
}

enum Unopened {
    DoesNotOpen,
    Unreadable,
}

fn open_entry(sealer: &impl Sealer, entry: &Entry) -> Result<Opened, Refusal> {
    let at = match (entry.slot, entry.row, &entry.op) {
        (Slot::Project, None, Op::Write { .. }) => Address::Project(entry.project_id),
        (Slot::Project, _, _) => return Err(Refusal::Unaddressed),
        (Slot::Expense, Some(row), _) => Address::Expense { project_id: entry.project_id, row },
        (Slot::Expense, None, Op::Write { .. }) if entry.base.is_none() => return Err(Refusal::Creation),
        (Slot::Expense, None, _) => return Err(Refusal::Unaddressed),
    };
    let open = |pair: &EncryptedPair| {
        open_payload(sealer, entry.slot, pair).map_err(|e| match e {
            Unopened::DoesNotOpen => Refusal::EntryDoesNotOpen,
            Unopened::Unreadable => Refusal::EntryUnreadable,
        })
    };
    let base = entry.base.as_ref().map(open).transpose()?;
    let mine = match &entry.op {
        Op::Write { intent } => Some(open(intent)?),
        Op::Remove => None,
    };
    let kind = match entry.slot {
        Slot::Expense => Kind::Expense,
        Slot::Project => Kind::Project,
    };
    Ok(Opened { at, kind, base, mine })
}

/// A payload as `shared::merge` wants it: R1 and R2 passed, decoded by this build's typed payload,
/// then `to_value` of that.
fn open_payload(sealer: &impl Sealer, slot: Slot, pair: &EncryptedPair) -> Result<Payload, Unopened> {
    let plain = sealer.open(pair).map_err(|_| Unopened::DoesNotOpen)?;
    let value = match slot {
        Slot::Expense => {
            entity::read(&plain, entity::Slot::Expense).map_err(|_| Unopened::Unreadable)?;
            serde_json::from_str::<ExpensePayload>(&plain).and_then(serde_json::to_value)
        }
        Slot::Project => serde_json::from_str::<ProjectPayload>(&plain).and_then(serde_json::to_value),
    };
    match value {
        Ok(Value::Object(payload)) => Ok(payload),
        _ => Err(Unopened::Unreadable),
    }
}

fn decide(sealer: &impl Sealer, entry: &Entry, opened: &Opened, read: Read, check: &impl Fn(&Payload) -> Vec<Group>) -> Step {
    let served = match read {
        Read::Row(served) => served,
        Read::Gone => return Step::Done(gone(opened)),
        Read::Rotating => return Step::Done(Outcome::RetryLater(Later::Rotating)),
        Read::Unreachable => return Step::Done(Outcome::RetryLater(Later::Unreachable)),
        Read::Refused { status, code } => return Step::Done(refused_or_later(status, code)),
    };
    if let (Some(queued), Some(now)) = (entry.eid, served.eid) {
        if queued != now {
            return Step::Done(Outcome::Refused(Refusal::OtherRow));
        }
    }
    let theirs = match open_payload(sealer, entry.slot, &served.payload) {
        Ok(theirs) => theirs,
        Err(Unopened::DoesNotOpen) => return Step::Done(Outcome::Refused(Refusal::RowDoesNotOpen)),
        Err(Unopened::Unreadable) => return Step::Done(Outcome::Refused(Refusal::RowUnreadable)),
    };
    let seen = Seen { pair: served.payload, payload: theirs, rev: served.rev, eid: served.eid };
    let Some(base) = &opened.base else {
        return Step::Done(Outcome::Parked(Park::NoBase { seen, mine: opened.mine.clone() }));
    };
    match &opened.mine {
        Some(mine) => merged(entry, opened.kind, base, seen, mine, check),
        None => removed(entry, base, seen),
    }
}

/// Rules 1, 2, 6 and 7: always merged, whatever `base_rev` says.
fn merged(entry: &Entry, kind: Kind, base: &Payload, seen: Seen, mine: &Payload, check: &impl Fn(&Payload) -> Vec<Group>) -> Step {
    if kind == Kind::Expense && [base, &seen.payload, mine].into_iter().any(legacy) {
        return onto_legacy(entry, kind, base, seen, mine, check);
    }
    if flawed(Flaw::SkipMergeOnSameRev) && entry.base_rev.is_some() && entry.base_rev == seen.rev {
        return Step::Write { seen, payload: mine.clone(), changed: merge::groups(kind).iter().map(|(g, _)| Group::Known(g)).collect() };
    }
    match merge::merge(kind, base, &seen.payload, mine, check) {
        merge::Outcome::Merged { payload, changed } if changed.is_empty() && flawed(Flaw::AlwaysSendOnNoChange) => Step::Write { seen, payload, changed },
        merge::Outcome::Merged { changed, .. } if changed.is_empty() => Step::Done(Outcome::Landed { rev: seen.rev, sent: false }),
        merge::Outcome::Merged { payload, changed } => Step::Write { seen, payload, changed },
        merge::Outcome::Choose(choice) if flawed(Flaw::MineWins) => match choice.resolve(|_| Side::Mine, check) {
            merge::Outcome::Merged { payload, changed } => Step::Write { seen, payload, changed },
            _ => Step::Done(Outcome::Refused(Refusal::Unfixable(Vec::new()))),
        },
        merge::Outcome::Choose(_) if flawed(Flaw::DropOnConflict) => Step::Done(Outcome::Landed { rev: seen.rev, sent: false }),
        merge::Outcome::Choose(choice) => match asked(choice, seen) {
            Ok(pending) => Step::Done(Outcome::Parked(Park::Conflict(Box::new(pending)))),
            Err(refusal) => Step::Done(Outcome::Refused(refusal)),
        },
        merge::Outcome::Unfixable(groups) => Step::Done(Outcome::Refused(Refusal::Unfixable(groups))),
        merge::Outcome::OtherRow => Step::Done(Outcome::Refused(Refusal::OtherRow)),
    }
}

/// A legacy row's money is in its `payments` rows, which no comparison of payloads sees. So the
/// intent is sent whole, unmerged, only onto the row exactly as its user saw it and at the `rev`
/// they saw, which the server enforces, since every write of an expense moves `rev`. An intent
/// that is legacy too would make the server drop those rows.
fn onto_legacy(entry: &Entry, kind: Kind, base: &Payload, seen: Seen, mine: &Payload, check: &impl Fn(&Payload) -> Vec<Group>) -> Step {
    if !as_seen(entry, base, &seen) || legacy(mine) {
        return Step::Done(Outcome::Refused(Refusal::Legacy));
    }
    let refused = check(mine);
    if !refused.is_empty() {
        return Step::Done(Outcome::Refused(Refusal::Unfixable(refused)));
    }
    let changed = changed_groups(kind, &seen.payload, mine);
    Step::Write { seen, payload: mine.clone(), changed }
}

fn as_seen(entry: &Entry, base: &Payload, seen: &Seen) -> bool {
    differing_keys(base, &seen.payload).is_empty() && entry.base_rev.is_some() && entry.base_rev == seen.rev
}

/// Rule 5: deleted only as the user saw it, a legacy row only as [`onto_legacy`] writes one. An
/// `eid` or `slot` present on both sides with two values names another entity; one added or
/// dropped since is a change like any other.
fn removed(entry: &Entry, base: &Payload, seen: Seen) -> Step {
    if (legacy(base) || legacy(&seen.payload)) && !as_seen(entry, base, &seen) {
        return Step::Done(Outcome::Refused(Refusal::Legacy));
    }
    let differing = differing_keys(base, &seen.payload);
    if [EID, SLOT].into_iter().any(|k| differing.contains(k) && base.contains_key(k) && seen.payload.contains_key(k)) {
        return Step::Done(Outcome::Refused(Refusal::OtherRow));
    }
    if !differing.is_empty() {
        return Step::Done(Outcome::Parked(Park::ChangedSinceRemoved(seen)));
    }
    Step::Delete(seen)
}

fn asked(choice: Choice, seen: Seen) -> Result<Pending, Refusal> {
    match choice.conflicts.iter().any(|c| c.group == ENTITY) {
        true => Err(Refusal::EntityConflict),
        false => Ok(Pending { choice, seen }),
    }
}

fn gone(opened: &Opened) -> Outcome {
    match &opened.mine {
        Some(mine) => Outcome::Parked(Park::Gone { mine: mine.clone() }),
        None => Outcome::Landed { rev: None, sent: false },
    }
}

/// Rule 8, and SPEC 15.9.5 Q4's "refused for good".
fn refused_or_later(status: u16, code: String) -> Outcome {
    match status >= 500 || status == 426 || status == 429 {
        true => Outcome::RetryLater(Later::Server(status)),
        false => Outcome::Refused(Refusal::Server { status, code }),
    }
}

/// Shares absent or `null`: still in the `payments` rows.
fn legacy(p: &Payload) -> bool {
    matches!(p.get("shares"), None | Some(Value::Null))
}

/// Byte for byte, as `shared::merge` compares: an absent key differs from `null`, `1` from `1.0`.
fn differing_keys(a: &Payload, b: &Payload) -> BTreeSet<String> {
    let render = |p: &Payload, k: &String| p.get(k).map(|v| serde_json::to_vec(v).ok());
    a.keys().chain(b.keys()).filter(|k| render(a, k) != render(b, k)).cloned().collect()
}

/// The groups of `kind` in which `to` differs from `from`, as `shared::merge` names them.
fn changed_groups(kind: Kind, from: &Payload, to: &Payload) -> Vec<Group> {
    let differing = differing_keys(from, to);
    let table = merge::groups(kind);
    let known = table.iter().filter(|(_, keys)| keys.iter().any(|k| differing.contains(*k))).map(|(name, _)| Group::Known(name));
    let named = |k: &String| table.iter().any(|(_, keys)| keys.contains(&k.as_str()));
    known.chain(differing.iter().filter(|k| !named(k)).cloned().map(Group::Unknown)).collect()
}

fn entry_history_id(entry: &Entry) -> Uuid {
    match flawed(Flaw::FreshHistoryId) {
        true => Uuid::new_v4(),
        false => history_id(entry.id, entry.gen),
    }
}

fn sealed_history(
    sealer: &impl Sealer,
    build: &mut impl FnMut(Landing<'_>) -> Option<HistoryPayload>,
    landing: Landing<'_>,
) -> Result<Option<HistoryContext>, String> {
    let id = landing.id;
    let Some(mut payload) = build(landing) else { return Ok(None) };
    entity::bind(&mut payload.extra, entity::Slot::History, id);
    let Value::Object(sealed) = serde_json::to_value(&payload).map_err(|e| e.to_string())? else {
        return Err("a history payload that is not an object".into());
    };
    Ok(Some(HistoryContext { actor_user_id: None, payload: sealer.seal(&sealed)?, eid: Some(id) }))
}

/// Deliberately broken variants, switched on only by the tests, each of which some test must catch.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Flaw {
    MineWins,
    SkipMergeOnSameRev,
    FreshHistoryId,
    DropOnConflict,
    AlwaysSendOnNoChange,
}

#[cfg(test)]
thread_local! {
    static FLAW: std::cell::Cell<Option<Flaw>> = const { std::cell::Cell::new(None) };
}

#[cfg(test)]
fn flawed(flaw: Flaw) -> bool {
    FLAW.with(|f| f.get() == Some(flaw))
}

#[cfg(not(test))]
fn flawed(_: Flaw) -> bool {
    false
}

#[cfg(test)]
#[path = "write_fresh_tests.rs"]
mod tests;
