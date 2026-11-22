use crate::common::{
    is_mobile, replay_queue, use_keyboard, use_overscroll, use_route_focus, use_sheet_drag, BottomDock, DeepLinkListener, Flash, KeyOfferSheet,
    Landed, LocalStorageState, NavigationSync, PageTransition, Queue, SyncFailure, ToastHost,
};
use crate::common::queue_replay::{still_blocked, HeldKeys};
use crate::demo::DemoBar;
use crate::expenses::hooks::use_project_store::use_project_store;
use crate::expenses::tabs::expenses_tab::ExpenseMutation;
use crate::i18n::current_lang;
use crate::route::Route;
use crate::welcome::WelcomePage;
use dioxus::prelude::*;
use crate::tid;

/// A replay stopped by a failure that is not a refusal (no network, a 5xx) is tried again after this,
/// doubled on each failure up to five minutes: the web sees no connectivity change to retry on.
const RETRY_FIRST_MS: u32 = 5_000;
const RETRY_MAX_MS: u32 = 300_000;

#[component]
pub fn AppLayout() -> Element {
    let ls = use_context::<Signal<LocalStorageState>>();
    let is_online = use_context::<Signal<bool>>();
    let pending_ops = use_context::<Signal<Queue>>();
    let resource_version = use_context::<Signal<u64>>();
    let mut conflict_msg: Signal<Option<SyncFailure>> = use_signal(|| None);
    let mut flash = use_context::<Signal<Option<Flash>>>();
    use_effect(move || {
        if let Some(failure) = conflict_msg() {
            conflict_msg.set(None);
            flash.set(Some(Flash::err(match failure {
                SyncFailure::ConflictEdit(name) => tid!("sync-conflict-edit", name: name),
                SyncFailure::ConflictOther(name) => tid!("sync-conflict-other", name: name),
                SyncFailure::Held(name) if name.is_empty() => tid!("sync-held-unnamed"),
                SyncFailure::Held(name) => tid!("sync-held", name: name),
                SyncFailure::Error(reason) => tid!("sync-error", reason: reason),
                SyncFailure::Storage => tid!("sync-storage-full"),
            })));
        }
    });
    let auth = use_context::<Signal<Option<shared::Account>>>();
    let account_key = use_context::<Signal<Option<[u8; 32]>>>();
    let badges = use_context::<crate::badges::Badges>();
    use_effect(move || {
        let (Some(account), Some(key)) = (auth(), account_key()) else { return };
        crate::badges::evaluate(badges.0, &account, &key, &ls.read());
        crate::badges::shown::renew_if_needed(&account, &key, &ls.read());
    });
    let replaying = use_signal(|| false);
    let retry_ms = use_signal(|| RETRY_FIRST_MS);
    let mut replay_blocked: Signal<Option<HeldKeys>> = use_signal(|| None);

    // One store for every project route, held here because this layout never remounts - that is
    // what makes expenses -> expense -> back cost no request. Provided, never read: reading
    // `store.data` here would re-render the whole shell on every decryption pass.
    //
    // Called first, then provided: `use_context_provider` runs its closure once, so calling a
    // hook-bearing function inside it would register those hooks on the first render only.
    let store = use_project_store();
    use_context_provider(|| store);

    // SPEC §15.9.5 Q0, Q2f: the old key moves at start and on each `storage` event for it, the
    // entries of the event's new value kept until they have moved, online or not and under no lock;
    // another tab's change starts a replay, even one that had stopped. A burst of `storage` events
    // costs one read.
    use_effect(move || crate::common::queue_replay::move_old_key(pending_ops));
    use_hook(move || {
        #[cfg(target_arch = "wasm32")]
        crate::common::web_dom::on_storage(250, crate::common::queue_store::OLD_KEY, move |text| {
            use crate::common::queue_store::{device_store, receive_old_text};
            receive_old_text(&mut device_store(), text);
            crate::common::queue_replay::move_old_key(pending_ops);
        }, move |keys| {
            use crate::common::queue_store::{OLD_KEY, PREFIX};
            let mut pending_ops = pending_ops;
            let mut replay_blocked = replay_blocked;
            let queue_changed = keys.iter().any(|k| k.as_deref().is_none_or(|k| k.starts_with(PREFIX) || k == OLD_KEY));
            let erased = !queue_changed
                && keys.iter().any(|k| k.as_deref() == Some(crate::common::local_storage::LS_KEY))
                && {
                    let state = crate::common::read_from_ls();
                    pending_ops.peek().iter().any(|e| !state.projects.iter().any(|p| Some(p.project_id) == e.project && crate::common::key_of(p).is_some()))
                };
            if !queue_changed && !erased {
                return;
            }
            let shown = crate::common::read_queue();
            if *pending_ops.peek() != shown {
                pending_ops.set(shown);
            }
            replay_blocked.set(None);
        });
        #[cfg(target_arch = "wasm32")]
        crate::common::web_dom::on_page_restored(move || {
            let mut replay_blocked = replay_blocked;
            replay_blocked.set(None);
        });
    });

    // A pending add that leaves what this tab shows without this tab landing it was sent by another
    // tab (or discarded): the project is brought again, or its row would vanish until a refetch.
    let shown_adds = use_hook(|| std::rc::Rc::new(std::cell::RefCell::new(std::collections::HashSet::<uuid::Uuid>::new())));
    use_effect(move || {
        let now: std::collections::HashSet<uuid::Uuid> = pending_ops
            .read()
            .iter()
            .filter(|e| matches!(e.op.as_ref().map(|q| &q.op), Some(crate::common::OpKind::AddExpense(_))))
            .filter_map(|e| e.id)
            .collect();
        let elsewhere = shown_adds.borrow().iter().any(|id| !now.contains(id) && !crate::common::queue_replay::landed_here(*id));
        *shown_adds.borrow_mut() = now;
        if elsewhere {
            let mut sync = store.sync;
            sync.restart();
        }
    });

    let route = use_route::<Route>();
    use_route_focus(&route);

    // Elastic scrolling + pull-to-refresh gesture, one JS install for every page. No-op off mobile.
    use_overscroll();
    use_sheet_drag();
    use_keyboard();

    // Onboarding depends on localStorage + user agent, which SSR can't know. Deciding it on the
    // first render diverges from the served HTML and breaks hydration (dead app, no listeners).
    let mut mounted = use_signal(|| false);
    use_effect(move || mounted.set(true));

    // The app ships dx's default index template on purpose (see docs/dioxus-overrides.md), so
    // <html lang> is never set. Without it a screen reader reads the whole UI with English
    // pronunciation rules. Reactive on the locale: `current_lang()` subscribes, so switching
    // language updates the attribute too.
    use_effect(move || {
        let lang = current_lang();
        #[cfg(target_arch = "wasm32")]
        crate::common::web_dom::set_html_lang(&lang);
        #[cfg(not(target_arch = "wasm32"))]
        document::eval(&format!(
            "document.documentElement.lang = {};",
            serde_json::to_string(&lang).unwrap_or_default()
        ));
    });

    let show_onboarding =
        mounted() && !ls().onboarding_seen && is_mobile() && matches!(route, Route::ProjectsPage {});

    // Clear replay block on any connectivity change (going offline → cleared, but replay won't
    // start because is_online() is false; going online → cleared so reconnect retries).
    use_effect(move || {
        let _ = is_online();
        replay_blocked.set(None);
    });

    // A new write retries a blocked queue too: the web never sees a connectivity change, and every
    // add, edit and delete goes through the queue.
    let queued_len = use_hook(|| std::rc::Rc::new(std::cell::Cell::new(pending_ops.peek().len())));
    use_effect(move || {
        let len = pending_ops.read().len();
        if len > queued_len.replace(len) && replay_blocked.peek().is_some() {
            replay_blocked.set(None);
        }
    });

    // Creations whose outcome is unknown, settled at start and on every reconnect once the boot
    // `me()` is over. Adoption still waits for the creator's account (`project_create::settle_by`).
    let auth_resolved = use_context::<Signal<crate::common::AuthResolved>>();
    use_effect(move || {
        if is_online() && auth_resolved().0 && !ls.peek().pending_creations.is_empty() {
            spawn(crate::common::resolve_pending(ls));
        }
    });

    let on_landed = use_callback(move |landed: Landed| {
        store.on_expenses_changed.call(match landed {
            Landed::Added(c) => ExpenseMutation::Added(c),
            Landed::Edited(c) => ExpenseMutation::Edited(c),
            Landed::Deleted(id) => ExpenseMutation::Deleted(id),
        });
    });

    // Drain the queue when online with pending ops and not already replaying.
    use_effect(move || {
        if is_online()
            && !pending_ops.read().is_empty()
            && !replaying()
            && !still_blocked(replay_blocked.read().as_ref(), &ls.read())
        {
            let mut replaying = replaying;
            let mut replay_blocked = replay_blocked;
            let mut resource_version = resource_version;
            let mut retry_ms = retry_ms;
            replaying.set(true);
            spawn(async move {
                let end = replay_queue(pending_ops, conflict_msg, on_landed).await;
                replay_blocked.set(end.blocked);
                if end.transient {
                    let delay = *retry_ms.peek();
                    retry_ms.set((delay * 2).min(RETRY_MAX_MS));
                    spawn(async move {
                        crate::common::sleep(delay).await;
                        replay_blocked.set(None);
                    });
                } else {
                    retry_ms.set(RETRY_FIRST_MS);
                }
                // Expense ops patched the store as they landed; anything else re-runs the pages'
                // resources.
                if end.refetch {
                    resource_version.set(resource_version() + 1);
                }
                replaying.set(false);
            });
        }
    });

    rsx! {
        NavigationSync {}
        // Must live inside the Router - it navigates. Both are no-ops off mobile.
        DeepLinkListener {}
        if !is_online() {
            div {
                class: "offline-banner fixed top-0 inset-x-0 z-[60] safe-top-banner flex items-center justify-center gap-2 \
                        bg-warning text-warning-content text-xs font-medium py-1.5 px-3",
                role: "status",
                aria_live: "polite",
                span { {tid!("offline-banner")} }
                if !pending_ops.read().is_empty() {
                    span { class: "badge badge-xs badge-neutral",
                        {tid!("offline-pending", count: pending_ops.read().len() as i64)}
                    }
                }
            }
        }
        DemoBar {}
        if show_onboarding {
            WelcomePage {}
        } else {
            // Renders the Outlet inside an animated, route-keyed shell on mobile; a bare Outlet
            // everywhere else.
            PageTransition {}
            // Self-gating: BottomDock renders nothing on the routes that have no dock.
            BottomDock {}
        }
        KeyOfferSheet {}
        ToastHost {}
    }
}
