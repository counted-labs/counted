//! The account's personal project on the projects page (designs J1, J2, J3): the card offering to
//! create it, the sheet that does, and the card it becomes once it exists.

use api::account_projects::account_projects_controller::upsert_account_project;
use api::projects::projects_controller::add_project;
use api::users::users_controller::add_user;
use dioxus::fullstack::Json;
use dioxus::prelude::*;
use shared::{
    Account, CreatableProject, CreatableUser, CreatableUserBatch, ProjectDto, ProjectPayload,
    UpsertAccountProject, UserPayload,
};
use uuid::Uuid;

use crate::common::{
    error_message, key_of, month_title, update_ls, upsert_project, upsert_project_key,
    CurrencyPicker, LocalStorageState,
};
use crate::crypto::{
    claim_token, claim_verifier, decrypt_json, encrypt_padded_json, generate_key, key_to_fragment, wrap_key, WrapUse,
};
use crate::common::fx_cache::read_fx;
use crate::expenses::budget_panel::money;
use crate::expenses::helpers::budgets::Meter;
use crate::expenses::helpers::my_spending::{month_glance, GroupLedger};
use crate::preferences::uncounted_projects;
use crate::icons::{ChartIcon, CloseIcon, LockIcon, UserIcon, WalletIcon, ICON_HEADER};
use crate::participants::use_my_display_name;
use crate::route::Route;
use crate::tid;

/// Whether `p` is the account's personal project, whatever its status. One per account: an
/// archived one still counts, so the offer to create another never comes back. The account says
/// which, never the payload (audit 2026-10-19 M3).
pub fn is_personal_dto(p: &ProjectDto, state: &LocalStorageState) -> bool {
    state.personal_project == Some(p.id)
}

/// The project, its one participant and the escrowed key, in the order the shared-project modal
/// writes them - the key is stored the moment the project exists.
async fn create_personal_project(
    name: String,
    currency: String,
    me: String,
    account_key: Option<[u8; 32]>,
    ls_ctx: Signal<LocalStorageState>,
    mut auth: Signal<Option<Account>>,
) -> Result<Uuid, String> {
    let key = generate_key();
    let mut payload = ProjectPayload { name, currency, description: None, status: Some(shared::ProjectStatus::Ongoing), extra: Default::default() };
    payload.extra.insert("kind".into(), "personal".into());
    let project = add_project(Json(CreatableProject {
        payload: encrypt_padded_json(&key, &payload)?,
        claim_verifier: Some(claim_verifier(&key).to_vec()),
        demo: crate::common::persist::is_demo(),
    }))
    .await
    .map_err(|e| error_message(&e))?;
    update_ls(ls_ctx, |state| {
        upsert_project(state, project.id, None);
        upsert_project_key(state, project.id, key_to_fragment(&key));
    });

    let me_payload = encrypt_padded_json(&key, &UserPayload { name: me, removed: false })?;
    let users = add_user(Json(CreatableUserBatch::Multiple(vec![CreatableUser {
        payload: me_payload,
        project_id: project.id,
    }])))
    .await
    .map_err(|e| error_message(&e))?;
    let user_id = users.first().map(|u| u.id);
    update_ls(ls_ctx, |state| {
        upsert_project(state, project.id, user_id);
        state.personal_project = Some(project.id);
    });
    let account = auth.peek().clone();
    let typed = account.as_ref().is_some_and(|a| a.typed_wraps);
    if let (Some(account), Some(ak)) = (account, account_key) {
        if let Ok(written) = crate::preferences::remember_personal_project(account, ak, project.id).await {
            auth.set(Some(written));
        }
    }

    // Escrowed, so the account's other devices open it without a link. A failure here is retried by
    // the projects page's reconciliation, as for any project.
    let _ = upsert_account_project(Json(UpsertAccountProject {
        project_id: project.id,
        user_id,
        key: account_key.and_then(|ak| wrap_key(&ak, WrapUse::Escrow(project.id), &key, typed)),
        claim_label: None,
        claim_token: Some(claim_token(&key).to_vec()),
    }))
    .await;
    Ok(project.id)
}

/// J1: the last card of the list until the project exists. Signed out, it asks for an account
/// first - the personal project is for accounts only.
#[component]
pub fn PersonalCreateCard(signed_in: bool, on_create: EventHandler<()>) -> Element {
    rsx! {
        div { id: "personal-create-card", class: "flex items-center gap-3 rounded-box border-[1.5px] border-dashed border-primary/40 px-4 py-3.5",
            span { class: "size-10 shrink-0 rounded-xl bg-primary/10 text-primary flex items-center justify-center",
                WalletIcon {}
            }
            span { class: "flex-1 min-w-0 flex flex-col",
                span { class: "font-semibold", {tid!("personal-create-title")} }
                span { class: "text-xs text-base-content/70", {tid!("personal-create-hint")} }
            }
            if signed_in {
                button {
                    id: "personal-create-btn",
                    r#type: "button",
                    class: "btn btn-sm btn-primary",
                    onclick: move |_| on_create.call(()),
                    {tid!("personal-create-action")}
                }
            } else {
                Link { id: "personal-create-signup", class: "btn btn-sm btn-primary", to: Route::RegisterPage {},
                    {tid!("personal-create-signup")}
                }
            }
        }
    }
}

/// J2: name and currency, nothing else. The one participant is the account's display name.
#[component]
pub fn PersonalCreateSheet(on_close: EventHandler<()>) -> Element {
    crate::common::use_modal_focus("personal-create-sheet");
    let nav = use_navigator();
    let ls_ctx = use_context::<Signal<LocalStorageState>>();
    let account_key_ctx = use_context::<Signal<Option<[u8; 32]>>>();
    let auth_ctx = use_context::<Signal<Option<Account>>>();
    let is_online = use_context::<Signal<bool>>();
    let display_name = use_my_display_name();
    let mut name = use_signal(|| tid!("personal-default-name"));
    let mut currency = use_signal(|| "EUR".to_string());
    let mut error_msg: Signal<Option<String>> = use_signal(|| None);
    let mut loading = use_signal(|| false);

    let on_submit = move |e: FormEvent| {
        e.prevent_default();
        if !is_online() {
            error_msg.set(Some(tid!("add-project-offline")));
            return;
        }
        let name_val = name().trim().to_string();
        if name_val.is_empty() {
            error_msg.set(Some(tid!("add-project-name-required")));
            return;
        }
        let me = display_name.clone().unwrap_or_else(|| tid!("participants-you-badge"));
        let account_key = account_key_ctx();
        let currency_val = currency();
        loading.set(true);
        error_msg.set(None);
        spawn(async move {
            match create_personal_project(name_val, currency_val, me, account_key, ls_ctx, auth_ctx).await {
                Ok(project_id) => {
                    on_close.call(());
                    nav.push(Route::ExpensesPage { project_id });
                }
                Err(e) => {
                    error_msg.set(Some(e));
                    loading.set(false);
                }
            }
        });
    };

    rsx! {
        div {
            id: "personal-create-sheet",
            class: "modal modal-open modal-bottom sm:modal-middle",
            role: "dialog",
            aria_modal: "true",
            aria_labelledby: "personal-create-title",
            tabindex: "-1",
            form { class: "modal-box max-w-md flex flex-col gap-4", onsubmit: on_submit,
                div { class: "flex items-start gap-3",
                    div { class: "flex-1 flex flex-col gap-1",
                        h2 { id: "personal-create-title", class: "font-display font-bold text-xl", {tid!("personal-sheet-title")} }
                        p { class: "text-sm text-base-content/70", {tid!("personal-sheet-intro")} }
                    }
                    button {
                        r#type: "button",
                        class: "btn btn-circle size-11 min-h-11 border-0 bg-base-200 text-base-content/80",
                        aria_label: tid!("close"),
                        onclick: move |_| on_close.call(()),
                        CloseIcon { size: ICON_HEADER }
                    }
                }
                if let Some(err) = error_msg() {
                    div { role: "alert", class: "alert alert-error text-sm", "{err}" }
                }
                div { class: "flex flex-col gap-1.5",
                    label { class: "text-xs font-semibold text-base-content/70", r#for: "personal-name", {tid!("add-project-name-label")} }
                    input {
                        id: "personal-name",
                        class: "input w-full",
                        r#type: "text",
                        aria_required: "true",
                        value: "{name}",
                        oninput: move |e| name.set(e.value()),
                    }
                }
                div { class: "flex flex-col gap-1.5",
                    span { class: "text-xs font-semibold text-base-content/70", {tid!("project-currency")} }
                    CurrencyPicker {
                        id: "personal-currency",
                        value: currency(),
                        label: tid!("project-currency"),
                        button_class: "input w-full gap-1 font-medium cursor-pointer",
                        onchange: move |code| currency.set(code),
                    }
                }
                ul { class: "rounded-box bg-success/10 px-4 py-3.5 flex flex-col gap-2.5 text-sm",
                    li { class: "flex gap-2.5 items-start",
                        span { class: "text-success shrink-0 mt-0.5", UserIcon { size: 18 } }
                        span { {tid!("personal-sheet-point-alone")} }
                    }
                    li { class: "flex gap-2.5 items-start",
                        span { class: "text-success shrink-0 mt-0.5", LockIcon { size: 18 } }
                        span { {tid!("personal-sheet-point-encrypted")} }
                    }
                    li { class: "flex gap-2.5 items-start",
                        span { class: "text-success shrink-0 mt-0.5", ChartIcon { size: 18 } }
                        span { {tid!("personal-sheet-point-stats")} }
                    }
                }
                div { class: "flex gap-2.5",
                    button {
                        r#type: "button",
                        class: "btn flex-1 rounded-full",
                        onclick: move |_| on_close.call(()),
                        {tid!("cancel")}
                    }
                    button {
                        id: "personal-create-submit",
                        r#type: "submit",
                        class: "btn btn-primary flex-[2] rounded-full",
                        disabled: loading(),
                        if loading() {
                            span { class: "loading loading-spinner loading-sm" }
                        }
                        {tid!("personal-create-action")}
                    }
                }
            }
            div { class: "modal-backdrop", onclick: move |_| on_close.call(()) }
        }
    }
}

/// J3, P4: the personal project, pinned above the groups, with the month as Mes dépenses adds it
/// up and, once ceilings are set, the budget left.
#[component]
pub fn PersonalCard(project: ProjectDto, ledgers: Signal<Vec<GroupLedger>>) -> Element {
    let ls_ctx = use_context::<Signal<LocalStorageState>>();
    let auth_ctx = use_context::<Signal<Option<Account>>>();
    let account_key_ctx = use_context::<Signal<Option<[u8; 32]>>>();
    let project_id = project.id;
    let today = chrono::Utc::now().date_naive();
    let name = ls_ctx
        .read()
        .projects
        .iter()
        .find(|p| p.project_id == project_id)
        .and_then(key_of)
        .and_then(|k| decrypt_json::<ProjectPayload>(&k, &project.payload).ok())
        .map(|p| (p.name, p.currency));
    let glance = use_memo(move || {
        let uncounted = match (auth_ctx.read().as_ref(), account_key_ctx()) {
            (Some(a), Some(k)) => uncounted_projects(a, &k),
            _ => Default::default(),
        };
        month_glance(&ls_ctx.read(), project_id, &ledgers.read(), &uncounted, read_fx().as_ref(), today)
    });
    let (name, currency) = name.unwrap_or_default();
    let month = month_title(chrono::Datelike::month0(&today) as usize);
    let budget = glance().and_then(|g| g.budget);
    rsx! {
        Link {
            id: "personal-card",
            class: "card-clickable flex flex-col gap-2.5 p-4 rounded-[20px] bg-base-100 shadow-soft border border-primary/25",
            to: Route::ExpensesPage { project_id },
                div { class: "flex items-center gap-3",
                    span { class: "size-10 shrink-0 rounded-xl bg-primary/10 text-primary flex items-center justify-center",
                        WalletIcon {}
                    }
                    span { class: "flex-1 min-w-0 flex flex-col",
                        span { class: "flex items-center gap-2 min-w-0",
                            span { class: "font-semibold truncate", "{name}" }
                            span { class: "badge badge-sm badge-soft badge-success shrink-0 font-semibold", {tid!("personal-badge")} }
                        }
                        span { id: "personal-card-month", class: "text-xs text-base-content/70",
                            match &budget {
                                Some(b) => tid!("personal-card-budget", month: month.clone(), amount: crate::common::round_amount(b.limit, &currency)),
                                None => month.clone(),
                            }
                        }
                    }
                    if let Some(g) = glance() {
                        span { class: "flex flex-col items-end",
                            span { id: "personal-card-spent", class: "font-bold tabular-nums", "{money(g.spent, &currency)}" }
                            if let Some(b) = &budget {
                                span { id: "personal-card-left", class: "text-[11px] font-semibold tabular-nums",
                                    match b.meter() {
                                        Meter::Left(left) => rsx! { span { class: "text-success", {tid!("budget-left", amount: money(left, &currency))} } },
                                        Meter::Near(_) => rsx! { span { class: "text-warning", {tid!("budget-left", amount: money(b.limit - b.spent, &currency))} } },
                                        Meter::Over(by) => rsx! { span { class: "text-error", {tid!("budget-over", amount: money(by, &currency))} } },
                                    }
                                }
                            }
                        }
                    }
                }
                if let Some(b) = &budget {
                    span { class: "block h-1.5 rounded-full bg-base-300 overflow-hidden",
                        span {
                            class: match b.meter() {
                                Meter::Left(_) => "block h-full rounded-full bg-success",
                                Meter::Near(_) => "block h-full rounded-full bg-warning",
                                Meter::Over(_) => "block h-full rounded-full bg-error",
                            },
                            style: "width: {(b.ratio() * 100.0).clamp(0.0, 100.0)}%;",
                        }
                    }
                }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::test_fixtures::{make_project, test_key};
    use crate::crypto::encrypt_json;
    use crate::common::LocalStorageProject;

    fn held(p: &ProjectDto, k: &[u8; 32], expenses: Option<Vec<shared::Expense>>) -> LocalStorageProject {
        LocalStorageProject {
            project_id: p.id,
            encryption_key: Some(key_to_fragment(k)),
            cached_expenses: expenses,
            ..Default::default()
        }
    }

    #[test]
    fn a_personal_project_is_recognised_whatever_its_status() {
        let k = test_key();
        let mut mine = make_project(&k, Uuid::new_v4(), "Mes dépenses", "EUR");
        let mut payload: ProjectPayload = decrypt_json(&k, &mine.payload).unwrap();
        payload.extra.insert("kind".into(), "personal".into());
        payload.status = Some(shared::ProjectStatus::Archived);
        mine.payload = encrypt_json(&k, &payload).unwrap();
        let trip = make_project(&k, Uuid::new_v4(), "Corsica", "EUR");
        let state = LocalStorageState {
            projects: vec![held(&mine, &k, None), held(&trip, &k, None)],
            personal_project: Some(mine.id),
            ..Default::default()
        };
        assert!(is_personal_dto(&mine, &state));
        assert!(!is_personal_dto(&trip, &state));
        assert!(!is_personal_dto(&mine, &LocalStorageState::default()), "no key, no way to tell");
    }
}
