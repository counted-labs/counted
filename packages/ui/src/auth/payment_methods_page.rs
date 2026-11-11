use dioxus::prelude::*;
use shared::{Account, PaymentMethod};
use uuid::Uuid;

use crate::auth::payment_method_sheet::PaymentMethodSheet;
use crate::common::{
    haptic, AppHeader, AuthResolved, GroupedListSkeleton, Haptic, ListFooter, ListRow, Mascot, MascotPose, RowTile,
};
use crate::icons::{BankIcon, PlusIcon, WalletIcon, ICON_INLINE};
use crate::payment_methods::{
    masked_tail, masked_value, method_display_name, method_kind_name, writable_methods, MAX_METHODS, OTHER_KIND,
};
use crate::route::Route;
use crate::tid;

#[derive(Clone, PartialEq)]
enum Editing {
    New,
    Existing(PaymentMethod),
}

fn kind_label(method: &PaymentMethod) -> String {
    method_kind_name(method).unwrap_or_else(|| tid!("payment-method-kind-other"))
}

fn tile_of(method: &PaymentMethod) -> (&'static str, Element) {
    match method.kind.as_str() {
        "iban" => ("bg-tile-blue text-tile-blue-content", rsx! { BankIcon {} }),
        OTHER_KIND => ("bg-base-200 text-base-content/70", rsx! { WalletIcon {} }),
        _ => ("bg-tile-violet text-tile-violet-content", rsx! { WalletIcon {} }),
    }
}

/// How the holder wants to be paid back (docs/plans/friends-settings-redesign.md §2.3). The list
/// renders only what the account stores; adding and editing happen in `PaymentMethodSheet`.
#[component]
pub fn PaymentMethodsPage() -> Element {
    let nav = use_navigator();
    let auth_ctx = use_context::<Signal<Option<Account>>>();
    let account_enc_key_ctx = use_context::<Signal<Option<[u8; 32]>>>();
    let auth_resolved = use_context::<Signal<AuthResolved>>();
    let mut editing = use_signal(|| None::<(Uuid, Editing)>);

    let mut open = move |what: Editing| {
        if let Some(account) = auth_ctx.peek().as_ref() {
            editing.set(Some((account.id, what)));
        }
    };

    let account = auth_ctx();
    let key = account_enc_key_ctx();

    rsx! {
        div { class: "container app-container bg-base-100 p-4 pb-24 max-w-md mx-auto flex flex-col gap-4 overflow-auto",
            AppHeader { title: tid!("payment-methods-title"), back_button_route: Route::SettingsPage {} }

            if !auth_resolved().0 {
                GroupedListSkeleton { rows: 3 }
            } else if let Some(account) = account {
                match key.and_then(|key| writable_methods(&account, &key, account.id).map(|methods| (key, methods))) {
                    None => rsx! {
                        p { id: "payment-methods-key-missing", class: "list-group p-4 text-sm", {tid!("payment-methods-key-missing")} }
                    },
                    Some((key, methods)) => {
                        let account_id = account.id;
                        rsx! {
                            if methods.is_empty() {
                                div { id: "payment-methods-empty", class: "flex flex-col items-center gap-3 pt-10 text-center",
                                    Mascot { pose: MascotPose::Empty }
                                    h2 { class: "font-display font-bold text-lg", {tid!("payment-methods-empty-title")} }
                                    p { class: "text-sm text-base-content/70 max-w-60", {tid!("payment-methods-empty-body")} }
                                    button {
                                        id: "payment-method-add",
                                        r#type: "button",
                                        class: "btn btn-primary mt-1",
                                        aria_haspopup: "dialog",
                                        onclick: move |_| {
                                            haptic(Haptic::Light);
                                            open(Editing::New);
                                        },
                                        PlusIcon { size: ICON_INLINE }
                                        {tid!("payment-method-add")}
                                    }
                                }
                            } else {
                                section { class: "flex flex-col",
                                    ListFooter { {tid!("payment-methods-intro")} }
                                    ul { id: "payment-methods-list", class: "list-group mt-1.5", style: "--separator-inset: 3.75rem",
                                        for (i , method) in methods.iter().enumerate() {
                                            {
                                                let (tint, icon) = tile_of(method);
                                                let opened = method.clone();
                                                let sr = masked_tail(method).map(|last| tid!("payment-method-masked", kind: kind_label(method), last: last));
                                                rsx! {
                                                    ListRow {
                                                        key: "{i}",
                                                        id: format!("payment-method-{i}"),
                                                        class: "payment-method-row",
                                                        title: method_display_name(method),
                                                        subtitle: masked_value(method),
                                                        subtitle_class: "font-mono text-base-content/70",
                                                        subtitle_sr: sr,
                                                        chevron: true,
                                                        long_press: format!("long-press-payment-method-{i}"),
                                                        onclick: move |_| open(Editing::Existing(opened.clone())),
                                                        trailing: method.shared.then(|| rsx! {
                                                            span { class: "badge badge-sm badge-soft badge-primary font-semibold", {tid!("payment-method-shared-pill")} }
                                                        }),
                                                        RowTile { tint, {icon} }
                                                    }
                                                }
                                            }
                                        }
                                        if methods.len() < MAX_METHODS {
                                            ListRow {
                                                id: "payment-method-add",
                                                title: tid!("payment-method-add"),
                                                title_class: "font-medium text-secondary-content",
                                                onclick: move |_| open(Editing::New),
                                                RowTile { tint: "bg-tile-green text-tile-green-content", PlusIcon {} }
                                            }
                                        }
                                    }
                                    ListFooter { id: "payment-methods-footnote", {tid!("payment-methods-footnote")} }
                                }
                            }
                            if let Some((_, what)) = editing().filter(|(opened_for, _)| *opened_for == account_id) {
                                PaymentMethodSheet {
                                    original: match what {
                                        Editing::New => None,
                                        Editing::Existing(method) => Some(method),
                                    },
                                    account_id,
                                    account_key: key,
                                    on_close: move |_| editing.set(None),
                                }
                            }
                        }
                    }
                }
            } else {
                div { class: "flex flex-col shrink-0 rounded-box bg-base-100 p-4 gap-2",
                    h2 { class: "font-display font-bold text-lg", {tid!("settings-anonymous-title")} }
                    p { class: "text-sm text-base-content/70", {tid!("settings-locked-payment-short")} }
                    div { class: "flex justify-end",
                        button {
                            r#type: "button",
                            class: "btn btn-primary",
                            onclick: move |_| { nav.push(Route::LoginPage {}); },
                            {tid!("login-submit")}
                        }
                    }
                }
            }
        }
    }
}
