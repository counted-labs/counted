use api::auth::auth_controller::{logout, me};
use dioxus::prelude::*;
use crate::tid;
use shared::Account;
use std::collections::VecDeque;

use crate::common::{
    account_color_class, app_version, error_message, flush_session, read_from_ls, remove_quarantined_ls,
    unregister_push, write_to_ls, AppHeader, AuthResolved, Avatar, LanguagePicker, ListFooter, ListRow,
    LocalStorageState, PullToRefresh, QueuedOp, RowTile, SizeClass, ThemePicker,
};
use crate::auth::delete_account_sheet::DeleteAccountSheet;
use crate::auth::erase_device::{check_erase, erase_device_stores, EraseCheck};
use crate::auth::erase_device_sheet::EraseDeviceSheet;
use crate::crypto::{decrypt_json, forget_decrypted};
use crate::friends::use_friends;
use crate::icons::{EyeIcon, FileTextIcon, HelpIcon, LockIcon, ScaleIcon, UsersIcon, WalletIcon, ICON_INLINE};
use crate::payment_methods::account_payment_methods;
use crate::route::Route;

const SOURCE_URL: &str = "https://github.com/counted-labs/counted";

/// The account key must go with the session in both directions: leaving it in localStorage after
/// the account is gone leaves a key that decrypts nothing and a UI that thinks it is signed in.
///
/// The language is deliberately *not* cleared. It belongs to the device as much as to the account,
/// and flipping the UI back to English at logout would read as a bug.
fn clear_local_session(
    mut ls_ctx: Signal<LocalStorageState>,
    mut account_enc_key_ctx: Signal<Option<[u8; 32]>>,
    mut auth_ctx: Signal<Option<Account>>,
) {
    let mut state = read_from_ls();
    state.account_key = None;
    state.account_key_owner = None;
    state.account_key_epoch = None;
    write_to_ls(&state);
    ls_ctx.set(state);
    remove_quarantined_ls();
    forget_decrypted();

    account_enc_key_ctx.set(None);
    auth_ctx.set(None);
}

/// Falls back to the email when there is no key to decrypt with (local store cleared): showing the
/// address beats showing nothing.
fn display_name_of(account: &Account, key: Option<[u8; 32]>) -> String {
    key.and_then(|k| decrypt_json::<String>(&k, &account.display_name).ok())
        .unwrap_or_else(|| account.email.clone())
}

/// Settings, for everyone, on one page: profile, sharing, security, preferences, help, then the
/// ways out (docs/plans/friends-settings-redesign.md, track B). The dock routes here
/// unconditionally, so an anonymous user reaches the preferences too.
#[component]
pub fn SettingsPage() -> Element {
    let nav = use_navigator();
    let auth_ctx = use_context::<Signal<Option<Account>>>();
    let account_enc_key_ctx = use_context::<Signal<Option<[u8; 32]>>>();
    let ls_ctx = use_context::<Signal<LocalStorageState>>();
    let auth_resolved = use_context::<Signal<AuthResolved>>();
    let mut loading = use_signal(|| false);
    let mut error_msg: Signal<Option<String>> = use_signal(|| None);
    let mut show_delete_confirm = use_signal(|| false);
    let mut refreshing_me = use_signal(|| false);
    let mut pending_ops = use_context::<Signal<VecDeque<QueuedOp>>>();
    let mut erase_check: Signal<Option<EraseCheck>> = use_signal(|| None);
    let mut erasing = use_signal(|| false);
    let friends = use_friends();

    let account = auth_ctx();

    let refresh_account = move |_: ()| {
        let mut auth_ctx = auth_ctx;
        spawn(async move {
            refreshing_me.set(true);
            if let Ok(a) = me().await {
                auth_ctx.set(a);
            }
            friends.reload();
            refreshing_me.set(false);
        });
    };

    let on_logout = move |_| {
        spawn(async move {
            loading.set(true);
            unregister_push().await;
            match logout().await {
                Ok(_) => {
                    flush_session();
                    clear_local_session(ls_ctx, account_enc_key_ctx, auth_ctx);
                    nav.replace(Route::ProjectsPage {});
                }
                Err(e) => {
                    error_msg.set(Some(error_message(&e)));
                    loading.set(false);
                }
            }
        });
    };

    let open_erase = move |_| {
        let Some(account) = auth_ctx() else { return };
        erase_check.set(Some(EraseCheck::Checking));
        let pending = pending_ops.read().len();
        let account_key = account_enc_key_ctx();
        spawn(async move {
            let check = check_erase(account, account_key, pending).await;
            if erase_check.peek().is_some() {
                erase_check.set(Some(check));
            }
        });
    };

    let on_erase = move |_| {
        spawn(async move {
            erasing.set(true);
            unregister_push().await;
            match logout().await {
                Ok(_) => {
                    flush_session();
                    erase_device_stores();
                    pending_ops.set(VecDeque::new());
                    clear_local_session(ls_ctx, account_enc_key_ctx, auth_ctx);
                    erase_check.set(None);
                    nav.replace(Route::ProjectsPage {});
                }
                Err(e) => {
                    error_msg.set(Some(error_message(&e)));
                    erase_check.set(None);
                    erasing.set(false);
                }
            }
        });
    };

    let on_deleted = move |_| {
        show_delete_confirm.set(false);
        clear_local_session(ls_ctx, account_enc_key_ctx, auth_ctx);
        nav.replace(Route::ProjectsPage {});
    };

    let (incoming, changed) = friends.current().map(|l| (l.incoming.len(), l.changed_count())).unwrap_or_default();

    rsx! {
        div { class: "container app-container bg-base-100 p-4 pb-24 max-w-md mx-auto flex flex-col gap-6 overflow-auto",
            PullToRefresh { on_refresh: refresh_account, busy: refreshing_me() }
            AppHeader {
                title: tid!("settings-title"),
                back_button_route: Route::ProjectsPage {},
                back: false,
            }

            if let Some(err) = error_msg() {
                div { role: "alert", class: "alert alert-error text-sm", "{err}" }
            }

            // `auth_ctx` is None both for an anonymous user and while `me()` is in flight, so
            // without `AuthResolved` this slot would flash the sign-in card at someone who is
            // signed in. Server and first client render both see `false`, so hydration agrees.
            if !auth_resolved().0 {
                crate::common::AccountCardSkeleton {}
            } else if let Some(account) = account.clone() {
                crate::auth::recovery_ui::PendingResetBanner {}
                ProfileGroup { name: display_name_of(&account, account_enc_key_ctx()), email: account.email.clone(), tint: account_color_class(account.id) }
                SharingGroup {
                    incoming,
                    changed,
                    methods: account_enc_key_ctx().map(|k| account_payment_methods(&account, &k).len()).unwrap_or(0),
                }
                if account.key_lifecycle {
                    crate::auth::security_card::SecurityCard { account: account.clone() }
                }
            } else {
                UpsellCard {}
                LockedSharingGroup {}
            }

            PreferencesGroup { synced: auth_resolved().0.then_some(account.is_some()) }
            HelpGroup {}

            if auth_resolved().0 && account.is_some() {
                ul { class: "list-group",
                    ListRow {
                        id: "logout-btn",
                        title: if loading() { tid!("account-logging-out") } else { tid!("account-logout") },
                        title_class: "font-medium text-secondary-content",
                        centred: true,
                        disabled: loading(),
                        onclick: on_logout,
                    }
                }
                section { class: "flex flex-col",
                    ul { class: "list-group",
                        ListRow {
                            id: "erase-device-btn",
                            title: tid!("erase-device-row"),
                            title_class: "font-medium text-error",
                            centred: true,
                            disabled: loading(),
                            onclick: open_erase,
                        }
                    }
                    ListFooter { {tid!("erase-device-row-hint")} }
                }
                section { class: "flex flex-col",
                    ul { class: "list-group",
                        ListRow {
                            id: "delete-account",
                            title: tid!("account-delete-title"),
                            title_class: "font-medium text-error",
                            centred: true,
                            disabled: loading(),
                            onclick: move |_| show_delete_confirm.set(true),
                        }
                    }
                    ListFooter { {tid!("account-delete-hint")} }
                }
            }

            p { class: "flex flex-col items-center gap-0.5 text-xs text-base-content/70 text-center",
                if let Some(version) = app_version() {
                    span { id: "app-version", "Counted" " {version}" }
                }
                a {
                    class: "link link-hover font-medium text-secondary-content",
                    href: SOURCE_URL,
                    target: "_blank",
                    rel: "noopener noreferrer",
                    {tid!("settings-open-source")}
                }
            }

            if let Some(check) = erase_check() {
                EraseDeviceSheet {
                    check,
                    erasing: erasing(),
                    on_close: move |_| {
                        if !erasing() {
                            erase_check.set(None);
                        }
                    },
                    on_confirm: on_erase,
                }
            }

            if show_delete_confirm() {
                DeleteAccountSheet {
                    on_close: move |_| show_delete_confirm.set(false),
                    on_deleted,
                }
            }
        }
    }
}

#[component]
fn ProfileGroup(name: String, email: String, tint: &'static str) -> Element {
    rsx! {
        ul { class: "list-group",
            ListRow {
                id: "settings-profile",
                title: name.clone(),
                title_class: "font-semibold text-lg",
                subtitle: email,
                subtitle_class: "text-base-content/70",
                Avatar { initials: crate::common::local_storage::initials(&name), color_class: tint.to_string(), size: SizeClass::W12 }
            }
        }
    }
}

#[component]
fn SharingGroup(incoming: usize, changed: usize, methods: usize) -> Element {
    rsx! {
        section { class: "flex flex-col",
            h2 { class: "list-header", {tid!("settings-group-sharing")} }
            ul { class: "list-group", style: "--separator-inset: 3.75rem",
                ListRow {
                    id: "settings-friends",
                    title: tid!("friends-title"),
                    subtitle: (changed > 0).then(|| tid!("settings-friends-changed", count: changed)),
                    subtitle_class: "text-error font-medium",
                    to: Route::FriendsPage {},
                    trailing: (incoming > 0).then(|| rsx! {
                        span { class: "badge badge-sm badge-soft badge-primary font-semibold tabular-nums", "{incoming}" }
                    }),
                    RowTile { tint: "bg-tile-violet text-tile-violet-content", UsersIcon {} }
                }
                ListRow {
                    id: "settings-payment-methods",
                    title: tid!("payment-methods-title"),
                    subtitle: tid!("payment-methods-row-hint"),
                    subtitle_class: "text-base-content/70",
                    to: Route::PaymentMethodsPage {},
                    trailing: (methods > 0).then(|| rsx! {
                        span { class: "text-base-content/70 tabular-nums", "{methods}" }
                    }),
                    RowTile { tint: "bg-tile-amber text-tile-amber-content", WalletIcon {} }
                }
            }
        }
    }
}

/// What an account unlocks, in the slots it would fill, with the one call to action above: the
/// rows are inert. The account is free and the card says so: a locked look must never read as a
/// paid tier.
#[component]
fn UpsellCard() -> Element {
    let nav = use_navigator();
    rsx! {
        div { class: "flex flex-col shrink-0 rounded-box bg-base-100 p-4 gap-3",
            h2 { class: "flex flex-wrap items-center gap-2 font-display font-bold text-lg",
                {tid!("settings-upsell-title")}
                span { class: "badge badge-primary badge-soft badge-sm", {tid!("settings-upsell-free")} }
            }
            p { class: "text-sm text-base-content/70", {tid!("settings-upsell-short")} }
            div { class: "flex gap-2",
                button {
                    id: "settings-login",
                    r#type: "button",
                    class: "btn flex-1 whitespace-nowrap",
                    onclick: move |_| {
                        nav.push(Route::LoginPage {});
                    },
                    {tid!("login-submit")}
                }
                button {
                    id: "settings-register",
                    r#type: "button",
                    class: "btn btn-primary flex-1 whitespace-nowrap",
                    onclick: move |_| {
                        nav.push(Route::RegisterPage {});
                    },
                    {tid!("register-submit")}
                }
            }
        }
    }
}

#[component]
fn LockedSharingGroup() -> Element {
    rsx! {
        section { class: "flex flex-col",
            h2 { class: "list-header flex items-center justify-between gap-2",
                {tid!("settings-group-sharing")}
                span { class: "badge badge-ghost badge-sm gap-1 normal-case tracking-normal",
                    LockIcon { size: ICON_INLINE }
                    {tid!("settings-locked-group")}
                }
            }
            ul { class: "list-group opacity-60", style: "--separator-inset: 3.75rem",
                ListRow {
                    title: tid!("friends-title"),
                    subtitle: tid!("settings-locked-friends-short"),
                    subtitle_class: "text-base-content/70",
                    RowTile { tint: "bg-tile-violet text-tile-violet-content", UsersIcon {} }
                }
                ListRow {
                    title: tid!("payment-methods-title"),
                    subtitle: tid!("settings-locked-payment-short"),
                    subtitle_class: "text-base-content/70",
                    RowTile { tint: "bg-tile-amber text-tile-amber-content", WalletIcon {} }
                }
            }
        }
    }
}

/// `synced` is `None` until `me()` has answered: the two captions say opposite things, and guessing
/// would tell a signed-in user their choice stays on this device.
#[component]
fn PreferencesGroup(synced: Option<bool>) -> Element {
    rsx! {
        section { class: "flex flex-col",
            h2 { class: "list-header", {tid!("settings-preferences")} }
            div { class: "list-group",
                fieldset { class: "fieldset px-4 py-3",
                    LanguagePicker {}
                    match synced {
                        Some(true) => rsx! { p { class: "label whitespace-normal", {tid!("settings-preferences-synced")} } },
                        Some(false) => rsx! { p { class: "label whitespace-normal", {tid!("settings-preferences-local")} } },
                        None => rsx! {},
                    }
                    ThemePicker {}
                }
            }
        }
    }
}

#[component]
fn HelpGroup() -> Element {
    rsx! {
        section { class: "flex flex-col",
            h2 { class: "list-header", {tid!("settings-group-help")} }
            ul { class: "list-group", style: "--separator-inset: 3.75rem",
                ListRow {
                    title: tid!("nav-help"),
                    to: Route::HelpPage {},
                    RowTile { tint: "bg-tile-blue text-tile-blue-content", HelpIcon {} }
                }
                ListRow {
                    title: tid!("nav-privacy"),
                    to: Route::PrivacyPage {},
                    RowTile { tint: "bg-base-200 text-base-content/70", EyeIcon {} }
                }
                ListRow {
                    title: tid!("nav-terms"),
                    to: Route::TermsPage {},
                    RowTile { tint: "bg-base-200 text-base-content/70", FileTextIcon {} }
                }
                ListRow {
                    title: tid!("nav-legal"),
                    to: Route::LegalNoticePage {},
                    RowTile { tint: "bg-base-200 text-base-content/70", ScaleIcon {} }
                }
            }
        }
    }
}
