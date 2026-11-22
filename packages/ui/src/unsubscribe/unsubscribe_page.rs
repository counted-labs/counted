use dioxus::prelude::*;
use dioxus::fullstack::Json;
use api::mail::mail_controller::unsubscribe;
use shared::UnsubscribePayload;

use crate::common::error_message;
use crate::route::Route;
use crate::tid;

#[derive(Clone, PartialEq)]
enum State {
    Asking,
    Busy,
    Done,
    Failed(String),
}

/// One tap, never on load: mail scanners open every link in a message, and must not unsubscribe
/// anyone (docs/plans/invite-emails.md §8).
fn use_unsubscribe(token: String) -> (Signal<State>, Callback<()>) {
    let mut state = use_signal(|| State::Asking);
    let confirm = use_callback(move |()| {
        let token = token.clone();
        spawn(async move {
            state.set(State::Busy);
            match unsubscribe(Json(UnsubscribePayload { token })).await {
                Ok(()) => state.set(State::Done),
                Err(e) => state.set(State::Failed(error_message(&e))),
            }
        });
    });
    (state, confirm)
}

#[component]
pub fn UnsubscribePage(token: String) -> Element {
    let (state, confirm) = use_unsubscribe(token);
    rsx! {
        div { class: "container app-container bg-base-100 p-4 max-w-md w-full mx-auto flex flex-col gap-6 mt-16",
            h1 { class: "font-display font-bold text-xl", {tid!("unsubscribe-title")} }
            match state() {
                State::Done => rsx! {
                    p { id: "unsubscribe-done", role: "status", {tid!("unsubscribe-done")} }
                },
                State::Failed(err) => rsx! {
                    div { id: "unsubscribe-error", role: "alert", class: "alert alert-error", "{err}" }
                },
                current => rsx! {
                    p { class: "text-base-content/70", {tid!("unsubscribe-body")} }
                    p { id: "unsubscribe-kept", class: "text-sm text-base-content/60",
                        {tid!("unsubscribe-kept")}
                        " "
                        Link { class: "link", to: Route::PrivacyPage {}, {tid!("nav-privacy")} }
                    }
                    button {
                        id: "unsubscribe-confirm",
                        r#type: "button",
                        class: "btn btn-primary w-full",
                        disabled: current == State::Busy,
                        onclick: move |_| confirm.call(()),
                        {tid!("unsubscribe-confirm")}
                    }
                },
            }
        }
    }
}
