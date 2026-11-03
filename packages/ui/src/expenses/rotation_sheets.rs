use dioxus::prelude::*;
use shared::{Account, User};
use uuid::Uuid;

use crate::common::{copy_text, error_message, haptic, share_link_for, share_text, Flash, Haptic, LocalStorageState};
use crate::decrypted::user_name;
use crate::icons::{CheckIcon, ICON_HEADER, ICON_INLINE};
use crate::rotation_flow::{load_audience, names_and_others, rotate, short_link, Audience, Removal, Rotated};
use crate::tid;

/// The key change, from the confirmation (artboard T8) to the new link (T9). `removee` is the
/// participant the rotation removes, or `None` for "Change the project key".
#[component]
pub fn ProjectRotation(
    project_id: Uuid,
    k1: [u8; 32],
    users: Vec<User>,
    removee: Option<i32>,
    my_user_id: Option<i32>,
    on_close: EventHandler<()>,
    on_switched: EventHandler<[u8; 32]>,
) -> Element {
    let ls_ctx = use_context::<Signal<LocalStorageState>>();
    let auth_ctx = use_context::<Signal<Option<Account>>>();
    let account_key_ctx = use_context::<Signal<Option<[u8; 32]>>>();
    let mut flash = use_context::<Signal<Option<Flash>>>();
    let mut busy = use_signal(|| false);
    let mut error: Signal<Option<String>> = use_signal(|| None);
    let mut done: Signal<Option<Rotated>> = use_signal(|| None);

    let removal = removee.and_then(|id| users.iter().find(|u| u.id == id)).map(|u| Removal { user_id: u.id, name: user_name(&k1, u) });
    let audience_users = users.clone();
    let audience = use_resource(move || {
        let users = audience_users.clone();
        async move {
            let me = auth_ctx.peek().as_ref().map(|a| a.id).unwrap_or_default();
            load_audience(project_id, k1, users, me, removee).await.map_err(|e| error_message(&e))
        }
    });
    let loaded = audience.read().clone();
    let load_error = loaded.as_ref().and_then(|r| r.as_ref().err().cloned());

    let confirm_removal = removal.clone();
    let on_confirm = move |_| {
        let removal = confirm_removal.clone();
        let (Some(account), Some(account_key)) = (auth_ctx.peek().clone(), *account_key_ctx.peek()) else {
            error.set(Some(tid!("error-generic")));
            return;
        };
        busy.set(true);
        error.set(None);
        spawn(async move {
            match rotate(ls_ctx, project_id, k1, &account, account_key, my_user_id, removal).await {
                Ok(rotated) => {
                    haptic(Haptic::Success);
                    on_switched.call(rotated.k2);
                    done.set(Some(rotated));
                }
                Err(e) => error.set(Some(error_message(&e))),
            }
            busy.set(false);
        });
    };

    if let Some(rotated) = done() {
        let link = share_link_for(project_id, &rotated.k2);
        let by_link = loaded.and_then(Result::ok).map_or(0, |a| a.by_link.len());
        let copy_link = link.clone();
        return rsx! {
            NewLinkSheet {
                removed: removal.map(|r| r.name),
                boxed: rotated.boxed,
                by_link,
                link: short_link(&link),
                on_copy: move |_| {
                    let url = copy_link.clone();
                    spawn(async move {
                        if copy_text(&url).await {
                            flash.set(Some(Flash::ok(tid!("link-copied"))));
                        } else {
                            flash.set(Some(Flash::err(tid!("copy-link-failed"))));
                        }
                    });
                },
                on_share: move |_| {
                    let url = link.clone();
                    spawn(async move {
                        if share_text(&url).await {
                            return;
                        }
                        if copy_text(&url).await {
                            flash.set(Some(Flash::ok(tid!("link-copied"))));
                        } else {
                            flash.set(Some(Flash::err(tid!("copy-link-failed"))));
                        }
                    });
                },
                on_close,
            }
        };
    }

    rsx! {
        RotateSheet {
            removee: removal.map(|r| r.name),
            audience: loaded.and_then(Result::ok),
            busy: busy(),
            error: error().or(load_error),
            on_confirm,
            on_cancel: on_close,
        }
    }
}

/// Artboard T8: what changing the key does and who it reaches. No recipient checklist (D11).
#[component]
pub fn RotateSheet(
    removee: Option<String>,
    audience: Option<Audience>,
    busy: bool,
    error: Option<String>,
    on_confirm: EventHandler<()>,
    on_cancel: EventHandler<()>,
) -> Element {
    crate::common::use_modal_focus("rotate-sheet");
    let cancel = move |_| {
        if !busy {
            on_cancel.call(());
        }
    };
    let removing = removee.is_some();
    let (title, body, confirm) = match removee {
        Some(name) => (tid!("rotation-remove-title", name: name), tid!("rotation-remove-body"), tid!("rotation-confirm-remove")),
        None => (tid!("rotation-rotate-title"), tid!("rotation-rotate-body"), tid!("rotation-confirm-rotate")),
    };
    let waiting = audience.is_none();
    rsx! {
        div {
            id: "rotate-sheet",
            class: "modal modal-open modal-bottom sm:modal-middle",
            role: "dialog",
            aria_modal: "true",
            aria_labelledby: "rotate-sheet-title",
            aria_describedby: "rotate-sheet-body",
            tabindex: "-1",
            div { class: "modal-box max-w-md flex flex-col gap-3",
                h2 { id: "rotate-sheet-title", class: "font-bold text-lg font-display", "{title}" }
                p { id: "rotate-sheet-body", class: "text-sm text-base-content/80 leading-snug", "{body}" }
                if let Some(a) = audience {
                    div { class: "flex items-center gap-3 min-h-11",
                        if a.boxed > 0 {
                            span { class: "size-8 shrink-0 rounded-full bg-secondary text-secondary-content flex items-center justify-center", aria_hidden: "true",
                                CheckIcon { size: ICON_INLINE }
                            }
                        }
                        p { id: "rotate-boxed", class: "flex-1 text-sm", {tid!("rotation-boxed", count: a.boxed as i64)} }
                    }
                    if !a.by_link.is_empty() {
                        div { id: "rotate-by-link", class: "bg-base-200 rounded-box px-4 py-3 flex flex-col gap-1",
                            span { class: "text-xs font-semibold text-base-content/70", {tid!("rotation-by-link-title")} }
                            span { class: "text-sm", {names_and_others(&a.by_link)} }
                            span { class: "text-xs text-base-content/70 leading-snug", {tid!("rotation-by-link-hint")} }
                        }
                    }
                } else if error.is_none() {
                    span { class: "loading loading-spinner loading-sm self-center", aria_label: tid!("loading") }
                }
                if let Some(msg) = error.clone() {
                    div { role: "alert", class: "alert alert-error alert-soft text-sm", "{msg}" }
                }
                div { class: "flex gap-2",
                    button {
                        id: "rotate-cancel",
                        r#type: "button",
                        class: "btn btn-outline border-base-300 h-12.5 flex-1 rounded-full",
                        disabled: busy,
                        onclick: cancel,
                        {tid!("cancel")}
                    }
                    button {
                        id: "rotate-confirm",
                        r#type: "button",
                        class: if removing { "btn btn-error h-12.5 flex-[2] rounded-full px-3 leading-tight" } else { "btn btn-primary h-12.5 flex-[2] rounded-full px-3 leading-tight" },
                        disabled: busy || waiting,
                        onclick: move |_| {
                            if !busy && !waiting {
                                on_confirm.call(());
                            }
                        },
                        if busy {
                            span { class: "loading loading-spinner loading-sm" }
                        }
                        "{confirm}"
                    }
                }
            }
            div { class: "modal-backdrop", onclick: cancel }
        }
    }
}

/// Artboard T9: the key is in place; the new link goes to the users without an account. Counts
/// only, never the names, so it reads the same for three people or thirty.
#[component]
pub fn NewLinkSheet(
    removed: Option<String>,
    boxed: usize,
    by_link: usize,
    link: String,
    on_copy: EventHandler<()>,
    on_share: EventHandler<()>,
    on_close: EventHandler<()>,
) -> Element {
    crate::common::use_modal_focus("new-link-sheet");
    let changed = match removed {
        Some(name) => tid!("rotation-done-removed", name: name),
        None => tid!("rotation-done-rotated"),
    };
    let summary = format!("{changed} {}", tid!("rotation-done-boxed", count: boxed as i64));
    rsx! {
        div {
            id: "new-link-sheet",
            class: "modal modal-open modal-bottom sm:modal-middle",
            role: "dialog",
            aria_modal: "true",
            aria_labelledby: "new-link-sheet-title",
            tabindex: "-1",
            div { class: "modal-box max-w-md flex flex-col gap-4",
                div { class: "flex items-center gap-3",
                    span { class: "size-11 shrink-0 rounded-full bg-secondary text-secondary-content flex items-center justify-center", aria_hidden: "true",
                        CheckIcon { size: ICON_HEADER }
                    }
                    div { class: "flex flex-col gap-0.5 min-w-0",
                        h2 { id: "new-link-sheet-title", class: "font-bold text-lg font-display", {tid!("rotation-done-title")} }
                        span { class: "text-sm text-base-content/70 leading-snug", "{summary}" }
                    }
                }
                div { class: "rounded-box border border-base-300 p-3 flex flex-col gap-2",
                    span { class: "text-xs font-semibold text-base-content/70", {tid!("rotation-new-link", count: by_link as i64)} }
                    span { class: "font-mono text-sm text-base-content/80 break-all", "{link}" }
                    div { class: "flex gap-2",
                        button { id: "new-link-copy", r#type: "button", class: "btn btn-outline border-base-300 flex-1 rounded-full", onclick: move |_| on_copy.call(()), {tid!("copy")} }
                        button { id: "new-link-share", r#type: "button", class: "btn btn-primary flex-1 rounded-full", onclick: move |_| on_share.call(()), {tid!("share-link")} }
                    }
                }
                div { role: "status", class: "alert alert-success alert-soft text-sm",
                    CheckIcon { size: ICON_INLINE }
                    span { {tid!("rotation-link-renewed")} }
                }
                button { id: "new-link-done", r#type: "button", class: "btn btn-outline border-base-300 h-12.5 rounded-full", onclick: move |_| on_close.call(()), {tid!("rotation-done")} }
            }
            div { class: "modal-backdrop", onclick: move |_| on_close.call(()) }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::test_dom::{click, listener_ids, texts};
    use dioxus::dioxus_core::VirtualDom;
    use std::cell::RefCell;
    use std::rc::Rc;

    fn clean(m: &dioxus::dioxus_core::Mutations) -> Vec<String> {
        texts(m).into_iter().map(|t| t.replace(['\u{2068}', '\u{2069}'], "")).collect()
    }

    #[derive(Props, Clone, PartialEq)]
    struct RotateProps {
        removee: Option<String>,
        audience: Option<Audience>,
        calls: Rc<RefCell<Vec<&'static str>>>,
    }

    #[component]
    fn RotateHarness(props: RotateProps) -> Element {
        crate::i18n::use_test_i18n();
        let log = |name: &'static str| {
            let calls = props.calls.clone();
            EventHandler::new(move |_: ()| calls.borrow_mut().push(name))
        };
        rsx! {
            RotateSheet { removee: props.removee.clone(), audience: props.audience.clone(), busy: false, error: None, on_confirm: log("confirm"), on_cancel: log("cancel") }
        }
    }

    fn rotate_sheet(removee: Option<&str>, audience: Option<Audience>) -> (Vec<String>, Vec<Vec<&'static str>>) {
        let calls = Rc::new(RefCell::new(Vec::new()));
        let props = RotateProps { removee: removee.map(str::to_string), audience, calls: calls.clone() };
        let mut dom = VirtualDom::new_with_props(RotateHarness, props);
        let edits = dom.rebuild_to_vec();
        let mut runs = Vec::new();
        for id in listener_ids(&edits, "click") {
            calls.borrow_mut().clear();
            dom.runtime().handle_event("click", click(), id);
            runs.push(calls.borrow().clone());
        }
        (clean(&edits), runs)
    }

    fn names(n: usize) -> Vec<String> {
        ["Tom", "Sam", "Hugo"].into_iter().map(str::to_string).chain((3..n).map(|i| format!("P{i}"))).take(n).collect()
    }

    #[test]
    fn removing_someone_says_who_gets_the_key_and_who_gets_the_link() {
        let (text, runs) = rotate_sheet(Some("Léa"), Some(Audience { boxed: 3, by_link: names(30) }));
        for line in [
            "Remove Léa from the project?",
            "3 members will get the key automatically",
            "Tom, Sam, Hugo and 27 others",
            "Users you’ll need to send the new link to",
            "Remove and change the key",
        ] {
            assert!(text.iter().any(|t| t == line), "{line:?} in {text:?}");
        }
        assert!(!text.iter().any(|t| t.contains("P29")), "never every name");
        assert!(runs.contains(&vec!["confirm"]) && runs.contains(&vec!["cancel"]));
    }

    #[test]
    fn a_few_names_are_all_listed_and_none_hide_the_block() {
        let (text, _) = rotate_sheet(None, Some(Audience { boxed: 1, by_link: names(3) }));
        assert!(text.iter().any(|t| t == "Change the project key?"), "{text:?}");
        assert!(text.iter().any(|t| t == "1 member will get the key automatically"), "{text:?}");
        assert!(text.iter().any(|t| t == "Tom, Sam and Hugo"), "{text:?}");
        let (one, _) = rotate_sheet(None, Some(Audience { boxed: 0, by_link: names(1) }));
        assert!(one.iter().any(|t| t == "Tom"), "{one:?}");
        let (none, _) = rotate_sheet(None, Some(Audience { boxed: 2, by_link: vec![] }));
        assert!(!none.iter().any(|t| t == "Users you’ll need to send the new link to"), "{none:?}");
    }

    #[test]
    fn nothing_is_confirmed_before_the_audience_is_known() {
        let (_, runs) = rotate_sheet(Some("Léa"), None);
        assert!(!runs.iter().any(|r| r.contains(&"confirm")), "{runs:?}");
    }

    #[derive(Props, Clone, PartialEq)]
    struct LinkProps {
        removed: Option<String>,
        boxed: usize,
        by_link: usize,
    }

    #[component]
    fn LinkHarness(props: LinkProps) -> Element {
        crate::i18n::use_test_i18n();
        rsx! {
            NewLinkSheet {
                removed: props.removed.clone(),
                boxed: props.boxed,
                by_link: props.by_link,
                link: "counted.fr/projects/8c1e…#k9Fq…",
                on_copy: |_| {},
                on_share: |_| {},
                on_close: |_| {},
            }
        }
    }

    fn link_sheet(removed: Option<&str>, boxed: usize, by_link: usize) -> Vec<String> {
        let mut dom = VirtualDom::new_with_props(LinkHarness, LinkProps { removed: removed.map(str::to_string), boxed, by_link });
        clean(&dom.rebuild_to_vec())
    }

    #[test]
    fn the_new_link_sheet_counts_and_scales_to_thirty() {
        let text = link_sheet(Some("Léa"), 3, 30);
        for line in [
            "Léa is no longer part of the project. 3 members got the key automatically.",
            "New link to send · 30 users without an account",
            "The link was renewed successfully.",
            "counted.fr/projects/8c1e…#k9Fq…",
        ] {
            assert!(text.iter().any(|t| t == line), "{line:?} in {text:?}");
        }
        let plain = link_sheet(None, 1, 1);
        assert!(plain.iter().any(|t| t == "The project key changed. 1 member got the key automatically."), "{plain:?}");
        assert!(plain.iter().any(|t| t == "New link to send · 1 user without an account"), "{plain:?}");
    }
}
