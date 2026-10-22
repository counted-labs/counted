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

use crate::common::{adopt_project_key, LocalStorageState};
use crate::crypto::{
    box_project_key, decrypt, encrypt, encrypt_json, key_fingerprint, open_wrap, public_key_of,
    unbox_project_key, unwrap_key, WrapUse,
};
use crate::decrypted::decrypt_project;

#[derive(Debug, Clone, PartialEq)]
pub struct FriendRow {
    pub id: Uuid,
    pub account_id: Uuid,
    pub email: String,
    /// `None` when the friend has not logged in on a build that makes a keypair yet - they cannot
    /// be invited until they do.
    pub fingerprint: Option<String>,
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

pub fn friend_row(f: &Friend) -> FriendRow {
    FriendRow {
        id: f.id,
        account_id: f.account_id,
        email: f.email.clone(),
        fingerprint: f.public_key.as_deref().map(key_fingerprint),
    }
}

/// `raw_ok` is `LocalStorageState::raw_wraps_ok` for the account.
pub fn to_lists(
    view: &FriendsView,
    account_key: Option<&[u8; 32]>,
    account: Option<&Account>,
    raw_ok: bool,
) -> FriendsLists {
    let my_private = account.zip(account_key).and_then(|(a, k)| my_private_key(a, k, raw_ok));
    FriendsLists {
        my_fingerprint: my_private.map(|p| key_fingerprint(&public_key_of(&p))),
        key_mismatch: account.zip(account_key).is_some_and(|(a, k)| served_key_mismatch(a, k, raw_ok)),
        friends: view.friends.iter().map(friend_row).collect(),
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

pub async fn load(account_key: Option<[u8; 32]>, account: Account, raw_ok: bool) -> Result<FriendsLists, ServerFnError> {
    Ok(to_lists(&get_friends().await?, account_key.as_ref(), Some(&account), raw_ok))
}

/// A label is JSON once typed wraps are on, so a co-member's participant name never decrypts as
/// a bare key fragment (audit 2026-10-19 L1); older labels are the bare text.
fn write_label(account_key: &[u8; 32], text: &str, typed: bool) -> Result<shared::EncryptedPair, ServerFnError> {
    if typed { encrypt_json(account_key, &text) } else { encrypt(account_key, text) }.map_err(ServerFnError::new)
}

fn read_label(plain: String) -> String {
    serde_json::from_str::<String>(&plain).unwrap_or(plain)
}

/// The response is the same whether or not the address has an account; so is what this returns.
pub async fn add_by_email(email: &str, account_key: &[u8; 32], typed: bool) -> Result<(), ServerFnError> {
    let email = email.trim().to_string();
    let label = write_label(account_key, &email, typed)?;
    request_by_email(Json(FriendRequestByEmail { email, label })).await
}

pub async fn add_from_project(
    project_id: Uuid,
    user_id: i32,
    participant_name: &str,
    account_key: &[u8; 32],
    typed: bool,
) -> Result<(), ServerFnError> {
    let label = write_label(account_key, participant_name, typed)?;
    request_from_project(Json(FriendRequestFromProject { project_id, user_id, label })).await
}

pub async fn accept(id: Uuid) -> Result<(), ServerFnError> {
    accept_request(id).await
}

pub async fn decline_or_withdraw(id: Uuid) -> Result<(), ServerFnError> {
    delete_request(id).await
}

pub async fn remove(id: Uuid) -> Result<(), ServerFnError> {
    remove_friend(id).await
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

pub fn open_invitation(inv: &ProjectInvitation, my_private: &[u8; 32]) -> Option<[u8; 32]> {
    unbox_project_key(my_private, &inv.from_public_key, &inv.key)
}

pub async fn load_invitations(my_private: Option<[u8; 32]>) -> Result<Vec<OpenedInvitation>, ServerFnError> {
    let boxed: Vec<(ProjectInvitation, Option<[u8; 32]>)> = get_invitations()
        .await?
        .into_iter()
        .map(|invitation| {
            let key = my_private.and_then(|p| open_invitation(&invitation, &p));
            (invitation, key)
        })
        .collect();
    let mut ids: Vec<Uuid> = boxed.iter().filter(|(_, k)| k.is_some()).map(|(i, _)| i.project_id).collect();
    ids.sort();
    ids.dedup();
    let projects = match ids.is_empty() {
        true => Vec::new(),
        false => get_projects_by_ids(Json(BatchProject { ids })).await.unwrap_or_default(),
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
    use crate::crypto::{generate_key, generate_keypair};
    use chrono::NaiveDateTime;
    use shared::{EncryptedPair, OutgoingFriendRequest};

    fn friend(public_key: Option<Vec<u8>>) -> Friend {
        Friend { id: Uuid::new_v4(), account_id: Uuid::new_v4(), email: "bob@x.io".into(), public_key }
    }

    #[test]
    fn a_friend_without_a_key_has_no_fingerprint() {
        assert_eq!(friend_row(&friend(None)).fingerprint, None);
        let (public, _) = generate_keypair();
        assert_eq!(friend_row(&friend(Some(public.clone()))).fingerprint, Some(key_fingerprint(&public)));
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
        assert_eq!(to_lists(&view, Some(&account_key), None, true).outgoing[0].label, "carol@x.io");
        assert_eq!(to_lists(&view, None, None, true).outgoing[0].label, "");
        assert_eq!(to_lists(&view, Some(&generate_key()), None, true).outgoing[0].label, "");
    }

    #[test]
    fn a_label_reads_back_in_either_form() {
        let account_key = generate_key();
        for typed in [false, true] {
            let label = write_label(&account_key, "Carol \"C\"", typed).unwrap();
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
            private_key: crate::crypto::wrap_key(&account_key, WrapUse::PrivateKey, &private, true),
            ..account
        };
        assert_eq!(my_private_key(&typed, &account_key, false), Some(private));
        assert!(private_key_is_typed(&typed, &account_key));
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
            private_key: crate::crypto::wrap_key(account_key, WrapUse::PrivateKey, private, false),
            typed_wraps: false,
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

        let on_alices_page = to_lists(&alices_view, None, None, true).friends[0].fingerprint.clone();
        let on_bobs_page = to_lists(&FriendsView::default(), Some(&bobs_key), Some(&bob), true).my_fingerprint;

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

        let lists = to_lists(&FriendsView::default(), Some(&bobs_key), Some(&bob), true);
        assert!(lists.key_mismatch);
        assert_ne!(lists.my_fingerprint, Some(key_fingerprint(&server_pub)));
        assert_eq!(lists.my_fingerprint, None);
        assert_eq!(my_private_key(&bob, &bobs_key, true), None);
    }

    #[test]
    fn an_account_without_a_keypair_has_no_code_of_its_own() {
        assert_eq!(to_lists(&FriendsView::default(), None, None, true).my_fingerprint, None);
    }

    #[test]
    fn an_invitation_opens_with_the_senders_public_key_only() {
        let (alice_pub, alice_priv) = generate_keypair();
        let (bob_pub, bob_priv) = generate_keypair();
        let project_key = generate_key();
        let inv = ProjectInvitation {
            id: Uuid::new_v4(),
            project_id: Uuid::new_v4(),
            from_email: "alice@x.io".into(),
            from_public_key: alice_pub,
            key: box_project_key(&alice_priv, &bob_pub, &project_key).unwrap(),
            created_at: NaiveDateTime::default(),
            user_id: None,
        };
        assert_eq!(open_invitation(&inv, &bob_priv), Some(project_key));
        assert_eq!(open_invitation(&inv, &generate_keypair().1), None);
        let swapped = ProjectInvitation { from_public_key: generate_keypair().0, ..inv };
        assert_eq!(open_invitation(&swapped, &bob_priv), None);
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
            owner_account_id: None,
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
        };
        assert_eq!(my_private_key(&account, &account_key, true), None);
        account.private_key = crate::crypto::wrap_key(&account_key, WrapUse::PrivateKey, &private, false);
        account.public_key = Some(public_key_of(&private));
        assert_eq!(my_private_key(&account, &account_key, true), Some(private));
        assert_eq!(my_private_key(&account, &generate_key(), true), None);
    }
}
