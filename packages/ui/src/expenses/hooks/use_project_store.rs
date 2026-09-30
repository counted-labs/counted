//! The one project store, held by `AppLayout` for the life of the app.
//!
//! `AppLayout` never remounts, so this survives every navigation: opening an expense and coming
//! back reads already-decrypted data out of context instead of refetching and re-decrypting. The
//! store follows the route's project via [`project_id_of`] and fetches only when that changes — see
//! `docs/plans/expenses-tab-performance.md`.

use api::payments::payments_controller::sync_project_data;
use dioxus::prelude::*;
use crate::tid;
use shared::{Account, ProjectSync, NO_CACHED_VERSION};
use std::sync::Arc;
use uuid::Uuid;

use crate::common::{
    adopt_project_key, ensure_membership, read_from_ls, set_project_cache, update_ls_if_changed,
    ClaimOutcome, Flash, LocalStorageState, ProjectKey,
};
use crate::expenses::helpers::expenses_page_helpers::{
    cached_data_version, decide, key_for_project, For, Outcome,
};
use crate::expenses::helpers::project_data::{self, LiveData, ProjectData};
use crate::expenses::tabs::expenses_tab::{apply_mutation, ExpenseMutation};
use crate::route::{project_id_of, Route};

/// The URL fragment, web only — no other target has one.
fn url_fragment() -> Option<String> {
    #[cfg(target_arch = "wasm32")]
    {
        let hash = web_sys::window().and_then(|w| w.location().hash().ok()).unwrap_or_default();
        let frag = hash.trim_start_matches('#').to_string();
        return (!frag.is_empty()).then_some(frag);
    }
    #[cfg(not(target_arch = "wasm32"))]
    return None;
}

/// Everything the project pages read. All `Copy` handles: the context is created once.
#[derive(Clone, Copy)]
pub struct ProjectStore {
    /// The page's one fetch. Outer `None` means no project has been opened yet.
    pub sync: Resource<Option<Result<For<ProjectSync>, ServerFnError>>>,
    /// Still-encrypted rows, resolved against the offline cache. Mutations patch this directly.
    pub live: Signal<Option<LiveData>>,
    /// The one decryption pass. **Check `data().project_id` before rendering it** — the store
    /// outlives any one page and still holds the previous project until the new sync lands.
    pub data: Memo<Arc<ProjectData>>,
    /// A share link opened without its `#fragment`: resolution failed for good.
    pub key_missing: Signal<bool>,
    /// The sync answered, but with nothing this device can render and no cache to fill the gap.
    /// Only a retry changes it — see [`Outcome::Unusable`].
    pub unusable: Signal<bool>,
    pub on_expenses_changed: Callback<ExpenseMutation>,
}

impl ProjectStore {
    /// The decrypted project, but only when it is the one asked for.
    pub fn data_for(&self, project_id: Uuid) -> Option<Arc<ProjectData>> {
        let d = self.data.read().clone();
        (d.project_id == Some(project_id)).then_some(d)
    }
}

/// Registers the store and the project-key context. Call once, from `AppLayout`.
pub fn use_project_store() -> ProjectStore {
    let ls_ctx = use_context::<Signal<LocalStorageState>>();
    let resource_version = use_context::<Signal<u64>>();
    let auth_ctx = use_context::<Signal<Option<Account>>>();
    let account_key_ctx = use_context::<Signal<Option<[u8; 32]>>>();
    let is_online = use_context::<Signal<bool>>();
    let flash_ctx = use_context::<Signal<Option<Flash>>>();

    // The context signal must start `None`: the URL fragment never reaches the server, so resolving
    // it during the first render is a hydration mismatch that crashes the interpreter.
    let mut key_ctx = use_context_provider(|| ProjectKey(Signal::new(None))).0;
    let mut key_missing = use_signal(|| false);
    let mut unusable = use_signal(|| false);
    let mut live: Signal<Option<LiveData>> = use_signal(|| None);
    let mut force_full_sync = use_signal(|| false);

    // The project the store is following. It never falls back to `None`: leaving a project for the
    // list and returning must cost no request, and clearing it would refetch on the way back.
    let mut active: Signal<Option<Uuid>> = use_signal(|| None);
    let wanted = project_id_of(&use_route::<Route>());
    use_effect(use_reactive!(|wanted| {
        if let Some(id) = wanted {
            if *active.peek() != Some(id) {
                active.set(Some(id));
            }
        }
    }));
    let project_id = active();

    // Resolving the key is reactive on the project, not on the key signal. Keyed off `key_ctx` it
    // never re-ran on a project change, so the page decrypted B's rows with A's key — and the
    // persist below then wrote A's key into B's entry.
    use_effect(use_reactive!(|project_id| {
        let Some(id) = project_id else { return };
        let resolved = key_for_project(url_fragment().as_deref(), &read_from_ls(), id);
        if *key_missing.peek() != resolved.is_none() {
            key_missing.set(resolved.is_none());
        }
        // Guarded: `Signal::set` notifies even for an equal value, and this one invalidates the
        // decryption memo.
        if *key_ctx.peek() != resolved {
            key_ctx.set(resolved);
        }
        let Some(k) = resolved else { return };

        // Holding the key is what makes this device a member, and every way in — create, join
        // modal, deep link, share link, Tricount import — passes through here. Reading `auth_ctx`
        // subscribes on purpose: when `me()` resolves later, an anonymous membership is upgraded.
        let account = auth_ctx();
        let account_key = account_key_ctx();
        let mut flash = flash_ctx;
        spawn(async move {
            // Persisted before the membership write, which escrows whatever the store holds. A
            // fragment key that differs from the held one is kept only if it decrypts the project.
            adopt_project_key(ls_ctx, id, k).await;
            // `IdentityTaken` has already dropped the local `user_id`, so `stored_user_id()` goes
            // `None` and `ExpensesPage` reopens `UserSelectionModal` on its own. All that is left is
            // to say why, otherwise the picker reappears with no explanation.
            match ensure_membership(ls_ctx, id, account, account_key).await {
                ClaimOutcome::IdentityTaken => {
                    flash.set(Some(Flash::err(tid!("identity-taken-repick"))));
                }
                ClaimOutcome::ParticipantGone => {
                    flash.set(Some(Flash::err(tid!("participant-gone-repick"))));
                }
                ClaimOutcome::ProofRejected => {
                    flash.set(Some(Flash::err(tid!("error-claim-proof-invalid"))));
                }
                ClaimOutcome::Ok => {}
            }
        });
        #[cfg(target_arch = "wasm32")]
        if url_fragment().is_none() {
            crate::common::web_dom::replace_state_hash(&crate::crypto::key_to_fragment(&k));
        }
    }));

    // Dropping the previous project's rows before the new sync lands saves a full decryption pass
    // over them under the wrong key. Only on a real switch — going to the list keeps them.
    use_effect(use_reactive!(|project_id| {
        let loaded = live.peek().as_ref().map(|l| l.project_id);
        if let (Some(loaded), Some(want)) = (loaded, project_id) {
            if loaded != want {
                live.set(None);
            }
        }
    }));

    let mut sync = use_resource(use_reactive!(|project_id| async move {
        let _v = resource_version();
        let id = project_id?;
        // `peek`: this must not subscribe, or clearing the flag below would restart the fetch it
        // was set to fix. The restart is always explicit.
        let known = match *force_full_sync.peek() {
            true => NO_CACHED_VERSION,
            false => cached_data_version(&read_from_ls(), id),
        };
        Some(sync_project_data(id, known).await.map(|s| (id, s)))
    }));

    // `use_reactive!`, like the two effects above, and for a sharper reason than they have: this
    // one runs first on `AppLayout`'s mount, where no project is open. Without a declared
    // dependency it would bail at the `let Some` before reading `sync`, subscribe to nothing, and
    // never be woken again — the store would stay empty for the life of the app and every project
    // page would sit on its skeleton for ever.
    use_effect(use_reactive!(|project_id| {
        let Some(id) = project_id else { return };
        // Every resource read stays in this block so the guards drop before anything below touches
        // a signal: one held across `sync.restart()` is a borrow conflict, across `ls_ctx.set()` a
        // re-entrancy panic.
        let outcome = {
            let guard = sync.read();
            // `read_from_ls()`, not `ls_ctx()`: the effect below writes `ls_ctx`, and reading it
            // here would make that a self-trigger.
            let state = read_from_ls();
            let cached = state.projects.iter().find(|p| p.project_id == id);
            decide(guard.as_ref().and_then(|r| r.as_ref()), cached, id)
        };

        // Guarded like every other set here: an unguarded one notifies on an equal value and
        // re-renders the page on every resolution.
        let dead_end = matches!(outcome, Outcome::Unusable);
        if *unusable.peek() != dead_end {
            unusable.set(dead_end);
        }

        let Outcome::Load { users, project, expenses, payments, version } = outcome else {
            // One retry, not a loop: the flag latches on the way *in*, so a second "unchanged"
            // leaves the page waiting rather than restarting forever. The Load path below clears
            // it, which is what re-arms the recovery.
            if matches!(outcome, Outcome::Refetch) && !*force_full_sync.peek() {
                force_full_sync.set(true);
                sync.restart();
            }
            return;
        };

        // The claim was honoured, so the flag has done its job.
        if *force_full_sync.peek() {
            force_full_sync.set(false);
        }

        // Only on change: `Signal::set` notifies even for an equal value, so an unguarded set
        // re-runs the whole decryption pass on every resource resolution.
        let next = LiveData { project_id: id, project, users, expenses, payments, version };
        if live.peek().as_ref() != Some(&next) {
            live.set(Some(next));
        }
    }));

    // Separate from the effect producing `live`: one effect cannot read and write it without
    // re-triggering. Reading `live` rather than the resource is what caches locally-patched rows
    // too, with `version: None` so the next open refetches instead of claiming a stale version.
    use_effect(move || {
        let snapshot = live.read().clone();
        let Some(l) = snapshot else { return };
        let Some(proj) = l.project else { return };

        // Both writes are expensive at scale (serialise the store, re-render every `ls_ctx`
        // reader) and an unchanged revisit is the common case — hence the `_if_changed` variant.
        update_ls_if_changed(ls_ctx, |next| {
            set_project_cache(
                next,
                l.project_id,
                proj,
                l.users,
                l.expenses,
                l.payments,
                l.version,
            );
        });
    });

    // Patch locally, or fall back to a refetch when the mutation cannot be applied (an offline
    // queue entry, or an edit to a row this device never had).
    let on_expenses_changed = use_callback(move |m: ExpenseMutation| {
        let mut guard = live.write();
        let applied = match guard.as_mut() {
            // Resolved after a switch: the other project picks the row up from the version bump
            // on its next open, and this one has nothing to refetch.
            Some(l) if !m.belongs_to(l.project_id) => return,
            Some(l) => {
                let applied = apply_mutation(&mut l.expenses, &mut l.payments, &m);
                if applied {
                    // The new version is unknown here; claiming the old one gets "unchanged"
                    // back and this device never sees its own write again.
                    l.version = None;
                }
                applied
            }
            None => false,
        };
        drop(guard);
        if !applied {
            sync.restart();
        }
    });

    // Nothing else may enter here: adding `ls_ctx` invalidates the memo on every load (the cache
    // effect writes it) and pays for a second full decrypt.
    let data: Memo<Arc<ProjectData>> =
        use_memo(move || Arc::new(project_data::build(key_ctx(), live.read().as_ref())));

    use_revalidation(sync, is_online);

    ProjectStore { sync, live, data, key_missing, unusable, on_expenses_changed }
}

/// Refetch on the two events that can have changed the project behind our back: the network coming
/// back, and the app being foregrounded. Navigation deliberately does not — that is the point of
/// the store.
fn use_revalidation(sync: Resource<Option<Result<For<ProjectSync>, ServerFnError>>>, is_online: Signal<bool>) {
    // `is_online` starts `true` on every platform, so without the previous value the first run
    // fires a spurious refetch at boot.
    let mut was_online = use_signal(|| true);
    use_effect(move || {
        let mut sync = sync;
        let online = is_online();
        let reconnected = online && !*was_online.peek();
        if *was_online.peek() != online {
            was_online.set(online);
        }
        if reconnected {
            sync.restart();
        }
    });

    // Android has no reliable `visibilitychange` in a WebView, and this is not JS — the same hook
    // the deep-link listener uses to read the resume intent.
    #[cfg(any(target_os = "android", target_os = "ios"))]
    dioxus::mobile::use_wry_event_handler(move |event, _| {
        use dioxus::mobile::tao::event::Event;
        let mut sync = sync;
        if let Event::Resumed = event {
            sync.restart();
        }
    });

    // Unconditional hook with the target inside it: gating the `use_hook` itself would give the
    // wasm render one more hook than the SSR render.
    use_hook(move || {
        // `visibilitychange` through web-sys, never `document::eval` — the CSP refuses
        // `new Function` and the throw freezes the app (DOCUMENTATION §8).
        #[cfg(target_arch = "wasm32")]
        crate::common::web_dom::on_tab_visible(move || {
            let mut sync = sync;
            sync.restart();
        });
        // Named, not `let _`: `Resource` is a future, and `let _ =` on one reads as a dropped
        // task (clippy::let_underscore_future). Nothing is being fired and forgotten — this
        // branch exists only so `sync` counts as used where there is no tab to watch.
        #[cfg(not(target_arch = "wasm32"))]
        let _sync_unused_off_web = sync;
    });
}
