use api::auth::auth_controller::{delete_account, logout, me};
use dioxus::prelude::*;
use crate::tid;
use shared::Account;

use crate::common::{
    error_message, flush_session, format_date, read_from_ls, unregister_push, write_to_ls,
    AppHeader, AuthResolved, ConfirmModal, LanguagePicker, LocalStorageState, PullToRefresh,
};
use crate::auth::{LockedFeatureCard, PaymentMethodsCard};
use crate::crypto::decrypt_json;
use crate::icons::{DollarIcon, RightArrowIcon, UserIcon};
use crate::route::Route;

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
    write_to_ls(&state);
    ls_ctx.set(state);

    account_enc_key_ctx.set(None);
    auth_ctx.set(None);
}

/// Falls back to the email when there is no key to decrypt with — a session restored from the
/// cookie has none, and showing the address beats showing nothing.
fn display_name_of(account: &Account, key: Option<[u8; 32]>) -> String {
    key.and_then(|k| decrypt_json::<String>(&k, &account.display_name).ok())
        .unwrap_or_else(|| account.email.clone())
}

/// Settings, for everyone. The dock routes here unconditionally, so this is the one page an
/// anonymous user can reach to change a preference — hence preferences first, account second.
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

    let account = auth_ctx();

    // The same call `use_app_contexts` makes at boot; nothing else on this page is fetched.
    let refresh_account = move |_: ()| {
        let mut auth_ctx = auth_ctx;
        spawn(async move {
            refreshing_me.set(true);
            if let Ok(a) = me().await {
                auth_ctx.set(a);
            }
            refreshing_me.set(false);
        });
    };

    let on_logout = move |_| async move {
        loading.set(true);
        unregister_push().await;
        match logout().await {
            Ok(_) => {
                flush_session();
                clear_local_session(ls_ctx, account_enc_key_ctx, auth_ctx);
                nav.push(Route::ProjectsPage {});
            }
            Err(e) => {
                error_msg.set(Some(error_message(&e)));
                loading.set(false);
            }
        }
    };

    let on_delete = move |_| {
        show_delete_confirm.set(false);
        spawn(async move {
            loading.set(true);
            match delete_account().await {
                Ok(_) => {
                    clear_local_session(ls_ctx, account_enc_key_ctx, auth_ctx);
                    nav.push(Route::ProjectsPage {});
                }
                Err(e) => {
                    error_msg.set(Some(error_message(&e)));
                    loading.set(false);
                }
            }
        });
    };

    rsx! {
        div { class: "container app-container bg-base-100 p-4 pb-24 max-w-md mx-auto flex flex-col gap-4 overflow-auto",
            PullToRefresh { on_refresh: refresh_account, busy: refreshing_me() }
            AppHeader {
                title: tid!("settings-title"),
                back_button_route: Route::ProjectsPage {},
            }

            if let Some(err) = error_msg() {
                div { role: "alert", class: "alert alert-error text-sm", "{err}" }
            }

            div { class: "card bg-base-100 shadow-soft",
                fieldset { class: "fieldset card-body",
                    legend { class: "fieldset-legend", {tid!("settings-preferences")} }
                    LanguagePicker {}
                    // Only once `me()` has answered — the two say opposite things, and guessing
                    // would tell a signed-in user their choice stays on this device.
                    if auth_resolved().0 {
                        p { class: "label",
                            if account.is_some() {
                                {tid!("settings-preferences-synced")}
                            } else {
                                {tid!("settings-preferences-local")}
                            }
                        }
                    }
                }
            }

            // `auth_ctx` is None both for an anonymous user and while `me()` is in flight, so
            // without `AuthResolved` this slot would flash the sign-in card at someone who is
            // signed in. Server and first client render both see `false`, so hydration agrees.
            if !auth_resolved().0 {
                div { class: "skeleton h-48 w-full", aria_hidden: "true" }
            } else if let Some(account) = account {
                div { class: "card bg-base-100 shadow-soft",
                    div { class: "card-body gap-2",
                        ul { class: "list",
                            li { class: "list-row",
                                span { class: "text-xs text-base-content/70 uppercase font-semibold",
                                    {tid!("field-name")}
                                }
                                span { class: "font-medium",
                                    {display_name_of(&account, account_enc_key_ctx())}
                                }
                            }
                            li { class: "list-row",
                                span { class: "text-xs text-base-content/70 uppercase font-semibold",
                                    {tid!("field-email")}
                                }
                                span { class: "font-medium", "{account.email}" }
                            }
                            li { class: "list-row",
                                span { class: "text-xs text-base-content/70 uppercase font-semibold",
                                    {tid!("account-member-since")}
                                }
                                // Was a hardcoded `%d/%m/%Y`, which reads as a different date in en-US.
                                span { class: "font-medium", {format_date(account.created_at.date())} }
                            }
                        }
                        div { class: "card-actions justify-end mt-2",
                            button {
                                id: "logout-btn",
                                r#type: "button",
                                class: "btn btn-error btn-outline",
                                disabled: loading(),
                                onclick: on_logout,
                                if loading() {
                                    {tid!("account-logging-out")}
                                } else {
                                    {tid!("account-logout")}
                                }
                            }
                        }
                    }
                }

                Link {
                    id: "settings-friends",
                    to: Route::FriendsPage {},
                    class: "card bg-base-100 shadow-soft",
                    div { class: "card-body flex-row items-center justify-between gap-3",
                        div { class: "flex flex-col gap-1 min-w-0",
                            h2 { class: "card-title text-base",
                                UserIcon {}
                                {tid!("friends-title")}
                            }
                            p { class: "text-sm text-base-content/70", {tid!("settings-friends-hint")} }
                        }
                        RightArrowIcon {}
                    }
                }

                PaymentMethodsCard {}

                div { class: "card bg-base-100 shadow-soft border border-error/30",
                    div { class: "card-body gap-3",
                        h2 { class: "card-title text-base text-error", {tid!("account-delete-title")} }
                        p { class: "text-sm text-base-content/70",
                            {tid!("account-delete-warning")}
                        }
                        div { class: "card-actions justify-end",
                            button {
                                id: "delete-account",
                                r#type: "button",
                                class: "btn btn-error",
                                disabled: loading(),
                                onclick: move |_| show_delete_confirm.set(true),
                                {tid!("account-delete-title")}
                            }
                        }
                    }
                }
            } else {
                // Same slots a signed-in user gets — account, friends, payment details — so what
                // an account unlocks is shown in place rather than described. The account is free
                // and the copy says so: a locked look must never read as a paid tier.
                div { class: "card bg-base-100 shadow-soft",
                    div { class: "card-body gap-2",
                        h2 { class: "card-title text-base",
                            {tid!("settings-upsell-title")}
                            span { class: "badge badge-primary badge-soft badge-sm", {tid!("settings-upsell-free")} }
                        }
                        p { class: "text-sm text-base-content/70",
                            {tid!("settings-upsell-body")}
                        }
                        div { class: "card-actions justify-end",
                            button {
                                id: "settings-login",
                                r#type: "button",
                                class: "btn btn-ghost",
                                onclick: move |_| {
                                    nav.push(Route::LoginPage {});
                                },
                                {tid!("login-submit")}
                            }
                            button {
                                id: "settings-register",
                                r#type: "button",
                                class: "btn btn-primary",
                                onclick: move |_| {
                                    nav.push(Route::RegisterPage {});
                                },
                                {tid!("register-submit")}
                            }
                        }
                    }
                }

                LockedFeatureCard {
                    icon: rsx! { UserIcon {} },
                    title: tid!("friends-title"),
                    body: tid!("settings-locked-friends"),
                }
                LockedFeatureCard {
                    icon: rsx! { DollarIcon {} },
                    title: tid!("settings-payment-methods"),
                    body: tid!("settings-locked-payment-methods"),
                }
            }

            // Moved off the projects header dropdown: these are read-once pages, not tools, and a
            // menu on the list screen was the wrong place for them.
            div { class: "card bg-base-100 shadow-soft",
                ul { class: "menu w-full",
                    li { class: "menu-title", {tid!("settings-about")} }
                    li {
                        Link { to: Route::HelpPage {}, {tid!("nav-help")} }
                    }
                    li {
                        Link { to: Route::PrivacyPage {}, {tid!("nav-privacy")} }
                    }
                    li {
                        Link { to: Route::TermsPage {}, {tid!("nav-terms")} }
                    }
                    li {
                        Link { to: Route::LegalNoticePage {}, {tid!("nav-legal")} }
                    }
                }
            }

            if show_delete_confirm() {
                ConfirmModal {
                    title: tid!("account-delete-confirm-title"),
                    message: tid!("account-delete-confirm-message"),
                    confirm_label: tid!("delete"),
                    on_cancel: move |_| show_delete_confirm.set(false),
                    on_confirm: on_delete,
                }
            }
        }
    }
}
