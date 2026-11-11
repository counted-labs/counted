use dioxus::prelude::*;
use shared::IncomingFriendRequest;

use super::friends_service::FriendRow;
use super::use_friends::Friends;
use crate::common::{account_color_class, email_initials, format_date, Avatar, FormSheetBar, SizeClass};
use crate::icons::{CheckIcon, CloseIcon, SendIcon, UserMinusIcon, ICON_HEADER};
use crate::tid;

/// A bottom sheet with the title leading and the close trailing, as every sheet in the app that is
/// not a form. The title wraps instead of truncating: an address cut at its domain is how a
/// lookalike account would pass.
#[component]
pub fn FriendsSheet(id: &'static str, title: String, on_close: EventHandler<()>, children: Element) -> Element {
    crate::common::use_modal_focus(id);
    rsx! {
        div {
            id,
            class: "modal modal-open modal-bottom sm:modal-middle",
            role: "dialog",
            aria_modal: "true",
            aria_labelledby: "{id}-title",
            tabindex: "-1",
            div { class: "modal-box max-w-md p-0",
                div { class: "flex flex-col gap-4 px-6 pt-5 pb-6",
                    div { class: "flex items-center gap-2",
                        h2 { id: "{id}-title", tabindex: "-1", class: "flex-1 min-w-0 break-all font-bold text-lg font-display", "{title}" }
                        button {
                            id: "{id}-close",
                            r#type: "button",
                            class: "btn btn-circle size-11 min-h-11 border-0 bg-base-200 text-base-content/80 shrink-0",
                            aria_label: tid!("close"),
                            onclick: move |_| on_close.call(()),
                            CloseIcon {}
                        }
                    }
                    {children}
                }
            }
            div { class: "modal-backdrop", onclick: move |_| on_close.call(()) }
        }
    }
}

/// A safety code in four groups, large enough to read aloud, each group read as letters.
#[component]
pub fn SafetyCode(code: String, #[props(default)] changed: bool) -> Element {
    let groups: Vec<String> = code.split_whitespace().map(str::to_string).collect();
    let tint = if changed { "bg-error/10" } else { "bg-base-200" };
    rsx! {
        div { class: "grid grid-cols-4 gap-x-3 gap-y-1 justify-items-center rounded-box px-3 py-4 font-mono font-semibold text-xl tracking-wider {tint}",
            for group in groups {
                span { aria_label: group.chars().map(String::from).collect::<Vec<_>>().join(" "), "{group}" }
            }
        }
    }
}

/// One friend: the code to compare, or why there is none; a changed key, with "The code matches"
/// only when comparing can prove something. Takes the row by value: the code shown and the key
/// `trust_new_key` pins come from the same row, never from a newer load.
#[component]
pub fn FriendSheet(
    row: FriendRow,
    busy: bool,
    on_close: EventHandler<()>,
    on_remove: EventHandler<FriendRow>,
    on_trust: EventHandler<FriendRow>,
) -> Element {
    let remove_row = row.clone();
    let remove = move |_| on_remove.call(remove_row.clone());

    if row.kem_only_change() {
        return rsx! {
            FriendsSheet { id: "friend-sheet", title: tid!("friends-kem-changed-title"), on_close,
                p { class: "font-medium break-all", "{row.email}" }
                div { role: "alert", class: "alert alert-error alert-soft text-sm", {tid!("friends-kem-changed-body")} }
                p { class: "text-sm text-base-content/70", {tid!("friends-kem-changed-hint")} }
                button { id: "friend-remove", r#type: "button", class: "btn btn-ghost text-error", disabled: busy, onclick: remove, {tid!("friends-remove-row")} }
            }
        };
    }

    if row.key_changed {
        let trusted = row.clone();
        return rsx! {
            FriendsSheet { id: "friend-sheet", title: tid!("friends-code-changed-row"), on_close,
                p { class: "font-medium break-all", "{row.email}" }
                if let Some(code) = row.new_fingerprint() {
                    div { class: "flex flex-col gap-1.5",
                        span { class: "text-sm font-medium text-base-content/70", {tid!("friends-new-code")} }
                        SafetyCode { code, changed: true }
                    }
                }
                p { class: "text-sm text-base-content/70", {tid!("friends-key-changed-hint-short")} }
                if row.new_fingerprint().is_some() {
                    button {
                        id: "friend-trust",
                        r#type: "button",
                        class: "btn",
                        disabled: busy,
                        onclick: move |_| {
                            on_trust.call(trusted.clone());
                            on_close.call(());
                        },
                        {tid!("friends-code-matches")}
                    }
                }
                button { id: "friend-remove", r#type: "button", class: "btn btn-ghost text-error", disabled: busy, onclick: remove, {tid!("friends-remove-row")} }
            }
        };
    }

    rsx! {
        FriendsSheet { id: "friend-sheet", title: row.email.clone(), on_close,
            match row.fingerprint.clone() {
                Some(code) => rsx! {
                    div { class: "flex flex-col gap-1.5",
                        span { class: "text-sm font-medium text-base-content/70", {tid!("friends-their-code")} }
                        SafetyCode { code }
                    }
                    p { class: "text-sm text-base-content/70", {tid!("friends-code-compare-hint")} }
                },
                None => rsx! {
                    p { class: "rounded-box bg-base-200 p-3 text-sm", {tid!("friends-no-key-hint")} }
                },
            }
            button {
                id: "friend-remove",
                r#type: "button",
                class: "btn btn-ghost text-error justify-start",
                disabled: busy,
                onclick: remove,
                UserMinusIcon { size: ICON_HEADER }
                {tid!("friends-remove-row")}
            }
        }
    }
}

#[component]
pub fn OwnCodeSheet(code: String, on_close: EventHandler<()>) -> Element {
    rsx! {
        FriendsSheet { id: "own-code-sheet", title: tid!("friends-own-code"), on_close,
            SafetyCode { code }
            p { class: "text-sm text-base-content/70", {tid!("friends-own-code-hint")} }
        }
    }
}

/// Adding by address, then the sent state in the same sheet. The server answers the same whether
/// or not the address has an account, and the sheet never says which.
#[component]
pub fn AddFriendSheet(friends: Friends, on_close: EventHandler<()>) -> Element {
    crate::common::use_modal_focus("add-friend-sheet");
    let mut email = friends.email;
    let mut sent_to = friends.sent_to;
    let mut close = move || {
        sent_to.set(None);
        on_close.call(());
    };

    rsx! {
        div {
            id: "add-friend-sheet",
            class: "modal modal-open modal-bottom sm:modal-middle",
            role: "dialog",
            aria_modal: "true",
            aria_labelledby: "add-friend-title",
            tabindex: "-1",
            div { class: "modal-box max-w-md p-0",
                if let Some(sent) = sent_to() {
                    div { id: "friend-request-sent", role: "status", class: "flex flex-col items-center gap-3 px-6 pt-6 pb-6 text-center",
                        span { class: "size-16 rounded-2xl grid place-items-center bg-primary/10 text-secondary-content", CheckIcon { size: 32 } }
                        h2 { id: "add-friend-title", class: "font-display font-bold text-lg", {tid!("friends-request-sent")} }
                        bdi { class: "text-sm text-base-content/70 break-all", "{sent}" }
                        p { class: "text-sm text-base-content/70", {tid!("friends-sent-hint")} }
                        button {
                            id: "friend-notify",
                            r#type: "button",
                            class: "btn btn-primary w-full",
                            onclick: move |_| friends.notify(sent.clone()),
                            SendIcon { size: ICON_HEADER }
                            {tid!("friends-notify")}
                        }
                        button { id: "friend-done", r#type: "button", class: "btn w-full", onclick: move |_| close(), {tid!("friends-done")} }
                    }
                } else {
                    form {
                        id: "add-friend-form",
                        class: "flex flex-col gap-3 px-6 pt-4 pb-6",
                        onsubmit: move |e| {
                            e.prevent_default();
                            friends.add_by_email();
                        },
                        FormSheetBar {
                            close_id: "add-friend-close",
                            confirm_id: "friend-add",
                            confirm_label: tid!("friends-send"),
                            form: "add-friend-form",
                            disabled: (friends.busy)() || email().trim().is_empty(),
                            on_close: move |_| close(),
                            h2 { id: "add-friend-title", class: "w-full text-center font-display font-bold text-lg truncate", {tid!("friends-add-title")} }
                        }
                        label { class: "flex flex-col gap-1.5", r#for: "friend-email",
                            span { class: "text-sm font-medium text-base-content/70", {tid!("field-email")} }
                            input {
                                id: "friend-email",
                                r#type: "email",
                                class: "input w-full",
                                inputmode: "email",
                                autocapitalize: "off",
                                autocorrect: "off",
                                spellcheck: "false",
                                autocomplete: "off",
                                autofocus: true,
                                required: true,
                                placeholder: tid!("field-email-placeholder"),
                                value: "{email}",
                                oninput: move |e| email.set(e.value()),
                            }
                        }
                        if let Some(err) = (friends.error)() {
                            p { role: "alert", class: "text-sm text-error", "{err}" }
                        }
                        p { class: "text-sm text-base-content/70", {tid!("friends-add-hint-short")} }
                    }
                }
            }
            div { class: "modal-backdrop", onclick: move |_| close() }
        }
    }
}

/// An incoming request: the address, when it came, and Accept and Decline under it so the row holds
/// at large text sizes and in long languages. A long press opens the same two in a sheet.
#[component]
pub fn IncomingRow(
    req: IncomingFriendRequest,
    friends: Friends,
    #[props(default)] on_actions: Option<EventHandler<IncomingFriendRequest>>,
    #[props(default)] subtitle: Option<String>,
) -> Element {
    let id = req.id;
    let busy = (friends.busy)();
    let long_press = on_actions.map(|_| format!("long-press-request-{id}"));
    let held = req.clone();
    let hidden = long_press.clone();
    let subtitle = subtitle.unwrap_or_else(|| tid!("friends-received-on", date: format_date(req.created_at.date())));
    rsx! {
        li { class: "request-row flex flex-col gap-2 px-4 py-3", "data-long-press": long_press.clone(),
            div { class: "flex items-center gap-3",
                Avatar { initials: email_initials(&req.requester_email), color_class: account_color_class(id).to_string(), size: SizeClass::W10 }
                div { class: "flex-1 min-w-0 flex flex-col",
                    bdi { class: "font-medium break-all", "{req.requester_email}" }
                    span { class: "text-sm text-base-content/70", "{subtitle}" }
                }
            }
            div { class: "flex flex-wrap gap-2 pl-[3.25rem]",
                button {
                    r#type: "button",
                    class: "btn btn-primary btn-sm",
                    aria_label: tid!("friends-accept-label", email: req.requester_email.clone()),
                    disabled: busy,
                    onclick: move |_| friends.accept(id),
                    {tid!("friends-accept")}
                }
                button {
                    r#type: "button",
                    class: "btn btn-sm",
                    aria_label: tid!("friends-decline-label", email: req.requester_email.clone()),
                    disabled: busy,
                    onclick: move |_| friends.decline_or_withdraw(id),
                    {tid!("friends-decline")}
                }
            }
            if let (Some(hidden), Some(on_actions)) = (hidden, on_actions) {
                button {
                    id: hidden,
                    r#type: "button",
                    hidden: true,
                    tabindex: "-1",
                    onclick: move |_| {
                        crate::common::haptic(crate::common::Haptic::Medium);
                        on_actions.call(held.clone());
                    },
                }
            }
        }
    }
}

pub fn changed_friend_avatar(row: &FriendRow) -> Element {
    rsx! {
        Avatar { initials: email_initials(&row.email), color_class: "bg-tile-red text-tile-red-content".to_string(), size: SizeClass::W10 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::test_dom::texts;
    use dioxus::dioxus_core::VirtualDom;
    use uuid::Uuid;

    #[component]
    fn Harness(row: FriendRow) -> Element {
        crate::i18n::use_test_i18n();
        rsx! {
            FriendSheet { row, busy: false, on_close: |_| {}, on_remove: |_| {}, on_trust: |_| {} }
        }
    }

    fn shown(row: FriendRow) -> Vec<String> {
        let mut dom = VirtualDom::new_with_props(Harness, HarnessProps { row });
        texts(&dom.rebuild_to_vec())
    }

    fn row(fingerprint: Option<&str>, key_changed: bool, new_key: Option<Vec<u8>>) -> FriendRow {
        FriendRow {
            id: Uuid::nil(),
            account_id: Uuid::nil(),
            email: "bob@x.io".into(),
            fingerprint: fingerprint.map(str::to_string),
            key_changed,
            new_key,
            new_kem_key: None,
        }
    }

    /// Security review 2026-11-07 H1: trusting a changed key is offered only beside its new code.
    #[test]
    fn the_code_matches_is_offered_only_beside_the_new_code() {
        let (new_key, _) = crate::crypto::generate_keypair();
        let changed = row(None, true, Some(new_key));
        let code = changed.new_fingerprint().expect("a served key has a code");
        let shown = shown(changed);
        assert!(shown.iter().any(|t| t == "The code matches"), "{shown:?}");
        assert!(shown.iter().any(|t| code.split_whitespace().any(|group| t == group)), "{shown:?}");
    }

    /// D8: only the post-quantum half changed, which the code does not cover, so no comparison
    /// can vouch for it and the sheet offers removal only.
    #[test]
    fn a_kem_only_change_offers_no_trust() {
        let shown = shown(row(Some("ABCD EFGH IJKL MNOP"), true, None));
        assert!(shown.iter().any(|t| t == "Remove from my friends"), "{shown:?}");
        assert!(!shown.iter().any(|t| t == "The code matches"), "{shown:?}");
    }

    #[test]
    fn a_settled_friend_offers_no_trust() {
        let shown = shown(row(Some("ABCD EFGH IJKL MNOP"), false, None));
        assert!(!shown.iter().any(|t| t == "The code matches"), "{shown:?}");
    }
}
