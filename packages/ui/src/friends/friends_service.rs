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

use crate::common::{update_ls, upsert_project, upsert_project_key, LocalStorageState};
use crate::crypto::{
    box_project_key, decrypt_pair, encrypt_pair, key_fingerprint, key_to_fragment, project_name,
    unbox_project_key, unwrap_private_key,
};

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

pub fn to_lists(view: &FriendsView, account_key: Option<&[u8; 32]>) -> FriendsLists {
    FriendsLists {
        friends: view.friends.iter().map(friend_row).collect(),
        incoming: view.incoming.clone(),
        outgoing: view
            .outgoing
            .iter()
            .map(|o| OutgoingRow {
                id: o.id,
                label: account_key
                    .and_then(|k| decrypt_pair(k, &o.label).ok())
                    .unwrap_or_default(),
            })
            .collect(),
    }
}

pub async fn load(account_key: Option<[u8; 32]>) -> Result<FriendsLists, ServerFnError> {
    Ok(to_lists(&get_friends().await?, account_key.as_ref()))
}

/// The response is the same whether or not the address has an account; so is what this returns.
pub async fn add_by_email(email: &str, account_key: &[u8; 32]) -> Result<(), ServerFnError> {
    let email = email.trim().to_string();
    let label = encrypt_pair(account_key, &email).map_err(ServerFnError::new)?;
    request_by_email(Json(FriendRequestByEmail { email, label })).await
}

pub async fn add_from_project(
    project_id: Uuid,
    user_id: i32,
    participant_name: &str,
    account_key: &[u8; 32],
) -> Result<(), ServerFnError> {
    let label = encrypt_pair(account_key, participant_name).map_err(ServerFnError::new)?;
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

/// This account's private key, opened with the account key. `None` for a session restored from
/// the cookie (no account key on this device) or an account with no keypair yet.
pub fn my_private_key(account: &Account, account_key: &[u8; 32]) -> Option<[u8; 32]> {
    unwrap_private_key(account_key, account.private_key.as_ref()?)
}

/// One invitation per friend, each carrying the key boxed to that friend. Friends without a public
/// key are skipped — the server would refuse them anyway. Returns how many were sent.
pub async fn invite_friends(
    project_id: Uuid,
    friends: &[Friend],
    my_private: &[u8; 32],
    project_key: &[u8; 32],
) -> Result<usize, ServerFnError> {
    let mut sent = 0;
    for friend in friends {
        let Some(public) = friend.public_key.as_deref() else { continue };
        let Some(key) = box_project_key(my_private, public, project_key) else { continue };
        invite(project_id, Json(CreatableProjectInvitation { to_account_id: friend.account_id, key }))
            .await?;
        sent += 1;
    }
    Ok(sent)
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
        let project_key = my_private.and_then(|p| open_invitation(&invitation, &p));
        let project_name = match project_key {
            Some(k) => get_project(invitation.project_id).await.ok().map(|p| project_name(&k, &p)),
            None => None,
        };
        opened.push(OpenedInvitation { invitation, project_key, project_name });
    }
    Ok(opened)
}

/// Accepting is a local act: the key goes into the store exactly as a pasted share link's would,
/// and `ExpensesPage` takes it from there (membership, escrow, the identity picker). The key is
/// written before the row is deleted — a delete that fails leaves the invitation visible, and
/// accepting it again is harmless.
pub async fn accept_invitation(
    ls_ctx: Signal<LocalStorageState>,
    invitation: &ProjectInvitation,
    project_key: &[u8; 32],
) -> Result<(), ServerFnError> {
    let project_id = invitation.project_id;
    let fragment = key_to_fragment(project_key);
    update_ls(ls_ctx, |state| {
        upsert_project(state, project_id, None);
        upsert_project_key(state, project_id, fragment);
    });
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
                label: encrypt_pair(&account_key, "carol@x.io").unwrap(),
                created_at: NaiveDateTime::default(),
            }],
        };
        assert_eq!(to_lists(&view, Some(&account_key)).outgoing[0].label, "carol@x.io");
        assert_eq!(to_lists(&view, None).outgoing[0].label, "");
        assert_eq!(to_lists(&view, Some(&generate_key())).outgoing[0].label, "");
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
        account.private_key = crate::crypto::wrap_private_key(&account_key, &private);
        assert_eq!(my_private_key(&account, &account_key), Some(private));
        assert_eq!(my_private_key(&account, &generate_key()), None);
    }
}
