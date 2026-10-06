use dioxus::prelude::*;
use crate::tid;
use shared::ProjectStatus;

use crate::common::{haptic, AppHeader, Haptic};
use crate::expenses::helpers::expenses_page_helpers::HeaderState;
use crate::expenses::project_actions_sheet::ProjectActionsSheet;
use crate::expenses::project_states::Spinner;
use crate::icons::MoreIcon;
use crate::route::Route;

#[derive(PartialEq, Props, Clone)]
pub struct ProjectHeaderProps {
    pub state: HeaderState,
    pub on_history: EventHandler<()>,
    pub on_recurring: EventHandler<()>,
    pub on_edit: EventHandler<()>,
    pub on_leave: EventHandler<()>,
    pub on_status: EventHandler<ProjectStatus>,
    pub on_export_json: EventHandler<()>,
    pub on_export_csv: EventHandler<()>,
    /// Opens the friend picker. `None` for an anonymous device: there is no friends list to pick from.
    pub on_invite: Option<EventHandler<()>>,
    /// The share link: platform share sheet where there is one, clipboard otherwise. The caller
    /// decides at click time, so the button renders identically on SSR and in the browser.
    pub on_share: Option<EventHandler<()>>,
    /// Drops the dangling project from this device — the only exit from a deleted project's link.
    pub on_forget: EventHandler<()>,
    /// `counted://` link, set only on a phone browser: the one handoff where universal links are
    /// unavailable (AltStore builds, unverified installs).
    pub native_app_link: Option<String>,
}

/// The project page's header and its actions sheet. Decides nothing — every entry calls back out.
#[component]
pub fn ProjectHeader(props: ProjectHeaderProps) -> Element {
    let mut sheet_open = use_signal(|| false);
    match props.state.clone() {
        HeaderState::Loading => rsx! {
            Spinner {}
        },
        // With the bar, not just the alert: an error here is often terminal for the page, and
        // without the back button the only way off it is the bottom dock.
        HeaderState::Error(key) => rsx! {
            AppHeader { back_button_route: Route::ProjectsPage {}, title: "Counted", sticky: true }
            div { role: "alert", class: "alert alert-error", {tid!(key)} }
        },
        HeaderState::Gone => rsx! {
            AppHeader { back_button_route: Route::ProjectsPage {}, title: "Counted", sticky: true }
            div { class: "flex flex-col items-center gap-3 py-8 text-center",
                p { class: "font-semibold", {tid!("project-gone-title")} }
                p { class: "text-sm text-base-content/70",
                    {tid!("project-gone-hint")}
                }
                button {
                    class: "btn btn-primary btn-sm mt-2",
                    onclick: move |_| props.on_forget.call(()),
                    {tid!("project-forget")}
                }
            }
        },
        // No actions menu: every entry below needs the live project row, which is what is missing.
        HeaderState::Cached { title } => rsx! {
            AppHeader { back_button_route: Route::ProjectsPage {}, title, sticky: true }
        },
        HeaderState::Ready { title, status, read_only } => rsx! {
            AppHeader { back_button_route: Route::ProjectsPage {}, title: title.clone(), sticky: true,
                button {
                    id: "project-actions",
                    r#type: "button",
                    class: "btn btn-circle size-11 bg-base-200 border-0",
                    aria_label: tid!("project-actions"),
                    aria_haspopup: "dialog",
                    aria_expanded: sheet_open(),
                    onclick: move |_| {
                        haptic(Haptic::Light);
                        sheet_open.set(true);
                    },
                    MoreIcon { size: 24 }
                }
            }
            // A sibling of the header, never inside it: a scrolled mobile header gets
            // `backdrop-filter`, which would trap this `position: fixed` sheet in the header box.
            if sheet_open() {
                ProjectActionsSheet {
                    title,
                    status,
                    read_only,
                    native_app_link: props.native_app_link.clone(),
                    on_close: move |_| sheet_open.set(false),
                    on_share: props.on_share,
                    on_invite: props.on_invite,
                    on_history: props.on_history,
                    on_recurring: props.on_recurring,
                    on_edit: props.on_edit,
                    on_status: props.on_status,
                    on_export_csv: props.on_export_csv,
                    on_export_json: props.on_export_json,
                    on_leave: props.on_leave,
                }
            }
        },
    }
}
