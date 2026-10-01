use api::recurring::recurring_controller::materialize_recurring_expense;
use dioxus::fullstack::Json;
use dioxus::prelude::*;
use shared::{ProjectStatus, ProjectSync, MAX_OCCURRENCES_PER_CALL};
use std::collections::HashSet;
use std::sync::Arc;
use uuid::Uuid;

use super::requests::materialize_request;
use super::schedule::due;
use super::store::RecurringData;
use crate::common::{format_date, is_offline_error, is_recurring_stale_error, Flash, LocalStorageState};
use crate::expenses::helpers::expenses_page_helpers::For;
use crate::expenses::helpers::project_data::ProjectData;
use crate::tid;

type Sync = Resource<Option<Result<For<ProjectSync>, ServerFnError>>>;

/// Writes every due occurrence of the open project, once per rule version.
///
/// Measured against the server's date from the sync, never the device clock. Every outcome ends
/// in a resync or nothing: a 409 means another member won and their expenses arrive with it, and a
/// network error leaves the claim for the next open. A success resyncs too, which is also what
/// sends the next chunk of a long catch-up.
pub fn use_recurring_materializer(
    recurring: Memo<RecurringData>,
    data: Memo<Arc<ProjectData>>,
    key: Signal<Option<[u8; 32]>>,
    is_online: Signal<bool>,
    ls_ctx: Signal<LocalStorageState>,
    sync: Sync,
    flash: Signal<Option<Flash>>,
) {
    let mut claimed: Signal<HashSet<(Uuid, i64)>> = use_signal(HashSet::new);
    use_effect(move || {
        let r = recurring();
        let d = data();
        let (Some(project_id), Some(today), Some(k)) = (r.project_id, r.server_date, key()) else {
            return;
        };
        if !is_online() || d.project_id != Some(project_id) || d.project_status != ProjectStatus::Ongoing {
            return;
        }
        let member = ls_ctx
            .peek()
            .projects
            .iter()
            .find(|p| p.project_id == project_id)
            .and_then(|p| p.user_id)
            .filter(|id| d.users.iter().any(|u| u.id == *id));
        for rule in r.rules.iter().filter(|r| !r.payload.paused) {
            let dates = due(&rule.payload.rule, rule.payload.next, today, MAX_OCCURRENCES_PER_CALL);
            let Some(actor) = rule.author_id.or(member) else { continue };
            let claim = (rule.id, rule.version);
            if dates.is_empty() || claimed.peek().contains(&claim) {
                continue;
            }
            let name = rule.payload.template.name.clone();
            let summary =
                |date| tid!("history-recurring-added", name: name.clone(), date: format_date(date));
            let Ok(req) = materialize_request(&k, project_id, rule, &dates, actor, summary) else {
                continue;
            };
            claimed.write().insert(claim);
            let count = dates.len() as i64;
            let (mut sync, mut flash) = (sync, flash);
            spawn(async move {
                match materialize_recurring_expense(Json(req)).await {
                    Ok(()) => {
                        flash.set(Some(Flash::ok(tid!("recurring-added", count: count))));
                        sync.restart();
                    }
                    Err(e) if is_recurring_stale_error(&e) => sync.restart(),
                    Err(e) if is_offline_error(&e) => {
                        claimed.write().remove(&claim);
                    }
                    Err(_) => {}
                }
            });
        }
    });
}
