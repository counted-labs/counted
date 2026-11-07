use std::cell::Cell;
use std::collections::VecDeque;
use std::rc::Rc;

use dioxus::prelude::*;

use crate::common::{haptic, sleep, Flash, Haptic};
use crate::icons::CheckIcon;

/// How long a toast stays on screen. Errors linger longer than successes - a failure needs more
/// reading time.
const SUCCESS_MS: u32 = 4000;
const ERROR_MS: u32 = 7000;
/// The exit transition on `.app-toast[data-leaving]` (native-feel.css), plus a frame.
const EXIT_MS: u32 = 220;

#[derive(Clone, PartialEq, Debug)]
struct Shown {
    id: u64,
    flash: Flash,
    leaving: bool,
}

/// One toast at a time, the next once the current one has played its exit, as a UIKit banner or
/// an M3 snackbar does.
#[derive(Default, Clone, PartialEq, Debug)]
struct Toasts {
    shown: Option<Shown>,
    waiting: VecDeque<Flash>,
    next_id: u64,
}

impl Toasts {
    /// A message already on screen or last in line is not queued twice: a burst of the same
    /// failure reads once.
    fn push(&mut self, flash: Flash) {
        let on_screen = self.shown.as_ref().is_some_and(|s| !s.leaving && s.flash == flash);
        if !on_screen && self.waiting.back() != Some(&flash) {
            self.waiting.push_back(flash);
        }
        self.advance();
    }

    /// Starts the exit of toast `id`. False when it is not the one shown or already leaving.
    fn leave(&mut self, id: u64) -> bool {
        match &mut self.shown {
            Some(s) if s.id == id && !s.leaving => {
                s.leaving = true;
                true
            }
            _ => false,
        }
    }

    /// Toast `id` has finished its exit: the next one comes in.
    fn gone(&mut self, id: u64) {
        if self.shown.as_ref().is_some_and(|s| s.id == id) {
            self.shown = None;
            self.advance();
        }
    }

    fn advance(&mut self) {
        if self.shown.is_none() {
            if let Some(flash) = self.waiting.pop_front() {
                self.next_id += 1;
                self.shown = Some(Shown { id: self.next_id, flash, leaving: false });
            }
        }
    }
}

/// Every `Flash` raised anywhere, shown one at a time. One markup on every platform, styled per
/// `html[data-plat]` in native-feel.css: an iOS banner under the status bar (swiped up to dismiss,
/// boot.js), an Android snackbar above the dock that lifts the FAB, a neutral bottom toast on the
/// web. A tap dismisses it on all three.
#[component]
pub fn ToastHost() -> Element {
    let mut flash = use_context::<Signal<Option<Flash>>>();
    let mut toasts = use_signal(Toasts::default);

    // The context is a mailbox: drained here, so the same message raised twice is two writes.
    use_effect(move || {
        if let Some(f) = flash() {
            flash.set(None);
            toasts.write().push(f);
        }
    });

    let started = use_hook(|| Rc::new(Cell::new(0u64)));
    use_effect(move || {
        let Some(shown) = toasts.read().shown.clone() else { return };
        let id = shown.id;
        if shown.leaving {
            spawn(async move {
                sleep(EXIT_MS).await;
                toasts.write().gone(id);
            });
            return;
        }
        if started.replace(id) == id {
            return;
        }
        let success = shown.flash.success;
        haptic(if success { Haptic::Success } else { Haptic::Error });
        spawn(async move {
            sleep(if success { SUCCESS_MS } else { ERROR_MS }).await;
            toasts.write().leave(id);
        });
    });

    let shown = toasts.read().shown.clone();
    rsx! {
        if let Some(s) = shown.clone() {
            div {
                key: "{s.id}",
                class: "app-toast",
                "data-kind": if s.flash.success { "success" } else { "error" },
                "data-leaving": if s.leaving { "true" },
                // A success is spoken by the permanent live region below: a region inserted
                // together with its text is often not announced. `alert` is, on insertion.
                role: if !s.flash.success { "alert" },
                aria_hidden: if s.flash.success { "true" },
                onclick: move |event| {
                    event.stop_propagation();
                    toasts.write().leave(s.id);
                },
                span { class: "app-toast-icon", aria_hidden: "true",
                    if s.flash.success {
                        CheckIcon { size: 18 }
                    } else {
                        "!"
                    }
                }
                span { class: "min-w-0 flex-1", "{s.flash.msg}" }
            }
        }
        div { class: "sr-only", role: "status", aria_live: "polite", aria_atomic: "true",
            if let Some(s) = shown.filter(|s| s.flash.success) {
                "{s.flash.msg}"
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn shown(t: &Toasts) -> Option<(u64, &str, bool)> {
        t.shown.as_ref().map(|s| (s.id, s.flash.msg.as_str(), s.leaving))
    }

    #[test]
    fn one_toast_at_a_time_the_next_after_the_exit() {
        let mut t = Toasts::default();
        t.push(Flash::ok("a"));
        t.push(Flash::err("b"));
        assert_eq!(shown(&t), Some((1, "a", false)));

        assert!(t.leave(1));
        t.push(Flash::ok("c"));
        assert_eq!(shown(&t), Some((1, "a", true)), "the next waits for the exit");

        t.gone(1);
        assert_eq!(shown(&t), Some((2, "b", false)));
        t.leave(2);
        t.gone(2);
        assert_eq!(shown(&t), Some((3, "c", false)));
        t.leave(3);
        t.gone(3);
        assert_eq!(shown(&t), None);
    }

    #[test]
    fn a_tap_and_the_timer_end_the_same_toast_once() {
        let mut t = Toasts::default();
        t.push(Flash::ok("a"));
        assert!(t.leave(1));
        assert!(!t.leave(1), "already leaving");
        t.gone(1);
        t.gone(1);
        assert_eq!(shown(&t), None);
    }

    #[test]
    fn a_late_timer_does_not_touch_the_toast_after_it() {
        let mut t = Toasts::default();
        t.push(Flash::ok("a"));
        t.push(Flash::ok("b"));
        t.leave(1);
        t.gone(1);
        assert!(!t.leave(1), "the first toast's timer fires after it is gone");
        t.gone(1);
        assert_eq!(shown(&t), Some((2, "b", false)));
    }

    #[test]
    fn the_same_message_is_not_queued_twice() {
        let mut t = Toasts::default();
        t.push(Flash::err("offline"));
        t.push(Flash::err("offline"));
        t.push(Flash::ok("saved"));
        t.push(Flash::ok("saved"));
        assert_eq!(t.waiting.len(), 1, "{:?}", t.waiting);
    }

    #[test]
    fn a_message_raised_again_while_its_toast_leaves_comes_back() {
        let mut t = Toasts::default();
        t.push(Flash::ok("copied"));
        t.leave(1);
        t.push(Flash::ok("copied"));
        t.gone(1);
        assert_eq!(shown(&t), Some((2, "copied", false)));
    }
}
