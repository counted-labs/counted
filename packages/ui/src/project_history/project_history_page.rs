use api::history::history_controller::get_project_history;
use chrono::NaiveDateTime;
use dioxus::prelude::*;
use crate::tid;
use shared::{HistoryAction, HistoryPayload};
use uuid::Uuid;

use crate::common::{error_message, format_date, pending, project_key, AppHeader, PullToRefresh};
use crate::crypto::decrypt_json;
use crate::expenses::hooks::use_project_store::ProjectStore;
use crate::icons::{PencilIcon, PlusIcon, TrashIcon, ICON_INLINE};
use crate::route::Route;

#[derive(Clone, PartialEq)]
struct DecryptedEntry {
    action: HistoryAction,
    actor_name: String,
    summary: String,
    created_at: NaiveDateTime,
}

fn decrypt_history(
    key: &[u8; 32],
    history_list: &[shared::HistoryEntry],
    names: Option<&std::collections::HashMap<i32, String>>,
) -> Vec<DecryptedEntry> {
    history_list
        .iter()
        .map(|e| {
            let payload = decrypt_json::<HistoryPayload>(key, &e.payload).ok();
            let actor_name = payload
                .as_ref()
                .and_then(|p| p.actor_user_id)
                .or(e.actor_user_id)
                .and_then(|id| names.and_then(|m| m.get(&id).cloned()))
                .unwrap_or_else(|| "?".to_string());
            let action = payload.as_ref().and_then(|p| p.action.clone()).unwrap_or_else(|| e.action.clone());
            let summary = payload.map(|p| p.summary).unwrap_or_default();
            DecryptedEntry { action, actor_name, summary, created_at: e.created_at }
        })
        .collect()
}

#[component]
pub fn ProjectHistoryPage(project_id: Uuid) -> Element {
    let key = use_memo(use_reactive!(|project_id| project_key(project_id)));

    // History is append-only and unbounded, so it stays its own fetch. The participant names come
    // from the project store, which is already decrypted - this page used to be the third place to
    // fetch and decrypt the same roster.
    let mut history = use_resource(move || async move {
        crate::common::project_token::scope(project_id);
        get_project_history(project_id).await
    });
    let store = use_context::<ProjectStore>();
    // A memo, not the body: the history is unbounded and the body re-runs on every sync state.
    let entries = use_memo(use_reactive!(|project_id| {
        let names = store.data_for(project_id).map(|d| d.user_names.clone());
        match (key(), &*history.read()) {
            (Some(k), Some(Ok(history_list))) => decrypt_history(&k, history_list, names.as_ref()),
            _ => vec![],
        }
    }));
    let key = key();
    let entries = entries();

    let loading = history.read().is_none();
    let error: Option<String> =
        history.read().as_ref().and_then(|r| r.as_ref().err().map(error_message));

    // use_effect is client-only (does not run during SSR). By gating all real content
    // behind `hydrated`, the SSR render and the first WASM render both produce the
    // skeleton, giving Dioxus a clean hydration match and preventing the "root is
    // undefined" crash that occurs when the two renders diverge.
    let mut hydrated = use_signal(|| false);
    use_effect(move || {
        hydrated.set(true);
    });

    rsx! {
        // The container is the scroller, as on every other page: the pull-to-refresh gesture finds
        // its scroller with `closest('.app-container')` and would miss an inner one.
        div { class: "container app-container bg-base-100 overflow-auto p-4 pb-24 max-w-md w-full mx-auto flex flex-col gap-4",
            PullToRefresh {
                on_refresh: move |_| {
                    history.restart();
                    let mut sync = store.sync;
                    sync.restart();
                },
                busy: pending(&history) || pending(&store.sync),
            }
            AppHeader {
                title: tid!("project-history-title"),
                back_button_route: Route::ExpensesPage { project_id },
            }

            if !hydrated() || loading {
                HistorySkeleton {}
            } else if key.is_none() {
                div { role: "alert", class: "alert alert-warning my-4", {tid!("missing-encryption-key-title")} }
            } else if let Some(e) = error {
                div { role: "alert", class: "alert alert-error my-4", "{e}" }
            } else if entries.is_empty() {
                div { class: "flex flex-col items-center justify-center flex-1 text-base-content/70",
                    p { class: "text-sm", {tid!("history-empty")} }
                }
            } else {
                HistoryTimeline { entries }
            }
        }
    }
}

#[component]
fn HistorySkeleton() -> Element {
    rsx! {
        div { class: "flex-1 pt-4 flex flex-col gap-5", aria_hidden: "true",
            for _ in 0..5 {
                div { class: "flex items-center gap-3",
                    div { class: "skeleton w-8 h-8 rounded-full shrink-0" }
                    div { class: "flex flex-col gap-2 flex-1",
                        div { class: "skeleton h-3 w-1/4 rounded" }
                        div { class: "skeleton h-4 w-3/5 rounded" }
                        div { class: "skeleton h-3 w-1/5 rounded" }
                    }
                }
            }
        }
    }
}

#[derive(PartialEq, Props, Clone)]
struct HistoryTimelineProps {
    entries: Vec<DecryptedEntry>,
}

#[component]
fn HistoryTimeline(props: HistoryTimelineProps) -> Element {
    let mut groups: Vec<(String, Vec<DecryptedEntry>)> = vec![];
    for entry in &props.entries {
        let date_label = format_date(entry.created_at.date());
        if let Some(last) = groups.last_mut() {
            if last.0 == date_label {
                last.1.push(entry.clone());
                continue;
            }
        }
        groups.push((date_label, vec![entry.clone()]));
    }

    rsx! {
        div { class: "pb-6",
            for (date, group_entries) in &groups {
                h2 { class: "divider divider-start text-xs text-base-content/70 my-3",
                    "{date}"
                }
                ul { class: "timeline timeline-vertical timeline-compact",
                    for (i, entry) in group_entries.iter().enumerate() {
                        HistoryItem {
                            entry: entry.clone(),
                            is_last: i == group_entries.len() - 1,
                        }
                    }
                }
            }
        }
    }
}

#[derive(PartialEq, Props, Clone)]
struct HistoryItemProps {
    entry: DecryptedEntry,
    is_last: bool,
}

#[component]
fn HistoryItem(props: HistoryItemProps) -> Element {
    let entry = &props.entry;
    let time_str = entry.created_at.format("%H:%M").to_string();

    let (icon_bg, icon_color, badge_classes, badge_label) = match &entry.action {
        HistoryAction::Created => (
            "bg-success/10",
            "text-success",
            "badge badge-soft badge-success badge-xs",
            "history-kind-add",
        ),
        HistoryAction::Deleted => (
            "bg-error/10",
            "text-error",
            "badge badge-soft badge-error badge-xs",
            "history-kind-delete",
        ),
        _ => (
            "bg-primary/10",
            "text-primary",
            "badge badge-soft badge-primary badge-xs",
            "history-kind-edit",
        ),
    };

    rsx! {
        li {
            hr { class: "bg-base-200" }
            div { class: "timeline-middle px-3",
                div {
                    class: "w-8 h-8 rounded-full flex items-center justify-center {icon_bg} {icon_color}",
                    match &entry.action {
                        HistoryAction::Created => rsx! { PlusIcon { size: ICON_INLINE } },
                        HistoryAction::Deleted => rsx! { TrashIcon { size: ICON_INLINE } },
                        _ => rsx! { PencilIcon { size: ICON_INLINE } },
                    }
                }
            }
            div { class: "timeline-end timeline-box text-sm w-full shadow-none border-base-200 bg-base-200/40",
                div { class: "flex items-center justify-between mb-1",
                    span { class: "{badge_classes}", {tid!(badge_label)} }
                    span { class: "text-xs text-base-content/70", "{time_str}" }
                }
                p { class: "font-medium text-base-content leading-snug", "{entry.summary}" }
                p { class: "text-xs text-base-content/70 mt-1", {tid!("history-by", name: entry.actor_name.clone())} }
            }
            if !props.is_last {
                hr { class: "bg-base-200" }
            }
        }
    }
}

#[cfg(any(test, fuzzing))]
pub(crate) fn exercise_hostile(entries: &[shared::HistoryEntry], names: &std::collections::HashMap<i32, String>) {
    assert_eq!(decrypt_history(&crate::common::test_fixtures::TEST_KEY, entries, Some(names)).len(), entries.len());
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::hostile;
    use crate::common::test_fixtures::test_key;
    use crate::crypto::encrypt_json;
    use proptest::prelude::*;
    use shared::{HistoryEntity, HistoryEntry};

    fn action() -> impl Strategy<Value = HistoryAction> {
        prop_oneof![Just(HistoryAction::Created), Just(HistoryAction::Updated), Just(HistoryAction::Deleted)]
    }

    fn payload() -> impl Strategy<Value = HistoryPayload> {
        (any::<String>(), proptest::option::of(hostile::id()), proptest::option::of(action()), proptest::option::of(hostile::id()))
            .prop_map(|(summary, actor_user_id, action, entity_id)| HistoryPayload { summary, actor_user_id, action, entity: Some(HistoryEntity::Expense), entity_id })
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(300))]

        #[test]
        fn any_history_row_is_listed(rows in prop::collection::vec((payload(), any::<String>(), proptest::option::of(hostile::id()), action(), hostile::timestamp()), 0..8), names in prop::collection::hash_map(hostile::id(), any::<String>(), 0..4)) {
            let key = test_key();
            let entries: Vec<HistoryEntry> = rows
                .into_iter()
                .enumerate()
                .map(|(i, (p, raw, actor_user_id, action, created_at))| HistoryEntry {
                    id: i as i64,
                    project_id: Uuid::nil(),
                    actor_user_id,
                    actor_account_id: None,
                    action,
                    entity: HistoryEntity::Project,
                    entity_id: None,
                    payload: if i % 2 == 0 { encrypt_json(&key, &p).unwrap() } else { crate::crypto::encrypt(&key, &raw).unwrap() },
                    created_at,
                })
                .collect();
            exercise_hostile(&entries, &names);
        }
    }
}
