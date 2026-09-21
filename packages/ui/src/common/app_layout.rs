use crate::common::{
    is_mobile, replay_queue, use_overscroll, BottomDock, DeepLinkListener, Flash,
    LocalStorageState, NavigationSync, PageTransition, QueuedOp, SyncFailure, Toast,
};
use crate::expenses::hooks::use_project_store::use_project_store;
use crate::i18n::current_lang;
use crate::route::Route;
use crate::welcome::WelcomePage;
use dioxus::prelude::*;
use crate::tid;
use std::collections::VecDeque;

#[component]
pub fn AppLayout() -> Element {
    let mut flash = use_context::<Signal<Option<Flash>>>();
    let ls = use_context::<Signal<LocalStorageState>>();
    let is_online = use_context::<Signal<bool>>();
    let pending_ops = use_context::<Signal<VecDeque<QueuedOp>>>();
    let resource_version = use_context::<Signal<u64>>();
    let mut conflict_msg: Signal<Option<SyncFailure>> = use_signal(|| None);
    let replaying = use_signal(|| false);
    let mut replay_blocked = use_signal(|| false);

    // One store for every project route, held here because this layout never remounts — that is
    // what makes expenses -> expense -> back cost no request. Provided, never read: reading
    // `store.data` here would re-render the whole shell on every decryption pass.
    //
    // Called first, then provided: `use_context_provider` runs its closure once, so calling a
    // hook-bearing function inside it would register those hooks on the first render only.
    let store = use_project_store();
    use_context_provider(|| store);

    let route = use_route::<Route>();

    // Elastic scrolling + pull-to-refresh gesture, one JS install for every page. No-op off mobile.
    use_overscroll();

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
        replay_blocked.set(false);
    });

    // Drain the queue when online with pending ops and not already replaying.
    use_effect(move || {
        if is_online() && !pending_ops.read().is_empty() && !replaying() && !replay_blocked() {
            let mut replaying = replaying;
            let mut replay_blocked = replay_blocked;
            let mut resource_version = resource_version;
            replaying.set(true);
            spawn(async move {
                let completed = replay_queue(pending_ops, conflict_msg).await;
                if !completed {
                    replay_blocked.set(true);
                } else {
                    // Bump version so child page resources re-execute and pick up replayed data.
                    resource_version.set(resource_version() + 1);
                }
                replaying.set(false);
            });
        }
    });

    rsx! {
        NavigationSync {}
        // Must live inside the Router — it navigates. Both are no-ops off mobile.
        DeepLinkListener {}
        if !is_online() {
            div {
                class: "fixed top-0 inset-x-0 z-[60] safe-top-banner flex items-center justify-center gap-2 \
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
        if show_onboarding {
            WelcomePage {}
        } else {
            // Renders the Outlet inside an animated, route-keyed shell on mobile; a bare Outlet
            // everywhere else.
            PageTransition {}
            // Self-gating: BottomDock renders nothing on the routes that have no dock.
            BottomDock {}
        }
        if let Some(f) = flash() {
            Toast {
                msg: f.msg,
                success: f.success,
                onclose: move |_| flash.set(None),
            }
        }
        if let Some(failure) = conflict_msg() {
            Toast {
                msg: match failure {
                    SyncFailure::ConflictEdit(name) => tid!("sync-conflict-edit", name: name),
                    SyncFailure::ConflictDelete(name) => tid!("sync-conflict-delete", name: name),
                    SyncFailure::ConflictOther(name) => tid!("sync-conflict-other", name: name),
                    SyncFailure::Error(reason) => tid!("sync-error", reason: reason),
                },
                success: false,
                onclose: move |_| conflict_msg.set(None),
            }
        }
    }
}
