//! The project's categories (designs 1b-1d): the list, the sheet that creates or edits one, and the
//! one that deletes. Every save is one edit applied to the payload as stored now, online only, with
//! one history entry. Logic lives in `categories::custom` and the helpers below.

use dioxus::prelude::*;
use shared::{EditableProject, HistoryAction, HistoryContext, HistoryEntity, HistoryPayload, ProjectPayload};
use std::collections::{HashMap, HashSet};
use uuid::Uuid;

use crate::categories::custom::{apply_edit, new_custom_id, CategoryEdit, CategoryError, Summary, MAX_CUSTOM, MAX_NAME_CHARS, OTHER};
use crate::categories::{category_label, parent_emoji, ProjectCategories, CHART_CATEGORIES};
use crate::common::{error_message, haptic, read_from_ls, update_project_fresh, ConfirmDialog, DialogAction, EmojiGrid, Haptic, ProjectKey};
use crate::crypto::{decrypt_json, encrypt_padded_json};
use crate::expenses::helpers::project_data::ProjectData;
use crate::expenses::hooks::use_project_store::ProjectStore;
use crate::icons::{ChevronRightIcon, CloseIcon, ICON_HEADER, ICON_INLINE};
use crate::recurring::store::RecurringData;
use crate::tid;

#[derive(Clone, Debug, PartialEq)]
pub enum Target {
    New,
    Custom(String),
    Builtin(&'static str),
}

/// Expenses per shown category id, and every category id an expense or a rule cites. `None`
/// when this device has not decrypted the whole project and its rules: then no count is shown, or
/// no tombstone dropped.
pub fn usage(data: Option<&ProjectData>, rules: Option<&RecurringData>) -> (Option<HashMap<String, usize>>, Option<HashSet<String>>) {
    let Some(data) = data.filter(|d| d.loaded) else { return (None, None) };
    let mut counts: HashMap<String, usize> = HashMap::new();
    for row in &data.expenses {
        *counts.entry(row.category.id.clone()).or_default() += 1;
    }
    let referenced = rules.filter(|_| data.unreadable_expenses == 0).map(|rules| {
        data.expenses
            .iter()
            .filter_map(|r| r.expense.category.clone())
            .chain(rules.rules.iter().filter_map(|rule| rule.payload.template.category.clone()))
            .collect()
    });
    (Some(counts), referenced)
}

/// What the sheet's fields make of `target`. A built-in's name equal to its own label is no rename.
pub fn edit_for(
    target: &Target,
    new_id: Option<String>,
    name: &str,
    emoji: Option<String>,
    parent: &'static str,
    hidden: bool,
    default_label: &str,
) -> Option<CategoryEdit> {
    Some(match target {
        Target::New => CategoryEdit::Create { id: new_id?, name: name.to_string(), emoji, parent },
        Target::Custom(id) => CategoryEdit::Update { id: id.clone(), name: name.to_string(), emoji, parent },
        Target::Builtin(id) => CategoryEdit::Builtin {
            id,
            name: crate::categories::custom::clean_name(name).filter(|n| n != default_label),
            emoji,
            hidden,
        },
    })
}

/// "Créées : 2 · Renommées : 1 · Masquées : 1", as keys and counts; empty when nothing changed.
pub fn summary_parts(summary: Summary) -> Vec<(&'static str, usize)> {
    [
        ("categories-summary-created", summary.created),
        ("categories-summary-renamed", summary.renamed),
        ("categories-summary-hidden", summary.hidden),
    ]
    .into_iter()
    .filter(|(_, n)| *n > 0)
    .collect()
}

pub fn summary_text(categories: &ProjectCategories) -> String {
    let parts = summary_parts(categories.summary());
    if parts.is_empty() {
        return tid!("categories-summary-default");
    }
    parts.into_iter().map(|(key, count)| tid!(key, count: count as i64)).collect::<Vec<_>>().join(" · ")
}

async fn save_edit(
    project_id: Uuid,
    key: [u8; 32],
    edit: CategoryEdit,
    referenced: Option<HashSet<String>>,
    summary: String,
) -> Result<ProjectCategories, String> {
    let actor_user_id = read_from_ls().projects.iter().find(|p| p.project_id == project_id).and_then(|p| p.user_id);
    let refused: std::cell::Cell<Option<CategoryError>> = std::cell::Cell::new(None);
    let result = update_project_fresh(project_id, |stored| {
        let mut payload: ProjectPayload = decrypt_json(&key, &stored.payload)?;
        if let Err(e) = apply_edit(&mut payload.extra, &edit, referenced.as_ref()) {
            refused.set(Some(e));
            return Err(e.key().to_string());
        }
        let history = encrypt_padded_json(
            &key,
            &HistoryPayload {
                summary: summary.clone(),
                actor_user_id,
                action: Some(HistoryAction::Updated),
                entity: Some(HistoryEntity::Project),
                entity_id: None,
            },
        )
        .ok()
        .map(|p| HistoryContext { actor_user_id: None, payload: p });
        Ok(EditableProject {
            id: project_id,
            payload: Some(encrypt_padded_json(&key, &payload)?),
            status: None,
            history,
            expected_iv: None,
            notify_closed: false,
        })
    })
    .await;
    match result {
        Ok(dto) => Ok(decrypt_json::<ProjectPayload>(&key, &dto.payload)
            .map(|p| ProjectCategories::from_payload(&p))
            .unwrap_or_default()),
        Err(e) => Err(match refused.get() {
            Some(c) => tid!(c.key()),
            None => error_message(&e),
        }),
    }
}

fn history_summary(edit: &CategoryEdit, label: String) -> String {
    match edit {
        CategoryEdit::Create { .. } => tid!("history-category-created", name: label),
        CategoryEdit::Delete { .. } => tid!("history-category-deleted", name: label),
        _ => tid!("history-category-edited", name: label),
    }
}

#[component]
pub fn CategoryManager(
    project_id: Uuid,
    initial: ProjectCategories,
    on_close: EventHandler<()>,
    #[props(default)] on_changed: Option<EventHandler<ProjectCategories>>,
) -> Element {
    crate::common::use_modal_focus("category-manager");
    let key_ctx = use_context::<ProjectKey>().0;
    let is_online = try_use_context::<Signal<bool>>();
    let store = try_use_context::<ProjectStore>();
    let current_usage = move || {
        let data = store.and_then(|s| s.data_for(project_id));
        let rules = store.and_then(|s| s.recurring_for(project_id));
        usage(data.as_deref(), rules.as_ref())
    };
    let (counts, _) = current_usage();
    let mut categories = use_signal(|| initial.clone());
    let mut editing: Signal<Option<Target>> = use_signal(|| None);
    let mut deleting: Signal<Option<String>> = use_signal(|| None);
    let mut saving = use_signal(|| false);
    let mut error: Signal<Option<String>> = use_signal(|| None);

    let cats = categories();
    let count_of = |id: &str| counts.as_ref().map(|c| c.get(id).copied().unwrap_or(0));
    let expense_count = |id: &str| count_of(id).map(|n| tid!("charts-expense-count", count: n as i64));
    let other_label = cats.shown_builtin(OTHER).label();
    let custom_count = cats.custom_count();

    let mut save = move |edit: CategoryEdit, label: String| {
        if is_online.is_some_and(|o| !o()) {
            error.set(Some(tid!("categories-offline")));
            return;
        }
        let Some(key) = key_ctx() else { return };
        let summary = history_summary(&edit, label);
        let (_, referenced) = current_usage();
        saving.set(true);
        error.set(None);
        spawn(async move {
            match save_edit(project_id, key, edit, referenced, summary).await {
                Ok(next) => {
                    haptic(Haptic::Success);
                    categories.set(next.clone());
                    editing.set(None);
                    deleting.set(None);
                    if let Some(on_changed) = on_changed {
                        on_changed.call(next);
                    }
                    if let Some(store) = store.filter(|s| s.data_for(project_id).is_some()) {
                        let mut sync = store.sync;
                        sync.restart();
                    }
                }
                Err(message) => error.set(Some(message)),
            }
            saving.set(false);
        });
    };

    let mut open = move |target: Target| {
        error.set(None);
        editing.set(Some(target));
    };

    rsx! {
        div {
            id: "category-manager",
            class: "modal modal-open modal-bottom sm:modal-middle",
            role: "dialog",
            aria_modal: "true",
            aria_labelledby: "category-manager-title",
            tabindex: "-1",
            div { class: "modal-box max-w-md p-0",
                div { class: "flex flex-col gap-3 px-6 pt-5 pb-6",
                div { class: "flex items-center gap-2",
                    h2 { id: "category-manager-title", class: "flex-1 font-bold text-lg font-display", {tid!("categories-title")} }
                    span { id: "category-manager-counter", class: "badge badge-ghost", "{custom_count} / {MAX_CUSTOM}" }
                    button {
                        r#type: "button",
                        class: "btn btn-circle size-11 min-h-11 border-0 bg-base-200 text-base-content/80",
                        aria_label: tid!("close"),
                        onclick: move |_| on_close.call(()),
                        CloseIcon { size: ICON_HEADER }
                    }
                }
                if let Some(message) = error().filter(|_| editing().is_none()) {
                    div { role: "alert", class: "alert alert-error text-sm", "{message}" }
                }
                if saving() && editing().is_none() {
                    span { class: "loading loading-spinner loading-sm self-center", aria_label: tid!("saving") }
                }
                h3 { class: "text-xs font-bold uppercase tracking-wide text-base-content/70", {tid!("categories-yours")} }
                if custom_count > 0 {
                    ul { class: "bg-base-200 rounded-box overflow-hidden divide-y divide-base-300",
                        for shown in cats.custom_shown() {
                            {
                                let subtitle = [Some(tid!("categories-in-parent", parent: cats.shown_builtin(shown.parent).label())), expense_count(&shown.id)]
                                    .into_iter()
                                    .flatten()
                                    .collect::<Vec<_>>()
                                    .join(" · ");
                                let id = shown.id.clone();
                                rsx! {
                                    CategoryRow {
                                        key: "{shown.id}",
                                        id: format!("category-row-{}", shown.id),
                                        emoji: shown.emoji.clone(),
                                        title: shown.label(),
                                        subtitle,
                                        muted: false,
                                        pill: None,
                                        on_open: move |_| open(Target::Custom(id.clone())),
                                    }
                                }
                            }
                        }
                    }
                }
                button {
                    id: "category-add",
                    r#type: "button",
                    class: "btn btn-soft btn-primary btn-sm self-start rounded-full",
                    disabled: custom_count >= MAX_CUSTOM,
                    onclick: move |_| {
                        haptic(Haptic::Light);
                        open(Target::New);
                    },
                    {tid!("categories-add")}
                }
                h3 { class: "text-xs font-bold uppercase tracking-wide text-base-content/70", {tid!("categories-builtin")} }
                ul { class: "bg-base-200 rounded-box overflow-hidden divide-y divide-base-300",
                    for id in CHART_CATEGORIES.iter().copied() {
                        {
                            let hidden = cats.is_hidden(id);
                            let shown = if hidden { cats.override_of(id).map(|e| (e.name.clone(), e.emoji.clone())) } else { None };
                            let (title, emoji) = match shown {
                                Some((name, emoji)) => (name.unwrap_or_else(|| category_label(id)), emoji.unwrap_or_else(|| parent_emoji(id).to_string())),
                                None => {
                                    let s = cats.shown_builtin(id);
                                    (s.label(), s.emoji)
                                }
                            };
                            let renamed = cats.override_of(id).is_some_and(|e| e.name.is_some());
                            let subtitle = if hidden {
                                tid!("categories-hidden-hint", other: other_label.clone())
                            } else if renamed {
                                tid!("categories-renamed-from", name: category_label(id))
                            } else if id == OTHER {
                                tid!("categories-always-shown")
                            } else {
                                expense_count(id).unwrap_or_default()
                            };
                            rsx! {
                                CategoryRow {
                                    key: "{id}",
                                    id: format!("category-row-{}", CHART_CATEGORIES.iter().position(|c| *c == id).unwrap_or(0)),
                                    emoji,
                                    title,
                                    subtitle,
                                    muted: hidden,
                                    pill: hidden.then(|| tid!("categories-hidden")),
                                    on_open: move |_| open(Target::Builtin(id)),
                                }
                            }
                        }
                    }
                }
                }
            }
            div { class: "modal-backdrop", onclick: move |_| on_close.call(()) }
        }
        if let Some(target) = editing() {
            {
                rsx! {
                    CategoryEditSheet {
                        target: target.clone(),
                        categories: cats.clone(),
                        saving: saving(),
                        error: error(),
                        on_save: move |(edit, label): (CategoryEdit, String)| save(edit, label),
                        on_delete: {
                            let target = target.clone();
                            move |_| {
                                if let Target::Custom(id) = &target {
                                    error.set(None);
                                    editing.set(None);
                                    deleting.set(Some(id.clone()));
                                }
                            }
                        },
                        on_close: move |_| editing.set(None),
                    }
                }
            }
        }
        if let Some(id) = deleting() {
            {
                let shown = cats.resolve(Some(&id), "");
                let parent = cats.shown_builtin(shown.parent);
                let label = shown.label();
                let parent = format!("{} {}", parent.emoji, parent.label());
                let body = match count_of(&id) {
                    Some(n) => tid!("categories-delete-body", count: n as i64, parent: parent),
                    None => tid!("categories-delete-body-unknown", parent: parent),
                };
                rsx! {
                    ConfirmDialog {
                        id: "category-delete",
                        title: tid!("categories-delete-title", name: label.clone()),
                        message: format!("{body}\n\n{}", tid!("categories-delete-warning")),
                        action_sheet: true,
                        actions: vec![DialogAction {
                            id: "category-delete-confirm",
                            label: tid!("delete"),
                            class: "btn-error",
                            on_click: EventHandler::new(move |_| {
                                deleting.set(None);
                                save(CategoryEdit::Delete { id: id.clone() }, label.clone());
                            }),
                        }],
                        on_cancel: move |_| deleting.set(None),
                    }
                }
            }
        }
    }
}

#[component]
fn CategoryRow(
    id: String,
    emoji: String,
    title: String,
    subtitle: String,
    muted: bool,
    pill: Option<String>,
    on_open: EventHandler<()>,
) -> Element {
    let mut press_task: Signal<Option<dioxus::core::Task>> = use_signal(|| None);
    let mut long_pressed = use_signal(|| false);
    let mut cancel_press = move || {
        if let Some(t) = press_task.write().take() {
            t.cancel();
        }
    };
    rsx! {
        li {
            button {
                id: "{id}",
                r#type: "button",
                class: if muted { "w-full flex items-center gap-3 px-3 py-2.5 min-h-13 text-left row-press opacity-70" } else { "w-full flex items-center gap-3 px-3 py-2.5 min-h-13 text-left hover:bg-base-300 row-press" },
                aria_haspopup: "dialog",
                ontouchstart: move |_| {
                    long_pressed.set(false);
                    let task = spawn(async move {
                        crate::common::sleep(500).await;
                        long_pressed.set(true);
                        haptic(Haptic::Medium);
                        on_open.call(());
                    });
                    press_task.set(Some(task));
                },
                ontouchcancel: move |_| cancel_press(),
                onpointercancel: move |_| cancel_press(),
                ontouchend: move |_| cancel_press(),
                onclick: move |_| {
                    if long_pressed() {
                        long_pressed.set(false);
                        return;
                    }
                    haptic(Haptic::Light);
                    on_open.call(());
                },
                span { class: "text-xl w-7 text-center shrink-0", aria_hidden: "true", "{emoji}" }
                span { class: "flex-1 min-w-0 flex flex-col",
                    span { class: "font-semibold truncate", dir: "auto", "{title}" }
                    if !subtitle.is_empty() {
                        span { class: "text-xs text-base-content/70 truncate", dir: "auto", "{subtitle}" }
                    }
                }
                if let Some(pill) = pill {
                    span { class: "badge badge-ghost badge-sm shrink-0", "{pill}" }
                }
                ChevronRightIcon { size: ICON_INLINE }
            }
        }
    }
}

#[component]
fn CategoryEditSheet(
    target: Target,
    categories: ProjectCategories,
    saving: bool,
    error: Option<String>,
    on_save: EventHandler<(CategoryEdit, String)>,
    on_delete: EventHandler<()>,
    on_close: EventHandler<()>,
) -> Element {
    crate::common::use_modal_focus("category-edit-sheet");
    let initial = target.clone();
    let start = categories.clone();
    let mut name = use_signal(move || match &initial {
        Target::New => String::new(),
        Target::Custom(id) => start.customs().find(|e| &e.id == id).and_then(|e| e.name.clone()).unwrap_or_default(),
        Target::Builtin(id) => start.override_of(id).and_then(|e| e.name.clone()).unwrap_or_default(),
    });
    let initial = target.clone();
    let start = categories.clone();
    let mut emoji: Signal<Option<String>> = use_signal(move || match &initial {
        Target::New => None,
        Target::Custom(id) => start.customs().find(|e| &e.id == id).and_then(|e| e.emoji.clone()),
        Target::Builtin(id) => start.override_of(id).and_then(|e| e.emoji.clone()),
    });
    let initial = target.clone();
    let start = categories.clone();
    let mut parent: Signal<&'static str> = use_signal(move || match &initial {
        Target::Custom(id) => start.customs().find(|e| &e.id == id).map(|e| e.parent).unwrap_or(OTHER),
        _ => OTHER,
    });
    let initial = target.clone();
    let mut hidden = use_signal(|| matches!(&initial, Target::Builtin(id) if categories.is_hidden(id)));

    let builtin = match &target {
        Target::Builtin(id) => Some(*id),
        _ => None,
    };
    let default_label = builtin.map(category_label).unwrap_or_default();
    let fallback_emoji = match builtin {
        Some(id) => parent_emoji(id).to_string(),
        None => categories.shown_builtin(parent()).emoji,
    };
    let title = match &target {
        Target::New => tid!("categories-new"),
        _ => tid!("categories-edit"),
    };
    let typed = name().chars().count();
    let shown_name = if name().trim().is_empty() { default_label.clone() } else { name().trim().to_string() };
    let parent_label = categories.shown_builtin(parent()).label();
    let submit_target = target.clone();
    let submit_default = default_label.clone();

    rsx! {
        div {
            id: "category-edit-sheet",
            class: "modal modal-open modal-bottom sm:modal-middle",
            role: "dialog",
            aria_modal: "true",
            aria_labelledby: "category-edit-title",
            tabindex: "-1",
            div { class: "modal-box max-w-md p-0",
                div { class: "flex flex-col gap-4 px-6 pt-5 pb-6",
                div { class: "flex items-center gap-2",
                    h2 { id: "category-edit-title", class: "flex-1 font-bold text-lg font-display", "{title}" }
                    button {
                        r#type: "button",
                        class: "btn btn-circle size-11 min-h-11 border-0 bg-base-200 text-base-content/80",
                        aria_label: tid!("close"),
                        onclick: move |_| on_close.call(()),
                        CloseIcon { size: ICON_HEADER }
                    }
                }
                div { class: "flex flex-col gap-1.5",
                    div { class: "flex justify-between text-sm",
                        label { class: "font-medium", r#for: "category-name", {tid!("field-name")} }
                        span { id: "category-name-counter", class: "text-xs text-base-content/70 tabular-nums", "{typed} / {MAX_NAME_CHARS}" }
                    }
                    label { class: "input w-full gap-2",
                        span { class: "text-lg", aria_hidden: "true", {emoji().unwrap_or_else(|| fallback_emoji.clone())} }
                        input {
                            id: "category-name",
                            class: "grow min-w-0",
                            r#type: "text",
                            dir: "auto",
                            maxlength: "{MAX_NAME_CHARS}",
                            placeholder: "{default_label}",
                            value: "{name}",
                            oninput: move |e| name.set(e.value().chars().take(MAX_NAME_CHARS).collect()),
                        }
                    }
                }
                div { class: "flex flex-col gap-1.5",
                    span { class: "text-sm font-medium", {tid!("emoji-picker-label")} }
                    EmojiGrid {
                        id: "category-emoji",
                        selected: emoji(),
                        none_preview: Some(fallback_emoji.clone()),
                        onpick: move |picked| emoji.set(picked),
                    }
                }
                if builtin.is_none() {
                    div { class: "flex flex-col gap-1.5",
                        label { class: "text-sm font-medium", r#for: "category-parent", {tid!("categories-parent")} }
                        select {
                            id: "category-parent",
                            class: "select w-full",
                            aria_describedby: "category-parent-hint",
                            onchange: move |e| {
                                if let Some(p) = crate::categories::custom::builtin_id(&e.value()) {
                                    parent.set(p);
                                }
                            },
                            for p in CHART_CATEGORIES.iter().copied() {
                                {
                                    let shown = categories.shown_builtin(p);
                                    rsx! {
                                        option { value: "{p}", selected: parent() == p, "{shown.emoji} {shown.label()}" }
                                    }
                                }
                            }
                        }
                        if !shown_name.is_empty() {
                            span { id: "category-parent-hint", class: "text-xs text-base-content/70",
                                {tid!("categories-parent-hint", name: shown_name.clone(), parent: parent_label.clone())}
                            }
                        }
                    }
                }
                if builtin.is_some_and(|id| id != OTHER) {
                    label { class: "flex items-center gap-3 px-3.5 py-3 rounded-box bg-base-200 text-sm cursor-pointer",
                        span { class: "flex-1",
                            {tid!("categories-hide")}
                            span { class: "block text-xs text-base-content/70", {tid!("categories-hide-hint", other: categories.shown_builtin(OTHER).label())} }
                        }
                        input {
                            id: "category-hidden",
                            r#type: "checkbox",
                            role: "switch",
                            class: "toggle toggle-primary",
                            checked: hidden(),
                            onchange: move |e| hidden.set(e.checked()),
                        }
                    }
                }
                if let Some(message) = error {
                    div { role: "alert", class: "alert alert-error text-sm", "{message}" }
                }
                div { class: "flex gap-2.5",
                    if matches!(target, Target::Custom(_)) {
                        button {
                            id: "category-delete",
                            r#type: "button",
                            class: "btn btn-ghost text-error rounded-full",
                            disabled: saving,
                            onclick: move |_| on_delete.call(()),
                            {tid!("delete")}
                        }
                    }
                    button {
                        id: "category-save",
                        r#type: "button",
                        class: "btn btn-primary flex-1 rounded-full",
                        disabled: saving || (builtin.is_none() && name().trim().is_empty()),
                        onclick: move |_| {
                            let edit = edit_for(&submit_target, new_custom_id(), &name(), emoji(), parent(), hidden(), &submit_default);
                            if let Some(edit) = edit {
                                let label = if name().trim().is_empty() { submit_default.clone() } else { name().trim().to_string() };
                                on_save.call((edit, label));
                            }
                        },
                        if saving {
                            span { class: "loading loading-spinner loading-sm" }
                        }
                        if matches!(target, Target::New) { {tid!("categories-create")} } else { {tid!("save")} }
                    }
                }
                }
            }
            div { class: "modal-backdrop", onclick: move |_| on_close.call(()) }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_builtin_renamed_to_its_own_label_is_no_rename() {
        let edit = edit_for(&Target::Builtin("Transport"), None, " Transport ", None, OTHER, false, "Transport");
        assert_eq!(edit, Some(CategoryEdit::Builtin { id: "Transport", name: None, emoji: None, hidden: false }));
        let edit = edit_for(&Target::Builtin("Loisirs"), None, "Sorties", Some("🎉".into()), OTHER, true, "Loisirs");
        assert_eq!(edit, Some(CategoryEdit::Builtin { id: "Loisirs", name: Some("Sorties".into()), emoji: Some("🎉".into()), hidden: true }));
        assert_eq!(edit_for(&Target::New, None, "x", None, OTHER, false, ""), None);
        assert!(matches!(edit_for(&Target::New, Some("c-0000000a".into()), "x", None, "Shopping", false, ""), Some(CategoryEdit::Create { parent: "Shopping", .. })));
    }

    #[test]
    fn the_summary_lists_only_what_changed() {
        assert!(summary_parts(Summary::default()).is_empty());
        assert_eq!(summary_parts(Summary { created: 2, renamed: 0, hidden: 1 }), vec![("categories-summary-created", 2), ("categories-summary-hidden", 1)]);
    }

    #[test]
    fn usage_counts_rows_by_category_and_collects_every_cited_id() {
        let k = crate::common::test_fixtures::test_key();
        let live = crate::expenses::helpers::project_data::LiveData {
            project_id: Uuid::nil(),
            project: None,
            users: vec![],
            expenses: vec![
                crate::common::test_fixtures::make_expense_with_category(&k, 1, "Biocoop", "c-0000000a"),
                crate::common::test_fixtures::make_expense_with_category(&k, 2, "Taxi", "Transport"),
            ],
            payments: vec![],
            version: None,
        };
        let data = crate::expenses::helpers::project_data::build(Some(k), Some(&live), None);
        let (counts, referenced) = usage(Some(&data), Some(&RecurringData::default()));
        assert_eq!(counts.unwrap().get("Transport"), Some(&1));
        assert!(referenced.unwrap().contains("c-0000000a"));
        assert_eq!(usage(Some(&data), None).1, None, "rules not loaded: no tombstone may go");
        assert_eq!(usage(None, None), (None, None));
    }

    /// `tid!` is called on these as values, which the crate-wide scanner does not see.
    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn every_dynamic_key_is_a_known_message() {
        let known = crate::i18n::locale_ids(crate::i18n::FALLBACK);
        let errors = [CategoryError::TooMany, CategoryError::Full, CategoryError::NameRequired, CategoryError::Missing].map(CategoryError::key);
        let summaries = summary_parts(Summary { created: 1, renamed: 1, hidden: 1 }).into_iter().map(|(k, _)| k);
        for key in errors.into_iter().chain(summaries) {
            assert!(known.contains(key), "{key} is not defined in the fallback locale");
        }
    }
}
