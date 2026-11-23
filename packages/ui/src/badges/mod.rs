//! Personal badges (docs/plans/gamification.md §3.2): worked out on this device, seen by nobody
//! else. Earned ones are kept in a small local store and ride along with the account's encrypted
//! `Preferences` on the next write that happens anyway (`preferences::write`), never on their own.

pub mod page;
pub mod members;
pub use members::{fact as members_fact, label as members_label};
pub mod shown;
use dioxus::prelude::*;
use shared::time::Date;
use serde::{Deserialize, Serialize};
use shared::{Account, Preferences};
use std::cell::Cell;
use std::collections::{BTreeMap, HashSet};
use uuid::Uuid;

use crate::common::{local_today, today_iso};
use crate::common::fx_cache::CachedFx;
use crate::common::{key_of, persist, Flash, LocalStorageState};
use crate::crypto::decrypt_json;
use crate::expenses::helpers::budgets::{budget_lines, spent_by_category};
use crate::expenses::helpers::my_spending::{cached_ledgers, first_of_month, share_rows, shift_month};
use crate::expenses::helpers::project_data::{self, LiveData};
use crate::tid;

pub const PREMIER_TICKET: &str = "premier_ticket";
pub const TOURISTE: &str = "touriste";
pub const RECURRENT: &str = "recurrent";
pub const DEMENAGEMENT: &str = "demenagement";
pub const ARCHIVISTE: &str = "archiviste";
pub const HORS_LIGNE: &str = "hors_ligne";
pub const SUR_MESURE: &str = "sur_mesure";
pub const CLE_DE_SECOURS: &str = "cle_de_secours";
pub const CODE_VERIFIE: &str = "code_verifie";
pub const CLE_CHANGEE: &str = "cle_changee";
pub const A_PLUSIEURS: &str = "a_plusieurs";
pub const PREMIER_AMI: &str = "premier_ami";
pub const COORDONNEES_PARTAGEES: &str = "coordonnees_partagees";
pub const PREMIER_BUDGET: &str = "premier_budget";
pub const BUDGET_TENU: &str = "budget_tenu";

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Family {
    Discovery,
    Security,
    Together,
    Personal,
}

pub const FAMILIES: [(Family, &str); 4] = [
    (Family::Discovery, "badges-family-discovery"),
    (Family::Security, "badges-family-security"),
    (Family::Together, "badges-family-together"),
    (Family::Personal, "badges-family-personal"),
];

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Badge {
    pub id: &'static str,
    pub family: Family,
    pub name: &'static str,
    pub how: &'static str,
}

const fn badge(id: &'static str, family: Family, name: &'static str, how: &'static str) -> Badge {
    Badge { id, family, name, how }
}

pub const CATALOGUE: [Badge; 15] = [
    badge(PREMIER_TICKET, Family::Discovery, "badge-premier-ticket", "badge-premier-ticket-how"),
    badge(TOURISTE, Family::Discovery, "badge-touriste", "badge-touriste-how"),
    badge(RECURRENT, Family::Discovery, "badge-recurrent", "badge-recurrent-how"),
    badge(DEMENAGEMENT, Family::Discovery, "badge-demenagement", "badge-demenagement-how"),
    badge(ARCHIVISTE, Family::Discovery, "badge-archiviste", "badge-archiviste-how"),
    badge(HORS_LIGNE, Family::Discovery, "badge-hors-ligne", "badge-hors-ligne-how"),
    badge(SUR_MESURE, Family::Discovery, "badge-sur-mesure", "badge-sur-mesure-how"),
    badge(CLE_DE_SECOURS, Family::Security, "badge-cle-de-secours", "badge-cle-de-secours-how"),
    badge(CODE_VERIFIE, Family::Security, "badge-code-verifie", "badge-code-verifie-how"),
    badge(CLE_CHANGEE, Family::Security, "badge-cle-changee", "badge-cle-changee-how"),
    badge(A_PLUSIEURS, Family::Together, "badge-a-plusieurs", "badge-a-plusieurs-how"),
    badge(PREMIER_AMI, Family::Together, "badge-premier-ami", "badge-premier-ami-how"),
    badge(COORDONNEES_PARTAGEES, Family::Together, "badge-coordonnees-partagees", "badge-coordonnees-partagees-how"),
    badge(PREMIER_BUDGET, Family::Personal, "badge-premier-budget", "badge-premier-budget-how"),
    badge(BUDGET_TENU, Family::Personal, "badge-budget-tenu", "badge-budget-tenu-how"),
];

pub fn find(id: &str) -> Option<&'static Badge> {
    CATALOGUE.iter().find(|b| b.id == id)
}

const KEY: &str = "counted_badges";
const LABEL: &str = "badges";

/// This device's copy, for one account. `baselined` turns true once the first evaluation for that
/// account has run: what was earned before it is recorded without a toast (plan R6).
#[derive(Serialize, Deserialize, Default, Clone, PartialEq, Debug)]
pub struct BadgeState {
    #[serde(default)]
    pub account: Option<Uuid>,
    #[serde(default)]
    pub baselined: bool,
    #[serde(default)]
    pub earned: BTreeMap<String, String>,
    #[serde(default)]
    pub off: bool,
}

impl BadgeState {
    pub fn count(&self) -> usize {
        CATALOGUE.iter().filter(|b| self.earned.contains_key(b.id)).count()
    }

    fn for_account(&mut self, account: Uuid) {
        if self.account != Some(account) {
            *self = BadgeState { account: Some(account), ..BadgeState::default() };
        }
    }
}

#[derive(Clone, Copy)]
pub struct Badges(pub Signal<BadgeState>);

pub fn read_store() -> BadgeState {
    persist::read_json(KEY)
}

fn write_store(state: &BadgeState) {
    persist::write_json(KEY, LABEL, state);
}

pub fn clear_store() {
    persist::remove_store(KEY);
}

fn signed_in() -> Option<Uuid> {
    try_consume_context::<Signal<Option<Account>>>().and_then(|a| a.peek().as_ref().map(|a| a.id))
}

/// Records `id` as earned by this account on this device. Toasts only after the first evaluation
/// and while badges are switched on; a badge already earned does nothing.
pub fn award(id: &'static str) {
    let (Some(Badges(mut sig)), Some(account)) = (try_consume_context::<Badges>(), signed_in()) else {
        return;
    };
    let newly = {
        let mut state = sig.write();
        state.for_account(account);
        state.earned.insert(id.to_string(), today_iso()).is_none()
    };
    if !newly {
        return;
    }
    let state = sig.peek().clone();
    write_store(&state);
    if state.baselined && !state.off {
        toast(id);
    }
}

/// Joins a success message the same action already raised in this tick, rather than queueing a
/// second toast and a second haptic; never covers a failure.
fn toast(id: &str) {
    let (Some(mut flash), Some(badge)) = (try_consume_context::<Signal<Option<Flash>>>(), find(id)) else {
        return;
    };
    let msg = tid!("badges-earned-toast", name: tid!(badge.name));
    let mut current = flash.write();
    match current.as_mut() {
        Some(existing) if existing.success => existing.msg = format!("{} {msg}", existing.msg),
        Some(_) => {}
        None => *current = Some(Flash::ok(msg)),
    }
}

/// What the account's `Preferences` already hold, folded into this device's copy without a toast:
/// another device earned it.
pub fn adopt(mut sig: Signal<BadgeState>, account: &Account, key: &[u8; 32]) {
    let stored = account
        .preferences
        .as_ref()
        .and_then(|p| decrypt_json::<Preferences>(key, p).ok())
        .map(|p| p.badges)
        .unwrap_or_default();
    let mut state = sig.peek().clone();
    let before = state.clone();
    state.for_account(account.id);
    merge_earned(&mut state.earned, &stored.earned);
    if stored.off {
        state.off = true;
    }
    if state != before {
        write_store(&state);
        sig.set(state);
    }
}

/// Called on every `Preferences` write: the union of what the account held and this device earned.
pub fn merge_into(prefs: &mut Preferences, account: Uuid) {
    let local = read_store();
    if local.account != Some(account) {
        return;
    }
    merge_earned(&mut prefs.badges.earned, &local.earned);
    prefs.badges.off = local.off;
}

fn merge_earned(into: &mut BTreeMap<String, String>, from: &BTreeMap<String, String>) {
    for (id, date) in from {
        match into.get(id) {
            Some(existing) if existing <= date => {}
            _ => {
                into.insert(id.clone(), date.clone());
            }
        }
    }
}

pub fn set_off(mut sig: Signal<BadgeState>, off: bool) {
    let mut state = sig.peek().clone();
    state.off = off;
    write_store(&state);
    sig.set(state);
}

thread_local! {
    static BUDGET_CHECKED: Cell<bool> = const { Cell::new(false) };
}

/// Badges read from data this device already holds. Each check is skipped once its badge is earned,
/// and the budget check, which decrypts the personal ledger, runs once per session.
pub fn evaluate(mut sig: Signal<BadgeState>, account: &Account, key: &[u8; 32], ls: &LocalStorageState) {
    if sig.peek().account != Some(account.id) {
        sig.write().for_account(account.id);
    }
    if !sig.peek().baselined {
        adopt(sig, account, key);
    }
    let earned = sig.peek().earned.clone();
    let missing = |id: &str| !earned.contains_key(id);

    if missing(CLE_DE_SECOURS) && account.has_recovery_key {
        award(CLE_DE_SECOURS);
    }
    if missing(COORDONNEES_PARTAGEES)
        && crate::payment_methods::account_payment_methods(account, key).iter().any(|m| m.shared)
    {
        award(COORDONNEES_PARTAGEES);
    }
    if missing(A_PLUSIEURS) && someone_joined(ls) {
        award(A_PLUSIEURS);
    }
    if (missing(PREMIER_BUDGET) || missing(BUDGET_TENU)) && !BUDGET_CHECKED.with(Cell::get) {
        if let Some(personal) = ls.personal_project {
            BUDGET_CHECKED.with(|c| c.set(true));
            let uncounted = crate::preferences::uncounted_projects(account, key);
            let fx = crate::common::fx_cache::read_fx();
            let (has_budget, held) = budget_status(ls, personal, &uncounted, fx.as_ref(), local_today());
            if has_budget {
                award(PREMIER_BUDGET);
            }
            if held {
                award(BUDGET_TENU);
            }
        }
    }

    if !sig.peek().baselined {
        let mut state = sig.peek().clone();
        state.baselined = true;
        write_store(&state);
        sig.set(state);
    }
}

/// A shared project where a participant other than this device's identity is held by an account.
pub fn someone_joined(ls: &LocalStorageState) -> bool {
    ls.projects.iter().filter(|p| Some(p.project_id) != ls.personal_project).any(|p| {
        p.user_id.is_some() && p.cached_users.iter().flatten().any(|u| u.claimed && Some(u.id) != p.user_id)
    })
}

/// Whether the personal project has a ceiling, and whether last month stayed under every ceiling
/// with something spent. Ceilings are kept without history, so last month is read against today's.
fn budget_status(
    ls: &LocalStorageState,
    personal: Uuid,
    uncounted: &HashSet<Uuid>,
    fx: Option<&CachedFx>,
    today: Date,
) -> (bool, bool) {
    let Some(p) = ls.projects.iter().find(|p| p.project_id == personal) else { return (false, false) };
    let (Some(key), Some(expenses)) = (key_of(p), p.cached_expenses.clone()) else { return (false, false) };
    let data = project_data::build(
        Some(key),
        Some(&LiveData {
            project_id: personal,
            project: p.cached_project.clone(),
            users: p.cached_users.clone().unwrap_or_default(),
            expenses,
            payments: p.cached_payments.clone().unwrap_or_default(),
            version: None,
        }),
        Some(personal),
    );
    let has_budget = data.budgets.monthly.values().any(|v| *v > 0.0) || data.budgets_custom.values().any(|v| *v > 0.0);
    if !has_budget {
        return (false, false);
    }
    let mut shares = share_rows(&cached_ledgers(ls, personal), uncounted, &data.currency, fx);
    shares.retain(|s| s.project_id != personal);
    let last_month = shift_month(first_of_month(today), -1);
    let spent = spent_by_category(&data.expenses, &shares, last_month);
    let lines = budget_lines(&data.budgets, &data.budgets_custom, &data.categories, &spent);
    let held = !lines.is_empty() && spent.values().any(|v| *v > 0.0) && lines.iter().all(|l| l.spent <= l.limit);
    (true, held)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn every_badge_key_is_a_known_message() {
        let known = crate::i18n::locale_ids(crate::i18n::FALLBACK);
        for b in CATALOGUE {
            assert!(known.contains(b.name), "{} is not defined in the fallback locale", b.name);
            assert!(known.contains(b.how), "{} is not defined in the fallback locale", b.how);
        }
        for (_, key) in FAMILIES {
            assert!(known.contains(key), "{key} is not defined in the fallback locale");
        }
    }

    #[test]
    fn ids_are_unique() {
        let ids: HashSet<&str> = CATALOGUE.iter().map(|b| b.id).collect();
        assert_eq!(ids.len(), CATALOGUE.len());
    }

    #[test]
    fn merging_keeps_the_earliest_date_and_every_badge() {
        let mut into = BTreeMap::from([("a".to_string(), "2026-11-10".to_string()), ("b".to_string(), "2026-11-01".to_string())]);
        let from = BTreeMap::from([("a".to_string(), "2026-11-05".to_string()), ("c".to_string(), "2026-11-12".to_string())]);
        merge_earned(&mut into, &from);
        assert_eq!(into["a"], "2026-11-05");
        assert_eq!(into["b"], "2026-11-01");
        assert_eq!(into["c"], "2026-11-12");
    }

    #[test]
    fn another_account_starts_from_nothing() {
        let mine = Uuid::new_v4();
        let mut state = BadgeState { account: Some(mine), baselined: true, earned: BTreeMap::from([("a".into(), "2026-11-01".into())]), off: true };
        state.for_account(mine);
        assert_eq!(state.count(), 0);
        assert!(state.baselined);
        state.for_account(Uuid::new_v4());
        assert!(state.earned.is_empty() && !state.baselined && !state.off);
    }

    #[test]
    fn preferences_round_trip_the_badges_and_skip_them_when_empty() {
        let empty = serde_json::to_string(&Preferences::default()).unwrap();
        assert!(!empty.contains("badges"));
        let mut prefs = Preferences::default();
        prefs.badges.earned.insert(TOURISTE.into(), "2026-11-17".into());
        let json = serde_json::to_string(&prefs).unwrap();
        let back: Preferences = serde_json::from_str(&json).unwrap();
        assert_eq!(back.badges.earned[TOURISTE], "2026-11-17");
    }
}
