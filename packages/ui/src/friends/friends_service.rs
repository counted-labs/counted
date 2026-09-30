//! Everything the friends pages decide, with no rendering in it: what a friend row shows, what a
//! request sends, how a boxed project key is opened and kept. See `docs/plans/friends.md`.

use api::friends::friends_controller::{
    accept_request, delete_request, get_friends, remove_friend, request_by_email,
    request_from_project,
};
use api::friends::invitations_controller::{delete_invitation, get_invitations, invite};
use api::projects::projects_controller::get_project;
use dioxus::fullstack::Json;
use dioxus::prelude::{ServerFnError, Signal};
use shared::{
    Account, CreatableProjectInvitation, Friend, FriendRequestByEmail, FriendRequestFromProject,
    FriendsView, IncomingFriendRequest, ProjectInvitation,
};
use uuid::Uuid;

use crate::common::{adopt_project_key, LocalStorageState};
use crate::crypto::{
    box_project_key, decrypt, encrypt, key_fingerprint, unbox_project_key, unwrap_key,
};
use crate::decrypted::decrypt_project;

#[derive(Debug, Clone, PartialEq)]
pub struct FriendRow {
    pub id: Uuid,
    pub account_id: Uuid,
    pub email: String,
    /// `None` when the friend has not logged in on a build that makes a keypair yet — they cannot
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
    pub my_fingerprint: Option<String>,
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

pub fn to_lists(
    view: &FriendsView,
    account_key: Option<&[u8; 32]>,
    my_public_key: Option<&[u8]>,
) -> FriendsLists {
    FriendsLists {
        my_fingerprint: my_public_key.map(key_fingerprint),
        friends: view.friends.iter().map(friend_row).collect(),
        incoming: view.incoming.clone(),
        outgoing: view
            .outgoing
            .iter()
            .map(|o| OutgoingRow {
                id: o.id,
                label: account_key
                    .and_then(|k| decrypt(k, &o.label).ok())
                    .unwrap_or_default(),
            })
            .collect(),
    }
}

pub async fn load(
    account_key: Option<[u8; 32]>,
    my_public_key: Option<Vec<u8>>,
) -> Result<FriendsLists, ServerFnError> {
    Ok(to_lists(&get_friends().await?, account_key.as_ref(), my_public_key.as_deref()))
}

/// The response is the same whether or not the address has an account; so is what this returns.
pub async fn add_by_email(email: &str, account_key: &[u8; 32]) -> Result<(), ServerFnError> {
    let email = email.trim().to_string();
    let label = encrypt(account_key, &email).map_err(ServerFnError::new)?;
    request_by_email(Json(FriendRequestByEmail { email, label })).await
}

pub async fn add_from_project(
    project_id: Uuid,
    user_id: i32,
    participant_name: &str,
    account_key: &[u8; 32],
) -> Result<(), ServerFnError> {
    let label = encrypt(account_key, participant_name).map_err(ServerFnError::new)?;
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
/// yet, or when `account_key` does not open it.
pub fn my_private_key(account: &Account, account_key: &[u8; 32]) -> Option<[u8; 32]> {
    unwrap_key(account_key, account.private_key.as_ref()?)
}

/// A friend to invite, and the participant created for them when there is one.
#[derive(Debug, Clone, PartialEq)]
pub struct Invitee {
    pub friend: Friend,
    pub user_id: Option<i32>,
}

/// One invitation per friend, each carrying the key boxed to that friend. Every one is attempted;
/// returns those that were not sent — no private key on this device, no public key on theirs, or a
/// refused request.
pub async fn invite_each(
    project_id: Uuid,
    invitees: &[Invitee],
    my_private: Option<[u8; 32]>,
    project_key: &[u8; 32],
) -> Vec<Invitee> {
    let mut failed = Vec::new();
    for invitee in invitees {
        let key = my_private.as_ref().zip(invitee.friend.public_key.as_deref()).and_then(
            |(private, public)| box_project_key(private, public, project_key),
        );
        let sent = match key {
            Some(key) => invite(
                project_id,
                Json(CreatableProjectInvitation {
                    to_account_id: invitee.friend.account_id,
                    key,
                    user_id: invitee.user_id,
                }),
            )
            .await
            .is_ok(),
            None => false,
        };
        if !sent {
            failed.push(invitee.clone());
        }
    }
    failed
}

pub fn invitee_emails(invitees: &[Invitee]) -> String {
    invitees.iter().map(|i| i.friend.email.as_str()).collect::<Vec<_>>().join(", ")
}

/// An invitation this device managed to open: the key is real, so the project name can be shown
/// before anything is accepted.
#[derive(Debug, Clone, PartialEq)]
pub struct OpenedInvitation {
    pub invitation: ProjectInvitation,
    /// `None` when the box did not open — the row can only be declined.
    pub project_key: Option<[u8; 32]>,
    pub project_name: Option<String>,
}

pub fn open_invitation(inv: &ProjectInvitation, my_private: &[u8; 32]) -> Option<[u8; 32]> {
    unbox_project_key(my_private, &inv.from_public_key, &inv.key)
}

pub async fn load_invitations(my_private: Option<[u8; 32]>) -> Result<Vec<OpenedInvitation>, ServerFnError> {
    let mut opened = Vec::new();
    for invitation in get_invitations().await? {
        let boxed = my_private.and_then(|p| open_invitation(&invitation, &p));
        // A box that opens is not a key that decrypts: one holding a wrong key used to show Accept
        // with a blank name. Unreachable projects keep the key — accepting still proves it before
        // it can replace one already held.
        let (project_key, project_name) = match boxed {
            Some(k) => match get_project(invitation.project_id).await {
                Ok(p) => match decrypt_project(&k, &p) {
                    Ok(d) => (Some(k), Some(d.name)),
                    Err(_) => (None, None),
                },
                Err(_) => (Some(k), None),
            },
            None => (None, None),
        };
        opened.push(OpenedInvitation { invitation, project_key, project_name });
    }
    Ok(opened)
}

/// Accepting is a local act: the key goes into the store exactly as a pasted share link's would —
/// through [`adopt_project_key`], so it cannot replace a held key it does not prove — and
/// `ExpensesPage` takes it from there (membership, escrow, the identity picker). The key is written
/// before the row is deleted — a delete that fails leaves the invitation visible, and accepting it
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
        assert_eq!(to_lists(&view, Some(&account_key), None).outgoing[0].label, "carol@x.io");
        assert_eq!(to_lists(&view, None, None).outgoing[0].label, "");
        assert_eq!(to_lists(&view, Some(&generate_key()), None).outgoing[0].label, "");
    }

    /// What makes the check possible at all: Alice's row for Bob and Bob's own code must be the
    /// same string, or two friends reading their codes to each other can never match.
    #[test]
    fn my_code_is_what_my_friends_see_next_to_my_email() {
        let (bob_pub, _) = generate_keypair();
        let alices_view = FriendsView { friends: vec![friend(Some(bob_pub.clone()))], ..Default::default() };

        let on_alices_page = to_lists(&alices_view, None, None).friends[0].fingerprint.clone();
        let on_bobs_page = to_lists(&FriendsView::default(), None, Some(&bob_pub)).my_fingerprint;

        assert!(on_bobs_page.is_some());
        assert_eq!(on_alices_page, on_bobs_page);
    }

    #[test]
    fn an_account_without_a_keypair_has_no_code_of_its_own() {
        assert_eq!(to_lists(&FriendsView::default(), None, None).my_fingerprint, None);
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
        };
        assert_eq!(my_private_key(&account, &account_key), None);
        account.private_key = crate::crypto::wrap_key(&account_key, &private);
        assert_eq!(my_private_key(&account, &account_key), Some(private));
        assert_eq!(my_private_key(&account, &generate_key()), None);
    }
}
