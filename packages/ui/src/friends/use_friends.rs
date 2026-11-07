use dioxus::prelude::*;
use crate::tid;
use shared::Account;
use uuid::Uuid;

use super::friends_service::{self, FriendRow, FriendsLists};
use crate::common::{copy_text, error_message, haptic, share_message, Flash, Haptic, LocalStorageState};

/// The friends page's state and every action on it. The page renders what this holds and calls
/// back into it; nothing on the page decides anything.
#[derive(Clone, Copy)]
pub struct Friends {
    pub lists: Resource<Result<FriendsLists, ServerFnError>>,
    pub email: Signal<String>,
    pub busy: Signal<bool>,
    pub error: Signal<Option<String>>,
    /// The address of the request just sent, offered "Let them know by message" until the next one.
    pub sent_to: Signal<Option<String>>,
    version: Signal<u64>,
    account_key: Signal<Option<[u8; 32]>>,
    flash: Signal<Option<Flash>>,
    ls_ctx: Signal<LocalStorageState>,
}

/// One handle per `use_friends` call: two are the same when they hold the same signals.
impl PartialEq for Friends {
    fn eq(&self, other: &Self) -> bool {
        self.version == other.version
    }
}

pub fn use_friends() -> Friends {
    let auth_ctx = use_context::<Signal<Option<Account>>>();
    let account_key = use_context::<Signal<Option<[u8; 32]>>>();
    let flash = use_context::<Signal<Option<Flash>>>();
    let ls_ctx = use_context::<Signal<LocalStorageState>>();
    let version = use_signal(|| 0u64);
    let email = use_signal(String::new);
    let busy = use_signal(|| false);
    let error = use_signal(|| None);
    let sent_to = use_signal(|| None);

    // Not `auth_ctx()`: every write to the account (a preference saved, a refill) would reload the
    // lists. The keypair is all `load` reads from it.
    let identity = use_memo(move || auth_ctx.read().as_ref().map(|a| (a.id, a.public_key.clone(), a.private_key.clone())));
    let lists = use_resource(move || {
        let _ = version();
        let _ = identity();
        let account = auth_ctx.peek().clone();
        let key = account_key();
        async move {
            let Some(account) = account else {
                return Ok(FriendsLists::default());
            };
            let raw_ok = ls_ctx.peek().raw_wraps_ok(account.id);
            friends_service::load(ls_ctx, key, account, raw_ok).await
        }
    });

    Friends { lists, email, busy, error, sent_to, version, account_key, flash, ls_ctx }
}

impl Friends {
    pub fn reload(mut self) {
        self.version += 1;
    }

    fn run(mut self, fut: impl std::future::Future<Output = Result<(), ServerFnError>> + 'static, done: Option<String>) {
        spawn(async move {
            self.busy.set(true);
            self.error.set(None);
            match fut.await {
                Ok(()) => {
                    if let Some(msg) = done {
                        self.flash.set(Some(Flash::ok(msg)));
                    }
                    self.reload();
                }
                Err(e) => self.error.set(Some(error_message(&e))),
            }
            self.busy.set(false);
        });
    }

    pub fn add_by_email(mut self) {
        let email = (self.email)().trim().to_string();
        if email.is_empty() {
            return;
        }
        let Some(key) = (self.account_key)() else {
            self.error.set(Some(tid!("friends-no-account-key")));
            return;
        };
        self.email.set(String::new());
        self.sent_to.set(None);
        let mut sent_to = self.sent_to;
        self.run(
            async move {
                friends_service::add_by_email(&email, &key).await?;
                sent_to.set(Some(email));
                Ok(())
            },
            None,
        );
    }

    /// Hands the user's own messaging app a note to the address: the site and the address to sign
    /// up with, never a project link. Offered whether or not the address has an account.
    pub fn notify(self, email: String) {
        haptic(Haptic::Light);
        let text = friends_service::plain_message(&tid!("friends-notify-message", email: email));
        let copied = tid!("friends-notify-copied");
        let failed = tid!("friends-notify-copy-failed");
        let (mut flash, mut error) = (self.flash, self.error);
        spawn(async move {
            if share_message(&text).await {
                return;
            }
            if copy_text(&text).await {
                flash.set(Some(Flash::ok(copied)));
            } else {
                error.set(Some(failed));
            }
        });
    }

    pub fn accept(self, id: Uuid) {
        self.run(friends_service::accept(id), None);
    }

    pub fn decline_or_withdraw(self, id: Uuid) {
        self.run(friends_service::decline_or_withdraw(id), None);
    }

    /// "The code matches": the friend read out the code shown, so its new key is pinned.
    pub fn trust_new_key(mut self, row: FriendRow) {
        let Some(key) = row.new_key else { return };
        crate::common::update_ls_if_changed(self.ls_ctx, |state| friends_service::trust_key(state, row.account_id, &key, row.new_kem_key.as_deref()));
        self.flash.set(Some(Flash::ok(tid!("friends-code-trusted"))));
        self.reload();
    }

    pub fn remove(self, row: FriendRow) {
        self.run(friends_service::remove(self.ls_ctx, row.id, row.account_id), None);
    }
}
