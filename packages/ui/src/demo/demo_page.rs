use dioxus::prelude::*;

use super::dataset::dataset;
use super::seed::seed_demo;
use super::{leave_demo, use_demo_mode};
use crate::common::persist::is_demo;
use crate::common::{read_from_ls, LocalStorageState};
use crate::i18n::{apply_inferred_language, current_lang};
use crate::route::Route;
use crate::tid;

/// Opens the tab's demo project, seeding one first if the tab has none. Outside the sandbox —
/// native, or a stale history entry — it goes to the real project list instead.
fn use_open_demo(lang: String) -> (Signal<Option<String>>, Callback<()>) {
    let nav = use_navigator();
    let ls_ctx = use_context::<Signal<LocalStorageState>>();
    let mut error = use_signal(|| None::<String>);
    let mut attempt = use_signal(|| 0u32);

    // In a task, so the language read below does not subscribe the effect: switching to the
    // requested language would otherwise seed a second project.
    use_effect(move || {
        let _ = attempt();
        let lang = lang.clone();
        spawn(async move {
            if !is_demo() {
                nav.replace(Route::ProjectsPage {});
                return;
            }
            if let Some(existing) = read_from_ls().projects.first() {
                nav.replace(Route::ExpensesPage { project_id: existing.project_id });
                return;
            }
            let seed_lang = match lang.as_str() {
                "fr" | "en" => {
                    apply_inferred_language(&lang);
                    lang
                }
                _ => current_lang(),
            };
            match seed_demo(ls_ctx, dataset(&seed_lang)).await {
                Ok(project_id) => {
                    nav.replace(Route::ExpensesPage { project_id });
                }
                Err(e) => error.set(Some(e)),
            }
        });
    });

    let retry = use_callback(move |_| {
        error.set(None);
        attempt.with_mut(|a| *a += 1);
    });
    (error, retry)
}

#[component]
pub fn DemoPage(lang: String) -> Element {
    let (error, retry) = use_open_demo(lang);
    rsx! {
        div { class: "container app-container bg-base-100 p-4 max-w-md w-full mx-auto flex flex-col items-center justify-center gap-4 text-center",
            if let Some(msg) = error() {
                div { role: "alert", class: "alert alert-error text-sm", "{msg}" }
                button { id: "demo-retry", class: "btn btn-primary btn-sm", onclick: move |_| retry.call(()),
                    {tid!("retry")}
                }
            } else {
                span { class: "loading loading-spinner loading-md", role: "status", aria_label: tid!("demo-preparing") }
                p { class: "text-sm text-base-content/70", {tid!("demo-preparing")} }
            }
        }
    }
}

/// On every page of the sandbox, so leaving is always one tap and never a surprise.
#[component]
pub fn DemoBar() -> Element {
    if !use_demo_mode() {
        return rsx! {};
    }
    rsx! {
        div { id: "demo-bar", role: "status",
            class: "h-10 max-w-md mx-auto px-4 flex items-center justify-between gap-2 text-sm",
            span { class: "badge badge-info badge-soft shrink-0", {tid!("demo-badge")} }
            span { class: "truncate text-base-content/70", {tid!("demo-bar")} }
            button { id: "demo-leave", r#type: "button", class: "btn btn-primary btn-xs shrink-0",
                onclick: move |_| leave_demo(),
                {tid!("demo-leave")}
            }
        }
    }
}

/// What the sign-in pages show inside the sandbox: an account there would adopt a throwaway store.
#[component]
pub fn DemoSignInBlocked() -> Element {
    rsx! {
        div { class: "container app-container bg-base-100 p-4 max-w-md w-full mx-auto flex flex-col items-center justify-center gap-4 text-center",
            p { {tid!("demo-sign-in-blocked")} }
            button { id: "demo-leave-sign-in", r#type: "button", class: "btn btn-primary",
                onclick: move |_| leave_demo(),
                {tid!("demo-leave")}
            }
        }
    }
}
