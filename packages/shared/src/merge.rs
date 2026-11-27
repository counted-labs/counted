//! The three-way merge by group of a queued edit onto the row as it is now (docs/plans/format-v2-f2a.md
//! §3.1). Pure: the caller decrypts `base`, `theirs` and `mine`, and seals what comes back.

use serde_json::{Map, Value};
use std::collections::BTreeSet;

use crate::entity::{EID, SLOT};
use crate::{is_valid_amount, sums_to_total};

/// A decrypted payload as `serde_json::to_value` of its typed form gives it.
pub type Payload = Map<String, Value>;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    Expense,
    Rule,
    Participant,
    Project,
}

/// `Unknown` is a top-level key no group of the kind names: a group of its own, except for a
/// participant, whose one group owns it.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Group {
    Known(&'static str),
    Unknown(String),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Side {
    Theirs,
    Mine,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Reason {
    BothChanged,
    Invalid,
}

/// One pair of cards: the group's keys as the row holds them now, and as the user left them. A key
/// absent from a side is absent from its map. The two always differ.
#[derive(Clone, Debug, PartialEq)]
pub struct Conflict {
    pub group: Group,
    pub reason: Reason,
    pub theirs: Payload,
    pub mine: Payload,
}

/// At least one pick of a side per conflict lands.
#[derive(Clone, Debug, PartialEq)]
pub struct Choice {
    kind: Kind,
    base: Payload,
    theirs: Payload,
    mine: Payload,
    draft: Payload,
    pub conflicts: Vec<Conflict>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Outcome {
    /// `changed` names the groups that differ from `theirs`: empty means there is nothing to write.
    Merged { payload: Payload, changed: Vec<Group> },
    Choose(Choice),
    /// No pick of a side for these groups gives a payload that passes: the op cannot land as it is.
    Unfixable(Vec<Group>),
    /// `base` and `theirs`, or `base` and `mine`, carry two different `eid`s or two different `slot`s:
    /// not the same row: the caller refuses the op and keeps it (`ui::common::write_fresh`).
    OtherRow,
}

type Table = &'static [(&'static str, &'static [&'static str])];

const ENTITY: (&str, &[&str]) = ("entity", &[EID, SLOT]);

const EXPENSE: Table = &[
    (MONEY, &["amount", "expenseType", "shares", "sourceCurrency", "sourceAmount", "rate", "estimate"]),
    ("name", &["name"]),
    ("date", &["date"]),
    ("description", &["description"]),
    ("category", &["category"]),
    ("author", &["authorId"]),
    ("recurring", &["recurringId"]),
    ENTITY,
];

const RULE: Table = &[
    ("template", &["template"]),
    (SCHEDULE, &[SCHEDULE_KEY, NEXT]),
    ("paused", &["paused"]),
    ("seed", &["seed"]),
    ("author", &["authorId"]),
    ENTITY,
];

const PARTICIPANT: Table = &[(PARTICIPANT_GROUP, &["name", "removed"]), ENTITY];

const PROJECT: Table = &[
    ("name", &["name", "description"]),
    ("status", &["status"]),
    ("emoji", &["emoji"]),
    ("currency", &["currency"]),
    ("kind", &["kind"]),
    ("budgets", &["budgets"]),
    ("budgetsCustom", &["budgetsCustom"]),
    ("categories", &["categories"]),
];

const MONEY: &str = "money";
const SCHEDULE: &str = "schedule";
const SCHEDULE_KEY: &str = "rule";
const NEXT: &str = "next";
const PARTICIPANT_GROUP: &str = "participant";

/// Above this many groups to choose in, no pick is searched and the op is `Unfixable`.
const MAX_OPEN: usize = 12;

/// The most calls of `check` one `merge` or one `resolve` makes, all searches included. Once spent,
/// the op is `Unfixable`, even if a pick not yet tried would have landed.
const MAX_CHECKS: usize = 4096;

/// Every group of `kind` and the keys it owns, in the order conflicts are listed. The participant
/// group also owns every key no other participant group names.
pub fn groups(kind: Kind) -> Table {
    match kind {
        Kind::Expense => EXPENSE,
        Kind::Rule => RULE,
        Kind::Participant => PARTICIPANT,
        Kind::Project => PROJECT,
    }
}

/// Merges `mine` onto `theirs` by group, `base` being what the user started from.
///
/// What the caller owes, none of which the merge can check:
///
/// - `check` carries the form's whole validation of a payload and names the groups it refuses: for
///   an expense, the amount above zero, payers and debtors not empty, the conversion matching the
///   total (`packages/ui/src/expenses/helpers/expense_modal_helpers.rs`). The merge itself only
///   refuses an expense whose amount is out of range or whose shares do not add up to it. A refused
///   key counts as the group that owns it; a name that is no group of these payloads cannot be
///   picked and makes the op `Unfixable` unless another pick passes. `check` is called at most
///   `MAX_CHECKS` (4096) times per `merge` and per `resolve`.
/// - All three payloads are `serde_json::to_value` of the typed payload of `kind`, decoded by this
///   build after `shared::entity::read` accepted their plaintext (R1, R2), so the three are written
///   alike and a payload the typed decode refuses never gets here.
/// - `kind` is the kind of the three payloads.
/// - A write built from the result, or from [`Choice::resolve`], carries the `rev` of the `theirs`
///   it was computed on. A `theirs` read later is merged again, from the same `base`.
/// - An expense whose shares are still in the `payments` rows (no `shares` key) has a money group
///   this merge cannot see whole: it is not fed here as if it had no shares.
/// - For a rule, `paused` and the schedule are two groups, but resuming writes both
///   (`packages/ui/src/recurring/actions.rs:18-20`). When the merged rule is not paused and its
///   schedule came from the other side than its `paused`, the caller recomputes `next` before
///   writing.
pub fn merge(
    kind: Kind,
    base: &Payload,
    theirs: &Payload,
    mine: &Payload,
    check: impl Fn(&Payload) -> Vec<Group>,
) -> Outcome {
    if other_row(base, theirs, mine) {
        return Outcome::OtherRow;
    }
    let mut choice = Choice {
        kind,
        base: base.clone(),
        theirs: theirs.clone(),
        mine: mine.clone(),
        draft: Payload::new(),
        conflicts: Vec::new(),
    };
    let mut asks = BTreeSet::new();
    for group in choice.groups() {
        let keys = choice.keys(&group);
        let side = pick(kind, &group, &keys, base, theirs, mine);
        copy(&keys, if side == Some(Side::Mine) { mine } else { theirs }, &mut choice.draft);
        if side.is_none() {
            asks.insert(group);
        }
    }
    let mut left = MAX_CHECKS;
    if !asks.is_empty() {
        return choice.offer(asks.clone(), asks, BTreeSet::new(), &mut left, &check);
    }
    let refused = choice.refused(&choice.draft, &mut left, &check);
    if refused.lands() {
        let draft = choice.draft.clone();
        return choice.landed(draft);
    }
    choice.offer(refused.groups, BTreeSet::new(), refused.stray, &mut left, &check)
}

/// What one call of `check` refused: groups of these payloads, and names that are none.
struct Refused {
    groups: BTreeSet<Group>,
    stray: BTreeSet<Group>,
}

impl Refused {
    fn lands(&self) -> bool {
        self.groups.is_empty() && self.stray.is_empty()
    }
}

impl Choice {
    /// The payload with `pick`'s side for each conflict. One that fails `check` asks again, with
    /// the groups that failed added, or is `Unfixable`.
    pub fn resolve(&self, pick: impl Fn(&Group) -> Side, check: impl Fn(&Payload) -> Vec<Group>) -> Outcome {
        let mut left = MAX_CHECKS;
        let attempt = self.apply(self.conflicts.iter().map(|c| (&c.group, pick(&c.group))));
        let refused = self.refused(&attempt, &mut left, &check);
        if refused.lands() {
            return self.landed(attempt);
        }
        let set = self.conflicts.iter().map(|c| c.group.clone()).chain(refused.groups).collect();
        let both = self.conflicts.iter().filter(|c| c.reason == Reason::BothChanged).map(|c| c.group.clone()).collect();
        self.clone().offer(set, both, refused.stray, &mut left, &check)
    }

    fn groups(&self) -> Vec<Group> {
        groups_of(self.kind, &[&self.base, &self.theirs, &self.mine])
    }

    fn keys(&self, group: &Group) -> Vec<String> {
        keys_of(self.kind, group, &[&self.base, &self.theirs, &self.mine])
    }

    /// The group of these payloads a refusal names: itself, or the group that owns the key.
    fn owner(&self, group: Group) -> Option<Group> {
        match &group {
            Group::Known(name) => groups(self.kind).iter().any(|(n, _)| n == name).then_some(group),
            Group::Unknown(key) => match groups(self.kind).iter().find(|(_, keys)| keys.contains(&key.as_str())) {
                Some((name, _)) => Some(Group::Known(name)),
                None if self.kind == Kind::Participant => Some(Group::Known(PARTICIPANT_GROUP)),
                None => [&self.base, &self.theirs, &self.mine].iter().any(|p| p.contains_key(key)).then_some(group),
            },
        }
    }

    fn apply<'a>(&self, picks: impl IntoIterator<Item = (&'a Group, Side)>) -> Payload {
        let mut out = self.draft.clone();
        for (group, side) in picks {
            let keys = self.keys(group);
            for key in &keys {
                out.remove(key);
            }
            copy(&keys, if side == Side::Theirs { &self.theirs } else { &self.mine }, &mut out);
        }
        out
    }

    fn refused(&self, payload: &Payload, left: &mut usize, check: &impl Fn(&Payload) -> Vec<Group>) -> Refused {
        *left -= 1;
        let mut refused = Refused { groups: BTreeSet::new(), stray: BTreeSet::new() };
        for group in check(payload) {
            match self.owner(group.clone()) {
                Some(owner) => refused.groups.insert(owner),
                None => refused.stray.insert(group),
            };
        }
        if self.kind == Kind::Expense && !money_adds_up(payload) {
            refused.groups.insert(Group::Known(MONEY));
        }
        refused
    }

    fn landed(&self, payload: Payload) -> Outcome {
        let changed = groups_of(self.kind, &[&self.base, &self.theirs, &self.mine])
            .into_iter()
            .filter(|g| !same(&self.keys(g), &payload, &self.theirs))
            .collect();
        Outcome::Merged { payload, changed }
    }

    /// Widens `set` with every group a failed pick gets refused on, until some pick of sides over
    /// it lands (then it is asked) or the refusals bring nothing new, the groups are too many or the
    /// checks are spent (then it is `Unfixable`). `both` are the groups both sides changed.
    fn offer(
        mut self,
        mut set: BTreeSet<Group>,
        both: BTreeSet<Group>,
        mut stray: BTreeSet<Group>,
        left: &mut usize,
        check: &impl Fn(&Payload) -> Vec<Group>,
    ) -> Outcome {
        loop {
            let open: Vec<Group> = set.iter().filter(|g| !same(&self.keys(g), &self.theirs, &self.mine)).cloned().collect();
            if open.len() > MAX_OPEN {
                return self.unfixable(set, stray);
            }
            let mut refusals = BTreeSet::new();
            let mut lands = false;
            for mask in 0u32..1 << open.len() {
                if *left == 0 {
                    return self.unfixable(set, stray);
                }
                let sides = open.iter().enumerate().map(|(i, g)| (g, if mask >> i & 1 == 1 { Side::Mine } else { Side::Theirs }));
                let refused = self.refused(&self.apply(sides), left, check);
                lands = refused.lands();
                refusals.extend(refused.groups);
                stray.extend(refused.stray);
                if lands {
                    break;
                }
            }
            if lands {
                self.conflicts = ordered(self.kind, open.into_iter().collect())
                    .into_iter()
                    .map(|group| {
                        let reason = if both.contains(&group) { Reason::BothChanged } else { Reason::Invalid };
                        let keys = self.keys(&group);
                        Conflict { reason, theirs: part(&keys, &self.theirs), mine: part(&keys, &self.mine), group }
                    })
                    .collect();
                return Outcome::Choose(self);
            }
            if refusals.is_subset(&set) {
                return self.unfixable(set, stray);
            }
            set.extend(refusals);
        }
    }

    fn unfixable(&self, mut set: BTreeSet<Group>, stray: BTreeSet<Group>) -> Outcome {
        set.extend(stray);
        Outcome::Unfixable(ordered(self.kind, set))
    }
}

/// Two of the three name different rows, or one carries `eid` without `slot` or `slot` without
/// `eid` (SPEC 15.9.2 R2).
fn other_row(base: &Payload, theirs: &Payload, mine: &Payload) -> bool {
    let half = |p: &Payload| p.contains_key(EID) != p.contains_key(SLOT);
    let differ = |a: &Payload, b: &Payload| [EID, SLOT].into_iter().any(|key| a.contains_key(key) && b.contains_key(key) && !same(&[key], a, b));
    half(base) || half(theirs) || half(mine) || differ(base, theirs) || differ(base, mine)
}

fn named(kind: Kind, key: &str) -> bool {
    groups(kind).iter().any(|(_, keys)| keys.contains(&key))
}

fn groups_of(kind: Kind, payloads: &[&Payload]) -> Vec<Group> {
    let unknown: BTreeSet<Group> = match kind {
        Kind::Participant => BTreeSet::new(),
        _ => payloads.iter().flat_map(|p| p.keys()).filter(|k| !named(kind, k)).map(|k| Group::Unknown(k.clone())).collect(),
    };
    known_groups(kind).chain(unknown).collect()
}

fn keys_of(kind: Kind, group: &Group, payloads: &[&Payload]) -> Vec<String> {
    match group {
        Group::Known(name) => {
            let Some((_, keys)) = groups(kind).iter().find(|(n, _)| n == name) else { return Vec::new() };
            let mut keys: Vec<String> = keys.iter().map(|k| k.to_string()).collect();
            if kind == Kind::Participant && *name == PARTICIPANT_GROUP {
                let rest: BTreeSet<&String> = payloads.iter().flat_map(|p| p.keys()).filter(|k| !named(kind, k)).collect();
                keys.extend(rest.into_iter().cloned());
            }
            keys
        }
        Group::Unknown(key) if kind != Kind::Participant && !named(kind, key) => vec![key.clone()],
        Group::Unknown(_) => Vec::new(),
    }
}

fn known_groups(kind: Kind) -> impl Iterator<Item = Group> {
    groups(kind).iter().map(|(name, _)| Group::Known(name))
}

fn ordered(kind: Kind, set: BTreeSet<Group>) -> Vec<Group> {
    let table: Vec<Group> = known_groups(kind).collect();
    let mut out: Vec<Group> = table.iter().filter(|g| set.contains(g)).cloned().collect();
    out.extend(set.into_iter().filter(|g| !table.contains(g)));
    out
}

/// `None` when both sides changed the group to different values.
fn pick(kind: Kind, group: &Group, keys: &[String], base: &Payload, theirs: &Payload, mine: &Payload) -> Option<Side> {
    let changed_by = (!same(keys, base, theirs), !same(keys, base, mine));
    match changed_by {
        (_, false) => Some(Side::Theirs),
        (false, true) => Some(Side::Mine),
        (true, true) if same(keys, theirs, mine) => Some(Side::Theirs),
        (true, true) if kind == Kind::Rule && *group == Group::Known(SCHEDULE) => larger_next(theirs, mine),
        (true, true) => None,
    }
}

/// Both sides on one schedule: `next` counts the same occurrences, which are idempotent by date, so
/// the larger index loses nothing. Across two schedules an index means nothing on the other one: a
/// re-anchored rule restarts at 0, and the larger index would skip its first occurrences.
fn larger_next(theirs: &Payload, mine: &Payload) -> Option<Side> {
    if !same(&[SCHEDULE_KEY], theirs, mine) {
        return None;
    }
    let (t, m) = (theirs.get(NEXT)?.as_u64()?, mine.get(NEXT)?.as_u64()?);
    Some(if m > t { Side::Mine } else { Side::Theirs })
}

/// Byte for byte, key by key: an absent key differs from `null`, `1` from `1.0`, `0.0` from `-0.0`.
fn same<K: AsRef<str>>(keys: &[K], a: &Payload, b: &Payload) -> bool {
    keys.iter().all(|key| match (a.get(key.as_ref()), b.get(key.as_ref())) {
        (None, None) => true,
        (Some(x), Some(y)) => matches!((serde_json::to_vec(x), serde_json::to_vec(y)), (Ok(x), Ok(y)) if x == y),
        _ => false,
    })
}

fn copy<K: AsRef<str>>(keys: &[K], from: &Payload, to: &mut Payload) {
    for key in keys {
        if let Some(value) = from.get(key.as_ref()) {
            to.insert(key.as_ref().to_string(), value.clone());
        }
    }
}

fn part<K: AsRef<str>>(keys: &[K], p: &Payload) -> Payload {
    let mut out = Payload::new();
    copy(keys, p, &mut out);
    out
}

/// A valid amount, and when the shares are inside the payload, payers and debtors each adding up to
/// it to the cent. Shares absent or `null` are still in the `payments` rows.
fn money_adds_up(p: &Payload) -> bool {
    let Some(total) = p.get("amount").and_then(Value::as_f64).filter(|a| is_valid_amount(*a)) else { return false };
    let shares = match p.get("shares") {
        None | Some(Value::Null) => return true,
        Some(Value::Array(shares)) => shares,
        Some(_) => return false,
    };
    let (mut paid, mut owed) = (Vec::new(), Vec::new());
    for share in shares {
        let amount = share.get("amount").and_then(Value::as_f64).filter(|a| is_valid_amount(*a));
        match (amount, share.get("isDebt").and_then(Value::as_bool)) {
            (Some(a), Some(false)) => paid.push(a),
            (Some(a), Some(true)) => owed.push(a),
            _ => return false,
        }
    }
    sums_to_total(total, paid) && sums_to_total(total, owed)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entity::{self, Slot};
    use crate::{Budgets, ExpensePayload, ExpenseShare, ProjectPayload, ProjectStatus, UserPayload};
    use proptest::prelude::*;
    use serde_json::json;
    use std::collections::BTreeMap;
    use uuid::Uuid;

    const A: &str = "0f8e2c4a-6b1d-4e3f-9a7b-5c2d1e0f3a4b";
    const B: &str = "7d2c9b1e-3a4f-4c5d-8e6f-0a1b2c3d4e5f";
    const KINDS: [Kind; 4] = [Kind::Expense, Kind::Rule, Kind::Participant, Kind::Project];

    fn p(v: Value) -> Payload {
        match v {
            Value::Object(m) => m,
            other => panic!("not an object: {other}"),
        }
    }

    fn accept(_: &Payload) -> Vec<Group> {
        Vec::new()
    }

    fn merged(o: Outcome) -> Payload {
        match o {
            Outcome::Merged { payload, .. } => payload,
            other => panic!("not merged: {other:?}"),
        }
    }

    fn asked(o: Outcome) -> Choice {
        match o {
            Outcome::Choose(c) => c,
            other => panic!("not asked: {other:?}"),
        }
    }

    fn reasons(c: &Choice) -> Vec<(Group, Reason)> {
        c.conflicts.iter().map(|c| (c.group.clone(), c.reason)).collect()
    }

    fn edit(from: &Payload, set: Value, remove: &[&str]) -> Payload {
        let mut out = from.clone();
        for key in remove {
            out.remove(*key);
        }
        out.extend(p(set));
        out
    }

    fn known(name: &'static str) -> Group {
        Group::Known(name)
    }

    fn unknown(key: &str) -> Group {
        Group::Unknown(key.to_string())
    }

    fn table(kind: Kind) -> Vec<(&'static str, Vec<&'static str>)> {
        groups(kind).iter().map(|(name, keys)| (*name, keys.to_vec())).collect()
    }

    fn table_keys(kind: Kind) -> BTreeSet<String> {
        groups(kind).iter().flat_map(|(_, keys)| keys.iter().map(|k| k.to_string())).collect()
    }

    fn typed_keys<T: serde::Serialize>(typed: &T) -> BTreeSet<String> {
        p(serde_json::to_value(typed).unwrap()).keys().cloned().collect()
    }

    fn id(s: &str) -> Uuid {
        Uuid::parse_str(s).unwrap()
    }

    #[test]
    fn expense_groups_own_every_key_of_the_expense_payload() {
        assert_eq!(
            table(Kind::Expense),
            vec![
                ("money", vec!["amount", "expenseType", "shares", "sourceCurrency", "sourceAmount", "rate", "estimate"]),
                ("name", vec!["name"]),
                ("date", vec!["date"]),
                ("description", vec!["description"]),
                ("category", vec!["category"]),
                ("author", vec!["authorId"]),
                ("recurring", vec!["recurringId"]),
                ("entity", vec!["eid", "slot"]),
            ]
        );
        let mut every_field = ExpensePayload {
            name: "Taxi".into(),
            amount: 12.5,
            expense_type: "expense".into(),
            date: "2026-11-23".into(),
            description: Some("Airport".into()),
            category: Some("Transport".into()),
            source_currency: Some("USD".into()),
            source_amount: Some(13.02),
            rate: Some(0.96),
            recurring_id: Some(id(B)),
            estimate: true,
            author_id: Some(1),
            shares: Some(vec![ExpenseShare { user_id: 1, amount: 12.5, is_debt: false, extra: Default::default() }]),
            extra: Default::default(),
        };
        entity::bind(&mut every_field.extra, Slot::Expense, id(A));
        assert_eq!(typed_keys(&every_field), table_keys(Kind::Expense));
    }

    #[test]
    fn participant_groups_own_every_key_of_the_participant_payload() {
        assert_eq!(table(Kind::Participant), vec![("participant", vec!["name", "removed"]), ("entity", vec!["eid", "slot"])]);
        let mut every_field = UserPayload { name: "Ana".into(), removed: true, extra: Default::default() };
        entity::bind(&mut every_field.extra, Slot::Participant, id(A));
        assert_eq!(typed_keys(&every_field), table_keys(Kind::Participant));

        every_field.extra.insert("later".into(), json!(1));
        let payload = p(serde_json::to_value(&every_field).unwrap());
        assert_eq!(groups_of(Kind::Participant, &[&payload]), vec![known("participant"), known("entity")]);
        assert_eq!(keys_of(Kind::Participant, &known("participant"), &[&payload]), vec!["name", "removed", "later"]);
        assert_eq!(keys_of(Kind::Participant, &unknown("later"), &[&payload]), Vec::<String>::new());
    }

    #[test]
    fn project_groups_own_every_key_of_the_project_payload() {
        assert_eq!(
            table(Kind::Project),
            vec![
                ("name", vec!["name", "description"]),
                ("status", vec!["status"]),
                ("emoji", vec!["emoji"]),
                ("currency", vec!["currency"]),
                ("kind", vec!["kind"]),
                ("budgets", vec!["budgets"]),
                ("budgetsCustom", vec!["budgetsCustom"]),
                ("categories", vec!["categories"]),
            ]
        );
        let mut every_field = ProjectPayload {
            name: "Ski".into(),
            currency: "EUR".into(),
            description: Some("Tignes".into()),
            status: Some(ProjectStatus::Closed),
            extra: Default::default(),
        };
        every_field.set_emoji(Some("\u{2600}"));
        every_field.set_budgets(&Budgets { monthly: BTreeMap::from([("Transport".into(), 10.0)]), alert: true });
        every_field.merge_budgets_custom(&["c-1".into()], &BTreeMap::from([("c-1".into(), 5.0)]));
        every_field.extra.insert("kind".into(), "personal".into());
        every_field.extra.insert("categories".into(), json!([]));
        assert_eq!(typed_keys(&every_field), table_keys(Kind::Project));
    }

    #[test]
    fn rule_groups_own_every_key_of_the_rule_payload_as_written_by_recurring_model() {
        assert_eq!(
            table(Kind::Rule),
            vec![
                ("template", vec!["template"]),
                ("schedule", vec!["rule", "next"]),
                ("paused", vec!["paused"]),
                ("seed", vec!["seed"]),
                ("author", vec!["authorId"]),
                ("entity", vec!["eid", "slot"]),
            ]
        );
        let written = p(json!({
            "template": {"name": "Rent", "expenseType": "expense", "category": null, "amount": 1450.0,
                "payers": {"exact": true, "entries": [[1, 1450.0]]}, "debtors": {"exact": false, "entries": [[1, 1.0]]},
                "sourceCurrency": null, "sourceAmount": null, "rate": null, "rateAck": null, "variable": false},
            "rule": {"freq": "monthly", "interval": 1, "anchor": "2026-08-01", "ends": {"kind": "never"}},
            "next": 2, "paused": false, "seed": A, "authorId": 1, "eid": B, "slot": "rule"
        }));
        assert_eq!(written.keys().cloned().collect::<BTreeSet<_>>(), table_keys(Kind::Rule));
    }

    #[test]
    fn no_key_and_no_group_name_appears_twice_in_a_kind() {
        for kind in KINDS {
            let keys: Vec<&str> = groups(kind).iter().flat_map(|(_, keys)| keys.iter().copied()).collect();
            assert_eq!(keys.len(), keys.iter().collect::<BTreeSet<_>>().len(), "{kind:?}");
            let names: Vec<&str> = groups(kind).iter().map(|(name, _)| *name).collect();
            assert_eq!(names.len(), names.iter().collect::<BTreeSet<_>>().len(), "{kind:?}");
        }
    }

    #[test]
    fn a_key_no_group_names_is_a_group_of_its_own() {
        let found = groups_of(Kind::Expense, &[&p(json!({"name": "x", "later": 1})), &p(json!({"zz": 2, "later": 3}))]);
        assert_eq!(found[EXPENSE.len()..], [unknown("later"), unknown("zz")]);
        assert_eq!(keys_of(Kind::Expense, &unknown("later"), &[]), vec!["later"]);
    }

    fn taxi() -> Payload {
        p(json!({
            "name": "Taxi", "amount": 30.0, "expenseType": "expense", "date": "2026-11-23",
            "shares": [{"userId": 1, "amount": 30.0, "isDebt": false},
                       {"userId": 1, "amount": 15.0, "isDebt": true}, {"userId": 2, "amount": 15.0, "isDebt": true}],
            "authorId": 1, "eid": A, "slot": "expense", "later": {"k": 1}
        }))
    }

    fn priced(amount: f64) -> Value {
        json!({"amount": amount, "shares": [{"userId": 1, "amount": amount, "isDebt": false}, {"userId": 2, "amount": amount, "isDebt": true}]})
    }

    #[test]
    fn edits_of_two_groups_both_land() {
        let base = taxi();
        let theirs = edit(&base, json!({"name": "Train"}), &[]);
        let mine = edit(&base, priced(42.0), &[]);
        let Outcome::Merged { payload, changed } = merge(Kind::Expense, &base, &theirs, &mine, accept) else { panic!() };
        assert_eq!(payload, edit(&mine, json!({"name": "Train"}), &[]));
        assert_eq!(changed, vec![known("money")]);
    }

    #[test]
    fn one_group_changed_differently_by_both_asks_with_both_values_and_takes_the_choice() {
        let base = taxi();
        let theirs = edit(&base, json!({"name": "Train", "date": "2026-11-24"}), &[]);
        let mine = edit(&base, json!({"name": "Bus", "category": "Transport"}), &[]);
        let choice = asked(merge(Kind::Expense, &base, &theirs, &mine, accept));
        assert_eq!(reasons(&choice), vec![(known("name"), Reason::BothChanged)]);
        assert_eq!(choice.conflicts[0].theirs, p(json!({"name": "Train"})));
        assert_eq!(choice.conflicts[0].mine, p(json!({"name": "Bus"})));

        let landed = |side: Side| merged(choice.resolve(|_| side, accept));
        let both = edit(&base, json!({"date": "2026-11-24", "category": "Transport"}), &[]);
        assert_eq!(landed(Side::Mine), edit(&both, json!({"name": "Bus"}), &[]));
        assert_eq!(landed(Side::Theirs), edit(&both, json!({"name": "Train"}), &[]));
    }

    #[test]
    fn a_group_changed_to_equal_values_by_both_is_no_conflict() {
        let base = taxi();
        let theirs = edit(&base, json!({"description": "Airport", "name": "Train"}), &[]);
        let mine = edit(&base, json!({"description": "Airport"}), &[]);
        assert_eq!(merged(merge(Kind::Expense, &base, &theirs, &mine, accept)), theirs);
    }

    #[test]
    fn money_is_never_merged_key_by_key() {
        let base = taxi();
        let theirs = edit(&base, json!({"expenseType": "gain"}), &[]);
        let mine = edit(&base, json!({"estimate": true}), &[]);
        let choice = asked(merge(Kind::Expense, &base, &theirs, &mine, accept));
        assert_eq!(reasons(&choice), vec![(known("money"), Reason::BothChanged)]);
        assert_eq!(choice.conflicts[0].theirs["expenseType"], "gain");
        assert!(!choice.conflicts[0].theirs.contains_key("estimate"));
        assert_eq!(choice.conflicts[0].mine["estimate"], true);
    }

    #[test]
    fn shares_that_do_not_add_up_ask_even_from_one_side_and_choosing_mine_keeps_what_the_user_saw() {
        let base = taxi();
        let broken = json!([{"userId": 1, "amount": 29.99, "isDebt": false}, {"userId": 2, "amount": 30.0, "isDebt": true}]);
        let theirs = edit(&base, json!({"shares": broken}), &[]);
        let mine = edit(&base, json!({"name": "Bus"}), &[]);
        let choice = asked(merge(Kind::Expense, &base, &theirs, &mine, accept));
        assert_eq!(reasons(&choice), vec![(known("money"), Reason::Invalid)]);
        assert_eq!(merged(choice.resolve(|_| Side::Mine, accept)), mine);
        let again = asked(choice.resolve(|_| Side::Theirs, accept));
        assert_eq!(reasons(&again), vec![(known("money"), Reason::Invalid)]);
    }

    #[test]
    fn money_that_cannot_be_read_asks() {
        let base = taxi();
        let mine = edit(&base, json!({"name": "Bus"}), &[]);
        for bad in [
            json!({"amount": 1e13, "shares": null}),
            json!({"amount": "30"}),
            json!({"shares": {"userId": 1}}),
            json!({"shares": [{"userId": 1, "amount": 30.0}]}),
            json!({"shares": [{"userId": 1, "amount": "30", "isDebt": false}]}),
            json!({"shares": [7]}),
        ] {
            let theirs = edit(&base, bad.clone(), &[]);
            let choice = asked(merge(Kind::Expense, &base, &theirs, &mine, accept));
            assert_eq!(reasons(&choice), vec![(known("money"), Reason::Invalid)], "{bad}");
        }
        let theirs = edit(&base, json!({}), &["amount"]);
        assert_eq!(reasons(&asked(merge(Kind::Expense, &base, &theirs, &mine, accept))), vec![(known("money"), Reason::Invalid)]);
        let legacy = edit(&base, json!({}), &["shares"]);
        assert_eq!(merged(merge(Kind::Expense, &legacy, &legacy, &edit(&legacy, json!({"name": "Bus"}), &[]), accept))["name"], "Bus");
    }

    #[test]
    fn a_group_the_form_refuses_asks() {
        let base = taxi();
        let theirs = edit(&base, json!({"category": "Food"}), &[]);
        let mine = edit(&base, json!({"name": "Bus"}), &[]);
        let choice = asked(merge(Kind::Expense, &base, &theirs, &mine, refuse_food));
        assert_eq!(reasons(&choice), vec![(known("category"), Reason::Invalid)]);
        assert_eq!(merged(choice.resolve(|_| Side::Mine, refuse_food)), mine);
    }

    #[test]
    fn unknown_keys_are_kept_taken_from_the_one_side_that_changed_them_and_asked_when_both_did() {
        let base = taxi();
        let theirs = edit(&base, json!({"later": {"k": 2}, "added": [1]}), &[]);
        let mine = edit(&base, json!({"name": "Bus", "mine": true}), &[]);
        let landed = merged(merge(Kind::Expense, &base, &theirs, &mine, accept));
        assert_eq!(landed, edit(&base, json!({"later": {"k": 2}, "added": [1], "name": "Bus", "mine": true}), &[]));

        let removed = edit(&base, json!({}), &["later"]);
        assert!(!merged(merge(Kind::Expense, &base, &removed, &mine, accept)).contains_key("later"));

        let mine = edit(&base, json!({"later": {"k": 3}}), &[]);
        let choice = asked(merge(Kind::Expense, &base, &theirs, &mine, accept));
        assert_eq!(reasons(&choice), vec![(unknown("later"), Reason::BothChanged)]);
        assert_eq!(merged(choice.resolve(|_| Side::Mine, accept))["later"], json!({"k": 3}));

        let removed_by_mine = edit(&base, json!({}), &["later"]);
        let choice = asked(merge(Kind::Expense, &base, &theirs, &removed_by_mine, accept));
        assert_eq!(reasons(&choice), vec![(unknown("later"), Reason::BothChanged)]);
        assert!(choice.conflicts[0].mine.is_empty());
    }

    #[test]
    fn absent_and_null_one_and_one_point_zero_zero_and_minus_zero_all_differ() {
        let base = p(json!({"name": "Ski", "x": 1}));
        let mine = p(json!({"name": "Ski", "x": 2}));
        for theirs in [json!({"name": "Ski", "x": 1.0}), json!({"name": "Ski", "x": null}), json!({"name": "Ski"})] {
            assert_eq!(reasons(&asked(merge(Kind::Project, &base, &p(theirs), &mine, accept))), vec![(unknown("x"), Reason::BothChanged)]);
        }
        let base = p(json!({"name": "Ski", "x": 1}));
        let (zero, minus_zero) = (p(json!({"name": "Ski", "x": 0.0})), p(json!({"name": "Ski", "x": -0.0})));
        assert_eq!(reasons(&asked(merge(Kind::Project, &base, &zero, &minus_zero, accept))), vec![(unknown("x"), Reason::BothChanged)]);
    }

    #[test]
    fn key_order_inside_a_value_is_no_change() {
        let base: Payload = serde_json::from_str(r#"{"name":"Ski","later":{"a":1,"b":[2,3]}}"#).unwrap();
        let theirs: Payload = serde_json::from_str(r#"{"later":{"b":[2,3],"a":1},"name":"Ski"}"#).unwrap();
        let mine = edit(&base, json!({"name": "Ski 2026"}), &[]);
        assert_eq!(merged(merge(Kind::Project, &base, &theirs, &mine, accept)), mine);
    }

    fn rent(rule: &str, next: u32) -> Payload {
        let anchor = if rule == "S0" { "2026-08-01" } else { "2026-12-01" };
        p(json!({
            "template": {"name": "Rent", "amount": 1450.0},
            "rule": {"freq": "monthly", "interval": 1, "anchor": anchor, "ends": {"kind": "never"}},
            "next": next, "paused": false, "seed": A, "eid": B, "slot": "rule"
        }))
    }

    #[test]
    fn next_merges_as_the_larger_value_on_one_schedule() {
        let base = rent("S0", 5);
        assert_eq!(merged(merge(Kind::Rule, &base, &rent("S0", 6), &rent("S0", 8), accept))["next"], 8);
        assert_eq!(merged(merge(Kind::Rule, &base, &rent("S0", 9), &rent("S0", 6), accept))["next"], 9);
        assert_eq!(merged(merge(Kind::Rule, &base, &rent("S0", 4), &base, accept))["next"], 4);
        assert_eq!(merged(merge(Kind::Rule, &base, &rent("S1", 2), &rent("S1", 0), accept)), rent("S1", 2));
    }

    #[test]
    fn next_across_two_schedules_asks_with_the_schedule() {
        let base = rent("S0", 5);
        let choice = asked(merge(Kind::Rule, &base, &rent("S0", 6), &rent("S1", 0), accept));
        assert_eq!(reasons(&choice), vec![(known("schedule"), Reason::BothChanged)]);
        assert_eq!(merged(choice.resolve(|_| Side::Mine, accept)), rent("S1", 0));
        assert_eq!(merged(choice.resolve(|_| Side::Theirs, accept)), rent("S0", 6));
        assert_eq!(merged(merge(Kind::Rule, &base, &rent("S1", 0), &edit(&base, json!({"paused": true}), &[]), accept)), edit(&rent("S1", 0), json!({"paused": true}), &[]));
    }

    #[test]
    fn a_next_that_is_not_an_index_asks() {
        let base = rent("S0", 5);
        for bad in [json!(-1), json!(6.0), json!("6"), json!(null)] {
            let theirs = edit(&base, json!({"next": bad}), &[]);
            let choice = asked(merge(Kind::Rule, &base, &theirs, &rent("S0", 7), accept));
            assert_eq!(reasons(&choice), vec![(known("schedule"), Reason::BothChanged)], "{bad}");
        }
    }

    #[test]
    fn a_participant_is_one_group() {
        let base = p(json!({"name": "Ana", "eid": A, "slot": "participant"}));
        let theirs = edit(&base, json!({"name": "Ana B"}), &[]);
        let mine = edit(&base, json!({"removed": true}), &[]);
        assert_eq!(reasons(&asked(merge(Kind::Participant, &base, &theirs, &mine, accept))), vec![(known("participant"), Reason::BothChanged)]);
    }

    #[test]
    fn a_project_name_travels_with_its_description() {
        let base = p(json!({"name": "Ski", "currency": "EUR", "description": "Tignes"}));
        let theirs = edit(&base, json!({"description": "Val Thorens", "emoji": "\u{2600}"}), &[]);
        let mine = edit(&base, json!({"name": "Ski 2026", "status": "closed"}), &[]);
        let choice = asked(merge(Kind::Project, &base, &theirs, &mine, accept));
        assert_eq!(reasons(&choice), vec![(known("name"), Reason::BothChanged)]);
        assert_eq!(choice.conflicts[0].mine, p(json!({"name": "Ski 2026", "description": "Tignes"})));
        let mine = edit(&base, json!({"status": "closed"}), &[]);
        assert_eq!(merged(merge(Kind::Project, &base, &theirs, &mine, accept)), edit(&theirs, json!({"status": "closed"}), &[]));
    }

    fn refuse_food(p: &Payload) -> Vec<Group> {
        if p.get("category") == Some(&json!("Food")) { vec![known("category")] } else { vec![] }
    }

    #[test]
    fn a_refusal_both_cards_share_is_unfixable_at_once() {
        let base = edit(&taxi(), json!({"category": "Food"}), &[]);
        let theirs = edit(&base, json!({"name": "Train"}), &[]);
        let mine = edit(&base, json!({"date": "2026-11-24"}), &[]);
        assert_eq!(merge(Kind::Expense, &base, &theirs, &mine, refuse_food), Outcome::Unfixable(vec![known("category")]));
    }

    #[test]
    fn a_refused_name_that_is_no_group_of_the_kind_is_unfixable() {
        let base = taxi();
        let mine = edit(&base, json!({"name": "Bus"}), &[]);
        let refuse_amount = |_: &Payload| vec![known("amount")];
        assert_eq!(merge(Kind::Expense, &base, &base, &mine, refuse_amount), Outcome::Unfixable(vec![known("amount")]));
    }

    #[test]
    fn a_choice_no_pick_of_its_groups_can_land_widens_to_the_refused_group() {
        let base = edit(&taxi(), json!({"category": "Transport"}), &[]);
        let theirs = edit(&base, json!({"name": "Train", "category": "Food"}), &[]);
        let mine = edit(&base, json!({"name": "Bus"}), &[]);
        let choice = asked(merge(Kind::Expense, &base, &theirs, &mine, refuse_food));
        assert_eq!(reasons(&choice), vec![(known("name"), Reason::BothChanged), (known("category"), Reason::Invalid)]);
        let landed = merged(choice.resolve(|g| if *g == known("category") { Side::Mine } else { Side::Theirs }, refuse_food));
        assert_eq!(landed, edit(&base, json!({"name": "Train"}), &[]));
        assert_eq!(reasons(&asked(choice.resolve(|_| Side::Theirs, refuse_food))), reasons(&choice));
    }

    #[test]
    fn a_choice_whose_pick_is_refused_asks_again_with_the_refused_group_added() {
        let base = edit(&taxi(), json!({"category": "Transport"}), &[]);
        let theirs = edit(&base, json!({"name": "Train"}), &[]);
        let mine = edit(&base, json!({"name": "Bus", "category": "Food"}), &[]);
        let refuse_bus_food = |p: &Payload| if p.get("name") == Some(&json!("Bus")) { refuse_food(p) } else { vec![] };
        let choice = asked(merge(Kind::Expense, &base, &theirs, &mine, refuse_bus_food));
        assert_eq!(reasons(&choice), vec![(known("name"), Reason::BothChanged)]);
        let again = asked(choice.resolve(|_| Side::Mine, refuse_bus_food));
        assert_eq!(reasons(&again), vec![(known("name"), Reason::BothChanged), (known("category"), Reason::Invalid)]);
        let landed = merged(again.resolve(|g| if *g == known("name") { Side::Mine } else { Side::Theirs }, refuse_bus_food));
        assert_eq!(landed, edit(&base, json!({"name": "Bus"}), &[]));
    }

    fn with_shares(shares: Value, amount: f64) -> Payload {
        edit(&taxi(), json!({"amount": amount, "shares": shares}), &[])
    }

    fn money_refused(theirs: &Payload) -> bool {
        let base = taxi();
        let mine = edit(&base, json!({"name": "Bus"}), &[]);
        match merge(Kind::Expense, &base, theirs, &mine, accept) {
            Outcome::Choose(c) => reasons(&c) == vec![(known("money"), Reason::Invalid)],
            Outcome::Merged { .. } => false,
            other => panic!("{other:?}"),
        }
    }

    #[test]
    fn debtors_off_by_a_cent_are_refused() {
        let shares = json!([{"userId": 1, "amount": 30.0, "isDebt": false}, {"userId": 1, "amount": 15.0, "isDebt": true}, {"userId": 2, "amount": 14.99, "isDebt": true}]);
        assert!(money_refused(&with_shares(shares, 30.0)));
        let payers = json!([{"userId": 1, "amount": 29.99, "isDebt": false}, {"userId": 2, "amount": 30.0, "isDebt": true}]);
        assert!(money_refused(&with_shares(payers, 30.0)));
    }

    #[test]
    fn an_oversized_share_is_refused_even_when_the_sums_hold() {
        let shares = json!([{"userId": 1, "amount": 1e13, "isDebt": false}, {"userId": 2, "amount": -9_999_999_999_970.0, "isDebt": false},
                            {"userId": 2, "amount": 30.0, "isDebt": true}]);
        assert!(money_refused(&with_shares(shares, 30.0)));
    }

    #[test]
    fn a_share_with_no_bool_is_debt_is_refused_even_when_the_sums_hold() {
        for odd in [json!({"userId": 3, "amount": 0.0}), json!({"userId": 3, "amount": 0.0, "isDebt": "true"}), json!({"userId": 3, "amount": 0.0, "isDebt": 1}), json!({"userId": 3, "amount": 0.0, "isDebt": null})] {
            let shares = json!([{"userId": 1, "amount": 30.0, "isDebt": false}, {"userId": 2, "amount": 30.0, "isDebt": true}, odd]);
            assert!(money_refused(&with_shares(shares, 30.0)), "{odd}");
        }
    }

    #[test]
    fn no_shares_on_a_zero_amount_passes_the_merge_and_is_the_form_check_to_refuse() {
        let base = taxi();
        let theirs = with_shares(json!([]), 0.0);
        let mine = edit(&base, json!({"name": "Bus"}), &[]);
        assert_eq!(merged(merge(Kind::Expense, &base, &theirs, &mine, accept)), edit(&theirs, json!({"name": "Bus"}), &[]));
    }

    #[test]
    fn a_theirs_or_a_mine_of_another_entity_is_not_merged_onto() {
        let base = taxi();
        let mine = edit(&base, json!({"name": "Bus"}), &[]);
        let other = edit(&base, json!({"eid": B}), &[]);
        assert_eq!(merge(Kind::Expense, &base, &other, &mine, accept), Outcome::OtherRow);
        assert_eq!(merge(Kind::Expense, &base, &base, &edit(&mine, json!({"eid": B}), &[]), accept), Outcome::OtherRow);
        let other_slot = edit(&base, json!({"slot": "rule"}), &[]);
        assert_eq!(merge(Kind::Expense, &base, &other_slot, &mine, accept), Outcome::OtherRow);
        let eid_as_number = edit(&base, json!({"eid": 1}), &[]);
        assert_eq!(merge(Kind::Expense, &base, &eid_as_number, &mine, accept), Outcome::OtherRow);
    }

    #[test]
    fn a_participant_s_unknown_keys_are_inside_its_one_group() {
        let base = p(json!({"name": "Ana", "later": 1, "eid": A, "slot": "participant"}));
        let theirs = edit(&base, json!({"later": 2}), &[]);
        let mine = edit(&base, json!({"name": "Ana B"}), &[]);
        let choice = asked(merge(Kind::Participant, &base, &theirs, &mine, accept));
        assert_eq!(reasons(&choice), vec![(known("participant"), Reason::BothChanged)]);
        assert_eq!(choice.conflicts[0].theirs, p(json!({"name": "Ana", "later": 2})));
        assert_eq!(choice.conflicts[0].mine, p(json!({"name": "Ana B", "later": 1})));
        assert_eq!(merged(choice.resolve(|_| Side::Mine, accept)), mine);

        assert_eq!(merged(merge(Kind::Participant, &base, &theirs, &base, accept)), theirs);
        let renamed_elsewhere = edit(&base, json!({"name": "Ana C"}), &[]);
        let unbound = edit(&base, json!({}), &["eid", "slot"]);
        assert_eq!(merged(merge(Kind::Participant, &base, &renamed_elsewhere, &unbound, accept)), edit(&renamed_elsewhere, json!({}), &["eid", "slot"]));
    }

    #[test]
    fn an_eid_the_conversion_adds_is_kept_and_both_keys_gone_merge_by_the_rule() {
        let base = edit(&taxi(), json!({}), &["eid", "slot"]);
        let converted = edit(&base, json!({"eid": A, "slot": "expense"}), &[]);
        let mine = edit(&base, json!({"name": "Bus"}), &[]);
        assert_eq!(merged(merge(Kind::Expense, &base, &converted, &mine, accept)), edit(&converted, json!({"name": "Bus"}), &[]));

        let base = taxi();
        let dropped = edit(&base, json!({}), &["eid", "slot"]);
        let mine = edit(&base, json!({"name": "Bus"}), &[]);
        assert_eq!(merged(merge(Kind::Expense, &base, &dropped, &mine, accept)), edit(&dropped, json!({"name": "Bus"}), &[]));
    }

    #[test]
    fn a_payload_with_one_entity_key_and_not_the_other_is_another_row() {
        let base = taxi();
        let mine = edit(&base, json!({"name": "Bus"}), &[]);
        for (key, kept) in [("eid", "slot"), ("slot", "eid")] {
            let one = |p: &Payload| edit(p, json!({}), &[key]);
            assert_eq!(merge(Kind::Expense, &base, &one(&base), &mine, accept), Outcome::OtherRow, "theirs keeps only {kept}");
            assert_eq!(merge(Kind::Expense, &base, &base, &one(&mine), accept), Outcome::OtherRow, "mine keeps only {kept}");
            assert_eq!(merge(Kind::Expense, &one(&base), &base, &mine, accept), Outcome::OtherRow, "base keeps only {kept}");
            let unbound = edit(&base, json!({}), &["eid", "slot"]);
            let half = edit(&unbound, json!({kept: base[kept].clone()}), &[]);
            assert_eq!(merge(Kind::Expense, &unbound, &half, &edit(&unbound, json!({"name": "Bus"}), &[]), accept), Outcome::OtherRow, "theirs adds only {kept}");
        }
    }

    #[test]
    fn a_refused_key_asks_through_the_group_that_owns_it() {
        let base = p(json!({"name": "Ana", "later": 1}));
        let theirs = edit(&base, json!({"later": 2}), &[]);
        let mine = edit(&base, json!({"name": "Ana B"}), &[]);
        let refuse_later = |p: &Payload| if p.get("later") == Some(&json!(2)) { vec![unknown("later")] } else { vec![] };
        let choice = asked(merge(Kind::Participant, &base, &theirs, &edit(&base, json!({}), &[]), refuse_later));
        assert_eq!(reasons(&choice), vec![(known("participant"), Reason::Invalid)]);
        assert_eq!(merged(choice.resolve(|_| Side::Mine, refuse_later)), base);
        let choice = asked(merge(Kind::Participant, &base, &theirs, &mine, refuse_later));
        assert_eq!(reasons(&choice), vec![(known("participant"), Reason::BothChanged)]);
        assert_eq!(merged(choice.resolve(|_| Side::Mine, refuse_later)), mine);

        let mine = edit(&taxi(), json!({"name": "Bus"}), &[]);
        let refuse_shares = |p: &Payload| if p.get("amount") == Some(&json!(42.0)) { vec![unknown("shares")] } else { vec![] };
        let theirs = edit(&taxi(), priced(42.0), &[]);
        let choice = asked(merge(Kind::Expense, &taxi(), &theirs, &mine, refuse_shares));
        assert_eq!(reasons(&choice), vec![(known("money"), Reason::Invalid)]);
        assert_eq!(merged(choice.resolve(|_| Side::Mine, refuse_shares)), mine);
    }

    #[test]
    fn widening_keeps_a_both_changed_group_both_changed() {
        let base = taxi();
        let theirs = edit(&base, json!({"name": "Train", "category": "Food"}), &[]);
        let mine = edit(&base, json!({"name": "Bus"}), &[]);
        let refuse_food = |p: &Payload| if p.get("category") == Some(&json!("Food")) { vec![known("name"), known("category")] } else { vec![] };
        let choice = asked(merge(Kind::Expense, &base, &theirs, &mine, refuse_food));
        assert_eq!(reasons(&choice), vec![(known("name"), Reason::BothChanged), (known("category"), Reason::Invalid)]);
        let again = asked(choice.resolve(|_| Side::Theirs, refuse_food));
        assert_eq!(reasons(&again), reasons(&choice));
    }

    fn counting<'a>(calls: &'a std::cell::Cell<usize>, refuse: impl Fn(usize, &Payload) -> Vec<Group> + 'a) -> impl Fn(&Payload) -> Vec<Group> + 'a {
        move |p: &Payload| {
            calls.set(calls.get() + 1);
            assert!(calls.get() <= MAX_CHECKS, "check called more than {MAX_CHECKS} times");
            refuse(calls.get(), p)
        }
    }

    #[test]
    fn a_check_that_names_a_new_group_on_every_call_stops_within_the_budget() {
        let calls = std::cell::Cell::new(0);
        let base = taxi();
        let mine = edit(&base, json!({"name": "Bus"}), &[]);
        let fresh_name = |n: usize, _: &Payload| vec![unknown(&format!("g{n}"))];
        assert!(matches!(merge(Kind::Expense, &base, &base, &mine, counting(&calls, fresh_name)), Outcome::Unfixable(_)));
        assert_eq!(calls.get(), 2, "the draft, then the one pick there is");

        let theirs = edit(&base, json!({"name": "Train"}), &[]);
        let choice = asked(merge(Kind::Expense, &base, &theirs, &mine, accept));
        calls.set(0);
        assert!(matches!(choice.resolve(|_| Side::Mine, counting(&calls, fresh_name)), Outcome::Unfixable(_)));
        assert_eq!(calls.get(), 3, "the pick, then both picks of the name");
    }

    #[test]
    fn a_chain_of_refusals_over_twelve_keys_stays_within_the_budget() {
        let keys: Vec<String> = (0..12).map(|i| format!("u{i:02}")).collect();
        let with = |value: &str| p(Value::Object(keys.iter().map(|k| (k.clone(), json!(value))).chain([("name".to_string(), json!("Ski"))]).collect()));
        let (base, theirs) = (with("b"), with("t"));
        let mine = edit(&base, json!({"name": "Ski 2026"}), &[]);
        let first_left = |_: usize, p: &Payload| keys.iter().find(|k| p[k.as_str()] == "t").map(|k| vec![unknown(k)]).unwrap_or_default();
        let calls = std::cell::Cell::new(0);
        let outcome = merge(Kind::Project, &base, &theirs, &mine, counting(&calls, first_left));
        assert!(calls.get() <= MAX_CHECKS);
        assert!(!matches!(outcome, Outcome::Merged { .. } | Outcome::OtherRow), "{outcome:?}");
    }

    #[test]
    fn more_groups_to_choose_in_than_the_cap_is_unfixable() {
        let both_changed = |n: usize| {
            let side = |value: &str| p(Value::Object((0..n).map(|i| (format!("u{i:02}"), json!(value))).chain([("name".to_string(), json!("Ski"))]).collect()));
            (side("b"), side("t"), side("m"))
        };
        let (base, theirs, mine) = both_changed(12);
        assert_eq!(asked(merge(Kind::Project, &base, &theirs, &mine, accept)).conflicts.len(), 12);
        let (base, theirs, mine) = both_changed(13);
        assert!(matches!(merge(Kind::Project, &base, &theirs, &mine, accept), Outcome::Unfixable(groups) if groups.len() == 13));
    }

    struct Pool {
        kind: Kind,
        groups: Vec<(Group, Vec<Payload>)>,
    }

    fn values(name: &'static str, fragments: Vec<Value>) -> (Group, Vec<Payload>) {
        (Group::Known(name), fragments.into_iter().map(p).collect())
    }

    fn entity_values(slot: &str) -> (Group, Vec<Payload>) {
        values("entity", vec![json!({}), json!({"eid": A, "slot": slot}), json!({"eid": B, "slot": slot})])
    }

    fn unknown_values() -> Vec<(Group, Vec<Payload>)> {
        vec![
            (unknown("later"), vec![p(json!({})), p(json!({"later": 1})), p(json!({"later": {"a": [1, 2]}}))]),
            (unknown("zz"), vec![p(json!({})), p(json!({"zz": "x"})), p(json!({"zz": null}))]),
        ]
    }

    fn sound_money() -> Vec<Value> {
        vec![
            json!({"amount": 30.0, "expenseType": "expense", "shares": [{"userId": 1, "amount": 30.0, "isDebt": false},
                {"userId": 1, "amount": 15.0, "isDebt": true}, {"userId": 2, "amount": 15.0, "isDebt": true}]}),
            json!({"amount": 31.0, "expenseType": "expense", "estimate": true, "shares": [{"userId": 2, "amount": 31.0, "isDebt": false},
                {"userId": 1, "amount": 31.0, "isDebt": true, "later": 1}]}),
            json!({"amount": 10.0, "expenseType": "transfer", "sourceCurrency": "USD", "sourceAmount": 10.42, "rate": 0.96,
                "shares": [{"userId": 1, "amount": 10.0, "isDebt": false}, {"userId": 2, "amount": 10.0, "isDebt": true}]}),
            json!({"amount": 30.0, "expenseType": "expense"}),
        ]
    }

    fn broken_money() -> Vec<Value> {
        vec![
            json!({"amount": 30.0, "expenseType": "expense", "shares": [{"userId": 1, "amount": 29.99, "isDebt": false},
                {"userId": 2, "amount": 30.0, "isDebt": true}]}),
            json!({"amount": 1e13, "expenseType": "expense"}),
            json!({"amount": 30.0, "shares": [{"userId": 1, "amount": 30.0}]}),
            json!({"expenseType": "expense", "shares": []}),
        ]
    }

    fn expense_pool(money: Vec<Value>) -> Pool {
        let mut groups = vec![
            values("money", money),
            values("name", vec![json!({"name": "Taxi"}), json!({"name": "Train"}), json!({"name": "Bus"})]),
            values("date", vec![json!({"date": "2026-11-01"}), json!({"date": "2026-11-02"}), json!({})]),
            values("description", vec![json!({}), json!({"description": "a"}), json!({"description": "b"})]),
            values("category", vec![json!({}), json!({"category": "Transport"}), json!({"category": "Food"})]),
            values("author", vec![json!({}), json!({"authorId": 1}), json!({"authorId": 2})]),
            values("recurring", vec![json!({}), json!({"recurringId": A}), json!({"recurringId": B})]),
            entity_values("expense"),
        ];
        groups.extend(unknown_values());
        Pool { kind: Kind::Expense, groups }
    }

    fn rule_pool() -> Pool {
        let s0 = json!({"freq": "monthly", "interval": 1, "anchor": "2026-08-01", "ends": {"kind": "never"}});
        let s1 = json!({"freq": "weekly", "interval": 2, "anchor": "2026-12-01", "ends": {"kind": "after", "value": 4}});
        let template = |name: &str, amount: f64| {
            json!({"template": {"name": name, "expenseType": "expense", "amount": amount,
                "payers": {"exact": true, "entries": [[1, amount]]}, "debtors": {"exact": false, "entries": [[1, 1.0], [2, 1.0]]}}})
        };
        let mut groups = vec![
            values("template", vec![template("Rent", 1450.0), template("Rent 2", 1450.0), template("Rent", 1500.0)]),
            values(
                "schedule",
                vec![
                    json!({"rule": s0, "next": 5}),
                    json!({"rule": s0, "next": 6}),
                    json!({"rule": s0, "next": 9}),
                    json!({"rule": s1, "next": 0}),
                    json!({"rule": s1, "next": 2}),
                    json!({"rule": s1, "next": 5}),
                ],
            ),
            values("paused", vec![json!({}), json!({"paused": true}), json!({"paused": false})]),
            values("seed", vec![json!({"seed": A}), json!({"seed": B})]),
            values("author", vec![json!({}), json!({"authorId": 1}), json!({"authorId": 2})]),
            entity_values("rule"),
        ];
        groups.extend(unknown_values());
        Pool { kind: Kind::Rule, groups }
    }

    fn participant_pool() -> Pool {
        let groups = vec![
            values(
                "participant",
                vec![json!({"name": "Ana"}), json!({"name": "Ana B", "later": 1}), json!({"name": "Ana", "removed": true, "later": {"a": [1]}, "zz": null})],
            ),
            entity_values("participant"),
        ];
        Pool { kind: Kind::Participant, groups }
    }

    fn project_pool() -> Pool {
        let mut groups = vec![
            values("name", vec![json!({"name": "Ski"}), json!({"name": "Ski", "description": "Tignes"}), json!({"name": "Ski 2026"})]),
            values("status", vec![json!({}), json!({"status": "closed"}), json!({"status": "archived"})]),
            values("emoji", vec![json!({}), json!({"emoji": "\u{2600}"}), json!({"emoji": "\u{26F7}"})]),
            values("currency", vec![json!({"currency": "EUR"}), json!({"currency": "CHF"})]),
            values("kind", vec![json!({}), json!({"kind": "personal"})]),
            values(
                "budgets",
                vec![json!({}), json!({"budgets": {"monthly": {"Transport": 10.0}, "alert": false}}), json!({"budgets": {"monthly": {"Transport": 12.0}, "alert": true}})],
            ),
            values("budgetsCustom", vec![json!({}), json!({"budgetsCustom": {"c-1": 5.0}})]),
            values("categories", vec![json!({}), json!({"categories": [{"id": "c-1", "name": "Ski pass"}]}), json!({"categories": []})]),
        ];
        groups.extend(unknown_values());
        Pool { kind: Kind::Project, groups }
    }

    fn pool(kind: usize) -> Pool {
        match kind % 4 {
            0 => expense_pool(sound_money()),
            1 => rule_pool(),
            2 => participant_pool(),
            _ => project_pool(),
        }
    }

    #[test]
    fn the_pools_cover_every_group_in_order_with_distinct_values() {
        for kind in 0..4 {
            let pool = pool(kind);
            let mut expected: Vec<Group> = known_groups(pool.kind).collect();
            if pool.kind != Kind::Participant {
                expected.extend([unknown("later"), unknown("zz")]);
            }
            assert_eq!(pool.groups.iter().map(|(g, _)| g.clone()).collect::<Vec<_>>(), expected);
            for (group, fragments) in &pool.groups {
                let distinct: BTreeSet<Vec<u8>> = fragments.iter().map(|f| serde_json::to_vec(f).unwrap()).collect();
                assert_eq!(distinct.len(), fragments.len(), "{group:?}");
                let keys = keys_of(pool.kind, group, &fragments.iter().collect::<Vec<_>>());
                assert!(fragments.iter().all(|f| f.keys().all(|k| keys.contains(k))), "{group:?}");
            }
        }
    }

    type Picks = Vec<(u8, u8, u8)>;

    fn index(pool: &Pool, picks: &Picks, i: usize) -> (usize, usize, usize) {
        let n = pool.groups[i].1.len();
        let (b, t, m) = picks[i];
        (b as usize % n, t as usize % n, m as usize % n)
    }

    fn build(pool: &Pool, pick: impl Fn(usize) -> usize) -> Payload {
        let mut out = Payload::new();
        for (i, (_, fragments)) in pool.groups.iter().enumerate() {
            out.extend(fragments[pick(i)].clone());
        }
        out
    }

    /// The rule of plan §3.1 on pool indices, written apart from the module: `None` asks.
    fn expected(pool: &Pool, i: usize, (b, t, m): (usize, usize, usize)) -> Option<usize> {
        if t == b {
            return Some(m);
        }
        if m == b || t == m {
            return Some(t);
        }
        let (group, fragments) = &pool.groups[i];
        if pool.kind == Kind::Rule && *group == known("schedule") && fragments[t]["rule"] == fragments[m]["rule"] {
            let next = |j: usize| fragments[j]["next"].as_u64().unwrap();
            return Some(if next(m) > next(t) { m } else { t });
        }
        None
    }

    fn names_another_row(a: &Payload, b: &Payload) -> bool {
        let half = |p: &Payload| p.contains_key("eid") ^ p.contains_key("slot");
        half(a)
            || half(b)
            || ["eid", "slot"].iter().any(|k| matches!((a.get(*k), b.get(*k)), (Some(x), Some(y)) if serde_json::to_vec(x).unwrap() != serde_json::to_vec(y).unwrap()))
    }

    fn rendered(keys: &[String], payload: &Payload) -> Vec<Option<Vec<u8>>> {
        keys.iter().map(|k| payload.get(k).map(|v| serde_json::to_vec(v).unwrap())).collect()
    }

    fn shares_add_up(payload: &Payload) -> bool {
        let cents = |v: &Value| v.as_f64().filter(|a| a.is_finite() && a.abs() <= 1e12).map(|a| (a * 100.0).round() as i64);
        let Some(total) = payload.get("amount").and_then(cents) else { return false };
        let shares = match payload.get("shares") {
            None | Some(Value::Null) => return true,
            Some(Value::Array(shares)) => shares,
            Some(_) => return false,
        };
        [false, true].into_iter().all(|is_debt| {
            let side: Option<Vec<i64>> =
                shares.iter().filter(|s| s.get("isDebt") == Some(&Value::Bool(is_debt))).map(|s| s.get("amount").and_then(cents)).collect();
            side.is_some_and(|c| c.iter().sum::<i64>() == total)
        }) && shares.iter().all(|s| s.get("isDebt").is_some_and(Value::is_boolean))
    }

    /// The invariants of plan §3.1 on any landed result. `decided` are the groups the user picked.
    fn holds(kind: Kind, base: &Payload, theirs: &Payload, mine: &Payload, result: &Payload, decided: &[Group]) -> Result<(), TestCaseError> {
        prop_assert!(!names_another_row(base, theirs) && !names_another_row(base, mine), "merged onto another row");
        let all = [base, theirs, mine, result];
        for group in groups_of(kind, &all) {
            let keys = keys_of(kind, &group, &all);
            let (b, t, m, r) = (rendered(&keys, base), rendered(&keys, theirs), rendered(&keys, mine), rendered(&keys, result));
            prop_assert!(r == t || r == m, "{:?} is neither side's", group);
            if decided.contains(&group) {
                continue;
            }
            if kind == Kind::Rule && group == known("schedule") && t != b && m != b && t != m {
                prop_assert_eq!(theirs.get("rule"), mine.get("rule"), "the larger next across two schedules");
                let next = |p: &Payload| p.get("next").and_then(Value::as_u64);
                prop_assert_eq!(next(result), next(theirs).max(next(mine)));
                continue;
            }
            if r != t {
                prop_assert_eq!(&t, &b, "{:?}: theirs overwritten by a writer who never saw it", group);
            }
            if r != m {
                prop_assert_eq!(&m, &b, "{:?}: mine dropped", group);
            }
        }
        if kind == Kind::Expense {
            prop_assert!(shares_add_up(result), "{:?}", result);
        }
        Ok(())
    }

    fn by_mask(choice: &Choice, mask: u32) -> impl Fn(&Group) -> Side + '_ {
        move |g: &Group| {
            let i = choice.conflicts.iter().position(|c| &c.group == g).unwrap_or(0);
            if mask >> i & 1 == 1 { Side::Mine } else { Side::Theirs }
        }
    }

    /// Every card is the side's own, the two differ, some pick lands, every pick that lands keeps the
    /// invariants, and a pick that does not asks again over at least the same groups.
    fn choice_holds(kind: Kind, base: &Payload, theirs: &Payload, mine: &Payload, choice: &Choice, check: impl Fn(&Payload) -> Vec<Group> + Copy) -> Result<(), TestCaseError> {
        prop_assert!(!choice.conflicts.is_empty() && choice.conflicts.len() <= MAX_OPEN);
        for c in &choice.conflicts {
            let keys = keys_of(kind, &c.group, &[base, theirs, mine]);
            prop_assert_eq!(&c.theirs, &part(&keys, theirs));
            prop_assert_eq!(&c.mine, &part(&keys, mine));
            prop_assert_ne!(&c.theirs, &c.mine);
        }
        let decided: Vec<Group> = choice.conflicts.iter().map(|c| c.group.clone()).collect();
        let mut lands = 0;
        for mask in 0u32..1 << choice.conflicts.len() {
            match choice.resolve(by_mask(choice, mask), check) {
                Outcome::Merged { payload, .. } => {
                    lands += 1;
                    prop_assert!(check(&payload).is_empty());
                    holds(kind, base, theirs, mine, &payload, &decided)?;
                }
                Outcome::Choose(again) => {
                    let wider: Vec<Group> = again.conflicts.iter().map(|c| c.group.clone()).collect();
                    prop_assert!(decided.iter().all(|g| wider.contains(g)), "{:?} narrowed to {:?}", decided, wider);
                }
                other => prop_assert!(false, "a choice led to {:?}", other),
            }
        }
        prop_assert!(lands > 0, "no pick of {:?} lands", decided);
        Ok(())
    }

    /// A picker that runs through every pick of the groups shown lands within 2^n rounds of the
    /// groups last changing, and the groups only grow, so it lands; it never meets `Unfixable`.
    fn rounds_are_bounded(mut outcome: Outcome, check: impl Fn(&Payload) -> Vec<Group> + Copy) -> Result<(), TestCaseError> {
        let (mut shown, mut since, mut round) = (Vec::new(), 0u32, 0u32);
        loop {
            let Outcome::Choose(choice) = &outcome else { return Ok(()) };
            let groups: Vec<Group> = choice.conflicts.iter().map(|c| c.group.clone()).collect();
            if groups != shown {
                prop_assert!(shown.iter().all(|g| groups.contains(g)), "{:?} narrowed to {:?}", shown, groups);
                (shown, since) = (groups, 0);
            }
            prop_assert!(since < 1 << shown.len(), "{} rounds over {:?}", since, shown);
            outcome = choice.resolve(by_mask(choice, round), check);
            prop_assert!(!matches!(outcome, Outcome::Unfixable(_) | Outcome::OtherRow), "{:?}", outcome);
            since += 1;
            round = round.wrapping_add(1);
        }
    }

    fn check_against_the_rule(pool: &Pool, picks: &Picks) -> Result<(), TestCaseError> {
        let n = pool.groups.len();
        let at = |i: usize| index(pool, picks, i);
        let (base, theirs, mine) = (build(pool, |i| at(i).0), build(pool, |i| at(i).1), build(pool, |i| at(i).2));
        let outcome = merge(pool.kind, &base, &theirs, &mine, accept);
        if names_another_row(&base, &theirs) || names_another_row(&base, &mine) {
            prop_assert_eq!(outcome, Outcome::OtherRow);
            return Ok(());
        }
        let asks: Vec<Group> = (0..n).filter(|&i| expected(pool, i, at(i)).is_none()).map(|i| pool.groups[i].0.clone()).collect();
        let chosen = |side: Side| move |i: usize| expected(pool, i, at(i)).unwrap_or(if side == Side::Theirs { at(i).1 } else { at(i).2 });
        let is_sound = |payload: &Payload| pool.kind != Kind::Expense || shares_add_up(payload);
        let some_money_lands = pool.kind != Kind::Expense || {
            let (_, t, m) = at(0);
            [t, m].into_iter().any(|j| is_sound(&build(pool, |i| if i == 0 { j } else { chosen(Side::Theirs)(i) })))
        };
        match outcome {
            Outcome::Merged { payload, changed } => {
                prop_assert!(asks.is_empty(), "merged where {:?} should ask", asks);
                prop_assert_eq!(&payload, &build(pool, chosen(Side::Theirs)));
                let expected_changed: Vec<Group> = (0..n).filter(|&i| chosen(Side::Theirs)(i) != at(i).1).map(|i| pool.groups[i].0.clone()).collect();
                prop_assert_eq!(changed, expected_changed);
                holds(pool.kind, &base, &theirs, &mine, &payload, &[])?;
            }
            Outcome::Choose(choice) => {
                prop_assert!(some_money_lands);
                prop_assert!(asks.iter().all(|g| choice.conflicts.iter().any(|c| &c.group == g && c.reason == Reason::BothChanged)), "{:?}", asks);
                for c in &choice.conflicts {
                    prop_assert!(asks.contains(&c.group) || (c.group == known("money") && c.reason == Reason::Invalid), "{:?}", c);
                }
                for side in [Side::Theirs, Side::Mine] {
                    let want = build(pool, chosen(side));
                    if is_sound(&want) && choice.conflicts.iter().all(|c| asks.contains(&c.group)) {
                        prop_assert_eq!(merged(choice.resolve(|_| side, accept)), want);
                    }
                }
                choice_holds(pool.kind, &base, &theirs, &mine, &choice, accept)?;
            }
            Outcome::Unfixable(groups) => {
                prop_assert!(!some_money_lands);
                prop_assert!(groups.contains(&known("money")));
            }
            Outcome::OtherRow => prop_assert!(false, "not another row"),
        }
        Ok(())
    }

    fn picks() -> impl Strategy<Value = Picks> {
        prop::collection::vec(any::<(u8, u8, u8)>(), 16)
    }

    fn leaf() -> impl Strategy<Value = Value> {
        prop_oneof![
            Just(Value::Null),
            any::<bool>().prop_map(Value::Bool),
            any::<i64>().prop_map(Value::from),
            any::<u64>().prop_map(Value::from),
            any::<f64>().prop_map(Value::from),
            prop::sample::select(vec![0.0, -0.0, 0.004, 0.005, 1e12, 1e13, 12.5, 15.0, 30.0]).prop_map(Value::from),
            prop::sample::select(vec!["", "x", "expense", A, B, "rule", "Food"]).prop_map(Value::from),
        ]
    }

    fn hostile_value() -> impl Strategy<Value = Value> {
        leaf().prop_recursive(4, 32, 4, |inner| {
            let key = prop::sample::select(vec!["amount", "isDebt", "userId", "x", "next"]).prop_map(String::from);
            prop_oneof![
                prop::collection::vec(inner.clone(), 0..4).prop_map(Value::Array),
                prop::collection::btree_map(key, inner, 0..4).prop_map(|m| Value::Object(m.into_iter().collect())),
            ]
        })
    }

    fn hostile_payload(kind: Kind) -> impl Strategy<Value = Payload> {
        let mut keys: Vec<String> = table_keys(kind).into_iter().collect();
        keys.extend(["later".to_string(), "zz".to_string()]);
        prop::collection::btree_map(prop::sample::select(keys), hostile_value(), 0..12).prop_map(|m| m.into_iter().collect())
    }

    fn mix(base: &Payload, other: &Payload, mask: u64) -> Payload {
        let keys: BTreeSet<&String> = base.keys().chain(other.keys()).collect();
        let mut out = Payload::new();
        for (bit, key) in keys.into_iter().enumerate() {
            let from = if mask >> (bit % 64) & 1 == 1 { other } else { base };
            if let Some(v) = from.get(key) {
                out.insert(key.clone(), v.clone());
            }
        }
        out
    }

    fn hostile_case() -> impl Strategy<Value = (Kind, Payload, Payload, Payload)> {
        prop::sample::select(KINDS.to_vec()).prop_flat_map(|kind| {
            (Just(kind), hostile_payload(kind), hostile_payload(kind), hostile_payload(kind), any::<u64>(), any::<u64>()).prop_map(
                |(kind, base, t, m, tm, mm)| {
                    let (theirs, mine) = (mix(&base, &t, tm), mix(&base, &m, mm));
                    (kind, base, theirs, mine)
                },
            )
        })
    }

    fn refuse_when_zz(p: &Payload) -> Vec<Group> {
        if p.contains_key("zz") { vec![unknown("later"), known("name")] } else { vec![] }
    }

    fn refuse_a_key_name(_: &Payload) -> Vec<Group> {
        vec![known("amount")]
    }

    fn refuse_by_hash(p: &Payload) -> Vec<Group> {
        let bytes = serde_json::to_vec(p).unwrap();
        let hash = bytes.iter().fold(7u32, |h, b| h.wrapping_mul(31).wrapping_add(u32::from(*b)));
        match hash % 4 {
            0 => vec![known("name"), unknown("later")],
            1 => vec![known("participant")],
            _ => vec![],
        }
    }

    const CHECKS: [fn(&Payload) -> Vec<Group>; 5] = [accept, refuse_when_zz, refuse_a_key_name, refuse_by_hash, refuse_food];

    /// Structurally sound shares of `cents` split by `weights` on each side.
    fn split(cents: i64, weights: &[u8], is_debt: bool, first_user: i32) -> Vec<Value> {
        let total: i64 = weights.iter().map(|w| i64::from(*w)).sum();
        let mut left = cents;
        weights
            .iter()
            .enumerate()
            .map(|(i, w)| {
                let part = if i + 1 == weights.len() { left } else { cents * i64::from(*w) / total };
                left -= part;
                json!({"userId": first_user + i as i32, "amount": part as f64 / 100.0, "isDebt": is_debt})
            })
            .collect()
    }

    fn perturbed(mut shares: Vec<Value>, which: usize, how: u8) -> Vec<Value> {
        let k = which % shares.len();
        let share = shares[k].as_object_mut().unwrap();
        match how % 8 {
            0 => {
                let a = share["amount"].as_f64().unwrap();
                share.insert("amount".into(), json!(((a * 100.0).round() + 1.0) / 100.0));
            }
            1 => {
                share.remove("isDebt");
            }
            2 => {
                let d = share["isDebt"].as_bool().unwrap();
                share.insert("isDebt".into(), json!(d.to_string()));
            }
            3 => {
                let d = share["isDebt"].as_bool().unwrap();
                share.insert("isDebt".into(), json!(!d));
            }
            4 => {
                let a = share["amount"].clone();
                share.insert("amount".into(), json!(a.to_string()));
            }
            5 => {
                let (a, d) = (share["amount"].as_f64().unwrap(), share["isDebt"].clone());
                share.insert("amount".into(), json!(1e13));
                shares.push(json!({"userId": 99, "amount": a - 1e13, "isDebt": d}));
            }
            6 => shares.push(json!({"userId": 99, "amount": 0.0})),
            _ => shares.push(json!({"userId": 99, "amount": 0.0, "isDebt": 0})),
        }
        shares
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(4000))]

        #[test]
        fn every_merge_follows_the_rule_group_by_group(kind in 0..4usize, picks in picks()) {
            check_against_the_rule(&pool(kind), &picks)?;
        }

        #[test]
        fn shares_add_up_in_every_landed_expense_or_it_asks(picks in picks()) {
            let mut money = sound_money();
            money.extend(broken_money());
            check_against_the_rule(&expense_pool(money), &picks)?;
        }

        #[test]
        fn a_side_that_changed_nothing_gives_the_other_side_whole(kind in 0..4usize, picks in picks()) {
            let pool = pool(kind);
            let base = build(&pool, |i| index(&pool, &picks, i).0);
            let x = build(&pool, |i| index(&pool, &picks, i).1);
            if names_another_row(&base, &x) {
                for outcome in [merge(pool.kind, &base, &x, &base, accept), merge(pool.kind, &base, &base, &x, accept), merge(pool.kind, &base, &x, &x, accept)] {
                    prop_assert_eq!(outcome, Outcome::OtherRow);
                }
                return Ok(());
            }
            let Outcome::Merged { payload, changed } = merge(pool.kind, &base, &x, &base, accept) else { panic!("asked") };
            prop_assert_eq!(payload, x.clone());
            prop_assert!(changed.is_empty());
            prop_assert_eq!(merged(merge(pool.kind, &base, &base, &x, accept)), x.clone());
            prop_assert_eq!(merged(merge(pool.kind, &base, &x, &x, accept)), x);
        }

        #[test]
        fn disjoint_edits_commute(kind in 0..4usize, picks in picks()) {
            let pool = pool(kind);
            let who = |i: usize| picks[i].2 % 3;
            let (base_at, edit_at) = (|i: usize| index(&pool, &picks, i).0, |i: usize| index(&pool, &picks, i).1);
            let base = build(&pool, base_at);
            let theirs = build(&pool, |i| if who(i) == 0 { edit_at(i) } else { base_at(i) });
            let mine = build(&pool, |i| if who(i) == 1 { edit_at(i) } else { base_at(i) });
            prop_assume!(!names_another_row(&base, &theirs) && !names_another_row(&base, &mine));
            let one_way = merged(merge(pool.kind, &base, &theirs, &mine, accept));
            prop_assert_eq!(&one_way, &merged(merge(pool.kind, &base, &mine, &theirs, accept)));
            prop_assert_eq!(one_way, build(&pool, |i| if who(i) < 2 { edit_at(i) } else { base_at(i) }));
        }

        #[test]
        fn hostile_payloads_never_panic_and_never_mix_a_group((kind, base, theirs, mine) in hostile_case()) {
            let elsewhere = names_another_row(&base, &theirs) || names_another_row(&base, &mine);
            for check in CHECKS {
                match merge(kind, &base, &theirs, &mine, check) {
                    Outcome::OtherRow => prop_assert!(elsewhere),
                    Outcome::Merged { payload, .. } => {
                        prop_assert!(check(&payload).is_empty());
                        holds(kind, &base, &theirs, &mine, &payload, &[])?;
                    }
                    Outcome::Choose(choice) => choice_holds(kind, &base, &theirs, &mine, &choice, check)?,
                    Outcome::Unfixable(groups) => prop_assert!(!elsewhere && !groups.is_empty()),
                }
            }
        }

        #[test]
        fn repeated_resolve_lands_or_is_unfixable_within_bounded_rounds((kind, base, theirs, mine) in hostile_case()) {
            for check in CHECKS {
                rounds_are_bounded(merge(kind, &base, &theirs, &mine, check), check)?;
            }
        }

        #[test]
        fn hostile_identities_hold_or_ask_only_for_validity((kind, base, theirs, _mine) in hostile_case()) {
            for outcome in [merge(kind, &base, &theirs, &base, accept), merge(kind, &base, &base, &theirs, accept), merge(kind, &base, &theirs, &theirs, accept)] {
                match outcome {
                    Outcome::Merged { payload, .. } => prop_assert_eq!(&payload, &theirs),
                    Outcome::OtherRow => prop_assert!(names_another_row(&base, &theirs)),
                    Outcome::Choose(choice) => {
                        prop_assert_eq!(kind, Kind::Expense);
                        prop_assert_eq!(reasons(&choice), vec![(known("money"), Reason::Invalid)]);
                        prop_assert!(!shares_add_up(&theirs));
                    }
                    Outcome::Unfixable(groups) => {
                        prop_assert_eq!(kind, Kind::Expense);
                        prop_assert_eq!(groups, vec![known("money")]);
                        prop_assert!(!shares_add_up(&theirs));
                    }
                }
            }
        }

        #[test]
        fn sound_shares_land_and_any_one_perturbation_asks(
            cents in 1i64..10_000_000,
            payers in prop::collection::vec(1u8..50, 1..4),
            debtors in prop::collection::vec(1u8..50, 1..5),
            which in any::<usize>(),
            how in any::<u8>(),
        ) {
            prop_assume!(cents >= 5);
            let mut shares = split(cents, &payers, false, 1);
            shares.extend(split(cents, &debtors, true, 10));
            prop_assume!(shares.iter().all(|s| s["amount"].as_f64().unwrap() > 0.0));
            let amount = cents as f64 / 100.0;
            let sound = with_shares(Value::Array(shares.clone()), amount);
            prop_assert!(shares_add_up(&sound));
            let mine = edit(&sound, json!({"name": "Bus"}), &[]);
            prop_assert_eq!(merged(merge(Kind::Expense, &sound, &sound, &mine, accept)), mine.clone());
            let broken = with_shares(Value::Array(perturbed(shares, which, how)), amount);
            match merge(Kind::Expense, &sound, &broken, &mine, accept) {
                Outcome::Choose(c) => prop_assert_eq!(reasons(&c), vec![(known("money"), Reason::Invalid)]),
                other => prop_assert!(false, "{:?} landed as {:?}", broken.get("shares"), other),
            }
        }
    }
}
