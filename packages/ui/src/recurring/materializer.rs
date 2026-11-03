use api::recurring::recurring_controller::materialize_recurring_expense;
use dioxus::fullstack::Json;
use dioxus::prelude::*;
use shared::{MaterializeRecurringRequest, ProjectStatus, ProjectSync, MAX_OCCURRENCES_PER_CALL};
use std::collections::{HashSet, VecDeque};
use std::sync::Arc;
use uuid::Uuid;

use super::requests::materialize_request;
use super::schedule::due;
use super::store::RecurringData;
use crate::common::{
    format_date, is_offline_error, is_rate_limited, is_recurring_stale_error, sleep, Flash, LocalStorageState,
    REQUEST_PACE_MS,
};
use crate::expenses::helpers::expenses_page_helpers::For;
use crate::expenses::helpers::project_data::ProjectData;
use crate::tid;

type Sync = Resource<Option<Result<For<ProjectSync>, ServerFnError>>>;

struct Job {
    claim: (Uuid, i64),
    project_id: Uuid,
    request: MaterializeRecurringRequest,
    count: i64,
}

/// Writes every due occurrence of the open project, once per rule version.
///
/// Measured against the server's date from the sync, never the device clock. The rules go one
/// after another, paced, and the project resyncs once at the end: one call and one resync per rule
/// at once was a burst nginx refused from sixteen due rules. Every outcome ends in that resync or
/// nothing: a 409 means another member won and their expenses arrive with it, and a network error
/// or a refusal leaves the claim for the next open. The resync is also what sends the next chunk
/// of a long catch-up.
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
    let mut queue: Signal<VecDeque<Job>> = use_signal(VecDeque::new);
    let mut running = use_signal(|| false);
    use_effect(move || {
        let r = recurring();
        let d = data();
        let (Some(project_id), Some(today), Some(k)) = (r.project_id, r.server_date, key()) else {
            return;
        };
        if !is_online() || d.project_id != Some(project_id) || d.project_status != ProjectStatus::Ongoing || d.read_only {
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
            let Ok(request) = materialize_request(&k, project_id, rule, &dates, today, actor, summary) else {
                continue;
            };
            claimed.write().insert(claim);
            queue.write().push_back(Job { claim, project_id, request, count: dates.len() as i64 });
        }
        if queue.peek().is_empty() || *running.peek() {
            return;
        }
        running.set(true);
        let (mut sync, mut flash) = (sync, flash);
        spawn(async move {
            let (mut added, mut resync) = (0i64, false);
            let mut first = true;
            loop {
                let Some(job) = queue.write().pop_front() else { break };
                if !first {
                    sleep(REQUEST_PACE_MS).await;
                }
                first = false;
                // Checked again per rule: the pace leaves time to leave the project, or for it to be
                // closed or frozen under us.
                let still_open = {
                    let d = data.peek();
                    d.project_id == Some(job.project_id)
                        && d.project_status == ProjectStatus::Ongoing
                        && !d.read_only
                        && key.peek().is_some()
                        && *is_online.peek()
                };
                if !still_open {
                    release(&mut claimed, &mut queue, job.claim);
                    break;
                }
                crate::common::project_token::scope(job.request.project_id);
                match materialize_recurring_expense(Json(job.request)).await {
                    Ok(()) => {
                        added += job.count;
                        resync = true;
                    }
                    Err(e) if is_recurring_stale_error(&e) => resync = true,
                    Err(e) if is_offline_error(&e) || is_rate_limited(&e) => {
                        release(&mut claimed, &mut queue, job.claim);
                        break;
                    }
                    Err(_) => {}
                }
            }
            running.set(false);
            if added > 0 {
                flash.set(Some(Flash::ok(tid!("recurring-added", count: added))));
            }
            if resync {
                sync.restart();
            }
        });
    });
}

/// Hands this claim and every queued one back, for the next open to try again.
fn release(claimed: &mut Signal<HashSet<(Uuid, i64)>>, queue: &mut Signal<VecDeque<Job>>, claim: (Uuid, i64)) {
    let mut claimed = claimed.write();
    claimed.remove(&claim);
    for job in queue.write().drain(..) {
        claimed.remove(&job.claim);
    }
}
