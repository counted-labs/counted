use dioxus::prelude::*;
use crate::tid;
use shared::{ProjectDto, User};

use crate::common::creator_proof::holds_creator_secret;
use crate::common::LocalStorageState;
use crate::crypto::decrypt_json;
use crate::decrypted::user_name;
use crate::icons::{EyeOffIcon, KeyIcon, LockIcon, ShieldIcon, UserMinusIcon, ICON_HEADER, ICON_INLINE};
use crate::projects::GroupPrefill;

/// What the group's admin actions say to this device (docs/plans/key-lifecycle.md §8.7). They are
/// never hidden: each state says what it takes to use them.
#[derive(Debug, Clone, PartialEq)]
pub enum AdminAccess {
    Admin,
    /// An account holder who is not admin, with the admins' names to ask (empty: "an admin").
    AskAdmins(Vec<String>),
    /// No account, in a group that has admins: an account, then an admin's grant.
    SignIn(Vec<String>),
    /// No admin, on the device holding the creator secret (T7, D9): an account makes it admin.
    Creator,
    /// No admin, and the creator's proof is on another device (T7b).
    CreatorElsewhere,
    /// No admin and no creator proof (D19): nobody can change this group's key.
    NoAdmin,
}

/// `creator_proof`: the project still holds its creator's proof; `holds_secret`: this device made it.
pub fn admin_access(
    signed_in: bool,
    me_admin: bool,
    has_admin: bool,
    admins: Vec<String>,
    creator_proof: bool,
    holds_secret: bool,
) -> AdminAccess {
    match (me_admin, has_admin, signed_in) {
        (true, _, true) => AdminAccess::Admin,
        (_, false, _) => match (creator_proof, holds_secret) {
            (true, true) => AdminAccess::Creator,
            (true, false) => AdminAccess::CreatorElsewhere,
            (false, _) => AdminAccess::NoAdmin,
        },
        (_, true, true) => AdminAccess::AskAdmins(admins),
        (_, true, false) => AdminAccess::SignIn(admins),
    }
}

/// The admins a member can name: claimed participants whose account holds the role, by the
/// account's display name when its label opens, else by the participant's.
pub fn admin_names(key: Option<&[u8; 32]>, users: &[User]) -> Vec<String> {
    let Some(key) = key else { return Vec::new() };
    users
        .iter()
        .filter(|u| u.claimed && u.admin)
        .map(|u| {
            u.claim_label
                .as_ref()
                .and_then(|label| decrypt_json::<String>(key, label).ok())
                .unwrap_or_else(|| user_name(key, u))
        })
        .filter(|name| !name.is_empty())
        .collect()
}

/// The account row says so, or the roster does for the participant this device claimed.
pub fn me_admin(stored: bool, users: &[User], my_user_id: Option<i32>) -> bool {
    stored || users.iter().any(|u| Some(u.id) == my_user_id && u.claimed && u.admin)
}

/// The state for this device, or `None` where the actions do not apply: before the floor, on the
/// demo, a personal or a read-only project.
pub fn security_access(
    project: &ProjectDto,
    users: &[User],
    key: &[u8; 32],
    signed_in: bool,
    stored_admin: bool,
    my_user_id: Option<i32>,
) -> Option<AdminAccess> {
    if !project.key_lifecycle || project.read_only {
        return None;
    }
    let me = signed_in && me_admin(stored_admin, users, my_user_id);
    let holds_secret = try_consume_context::<Signal<LocalStorageState>>()
        .is_some_and(|ls| holds_creator_secret(&ls.read(), project.id));
    Some(admin_access(
        signed_in,
        me,
        project.has_admin || me,
        admin_names(Some(key), users),
        project.has_creator_proof,
        holds_secret,
    ))
}

/// The new group "Recreate the group" opens with: same name, me, and everyone not hidden.
pub fn recreate_prefill(key: &[u8; 32], project: &ProjectDto, users: &[User], my_user_id: Option<i32>) -> GroupPrefill {
    let name = crate::decrypted::decrypt_project(key, project).map(|p| p.name).unwrap_or_default();
    let shown = crate::decrypted::pickable_users(Some(key), users, &[]);
    let me = shown.iter().find(|u| Some(u.id) == my_user_id).map(|u| user_name(key, u)).unwrap_or_default();
    let others = shown.iter().filter(|u| Some(u.id) != my_user_id).map(|u| user_name(key, u)).collect();
    GroupPrefill { name, me, others }
}

/// "Ask Marie or Paul, …" cut around each admin's name, flagged `true`, so the names can stand out.
pub fn ask_admins_parts(names: &[String]) -> Vec<(String, bool)> {
    let sentence = match names.len() {
        0 => tid!("group-ask-an-admin"),
        1 => tid!("group-ask-admin", names: names_or(names)),
        _ => tid!("group-ask-admins", names: names_or(names)),
    };
    let mut parts = Vec::new();
    let mut rest = sentence.as_str();
    for name in names {
        let isolated = format!("\u{2068}{name}\u{2069}");
        let found = rest.split_once(isolated.as_str()).or_else(|| rest.split_once(name.as_str()));
        if let Some((before, after)) = found {
            parts.push((before.to_string(), false));
            parts.push((name.clone(), true));
            rest = after;
        }
    }
    parts.push((rest.to_string(), false));
    parts
        .into_iter()
        .map(|(part, name)| (part.replace(['\u{2068}', '\u{2069}'], ""), name))
        .filter(|(part, _)| !part.is_empty())
        .collect()
}

/// "Marie", "Marie or Paul", "Marie, Paul or Léa".
pub fn names_or(names: &[String]) -> String {
    match names {
        [] => String::new(),
        [one] => one.clone(),
        [rest @ .., last] => tid!("names-or", first: rest.join(", "), last: last.clone()),
    }
}

#[derive(Clone, PartialEq)]
pub struct GroupSecurity {
    pub access: AdminAccess,
    /// Phase 6 (the rotation). `None` keeps an admin's rows inert until then.
    pub on_rotate: Option<EventHandler<()>>,
    pub on_remove: Option<EventHandler<()>>,
    /// Opens the group editor, where a participant is hidden (D8).
    pub on_hide: EventHandler<()>,
    /// A new group with the same name and participants, and so a new link.
    pub on_recreate: EventHandler<()>,
    pub on_sign_in: EventHandler<()>,
    pub on_register: EventHandler<()>,
}

/// The actions sheet's "Group security" block. `pick` closes the sheet before calling out.
#[component]
pub fn GroupSecuritySection(security: GroupSecurity, on_close: EventHandler<()>) -> Element {
    let pick = move |action: EventHandler<()>| {
        move |_: ()| {
            on_close.call(());
            action.call(());
        }
    };
    let tap = move |action: EventHandler<()>| {
        move |_: MouseEvent| {
            on_close.call(());
            action.call(());
        }
    };

    rsx! {
        div { id: "group-security", class: "flex flex-col gap-1",
            span { class: "px-1 text-xs font-semibold text-base-content/70",
                if security.access == AdminAccess::Admin { {tid!("group-security-admin")} } else { {tid!("group-security")} }
            }
            div { class: "bg-base-200 rounded-box overflow-hidden divide-y divide-base-300",
                match security.access.clone() {
                    AdminAccess::Admin => rsx! {
                        ActionRow {
                            id: "group-rotate",
                            title: tid!("group-rotate"),
                            hint: tid!("group-rotate-hint"),
                            onclick: security.on_rotate.map(|h| EventHandler::new(pick(h))),
                            KeyIcon { size: ICON_HEADER }
                        }
                        ActionRow {
                            id: "group-remove",
                            title: tid!("group-remove"),
                            hint: tid!("group-remove-hint"),
                            danger: true,
                            onclick: security.on_remove.map(|h| EventHandler::new(pick(h))),
                            UserMinusIcon { size: ICON_HEADER }
                        }
                    },
                    AdminAccess::AskAdmins(_) => rsx! {
                        ActionRow { id: "group-rotate", title: tid!("group-rotate"), hint: tid!("group-admins-only"), onclick: None, KeyIcon { size: ICON_HEADER } }
                        ActionRow { id: "group-remove", title: tid!("group-remove"), hint: tid!("group-admins-only"), onclick: None, UserMinusIcon { size: ICON_HEADER } }
                    },
                    AdminAccess::SignIn(_) => rsx! {
                        ActionRow { id: "group-rotate", title: tid!("group-rotate"), hint: tid!("group-admins-only-account"), onclick: None, KeyIcon { size: ICON_HEADER } }
                        ActionRow { id: "group-remove", title: tid!("group-remove"), hint: tid!("group-admins-only-account"), onclick: None, UserMinusIcon { size: ICON_HEADER } }
                    },
                    AdminAccess::Creator => rsx! {
                        ActionRow { id: "group-rotate", title: tid!("group-rotate"), hint: tid!("group-admins-only"), onclick: None, KeyIcon { size: ICON_HEADER } }
                        ActionRow { id: "group-remove", title: tid!("group-remove"), hint: tid!("group-admins-only"), onclick: None, UserMinusIcon { size: ICON_HEADER } }
                    },
                    AdminAccess::CreatorElsewhere | AdminAccess::NoAdmin => rsx! {
                        ActionRow {
                            id: "group-rotate",
                            title: tid!("group-rotate"),
                            hint: if security.access == AdminAccess::NoAdmin { tid!("group-no-admin") } else { tid!("group-no-admin-yet") },
                            onclick: None,
                            KeyIcon { size: ICON_HEADER }
                        }
                        ActionRow {
                            id: "group-hide",
                            title: tid!("group-hide"),
                            hint: tid!("group-hide-hint"),
                            onclick: Some(EventHandler::new(pick(security.on_hide))),
                            EyeOffIcon { size: ICON_HEADER }
                        }
                    },
                }
            }
            match security.access.clone() {
                AdminAccess::AskAdmins(names) => rsx! {
                    p { id: "group-ask-admins", role: "note", class: "px-1 pt-1 text-sm text-base-content/70",
                        for (part, name) in ask_admins_parts(&names) {
                            if name {
                                strong { class: "font-semibold text-base-content", "{part}" }
                            } else {
                                "{part}"
                            }
                        }
                    }
                },
                AdminAccess::SignIn(_) | AdminAccess::Creator => rsx! {
                    div { id: "group-sign-in", class: "mt-1 rounded-box border border-dashed border-primary/40 px-3.5 py-3 flex flex-col gap-2",
                        span { class: "flex items-center gap-2 font-semibold",
                            {tid!("group-with-account")}
                            span { class: "badge badge-secondary badge-sm", {tid!("group-free")} }
                        }
                        span { class: "text-sm text-base-content/70",
                            match security.access.clone() {
                                AdminAccess::SignIn(names) if !names.is_empty() => tid!("group-sign-in-hint", names: names_or(&names)),
                                AdminAccess::SignIn(_) => tid!("group-sign-in-hint-any"),
                                _ => tid!("group-creator-hint"),
                            }
                        }
                        div { class: "flex gap-2",
                            button { id: "group-sign-in-login", r#type: "button", class: "btn btn-primary flex-1 rounded-full px-2 leading-tight", onclick: tap(security.on_sign_in), {tid!("login-submit")} }
                            button { id: "group-sign-in-register", r#type: "button", class: "btn btn-outline border-base-300 flex-1 rounded-full px-2 leading-tight", onclick: tap(security.on_register), {tid!("register-submit")} }
                        }
                    }
                },
                AdminAccess::CreatorElsewhere | AdminAccess::NoAdmin => rsx! {
                    div { id: "group-recreate-note", role: "note", class: "mt-1 rounded-box bg-warning/15 px-3.5 py-3 flex flex-col gap-2 text-sm leading-snug",
                        span {
                            if security.access == AdminAccess::NoAdmin { {tid!("group-recreate-hint")} } else { {tid!("group-creator-elsewhere-hint")} }
                        }
                        button {
                            id: "group-recreate",
                            r#type: "button",
                            class: "btn btn-block rounded-full bg-base-100 border-warning/50",
                            onclick: tap(security.on_recreate),
                            {tid!("group-recreate")}
                        }
                    }
                },
                AdminAccess::Admin => rsx! {},
            }
        }
    }
}

/// A locked row stays a button with `aria-disabled`, so a screen reader still reaches its hint.
#[component]
fn ActionRow(
    id: String,
    title: String,
    hint: String,
    #[props(default)] danger: bool,
    onclick: Option<EventHandler<()>>,
    children: Element,
) -> Element {
    let locked = onclick.is_none();
    rsx! {
        button {
            id,
            r#type: "button",
            class: "flex w-full items-center gap-3 min-h-16 px-4 py-2 text-left",
            class: if locked { "text-base-content/60 cursor-default" } else if danger { "text-error" },
            aria_disabled: if locked { "true" },
            onclick: move |_| {
                if let Some(onclick) = onclick {
                    onclick.call(());
                }
            },
            {children}
            span { class: "flex-1 min-w-0 flex flex-col gap-0.5",
                span { class: "font-semibold", "{title}" }
                span { class: "text-sm leading-snug", class: if !locked { "text-base-content/70" }, "{hint}" }
            }
            if locked {
                LockIcon { size: ICON_INLINE }
            }
        }
    }
}

/// `POST /projects/admins`: the server checks that this account is admin and that `user_id` is an
/// account's claimed participant.
pub async fn grant_admin(project_id: uuid::Uuid, user_id: i32) -> Result<(), ServerFnError> {
    crate::common::project_token::scope(project_id);
    api::projects::projects_controller::grant_admin(dioxus::fullstack::Json(shared::GrantAdmin { project_id, user_id })).await
}

/// `POST /projects/admins/revoke` (D18): an admin takes the role from the account that claimed
/// `user_id`. Succession runs as for a departure.
pub async fn revoke_admin(project_id: uuid::Uuid, user_id: i32) -> Result<(), ServerFnError> {
    crate::common::project_token::scope(project_id);
    api::projects::projects_controller::revoke_admin(dioxus::fullstack::Json(shared::RevokeAdmin { project_id, user_id })).await
}

/// `POST /projects/admins/step-down` (D18): this account gives the role up. A last admin hands it
/// to the longest-standing key holder, or leaves the project without one.
pub async fn step_down_admin(project_id: uuid::Uuid) -> Result<(), ServerFnError> {
    crate::common::project_token::scope(project_id);
    api::projects::projects_controller::step_down_admin(dioxus::fullstack::Json(shared::ProjectRef { project_id })).await
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MemberAction {
    MakeAdmin,
    RevokeAdmin,
    StepDown,
    Remove,
}

/// The rows of a member's sheet (artboard T3, D18), for an admin viewer only. Only an account can
/// hold the role, so the role rows need a claimed participant.
pub fn member_actions(viewer_admin: bool, is_me: bool, claimed: bool, admin: bool) -> Vec<MemberAction> {
    match (viewer_admin, is_me, claimed, admin) {
        (false, ..) => vec![],
        (true, true, ..) => vec![MemberAction::StepDown],
        (true, false, true, false) => vec![MemberAction::MakeAdmin, MemberAction::Remove],
        (true, false, true, true) => vec![MemberAction::RevokeAdmin, MemberAction::Remove],
        (true, false, false, _) => vec![MemberAction::Remove],
    }
}

/// A member's actions in the project editor (artboard T3). `remove_rotates`: "Remove from the
/// project" changes the key; otherwise it only hides the participant (D8).
#[component]
pub fn MemberSheet(
    name: String,
    subtitle: String,
    guest: bool,
    actions: Vec<MemberAction>,
    remove_rotates: bool,
    on_action: EventHandler<MemberAction>,
    on_close: EventHandler<()>,
) -> Element {
    crate::common::use_modal_focus("member-sheet");
    let pick = move |action: MemberAction| {
        Some(EventHandler::new(move |_: ()| {
            on_close.call(());
            on_action.call(action);
        }))
    };
    rsx! {
        div {
            id: "member-sheet",
            class: "modal modal-open modal-bottom sm:modal-middle",
            role: "dialog",
            aria_modal: "true",
            aria_labelledby: "member-sheet-title",
            tabindex: "-1",
            div { class: "modal-box max-w-md flex flex-col gap-3",
                div { class: "flex items-center gap-3",
                    crate::common::Avatar { initials: crate::participants::participants_service::initial(&name), size: crate::common::SizeClass::W9, guest }
                    div { class: "flex flex-col min-w-0",
                        h2 { id: "member-sheet-title", class: "font-bold text-lg font-display truncate", "{name}" }
                        span { class: "text-sm text-base-content/70", "{subtitle}" }
                    }
                }
                if !actions.is_empty() {
                    div { class: "bg-base-200 rounded-box overflow-hidden divide-y divide-base-300",
                        for action in actions {
                            match action {
                                MemberAction::MakeAdmin => rsx! {
                                    ActionRow { id: "member-make-admin", title: tid!("member-make-admin"), hint: tid!("member-make-admin-hint"), onclick: pick(action), ShieldIcon { size: ICON_HEADER } }
                                },
                                MemberAction::RevokeAdmin => rsx! {
                                    ActionRow { id: "member-revoke-admin", title: tid!("member-revoke-admin"), hint: tid!("member-revoke-admin-hint"), onclick: pick(action), ShieldIcon { size: ICON_HEADER } }
                                },
                                MemberAction::StepDown => rsx! {
                                    ActionRow { id: "member-step-down", title: tid!("member-step-down"), hint: tid!("member-step-down-hint"), onclick: pick(action), ShieldIcon { size: ICON_HEADER } }
                                },
                                MemberAction::Remove if remove_rotates => rsx! {
                                    ActionRow {
                                        id: "member-remove",
                                        title: tid!("member-remove"),
                                        hint: tid!("member-remove-hint", name: name.clone()),
                                        danger: true,
                                        onclick: pick(action),
                                        UserMinusIcon { size: ICON_HEADER }
                                    }
                                },
                                MemberAction::Remove => rsx! {
                                    ActionRow { id: "member-hide", title: tid!("member-hide"), hint: tid!("group-hide-hint"), onclick: pick(action), EyeOffIcon { size: ICON_HEADER } }
                                },
                            }
                        }
                    }
                }
                button { r#type: "button", class: "btn", onclick: move |_| on_close.call(()), {tid!("close")} }
            }
            div { class: "modal-backdrop", onclick: move |_| on_close.call(()) }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::test_dom::{click, listener_ids, texts};
    use crate::crypto::encrypt_json;
    use dioxus::dioxus_core::VirtualDom;
    use shared::UserPayload;
    use std::cell::RefCell;
    use std::rc::Rc;

    fn user(key: &[u8; 32], id: i32, name: &str, claimed: bool, admin: bool, label: Option<&str>) -> User {
        User {
            id,
            payload: encrypt_json(key, &UserPayload { name: name.into(), removed: false }).unwrap(),
            claimed,
            admin,
            claim_label: label.map(|l| encrypt_json(key, &l.to_string()).unwrap()),
            ..Default::default()
        }
    }

    #[test]
    fn each_device_gets_one_state() {
        let names = vec!["Marie".to_string()];
        assert_eq!(admin_access(true, true, true, names.clone(), false, false), AdminAccess::Admin);
        assert_eq!(admin_access(true, false, true, names.clone(), false, false), AdminAccess::AskAdmins(names.clone()));
        assert_eq!(admin_access(false, false, true, names.clone(), false, false), AdminAccess::SignIn(names.clone()));
        assert_eq!(admin_access(false, false, false, vec![], false, false), AdminAccess::NoAdmin);
        assert_eq!(admin_access(true, false, false, vec![], false, false), AdminAccess::NoAdmin, "an account unlocks nothing without an admin");
    }

    #[test]
    fn without_an_admin_the_creator_proof_picks_the_wording() {
        assert_eq!(admin_access(false, false, false, vec![], true, true), AdminAccess::Creator, "T7: the creating device");
        assert_eq!(admin_access(false, false, false, vec![], true, false), AdminAccess::CreatorElsewhere, "T7b");
        assert_eq!(admin_access(true, false, false, vec![], true, false), AdminAccess::CreatorElsewhere);
        assert_eq!(admin_access(false, false, false, vec![], false, true), AdminAccess::NoAdmin, "a secret without a proof is spent (D19)");
        assert_eq!(admin_access(false, false, true, vec![], true, true), AdminAccess::SignIn(vec![]), "an admin already: the secret is moot");
    }

    #[test]
    fn a_stored_role_on_a_signed_out_device_does_not_unlock() {
        assert_eq!(admin_access(false, true, true, vec![], false, false), AdminAccess::SignIn(vec![]));
    }

    #[test]
    fn admins_are_named_by_their_label_else_their_participant() {
        let key = [3u8; 32];
        let users = vec![
            user(&key, 1, "Marie", true, true, Some("Marie D.")),
            user(&key, 2, "Paul", true, true, None),
            user(&key, 3, "Léa", true, false, Some("Léa")),
            user(&key, 4, "Tom", false, false, None),
        ];
        assert_eq!(admin_names(Some(&key), &users), vec!["Marie D.".to_string(), "Paul".to_string()]);
        assert!(admin_names(None, &users).is_empty());
    }

    #[test]
    fn the_roster_or_the_account_row_makes_me_admin() {
        let key = [3u8; 32];
        let users = vec![user(&key, 1, "Marie", true, true, None), user(&key, 2, "Paul", true, false, None)];
        assert!(me_admin(false, &users, Some(1)));
        assert!(!me_admin(false, &users, Some(2)));
        assert!(!me_admin(false, &users, None));
        assert!(me_admin(true, &users, None));
    }

    #[derive(Props, Clone, PartialEq)]
    struct HarnessProps {
        access: AdminAccess,
        calls: Rc<RefCell<Vec<&'static str>>>,
    }

    #[component]
    fn Harness(props: HarnessProps) -> Element {
        crate::i18n::use_test_i18n();
        let log = |name: &'static str| {
            let calls = props.calls.clone();
            EventHandler::new(move |_: ()| calls.borrow_mut().push(name))
        };
        rsx! {
            GroupSecuritySection {
                security: GroupSecurity {
                    access: props.access.clone(),
                    on_rotate: Some(log("rotate")),
                    on_remove: Some(log("remove")),
                    on_hide: log("hide"),
                    on_recreate: log("recreate"),
                    on_sign_in: log("sign-in"),
                    on_register: log("register"),
                },
                on_close: log("close"),
            }
        }
    }

    fn shown(access: AdminAccess) -> Vec<String> {
        let calls = Rc::new(RefCell::new(Vec::new()));
        let mut dom = VirtualDom::new_with_props(Harness, HarnessProps { access, calls });
        // Fluent wraps interpolated values in bidi isolates.
        texts(&dom.rebuild_to_vec()).into_iter().map(|t| t.replace(['\u{2068}', '\u{2069}'], "")).collect()
    }

    fn clicked(access: AdminAccess) -> Vec<Vec<&'static str>> {
        let calls = Rc::new(RefCell::new(Vec::new()));
        let mut dom = VirtualDom::new_with_props(Harness, HarnessProps { access, calls: calls.clone() });
        let mut runs = Vec::new();
        for id in listener_ids(&dom.rebuild_to_vec(), "click") {
            calls.borrow_mut().clear();
            dom.runtime().handle_event("click", click(), id);
            runs.push(calls.borrow().clone());
        }
        runs
    }

    #[test]
    fn a_non_admin_reads_who_to_ask() {
        let names = vec!["Marie".to_string(), "Paul".to_string()];
        let text = shown(AdminAccess::AskAdmins(names));
        assert!(text.windows(5).any(|w| w == ["Ask ", "Marie", " or ", "Paul", ", the project’s admins."]), "names apart, to set them in bold: {text:?}");
        let three = shown(AdminAccess::AskAdmins(vec!["Marie".into(), "Paul".into(), "Léa".into()]));
        assert!(three.windows(7).any(|w| w == ["Ask ", "Marie", ", ", "Paul", " or ", "Léa", ", the project’s admins."]), "{three:?}");
        let one = shown(AdminAccess::AskAdmins(vec!["Marie".into()]));
        assert!(one.windows(3).any(|w| w == ["Ask ", "Marie", ", the project’s admin."]), "{one:?}");
        assert!(shown(AdminAccess::AskAdmins(vec![])).iter().any(|t| t == "Ask one of the project’s admins."));
    }

    #[test]
    fn locked_rows_call_nothing() {
        for access in [AdminAccess::AskAdmins(vec![]), AdminAccess::SignIn(vec![])] {
            assert!(clicked(access.clone()).iter().all(|run| !run.contains(&"rotate") && !run.contains(&"remove")), "{access:?}");
        }
    }

    #[test]
    fn an_admin_rotates_and_removes_after_the_sheet_closes() {
        let runs = clicked(AdminAccess::Admin);
        assert!(runs.contains(&vec!["close", "rotate"]));
        assert!(runs.contains(&vec!["close", "remove"]));
    }

    #[test]
    fn without_an_admin_no_account_is_offered() {
        let runs = clicked(AdminAccess::NoAdmin);
        assert!(runs.contains(&vec!["close", "hide"]));
        assert!(runs.contains(&vec!["close", "recreate"]));
        assert!(runs.iter().all(|run| !run.contains(&"sign-in") && !run.contains(&"rotate")));
    }

    #[test]
    fn on_the_creating_device_an_account_makes_you_admin() {
        let text = shown(AdminAccess::Creator);
        assert!(text.iter().any(|t| t == "Only you can become admin of this project, by creating an account."), "{text:?}");
        let runs = clicked(AdminAccess::Creator);
        assert!(runs.contains(&vec!["close", "sign-in"]));
        assert!(runs.contains(&vec!["close", "register"]));
        assert!(runs.iter().all(|run| !run.contains(&"rotate") && !run.contains(&"remove") && !run.contains(&"hide")), "{runs:?}");
    }

    #[test]
    fn elsewhere_only_the_creator_can_become_admin_and_hiding_stays() {
        let text = shown(AdminAccess::CreatorElsewhere);
        assert!(
            text.iter().any(|t| t == "Only the person who created the project can become its admin, by creating an account. Otherwise, create a new project to remove someone."),
            "{text:?}"
        );
        assert!(text.iter().any(|t| t == "This project has no admin yet."), "{text:?}");
        assert!(!text.iter().any(|t| t == "This project has no admin. To remove someone, create a new project."));
        let runs = clicked(AdminAccess::CreatorElsewhere);
        assert!(runs.contains(&vec!["close", "hide"]));
        assert!(runs.contains(&vec!["close", "recreate"]));
        assert!(runs.iter().all(|run| !run.contains(&"sign-in") && !run.contains(&"register")), "{runs:?}");
    }

    #[test]
    fn without_a_proof_the_legacy_wording_stays() {
        let text = shown(AdminAccess::NoAdmin);
        assert!(text.iter().any(|t| t == "This project has no admin. To remove someone, create a new project."), "{text:?}");
        assert!(text.iter().any(|t| t == "This project has no admin, so its key can’t change."), "{text:?}");
    }

    #[test]
    fn an_admin_sees_the_role_rows_a_member_allows() {
        use MemberAction::*;
        assert_eq!(member_actions(true, false, true, false), vec![MakeAdmin, Remove]);
        assert_eq!(member_actions(true, false, true, true), vec![RevokeAdmin, Remove]);
        assert_eq!(member_actions(true, false, false, false), vec![Remove], "no account, no role");
        assert_eq!(member_actions(true, true, true, true), vec![StepDown]);
        assert!(member_actions(false, false, true, false).is_empty());
        assert!(member_actions(false, true, true, true).is_empty());
    }

    #[derive(Props, Clone, PartialEq)]
    struct SheetProps {
        actions: Vec<MemberAction>,
        remove_rotates: bool,
        calls: Rc<RefCell<Vec<String>>>,
    }

    #[component]
    fn SheetHarness(props: SheetProps) -> Element {
        crate::i18n::use_test_i18n();
        let calls = props.calls.clone();
        let closes = props.calls.clone();
        rsx! {
            MemberSheet {
                name: "Léa",
                subtitle: "Member with an account",
                guest: false,
                actions: props.actions.clone(),
                remove_rotates: props.remove_rotates,
                on_action: move |a: MemberAction| calls.borrow_mut().push(format!("{a:?}")),
                on_close: move |_| closes.borrow_mut().push("close".into()),
            }
        }
    }

    fn sheet(actions: Vec<MemberAction>, remove_rotates: bool) -> (Vec<String>, Vec<Vec<String>>) {
        let calls = Rc::new(RefCell::new(Vec::new()));
        let mut dom = VirtualDom::new_with_props(SheetHarness, SheetProps { actions, remove_rotates, calls: calls.clone() });
        let edits = dom.rebuild_to_vec();
        let text = texts(&edits).into_iter().map(|t| t.replace(['\u{2068}', '\u{2069}'], "")).collect();
        let mut runs = Vec::new();
        for id in listener_ids(&edits, "click") {
            calls.borrow_mut().clear();
            dom.runtime().handle_event("click", click(), id);
            runs.push(calls.borrow().clone());
        }
        (text, runs)
    }

    #[test]
    fn the_member_sheet_closes_then_acts() {
        let (text, runs) = sheet(vec![MemberAction::RevokeAdmin, MemberAction::Remove], true);
        assert!(text.iter().any(|t| t == "Remove admin role"), "{text:?}");
        assert!(text.iter().any(|t| t == "Changes the project key: Léa won’t be able to read anything new."), "{text:?}");
        assert!(runs.contains(&vec!["close".to_string(), "RevokeAdmin".to_string()]), "{runs:?}");
        assert!(runs.contains(&vec!["close".to_string(), "Remove".to_string()]), "{runs:?}");
    }

    #[test]
    fn without_a_rotation_remove_says_it_only_hides() {
        let (text, _) = sheet(vec![MemberAction::Remove], false);
        assert!(text.iter().any(|t| t == "Hide from the lists"), "{text:?}");
        assert!(!text.iter().any(|t| t == "Remove from the project"), "{text:?}");
        let (mine, runs) = sheet(vec![MemberAction::StepDown], false);
        assert!(mine.iter().any(|t| t == "Stop being admin"), "{mine:?}");
        assert!(runs.contains(&vec!["close".to_string(), "StepDown".to_string()]), "{runs:?}");
    }

    #[test]
    fn signed_out_the_account_card_says_an_admin_grants_the_role() {
        let text = shown(AdminAccess::SignIn(vec!["Marie".into()]));
        assert!(text.iter().any(|t| t == "An admin of the project, Marie, can then give you the role."), "{text:?}");
        let runs = clicked(AdminAccess::SignIn(vec![]));
        assert!(runs.contains(&vec!["close", "sign-in"]));
        assert!(runs.contains(&vec!["close", "register"]));
    }
}
