use dioxus::prelude::*;

use crate::common::{haptic, sleep, Haptic};

#[derive(PartialEq, Props, Clone)]
pub struct ToastProps {
    msg: String,
    #[props(default = false)]
    success: bool,
    onclose: EventHandler<()>,
}

/// How long a toast stays on screen. Errors linger longer than successes — a failure needs more
/// reading time.
const SUCCESS_MS: u32 = 4000;
const ERROR_MS: u32 = 7000;

#[component]
pub fn Toast(props: ToastProps) -> Element {
    let alert_class = if props.success { "alert-success" } else { "alert-error" };
    // Successes are informational; failures need to interrupt whatever is being read.
    let (role, live) = if props.success { ("status", "polite") } else { ("alert", "assertive") };
    let ms = if props.success { SUCCESS_MS } else { ERROR_MS };
    let onclose = props.onclose;

    // Runs once per mount. Raising a toast rebuilds this scope (the enclosing `if let` swaps the
    // dynamic node between a placeholder and the component), so the timer restarts with every new
    // message; dismissing by tap drops the scope, which cancels the pending task.
    // Runs once per mount, so the haptic follows the same rule as the timer: a new message is a
    // new scope, a re-render of the same one is not.
    use_hook(move || {
        haptic(if props.success { Haptic::Success } else { Haptic::Error });
        spawn(async move {
            sleep(ms).await;
            onclose.call(());
        })
    });

    rsx! {
        div {
            // `bottom-42` (10.5rem) clears the dock (`safe-bottom-dock`, 1.5rem + 3.75rem tall) and
            // the FAB above it (`safe-bottom-fab`, 6rem + 3.5rem tall) with a 1rem gap; `z-[60]`
            // beats the dock's `z-50` — daisyUI's `.toast` sets no z-index. Mobile's `.safe-toast`
            // restates the offset with the safe-area inset added.
            class: "toast toast-end safe-toast bottom-42 z-[60]",
            role,
            aria_live: live,
            onclick: move |event| {
                event.stop_propagation();
                onclose.call(())
            },
            div { class: "alert {alert_class}",
                span { class: "text-xs", "{props.msg}" }
            }
        }
    }
}
