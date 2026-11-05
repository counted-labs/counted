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
use crate::hybrid::{kem_public_key, KEM_SEED_LENGTH};
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use std::collections::BTreeMap;
use crate::decrypted::decrypt_project;

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
    }
}

/// After the two friends read the new code to each other: the served key becomes the pin. A
/// device cannot tell a rotation from a substitution, so only a person comparing codes can.
pub fn trust_key(pins: &mut BTreeMap<Uuid, String>, account_id: Uuid, key: &[u8]) {
    pins.insert(account_id, URL_SAFE_NO_PAD.encode(key));
}

/// Pins each friend's key the first time it is served, and takes the key off a friend served one
/// that differs from the pin, returning those friends. A keypair is written once and only a
/// verified account can be a friend, so a changed key is the server's substitution: without its
/// key, the friend is neither boxed to nor opened from.
pub fn pin_keys(pins: &mut BTreeMap<Uuid, String>, friends: &mut [Friend]) -> Vec<Uuid> {
    let mut changed = Vec::new();
    for friend in friends {
        let Some(served) = friend.public_key.as_deref().map(|k| URL_SAFE_NO_PAD.encode(k)) else { continue };
        match pins.get(&friend.account_id) {
            Some(pinned) if *pinned != served => {
                friend.public_key = None;
                changed.push(friend.account_id);
            }
            Some(_) => {}
            None => {
                pins.insert(friend.account_id, served);
            }
        }
    }
    changed
}

/// The only way the friends list is read: every key checked against this device's pins.
pub async fn get_pinned_friends(ls_ctx: Signal<LocalStorageState>) -> Result<(FriendsView, Vec<Uuid>), ServerFnError> {
    let mut view = get_friends().await?;
    let mut changed = Vec::new();
    update_ls_if_changed(ls_ctx, |state| changed = pin_keys(&mut state.friend_keys, &mut view.friends));
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
    let served: BTreeMap<Uuid, Vec<u8>> = view.friends.iter().filter_map(|f| Some((f.account_id, f.public_key.clone()?))).collect();
    let mut changed = Vec::new();
    update_ls_if_changed(ls_ctx, |state| changed = pin_keys(&mut state.friend_keys, &mut view.friends));
    let mut lists = to_lists(&view, &changed, account_key.as_ref(), Some(&account), raw_ok);
    for row in lists.friends.iter_mut().filter(|r| r.key_changed) {
        row.new_key = served.get(&row.account_id).cloned();
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

/// One invitation per friend, each carrying the key boxed to that friend, all in one request.
/// Returns those that were not sent - no private key on this device, no public key on theirs, a
/// refusal, or a request that failed as a whole.
pub async fn invite_each(
    project_id: Uuid,
    invitees: &[Invitee],
    my_private: Option<[u8; 32]>,
    project_key: &[u8; 32],
) -> Vec<Invitee> {
    let (mut boxed, mut failed) = (Vec::new(), Vec::new());
    for invitee in invitees {
        let key = my_private.as_ref().zip(invitee.friend.public_key.as_deref()).and_then(
            |(private, public)| box_project_key(private, public, project_key),
        );
        let Some(key) = key else {
            failed.push(invitee.clone());
            continue;
        };
        let invitation = CreatableProjectInvitation { to_account_id: invitee.friend.account_id, key, user_id: invitee.user_id };
        // One by one, a friend named twice kept the later invitation; the batch refuses a repeat.
        boxed.retain(|b: &CreatableProjectInvitation| b.to_account_id != invitation.to_account_id);
        boxed.push(invitation);
    }
    if boxed.is_empty() {
        return failed;
    }
    crate::common::project_token::scope(project_id);
    let refused: Vec<Uuid> = match invite_batch(project_id, Json(boxed)).await {
        Ok(refused) => refused,
        Err(_) => invitees.iter().map(|i| i.friend.account_id).collect(),
    };
    invitees.iter().filter(|i| refused.contains(&i.friend.account_id) || failed.contains(i)).cloned().collect()
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

/// The key and the email of the friend whose public key opens the box. Only a friend's key, the one
/// whose code the user can compare: the sender key served with the invitation is the server's word
/// alone, and a box under a key of its own would otherwise open.
pub fn open_invitation(inv: &ProjectInvitation, my_private: &[u8; 32], friends: &[Friend]) -> Option<([u8; 32], String)> {
    let sender = friends.iter().find(|f| f.public_key.as_deref() == Some(inv.from_public_key.as_slice()))?;
    let key = unbox_project_key(my_private, &inv.from_public_key, &inv.key)?;
    Some((key, sender.email.clone()))
}

pub async fn load_invitations(
    ls_ctx: Signal<LocalStorageState>,
    my_private: Option<[u8; 32]>,
) -> Result<Vec<OpenedInvitation>, ServerFnError> {
    let invitations = get_invitations().await?;
    let friends = match my_private.is_some() && !invitations.is_empty() {
        true => get_pinned_friends(ls_ctx).await?.0.friends,
        false => Vec::new(),
    };
    let boxed: Vec<(ProjectInvitation, Option<[u8; 32]>)> = invitations
        .into_iter()
        .map(|mut invitation| {
            let key = my_private.and_then(|p| open_invitation(&invitation, &p, &friends)).map(|(key, email)| {
                invitation.from_email = email;
                key
            });
            (invitation, key)
        })
        .collect();
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
        Friend { id: Uuid::new_v4(), account_id: Uuid::new_v4(), email: "bob@x.io".into(), public_key }
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
            key: box_project_key(sender_priv, to, project_key).unwrap(),
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

        assert_eq!(open_invitation(&inv, &bob_priv, &friends), Some((project_key, "alice@x.io".into())));
        assert_eq!(open_invitation(&inv, &generate_keypair().1, &friends), None);
        let swapped = ProjectInvitation { from_public_key: generate_keypair().0, ..inv };
        assert_eq!(open_invitation(&swapped, &bob_priv, &friends), None);
    }

    /// The server boxes a key of its own to Bob and serves its public key as the sender's: it is
    /// no friend's key, so the box stays shut.
    #[test]
    fn an_invitation_from_a_key_no_friend_holds_never_opens() {
        let (server_pub, server_priv) = generate_keypair();
        let (bob_pub, bob_priv) = generate_keypair();
        let inv = boxed_invitation(server_pub, &server_priv, &bob_pub, &generate_key());

        assert_eq!(open_invitation(&inv, &bob_priv, &[friend(Some(generate_keypair().0)), friend(None)]), None);
        assert_eq!(open_invitation(&inv, &bob_priv, &[]), None);
    }

    #[test]
    fn a_friends_key_is_pinned_once_served_and_kept() {
        let mut pins = BTreeMap::new();
        let mut later = friend(None);
        assert!(pin_keys(&mut pins, std::slice::from_mut(&mut later)).is_empty());
        assert!(pins.is_empty(), "no key, nothing to pin");

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
        let mut pins = BTreeMap::new();
        pin_keys(&mut pins, &mut [alice.clone()]);

        let mut served = [Friend { public_key: Some(server_pub.clone()), ..alice.clone() }];
        assert_eq!(pin_keys(&mut pins, &mut served), vec![alice.account_id]);
        assert_eq!(served[0].public_key, None);
        let inv = boxed_invitation(server_pub, &server_priv, &bob_pub, &generate_key());
        assert_eq!(open_invitation(&inv, &bob_priv, &served), None);

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
        let mut pins = BTreeMap::new();
        pin_keys(&mut pins, &mut [alice.clone()]);
        pins.remove(&alice.account_id);

        let mut readded = [Friend { public_key: Some(new.clone()), ..alice }];
        assert!(pin_keys(&mut pins, &mut readded).is_empty());
        assert_eq!(readded[0].public_key, Some(new));
    }

    /// Plan §7: once the codes match, the new key is pinned and the friend is trusted again.
    #[test]
    fn a_new_key_trusted_after_comparing_codes_is_kept() {
        let mut pins = BTreeMap::new();
        let alice = friend(Some(vec![1; 32]));
        pin_keys(&mut pins, &mut [alice.clone()]);
        let mut rotated = [Friend { public_key: Some(vec![2; 32]), ..alice.clone() }];
        assert_eq!(pin_keys(&mut pins, &mut rotated.clone()), vec![alice.account_id]);

        trust_key(&mut pins, alice.account_id, &[2; 32]);
        assert!(pin_keys(&mut pins, &mut rotated).is_empty());
        assert_eq!(rotated[0].public_key, Some(vec![2; 32]));
    }

    fn invitation(project_id: Uuid) -> ProjectInvitation {
        ProjectInvitation {
            id: Uuid::new_v4(),
            project_id,
            from_email: "alice@x.io".into(),
            from_public_key: vec![],
            key: EncryptedPair { ct: String::new(), iv: String::new() },
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
}
