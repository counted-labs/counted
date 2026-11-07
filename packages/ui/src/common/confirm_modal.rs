use dioxus::prelude::*;
use std::future::{poll_fn, Future};
use std::sync::{Arc, Mutex, OnceLock};
use std::task::{Poll, Waker};

use crate::common::{haptic, use_modal_focus, Haptic};
use crate::tid;

#[derive(Clone, PartialEq)]
pub struct DialogAction {
    pub id: &'static str,
    pub label: String,
    /// daisyUI button classes, e.g. `btn-error` for the irreversible one.
    pub class: &'static str,
    pub on_click: EventHandler<()>,
}

impl DialogAction {
    fn destructive(&self) -> bool {
        self.class.contains("btn-error")
    }
}

/// What a platform alert shows: the actions in order, each `(label, destructive)`, then Cancel.
pub struct ConfirmRequest {
    pub title: String,
    pub message: String,
    pub actions: Vec<(String, bool)>,
    pub cancel: String,
    /// An action sheet rather than an alert: deleting a row or an object, discarding a form.
    pub action_sheet: bool,
}

/// The index of the action picked, `None` for Cancel. Called once, on the UI thread.
pub type ConfirmReply = Box<dyn FnOnce(Option<usize>) + Send>;

/// Presents a platform alert, true when it is on screen. False and the HTML dialog renders.
pub type NativeConfirm = Arc<dyn Fn(ConfirmRequest, ConfirmReply) -> bool + Send + Sync + 'static>;

static CONFIRM: OnceLock<NativeConfirm> = OnceLock::new();

/// iOS installs `UIAlertController` here before launch; every other target leaves it unset and
/// gets the HTML dialog. Same construction as `set_native_sharer`.
pub fn set_native_confirm(confirm: NativeConfirm) {
    let _ = CONFIRM.set(confirm);
}

#[derive(Default)]
struct Reply {
    choice: Option<Option<usize>>,
    waker: Option<Waker>,
}

/// A one-shot channel by hand: the reply comes from a UIKit block, the wait is a Dioxus task.
fn reply_channel() -> (ConfirmReply, impl Future<Output = Option<usize>>) {
    let slot = Arc::new(Mutex::new(Reply::default()));
    let tx = slot.clone();
    let send: ConfirmReply = Box::new(move |choice| {
        let mut reply = tx.lock().unwrap_or_else(|e| e.into_inner());
        reply.choice = Some(choice);
        if let Some(waker) = reply.waker.take() {
            waker.wake();
        }
    });
    let wait = poll_fn(move |cx| {
        let mut reply = slot.lock().unwrap_or_else(|e| e.into_inner());
        match reply.choice.take() {
            Some(choice) => Poll::Ready(choice),
            None => {
                reply.waker = Some(cx.waker().clone());
                Poll::Pending
            }
        }
    });
    (send, wait)
}

fn choose(actions: &[DialogAction], index: usize) {
    if let Some(action) = actions.get(index) {
        haptic(if action.destructive() { Haptic::Warning } else { Haptic::Light });
        action.on_click.call(());
    }
}

/// The app's one confirmation. On iOS a real `UIAlertController` (an action sheet when
/// `action_sheet`), so the system look, VoiceOver and Dynamic Type come with it; elsewhere, and on
/// iOS when no alert could be presented, one HTML dialog that `native-feel.css` draws as the M3
/// dialog on Android and a neutral card on the web. One action sits right of Cancel; several stack
/// full width, Cancel last. Ids: the dialog `id`, `{id}-title`, `{id}-message`, `{id}-cancel`
/// (the E2E suite selects on them, docs/e2e.md).
#[component]
pub fn ConfirmDialog(
    id: &'static str,
    title: String,
    message: String,
    actions: Vec<DialogAction>,
    on_cancel: EventHandler<()>,
    #[props(default)] action_sheet: bool,
) -> Element {
    let native = use_hook(|| {
        let Some(confirm) = CONFIRM.get() else { return false };
        let (send, wait) = reply_channel();
        let request = ConfirmRequest {
            title: title.clone(),
            message: message.clone(),
            actions: actions.iter().map(|a| (a.label.clone(), a.destructive())).collect(),
            cancel: tid!("cancel"),
            action_sheet,
        };
        if !confirm(request, send) {
            return false;
        }
        let actions = actions.clone();
        spawn(async move {
            match wait.await {
                Some(index) => choose(&actions, index),
                None => on_cancel.call(()),
            }
        });
        true
    });
    if native {
        return rsx! {};
    }
    rsx! {
        HtmlConfirm { id, title, message, actions, on_cancel }
    }
}

#[component]
fn HtmlConfirm(
    id: &'static str,
    title: String,
    message: String,
    actions: Vec<DialogAction>,
    on_cancel: EventHandler<()>,
) -> Element {
    use_modal_focus(id);
    let stacked = actions.len() > 1;
    let cancel = rsx! {
        button {
            id: "{id}-cancel",
            r#type: "button",
            class: if stacked { "btn btn-ghost w-full confirm-cancel" } else { "btn confirm-cancel" },
            onclick: move |_| on_cancel.call(()),
            {tid!("cancel")}
        }
    };
    rsx! {
        div {
            id,
            class: "modal modal-open confirm-dialog",
            role: "alertdialog",
            aria_modal: "true",
            aria_labelledby: "{id}-title",
            aria_describedby: "{id}-message",
            tabindex: "-1",
            div { class: "modal-box max-w-sm p-0 flex flex-col",
                div { class: "flex flex-col gap-2 px-6 pt-6 pb-4",
                    h2 { id: "{id}-title", class: "font-bold text-lg font-display", "{title}" }
                    p { id: "{id}-message", class: "text-sm text-base-content/70 whitespace-pre-line", "{message}" }
                }
                div { class: if stacked { "confirm-actions confirm-actions-stacked flex flex-col gap-2 px-6 pb-6" } else { "confirm-actions flex justify-end gap-2 px-6 pb-6" },
                    if !stacked {
                        {cancel.clone()}
                    }
                    for (index , action) in actions.iter().cloned().enumerate() {
                        {
                            let all = actions.clone();
                            rsx! {
                                button {
                                    id: action.id,
                                    r#type: "button",
                                    class: if stacked { "btn w-full {action.class}" } else { "btn {action.class}" },
                                    onclick: move |_| choose(&all, index),
                                    "{action.label}"
                                }
                            }
                        }
                    }
                    if stacked {
                        {cancel}
                    }
                }
            }
            div { class: "modal-backdrop", onclick: move |_| on_cancel.call(()) }
        }
    }
}

/// Yes/no for an action that cannot be undone.
#[derive(PartialEq, Props, Clone)]
pub struct ConfirmModalProps {
    pub title: String,
    pub message: String,
    pub confirm_label: String,
    pub on_confirm: EventHandler<()>,
    pub on_cancel: EventHandler<()>,
    /// An action sheet on iOS: removing a row or an object rather than answering a question.
    #[props(default)]
    pub action_sheet: bool,
}

#[component]
pub fn ConfirmModal(props: ConfirmModalProps) -> Element {
    rsx! {
        ConfirmDialog {
            id: "confirm-modal",
            title: props.title,
            message: props.message,
            actions: vec![DialogAction {
                id: "confirm-modal-confirm",
                label: props.confirm_label,
                class: "btn-error",
                on_click: props.on_confirm,
            }],
            on_cancel: props.on_cancel,
            action_sheet: props.action_sheet,
        }
    }
}

/// Asked before a changed form is thrown away, on any close: ✕, backdrop, Android back, a drag.
#[component]
pub fn DiscardConfirm(on_keep: EventHandler<()>, on_discard: EventHandler<()>) -> Element {
    rsx! {
        ConfirmModal {
            title: tid!("discard-title"),
            message: tid!("discard-message"),
            confirm_label: tid!("discard"),
            action_sheet: true,
            on_confirm: on_discard,
            on_cancel: on_keep,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::task::{Context, Waker};

    #[test]
    fn the_reply_reaches_the_waiting_task() {
        let (send, wait) = reply_channel();
        let mut wait = Box::pin(wait);
        let mut cx = Context::from_waker(Waker::noop());
        assert!(wait.as_mut().poll(&mut cx).is_pending());
        send(Some(1));
        assert_eq!(wait.as_mut().poll(&mut cx), Poll::Ready(Some(1)));
    }

    #[test]
    fn a_reply_before_the_first_poll_is_kept() {
        let (send, wait) = reply_channel();
        send(None);
        let mut wait = Box::pin(wait);
        let mut cx = Context::from_waker(Waker::noop());
        assert_eq!(wait.as_mut().poll(&mut cx), Poll::Ready(None));
    }
}
