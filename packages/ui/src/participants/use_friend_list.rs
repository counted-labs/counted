use dioxus::prelude::*;
use shared::{Account, Friend};

use crate::crypto::decrypt_json;

#[derive(Clone, PartialEq, Default)]
pub struct FriendList {
    pub signed_in: bool,
    pub friends: Vec<Friend>,
}

/// This account's friends, empty for an anonymous user or while loading. A failed load leaves the
/// block usable with typed names only.
pub fn use_friend_list() -> FriendList {
    let auth = use_context::<Signal<Option<Account>>>();
    let friends = use_resource(move || {
        let signed_in = auth.read().is_some();
        async move {
            if !signed_in {
                return Vec::new();
            }
            api::friends::friends_controller::get_friends()
                .await
                .map(|v| v.friends)
                .unwrap_or_default()
        }
    });
    let signed_in = auth.read().is_some();
    let friends = friends.read().clone().unwrap_or_default();
    FriendList { signed_in, friends }
}

/// The account's display name, decrypted: what "your name in this project" starts with.
pub fn use_my_display_name() -> Option<String> {
    let auth = use_context::<Signal<Option<Account>>>();
    let account_key = use_context::<Signal<Option<[u8; 32]>>>();
    let account = auth.read().clone()?;
    let key = account_key()?;
    decrypt_json::<String>(&key, &account.display_name).ok()
}
