use dioxus::prelude::*;
use crate::tid;
use shared::Account;
use uuid::Uuid;

use super::friends_service::{self, FriendsLists};
use crate::common::{error_message, Flash};

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
    flash: Signal<Option<Flash>>,
}

pub fn use_friends() -> Friends {
    let auth_ctx = use_context::<Signal<Option<Account>>>();
    let account_key = use_context::<Signal<Option<[u8; 32]>>>();
    let flash = use_context::<Signal<Option<Flash>>>();
    let version = use_signal(|| 0u64);
    let email = use_signal(String::new);
    let busy = use_signal(|| false);
    let error = use_signal(|| None);

    let lists = use_resource(move || {
        let _ = version();
        let signed_in = auth_ctx().is_some();
        let key = account_key();
        async move {
            if !signed_in {
                return Ok(FriendsLists::default());
            }
            friends_service::load(key).await
        }
    });

    Friends { lists, email, busy, error, version, account_key, flash }
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
        self.run(async move { friends_service::add_by_email(&email, &key).await }, Some(tid!("friends-request-sent")));
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
