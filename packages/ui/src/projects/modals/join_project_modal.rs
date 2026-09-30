use api::account_projects::account_projects_controller::upsert_account_project;
use dioxus::fullstack::Json;
use dioxus::prelude::*;
use crate::tid;
use shared::{Account as AccountData, UpsertAccountProject};

use crate::common::{adopt_project_key, parse_share_link, LocalStorageState, NativeClipboardReader};
use crate::crypto::{claim_token, wrap_key};
use crate::icons::{CloseIcon, ICON_HEADER};
use crate::route::Route;

/// Brings a shared project link into the app by hand — the counterpart of the native deep link,
/// and the only path available when the link arrives somewhere the OS cannot hand to the app
/// (a desktop browser, a chat app that strips the handoff, an iOS build without universal links).
///
/// The encryption key lives only in the link's fragment, so joining is a local-storage write plus a
/// navigation — with one fetch of the project when a different key is already held, to prove the
/// new one before it replaces it. The account link is best-effort on top.
#[derive(Props, Clone, PartialEq)]
pub struct JoinProjectModalProps {
    pub on_close: EventHandler<()>,
    /// Set when unlocking a specific project rather than joining any: a link for a *different*
    /// project is then rejected instead of silently navigating somewhere the user did not ask for.
    #[props(default)]
    pub expect_project_id: Option<uuid::Uuid>,
}

#[component]
pub fn JoinProjectModal(props: JoinProjectModalProps) -> Element {
    let mut link = use_signal(String::new);
    let mut error_msg: Signal<Option<String>> = use_signal(|| None);

    // WebView only: a native paste (long-press → Coller) never reaches oninput there. On the web
    // oninput already fires on paste, and this listener would need eval — CSP-blocked, see
    // `common::web_dom`.
    #[cfg(not(target_arch = "wasm32"))]
    use_effect(move || {
        spawn(async move {
            let mut eval = document::eval(
                r#"(function(){var i=document.getElementById('join-link-input');if(!i)return;i.addEventListener('paste',function(e){var t=(e.clipboardData||window.clipboardData).getData('text/plain');if(t)dioxus.send(t);});})()"#,
            );
            while let Ok(val) = eval.recv::<serde_json::Value>().await {
                if let Some(text) = val.as_str() {
                    let text = text.trim().to_string();
                    if !text.is_empty() {
                        link.set(text);
                        error_msg.set(None);
                    }
                }
            }
        });
    });

    let clipboard_ctx = use_context::<Option<NativeClipboardReader>>();

    let nav = use_navigator();
    let auth_ctx = use_context::<Signal<Option<AccountData>>>();
    let account_key_ctx = use_context::<Signal<Option<[u8; 32]>>>();
    let ls_ctx = use_context::<Signal<LocalStorageState>>();

    let on_submit = move |e: FormEvent| {
        e.prevent_default();
        let Some((project_id, key)) = parse_share_link(&link()) else {
            error_msg.set(Some(tid!("join-invalid-link")));
            return;
        };
        if props.expect_project_id.is_some_and(|expected| expected != project_id) {
            error_msg.set(Some(tid!("join-wrong-project")));
            return;
        }

        let is_auth = auth_ctx().is_some();
        let escrowed = account_key_ctx().and_then(|ak| wrap_key(&ak, &key));
        let token = claim_token(&key).to_vec();
        let on_close = props.on_close;
        spawn(async move {
            // The key must be in local storage *before* navigating: on mobile there is no URL
            // fragment to read it back from (MemoryHistory), so ExpensesPage resolves it from here.
            let adopted = adopt_project_key(ls_ctx, project_id, key).await;
            on_close.call(());
            nav.push(Route::ExpensesPage { project_id });
            if is_auth && adopted {
                // user_id is None — the user identifies themselves via UserSelectionModal, which is
                // also what attaches the claim label. Nothing to claim here yet.
                let _ = upsert_account_project(Json(UpsertAccountProject {
                    project_id,
                    user_id: None,
                    key: escrowed,
                    claim_label: None,
                    claim_token: Some(token),
                }))
                .await;
            }
        });
    };

    let on_close = props.on_close;

    rsx! {
        div { class: "modal modal-open modal-bottom sm:modal-middle", role: "dialog",
            div { class: "modal-box max-w-sm p-0 flex flex-col",
                div { class: "flex items-center justify-between px-6 pt-5 pb-4 border-b border-base-200 flex-shrink-0",
                    h3 { class: "font-bold text-lg font-display", {tid!("projects-join")} }
                    button {
                        r#type: "button",
                        class: "btn btn-ghost btn-circle h-11 w-11 min-h-11",
                        aria_label: tid!("close"),
                        onclick: move |_| on_close.call(()),
                        CloseIcon { size: ICON_HEADER }
                    }
                }
                form { class: "flex flex-col flex-1 overflow-hidden", onsubmit: on_submit,
                    div { class: "flex-1 overflow-y-auto px-6 py-4 flex flex-col gap-4",
                        if let Some(err) = error_msg() {
                            div { role: "alert", class: "alert alert-error text-sm", "{err}" }
                        }
                        fieldset { class: "fieldset",
                            label { class: "fieldset-legend", r#for: "join-link-input", {tid!("join-link-label")} }
                            div { class: "join w-full",
                                input {
                                    id: "join-link-input",
                                    class: "input join-item flex-1 min-w-0",
                                    r#type: "text",
                                    placeholder: "https://counted.fr/projects/…",
                                    value: "{link}",
                                    oninput: move |e| link.set(e.value()),
                                }
                                button {
                                    id: "join-paste-btn",
                                    r#type: "button",
                                    class: "btn btn-outline join-item shrink-0",
                                    onclick: move |_| {
                                        match &clipboard_ctx {
                                            Some(reader) => {
                                                if let Some(text) = reader() {
                                                    let text = text.trim().to_string();
                                                    if !text.is_empty() {
                                                        link.set(text);
                                                        error_msg.set(None);
                                                    }
                                                }
                                            }
                                            None => {
                                                spawn(async move {
                                                    #[cfg(target_arch = "wasm32")]
                                                    let pasted = crate::common::web_dom::read_clipboard()
                                                        .await;
                                                    #[cfg(not(target_arch = "wasm32"))]
                                                    let pasted = document::eval(
                                                            crate::common::READ_CLIPBOARD_JS,
                                                        )
                                                        .await
                                                        .ok()
                                                        .and_then(|v| v.as_str().map(str::to_string));
                                                    if let Some(text) = pasted {
                                                        let text = text.trim().to_string();
                                                        if !text.is_empty() {
                                                            link.set(text);
                                                            error_msg.set(None);
                                                        }
                                                    }
                                                });
                                            }
                                        }
                                    },
                                    {tid!("paste")}
                                }
                            }
                            span { class: "label text-xs whitespace-normal",
                                {tid!("join-link-hint")}
                            }
                        }
                    }
                    div { class: "flex justify-end gap-2 px-6 py-4 border-t border-base-200 flex-shrink-0",
                        button {
                            r#type: "button",
                            class: "btn",
                            onclick: move |_| props.on_close.call(()),
                            {tid!("cancel")}
                        }
                        button {
                            id: "join-submit",
                            r#type: "submit",
                            class: "btn btn-primary",
                            {tid!("join")}
                        }
                    }
                }
            }
            div { class: "modal-backdrop", onclick: move |_| props.on_close.call(()) }
        }
    }
}
