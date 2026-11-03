//! The project key as the capability, not the project id (docs/plans/key-lifecycle.md §8.2): every
//! project route carries `SHA-256("counted-claim-v1" || key)`, the claim token the server already
//! checks at membership writes, in the `x-project-token` header - never the query string, which
//! reaches nginx logs. Batch routes carry one token per project in their body instead.
//!
//! The header is the server-function client's global one, read when a request is built on its first
//! poll. So [`scope`] goes immediately before the call it is for, with no `.await` between them: on
//! the one-threaded wasm and on the mobile event loop, nothing else can run in that gap.

use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use dioxus::prelude::*;
use uuid::Uuid;

use super::local_storage::{key_of, read_from_ls, LocalStorageState};
use crate::crypto::claim_token;

pub const HEADER: &str = "x-project-token";

fn held_key(project_id: Uuid) -> Option<[u8; 32]> {
    let find = |state: &LocalStorageState| state.projects.iter().find(|p| p.project_id == project_id).and_then(key_of);
    match try_consume_context::<Signal<LocalStorageState>>() {
        Some(ls) => find(&ls.peek()),
        None => find(&read_from_ls()),
    }
}

pub fn header_value(key: &[u8; 32]) -> String {
    URL_SAFE_NO_PAD.encode(claim_token(key))
}

/// The next request carries `project_id`'s token, or none when this device holds no key for it.
pub fn scope(project_id: Uuid) {
    scope_key(held_key(project_id).as_ref());
}

/// [`scope`] for a caller that already holds the key, such as a project it has just created.
pub fn scope_key(key: Option<&[u8; 32]>) {
    let mut headers = dioxus::fullstack::HeaderMap::new();
    if let Some(value) = key.and_then(|k| header_value(k).parse().ok()) {
        headers.insert(HEADER, value);
    }
    dioxus::fullstack::set_request_headers(headers);
}

/// One token per project, in order, for the batch routes.
pub fn tokens(ids: &[Uuid]) -> Vec<Option<Vec<u8>>> {
    ids.iter().map(|id| held_key(*id).map(|k| claim_token(&k).to_vec())).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_header_is_the_claim_token_in_base64url() {
        let key = [3u8; 32];
        let decoded = URL_SAFE_NO_PAD.decode(header_value(&key)).unwrap();
        assert_eq!(decoded, claim_token(&key).to_vec());
        assert!(!header_value(&key).contains(['+', '/', '=']));
    }
}
