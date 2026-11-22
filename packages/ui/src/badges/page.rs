use dioxus::prelude::*;
use shared::time::Date;
use shared::Account;

use super::{set_off, Badge, Badges, Family, CATALOGUE, FAMILIES};
use crate::common::{
    format_date, haptic, use_modal_focus, AppHeader, AuthResolved, GroupedListSkeleton, Haptic, ListFooter, SheetBar,
    SHEET_TITLE,
};
use crate::icons::{
    ArchiveIcon, CameraIcon, CloudOffIcon, ImportIcon, KeyIcon, LockIcon, PieChartIcon, PlaneIcon, RefreshIcon,
    RepeatIcon, ShieldIcon, TagIcon, TargetIcon, UserPlusIcon, UsersIcon, WalletIcon, ICON_INLINE,
};
use crate::route::Route;
use crate::tid;

fn icon_of(id: &str) -> Element {
    match id {
        super::PREMIER_TICKET => rsx! { CameraIcon {} },
        super::TOURISTE => rsx! { PlaneIcon {} },
        super::RECURRENT => rsx! { RepeatIcon {} },
        super::DEMENAGEMENT => rsx! { ImportIcon {} },
        super::ARCHIVISTE => rsx! { ArchiveIcon {} },
        super::HORS_LIGNE => rsx! { CloudOffIcon {} },
        super::SUR_MESURE => rsx! { TagIcon {} },
        super::CLE_DE_SECOURS => rsx! { KeyIcon {} },
        super::CODE_VERIFIE => rsx! { ShieldIcon {} },
        super::CLE_CHANGEE => rsx! { RefreshIcon {} },
        super::A_PLUSIEURS => rsx! { UsersIcon {} },
        super::PREMIER_AMI => rsx! { UserPlusIcon {} },
        super::COORDONNEES_PARTAGEES => rsx! { WalletIcon {} },
        super::PREMIER_BUDGET => rsx! { PieChartIcon {} },
        _ => rsx! { TargetIcon {} },
    }
}

fn tint_of(family: Family) -> &'static str {
    match family {
        Family::Discovery => "bg-tile-blue text-tile-blue-content",
        Family::Security => "bg-tile-green text-tile-green-content",
        Family::Together => "bg-tile-violet text-tile-violet-content",
        Family::Personal => "bg-tile-amber text-tile-amber-content",
    }
}

/// The collection (docs/plans/gamification.md §5): every badge, earned in colour, locked greyed,
/// and the switch that hides the feature. Seen by the account holder alone.
#[component]
pub fn BadgesPage() -> Element {
    let auth = use_context::<Signal<Option<Account>>>();
    let account_key = use_context::<Signal<Option<[u8; 32]>>>();
    let auth_resolved = use_context::<Signal<AuthResolved>>();
    let Badges(state) = use_context::<Badges>();
    let mut opened = use_signal(|| None::<Badge>);

    let current = state();
    let shown = auth().is_some() && current.account == auth().as_ref().map(|a| a.id);
    let earned = |b: &Badge| shown.then(|| current.earned.get(b.id).cloned()).flatten();

    rsx! {
        div { class: "container app-container bg-base-100 p-4 pb-24 max-w-md mx-auto flex flex-col gap-4 overflow-auto",
            AppHeader { title: tid!("badges-title"), back_button_route: Route::SettingsPage {} }

            if !auth_resolved().0 {
                GroupedListSkeleton { rows: 3 }
            } else if auth().is_some() {
                if !current.off {
                    ShownChoice {}
                    div { class: "flex items-baseline justify-between px-1",
                        h2 { class: "list-header", {tid!("badges-collection")} }
                        span { id: "badges-count", class: "text-sm text-base-content/70 tabular-nums",
                            {tid!("badges-row-count", earned: current.count() as i64, total: CATALOGUE.len() as i64)}
                        }
                    }
                    for (family , key) in FAMILIES {
                        section { class: "flex flex-col gap-2",
                            h3 { class: "list-header", {tid!(key)} }
                            div { class: "grid grid-cols-3 gap-2",
                                for b in CATALOGUE.iter().filter(|b| b.family == family).copied() {
                                    BadgeTile {
                                        key: "{b.id}",
                                        badge: b,
                                        earned: earned(&b).is_some(),
                                        on_open: move |b| opened.set(Some(b)),
                                    }
                                }
                            }
                        }
                    }
                }
                section { class: "flex flex-col",
                    div { class: "list-group",
                        label { class: "flex items-center gap-3 min-h-16 px-4 py-2 cursor-pointer",
                            span { class: "flex-1 min-w-0",
                                span { class: "block font-medium", {tid!("badges-show")} }
                                span { class: "block text-xs text-base-content/70 leading-snug", {tid!("badges-show-hint")} }
                            }
                            input {
                                id: "badges-switch",
                                r#type: "checkbox",
                                role: "switch",
                                class: "toggle toggle-primary",
                                checked: !current.off,
                                onchange: move |e| {
                                    set_off(state, !e.checked());
                                    if let (Some(account), Some(key)) = (auth.peek().clone(), account_key.peek().as_ref().copied()) {
                                        crate::preferences::push_badges(auth, &account, key);
                                    }
                                },
                            }
                        }
                    }
                    ListFooter { {tid!("badges-private")} }
                }
                if let Some(b) = opened() {
                    BadgeSheet { badge: b, earned_on: earned(&b), on_close: move |_| opened.set(None) }
                }
            }
        }
    }
}

#[component]
fn BadgeTile(badge: Badge, earned: bool, on_open: EventHandler<Badge>) -> Element {
    let long_press = format!("long-press-badge-{}", badge.id);
    let tint = if earned { tint_of(badge.family) } else { "bg-base-200 text-base-content/50" };
    let label = if earned { tid!(badge.name) } else { format!("{} · {}", tid!(badge.name), tid!("badges-locked")) };
    rsx! {
        div { class: "relative",
            button {
                id: "badge-{badge.id}",
                "data-long-press": long_press.clone(),
                "data-earned": if earned { "true" } else { "false" },
                r#type: "button",
                class: "row-press w-full h-full flex flex-col items-center gap-1.5 rounded-box bg-base-200/60 px-1 py-3 text-center",
                aria_haspopup: "dialog",
                aria_label: label,
                onclick: move |_| {
                    haptic(Haptic::Light);
                    on_open.call(badge);
                },
                span { class: "size-11 rounded-full grid place-items-center {tint}", {icon_of(badge.id)} }
                span { class: if earned { "text-xs font-medium leading-tight" } else { "text-xs leading-tight text-base-content/60" },
                    {tid!(badge.name)}
                }
                if !earned {
                    span { class: "absolute top-2 right-2 text-base-content/50", aria_hidden: "true", LockIcon { size: ICON_INLINE } }
                }
            }
            button {
                id: long_press,
                r#type: "button",
                hidden: true,
                tabindex: "-1",
                onclick: move |_| {
                    haptic(Haptic::Medium);
                    on_open.call(badge);
                },
            }
        }
    }
}

/// What a badge means, and when it was earned or how to earn it. A content sheet: no confirm.
#[component]
fn BadgeSheet(badge: Badge, earned_on: Option<String>, on_close: EventHandler<()>) -> Element {
    use_modal_focus("badge-sheet");
    let date = earned_on.as_deref().and_then(Date::parse_ymd);
    let tint = if date.is_some() { tint_of(badge.family) } else { "bg-base-200 text-base-content/50" };
    rsx! {
        div {
            id: "badge-sheet",
            class: "modal modal-open modal-bottom sm:modal-middle",
            role: "dialog",
            aria_modal: "true",
            aria_labelledby: "badge-sheet-title",
            tabindex: "-1",
            div { class: "modal-box max-w-md p-0 flex flex-col",
                SheetBar { close_id: "badge-sheet-close", on_close: move |_| on_close.call(()),
                    h2 { id: "badge-sheet-title", class: SHEET_TITLE, {tid!(badge.name)} }
                }
                div { class: "flex flex-col gap-3 px-5 pt-1 pb-6",
                    div { class: "flex items-center gap-3",
                        span { class: "size-12 shrink-0 rounded-full grid place-items-center {tint}", {icon_of(badge.id)} }
                        p { id: "badge-sheet-status", class: "font-semibold",
                            match date {
                                Some(d) => tid!("badges-earned-on", date: format_date(d)),
                                None => tid!("badges-locked"),
                            }
                        }
                    }
                    p { class: "text-sm text-base-content/70", {tid!(badge.how)} }
                }
            }
            div { class: "modal-backdrop", onclick: move |_| on_close.call(()) }
        }
    }
}

/// « Badge affiché » (docs/plans/gamification.md §5): one shown badge or none, among those this
/// account has earned. Hidden while the server signs nothing.
#[component]
fn ShownChoice() -> Element {
    let auth = use_context::<Signal<Option<Account>>>();
    let account_key = use_context::<Signal<Option<[u8; 32]>>>();
    let ls = use_context::<Signal<crate::common::LocalStorageState>>();
    let keys = use_resource(super::shown::keys);
    let (Some(account), Some(key)) = (auth(), account_key()) else { return rsx! {} };
    let offered = super::shown::eligible(&account, crate::common::local_today());
    if offered.is_empty() || keys.read().clone().flatten().is_none() {
        return rsx! {};
    }
    let chosen = super::shown::choice(&account, &key);
    let mut options: Vec<(Option<&'static str>, String)> = vec![(None, tid!("badges-shown-none"))];
    for (id, years) in offered {
        options.push((Some(id), super::members_label(id, years)));
    }
    rsx! {
        section { class: "flex flex-col",
            h2 { class: "list-header", {tid!("badges-shown")} }
            div { class: "list-group", role: "radiogroup", aria_label: tid!("badges-shown"),
                for (id , text) in options {
                    {
                        let selected = chosen.as_deref() == id;
                        let account = account.clone();
                        rsx! {
                            button {
                                key: "{text}",
                                id: format!("badges-shown-{}", id.unwrap_or("none")),
                                r#type: "button",
                                role: "radio",
                                aria_checked: if selected { "true" } else { "false" },
                                class: "row-press flex w-full items-center gap-3 min-h-11 px-4 py-2.5 text-left",
                                onclick: move |_| {
                                    if selected {
                                        return;
                                    }
                                    haptic(Haptic::Selection);
                                    crate::preferences::push_shown(auth, &account, key, ls.peek().clone(), id.map(str::to_string));
                                },
                                span { class: "flex-1", "{text}" }
                                if selected {
                                    span { class: "text-primary", aria_hidden: "true", crate::icons::CheckIcon { size: ICON_INLINE } }
                                }
                            }
                        }
                    }
                }
            }
            ListFooter { {tid!("badges-shown-hint")} }
        }
    }
}
