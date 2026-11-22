use dioxus::prelude::*;
use shared::time::DateTime;
use shared::User;
use uuid::Uuid;

use super::shown::{self, ANNIVERSAIRE};
use crate::common::{
    haptic, initials, use_modal_focus, user_color_class, Avatar, AvatarGroup, Haptic, SheetBar, SizeClass, SHEET_TITLE,
};
use crate::decrypted::user_name_opt;
use crate::icons::{ChevronRightIcon, ICON_INLINE};
use crate::tid;

type Shown = Option<(&'static str, u8)>;

pub fn label(id: &str, years: u8) -> String {
    if id == ANNIVERSAIRE {
        format!("{} · {years}", tid!("badge-anniversaire"))
    } else {
        tid!("badge-premiere-heure")
    }
}

/// The project page's avatar group, made a button that opens the member sheet
/// (docs/plans/gamification.md §5).
#[component]
pub fn MembersButton(users: Vec<User>, encryption_key: [u8; 32], on_open: EventHandler<()>) -> Element {
    let count = users.len() as i64;
    rsx! {
        button {
            id: "members-button",
            r#type: "button",
            class: "row-press flex items-center gap-2 rounded-full bg-base-200/60 py-1 pl-1 pr-3",
            aria_haspopup: "dialog",
            onclick: move |_| {
                haptic(Haptic::Light);
                on_open.call(());
            },
            AvatarGroup { users: users.clone(), encryption_key: Some(encryption_key), size: SizeClass::W10 }
            span { class: "text-sm text-base-content/70", {tid!("move-members", count: count)} }
            span { class: "text-base-content/50", aria_hidden: "true", ChevronRightIcon { size: ICON_INLINE } }
        }
    }
}

/// Names and, at most, the one badge each member chose to show, once both of the server's
/// signatures hold for that very membership. No amount, no action.
#[component]
pub fn MembersSheet(users: Vec<User>, project_id: Uuid, encryption_key: [u8; 32], me: Option<i32>, on_close: EventHandler<()>) -> Element {
    use_modal_focus("members-sheet");
    let keys = use_resource(shown::keys);
    let mut opened = use_signal(|| Shown::None);
    let now = DateTime::now_utc().unix_secs();
    let keys_now = keys.read().clone().flatten();
    let rows: Vec<(User, String, Shown)> = users
        .iter()
        .map(|u| {
            let badge = keys_now.as_ref().and_then(|k| shown::verified(k, u, project_id, &encryption_key, now));
            (u.clone(), user_name_opt(Some(&encryption_key), u), badge)
        })
        .collect();
    let any_badge = rows.iter().any(|(_, _, b)| b.is_some());

    rsx! {
        div {
            id: "members-sheet",
            class: "modal modal-open modal-bottom sm:modal-middle",
            role: "dialog",
            aria_modal: "true",
            aria_labelledby: "members-sheet-title",
            tabindex: "-1",
            div { class: "modal-box max-w-md p-0 flex flex-col",
                SheetBar { close_id: "members-sheet-close", on_close: move |_| on_close.call(()),
                    h2 { id: "members-sheet-title", class: SHEET_TITLE, {tid!("members-title")} }
                }
                div { class: "flex-1 overflow-y-auto flex flex-col gap-2 px-5 pt-1 pb-6",
                    ul { class: "list-group",
                        for (user , name , badge) in rows {
                            li { key: "{user.id}", class: "flex items-center gap-3 px-4 py-2.5",
                                Avatar { initials: initials(&name), color_class: user_color_class(user.id).to_string(), size: SizeClass::W8 }
                                span { class: "flex-1 min-w-0 flex flex-wrap items-center gap-1.5",
                                    span { class: "break-words", "{name}" }
                                    if Some(user.id) == me {
                                        span { class: "badge badge-sm badge-soft", {tid!("participants-you-badge")} }
                                    }
                                    if let Some((id , years)) = badge {
                                        button {
                                            id: "member-badge-{user.id}",
                                            r#type: "button",
                                            class: "badge badge-sm badge-soft badge-primary relative after:absolute after:-inset-3",
                                            aria_haspopup: "dialog",
                                            onclick: move |_| {
                                                haptic(Haptic::Light);
                                                opened.set(Some((id, years)));
                                            },
                                            {label(id, years)}
                                        }
                                    }
                                }
                            }
                        }
                    }
                    if any_badge {
                        p { class: "text-xs text-base-content/60 px-1", {tid!("members-hint")} }
                    }
                }
            }
            div { class: "modal-backdrop", onclick: move |_| on_close.call(()) }
            if let Some((id , years)) = opened() {
                ShownBadgeSheet { id, years, on_close: move |_| opened.set(None) }
            }
        }
    }
}

#[component]
fn ShownBadgeSheet(id: &'static str, years: u8, on_close: EventHandler<()>) -> Element {
    use_modal_focus("shown-badge-sheet");
    let title = if id == ANNIVERSAIRE { tid!("badge-anniversaire") } else { tid!("badge-premiere-heure") };
    let fact = if id == ANNIVERSAIRE { tid!("badge-anniversaire-how", years: years as i64) } else { tid!("badge-premiere-heure-how") };
    rsx! {
        div {
            id: "shown-badge-sheet",
            class: "modal modal-open modal-bottom sm:modal-middle",
            role: "dialog",
            aria_modal: "true",
            aria_labelledby: "shown-badge-sheet-title",
            tabindex: "-1",
            div { class: "modal-box max-w-md p-0 flex flex-col",
                SheetBar { close_id: "shown-badge-sheet-close", on_close: move |_| on_close.call(()),
                    h2 { id: "shown-badge-sheet-title", class: SHEET_TITLE, "{title}" }
                }
                div { class: "flex flex-col gap-2 px-5 pt-1 pb-6",
                    p { id: "shown-badge-fact", class: "font-semibold", "{fact}" }
                    p { class: "text-sm text-base-content/70", {tid!("badge-shown-note")} }
                }
            }
            div { class: "modal-backdrop", onclick: move |_| on_close.call(()) }
        }
    }
}
