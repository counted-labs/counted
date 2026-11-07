//! Moving an expense to another project (designs R8, R9, R9b, R10): the destination's own form,
//! prefilled, writes the expense there; only then is it deleted here. Two writes, in that order,
//! so a failure in between leaves the expense in both projects and never in neither.

use dioxus::prelude::*;
use shared::{ExpensePayload, ExpenseType, ProjectDto, ProjectPayload, ProjectStatus};
use std::collections::VecDeque;
use uuid::Uuid;

use crate::common::{
    key_of, write_queue, Flash, LocalStorageState, OpKind, ProjectTile, QueuedOp,
};
use crate::categories::ProjectCategories;
use crate::crypto::decrypt_json;
use crate::decrypted::{pickable_users, project_status};
use crate::expenses::helpers::delete_expense_action::delete_expense_request;
use crate::icons::{ChevronRightIcon, CloseIcon, WalletIcon};
use crate::tid;

/// Everything the destination's form needs, gathered on the source's page while its key is the one
/// in context.
#[derive(Clone, Debug, PartialEq)]
pub struct MoveDraft {
    pub from_project: Uuid,
    pub from_name: String,
    pub from_key: [u8; 32],
    pub from_user_id: Option<i32>,
    pub expense_id: i32,
    pub to_project: Uuid,
    pub to_name: String,
    pub to_personal: bool,
    /// Description, estimate and fields from newer builds - kept, as an edit keeps them.
    pub payload: ExpensePayload,
    /// What the amount was typed in: the original when the expense was entered in another
    /// currency, so the destination converts from what was paid rather than from a conversion.
    pub amount: f64,
    pub currency: String,
    /// The built-in the source files the expense under, for a destination without its category.
    pub category_parent: &'static str,
}

impl MoveDraft {
    /// The category as picked when the destination has it, its parent when it is a custom category
    /// of the source only.
    pub fn category_for(&self, destination: &ProjectCategories) -> Option<String> {
        let picked = self.payload.category.as_deref()?;
        Some(if destination.knows(picked) { picked.to_string() } else { self.category_parent.to_string() })
    }

    pub fn expense_type(&self) -> ExpenseType {
        match self.payload.expense_type.as_str() {
            "gain" => ExpenseType::Gain,
            _ => ExpenseType::Expense,
        }
    }
}

/// `(amount, currency)` to prefill: the original when the payload carries a usable conversion.
pub fn typed_amount(payload: &ExpensePayload, project_currency: &str) -> (f64, String) {
    match (&payload.source_currency, payload.source_amount, payload.rate) {
        (Some(c), Some(a), Some(r)) if a.is_finite() && shared::is_valid_rate(r) => (a, c.clone()),
        _ => (payload.amount, project_currency.to_string()),
    }
}

/// A transfer settles a debt between two people of this project; it has no meaning elsewhere.
pub fn can_move(expense_type: &ExpenseType) -> bool {
    *expense_type != ExpenseType::Transfer
}

#[derive(Clone, Debug, PartialEq)]
pub struct Destination {
    pub project_id: Uuid,
    pub name: String,
    pub emoji: Option<String>,
    pub currency: String,
    /// Participants still in the project, when this device has them cached.
    pub members: Option<usize>,
    pub personal: bool,
    pub current: bool,
}

/// The source first (shown, not pickable), then every project that can take an expense: held with
/// its key, ongoing, not read-only. "Mes dépenses" comes last. A source the list cache does not hold
/// yet - a project created moments ago - comes from its own cached copy.
pub fn destinations(list: &[ProjectDto], state: &LocalStorageState, from: Uuid) -> Vec<Destination> {
    let source = state
        .projects
        .iter()
        .find(|lp| lp.project_id == from)
        .and_then(|lp| lp.cached_project.as_ref())
        .filter(|_| !list.iter().any(|p| p.id == from));
    let mut out: Vec<Destination> = list
        .iter()
        .chain(source)
        .filter_map(|p| {
            let entry = state.projects.iter().find(|lp| lp.project_id == p.id)?;
            let key = key_of(entry)?;
            let payload: ProjectPayload = decrypt_json(&key, &p.payload).ok()?;
            let current = p.id == from;
            let open = project_status(Some(&key), p) == ProjectStatus::Ongoing && !p.read_only;
            (current || open).then(|| Destination {
                project_id: p.id,
                personal: state.personal_project == Some(p.id),
                emoji: payload.emoji(),
                name: payload.name,
                currency: payload.currency,
                members: entry.cached_users.as_ref().map(|u| pickable_users(Some(&key), u, &[]).len()),
                current,
            })
        })
        .collect();
    out.sort_by_key(|d| (!d.current, d.personal));
    out
}

#[component]
pub fn MoveExpenseSheet(
    destinations: Vec<Destination>,
    /// The source's display name; `None` when it is "Mes dépenses".
    from_group: Option<String>,
    on_pick: EventHandler<Uuid>,
    on_close: EventHandler<()>,
) -> Element {
    crate::common::use_modal_focus("move-expense-sheet");
    let from_currency = destinations.iter().find(|d| d.current).map(|d| d.currency.clone()).unwrap_or_default();
    rsx! {
        div {
            id: "move-expense-sheet",
            class: "modal modal-open modal-bottom sm:modal-middle",
            role: "dialog",
            aria_modal: "true",
            aria_labelledby: "move-expense-title",
            tabindex: "-1",
            div { class: "modal-box max-w-md p-0",
                div { class: "flex flex-col gap-3 px-6 pt-5 pb-6",
                    div { class: "flex items-center gap-2",
                        h2 { id: "move-expense-title", class: "flex-1 min-w-0 truncate font-bold text-lg font-display",
                            {tid!("move-expense-title")}
                        }
                        button {
                            id: "move-expense-close",
                            r#type: "button",
                            class: "btn btn-circle size-11 min-h-11 border-0 bg-base-200 text-base-content/80",
                            aria_label: tid!("close"),
                            onclick: move |_| on_close.call(()),
                            CloseIcon {}
                        }
                    }
                    ul { class: "flex flex-col",
                        for d in destinations {
                            {
                                let id = d.project_id;
                                let detail = destination_detail(&d, &from_currency);
                                let tile = if d.personal {
                                    rsx! {
                                        span { class: "size-[2.625rem] shrink-0 rounded-[13px] bg-primary/10 text-primary flex items-center justify-center",
                                            WalletIcon {}
                                        }
                                    }
                                } else {
                                    rsx! { ProjectTile { name: d.name.clone(), id, emoji: d.emoji.clone() } }
                                };
                                rsx! {
                                    li { key: "{id}",
                                        if d.current {
                                            div { class: "flex items-center gap-3 py-2.5 px-1 opacity-55", aria_disabled: "true",
                                                {tile}
                                                span { class: "flex-1 min-w-0 flex flex-col",
                                                    span { class: "font-semibold truncate", "{d.name}" }
                                                    span { class: "text-xs text-base-content/70", "{detail}" }
                                                }
                                                span { class: "text-xs font-semibold text-base-content/70", {tid!("move-current")} }
                                            }
                                        } else {
                                            button {
                                                id: "move-to-{id}",
                                                r#type: "button",
                                                class: "w-full flex items-center gap-3 py-2.5 px-1 text-left rounded-xl hover:bg-base-200 row-press",
                                                onclick: move |_| on_pick.call(id),
                                                {tile}
                                                span { class: "flex-1 min-w-0 flex flex-col",
                                                    span { class: "font-semibold truncate", "{d.name}" }
                                                    span { class: "text-xs text-base-content/70", "{detail}" }
                                                }
                                                span { class: "text-base-content/50", ChevronRightIcon {} }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    match from_group {
                        Some(group) => rsx! {
                            p { class: "rounded-xl bg-warning/15 text-sm leading-snug px-3.5 py-3",
                                {tid!("move-note-group", project: group)}
                            }
                        },
                        None => rsx! {
                            p { class: "rounded-xl bg-success/10 text-sm leading-snug px-3.5 py-3",
                                {tid!("move-note-personal")}
                            }
                        },
                    }
                }
            }
            div { class: "modal-backdrop", onclick: move |_| on_close.call(()) }
        }
    }
}

/// "7 membres · EUR", "Perso · EUR", or the currency with the conversion it implies.
fn destination_detail(d: &Destination, from_currency: &str) -> String {
    let currency = if d.currency == from_currency || d.current {
        d.currency.clone()
    } else {
        tid!("move-converted", currency: d.currency.clone())
    };
    match (d.personal, d.members) {
        (true, _) => format!("{} · {currency}", tid!("personal-badge")),
        (false, Some(n)) => format!("{} · {currency}", tid!("move-members", count: n as i64)),
        (false, None) => currency,
    }
}

/// Once the destination has queued the expense (`add_op`): delete it here, with this project's key
/// and identity. The delete queues behind the add and waits on it, so the replay keeps the order and
/// an add the server drops takes the delete with it.
pub fn finish_move(
    draft: &MoveDraft,
    add_op: Uuid,
    mut pending_ops: Signal<VecDeque<QueuedOp>>,
    mut flash: Signal<Option<Flash>>,
) {
    let req = delete_expense_request(
        &draft.from_key,
        draft.expense_id,
        draft.from_project,
        tid!("history-expense-moved", name: draft.payload.name.clone()),
        draft.from_user_id.unwrap_or(0),
    );
    let delete = QueuedOp {
        after: Some(add_op),
        ..QueuedOp::new(Uuid::new_v4(), &draft.payload.name, OpKind::DeleteExpense(req))
    };
    pending_ops.write().push_back(delete);
    write_queue(&pending_ops.read());
    flash.set(Some(Flash::ok(tid!("move-done", project: draft.to_name.clone()))));
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::test_fixtures::{make_project, make_user, test_key};
    use crate::common::LocalStorageProject;
    use crate::crypto::{encrypt_json, key_to_fragment};

    fn payload(amount: f64) -> ExpensePayload {
        ExpensePayload {
            name: "Hotel".into(),
            amount,
            expense_type: "expense".into(),
            date: "2026-10-11".into(),
            description: None,
            category: None,
            source_currency: None,
            source_amount: None,
            rate: None,
            recurring_id: None,
            estimate: false,
            author_id: None,
            shares: None,
            extra: Default::default(),
        }
    }

    #[test]
    fn the_prefill_starts_from_what_was_paid() {
        assert_eq!(typed_amount(&payload(92.0), "EUR"), (92.0, "EUR".to_string()));
        let converted = ExpensePayload {
            source_currency: Some("CHF".into()),
            source_amount: Some(100.0),
            rate: Some(0.92),
            ..payload(92.0)
        };
        assert_eq!(typed_amount(&converted, "EUR"), (100.0, "CHF".to_string()));
        let broken = ExpensePayload { rate: Some(0.0), ..converted };
        assert_eq!(typed_amount(&broken, "EUR"), (92.0, "EUR".to_string()), "an unusable rate is ignored");
    }

    #[test]
    fn a_transfer_stays_where_it_was_made() {
        assert!(can_move(&ExpenseType::Expense));
        assert!(can_move(&ExpenseType::Gain));
        assert!(!can_move(&ExpenseType::Transfer));
    }

    fn held(p: &ProjectDto, k: &[u8; 32], users: Option<Vec<shared::User>>) -> LocalStorageProject {
        LocalStorageProject {
            project_id: p.id,
            encryption_key: Some(key_to_fragment(k)),
            cached_users: users,
            ..Default::default()
        }
    }

    fn with(k: &[u8; 32], name: &str, f: impl FnOnce(&mut ProjectPayload)) -> ProjectDto {
        let mut p = make_project(k, Uuid::new_v4(), name, "EUR");
        let mut pp: ProjectPayload = decrypt_json(k, &p.payload).unwrap();
        f(&mut pp);
        p.payload = encrypt_json(k, &pp).unwrap();
        p
    }

    #[test]
    fn destinations_are_the_open_projects_this_device_can_write_to() {
        let k = test_key();
        let source = with(&k, "Offsite", |_| {});
        let corsica = with(&k, "Corsica", |p| p.set_emoji(Some("🏝️")));
        let mine = with(&k, "Mes dépenses", |p| { p.extra.insert("kind".into(), "personal".into()); });
        let archived = with(&k, "Old", |p| p.status = Some(ProjectStatus::Archived));
        let mut demo = with(&k, "Demo", |_| {});
        demo.read_only = true;
        let keyless = with(&k, "Locked", |_| {});
        let state = LocalStorageState {
            projects: vec![
                held(&source, &k, None),
                held(&mine, &k, None),
                held(&corsica, &k, Some(vec![make_user(&k, 1, "A"), make_user(&k, 2, "B")])),
                held(&archived, &k, None),
                held(&demo, &k, None),
            ],
            personal_project: Some(mine.id),
            ..Default::default()
        };
        let list = [source.clone(), mine.clone(), corsica.clone(), archived, demo, keyless];
        let got = destinations(&list, &state, source.id);
        let names: Vec<&str> = got.iter().map(|d| d.name.as_str()).collect();
        assert_eq!(names, ["Offsite", "Corsica", "Mes dépenses"], "source first, personal last");
        assert!(got[0].current);
        assert_eq!(got[1].members, Some(2));
        assert_eq!(got[1].emoji.as_deref(), Some("🏝️"));
        assert_eq!(got[0].emoji, None);
        assert!(got[2].personal);
    }

    /// The project just created is in no list cache yet: the move banner must still name it.
    #[test]
    fn a_source_missing_from_the_list_cache_is_read_from_its_own_entry() {
        let k = test_key();
        let source = with(&k, "Offsite", |_| {});
        let corsica = with(&k, "Corsica", |_| {});
        let mut entry = held(&source, &k, None);
        entry.cached_project = Some(source.clone());
        let state = LocalStorageState { projects: vec![entry, held(&corsica, &k, None)], ..Default::default() };

        let got = destinations(std::slice::from_ref(&corsica), &state, source.id);
        let names: Vec<&str> = got.iter().map(|d| d.name.as_str()).collect();
        assert_eq!(names, ["Offsite", "Corsica"]);
        assert!(got[0].current);
    }

    #[test]
    fn the_draft_keeps_the_type_and_never_becomes_a_transfer() {
        let draft = MoveDraft {
            from_project: Uuid::nil(),
            from_name: "A".into(),
            from_key: test_key(),
            from_user_id: None,
            expense_id: 1,
            to_project: Uuid::nil(),
            to_name: "B".into(),
            to_personal: false,
            payload: ExpensePayload { expense_type: "gain".into(), ..payload(10.0) },
            amount: 10.0,
            currency: "EUR".into(),
            category_parent: "Autres",
        };
        assert_eq!(draft.expense_type(), ExpenseType::Gain);
    }

    #[test]
    fn a_custom_category_the_destination_lacks_becomes_its_parent() {
        let draft = |category: Option<&str>| MoveDraft {
            from_project: Uuid::nil(),
            from_name: "A".into(),
            from_key: test_key(),
            from_user_id: None,
            expense_id: 1,
            to_project: Uuid::nil(),
            to_name: "B".into(),
            to_personal: false,
            payload: ExpensePayload { category: category.map(String::from), ..payload(10.0) },
            amount: 10.0,
            currency: "EUR".into(),
            category_parent: "Nourriture",
        };
        let empty = ProjectCategories::default();
        let has = ProjectCategories::from_value(Some(&serde_json::json!([{ "id": "c-0000000a", "name": "Bio", "parent": "Shopping" }])));
        assert_eq!(draft(Some("c-0000000a")).category_for(&empty).as_deref(), Some("Nourriture"));
        assert_eq!(draft(Some("c-0000000a")).category_for(&has).as_deref(), Some("c-0000000a"));
        assert_eq!(draft(Some("Transport")).category_for(&empty).as_deref(), Some("Transport"));
        assert_eq!(draft(None).category_for(&empty), None);
    }
}
