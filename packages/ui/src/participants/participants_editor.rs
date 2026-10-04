use dioxus::prelude::*;
use crate::tid;
use shared::Friend;

use super::participants_service::{
    added_friend_ids, apply_pick, email_local_part, initial, picks, quick_add, rename,
    suggestions, DraftParticipant, NameError, Pick,
};
use crate::friends::FriendPickerModal;
use crate::icons::{CheckIcon, CloseIcon, PencilIcon, SendIcon, UserPlusIcon, UsersIcon, ICON_INLINE};
use crate::route::Route;

#[derive(PartialEq, Props, Clone)]
pub struct ParticipantsEditorProps {
    pub drafts: Signal<Vec<DraftParticipant>>,
    /// Names in use outside the drafts: yours and the saved participants'.
    pub taken: Vec<String>,
    pub friends: Vec<Friend>,
    pub signed_in: bool,
    /// Prefix of every element id: `add-project`, `edit-project`.
    pub id_prefix: String,
    /// The create modal shows a "nobody yet" box; the edit modal lists saved rows above instead.
    pub show_empty: bool,
    /// Drafts are marked "new" in the edit modal, next to the saved participants.
    #[props(default)]
    pub mark_new: bool,
}

/// The unsaved participants and the one field that adds them: typed names become guests, friends
/// are suggested as their email matches and offered as chips. See `docs/plans/friends.md` §11.
#[component]
pub fn ParticipantsEditor(props: ParticipantsEditorProps) -> Element {
    let mut drafts = props.drafts;
    let mut query = use_signal(String::new);
    let mut highlight = use_signal(|| 0usize);
    let mut dismissed = use_signal(|| false);
    let mut renaming: Signal<Option<(usize, String)>> = use_signal(|| None);
    let mut error: Signal<Option<NameError>> = use_signal(|| None);
    let mut show_picker = use_signal(|| false);

    let taken_with_drafts = {
        let taken = props.taken.clone();
        move |skip: Option<usize>| -> Vec<String> {
            taken
                .iter()
                .cloned()
                .chain(
                    drafts
                        .read()
                        .iter()
                        .enumerate()
                        .filter(|(i, _)| Some(*i) != skip)
                        .map(|(_, d)| d.name.clone()),
                )
                .collect()
        }
    };

    let friends = props.friends.clone();
    let added = added_friend_ids(&drafts.read());
    let found = if props.signed_in { suggestions(&query(), &friends, &added) } else { vec![] };
    let options = if props.signed_in { picks(&query(), &found) } else { vec![] };
    let list_open = props.signed_in && !dismissed() && !query().trim().is_empty();
    let chips = quick_add(&friends, &added);
    let pickable: Vec<Friend> =
        friends.iter().filter(|f| !added.contains(&f.account_id)).cloned().collect();
    let prefix = props.id_prefix.clone();

    let mut add = {
        let taken_with_drafts = taken_with_drafts.clone();
        move |pick: Pick| {
            let taken = taken_with_drafts(None);
            match apply_pick(&mut drafts.write(), pick, &taken) {
                Ok(()) => {
                    query.set(String::new());
                    highlight.set(0);
                    error.set(None);
                }
                Err(NameError::Empty) => {}
                Err(e) => error.set(Some(e)),
            }
        }
    };

    let save_rename = {
        let taken_with_drafts = taken_with_drafts.clone();
        move || {
            let Some((index, text)) = renaming() else { return };
            let taken = taken_with_drafts(Some(index));
            match rename(&mut drafts.write(), index, &text, &taken) {
                Ok(()) => {
                    renaming.set(None);
                    error.set(None);
                }
                Err(NameError::Empty) => {}
                Err(e) => error.set(Some(e)),
            }
        }
    };

    let options_for_keys = options.clone();
    let mut add_for_keys = add.clone();
    let mut add_for_button = add.clone();
    let add_for_options = add.clone();
    let add_for_chips = add.clone();

    rsx! {
        if !drafts.read().is_empty() {
            ul { class: "flex flex-col",
                for (i, draft) in drafts.read().iter().cloned().enumerate() {
                    if renaming().map(|(r, _)| r) == Some(i) {
                        RenameRow {
                            key: "{prefix}-rename-{i}",
                            id: format!("{prefix}-participant-{i}-name"),
                            value: renaming().map(|(_, t)| t).unwrap_or_default(),
                            friend_email: draft.friend.as_ref().map(|f| f.email.clone()),
                            on_input: move |t: String| renaming.set(Some((i, t))),
                            on_save: {
                                let mut save_rename = save_rename.clone();
                                move |_| save_rename()
                            },
                            on_cancel: move |_| {
                                renaming.set(None);
                                error.set(None);
                            },
                        }
                    } else {
                        DraftRow {
                            key: "{prefix}-draft-{i}",
                            id: format!("{prefix}-participant-{i}"),
                            name: draft.name.clone(),
                            friend_email: draft.friend.as_ref().map(|f| f.email.clone()),
                            mark_new: props.mark_new,
                            on_rename: move |_| {
                                let name = drafts.read().get(i).map(|d| d.name.clone()).unwrap_or_default();
                                renaming.set(Some((i, name)));
                                error.set(None);
                            },
                            on_remove: move |_| {
                                if i < drafts.read().len() {
                                    drafts.write().remove(i);
                                }
                                renaming.set(None);
                                error.set(None);
                            },
                        }
                    }
                }
            }
        } else if props.show_empty {
            p { class: "text-sm text-base-content/70 text-center border border-dashed border-base-300 rounded-box px-4 py-3",
                {tid!("participants-empty")}
            }
        }

        if let Some(NameError::Taken(name)) = error() {
            p { role: "alert", class: "text-sm text-error", {tid!("participants-duplicate", name: name)} }
        }

        // A drop-up: with the keyboard open the field sits at the bottom of what is visible, so a
        // list below it opened out of sight (seen on a phone). Above, it always shows.
        div { class: "relative",
        div { class: "flex gap-2",
            label { class: "input flex-1",
                UserPlusIcon { size: ICON_INLINE }
                input {
                    r#type: "text",
                    id: "{prefix}-user",
                    role: if props.signed_in { "combobox" },
                    aria_expanded: if props.signed_in { if list_open { "true" } else { "false" } },
                    aria_controls: if props.signed_in { "{prefix}-suggestions" },
                    aria_label: if props.signed_in { tid!("participants-input-label") } else { tid!("add-project-participant-name") },
                    placeholder: if props.signed_in { tid!("participants-input-placeholder") } else { tid!("add-project-participant-name") },
                    enterkeyhint: "done",
                    autocapitalize: "words",
                    autocomplete: "off",
                    value: "{query}",
                    oninput: move |e| {
                        query.set(e.value());
                        highlight.set(0);
                        dismissed.set(false);
                    },
                    // Enter adds instead of submitting. Nothing blurs, so the keyboard stays up and
                    // the view does not reflow between names.
                    onkeydown: move |e| match e.key() {
                        Key::Enter => {
                            e.prevent_default();
                            let pick = if list_open {
                                options_for_keys.get(highlight()).cloned()
                            } else {
                                Some(Pick::Guest(query()))
                            };
                            if let Some(pick) = pick {
                                add_for_keys(pick);
                            }
                        }
                        Key::ArrowDown if list_open => {
                            e.prevent_default();
                            let last = options_for_keys.len().saturating_sub(1);
                            highlight.set((highlight() + 1).min(last));
                        }
                        Key::ArrowUp if list_open => {
                            e.prevent_default();
                            highlight.set(highlight().saturating_sub(1));
                        }
                        Key::Escape if list_open => dismissed.set(true),
                        _ => {}
                    },
                }
            }
            button {
                id: "{prefix}-user-add",
                r#type: "button",
                class: "btn btn-secondary",
                onclick: move |_| add_for_button(Pick::Guest(query())),
                {tid!("add")}
            }
        }

        if list_open {
            ul {
                id: "{prefix}-suggestions",
                role: "listbox",
                class: "absolute bottom-full left-0 right-0 mb-2 z-10 flex flex-col gap-0.5 p-1.5 rounded-box border border-base-200 bg-base-100 shadow-lg",
                for s in found.iter().cloned() {
                    {
                        let index = options.iter().position(|p| matches!(p, Pick::Friend(f) if f.account_id == s.friend.account_id));
                        let selected = index == Some(highlight());
                        let name = super::participants_service::name_from_email(&s.friend.email);
                        let friend = s.friend.clone();
                        let mut add_option = add_for_options.clone();
                        rsx! {
                            li { role: "presentation",
                                button {
                                    r#type: "button",
                                    role: "option",
                                    aria_selected: if selected { "true" } else { "false" },
                                    aria_disabled: if !s.ready { "true" },
                                    disabled: !s.ready,
                                    class: if selected { "w-full flex items-center gap-3 px-2.5 py-2 rounded-field text-left bg-primary/10" } else { "w-full flex items-center gap-3 px-2.5 py-2 rounded-field text-left disabled:opacity-60" },
                                    onclick: move |_| add_option(Pick::Friend(friend.clone())),
                                    Avatar { name: s.friend.email.clone(), guest: false }
                                    span { class: "flex flex-col min-w-0 flex-1",
                                        span { class: "font-semibold truncate",
                                            "{s.before}"
                                            mark { class: "bg-warning/40 text-inherit rounded-sm", "{s.matched}" }
                                            "{s.after}"
                                        }
                                        span { class: "text-sm text-base-content/70 truncate",
                                            if s.ready {
                                                {tid!("participants-suggest-friend", name: name)}
                                            } else {
                                                {tid!("participants-suggest-not-ready")}
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                {
                    let guest_index = options.len().saturating_sub(1);
                    let selected = highlight() == guest_index;
                    let text = query().trim().to_string();
                    let mut add_guest = add.clone();
                    rsx! {
                        li { role: "presentation", class: if !found.is_empty() { "border-t border-base-200 mt-1 pt-1" },
                            button {
                                id: "{prefix}-suggest-guest",
                                r#type: "button",
                                role: "option",
                                aria_selected: if selected { "true" } else { "false" },
                                class: if selected { "w-full flex items-center gap-3 px-2.5 py-2 rounded-field text-left bg-primary/10" } else { "w-full flex items-center gap-3 px-2.5 py-2 rounded-field text-left" },
                                onclick: move |_| add_guest(Pick::Guest(query())),
                                Avatar { name: "+".to_string(), guest: true }
                                span { class: "flex flex-col min-w-0 flex-1",
                                    span { class: "font-semibold truncate", {tid!("participants-suggest-guest", text: text)} }
                                    span { class: "text-sm text-base-content/70 truncate", {tid!("participants-suggest-guest-sub")} }
                                }
                            }
                        }
                    }
                }
            }
        }
        }

        if props.signed_in && !friends.is_empty() {
            div { class: "flex flex-col gap-2 mt-1",
                span { class: "text-xs font-semibold text-base-content/70", {tid!("participants-friends")} }
                div { class: "flex flex-wrap gap-2",
                    for (i, f) in chips.iter().cloned().enumerate() {
                        {
                        let mut add_chip = add_for_chips.clone();
                        rsx! {
                        button {
                            key: "{f.account_id}",
                            id: "{prefix}-chip-{i}",
                            r#type: "button",
                            class: "btn btn-sm btn-outline border-base-300 rounded-full pl-1 gap-2 font-semibold",
                            onclick: move |_| add_chip(Pick::Friend(f.clone())),
                            Avatar { name: f.email.clone(), guest: false, small: true }
                            "{email_local_part(&f.email)}"
                        }
                        }
                        }
                    }
                    button {
                        id: "{prefix}-all-friends",
                        r#type: "button",
                        class: "btn btn-sm btn-outline border-base-300 rounded-full gap-2 font-semibold",
                        onclick: move |_| show_picker.set(true),
                        UsersIcon { size: ICON_INLINE }
                        {tid!("participants-all-friends")}
                    }
                }
            }
        }

        if !props.signed_in {
            Link {
                class: "link link-primary text-sm bg-base-200 rounded-field px-3.5 py-3 no-underline",
                to: Route::LoginPage {},
                {tid!("participants-login-hint")}
            }
        }

        if show_picker() {
            FriendPickerModal {
                friends: pickable,
                on_close: move |_| show_picker.set(false),
                on_pick: move |chosen: Vec<Friend>| {
                    for f in chosen {
                        add(Pick::Friend(f));
                    }
                    show_picker.set(false);
                },
            }
        }
    }
}

#[derive(PartialEq, Props, Clone)]
pub struct AvatarProps {
    pub name: String,
    pub guest: bool,
    #[props(default)]
    pub small: bool,
}

#[component]
pub fn Avatar(props: AvatarProps) -> Element {
    let size = if props.small { "w-7 h-7 text-xs" } else { "w-9 h-9 text-sm" };
    let look = if props.guest {
        "border-[1.5px] border-dashed border-base-300 bg-base-200 text-base-content/70"
    } else {
        "bg-primary/15 text-primary"
    };
    rsx! {
        span {
            aria_hidden: "true",
            class: "{size} {look} rounded-full flex items-center justify-center font-semibold flex-shrink-0",
            "{initial(&props.name)}"
        }
    }
}

#[derive(PartialEq, Props, Clone)]
struct DraftRowProps {
    id: String,
    name: String,
    friend_email: Option<String>,
    mark_new: bool,
    on_rename: EventHandler<()>,
    on_remove: EventHandler<()>,
}

#[component]
fn DraftRow(props: DraftRowProps) -> Element {
    let name = props.name.clone();
    rsx! {
        li { id: "{props.id}", class: "flex items-center gap-2.5 min-h-13",
            Avatar { name: props.name.clone(), guest: props.friend_email.is_none() }
            span { class: "flex flex-col min-w-0 flex-1 gap-0.5",
                span { class: "flex items-center gap-1.5 min-w-0",
                    span { class: if props.mark_new { "font-semibold truncate italic" } else { "font-semibold truncate" }, "{props.name}" }
                    if props.mark_new {
                        span { class: "badge badge-soft badge-info badge-xs", {tid!("edit-project-new-badge")} }
                    }
                    if props.friend_email.is_some() {
                        span { class: "badge badge-soft badge-primary badge-sm gap-1",
                            SendIcon { size: 12 }
                            {tid!("participants-invite-badge")}
                        }
                    } else {
                        span { class: "badge badge-ghost badge-sm", {tid!("participants-guest-badge")} }
                    }
                }
                if let Some(email) = props.friend_email.clone() {
                    span { class: "text-sm text-base-content/70 truncate", "{email}" }
                }
            }
            button {
                id: "{props.id}-rename",
                r#type: "button",
                class: "btn btn-ghost btn-circle h-11 w-11 min-h-11",
                aria_label: tid!("participants-rename", name: name.clone()),
                onclick: move |_| props.on_rename.call(()),
                PencilIcon { size: ICON_INLINE }
            }
            button {
                id: "{props.id}-remove",
                r#type: "button",
                class: "btn btn-ghost btn-circle h-11 w-11 min-h-11",
                aria_label: tid!("participants-remove", name: name),
                onclick: move |_| props.on_remove.call(()),
                CloseIcon { size: ICON_INLINE }
            }
        }
    }
}

#[derive(PartialEq, Props, Clone)]
struct RenameRowProps {
    id: String,
    value: String,
    friend_email: Option<String>,
    on_input: EventHandler<String>,
    on_save: EventHandler<()>,
    on_cancel: EventHandler<()>,
}

#[component]
fn RenameRow(props: RenameRowProps) -> Element {
    rsx! {
        li {
            class: "flex flex-col gap-1.5 -mx-2.5 px-2.5 py-2 rounded-box bg-primary/5",
            // Above the field, not below: with the keyboard up the field sits at the bottom of what
            // is visible (seen on a phone).
            if let Some(email) = props.friend_email.clone() {
                p { class: "text-xs text-base-content/70 pl-11", {tid!("participants-rename-hint", email: email)} }
            }
            div { class: "flex items-center gap-2",
                Avatar { name: props.value.clone(), guest: props.friend_email.is_none() }
                label { class: "input input-primary flex-1 h-10",
                    input {
                        id: "{props.id}",
                        r#type: "text",
                        aria_label: tid!("participants-rename-label"),
                        autocapitalize: "words",
                        autocomplete: "off",
                        enterkeyhint: "done",
                        autofocus: true,
                        value: "{props.value}",
                        oninput: move |e| props.on_input.call(e.value()),
                        onkeydown: move |e| match e.key() {
                            Key::Enter => {
                                e.prevent_default();
                                props.on_save.call(());
                            }
                            Key::Escape => props.on_cancel.call(()),
                            _ => {}
                        },
                    }
                }
                button {
                    id: "{props.id}-save",
                    r#type: "button",
                    class: "btn btn-primary btn-square h-10 w-10 min-h-10",
                    aria_label: tid!("participants-rename-save"),
                    disabled: props.value.trim().is_empty(),
                    onclick: move |_| props.on_save.call(()),
                    CheckIcon { size: ICON_INLINE }
                }
                button {
                    r#type: "button",
                    class: "btn btn-ghost btn-square h-10 w-10 min-h-10",
                    aria_label: tid!("cancel"),
                    onclick: move |_| props.on_cancel.call(()),
                    CloseIcon { size: ICON_INLINE }
                }
            }
        }
    }
}
