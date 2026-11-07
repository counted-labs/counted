use dioxus::prelude::*;

use crate::common::{haptic, Haptic};
use crate::tid;
use crate::icons::{CloseIcon, PlusIcon, ICON_INLINE};

/// A FAB expanding into labelled actions, rebuilt on a signal.
///
/// Not `class="fab"`: daisyUI 5.0.43 ships no `fab` at all, and upstream's opens through
/// `.fab:focus-within`. iOS only focuses form controls and links on tap, so a focus-driven FAB
/// never opens in WKWebView. The toggle is a `use_signal` on a real `<button>` over a
/// `fixed inset-0` scrim.
#[derive(PartialEq, Props, Clone)]
pub struct SpeedDialFabProps {
    /// `SpeedDialAction`s, rendered above the trigger in source order.
    children: Element,
    /// Trigger content when closed. Falls back to `PlusIcon`.
    trigger: Option<Element>,
    /// Accessible name for the closed trigger.
    #[props(default = "Actions".to_string())]
    label: String,
    /// DOM id for the trigger. E2E selects on it - `aria-label` is French copy and changes freely.
    id: Option<String>,
}

#[component]
pub fn SpeedDialFab(props: SpeedDialFabProps) -> Element {
    let mut is_open = use_signal(|| false);
    let mut trigger: Signal<Option<std::rc::Rc<MountedData>>> = use_signal(|| None);

    rsx! {
        // Below the scrim's z-30 when closed is fine: nothing overlaps it, and the z-50 dock sits
        // lower on screen. `safe-bottom-fab` is the safe-area-aware offset.
        if is_open() {
            div {
                class: "fixed inset-0 z-30 bg-base-content/30",
                onclick: move |e| {
                    e.stop_propagation();
                    is_open.set(false);
                },
            }
        }
        div {
            class: "fixed safe-bottom-fab left-1/2 -translate-x-1/2 z-40 flex flex-col items-end gap-3",
            onkeydown: move |e| {
                if e.key() == Key::Escape && is_open() {
                    e.prevent_default();
                    is_open.set(false);
                    if let Some(trigger) = trigger() {
                        spawn(async move {
                            let _ = trigger.set_focus(true).await;
                        });
                    }
                }
            },
            if is_open() {
                // One handler for the whole stack - the single place that closes the dial.
                div {
                    class: "flex flex-col items-end gap-3",
                    onclick: move |e| {
                        e.stop_propagation();
                        is_open.set(false);
                    },
                    {props.children}
                }
            }
            // Icons, not "+"/"✕" text: flex centring centres the *line box*, but a glyph's ink
            // sits on baseline metrics - "+" measured 1.3px low in a 48px button, "✕" 1.0px. An
            // inline SVG has no baseline to fight. `size-6` matches `SpeedDialAction`, so trigger
            // and actions carry identically sized icons.
            button {
                id: props.id.clone(),
                r#type: "button",
                // `hover:brightness-*`, not `hover:bg-*`: daisyUI's `.btn` hover only sets
                // `--btn-bg` → `background-color`, which `.bg-gradient-brand`'s `background-image`
                // paints over. A filter is the one thing the gradient can't occlude, and
                // Tailwind's `@media (hover: hover)` spares touch.
                class: "btn btn-circle btn-lg border-0 text-primary-content bg-gradient-brand shadow-soft self-center [&>svg]:size-6 hover:brightness-110 active:brightness-95 transition-[filter]",
                "aria-label": if is_open() { tid!("close") } else { props.label.clone() },
                "aria-expanded": is_open(),
                onmounted: move |e| trigger.set(Some(e.data())),
                onclick: move |e| {
                    e.stop_propagation();
                    haptic(Haptic::Light);
                    is_open.set(!is_open());
                },
                if is_open() {
                    CloseIcon { size: ICON_INLINE }
                } else if let Some(trigger) = props.trigger.clone() {
                    {trigger}
                } else {
                    PlusIcon { size: ICON_INLINE }
                }
            }
        }
    }
}

#[derive(PartialEq, Props, Clone)]
pub struct SpeedDialActionProps {
    pub label: String,
    pub icon: Element,
    pub onclick: EventHandler<()>,
    /// DOM id for the action button - see `SpeedDialFabProps::id`.
    pub id: Option<String>,
    /// The one action the dial is mostly opened for: brand-filled and bold.
    #[props(default)]
    pub primary: bool,
}

#[component]
pub fn SpeedDialAction(props: SpeedDialActionProps) -> Element {
    rsx! {
        button {
            id: props.id.clone(),
            r#type: "button",
            class: "flex items-center gap-3 cursor-pointer",
            "aria-label": "{props.label}",
            onclick: move |_| props.onclick.call(()),
            span {
                class: if props.primary { "bg-base-100 shadow-soft rounded-box px-3 py-1 text-sm font-semibold whitespace-nowrap" } else { "bg-base-100 shadow-soft rounded-box px-3 py-1 text-sm font-medium whitespace-nowrap" },
                "{props.label}"
            }
            // Icons come from callers at whatever intrinsic size they were drawn at
            // (PlusIcon is 16, LinkIcon 22); normalise here so the row reads as one set.
            span {
                class: if props.primary { "btn btn-circle btn-lg bg-gradient-brand text-primary-content border-0 shadow-soft [&>svg]:size-6" } else { "btn btn-circle btn-lg bg-base-100 border-0 shadow-soft [&>svg]:size-6" },
                {props.icon.clone()}
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::test_dom::listener_ids;
    use dioxus::dioxus_core::ElementId;
    use dioxus_html::{
        set_event_converter, PlatformEventData, SerializedHtmlEventConverter, SerializedMouseData,
    };
    use std::any::Any;
    use std::cell::Cell;
    use std::rc::Rc;
    use std::sync::Once;

    #[derive(Props, Clone, PartialEq)]
    struct HarnessProps {
        /// Flipped by the single action's handler.
        fired: Rc<Cell<bool>>,
    }

    #[component]
    fn Harness(props: HarnessProps) -> Element {
        crate::i18n::use_test_i18n();
        let fired = props.fired.clone();
        rsx! {
            SpeedDialFab { label: "Ajouter",
                SpeedDialAction {
                    label: "Rejoindre un projet",
                    icon: rsx! {
                        span { "L" }
                    },
                    onclick: move |_| fired.set(true),
                }
            }
        }
    }

    /// The globally installed converter is what turns the platform data back into typed
    /// MouseData.
    fn click() -> Event<dyn Any> {
        static CONVERTER: Once = Once::new();
        CONVERTER.call_once(|| set_event_converter(Box::new(SerializedHtmlEventConverter)));
        let data = PlatformEventData::new(Box::new(SerializedMouseData::default()));
        Event::new(Rc::new(data) as Rc<dyn Any>, true)
    }

    fn harness() -> (VirtualDom, Rc<Cell<bool>>) {
        let fired = Rc::new(Cell::new(false));
        let dom = VirtualDom::new_with_props(Harness, HarnessProps { fired: fired.clone() });
        (dom, fired)
    }

    /// Opens the dial and returns every click target that appeared.
    fn open(dom: &mut VirtualDom) -> Vec<ElementId> {
        let m = dom.rebuild_to_vec();
        let closed = listener_ids(&m, "click");
        assert_eq!(closed.len(), 1, "only the trigger is clickable while closed: {closed:?}");
        dom.runtime().handle_event("click", click(), closed[0]);
        listener_ids(&dom.render_immediate_to_vec(), "click")
    }

    #[test]
    fn opens_on_a_real_button_click_not_focus() {
        let (mut dom, _fired) = harness();
        // scrim, action-stack wrapper, action button - the trigger keeps its existing listener.
        assert_eq!(open(&mut dom).len(), 3, "the trigger click must reveal the action stack");
    }

    #[test]
    fn an_action_click_fires_its_handler_and_closes_the_dial() {
        let (mut dom, fired) = harness();
        let ids = open(&mut dom);

        // Discover the action button rather than hardcoding an index: a template change then
        // fails the assertion below instead of silently dispatching at the scrim.
        let action = *ids
            .iter()
            .find(|id| {
                let probe = Rc::new(Cell::new(false));
                let mut probe_dom =
                    VirtualDom::new_with_props(Harness, HarnessProps { fired: probe.clone() });
                open(&mut probe_dom);
                probe_dom.runtime().handle_event("click", click(), **id);
                probe.get()
            })
            .expect("one of the revealed elements must be the action button");

        dom.runtime().handle_event("click", click(), action);
        assert!(fired.get(), "the action handler must fire");

        let after = listener_ids(&dom.render_immediate_to_vec(), "click");
        assert!(
            after.is_empty(),
            "the action stack and scrim must be torn down, i.e. the dial closed: {after:?}"
        );
    }

    #[test]
    fn the_scrim_closes_the_dial_without_firing_an_action() {
        let (mut dom, fired) = harness();
        let ids = open(&mut dom);

        // The scrim is created before the stack it covers, so it is the first new target.
        dom.runtime().handle_event("click", click(), ids[0]);
        assert!(!fired.get(), "the scrim must not trigger an action");
        assert!(
            listener_ids(&dom.render_immediate_to_vec(), "click").is_empty(),
            "the scrim must close the dial"
        );
    }
}
