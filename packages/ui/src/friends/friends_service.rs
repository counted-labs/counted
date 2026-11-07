//! Everything the friends pages decide, with no rendering in it: what a friend row shows, what a
//! request sends, how a boxed project key is opened and kept. See `docs/plans/friends.md`.

use api::friends::friends_controller::{
    accept_request, delete_request, get_friends, remove_friend, request_by_email,
    request_from_project,
};
use api::friends::invitations_controller::{delete_invitation, get_invitations, invite_batch};
use api::projects::projects_controller::get_projects_by_ids;
use dioxus::fullstack::Json;
use dioxus::prelude::{ServerFnError, Signal};
use shared::{
    Account, BatchProject, CreatableProjectInvitation, Friend, FriendRequestByEmail,
    FriendRequestFromProject, FriendsView, IncomingFriendRequest, ProjectDto, ProjectInvitation,
};
use uuid::Uuid;

use crate::common::{adopt_project_key, update_ls_if_changed, LocalStorageState};
use crate::crypto::{
    box_project_key, decrypt, encrypt_padded_json_within, key_fingerprint, open_kem_seed, open_wrap, public_key_of,
    unbox_project_key, unwrap_key, WrapUse,
};
use crate::hybrid::{kem_public_key, open_auth, seal_auth, KEM_SEED_LENGTH};
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use serde::{Deserialize, Serialize};
use shared::{EncryptedPair, HybridBox};
use std::collections::BTreeMap;
use crate::decrypted::decrypt_project;

const INVITE_CONTEXT: &[u8] = b"counted-invite-v1";

/// What a hybrid invitation box holds: the key, and which project, sender and recipient it is for,
/// so a box re-served for another project or account is refused (security-fixes §5).
#[derive(Serialize, Deserialize)]
struct InviteKey {
    #[serde(rename = "use")]
    use_: String,
    project: Uuid,
    from: Uuid,
    to: Uuid,
    key: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FriendRow {
    pub id: Uuid,
    pub account_id: Uuid,
    pub email: String,
    /// `None` when the friend has not logged in on a build that makes a keypair yet - they cannot
    /// be invited until they do.
    pub fingerprint: Option<String>,
    /// The server now serves a key other than the one this device first saw: nothing is boxed to
    /// it or opened from it until the codes are compared again.
    pub key_changed: bool,
    /// The key now served when `key_changed`: its code is shown, and "The code matches" pins it
    /// once read out by the friend (docs/plans/key-lifecycle.md §7).
    pub new_key: Option<Vec<u8>>,
    /// The ML-KEM key now served, pinned with `new_key`. The code covers only the X25519 key.
    pub new_kem_key: Option<Vec<u8>>,
}

impl FriendRow {
    pub fn new_fingerprint(&self) -> Option<String> {
        self.new_key.as_deref().map(key_fingerprint)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct OutgoingRow {
    pub id: Uuid,
    /// What the requester typed, decrypted with their own key; blank when this device has no key.
    pub label: String,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct FriendsLists {
    /// This account's own code: what its friends see next to its email, and what they read out.
    /// Computed from the private key this device opened, never from the served public key.
    pub my_fingerprint: Option<String>,
    /// The served public key is not the one behind this device's private key.
    pub key_mismatch: bool,
    pub friends: Vec<FriendRow>,
    pub incoming: Vec<IncomingFriendRequest>,
    pub outgoing: Vec<OutgoingRow>,
}

pub fn friend_row(f: &Friend, changed: &[Uuid]) -> FriendRow {
    FriendRow {
        id: f.id,
        account_id: f.account_id,
        email: f.email.clone(),
        fingerprint: f.public_key.as_deref().map(key_fingerprint),
        key_changed: changed.contains(&f.account_id),
        new_key: None,
        new_kem_key: None,
    }
}

/// After the two friends read the new code to each other: the served keys become the pins. A
/// device cannot tell a rotation from a substitution, so only a person comparing codes can. The
/// code does not cover the ML-KEM key: one substituted here leaves this friend's invitations at
/// X25519 strength, never below the classical box (security-fixes §5 c).
pub fn trust_key(state: &mut LocalStorageState, account_id: Uuid, key: &[u8], kem_key: Option<&[u8]>) {
    state.friend_keys.insert(account_id, URL_SAFE_NO_PAD.encode(key));
    if let Some(kem_key) = kem_key {
        state.friend_kem_keys.insert(account_id, URL_SAFE_NO_PAD.encode(kem_key));
    }
}

/// `true` when `served` differs from the pin, and then takes it off.
fn pin(pins: &mut BTreeMap<Uuid, String>, account_id: Uuid, served: &mut Option<Vec<u8>>) -> bool {
    let Some(encoded) = served.as_deref().map(|k| URL_SAFE_NO_PAD.encode(k)) else { return false };
    match pins.get(&account_id) {
        Some(pinned) if *pinned != encoded => {
            *served = None;
            true
        }
        Some(_) => false,
        None => {
            pins.insert(account_id, encoded);
            false
        }
    }
}

/// Pins each friend's X25519 and ML-KEM keys the first time they are served, and takes off a key
/// that differs from its pin, returning those friends. Both keys are written once and only a
/// verified account can be a friend, so a changed key is the server's substitution: without its
/// key, the friend is neither boxed to nor opened from.
pub fn pin_keys(state: &mut LocalStorageState, friends: &mut [Friend]) -> Vec<Uuid> {
    let mut changed = Vec::new();
    for friend in friends {
        let x25519_changed = pin(&mut state.friend_keys, friend.account_id, &mut friend.public_key);
        let kem_changed = pin(&mut state.friend_kem_keys, friend.account_id, &mut friend.kem_public_key);
        if x25519_changed || kem_changed {
            changed.push(friend.account_id);
        }
    }
    changed
}

/// The only way the friends list is read: every key checked against this device's pins.
pub async fn get_pinned_friends(ls_ctx: Signal<LocalStorageState>) -> Result<(FriendsView, Vec<Uuid>), ServerFnError> {
    let mut view = get_friends().await?;
    let mut changed = Vec::new();
    update_ls_if_changed(ls_ctx, |state| changed = pin_keys(state, &mut view.friends));
    Ok((view, changed))
}

/// `raw_ok` is `LocalStorageState::raw_wraps_ok` for the account; `changed` comes from [`pin_keys`].
pub fn to_lists(
    view: &FriendsView,
    changed: &[Uuid],
    account_key: Option<&[u8; 32]>,
    account: Option<&Account>,
    raw_ok: bool,
) -> FriendsLists {
    let my_private = account.zip(account_key).and_then(|(a, k)| my_private_key(a, k, raw_ok));
    FriendsLists {
        my_fingerprint: my_private.map(|p| key_fingerprint(&public_key_of(&p))),
        key_mismatch: account.zip(account_key).is_some_and(|(a, k)| served_key_mismatch(a, k, raw_ok)),
        friends: view.friends.iter().map(|f| friend_row(f, changed)).collect(),
        incoming: view.incoming.clone(),
        outgoing: view
            .outgoing
            .iter()
            .map(|o| OutgoingRow {
                id: o.id,
                label: account_key.and_then(|k| decrypt(k, &o.label).ok()).map(read_label).unwrap_or_default(),
            })
            .collect(),
    }
}

pub async fn load(
    ls_ctx: Signal<LocalStorageState>,
    account_key: Option<[u8; 32]>,
    account: Account,
    raw_ok: bool,
) -> Result<FriendsLists, ServerFnError> {
    let mut view = get_friends().await?;
    let served: BTreeMap<Uuid, Friend> = view.friends.iter().map(|f| (f.account_id, f.clone())).collect();
    let mut changed = Vec::new();
    update_ls_if_changed(ls_ctx, |state| changed = pin_keys(state, &mut view.friends));
    let mut lists = to_lists(&view, &changed, account_key.as_ref(), Some(&account), raw_ok);
    for row in lists.friends.iter_mut().filter(|r| r.key_changed) {
        let served = served.get(&row.account_id);
        row.new_key = served.and_then(|f| f.public_key.clone());
        row.new_kem_key = served.and_then(|f| f.kem_public_key.clone());
    }
    Ok(lists)
}

/// A label is a JSON string, so a co-member's participant name never decrypts as a bare key
/// fragment (audit 2026-10-19 L1); labels written before typed wraps are the bare text.
fn write_label(account_key: &[u8; 32], text: &str) -> Result<shared::EncryptedPair, ServerFnError> {
    encrypt_padded_json_within(account_key, &text, shared::MAX_FRIEND_LABEL_LENGTH).map_err(ServerFnError::new)
}

fn read_label(plain: String) -> String {
    serde_json::from_str::<String>(&plain).unwrap_or(plain)
}

/// The address a sent request can be announced to. A request by email is labelled with the address
/// typed; one made from a project carries the participant's name, and its target has an account.
pub fn notify_address(label: &str) -> Option<&str> {
    let label = label.trim();
    let (local, domain) = label.split_once('@')?;
    let printable = !label.chars().any(|c| c.is_whitespace() || c.is_control());
    (printable && !local.is_empty() && !domain.is_empty() && !domain.contains('@')).then_some(label)
}

/// Fluent wraps every placeable in U+2068/U+2069: pasted into another app, they would sit inside the
/// address the friend has to sign up with.
pub fn plain_message(text: &str) -> String {
    text.replace(['\u{2068}', '\u{2069}'], "")
}

/// The response is the same whether or not the address has an account; so is what this returns.
pub async fn add_by_email(email: &str, account_key: &[u8; 32]) -> Result<(), ServerFnError> {
    let email = email.trim().to_string();
    let label = write_label(account_key, &email)?;
    let sent = request_by_email(Json(FriendRequestByEmail { email, label, amk_epoch: crate::auth::account_key::held_amk_epoch() })).await;
    crate::auth::account_key::note(&sent);
    sent
}

pub async fn add_from_project(
    project_id: Uuid,
    user_id: i32,
    participant_name: &str,
    account_key: &[u8; 32],
) -> Result<(), ServerFnError> {
    let label = write_label(account_key, participant_name)?;
    let sent = request_from_project(Json(FriendRequestFromProject { project_id, user_id, label, amk_epoch: crate::auth::account_key::held_amk_epoch() })).await;
    crate::auth::account_key::note(&sent);
    sent
}

pub async fn accept(id: Uuid) -> Result<(), ServerFnError> {
    accept_request(id).await
}

pub async fn decline_or_withdraw(id: Uuid) -> Result<(), ServerFnError> {
    delete_request(id).await
}

/// Drops the pin too, so the friend added again is pinned afresh: the way out once codes compared
/// in person match.
pub async fn remove(ls_ctx: Signal<LocalStorageState>, id: Uuid, account_id: Uuid) -> Result<(), ServerFnError> {
    remove_friend(id).await?;
    update_ls_if_changed(ls_ctx, |state| {
        state.friend_keys.remove(&account_id);
        state.friend_kem_keys.remove(&account_id);
    });
    Ok(())
}

/// This account's private key, opened with the account key. `None` for an account with no keypair
/// yet, when `account_key` does not open it, or when the served `public_key` is not its own: a
/// server that hands this account someone else's public key and that account this one's can make
/// both codes match, so nothing is boxed or opened then (audit 2026-10-19 M2).
/// An untyped wrap is refused once this device has seen a typed one (`raw_ok`, audit L1).
pub fn my_private_key(account: &Account, account_key: &[u8; 32], raw_ok: bool) -> Option<[u8; 32]> {
    let private = unwrap_key(account_key, WrapUse::PrivateKey, account.private_key.as_ref()?, raw_ok)?;
    (account.public_key.as_deref() == Some(public_key_of(&private).as_slice())).then_some(private)
}

pub fn served_key_mismatch(account: &Account, account_key: &[u8; 32], raw_ok: bool) -> bool {
    account
        .private_key
        .as_ref()
        .and_then(|wrapped| unwrap_key(account_key, WrapUse::PrivateKey, wrapped, raw_ok))
        .is_some_and(|private| account.public_key.as_deref() != Some(public_key_of(&private).as_slice()))
}

/// The ML-KEM seed the account key opens, only if it expands to the encapsulation key the server
/// serves for the account.
fn my_kem_seed(account: &Account, account_key: &[u8; 32]) -> Option<[u8; KEM_SEED_LENGTH]> {
    let seed = open_kem_seed(account_key, account.kem_private_key.as_ref()?)?;
    (account.kem_public_key.as_deref() == Some(kem_public_key(&seed).as_slice())).then_some(seed)
}

/// Both halves of the account's keypair as this device holds them, each equal to the half the
/// server serves. Anything that vouches for the account's public keys (the member-key directory
/// entry, docs/plans/key-lifecycle.md §8.5) or opens a box with them derives them from here, never
/// from the served `Account`: a server that hands this device substituted public halves gets no
/// entry under the project key, and so no box sealed to keys it holds (security review 2026-11-02 H1).
#[derive(Clone, PartialEq, Debug)]
pub struct OwnKeys {
    pub x25519: [u8; 32],
    pub kem_seed: [u8; KEM_SEED_LENGTH],
    pub x25519_public: Vec<u8>,
    pub kem_public: Vec<u8>,
}

pub fn own_keys(account: &Account, account_key: &[u8; 32], raw_ok: bool) -> Option<OwnKeys> {
    let x25519 = my_private_key(account, account_key, raw_ok)?;
    let kem_seed = my_kem_seed(account, account_key)?;
    Some(OwnKeys { x25519, kem_seed, x25519_public: public_key_of(&x25519), kem_public: kem_public_key(&kem_seed) })
}

/// The served keypair is not this device's: a half the account key opens differs from the half the
/// server serves. Shown wherever the account is used, not only on the friends page.
pub fn served_keys_mismatch(account: &Account, account_key: &[u8; 32], raw_ok: bool) -> bool {
    let kem_mismatch = account
        .kem_private_key
        .as_ref()
        .and_then(|wrapped| open_kem_seed(account_key, wrapped))
        .is_some_and(|seed| account.kem_public_key.as_deref() != Some(kem_public_key(&seed).as_slice()));
    served_key_mismatch(account, account_key, raw_ok) || kem_mismatch
}

pub fn private_key_is_typed(account: &Account, account_key: &[u8; 32]) -> bool {
    account
        .private_key
        .as_ref()
        .and_then(|wrapped| open_wrap(account_key, WrapUse::PrivateKey, wrapped))
        .is_some_and(|u| u.typed)
}

/// A friend to invite, and the participant created for them when there is one.
#[derive(Debug, Clone, PartialEq)]
pub struct Invitee {
    pub friend: Friend,
    pub user_id: Option<i32>,
}

/// Who sends invitations from this device, and in which box.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Sender {
    pub account_id: Uuid,
    pub private: [u8; 32],
    /// The hybrid box only, never the classical one: the server reports the floor, or this device
    /// already saw it (`LocalStorageState::hybrid_invitations`).
    pub hybrid: bool,
}

pub fn sender(account: &Account, account_key: &[u8; 32], state: &LocalStorageState) -> Option<Sender> {
    let private = my_private_key(account, account_key, state.raw_wraps_ok(account.id))?;
    Some(Sender { account_id: account.id, private, hybrid: account.hybrid_invitations || state.hybrid_invitations })
}

pub fn saw_hybrid_floor(state: &mut LocalStorageState, account: &Account) {
    state.hybrid_invitations |= account.hybrid_invitations;
}

/// What was not sent: `failed` can be sent again; `waiting` friends have no ML-KEM key yet, which
/// they make by opening Counted once, and get no classical box in its place.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Unsent {
    pub failed: Vec<Invitee>,
    pub waiting: Vec<Invitee>,
}

impl Unsent {
    pub fn is_empty(&self) -> bool {
        self.failed.is_empty() && self.waiting.is_empty()
    }
}

#[derive(Debug, PartialEq)]
pub enum Unboxed {
    Failed,
    Waiting,
}

/// The project key for one friend: the classical box before the hybrid floor, the hybrid box sealed
/// to both pinned keys and typed with project, sender and recipient from it.
pub fn box_for(sender: &Sender, project_id: Uuid, friend: &Friend, project_key: &[u8; 32]) -> Result<(Option<EncryptedPair>, Option<HybridBox>), Unboxed> {
    let public = friend.public_key.as_deref().ok_or(Unboxed::Failed)?;
    if !sender.hybrid {
        return box_project_key(&sender.private, public, project_key).map(|k| (Some(k), None)).ok_or(Unboxed::Failed);
    }
    let kem = friend.kem_public_key.as_deref().ok_or(Unboxed::Waiting)?;
    let typed = InviteKey { use_: "invite".into(), project: project_id, from: sender.account_id, to: friend.account_id, key: URL_SAFE_NO_PAD.encode(project_key) };
    let plaintext = serde_json::to_vec(&typed).map_err(|_| Unboxed::Failed)?;
    let sealed = seal_auth(&sender.private, public, kem, INVITE_CONTEXT, &plaintext).ok_or(Unboxed::Failed)?;
    Ok((None, Some(sealed)))
}

/// One invitation per friend, each carrying the key boxed to that friend, all in one request.
/// Returns those that were not sent - no private key on this device, no key on theirs, a refusal,
/// or a request that failed as a whole.
pub async fn invite_each(
    ls_ctx: Signal<LocalStorageState>,
    project_id: Uuid,
    invitees: &[Invitee],
    sender: Option<Sender>,
    project_key: &[u8; 32],
) -> Unsent {
    let Some(sender) = sender else {
        return Unsent { failed: invitees.to_vec(), waiting: Vec::new() };
    };
    if sender.hybrid {
        update_ls_if_changed(ls_ctx, |state| state.hybrid_invitations = true);
    }
    let (mut boxed, mut unsent) = (Vec::new(), Unsent::default());
    for invitee in invitees {
        let (key, hybrid) = match box_for(&sender, project_id, &invitee.friend, project_key) {
            Ok(forms) => forms,
            Err(Unboxed::Failed) => {
                unsent.failed.push(invitee.clone());
                continue;
            }
            Err(Unboxed::Waiting) => {
                unsent.waiting.push(invitee.clone());
                continue;
            }
        };
        let invitation = CreatableProjectInvitation { to_account_id: invitee.friend.account_id, key, hybrid, user_id: invitee.user_id };
        // One by one, a friend named twice kept the later invitation; the batch refuses a repeat.
        boxed.retain(|b: &CreatableProjectInvitation| b.to_account_id != invitation.to_account_id);
        boxed.push(invitation);
    }
    if boxed.is_empty() {
        return unsent;
    }
    crate::common::project_token::scope(project_id);
    let refused: Vec<Uuid> = match invite_batch(project_id, Json(boxed)).await {
        Ok(refused) => refused,
        Err(_) => invitees.iter().map(|i| i.friend.account_id).collect(),
    };
    let failed = invitees.iter().filter(|i| unsent.failed.contains(i) || (refused.contains(&i.friend.account_id) && !unsent.waiting.contains(i)));
    Unsent { failed: failed.cloned().collect(), waiting: unsent.waiting }
}

pub fn invitee_emails(invitees: &[Invitee]) -> String {
    invitees.iter().map(|i| i.friend.email.as_str()).collect::<Vec<_>>().join(", ")
}

/// An invitation this device managed to open: the key is real, so the project name can be shown
/// before anything is accepted.
#[derive(Debug, Clone, PartialEq)]
pub struct OpenedInvitation {
    pub invitation: ProjectInvitation,
    /// `None` when the box did not open - the row can only be declined.
    pub project_key: Option<[u8; 32]>,
    pub project_name: Option<String>,
}

/// What opens invitations on this device: the X25519 key, and the ML-KEM seed once the account has
/// one. Both are the halves the account key opens, each equal to the one the server serves.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Opener {
    pub account_id: Uuid,
    pub x25519: [u8; 32],
    pub kem_seed: Option<[u8; KEM_SEED_LENGTH]>,
}

pub fn opener(account: &Account, account_key: &[u8; 32], raw_ok: bool) -> Option<Opener> {
    let x25519 = my_private_key(account, account_key, raw_ok)?;
    Some(Opener { account_id: account.id, x25519, kem_seed: my_kem_seed(account, account_key) })
}

/// The key and the email of the friend whose public key opens the box. Only a friend's key, the one
/// whose code the user can compare: the sender key served with the invitation is the server's word
/// alone, and a box under a key of its own would otherwise open. A hybrid box opens only if it
/// names this project, that friend as sender and this account as recipient.
pub fn open_invitation(inv: &ProjectInvitation, opener: &Opener, friends: &[Friend]) -> Option<([u8; 32], String)> {
    let sender = friends.iter().find(|f| f.public_key.as_deref() == Some(inv.from_public_key.as_slice()))?;
    let key = match (&inv.key, &inv.hybrid) {
        (Some(pair), None) => unbox_project_key(&opener.x25519, &inv.from_public_key, pair)?,
        (None, Some(sealed)) => {
            let plaintext = open_auth(&opener.x25519, opener.kem_seed.as_ref()?, &inv.from_public_key, INVITE_CONTEXT, sealed)?;
            let typed: InviteKey = serde_json::from_slice(&plaintext).ok()?;
            let fits = typed.use_ == "invite" && typed.project == inv.project_id && typed.from == sender.account_id && typed.to == opener.account_id;
            if !fits {
                return None;
            }
            URL_SAFE_NO_PAD.decode(typed.key).ok()?.try_into().ok()?
        }
        _ => return None,
    };
    Some((key, sender.email.clone()))
}

pub async fn load_invitations(
    ls_ctx: Signal<LocalStorageState>,
    opener: Option<Opener>,
) -> Result<Vec<OpenedInvitation>, ServerFnError> {
    let invitations = get_invitations().await?;
    let friends = match opener.is_some() && !invitations.is_empty() {
        true => get_pinned_friends(ls_ctx).await?.0.friends,
        false => Vec::new(),
    };
    let boxed: Vec<(ProjectInvitation, Option<[u8; 32]>)> = invitations
        .into_iter()
        .map(|mut invitation| {
            let key = opener.and_then(|o| open_invitation(&invitation, &o, &friends)).map(|(key, email)| {
                invitation.from_email = email;
                key
            });
            (invitation, key)
        })
        .collect();
    if boxed.iter().any(|(i, k)| i.hybrid.is_some() && k.is_some()) {
        update_ls_if_changed(ls_ctx, |state| state.hybrid_invitations = true);
    }
    let mut ids: Vec<Uuid> = boxed.iter().filter(|(_, k)| k.is_some()).map(|(i, _)| i.project_id).collect();
    ids.sort();
    ids.dedup();
    let projects = match ids.is_empty() {
        true => Vec::new(),
        false => {
            let tokens = crate::common::project_token::tokens(&ids);
            get_projects_by_ids(Json(BatchProject { ids, tokens })).await.unwrap_or_default()
        }
    };
    Ok(boxed.into_iter().map(|(invitation, key)| opened_invitation(invitation, key, &projects)).collect())
}

/// A box that opens is not a key that decrypts: one holding a wrong key used to show Accept with a
/// blank name. Unreachable projects keep the key - accepting still proves it before it can replace
/// one already held.
fn opened_invitation(invitation: ProjectInvitation, boxed: Option<[u8; 32]>, projects: &[ProjectDto]) -> OpenedInvitation {
    let (project_key, project_name) = match boxed {
        Some(k) => match projects.iter().find(|p| p.id == invitation.project_id) {
            Some(p) => match decrypt_project(&k, p) {
                Ok(d) => (Some(k), Some(d.name)),
                Err(_) => (None, None),
            },
            None => (Some(k), None),
        },
        None => (None, None),
    };
    OpenedInvitation { invitation, project_key, project_name }
}

/// Accepting is a local act: the key goes into the store exactly as a pasted share link's would -
/// through [`adopt_project_key`], so it cannot replace a held key it does not prove - and
/// `ExpensesPage` takes it from there (membership, escrow, the identity picker). The key is written
/// before the row is deleted - a delete that fails leaves the invitation visible, and accepting it
/// again is harmless.
pub async fn accept_invitation(
    ls_ctx: Signal<LocalStorageState>,
    invitation: &ProjectInvitation,
    project_key: &[u8; 32],
) -> Result<(), ServerFnError> {
    adopt_project_key(ls_ctx, invitation.project_id, *project_key).await;
    delete_invitation(invitation.id).await
}

pub async fn decline_invitation(id: Uuid) -> Result<(), ServerFnError> {
    delete_invitation(id).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypto::{encrypt, generate_key, generate_keypair};
    use chrono::NaiveDateTime;
    use shared::{EncryptedPair, OutgoingFriendRequest};

    fn friend(public_key: Option<Vec<u8>>) -> Friend {
        Friend { id: Uuid::new_v4(), account_id: Uuid::new_v4(), email: "bob@x.io".into(), public_key, kem_public_key: None }
    }

    #[test]
    fn only_an_address_label_can_be_notified() {
        assert_eq!(notify_address(" tom.bernard@gmail.com "), Some("tom.bernard@gmail.com"));
        for label in ["Tom", "", "@x.io", "tom@", "a@b@c", "tom bernard@x.io", "tom@x.io\u{0}"] {
            assert_eq!(notify_address(label), None, "{label:?}");
        }
    }

    #[test]
    fn the_message_carries_no_isolation_marks() {
        assert_eq!(plain_message("sur https://counted.fr avec \u{2068}tom@x.io\u{2069} :"), "sur https://counted.fr avec tom@x.io :");
    }

    /// The message goes through the user's messaging app: never a project link or key, in any
    /// language, and always the site and the address to sign up with.
    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn every_notify_message_names_the_site_and_the_address_and_no_project() {
        for (code, _) in crate::i18n::SUPPORTED {
            let text = crate::i18n::locale_text(code);
            let line = text
                .lines()
                .find(|l| l.starts_with("friends-notify-message ="))
                .unwrap_or_else(|| panic!("{code}: friends-notify-message missing"));
            assert!(line.contains("https://counted.fr "), "{code}: {line}");
            assert!(line.contains("{ $email }"), "{code}: {line}");
            assert!(!line.contains('#') && !line.contains("/projects"), "{code}: {line}");
        }
    }

    #[test]
    fn a_friend_without_a_key_has_no_fingerprint() {
        assert_eq!(friend_row(&friend(None), &[]).fingerprint, None);
        let (public, _) = generate_keypair();
        assert_eq!(friend_row(&friend(Some(public.clone())), &[]).fingerprint, Some(key_fingerprint(&public)));
    }

    #[test]
    fn outgoing_labels_decrypt_with_the_account_key_and_blank_without() {
        let account_key = generate_key();
        let view = FriendsView {
            friends: vec![],
            incoming: vec![],
            outgoing: vec![OutgoingFriendRequest {
                id: Uuid::new_v4(),
                label: encrypt(&account_key, "carol@x.io").unwrap(),
                created_at: NaiveDateTime::default(),
            }],
        };
        assert_eq!(to_lists(&view, &[], Some(&account_key), None, true).outgoing[0].label, "carol@x.io");
        assert_eq!(to_lists(&view, &[], None, None, true).outgoing[0].label, "");
        assert_eq!(to_lists(&view, &[], Some(&generate_key()), None, true).outgoing[0].label, "");
    }

    #[test]
    fn a_label_reads_back_in_either_form() {
        let account_key = generate_key();
        let typed = write_label(&account_key, "Carol \"C\"").unwrap();
        let bare = encrypt(&account_key, "Carol \"C\"").unwrap();
        for label in [typed, bare] {
            assert_eq!(read_label(decrypt(&account_key, &label).unwrap()), "Carol \"C\"");
        }
    }

    /// Audit 2026-10-19 L1: once this device has seen a typed wrap for the account, a server that
    /// serves an untyped one - a planted label, another row - gets nothing boxed or opened.
    #[test]
    fn a_device_that_saw_a_typed_wrap_refuses_an_untyped_private_key() {
        let account_key = generate_key();
        let (public, private) = generate_keypair();
        let account = account_with(&account_key, public, &private);

        assert_eq!(my_private_key(&account, &account_key, true), Some(private));
        assert_eq!(my_private_key(&account, &account_key, false), None);
        assert!(!private_key_is_typed(&account, &account_key));

        let typed = Account {
            private_key: crate::crypto::wrap_key(&account_key, WrapUse::PrivateKey, &private),
            ..account
        };
        assert_eq!(my_private_key(&typed, &account_key, false), Some(private));
        assert!(private_key_is_typed(&typed, &account_key));
    }

    /// Security review 2026-11-02 H1: the keys this device vouches for are the ones its account key
    /// opens, and only while the server serves those same halves. A substituted half, X25519 or
    /// ML-KEM, yields no keys at all.
    #[test]
    fn own_keys_are_the_opened_halves_and_only_while_the_served_ones_match() {
        let account_key = generate_key();
        let (public, private) = generate_keypair();
        let seed = crate::hybrid::generate_kem_seed();
        let mut account = Account {
            private_key: crate::crypto::wrap_key(&account_key, WrapUse::PrivateKey, &private),
            kem_public_key: Some(crate::hybrid::kem_public_key(&seed)),
            kem_private_key: crate::crypto::wrap_kem_seed(&account_key, &seed),
            ..account_with(&account_key, public.clone(), &private)
        };
        let own = own_keys(&account, &account_key, false).unwrap();
        assert_eq!((own.x25519, own.kem_seed), (private, seed));
        assert_eq!((own.x25519_public.as_slice(), own.kem_public.as_slice()), (public.as_slice(), account.kem_public_key.as_deref().unwrap()));
        assert!(!served_keys_mismatch(&account, &account_key, false));

        let (other_public, _) = generate_keypair();
        account.public_key = Some(other_public);
        assert_eq!(own_keys(&account, &account_key, false), None, "a substituted X25519 half");
        assert!(served_keys_mismatch(&account, &account_key, false));

        account.public_key = Some(public);
        account.kem_public_key = Some(crate::hybrid::kem_public_key(&crate::hybrid::generate_kem_seed()));
        assert_eq!(own_keys(&account, &account_key, false), None, "a substituted ML-KEM half");
        assert!(served_keys_mismatch(&account, &account_key, false));

        account.kem_public_key = None;
        account.kem_private_key = None;
        assert_eq!(own_keys(&account, &account_key, false), None, "no ML-KEM key yet");
        assert!(!served_keys_mismatch(&account, &account_key, false), "nothing served, nothing to mismatch");
    }

    fn account_with(account_key: &[u8; 32], public_key: Vec<u8>, private: &[u8; 32]) -> Account {
        Account {
            id: Uuid::nil(),
            email: "me@x.io".into(),
            display_name: EncryptedPair { ct: String::new(), iv: String::new() },
            created_at: NaiveDateTime::default(),
            email_verified: true,
            kdf_salt: vec![0; 16],
            kdf_version: 1,
            preferences: None,
            payment_methods: None,
            public_key: Some(public_key),
            private_key: crate::crypto::wrap_key_untyped(account_key, WrapUse::PrivateKey, private),
            typed_wraps: false,
            ..Default::default()
        }
    }

    /// What makes the check possible at all: Alice's row for Bob and Bob's own code must be the
    /// same string, or two friends reading their codes to each other can never match.
    #[test]
    fn my_code_is_what_my_friends_see_next_to_my_email() {
        let (bob_pub, bob_priv) = generate_keypair();
        let bobs_key = generate_key();
        let bob = account_with(&bobs_key, bob_pub.clone(), &bob_priv);
        let alices_view = FriendsView { friends: vec![friend(Some(bob_pub))], ..Default::default() };

        let on_alices_page = to_lists(&alices_view, &[], None, None, true).friends[0].fingerprint.clone();
        let on_bobs_page = to_lists(&FriendsView::default(), &[], Some(&bobs_key), Some(&bob), true).my_fingerprint;

        assert!(on_bobs_page.is_some());
        assert_eq!(on_alices_page, on_bobs_page);
    }

    /// Audit 2026-10-19 M2: a server serving Bob a key S as his own made Bob's code S's code, which
    /// it could also hand Alice in place of Bob's.
    #[test]
    fn a_served_public_key_that_is_not_mine_is_flagged_and_nothing_is_boxed() {
        let (_, bob_priv) = generate_keypair();
        let (server_pub, _) = generate_keypair();
        let bobs_key = generate_key();
        let bob = account_with(&bobs_key, server_pub.clone(), &bob_priv);

        let lists = to_lists(&FriendsView::default(), &[], Some(&bobs_key), Some(&bob), true);
        assert!(lists.key_mismatch);
        assert_ne!(lists.my_fingerprint, Some(key_fingerprint(&server_pub)));
        assert_eq!(lists.my_fingerprint, None);
        assert_eq!(my_private_key(&bob, &bobs_key, true), None);
    }

    #[test]
    fn an_account_without_a_keypair_has_no_code_of_its_own() {
        assert_eq!(to_lists(&FriendsView::default(), &[], None, None, true).my_fingerprint, None);
    }

    fn boxed_invitation(sender_pub: Vec<u8>, sender_priv: &[u8; 32], to: &[u8], project_key: &[u8; 32]) -> ProjectInvitation {
        ProjectInvitation {
            id: Uuid::new_v4(),
            project_id: Uuid::new_v4(),
            from_email: "served@x.io".into(),
            from_public_key: sender_pub,
            key: Some(box_project_key(sender_priv, to, project_key).unwrap()),
            hybrid: None,
            created_at: NaiveDateTime::default(),
            user_id: None,
        }
    }

    #[test]
    fn an_invitation_opens_with_the_senders_public_key_only() {
        let (alice_pub, alice_priv) = generate_keypair();
        let (bob_pub, bob_priv) = generate_keypair();
        let project_key = generate_key();
        let alice = Friend { email: "alice@x.io".into(), ..friend(Some(alice_pub.clone())) };
        let inv = boxed_invitation(alice_pub, &alice_priv, &bob_pub, &project_key);
        let friends = [alice];

        assert_eq!(open_invitation(&inv, &opener_of(bob_priv), &friends), Some((project_key, "alice@x.io".into())));
        assert_eq!(open_invitation(&inv, &opener_of(generate_keypair().1), &friends), None);
        let swapped = ProjectInvitation { from_public_key: generate_keypair().0, ..inv };
        assert_eq!(open_invitation(&swapped, &opener_of(bob_priv), &friends), None);
    }

    /// The server boxes a key of its own to Bob and serves its public key as the sender's: it is
    /// no friend's key, so the box stays shut.
    #[test]
    fn an_invitation_from_a_key_no_friend_holds_never_opens() {
        let (server_pub, server_priv) = generate_keypair();
        let (bob_pub, bob_priv) = generate_keypair();
        let inv = boxed_invitation(server_pub, &server_priv, &bob_pub, &generate_key());

        assert_eq!(open_invitation(&inv, &opener_of(bob_priv), &[friend(Some(generate_keypair().0)), friend(None)]), None);
        assert_eq!(open_invitation(&inv, &opener_of(bob_priv), &[]), None);
    }

    #[test]
    fn a_friends_key_is_pinned_once_served_and_kept() {
        let mut pins = LocalStorageState::default();
        let mut later = friend(None);
        assert!(pin_keys(&mut pins, std::slice::from_mut(&mut later)).is_empty());
        assert!(pins.friend_keys.is_empty(), "no key, nothing to pin");

        let (public, _) = generate_keypair();
        later.public_key = Some(public.clone());
        let mut friends = [later.clone()];
        assert!(pin_keys(&mut pins, &mut friends).is_empty());
        assert!(pin_keys(&mut pins, &mut friends).is_empty());
        assert_eq!(friends[0].public_key, Some(public));
    }

    /// The server swaps Alice's key for one of its own after Bob saw hers: Bob's device takes the
    /// key off, so nothing is boxed to the server's key and its box from "Alice" stays shut.
    #[test]
    fn a_changed_key_is_taken_off_the_friend_and_flagged() {
        let (alice_pub, _) = generate_keypair();
        let (server_pub, server_priv) = generate_keypair();
        let (bob_pub, bob_priv) = generate_keypair();
        let alice = Friend { email: "alice@x.io".into(), ..friend(Some(alice_pub)) };
        let mut pins = LocalStorageState::default();
        pin_keys(&mut pins, &mut [alice.clone()]);

        let mut served = [Friend { public_key: Some(server_pub.clone()), ..alice.clone() }];
        assert_eq!(pin_keys(&mut pins, &mut served), vec![alice.account_id]);
        assert_eq!(served[0].public_key, None);
        let inv = boxed_invitation(server_pub, &server_priv, &bob_pub, &generate_key());
        assert_eq!(open_invitation(&inv, &opener_of(bob_priv), &served), None);

        let view = FriendsView { friends: served.to_vec(), ..Default::default() };
        let row = &to_lists(&view, &[alice.account_id], None, None, true).friends[0];
        assert!(row.key_changed);
        assert_eq!(row.fingerprint, None);
    }

    #[test]
    fn an_unpinned_friend_is_pinned_afresh() {
        let (old, _) = generate_keypair();
        let (new, _) = generate_keypair();
        let alice = friend(Some(old));
        let mut pins = LocalStorageState::default();
        pin_keys(&mut pins, &mut [alice.clone()]);
        pins.friend_keys.remove(&alice.account_id);

        let mut readded = [Friend { public_key: Some(new.clone()), ..alice }];
        assert!(pin_keys(&mut pins, &mut readded).is_empty());
        assert_eq!(readded[0].public_key, Some(new));
    }

    /// Plan §7: once the codes match, the new key is pinned and the friend is trusted again.
    #[test]
    fn a_new_key_trusted_after_comparing_codes_is_kept() {
        let mut pins = LocalStorageState::default();
        let alice = friend(Some(vec![1; 32]));
        pin_keys(&mut pins, &mut [alice.clone()]);
        let mut rotated = [Friend { public_key: Some(vec![2; 32]), ..alice.clone() }];
        assert_eq!(pin_keys(&mut pins, &mut rotated.clone()), vec![alice.account_id]);

        trust_key(&mut pins, alice.account_id, &[2; 32], None);
        assert!(pin_keys(&mut pins, &mut rotated).is_empty());
        assert_eq!(rotated[0].public_key, Some(vec![2; 32]));
    }

    fn invitation(project_id: Uuid) -> ProjectInvitation {
        ProjectInvitation {
            id: Uuid::new_v4(),
            project_id,
            from_email: "alice@x.io".into(),
            from_public_key: vec![],
            key: Some(EncryptedPair { ct: String::new(), iv: String::new() }),
            hybrid: None,
            created_at: NaiveDateTime::default(),
            user_id: None,
        }
    }

    fn project(id: Uuid, key: &[u8; 32], name: &str) -> ProjectDto {
        let payload = shared::ProjectPayload {
            name: name.into(),
            currency: "EUR".into(),
            description: None,
            status: None,
            extra: Default::default(),
        };
        ProjectDto {
            id,
            payload: crate::crypto::encrypt_json(key, &payload).unwrap(),
            status: shared::ProjectStatus::Ongoing,
            created_at: NaiveDateTime::default(),
            has_admin: false,
            key_lifecycle: false,
            key_epoch: 0,
            has_creator_proof: false,
            read_only: false,
        }
    }

    #[test]
    fn each_invitation_takes_its_own_project_from_the_batch() {
        let (a, b, gone) = (Uuid::new_v4(), Uuid::new_v4(), Uuid::new_v4());
        let (key_a, key_b) = (generate_key(), generate_key());
        let projects = vec![project(b, &key_b, "Ski"), project(a, &key_a, "Rome")];

        let opened = opened_invitation(invitation(a), Some(key_a), &projects);
        assert_eq!((opened.project_key, opened.project_name.as_deref()), (Some(key_a), Some("Rome")));

        let wrong = opened_invitation(invitation(a), Some(key_b), &projects);
        assert_eq!((wrong.project_key, wrong.project_name), (None, None));

        let missing = opened_invitation(invitation(gone), Some(key_a), &projects);
        assert_eq!((missing.project_key, missing.project_name), (Some(key_a), None));

        let unopened = opened_invitation(invitation(b), None, &projects);
        assert_eq!((unopened.project_key, unopened.project_name), (None, None));
    }

    #[test]
    fn the_private_key_needs_both_the_blob_and_the_account_key() {
        let account_key = generate_key();
        let (_, private) = generate_keypair();
        let mut account = Account {
            id: Uuid::nil(),
            email: "me@x.io".into(),
            display_name: EncryptedPair { ct: String::new(), iv: String::new() },
            created_at: NaiveDateTime::default(),
            email_verified: true,
            kdf_salt: vec![0; 16],
            kdf_version: 1,
            preferences: None,
            payment_methods: None,
            public_key: None,
            private_key: None,
            typed_wraps: false,
            ..Default::default()
        };
        assert_eq!(my_private_key(&account, &account_key, true), None);
        account.private_key = crate::crypto::wrap_key_untyped(&account_key, WrapUse::PrivateKey, &private);
        account.public_key = Some(public_key_of(&private));
        assert_eq!(my_private_key(&account, &account_key, true), Some(private));
        assert_eq!(my_private_key(&account, &generate_key(), true), None);
    }

    fn opener_of(x25519: [u8; 32]) -> Opener {
        Opener { account_id: Uuid::nil(), x25519, kem_seed: None }
    }

    /// An account as the hybrid tests need it: both private halves, and the friend row others see.
    struct Member {
        opener: Opener,
        friend: Friend,
    }

    fn member(email: &str) -> Member {
        let (public, x25519) = generate_keypair();
        let seed = crate::hybrid::generate_kem_seed();
        let account_id = Uuid::new_v4();
        Member {
            opener: Opener { account_id, x25519, kem_seed: Some(seed) },
            friend: Friend { id: Uuid::new_v4(), account_id, email: email.into(), public_key: Some(public), kem_public_key: Some(crate::hybrid::kem_public_key(&seed)) },
        }
    }

    fn hybrid_sender(m: &Member) -> Sender {
        Sender { account_id: m.opener.account_id, private: m.opener.x25519, hybrid: true }
    }

    fn served(from: &Member, project_id: Uuid, forms: (Option<EncryptedPair>, Option<HybridBox>)) -> ProjectInvitation {
        ProjectInvitation {
            id: Uuid::new_v4(),
            project_id,
            from_email: "served@x.io".into(),
            from_public_key: from.friend.public_key.clone().unwrap(),
            key: forms.0,
            hybrid: forms.1,
            created_at: NaiveDateTime::default(),
            user_id: None,
        }
    }

    /// Security-fixes §5: a hybrid invitation opens for its recipient with the sender's pinned key,
    /// and carries the project key itself.
    #[test]
    fn a_hybrid_invitation_opens_for_its_recipient() {
        let (alice, bob) = (member("alice@x.io"), member("bob@x.io"));
        let (project_id, project_key) = (Uuid::new_v4(), generate_key());
        let forms = box_for(&hybrid_sender(&alice), project_id, &bob.friend, &project_key).unwrap();
        assert!(forms.0.is_none() && forms.1.is_some(), "the hybrid form only");

        let inv = served(&alice, project_id, forms);
        assert_eq!(open_invitation(&inv, &bob.opener, std::slice::from_ref(&alice.friend)), Some((project_key, "alice@x.io".into())));
        assert_eq!(open_invitation(&inv, &Opener { kem_seed: None, ..bob.opener }, &[alice.friend]), None, "no ML-KEM seed");
    }

    /// The box names its project, sender and recipient: re-served under another project, to another
    /// account or from another friend, it is refused.
    #[test]
    fn a_hybrid_invitation_for_another_project_sender_or_recipient_is_refused() {
        let (alice, bob) = (member("alice@x.io"), member("bob@x.io"));
        let project_id = Uuid::new_v4();
        let forms = box_for(&hybrid_sender(&alice), project_id, &bob.friend, &generate_key()).unwrap();

        let moved = served(&alice, Uuid::new_v4(), forms.clone());
        assert_eq!(open_invitation(&moved, &bob.opener, std::slice::from_ref(&alice.friend)), None, "another project");

        let to_another = Opener { account_id: Uuid::new_v4(), ..bob.opener };
        let inv = served(&alice, project_id, forms);
        assert_eq!(open_invitation(&inv, &to_another, std::slice::from_ref(&alice.friend)), None, "another recipient");

        let renamed = Friend { account_id: Uuid::new_v4(), ..alice.friend.clone() };
        assert_eq!(open_invitation(&inv, &bob.opener, &[renamed]), None, "the key under another account");
    }

    /// Decision b: from the floor, a friend without an ML-KEM key gets no invitation at all, never a
    /// classical box; before it, everyone gets the classical box.
    #[test]
    fn the_floor_decides_the_form_and_a_friend_without_a_kem_key_waits() {
        let (alice, bob) = (member("alice@x.io"), member("bob@x.io"));
        let (project_id, project_key) = (Uuid::new_v4(), generate_key());
        let classical = Sender { hybrid: false, ..hybrid_sender(&alice) };
        let forms = box_for(&classical, project_id, &bob.friend, &project_key).unwrap();
        assert!(forms.0.is_some() && forms.1.is_none(), "the classical form only, before the floor");

        let no_kem = Friend { kem_public_key: None, ..bob.friend.clone() };
        assert_eq!(box_for(&hybrid_sender(&alice), project_id, &no_kem, &project_key), Err(Unboxed::Waiting));
        let no_key = Friend { public_key: None, ..bob.friend };
        assert_eq!(box_for(&hybrid_sender(&alice), project_id, &no_key, &project_key), Err(Unboxed::Failed));
    }

    /// Plan §5 c: the ML-KEM key is pinned like the X25519 one. A changed one is taken off and
    /// flagged, so nothing is boxed to the substitute.
    #[test]
    fn a_changed_kem_key_is_taken_off_and_nothing_is_boxed() {
        let (alice, bob) = (member("alice@x.io"), member("bob@x.io"));
        let mut state = LocalStorageState::default();
        assert!(pin_keys(&mut state, &mut [bob.friend.clone()]).is_empty());
        assert!(state.friend_kem_keys.contains_key(&bob.friend.account_id));

        let substitute = crate::hybrid::kem_public_key(&crate::hybrid::generate_kem_seed());
        let mut served = [Friend { kem_public_key: Some(substitute.clone()), ..bob.friend.clone() }];
        assert_eq!(pin_keys(&mut state, &mut served), vec![bob.friend.account_id]);
        assert_eq!(served[0].kem_public_key, None);
        assert!(box_for(&hybrid_sender(&alice), Uuid::new_v4(), &served[0], &generate_key()).is_err());

        trust_key(&mut state, bob.friend.account_id, bob.friend.public_key.as_deref().unwrap(), Some(&substitute));
        let mut trusted = [Friend { kem_public_key: Some(substitute.clone()), ..bob.friend.clone() }];
        assert!(pin_keys(&mut state, &mut trusted).is_empty(), "pinned once the codes were compared");
    }

    /// A device that saw the floor never writes a classical box again, whatever the server says.
    #[test]
    fn a_device_that_saw_the_hybrid_floor_stays_hybrid() {
        let account_key = generate_key();
        let (public, private) = generate_keypair();
        let account = Account { hybrid_invitations: false, ..account_with(&account_key, public, &private) };
        let mut state = LocalStorageState::default();
        assert!(!sender(&account, &account_key, &state).unwrap().hybrid);

        state.hybrid_invitations = true;
        assert!(sender(&account, &account_key, &state).unwrap().hybrid);
        let floor = Account { hybrid_invitations: true, ..account };
        assert!(sender(&floor, &account_key, &LocalStorageState::default()).unwrap().hybrid);
    }

    #[test]
    fn seeing_the_floor_alone_pins_the_device() {
        let account_key = generate_key();
        let (public, private) = generate_keypair();
        let floor = Account { hybrid_invitations: true, ..account_with(&account_key, public, &private) };
        let mut state = LocalStorageState::default();
        saw_hybrid_floor(&mut state, &Account { hybrid_invitations: false, ..floor.clone() });
        assert!(!state.hybrid_invitations);

        saw_hybrid_floor(&mut state, &floor);
        let turned_back = Account { hybrid_invitations: false, ..floor };
        assert!(sender(&turned_back, &account_key, &state).unwrap().hybrid);
    }

    #[test]
    fn an_invitation_with_both_forms_or_neither_never_opens() {
        let (alice, bob) = (member("alice@x.io"), member("bob@x.io"));
        let project_id = Uuid::new_v4();
        let classical = box_for(&Sender { hybrid: false, ..hybrid_sender(&alice) }, project_id, &bob.friend, &generate_key()).unwrap().0;
        let hybrid = box_for(&hybrid_sender(&alice), project_id, &bob.friend, &generate_key()).unwrap().1;

        assert_eq!(open_invitation(&served(&alice, project_id, (classical, hybrid)), &bob.opener, std::slice::from_ref(&alice.friend)), None);
        assert_eq!(open_invitation(&served(&alice, project_id, (None, None)), &bob.opener, &[alice.friend]), None);
    }
}
