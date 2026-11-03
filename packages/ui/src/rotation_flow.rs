//! The network half of a project key rotation (docs/plans/key-lifecycle.md §8.3-8.6): the admin's
//! scan, staging and commit, and every other device's switch to the new key.

use api::account_projects::account_projects_controller::{get_account_projects, upsert_account_project};
use api::projects::projects_controller::get_project;
use api::projects::rotation_controller::{
    abort_rotation, commit_rotation, drop_rotation_box, read_rotation_rows, rotation_boxes, rotation_members,
    set_member_key, stage_rotation_rows, start_rotation,
};
use dioxus::fullstack::Json;
use dioxus::prelude::*;
use shared::{
    errors, Account, AbortRotation, CommitRotation, EncryptedPair, HistoryAction, HistoryContext, HistoryEntity, HistoryPayload,
    ProjectRef, QuarantinedRow, ReadRotationRows, RotationMember, RotationRow, SetMemberKey, StageRotationRows,
    ProjectSync, StartRotation, UpsertAccountProject, User,
};
use uuid::Uuid;

use crate::common::local_storage::{key_of, read_from_ls, LocalStorageState};
use crate::common::offline_queue::{read_queue, write_queue};
use crate::common::project_token::{scope, scope_key};
use crate::common::{update_ls, upsert_project_key};
use crate::crypto::{
    claim_token, claim_verifier, decrypt, encrypt, encrypt_padded_json, generate_key, key_to_fragment, open_wrap, wrap_key,
    WrapUse,
};
use crate::decrypted::{pickable_users, user_name};
use crate::friends::friends_service::own_keys;
use crate::rotation::{chained_payload, member_key_entry, may_switch, open_box, recipient_keys, restage, seal_box};
use crate::tid;

/// What the scan found, with every readable row already staged under the new key. Nothing is
/// switched until [`commit`]; [`abort`] lifts the freeze.
#[derive(Clone, PartialEq)]
pub struct Scan {
    pub project_id: Uuid,
    pub rotation_id: Uuid,
    pub key_epoch: i32,
    pub k2: [u8; 32],
    /// Rows no member's key opens (an AEAD failure, nothing else): set aside, never shown (D10).
    pub quarantine: Vec<QuarantinedRow>,
    pub members: Vec<RotationMember>,
}

/// The participant a rotation removes, by name for the history entry and the sheets.
#[derive(Debug, Clone, PartialEq)]
pub struct Removal {
    pub user_id: i32,
    pub name: String,
}

/// The account that claims this participant. With one, the removal is the commit's `remove`;
/// without, there is no membership to delete and hiding the participant is all it takes (D13).
pub fn claimant(members: &[RotationMember], user_id: i32) -> Option<Uuid> {
    members.iter().find(|m| m.user_id == Some(user_id)).map(|m| m.account_id)
}

/// Every member but the rotater and whoever leaves, whose directory entry checks (D11).
fn boxable<'a>(
    k1: &'a [u8; 32],
    members: &'a [RotationMember],
    me: Uuid,
    leaving: &'a [Uuid],
) -> impl Iterator<Item = (&'a RotationMember, (Vec<u8>, Vec<u8>))> + 'a {
    members
        .iter()
        .filter(move |m| m.account_id != me && !leaving.contains(&m.account_id))
        .filter_map(move |m| Some((m, recipient_keys(k1, m)?)))
}

/// What the rotate sheet says (artboard T8): how many members get the key on their own, and the
/// participants no boxed account claims, who get the new link from the rotater. Hidden participants
/// and the one being removed get neither.
#[derive(Debug, Clone, PartialEq)]
pub struct Audience {
    pub boxed: usize,
    pub by_link: Vec<String>,
}

pub fn audience(k1: &[u8; 32], users: &[User], members: &[RotationMember], me: Uuid, removee: Option<i32>) -> Audience {
    let leaving: Vec<Uuid> = removee.and_then(|id| claimant(members, id)).into_iter().collect();
    let boxed: Vec<&RotationMember> = boxable(k1, members, me, &leaving).map(|(m, _)| m).collect();
    let claimed: Vec<i32> = members
        .iter()
        .filter(|m| m.account_id == me || boxed.iter().any(|b| b.account_id == m.account_id))
        .filter_map(|m| m.user_id)
        .collect();
    Audience {
        boxed: boxed.len(),
        by_link: pickable_users(Some(k1), users, &[])
            .iter()
            .filter(|u| Some(u.id) != removee && !claimed.contains(&u.id))
            .map(|u| user_name(k1, u))
            .collect(),
    }
}

/// The audience before anything is frozen: the member list needs no rotation in progress.
pub async fn load_audience(
    project_id: Uuid,
    k1: [u8; 32],
    users: Vec<User>,
    me: Uuid,
    removee: Option<i32>,
) -> Result<Audience, ServerFnError> {
    scope_key(Some(&k1));
    let members = rotation_members(Json(ProjectRef { project_id })).await?;
    Ok(audience(&k1, &users, &members, me, removee))
}

/// The names shown, at most three, and how many more there are.
pub fn first_names(names: &[String]) -> (&[String], usize) {
    let shown = names.len().min(3);
    (&names[..shown], names.len() - shown)
}

/// "Tom", "Tom and Sam", "Tom, Sam and Hugo", "Tom, Sam, Hugo and 27 others".
pub fn names_and_others(names: &[String]) -> String {
    match first_names(names) {
        ([], _) => String::new(),
        ([one], 0) => one.clone(),
        ([rest @ .., last], 0) => tid!("rotation-names-and", first: rest.join(", "), last: last.clone()),
        (shown, more) => tid!("rotation-names-and-others", names: shown.join(", "), count: more as i64),
    }
}

/// The link as the new-link sheet shows it: enough to recognise it, not the key.
pub fn short_link(url: &str) -> String {
    let url = url.split_once("://").map_or(url, |(_, rest)| rest);
    let (base, fragment) = url.split_once('#').unwrap_or((url, ""));
    let base = base.get(..base.len().saturating_sub(32)).unwrap_or(base);
    let fragment: String = fragment.chars().take(4).collect();
    format!("{base}…#{fragment}…")
}

fn project_error() -> ServerFnError {
    ServerFnError::ServerError { message: errors::INVALID_PAYLOAD.into(), code: 400, details: None }
}

/// Freezes the project, reads every row, re-seals each under a new key and stages it. A wrong key
/// stops here: the project row must open under `k1`, or nothing is staged. `hidden` is the
/// participant a removal hides under the new key.
pub async fn begin(project_id: Uuid, k1: [u8; 32], hidden: Option<i32>) -> Result<Scan, ServerFnError> {
    scope_key(Some(&k1));
    let started = start_rotation(Json(StartRotation { project_id })).await?;
    let scanned = scan(project_id, started.rotation_id, &k1, hidden).await;
    if scanned.is_err() {
        scope_key(Some(&k1));
        let _ = abort_rotation(Json(AbortRotation { project_id, rotation_id: started.rotation_id })).await;
    }
    let (k2, quarantine) = scanned?;
    scope_key(Some(&k1));
    let members = rotation_members(Json(ProjectRef { project_id })).await?;
    Ok(Scan { project_id, rotation_id: started.rotation_id, key_epoch: started.key_epoch, k2, quarantine, members })
}

async fn scan(
    project_id: Uuid,
    rotation_id: Uuid,
    k1: &[u8; 32],
    hidden: Option<i32>,
) -> Result<([u8; 32], Vec<QuarantinedRow>), ServerFnError> {
    let k2 = generate_key();
    let mut quarantine = Vec::new();
    for table in shared::ROTATION_TABLES {
        let mut after: Option<String> = None;
        loop {
            scope_key(Some(k1));
            let page = read_rotation_rows(Json(ReadRotationRows { project_id, rotation_id, table: table.into(), after: after.clone() })).await?;
            let full = page.len() >= shared::ROTATION_PAGE;
            after = page.last().map(|r| r.id.clone());
            let (staged, set_aside) = restage_page(table, page, hidden, k1, &k2)?;
            quarantine.extend(set_aside);
            if !staged.is_empty() {
                scope_key(Some(k1));
                stage_rotation_rows(Json(StageRotationRows { project_id, rotation_id, table: table.into(), rows: staged })).await?;
            }
            if !full || table == "projects" {
                break;
            }
        }
    }
    Ok((k2, quarantine))
}

/// One page re-sealed under `k2`, and the rows no key opens, set aside (D10). An unreadable claim
/// label or payment-methods copy is neither: the commit clears every membership blob not staged.
fn restage_page(
    table: &str,
    page: Vec<RotationRow>,
    hidden: Option<i32>,
    k1: &[u8; 32],
    k2: &[u8; 32],
) -> Result<(Vec<RotationRow>, Vec<QuarantinedRow>), ServerFnError> {
    let (mut staged, mut quarantine) = (Vec::new(), Vec::new());
    for row in page {
        if table == "projects" {
            let plain = decrypt(k1, &row.payload).map_err(|_| project_error())?;
            let chained = chained_payload(k1, &plain).map_err(|_| project_error())?;
            staged.push(RotationRow { id: row.id, payload: encrypt(k2, &chained).map_err(|_| project_error())? });
            continue;
        }
        match restage(table, &row.id, hidden, k1, k2, &row.payload) {
            Some(payload) => staged.push(RotationRow { id: row.id, payload }),
            None if table == "claim_labels" || table == "payment_methods" => {}
            None => quarantine.push(QuarantinedRow { table: table.into(), id: row.id }),
        }
    }
    Ok((staged, quarantine))
}

pub async fn abort(scan: &Scan, k1: &[u8; 32]) {
    scope_key(Some(k1));
    let _ = abort_rotation(Json(AbortRotation { project_id: scan.project_id, rotation_id: scan.rotation_id })).await;
}

/// The switch. `remove` leaves in the same transaction; every other member whose directory entry
/// checks gets the new key boxed (D11). The rotater's other devices pull it from the escrow written
/// here. Answers how many boxes went out.
pub async fn commit(
    ls_ctx: Signal<LocalStorageState>,
    scan: &Scan,
    k1: [u8; 32],
    account: &Account,
    account_key: [u8; 32],
    remove: Vec<Uuid>,
    history: Option<HistoryContext>,
) -> Result<usize, ServerFnError> {
    let project_id = scan.project_id;
    let boxes: Vec<_> = boxable(&k1, &scan.members, account.id, &remove)
        .filter_map(|(m, (x25519, kem))| {
            let sealed = seal_box(&x25519, &kem, project_id, m.account_id, scan.key_epoch, &scan.k2)?;
            Some(shared::BoxedRotationKey { to_account_id: m.account_id, sealed })
        })
        .collect();
    let boxed = boxes.len();
    let escrow = wrap_key(&account_key, WrapUse::Escrow(project_id), &scan.k2).ok_or_else(project_error)?;
    let raw_ok = read_from_ls().raw_wraps_ok(account.id);
    let member_key = own_member_key_entry(&scan.k2, account, &account_key, raw_ok);
    let wrote_entry = member_key.is_some();
    scope_key(Some(&k1));
    let committed = commit_rotation(Json(CommitRotation {
        project_id,
        rotation_id: scan.rotation_id,
        new_verifier: claim_verifier(&scan.k2).to_vec(),
        escrow,
        amk_epoch: crate::auth::account_key::held_amk_epoch(),
        member_key,
        boxes,
        remove,
        quarantine: scan.quarantine.clone(),
        history,
    }))
    .await;
    crate::auth::account_key::note(&committed);
    committed?;
    let stamp = wrote_entry.then(|| own_keys(account, &account_key, raw_ok).map(|own| member_key_stamp(&own))).flatten();
    update_ls(ls_ctx, |state| {
        let seen = state.projects.iter().find(|p| p.project_id == project_id).map_or(0, |p| p.chain_seen);
        switch_local(state, project_id, &k1, &scan.k2, scan.key_epoch, seen + 1);
        if let (Some(stamp), Some(entry)) = (&stamp, state.projects.iter_mut().find(|p| p.project_id == project_id)) {
            entry.member_key_at = Some(scan.key_epoch);
            entry.member_key_for = Some(stamp.clone());
        }
    });
    Ok(boxed)
}

/// A finished rotation: the key this device switched to, and how many members got it boxed.
#[derive(Debug, Clone, PartialEq)]
pub struct Rotated {
    pub k2: [u8; 32],
    pub boxed: usize,
}

/// The whole rotation, from the freeze to the switch, with the history entry under the new key
/// (D16) and the actor inside it. A removed participant is hidden under the new key either way, and
/// one an account claims also loses its membership in the commit (D13). A failed commit lifts the
/// freeze.
pub async fn rotate(
    ls_ctx: Signal<LocalStorageState>,
    project_id: Uuid,
    k1: [u8; 32],
    account: &Account,
    account_key: [u8; 32],
    my_user_id: Option<i32>,
    removal: Option<Removal>,
) -> Result<Rotated, ServerFnError> {
    let scan = begin(project_id, k1, removal.as_ref().map(|r| r.user_id)).await?;
    let remove: Vec<Uuid> = removal.as_ref().and_then(|r| claimant(&scan.members, r.user_id)).into_iter().collect();
    let summary = match &removal {
        Some(r) => tid!("history-member-removed", name: r.name.clone()),
        None => tid!("history-key-changed"),
    };
    let history = encrypt_padded_json(
        &scan.k2,
        &HistoryPayload {
            summary,
            actor_user_id: my_user_id,
            action: Some(HistoryAction::Updated),
            entity: Some(HistoryEntity::Project),
            entity_id: None,
        },
    )
    .ok()
    .map(|payload| HistoryContext { actor_user_id: None, payload });
    match commit(ls_ctx, &scan, k1, account, account_key, remove, history).await {
        Ok(boxed) => Ok(Rotated { k2: scan.k2, boxed }),
        Err(e) => {
            abort(&scan, &k1).await;
            Err(e)
        }
    }
}

/// Moves this device to `k2`: the old key to `previous_keys`, the caches (K1 ciphertext) dropped,
/// queued ops re-sealed so they replay under the new key (§8.6). `chain_seen` is the length of the
/// chain the new key opened (L1). The member-key mark is cleared: the entry for the new epoch is
/// written by the next project open, or by `commit` when it sent one.
pub fn switch_local(state: &mut LocalStorageState, project_id: Uuid, k1: &[u8; 32], k2: &[u8; 32], key_epoch: i32, chain_seen: usize) {
    upsert_project_key(state, project_id, key_to_fragment(k2));
    if let Some(entry) = state.projects.iter_mut().find(|p| p.project_id == project_id) {
        entry.previous_keys.push((key_epoch - 1, key_to_fragment(k1)));
        entry.key_opened = true;
        entry.chain_seen = chain_seen;
        entry.key_rotated = false;
        entry.member_key_at = None;
        entry.member_key_for = None;
        entry.cached_project = None;
        entry.cached_users = None;
        entry.cached_expenses = None;
        entry.cached_payments = None;
        entry.cached_data_version = None;
    }
    let mut queue = read_queue();
    for op in queue.iter_mut().filter(|op| op.op.project_id() == project_id) {
        if let Ok(mut value) = serde_json::to_value(&*op) {
            crate::rotation::rekey_value(&mut value, k1, k2);
            if let Ok(rekeyed) = serde_json::from_value(value) {
                *op = rekeyed;
            }
        }
    }
    write_queue(&queue);
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Adopted {
    Switched,
    /// No box and no escrow passes §8.4: the device keeps its cache, read-only.
    NoKey,
}

/// After a `PROJECT_KEY_ROTATED`: the account's own escrow and its rotation boxes are the only
/// candidates, and each must pass `may_switch` against the row the server serves under it. A box
/// never fills an empty slot: with no key held, nothing is taken from one.
pub async fn adopt(
    ls_ctx: Signal<LocalStorageState>,
    project_id: Uuid,
    account: Option<Account>,
    account_key: Option<[u8; 32]>,
) -> Adopted {
    let Some(held) = read_from_ls().projects.iter().find(|p| p.project_id == project_id).and_then(key_of) else {
        return Adopted::NoKey;
    };
    let mut candidates: Vec<([u8; 32], bool)> = Vec::new();
    if let (Some(account), Some(ak)) = (account.as_ref(), account_key) {
        if let Ok(rows) = get_account_projects().await {
            let escrowed = rows
                .iter()
                .find(|r| r.project_id == project_id)
                .and_then(|r| r.key.as_ref())
                .and_then(|k| open_wrap(&ak, WrapUse::Escrow(project_id), k))
                .filter(|o| o.typed);
            candidates.extend(escrowed.map(|o| (o.key, false)));
        }
        if let (Some(own), Ok(boxes)) = (own_keys(account, &ak, false), rotation_boxes().await) {
            for b in boxes.iter().filter(|b| b.project_id == project_id) {
                candidates.extend(open_box(&own.x25519, &own.kem_seed, project_id, account.id, b).map(|k| (k, true)));
            }
        }
    }
    let chain_seen = read_from_ls().projects.iter().find(|p| p.project_id == project_id).map_or(0, |p| p.chain_seen);
    for (candidate, from_box) in candidates {
        scope_key(Some(&candidate));
        let Ok(served) = get_project(project_id).await else { continue };
        if !may_switch(&held, &candidate, &served, chain_seen) {
            continue;
        }
        let opened = crate::rotation::chain_len(&candidate, &served);
        update_ls(ls_ctx, |state| switch_local(state, project_id, &held, &candidate, served.key_epoch, opened));
        if let (true, Some(account), Some(ak)) = (from_box, account.as_ref(), account_key) {
            let user_id = read_from_ls().projects.iter().find(|p| p.project_id == project_id).and_then(|p| p.user_id);
            scope_key(Some(&candidate));
            let written = upsert_account_project(Json(UpsertAccountProject {
                project_id,
                user_id,
                key: wrap_key(&ak, WrapUse::Escrow(project_id), &candidate),
                claim_label: crate::crypto::claim_label(account, &ak, &candidate),
                claim_token: Some(claim_token(&candidate).to_vec()),
                amk_epoch: crate::auth::account_key::held_amk_epoch(),
            }))
            .await;
            crate::auth::account_key::note(&written);
            let _ = drop_rotation_box(Json(ProjectRef { project_id })).await;
        }
        return Adopted::Switched;
    }
    // Nothing of the project stays on screen or on disk for a device the new key did not reach. The
    // held key stays: it is what a new link's row must name to replace it (§8.4).
    update_ls(ls_ctx, |state| mark_rotated(state, project_id));
    Adopted::NoKey
}

/// A project whose new key did not reach this device: its caches go, its held key and its row on
/// the list stay, and the row opens T10 (D14).
pub fn mark_rotated(state: &mut LocalStorageState, project_id: Uuid) {
    if let Some(entry) = state.projects.iter_mut().find(|p| p.project_id == project_id) {
        entry.key_rotated = true;
        entry.cached_project = None;
        entry.cached_users = None;
        entry.cached_expenses = None;
        entry.cached_payments = None;
        entry.cached_data_version = None;
    }
}

/// Which of the projects the account no longer lists a rotation removed it from, rather than a
/// leave: the batch sync answers `rotated` for the token this device holds (D14, D17). Entries
/// already marked need no request.
pub async fn rotated_among(state: &LocalStorageState, left: &[Uuid]) -> Result<Vec<Uuid>, ServerFnError> {
    let unmarked: Vec<Uuid> =
        left.iter().copied().filter(|id| !state.projects.iter().any(|p| p.project_id == *id && p.key_rotated)).collect();
    if unmarked.is_empty() {
        return Ok(vec![]);
    }
    let (_, syncs) = crate::expenses::helpers::my_spending::sync_all(unmarked).await?;
    Ok(rotated_ids(&syncs))
}

/// This member's directory entry under `project_key`, from the keypair this device holds: the halves
/// the account key opens, and only while the server serves those same halves. Never from the served
/// `Account` alone, which would let a server have the member vouch for keys the server chose and
/// get the next key boxed to itself (security review 2026-11-02 H1).
pub fn own_member_key_entry(project_key: &[u8; 32], account: &Account, account_key: &[u8; 32], raw_ok: bool) -> Option<EncryptedPair> {
    let own = own_keys(account, account_key, raw_ok)?;
    member_key_entry(project_key, account.id, &own.x25519_public, &own.kem_public)
}

/// What `LocalStorageProject.member_key_for` remembers of the keys an entry vouched for.
pub fn member_key_stamp(own: &crate::friends::friends_service::OwnKeys) -> String {
    use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
    use sha2::{Digest, Sha256};
    URL_SAFE_NO_PAD.encode(Sha256::new().chain_update(&own.x25519_public).chain_update(&own.kem_public).finalize())
}

/// Whether the entry on record covers this epoch and these keys.
pub fn member_key_current(entry: &crate::common::local_storage::LocalStorageProject, key_epoch: i32, stamp: &str) -> bool {
    entry.member_key_at == Some(key_epoch) && entry.member_key_for.as_deref() == Some(stamp)
}

/// This member's directory entry, written from a project open once per key epoch and per keypair,
/// so a later rotation can box the new key to them. Nothing is written, and nothing is marked,
/// while the served keypair is not this device's.
pub async fn ensure_member_key(project_id: Uuid, account: &Account, account_key: &[u8; 32], raw_ok: bool, key_epoch: i32) {
    let state = read_from_ls();
    let Some(entry) = state.projects.iter().find(|p| p.project_id == project_id) else { return };
    let Some(pk) = key_of(entry) else { return };
    let Some(own) = own_keys(account, account_key, raw_ok) else { return };
    let stamp = member_key_stamp(&own);
    if member_key_current(entry, key_epoch, &stamp) {
        return;
    }
    let Some(member_key) = member_key_entry(&pk, account.id, &own.x25519_public, &own.kem_public) else { return };
    scope(project_id);
    if set_member_key(Json(SetMemberKey { project_id, member_key })).await.is_ok() {
        let mut state = read_from_ls();
        if let Some(e) = state.projects.iter_mut().find(|p| p.project_id == project_id) {
            e.member_key_at = Some(key_epoch);
            e.member_key_for = Some(stamp);
        }
        crate::common::local_storage::write_to_ls(&state);
    }
}

pub fn is_key_rotated(e: &ServerFnError) -> bool {
    matches!(e, ServerFnError::ServerError { message, code: 403, .. } if message == errors::PROJECT_KEY_ROTATED)
}

/// The projects a batch sync answered as rotated (D17).
pub fn rotated_ids(syncs: &[ProjectSync]) -> Vec<Uuid> {
    syncs.iter().filter_map(|s| s.rotated).collect()
}

/// Rotated projects the list keeps a row for, though no key reached this device: the row opens the
/// page that says so (artboard T10). Nothing of the project is left to name it by.
pub fn lost_projects(state: &LocalStorageState, listed: &[Uuid]) -> Vec<Uuid> {
    state.projects.iter().filter(|p| p.key_rotated && !listed.contains(&p.project_id)).map(|p| p.project_id).collect()
}

/// D17: every rotated project switched through a box or the escrow when one passes §8.4. True
/// when any did, so the list syncs again under the new keys.
pub async fn adopt_all(
    ls_ctx: Signal<LocalStorageState>,
    ids: &[Uuid],
    account: Option<Account>,
    account_key: Option<[u8; 32]>,
) -> bool {
    let mut switched = false;
    for id in ids {
        switched |= adopt(ls_ctx, *id, account.clone(), account_key).await == Adopted::Switched;
    }
    switched
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::local_storage::LocalStorageProject;
    use crate::common::test_fixtures::make_user;
    use crate::crypto::{encrypt_json, generate_key};
    use shared::UserPayload;

    fn member(k1: &[u8; 32], user_id: Option<i32>, entry: bool) -> RotationMember {
        let account_id = Uuid::new_v4();
        let (x, kem) = (vec![1u8; 32], vec![2u8; shared::KEM_PUBLIC_KEY_LENGTH]);
        RotationMember {
            account_id,
            user_id,
            member_key: entry.then(|| member_key_entry(k1, account_id, &x, &kem)).flatten(),
            public_key: Some(x),
            kem_public_key: Some(kem),
        }
    }

    fn hidden(k1: &[u8; 32], id: i32, name: &str) -> User {
        User { id, payload: encrypt_json(k1, &UserPayload { name: name.into(), removed: true }).unwrap(), ..Default::default() }
    }

    #[test]
    fn every_member_whose_entry_checks_gets_the_key_but_me_and_whoever_leaves() {
        let k1 = generate_key();
        let (me, lea, paul, old_client) = (member(&k1, Some(1), true), member(&k1, Some(2), true), member(&k1, Some(3), true), member(&k1, Some(4), false));
        let members = vec![me.clone(), lea, paul, old_client];
        assert_eq!(audience(&k1, &[], &members, me.account_id, None).boxed, 2);
        assert_eq!(audience(&k1, &[], &members, me.account_id, Some(2)).boxed, 1, "Léa leaves with the commit");
        assert_eq!(audience(&k1, &[], &members, me.account_id, Some(9)).boxed, 2, "a participant no account claims");
    }

    #[test]
    fn the_link_goes_to_participants_no_account_claims_neither_hidden_nor_removed() {
        let k1 = generate_key();
        let me = member(&k1, Some(1), true);
        let users = vec![make_user(&k1, 1, "Marie"), make_user(&k1, 2, "Tom"), make_user(&k1, 3, "Sam"), hidden(&k1, 4, "Hugo"), make_user(&k1, 5, "Léa")];
        let by_link = |removee| audience(&k1, &users, std::slice::from_ref(&me), me.account_id, removee).by_link;
        assert_eq!(by_link(None), vec!["Tom", "Sam", "Léa"]);
        assert_eq!(by_link(Some(5)), vec!["Tom", "Sam"]);
    }

    /// Security review 2026-11-02 H1, the reproduction inverted: the entry vouches for the halves
    /// the account key opens, and for nothing when a served half is not one of them.
    #[test]
    fn the_directory_entry_comes_from_the_devices_own_keys_or_not_at_all() {
        use crate::crypto::{public_key_of, wrap_kem_seed, wrap_key, WrapUse};
        use crate::hybrid::{generate_kem_seed, kem_public_key};
        let (k1, k2, ak) = (generate_key(), generate_key(), generate_key());
        let (x, seed) = (generate_key(), generate_kem_seed());
        let me = Uuid::new_v4();
        let genuine = Account {
            id: me,
            public_key: Some(public_key_of(&x)),
            kem_public_key: Some(kem_public_key(&seed)),
            private_key: wrap_key(&ak, WrapUse::PrivateKey, &x),
            kem_private_key: wrap_kem_seed(&ak, &seed),
            ..Default::default()
        };
        let entry = own_member_key_entry(&k1, &genuine, &ak, false).unwrap();
        let member = RotationMember { account_id: me, user_id: None, public_key: genuine.public_key.clone(), kem_public_key: genuine.kem_public_key.clone(), member_key: Some(entry) };
        let (to_x, to_kem) = recipient_keys(&k1, &member).expect("the admin boxes to the device's own keys");
        let sealed = seal_box(&to_x, &to_kem, Uuid::nil(), me, 1, &k2).unwrap();
        let boxed = shared::RotationBox { project_id: Uuid::nil(), key_epoch: 1, sealed };
        assert_eq!(open_box(&x, &seed, Uuid::nil(), me, &boxed), Some(k2), "and the member opens the box");

        let (server_x, server_seed) = (generate_key(), generate_kem_seed());
        let both = Account { public_key: Some(public_key_of(&server_x)), kem_public_key: Some(kem_public_key(&server_seed)), ..genuine.clone() };
        assert_eq!(own_member_key_entry(&k1, &both, &ak, false), None, "no entry vouches for keys the device does not hold");
        let kem_only = Account { kem_public_key: Some(kem_public_key(&server_seed)), ..genuine.clone() };
        assert_eq!(own_member_key_entry(&k1, &kem_only, &ak, false), None, "nor for a substituted ML-KEM half alone");

        let stamp = member_key_stamp(&own_keys(&genuine, &ak, false).unwrap());
        let mut on_record = LocalStorageProject { member_key_at: Some(1), member_key_for: Some(stamp.clone()), ..Default::default() };
        assert!(member_key_current(&on_record, 1, &stamp));
        assert!(!member_key_current(&on_record, 2, &stamp), "a new epoch is written again");
        on_record.member_key_for = Some("another keypair".into());
        assert!(!member_key_current(&on_record, 1, &stamp), "a new keypair is written again");
    }

    /// A switch clears the member-key mark: the entry for the new epoch is written by the next
    /// project open, from the keys the device holds then.
    #[test]
    fn a_switch_forgets_the_member_key_mark_and_keeps_the_chain_it_opened() {
        let (k1, k2) = (generate_key(), generate_key());
        let id = Uuid::new_v4();
        let mut state = LocalStorageState::default();
        state.projects.push(LocalStorageProject {
            project_id: id,
            encryption_key: Some(key_to_fragment(&k1)),
            admin: true,
            member_key_at: Some(1),
            member_key_for: Some("stamp".into()),
            chain_seen: 1,
            ..Default::default()
        });
        switch_local(&mut state, id, &k1, &k2, 2, 2);
        let entry = &state.projects[0];
        assert_eq!((entry.member_key_at, entry.member_key_for.as_deref(), entry.chain_seen), (None, None, 2));
        assert_eq!(key_of(entry), Some(k2));
        assert!(entry.key_opened);
    }

    #[test]
    fn a_member_whose_entry_does_not_check_gets_the_link() {
        let k1 = generate_key();
        let (me, paul, old_client) = (member(&k1, Some(1), true), member(&k1, Some(2), true), member(&k1, Some(3), false));
        let users = vec![make_user(&k1, 1, "Marie"), make_user(&k1, 2, "Paul"), make_user(&k1, 3, "Tom")];
        let audience = audience(&k1, &users, &[me.clone(), paul, old_client], me.account_id, None);
        assert_eq!((audience.boxed, audience.by_link), (1, vec!["Tom".to_string()]));
    }

    #[test]
    fn a_removee_an_account_claims_leaves_through_the_commit_else_by_hiding() {
        let k1 = generate_key();
        let lea = member(&k1, Some(2), false);
        assert_eq!(claimant(std::slice::from_ref(&lea), 2), Some(lea.account_id));
        assert_eq!(claimant(&[lea], 3), None);
    }

    /// D10 and the commit check (rotation_repository::swap_all): junk in a project set is listed
    /// as set aside, junk in a membership blob is left out of both lists for the commit to clear.
    #[test]
    fn an_unreadable_membership_blob_is_left_to_the_commit_not_quarantined() {
        let (k1, k2, other) = (generate_key(), generate_key(), generate_key());
        let row = |id: &str, key: &[u8; 32]| RotationRow { id: id.into(), payload: encrypt(key, "x").unwrap() };
        for table in ["claim_labels", "payment_methods"] {
            let (staged, quarantine) = restage_page(table, vec![row("a", &k1), row("b", &other)], None, &k1, &k2).unwrap();
            assert_eq!(staged.iter().map(|r| r.id.as_str()).collect::<Vec<_>>(), vec!["a"], "{table}");
            assert!(quarantine.is_empty(), "{table}");
        }
        let (staged, quarantine) = restage_page("expenses", vec![row("1", &k1), row("2", &other)], None, &k1, &k2).unwrap();
        assert_eq!((staged.len(), quarantine), (1, vec![QuarantinedRow { table: "expenses".into(), id: "2".into() }]));
        assert!(restage_page("projects", vec![row("p", &other)], None, &k1, &k2).is_err(), "a wrong key stops the scan");
    }

    #[test]
    fn three_names_at_most_then_how_many_more() {
        let names: Vec<String> = (0..30).map(|i| format!("P{i}")).collect();
        assert_eq!(first_names(&names), (&names[..3], 27));
        assert_eq!(first_names(&names[..3]), (&names[..3], 0));
        assert_eq!(first_names(&names[..1]), (&names[..1], 0));
        assert_eq!(first_names(&[]), (&[][..], 0));
    }

    #[test]
    fn the_shown_link_never_carries_the_key() {
        let key = generate_key();
        let id = Uuid::parse_str("8c1e2f3a-0000-4000-8000-000000000000").unwrap();
        let url = crate::common::share_link_for(id, &key);
        let shown = short_link(&url);
        let fragment = key_to_fragment(&key);
        assert_eq!(shown, format!("counted.fr/projects/8c1e…#{}…", &fragment[..4]));
        assert!(!shown.contains(&fragment[..8]));
    }

    #[test]
    fn a_rotated_project_keeps_its_row_until_a_key_reaches_it() {
        let syncs = vec![
            ProjectSync { data_version: -1, project: None, users: None, expenses: None, payments: None, recurring: None, server_date: None, rotated: Some(Uuid::from_u128(1)) },
            ProjectSync { data_version: 3, project: None, users: None, expenses: None, payments: None, recurring: None, server_date: None, rotated: None },
        ];
        assert_eq!(rotated_ids(&syncs), vec![Uuid::from_u128(1)]);
        let mut state = LocalStorageState::default();
        for (n, rotated) in [(1, true), (2, false), (3, true)] {
            state.projects.push(LocalStorageProject { project_id: Uuid::from_u128(n), key_rotated: rotated, ..Default::default() });
        }
        assert_eq!(lost_projects(&state, &[Uuid::from_u128(2)]), vec![Uuid::from_u128(1), Uuid::from_u128(3)]);
        assert_eq!(lost_projects(&state, &[Uuid::from_u128(3)]), vec![Uuid::from_u128(1)], "listed again once switched");
    }
}
