//! The account's personal project on the projects page (designs J1, J2, J3): the card offering to
//! create it, the sheet that does, and the card it becomes once it exists.

use api::account_projects::account_projects_controller::upsert_account_project;
use dioxus::fullstack::Json;
use dioxus::prelude::*;
use shared::{Account, CreatableProject, ProjectDto, ProjectPayload, UpsertAccountProject};
use uuid::Uuid;

use crate::entity_ids::{add_participants, new_participant};
use crate::common::{
    add_project_fresh, error_message, key_of, month_title, update_ls, upsert_project,
    CurrencyPicker, LocalStorageState, SheetBar, SheetFooter, SHEET_TITLE,
};
use crate::crypto::{
    claim_token, claim_verifier, decrypt_json, encrypt_padded_json, generate_key, wrap_key, WrapUse,
};
use crate::common::fx_cache::read_fx;
use crate::expenses::budget_panel::money;
use crate::expenses::helpers::my_spending::{month_glance, GroupLedger};
use crate::preferences::uncounted_projects;
use crate::icons::{ChartIcon, LockIcon, UserIcon, WalletIcon};
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
    unsent: Signal<Option<Uuid>>,
) -> Result<Uuid, String> {
    let mut payload = ProjectPayload { name, currency, description: None, status: Some(shared::ProjectStatus::Ongoing), extra: Default::default() };
    payload.extra.insert("kind".into(), "personal".into());
    let demo = crate::common::persist::is_demo();
    let seal = |id, key: &[u8; 32]| {
        Ok(CreatableProject {
            id: Some(id),
            payload: encrypt_padded_json(key, &payload)?,
            claim_verifier: Some(claim_verifier(key).to_vec()),
            demo,
            creator_verifier: None,
        })
    };
    let made = add_project_fresh(ls_ctx, Some(unsent), &generate_key(), None, seal).await.map_err(|e| e.message())?;
    let (project, key) = (made.project, made.key);

    let users = add_participants(&key, vec![new_participant(&key, project.id, me)?])
        .await
        .map_err(|e| error_message(&e))?;
    let user_id = users.first().map(|u| u.id);
    update_ls(ls_ctx, |state| {
        upsert_project(state, project.id, user_id);
        state.personal_project = Some(project.id);
    });
    let account = auth.peek().clone();
    if let (Some(account), Some(ak)) = (account, account_key) {
        if let Ok(written) = crate::preferences::remember_personal_project(account, ak, project.id).await {
            auth.set(Some(written));
        }
    }

    // Escrowed, so the account's other devices open it without a link. A failure here is retried by
    // the projects page's reconciliation, as for any project.
    let escrowed = upsert_account_project(Json(UpsertAccountProject {
        project_id: project.id,
        user_id,
        key: account_key.and_then(|ak| wrap_key(&ak, WrapUse::Escrow(project.id), &key)),
        claim_label: None,
        claim_token: Some(claim_token(&key).to_vec()),
        amk_epoch: crate::auth::account_key::held_amk_epoch(),
    }))
    .await;
    crate::auth::account_key::note(&escrowed);
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
    // The creation a press left unknown, which the next press settles before anything else.
    let unsent: Signal<Option<Uuid>> = use_signal(|| None);

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
            match create_personal_project(name_val, currency_val, me, account_key, ls_ctx, auth_ctx, unsent).await {
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
            form { class: "modal-box max-w-md p-0 flex flex-col", onsubmit: on_submit,
                SheetBar { close_id: "personal-create-close", task: true, on_close,
                    h2 { id: "personal-create-title", class: SHEET_TITLE, {tid!("personal-sheet-title")} }
                }
                div { class: "flex-1 overflow-y-auto flex flex-col gap-4 px-5 pt-1 pb-4",
                p { class: "text-sm text-base-content/70", {tid!("personal-sheet-intro")} }
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
                }
                SheetFooter {
                    on_cancel: on_close,
                    button {
                        id: "personal-create-submit",
                        r#type: "submit",
                        class: "btn btn-primary",
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
/// up. No budget bar: the ceilings are per category, and the budget panel shows each one.
#[component]
pub fn PersonalCard(project: ProjectDto, ledgers: Signal<Vec<GroupLedger>>) -> Element {
    let ls_ctx = use_context::<Signal<LocalStorageState>>();
    let auth_ctx = use_context::<Signal<Option<Account>>>();
    let account_key_ctx = use_context::<Signal<Option<[u8; 32]>>>();
    let project_id = project.id;
    let today = crate::common::local_today();
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
    let month = month_title(today.month0() as usize);
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
                        span { id: "personal-card-month", class: "text-xs text-base-content/70", "{month}" }
                    }
                    if let Some(g) = glance() {
                        span { id: "personal-card-spent", class: "font-bold tabular-nums", "{money(g.spent, &currency)}" }
                    }
                }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::test_fixtures::{make_project, test_key};
    use crate::crypto::{encrypt_json, key_to_fragment};
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
