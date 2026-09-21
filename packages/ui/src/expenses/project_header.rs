use dioxus::prelude::*;
use crate::tid;
use shared::ProjectStatus;

use crate::common::{AppHeader, DropdownButton, DropdownItem, ProjectStatusItems};
use crate::expenses::helpers::expenses_page_helpers::HeaderState;
use crate::expenses::project_states::Spinner;
use crate::icons::{HistoryIcon, ShareIcon, ICON_HEADER};
use crate::route::Route;

#[derive(PartialEq, Props, Clone)]
pub struct ProjectHeaderProps {
    pub state: HeaderState,
    pub on_history: EventHandler<()>,
    pub on_edit: EventHandler<()>,
    pub on_leave: EventHandler<()>,
    pub on_status: EventHandler<ProjectStatus>,
    pub on_export_json: EventHandler<()>,
    pub on_export_csv: EventHandler<()>,
    /// Opens the friend picker. `None` for an anonymous device: there is no friends list to pick from.
    pub on_invite: Option<EventHandler<()>>,
    /// The share link: platform share sheet where there is one, clipboard otherwise. The caller
    /// decides at click time, so the button renders identically on SSR and in the browser.
    pub on_share: EventHandler<()>,
    /// Drops the dangling project from this device — the only exit from a deleted project's link.
    pub on_forget: EventHandler<()>,
    /// `counted://` link, set only on a phone browser: the one handoff where universal links are
    /// unavailable (AltStore builds, unverified installs).
    pub native_app_link: Option<String>,
}

/// The project page's header and its actions menu. Decides nothing — every entry calls back out.
#[component]
pub fn ProjectHeader(props: ProjectHeaderProps) -> Element {
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
        HeaderState::Ready { title, status } => rsx! {
            AppHeader { back_button_route: Route::ProjectsPage {}, title, sticky: true,
                button {
                    id: "project-share-btn",
                    class: "btn btn-ghost btn-circle btn-sm",
                    aria_label: tid!("share-link"),
                    onclick: move |_| props.on_share.call(()),
                    ShareIcon { size: ICON_HEADER }
                }
                button {
                    id: "project-history-btn",
                    class: "btn btn-ghost btn-circle btn-sm",
                    aria_label: tid!("project-history-title"),
                    onclick: move |_| props.on_history.call(()),
                    HistoryIcon { size: ICON_HEADER }
                }
                DropdownButton {
                    id: "project-actions",
                    label: tid!("project-actions"),
                    DropdownItem {
                        variant: "primary",
                        label: tid!("edit"),
                        onclick: move |_| props.on_edit.call(()),
                    }
                    DropdownItem {
                        variant: "error",
                        label: tid!("leave"),
                        onclick: move |_| props.on_leave.call(()),
                    }
                    ProjectStatusItems {
                        status,
                        on_apply: move |s| props.on_status.call(s),
                    }
                    li { hr { class: "my-1 border-base-200" } }
                    li {
                        button {
                            onclick: move |_| props.on_export_json.call(()),
                            {tid!("export-json")}
                        }
                    }
                    li {
                        button {
                            onclick: move |_| props.on_export_csv.call(()),
                            {tid!("export-csv")}
                        }
                    }
                    if let Some(on_invite) = props.on_invite {
                        li {
                            button {
                                id: "invite-friends-item",
                                onclick: move |_| on_invite.call(()),
                                {tid!("invite-friends-title")}
                            }
                        }
                    }
                    if let Some(link) = props.native_app_link.clone() {
                        li {
                            // Plain href so the OS resolves the scheme.
                            a { href: "{link}", {tid!("open-in-app")} }
                        }
                    }
                }
            }
        },
    }
}
