use dioxus::prelude::*;
use shared::Account;

use crate::common::LocalStorageState;
use crate::friends::friends_service::served_keys_mismatch;
use crate::tid;

/// The keypair the server serves for the account is not the one this device holds, X25519 or
/// ML-KEM half. Nothing here vouches for or opens with the served keys (`own_keys`); this says so
/// wherever the account is used, not only on the friends page (security review 2026-11-02 H1).
#[component]
pub fn KeypairMismatchBanner() -> Element {
    let auth = use_context::<Signal<Option<Account>>>();
    let key = use_context::<Signal<Option<[u8; 32]>>>();
    let ls_ctx = use_context::<Signal<LocalStorageState>>();
    let mismatch = auth().zip(key()).is_some_and(|(a, k)| served_keys_mismatch(&a, &k, ls_ctx.peek().raw_wraps_ok(a.id)));
    if !mismatch {
        return rsx! {};
    }
    rsx! {
        div { id: "keypair-mismatch-banner", role: "alert", class: "alert alert-error text-sm", {tid!("keypair-mismatch")} }
    }
}
