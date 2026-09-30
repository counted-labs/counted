/// Serializes every test that touches the process-global `COUNTED_DATA_DIR` / `HOME` env vars.
///
/// It must be shared across modules: `local_storage`, `offline_queue` and `expenses::helpers::export`
/// all resolve their paths from those same two vars, so a mutex private to each module lets their
/// tests run concurrently and clobber each other's value — which showed up as a test reading
/// another test's file after the var was cleared and the path fell back to `$HOME`.
#[cfg(all(test, not(target_arch = "wasm32")))]
pub(crate) fn env_lock() -> std::sync::MutexGuard<'static, ()> {
    static ENV_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
    // A failing test panics while holding the guard, which poisons the mutex. Recover rather than
    // unwrap, or one genuine failure cascades into PoisonError for every other env-dependent test
    // and buries the actual cause.
    ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner())
}

/// The loaded-and-successful value of a resource, cloned. `None` while pending or on error.
///
/// Reads through `Resource::read()` on purpose: the built-in `Resource::result()` branches on
/// `peek()`, which would not subscribe the caller to later updates.
pub fn resource_ok<T: Clone + 'static, E: 'static>(
    resource: &dioxus::prelude::Resource<Result<T, E>>,
) -> Option<T> {
    use dioxus::prelude::ReadableExt;
    resource.read().as_ref().and_then(|r| r.as_ref().ok()).cloned()
}

/// The resource's failure as (message to display, whether the server was unreachable).
/// The second flag is what lets callers fall back to cached data for a connectivity error
/// without doing the same for a genuine server error.
pub fn resource_err<T: 'static>(
    resource: &dioxus::prelude::Resource<Result<T, dioxus::prelude::ServerFnError>>,
) -> Option<(String, bool)> {
    use dioxus::prelude::ReadableExt;
    resource
        .read()
        .as_ref()
        .and_then(|r| r.as_ref().err().map(|e| (error_message(e), is_offline_error(e))))
}

mod avatar;
pub use avatar::{Avatar, SizeClass};

mod date_format;
pub use date_format::{format_date, format_date_str, format_month_str, month_abbrev, month_name};

mod avatar_group;
pub use avatar_group::AvatarGroup;

mod mascot;
pub use mascot::{Mascot, MascotPose, MASCOT_EMPTY, MASCOT_INLINE};

mod back_button_arrow;
pub use back_button_arrow::BackButtonArrow;

mod app_header;
pub use app_header::AppHeader;

mod page_title;
pub use page_title::PageTitle;

mod next_paint;
pub use next_paint::next_paint;

mod sleep;
pub use sleep::sleep;

mod select_focused;
pub use select_focused::select_focused_input;

mod toast;
pub use toast::Toast;

mod dropdown_button;
pub use dropdown_button::{DropdownButton, DropdownItem};

mod project_status_menu;
pub use project_status_menu::ProjectStatusItems;

mod speed_dial_fab;
pub use speed_dial_fab::{SpeedDialAction, SpeedDialFab};

mod callout;
pub use callout::*;

pub mod persist;

pub mod fx_cache;
pub use fx_cache::CachedFx;

pub mod account_sync;
pub use account_sync::{apply_pull, copy_missing, left_elsewhere, to_push};

pub mod local_storage;
pub use local_storage::{
    clear_user_id, initials, is_mobile, key_of, mark_synced, project_key, read_from_ls, remove_project,
    set_anon_member_id, set_cached_projects_list, set_project_cache, update_ls,
    update_ls_if_changed, upsert_project, upsert_project_key, user_color_class, write_to_ls,
    LocalStorageProject, LocalStorageState,
};

mod haptics;
pub use haptics::{haptic, set_haptics_player, Haptic, HapticsPlayer};

mod session_flush;
pub use session_flush::{flush_session, set_session_flusher, SessionFlusher};

mod share_sheet;
pub use share_sheet::{
    open_external, set_native_file_sharer, set_native_opener, set_native_sharer, share_file,
    share_text, NativeFileSharer, NativeOpener, NativeSharer,
};

mod mail_link;
pub use mail_link::MailLink;

pub mod push;
pub use push::{native_push, read_push_token, set_native_push, NativePush};
#[cfg(not(target_arch = "wasm32"))]
pub use push::write_push_token;

mod push_sync;
pub use push_sync::{unregister_push, use_push_registration};

mod viewport;
pub use viewport::{css_viewport_vars, keyboard_inset, KEYBOARD_MIN_PX};

mod navigation_sync;
pub use navigation_sync::NavigationSync;

mod page_transition;
pub use page_transition::{nav_direction, route_depth, NavDirection, PageTransition};

mod overscroll;
pub use overscroll::use_overscroll;

mod pull_to_refresh;
pub use pull_to_refresh::{pending, PullToRefresh};

mod enter_next;
pub use enter_next::use_enter_advances_focus;

mod deep_link;
pub use deep_link::{DeepLinkListener, NativeDeepLinkReader};
#[cfg(any(target_os = "android", target_os = "ios"))]
pub use deep_link::{deep_link_listener_active, push_deep_link};

mod app_layout;
pub use app_layout::AppLayout;

mod bottom_dock;
pub use bottom_dock::BottomDock;

mod error_utils;
pub use error_utils::{
    error_key, error_message, is_claim_proof_error, is_client_outdated_error,
    is_email_not_verified, is_identity_taken_error, is_offline_error, is_participant_gone_error,
    is_payment_methods_stale_error, is_project_gone_error, is_user_gone_error,
};

mod membership;
pub use membership::{
    adopt_project_key, ensure_membership, identity_locked, leave_project_and_forget, ClaimOutcome,
    LEAVE_CONFIRM_MESSAGE, LEAVE_CONFIRM_TITLE,
};

mod confirm_modal;
pub use confirm_modal::ConfirmModal;

mod language_picker;
pub use language_picker::LanguagePicker;

pub mod offline_queue;
pub use offline_queue::{read_queue, write_queue, OpKind, QueuedOp};

mod queue_replay;
pub use queue_replay::{replay_queue, SyncFailure};

mod app_contexts;
pub use app_contexts::{
    use_app_contexts, AcceptedInvite, AuthResolved, Flash, InviteRetry, ProjectKey, UpdateRequired,
};

mod splash;
pub use splash::{use_client_ready, SplashScreen};

mod update_required;
pub use update_required::UpdateRequiredScreen;

#[cfg(target_arch = "wasm32")]
pub mod web_dom;

mod share_link;
pub use share_link::{
    copy_js, copy_text, parse_share_link, scheme_link_for, share_link_for, APP_BASE_URL,
    APP_SCHEME, READ_CLIPBOARD_JS,
};

// Test-only guards over files no compiler checks: the hand-written stylesheets and the iOS
// plist. Not `mod` outside tests — they contain assertions, not runtime code.
#[cfg(all(test, not(target_arch = "wasm32")))]
mod stylesheet_guard;

#[cfg(all(test, not(target_arch = "wasm32")))]
mod info_plist_guard;

#[cfg(test)]
pub(crate) mod test_dom;

#[cfg(test)]
pub(crate) mod test_fixtures;

pub type NativeClipboardReader = std::sync::Arc<dyn Fn() -> Option<String> + Send + Sync + 'static>;

mod scan;
pub use scan::{
    capture_and_scan, scan_error_key, scanner_available, set_native_scan, NativeScan, ScanError,
    ScanFields, ScanSource,
};
