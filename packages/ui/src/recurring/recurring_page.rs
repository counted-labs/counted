use dioxus::prelude::*;
use uuid::Uuid;

use super::actions::{paused, rate_kept, resumed, save_rule, stop_rule, with_rate};
use super::dialog::{ChoiceDialog, DialogAction};
use super::edit_rule_modal::EditRuleModal;
use super::labels::{ends_label, schedule_label};
use super::model::DecryptedRule;
use super::schedule::upcoming;
use super::split::side_amounts;
use super::view::{
    monthly_total, my_monthly_share, next_date, progress, rate_drift, sorted, state, RuleState,
};
use crate::categories::{find_category, parent_emoji};
use crate::common::{format_date, fx_cache, pending, read_from_ls, AppHeader, Flash, ProjectKey, PullToRefresh};
use crate::expenses::helpers::project_data::ProjectData;
use crate::expenses::hooks::use_fx_rates::use_fx_rates;
use crate::expenses::hooks::use_project_store::ProjectStore;
use crate::icons::{CloseIcon, RepeatIcon, ICON_HEADER, ICON_INLINE};
use crate::route::Route;
use crate::tid;

#[component]
pub fn RecurringPage(project_id: Uuid) -> Element {
    rsx! { RecurringView { project_id, open: None } }
}

#[component]
pub fn RecurringRulePage(project_id: Uuid, rule_id: Uuid) -> Element {
    rsx! { RecurringView { project_id, open: Some(rule_id) } }
}

pub fn emoji_of(rule: &DecryptedRule) -> &'static str {
    let t = &rule.payload.template;
    match t.category.as_deref() {
        Some(c) => parent_emoji(c),
        None => find_category(&t.name).emoji,
    }
}

fn payer_names(rule: &DecryptedRule, data: &ProjectData) -> String {
    rule.payload
        .template
        .payers
        .entries
        .iter()
        .map(|(id, _)| data.user_names.get(id).cloned().unwrap_or_else(|| "?".to_string()))
        .collect::<Vec<_>>()
        .join(", ")
}

/// Occurrences of `rule` present in the project, and how many of them await their real amount.
pub fn occurrence_counts(rule_id: Uuid, data: &ProjectData) -> (usize, usize) {
    let mine = data.expenses.iter().filter(|r| r.expense.recurring_id == Some(rule_id));
    let (all, estimates) = mine.fold((0, 0), |(a, e), r| (a + 1, e + r.expense.estimate as usize));
    (all, estimates)
}

#[component]
fn RecurringView(project_id: Uuid, open: Option<Uuid>) -> Element {
    let store = use_context::<ProjectStore>();
    let key_ctx = use_context::<ProjectKey>().0;
    let flash = use_context::<Signal<Option<Flash>>>();
    let mut selected: Signal<Option<Uuid>> = use_signal(|| open);
    let mut editing: Signal<Option<Uuid>> = use_signal(|| None);
    let mut confirm: Signal<Option<(Uuid, bool)>> = use_signal(|| None);
    let mut hydrated = use_signal(|| false);
    use_effect(move || hydrated.set(true));

    let data = store.data_for(project_id).filter(|d| d.loaded);
    let recurring = store.recurring_for(project_id);
    let rules = recurring.as_ref().map(|r| sorted(&r.rules)).unwrap_or_default();
    let today = recurring.as_ref().map(|r| r.today()).unwrap_or_else(|| chrono::Utc::now().date_naive());
    let stored_user_id = read_from_ls()
        .projects
        .iter()
        .find(|p| p.project_id == project_id)
        .and_then(|p| p.user_id);

    let needs_fx_rules = rules.iter().any(|r| r.payload.template.conversion().is_some());
    let needs_fx = use_memo(use_reactive!(|needs_fx_rules| needs_fx_rules));
    let fx = use_fx_rates(needs_fx);
    let currency = data.as_ref().map(|d| d.currency.clone()).unwrap_or_default();
    let today_rate = |rule: &DecryptedRule| -> Option<f64> {
        let (_, from, _) = rule.payload.template.conversion()?;
        fx_cache::cross_rate(fx.read().as_ref()?, from, &currency)
    };

    let refresh = move || {
        let mut sync = store.sync;
        sync.restart();
    };

    rsx! {
        div { class: "container app-container bg-base-100 overflow-auto p-4 pb-36 max-w-md w-full mx-auto flex flex-col gap-3",
            PullToRefresh { on_refresh: move |_| refresh(), busy: pending(&store.sync) }
            AppHeader {
                title: tid!("recurring-title"),
                back_button_route: Route::ExpensesPage { project_id },
                sticky: true,
            }
            match (hydrated(), data.clone(), key_ctx()) {
                (true, Some(d), Some(key)) => {
                    let groups = [
                        (RuleState::Active, "recurring-active"),
                        (RuleState::Paused, "recurring-paused"),
                        (RuleState::Finished, "recurring-finished"),
                    ];
                    let monthly = monthly_total(&rules);
                    let mine = stored_user_id.map(|uid| my_monthly_share(&rules, uid));
                    let sheet_rule = selected().and_then(|id| rules.iter().find(|r| r.id == id).cloned());
                    let edit_rule = editing().and_then(|id| rules.iter().find(|r| r.id == id).cloned());
                    let confirm_rule = confirm().and_then(|(id, resume)| rules.iter().find(|r| r.id == id).cloned().map(|r| (r, resume)));
                    rsx! {
                        if rules.is_empty() {
                            div { class: "flex flex-col items-center gap-3 py-12 text-center text-base-content/70",
                                RepeatIcon { size: ICON_HEADER }
                                p { class: "text-sm", {tid!("recurring-empty")} }
                            }
                        } else {
                            div { id: "recurring-totals", class: "flex rounded-box shadow-soft bg-base-100",
                                div { class: "flex-1 min-w-0 px-4 py-3",
                                    p { class: "text-xs text-base-content/70", {tid!("recurring-per-month")} }
                                    p { class: "font-display font-extrabold text-xl bg-gradient-brand bg-clip-text text-transparent tabular-nums", "{monthly:.0} {d.currency}" }
                                }
                                if let Some(share) = mine {
                                    div { class: "flex-1 min-w-0 px-4 py-3 border-l border-base-200",
                                        p { class: "text-xs text-base-content/70", {tid!("recurring-your-share")} }
                                        p { class: "font-display font-extrabold text-xl tabular-nums", "{share:.0} {d.currency}" }
                                    }
                                }
                            }
                            for (group , label) in groups {
                                if rules.iter().any(|r| state(&r.payload) == group) {
                                    h2 { class: "text-xs font-bold uppercase tracking-wide text-base-content/70 px-1 pt-2", {tid!(label)} }
                                    ul { class: "flex flex-col gap-2",
                                        for rule in rules.iter().filter(|r| state(&r.payload) == group) {
                                            {
                                                let id = rule.id;
                                                let (_, estimates) = occurrence_counts(id, &d);
                                                let drift = today_rate(rule).and_then(|rate| rate_drift(&rule.payload.template, rate));
                                                rsx! {
                                                    RuleCard {
                                                        key: "{id}",
                                                        rule: rule.clone(),
                                                        currency: d.currency.clone(),
                                                        payers: payer_names(rule, &d),
                                                        estimates,
                                                        drift,
                                                        on_open: move |_| selected.set(Some(id)),
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }

                        if let Some(rule) = sheet_rule {
                            {
                                let id = rule.id;
                                let (added, _) = occurrence_counts(id, &d);
                                let rate_now = today_rate(&rule);
                                let drift = rate_now.and_then(|rate| rate_drift(&rule.payload.template, rate));
                                let (use_rule, keep_rule) = (rule.clone(), rule.clone());
                                let pause_rule = rule.clone();
                                rsx! {
                                    RuleSheet {
                                        rule: rule.clone(),
                                        data: d.clone(),
                                        added,
                                        drift: drift.zip(rate_now),
                                        on_close: move |_| selected.set(None),
                                        on_edit: move |_| {
                                            selected.set(None);
                                            editing.set(Some(id));
                                        },
                                        on_pause: move |_| {
                                            if pause_rule.payload.paused {
                                                confirm.set(Some((id, true)));
                                            } else {
                                                save_rule(key, project_id, pause_rule.clone(), paused(&pause_rule.payload), stored_user_id, flash, refresh);
                                            }
                                        },
                                        on_stop: move |_| confirm.set(Some((id, false))),
                                        on_use_rate: move |rate: f64| {
                                            let mut p = use_rule.payload.clone();
                                            p.template = with_rate(&p.template, rate);
                                            save_rule(key, project_id, use_rule.clone(), p, stored_user_id, flash, refresh);
                                        },
                                        on_keep_rate: move |rate: f64| {
                                            let mut p = keep_rule.payload.clone();
                                            p.template = rate_kept(&p.template, rate);
                                            save_rule(key, project_id, keep_rule.clone(), p, stored_user_id, flash, refresh);
                                        },
                                    }
                                }
                            }
                        }

                        if let Some(rule) = edit_rule {
                            EditRuleModal {
                                rule,
                                users: d.users.clone(),
                                project_id,
                                currency: d.currency.clone(),
                                today,
                                actor: stored_user_id,
                                on_close: move |_| editing.set(None),
                                on_saved: move |_| {
                                    editing.set(None);
                                    refresh();
                                },
                            }
                        }

                        if let Some((rule, resume)) = confirm_rule {
                            {
                                let name = rule.payload.template.name.clone();
                                if resume {
                                    let next = resumed(&rule.payload, today);
                                    let next_on = next_date(&next).map(format_date).unwrap_or_default();
                                    rsx! {
                                        ChoiceDialog {
                                            title: tid!("recurring-resume-title", name: name.clone()),
                                            message: tid!("recurring-resume-message", date: next_on),
                                            actions: vec![DialogAction {
                                                id: "recurring-resume-confirm",
                                                label: tid!("recurring-resume"),
                                                class: "btn-primary",
                                                on_click: EventHandler::new(move |_| {
                                                    confirm.set(None);
                                                    save_rule(key, project_id, rule.clone(), next.clone(), stored_user_id, flash, refresh);
                                                }),
                                            }],
                                            on_cancel: move |_| confirm.set(None),
                                        }
                                    }
                                } else {
                                    rsx! {
                                        ChoiceDialog {
                                            title: tid!("recurring-stop-title", name: name.clone()),
                                            message: tid!("recurring-stop-message"),
                                            actions: vec![DialogAction {
                                                id: "recurring-stop-confirm",
                                                label: tid!("recurring-stop"),
                                                class: "btn-error",
                                                on_click: EventHandler::new(move |_| {
                                                    confirm.set(None);
                                                    selected.set(None);
                                                    stop_rule(key, project_id, &rule, stored_user_id, flash, refresh);
                                                }),
                                            }],
                                            on_cancel: move |_| confirm.set(None),
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                _ => rsx! {
                    div { class: "flex justify-center py-8",
                        span { class: "loading loading-spinner loading-md", role: "status", aria_label: tid!("loading") }
                    }
                },
            }
        }
    }
}

#[component]
fn RuleCard(
    rule: DecryptedRule,
    currency: String,
    payers: String,
    estimates: usize,
    drift: Option<f64>,
    on_open: EventHandler<()>,
) -> Element {
    let p = &rule.payload;
    let t = &p.template;
    let st = state(p);
    let next = next_date(p);
    let amount = if t.variable { format!("≈ {:.2} {currency}", t.amount) } else { format!("{:.2} {currency}", t.amount) };
    rsx! {
        li {
            button {
                id: "recurring-rule-{rule.id}",
                r#type: "button",
                class: if st == RuleState::Active { "w-full text-left flex items-start gap-3 p-3 rounded-box shadow-soft bg-base-100 hover:bg-base-200" } else { "w-full text-left flex items-start gap-3 p-3 rounded-box shadow-soft bg-base-100 hover:bg-base-200 opacity-70" },
                onclick: move |_| on_open.call(()),
                span { class: "w-9 h-9 shrink-0 flex items-center justify-center text-xl", aria_hidden: "true", "{emoji_of(&rule)}" }
                div { class: "flex-1 min-w-0 flex flex-col gap-0.5",
                    div { class: "flex justify-between gap-2 min-w-0",
                        span { class: "font-semibold truncate", "{t.name}" }
                        span { class: "shrink-0 font-semibold text-sm tabular-nums", "{amount}" }
                    }
                    span { class: "text-xs text-base-content/70 truncate",
                        {schedule_label(&p.rule)}
                        " · "
                        {tid!("recurring-paid-by", name: payers.clone())}
                    }
                    div { class: "flex flex-wrap items-center gap-1.5 text-xs pt-0.5",
                        match st {
                            RuleState::Paused => rsx! { span { class: "badge badge-sm badge-ghost", {tid!("recurring-paused")} } },
                            RuleState::Finished => rsx! { span { class: "badge badge-sm badge-ghost", {tid!("recurring-finished")} } },
                            RuleState::Active => rsx! {
                                if let Some(date) = next {
                                    span { class: "text-base-content/70", {tid!("recurring-next-on", date: format_date(date))} }
                                }
                            },
                        }
                        if let Some((done, total)) = progress(p) {
                            span { class: "badge badge-sm badge-ghost tabular-nums", {tid!("recurring-progress", done: done as i64, total: total as i64)} }
                        }
                        if estimates > 0 {
                            span { class: "badge badge-sm badge-warning badge-soft", {tid!("recurring-to-confirm", count: estimates as i64)} }
                        }
                        if let Some(drift) = drift {
                            span { class: "badge badge-sm badge-warning badge-soft", {tid!("recurring-rate-badge", percent: format!("{:.1}", drift.abs() * 100.0))} }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn RuleSheet(
    rule: DecryptedRule,
    data: std::sync::Arc<ProjectData>,
    added: usize,
    drift: Option<(f64, f64)>,
    on_close: EventHandler<()>,
    on_edit: EventHandler<()>,
    on_pause: EventHandler<()>,
    on_stop: EventHandler<()>,
    on_use_rate: EventHandler<f64>,
    on_keep_rate: EventHandler<f64>,
) -> Element {
    let p = &rule.payload;
    let t = &p.template;
    let currency = data.currency.clone();
    let name_of = |id: i32| data.user_names.get(&id).cloned().unwrap_or_else(|| "?".to_string());
    let payers = side_amounts(&t.payers, t.amount, p.next);
    let debtors = side_amounts(&t.debtors, t.amount, p.next);
    let next_dates = upcoming(&p.rule, p.next, 3);
    let finished = state(p) == RuleState::Finished;
    let author = rule.author_id.map(name_of);
    let created = format_date(rule.created_at.date());
    rsx! {
        div { class: "modal modal-open modal-bottom sm:modal-middle", role: "dialog", aria_labelledby: "rule-sheet-title",
            div { class: "modal-box max-w-md p-0 flex flex-col",
                div { class: "flex items-center gap-2 px-5 pt-4 pb-1",
                    h3 { id: "rule-sheet-title", class: "flex-1 min-w-0 truncate font-bold text-lg font-display", "{t.name}" }
                    button {
                        r#type: "button",
                        class: "btn btn-ghost btn-circle h-11 w-11 min-h-11",
                        aria_label: tid!("close"),
                        onclick: move |_| on_close.call(()),
                        CloseIcon { size: ICON_HEADER }
                    }
                }
                div { class: "flex-1 overflow-y-auto flex flex-col gap-3 px-5 pb-4",
                    div { class: "text-center",
                        p { class: "font-display font-extrabold text-3xl tabular-nums",
                            if t.variable { "≈ " }
                            "{t.amount:.2} {currency}"
                        }
                        if let Some((amount, from, _)) = t.conversion() {
                            p { class: "text-xs text-base-content/70 tabular-nums", "{amount:.2} {from}" }
                        }
                        p { class: "text-xs text-base-content/70", {schedule_label(&p.rule)} " · " {ends_label(&p.rule)} }
                    }
                    if let (Some((drift, rate)), Some((source_amount, from, old_rate))) = (drift, t.conversion()) {
                        div { id: "recurring-drift", role: "alert", class: "alert alert-warning alert-soft text-sm items-start",
                            div { class: "flex flex-col gap-2",
                                span {
                                    {tid!(
                                        "recurring-drift",
                                        currency: from.to_string(),
                                        percent: format!("{:.1}", drift.abs() * 100.0),
                                        amount: format!("{:.2}", t.amount),
                                        project_currency: currency.clone(),
                                        rate: format!("{old_rate}"),
                                        today_amount: format!("{:.2}", shared::convert_to_project(source_amount, rate))
                                    )}
                                }
                                div { class: "flex flex-wrap gap-2",
                                    button { id: "recurring-use-rate", r#type: "button", class: "btn btn-sm btn-secondary", onclick: move |_| on_use_rate.call(rate), {tid!("recurring-use-rate")} }
                                    button { id: "recurring-keep-rate", r#type: "button", class: "btn btn-sm", onclick: move |_| on_keep_rate.call(rate), {tid!("recurring-keep", amount: format!("{:.2}", t.amount), currency: currency.clone())} }
                                }
                            }
                        }
                    }
                    if !finished && !next_dates.is_empty() {
                        div { class: "flex flex-col gap-1.5",
                            span { class: "text-xs text-base-content/70", {tid!("recurring-next-ones")} }
                            div { class: "flex flex-wrap gap-1.5",
                                for date in next_dates {
                                    span { class: "badge badge-ghost tabular-nums", {format_date(date)} }
                                }
                            }
                        }
                    }
                    div { class: "flex flex-col gap-2",
                        span { class: "text-xs text-base-content/70", {tid!("payers-title-paid-by")} }
                        for (id , amount) in payers {
                            div { class: "flex justify-between gap-3 text-sm",
                                span { class: "truncate", "{name_of(id)}" }
                                span { class: "font-semibold tabular-nums", "{amount:.2}" }
                            }
                        }
                        span { class: "text-xs text-base-content/70 pt-1", {tid!("debtors-title-debtors")} }
                        for (id , amount) in debtors {
                            div { class: "flex justify-between gap-3 text-sm",
                                span { class: "truncate", "{name_of(id)}" }
                                span { class: "font-semibold tabular-nums", "{amount:.2}" }
                            }
                        }
                    }
                    div { class: "flex justify-between text-sm",
                        span { class: "text-base-content/70", {tid!("recurring-added-so-far")} }
                        span { class: "tabular-nums", "{added}" }
                    }
                    if let Some(author) = author {
                        div { class: "flex justify-between gap-3 text-sm",
                            span { class: "text-base-content/70", {tid!("recurring-set-up-by")} }
                            span { class: "truncate", "{author} · {created}" }
                        }
                    }
                }
                div { class: "flex flex-col gap-2 px-5 py-3 border-t border-base-200",
                    button { id: "recurring-edit", r#type: "button", class: "btn btn-primary w-full", onclick: move |_| on_edit.call(()), {tid!("edit")} }
                    div { class: "flex gap-2",
                        if !finished {
                            button {
                                id: "recurring-pause",
                                r#type: "button",
                                class: "btn flex-1",
                                onclick: move |_| on_pause.call(()),
                                if p.paused { {tid!("recurring-resume")} } else { {tid!("recurring-pause")} }
                            }
                        }
                        button { id: "recurring-stop", r#type: "button", class: "btn flex-1 text-error", onclick: move |_| on_stop.call(()), {tid!("recurring-stop")} }
                    }
                }
            }
            div { class: "modal-backdrop", onclick: move |_| on_close.call(()) }
        }
    }
}

/// The line at the top of the expense list, once the project has a rule.
#[component]
pub fn RecurringStrip(project_id: Uuid) -> Element {
    let store = use_context::<ProjectStore>();
    let nav = use_navigator();
    let Some(r) = store.recurring_for(project_id) else { return rsx! {} };
    if r.rules.is_empty() {
        return rsx! {};
    }
    let rules = sorted(&r.rules);
    let estimates = store
        .data_for(project_id)
        .map(|d| d.expenses.iter().filter(|e| e.expense.estimate).count())
        .unwrap_or(0);
    let next = rules
        .iter()
        .find(|r| state(&r.payload) == RuleState::Active)
        .and_then(|r| next_date(&r.payload).map(|d| (r.payload.template.name.clone(), d)));
    rsx! {
        button {
            id: "recurring-strip",
            r#type: "button",
            class: "flex items-center gap-3 p-3 rounded-box shadow-soft bg-base-100 hover:bg-base-200 text-left w-full",
            onclick: move |_| { nav.push(Route::RecurringPage { project_id }); },
            span { class: "w-8 h-8 shrink-0 rounded-full bg-primary/15 text-primary flex items-center justify-center",
                RepeatIcon { size: ICON_INLINE }
            }
            div { class: "flex-1 min-w-0",
                p { class: "text-sm font-semibold", {tid!("recurring-strip", count: rules.len() as i64)} }
                p { class: "text-xs text-base-content/70 truncate",
                    if let Some((name, date)) = next {
                        {tid!("recurring-next", name: name, date: format_date(date))}
                    }
                    if estimates > 0 {
                        " · "
                        {tid!("recurring-to-confirm", count: estimates as i64)}
                    }
                }
            }
        }
    }
}
