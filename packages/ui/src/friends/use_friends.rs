use dioxus::prelude::*;
use crate::tid;
use shared::Account;
use uuid::Uuid;

use super::friends_service::{self, FriendsLists};
use crate::common::{error_message, Flash, LocalStorageState};

/// The friends page's state and every action on it. The page renders what this holds and calls
/// back into it; nothing on the page decides anything.
#[derive(Clone, Copy)]
pub struct Friends {
    pub lists: Resource<Result<FriendsLists, ServerFnError>>,
    pub email: Signal<String>,
    pub busy: Signal<bool>,
    pub error: Signal<Option<String>>,
    version: Signal<u64>,
    account_key: Signal<Option<[u8; 32]>>,
    auth_ctx: Signal<Option<Account>>,
    flash: Signal<Option<Flash>>,
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
            friends_service::load(key, account, raw_ok).await
        }
    });

    Friends { lists, email, busy, error, version, account_key, auth_ctx, flash }
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
        let typed = (self.auth_ctx)().is_some_and(|a| a.typed_wraps);
        self.email.set(String::new());
        self.run(async move { friends_service::add_by_email(&email, &key, typed).await }, Some(tid!("friends-request-sent")));
    }

    pub fn accept(self, id: Uuid) {
        self.run(friends_service::accept(id), None);
    }

    pub fn decline_or_withdraw(self, id: Uuid) {
        self.run(friends_service::decline_or_withdraw(id), None);
    }

    pub fn remove(self, id: Uuid) {
        self.run(friends_service::remove(id), None);
    }
}
