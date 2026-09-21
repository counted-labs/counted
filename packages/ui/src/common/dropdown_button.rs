use dioxus::prelude::*;

#[derive(PartialEq, Props, Clone)]
pub struct DropdownButtonProps {
    children: Element,
    /// Falls back to "..." when None.
    trigger: Option<Element>,
    /// The trigger is icon-only, so without this it has no accessible name at all.
    #[props(default = "Actions".to_string())]
    label: String,
    /// Override the trigger button classes.
    button_class: Option<String>,
    /// Override the menu (ul) classes.
    menu_class: Option<String>,
    /// Close the menu when a menu item is clicked. Set false for menus with toggles.
    #[props(default = true)]
    close_on_select: bool,
    /// DOM id for the trigger. E2E selects on it — `aria-label` is French copy and changes freely.
    id: Option<String>,
    /// For callers with another way in — e.g. a long press on the surrounding card. Omitted, the
    /// trigger button is the only way to open it.
    open: Option<Signal<bool>>,
}

#[component]
pub fn DropdownButton(props: DropdownButtonProps) -> Element {
    // The hook runs unconditionally; the caller's signal just shadows it when provided.
    let internal = use_signal(|| false);
    let mut is_open = props.open.unwrap_or(internal);

    let button_class = props
        .button_class
        .clone()
        .unwrap_or_else(|| "btn btn-ghost btn-circle btn-sm relative z-50".to_string());
    let menu_class = props.menu_class.clone().unwrap_or_else(|| {
        "menu bg-base-100 rounded-box w-52 p-2 shadow absolute right-0 top-full mt-1 z-50".to_string()
    });
    let close_on_select = props.close_on_select;

    rsx! {
        div {
            class: "relative",
            // Touch counterpart of the ul's onclick guard, here rather than on the ul: ProjectCard
            // arms a long press from `ontouchstart` anywhere in its subtree and prevent_defaults
            // the resulting click, swallowing every menu action. One place covers the trigger, the
            // scrim and the menu alike.
            ontouchstart: move |e| e.stop_propagation(),
            ontouchend: move |e| e.stop_propagation(),
            // Keyboard counterpart of the onclick guards below. The scrim is pointer-only, so
            // without Escape a keyboard user cannot dismiss the menu. Enter/Space is swallowed for
            // the same reason the ul stops propagation: a clickable ancestor would navigate.
            onkeydown: move |e| {
                let key = e.key();
                if key == Key::Escape && is_open() {
                    e.stop_propagation();
                    is_open.set(false);
                } else if key == Key::Enter || key == Key::Character(" ".to_string()) {
                    e.stop_propagation();
                }
            },
            if is_open() {
                div {
                    class: "fixed inset-0 z-40",
                    onclick: move |e| { e.stop_propagation(); is_open.set(false); },
                }
            }
            button {
                id: props.id.clone(),
                r#type: "button",
                class: "{button_class}",
                aria_label: "{props.label}",
                aria_haspopup: "menu",
                aria_expanded: is_open(),
                onclick: move |e| { e.stop_propagation(); is_open.set(!is_open()); },
                if let Some(trigger) = props.trigger.clone() {
                    {trigger}
                } else {
                    "..."
                }
            }
            if is_open() {
                ul {
                    class: "{menu_class}",
                    // Unconditional — the one place keeping menu actions from reaching a clickable
                    // parent (ProjectCard navigating). Applies to toggle menus too.
                    onclick: move |e| {
                        e.stop_propagation();
                        if close_on_select {
                            is_open.set(false);
                        }
                    },
                    {props.children}
                }
            }
        }
    }
}

#[derive(PartialEq, Props, Clone)]
pub struct DropdownItemProps {
    pub variant: String,
    pub label: String,
    pub onclick: EventHandler<()>,
    /// DOM id for the item button — see `DropdownButtonProps::id`.
    pub id: Option<String>,
}

#[component]
pub fn DropdownItem(props: DropdownItemProps) -> Element {
    let text_class = match props.variant.as_str() {
        "error" => "text-error",
        _ => "",
    };
    rsx! {
        li {
            button {
                id: props.id.clone(),
                r#type: "button",
                class: "{text_class}",
                onclick: move |_| props.onclick.call(()),
                "{props.label}"
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::test_dom::{click, listener_ids, touch};
    use dioxus::dioxus_core::{ElementId, Mutations};
    use std::cell::Cell;
    use std::rc::Rc;

    /// Mirrors ProjectCard: a clickable container that navigates and arms a long press from
    /// `ontouchstart` below it. The handlers flip an Rc<Cell<bool>>, readable outside a scope.
    #[derive(Props, Clone, PartialEq)]
    struct HarnessProps {
        outer: Rc<Cell<bool>>,
        pressed: Rc<Cell<bool>>,
        item: Rc<Cell<bool>>,
    }

    #[component]
    fn Harness(props: HarnessProps) -> Element {
        let outer = props.outer.clone();
        let pressed = props.pressed.clone();
        let item = props.item.clone();
        rsx! {
            div {
                onclick: move |_| outer.set(true),
                ontouchstart: move |_| pressed.set(true),
                DropdownButton {
                    DropdownItem {
                        variant: "neutral",
                        label: "Archiver",
                        onclick: move |_| item.set(true),
                    }
                }
            }
        }
    }

    fn click_ids(m: Mutations) -> Vec<ElementId> {
        listener_ids(&m, "click")
    }

    fn touch_ids(m: Mutations) -> Vec<ElementId> {
        listener_ids(&m, "touchstart")
    }

    struct Card {
        dom: VirtualDom,
        outer: Rc<Cell<bool>>,
        pressed: Rc<Cell<bool>>,
        item: Rc<Cell<bool>>,
    }

    impl Card {
        fn new() -> Self {
            let outer = Rc::new(Cell::new(false));
            let pressed = Rc::new(Cell::new(false));
            let item = Rc::new(Cell::new(false));
            let dom = VirtualDom::new_with_props(
                Harness,
                HarnessProps { outer: outer.clone(), pressed: pressed.clone(), item: item.clone() },
            );
            Self { dom, outer, pressed, item }
        }

        /// Opens the menu through the trigger and returns the click ids the open menu created:
        /// [backdrop, ul, item button].
        fn open_menu(&mut self) -> Vec<ElementId> {
            let ids = click_ids(self.dom.rebuild_to_vec());
            assert_eq!(ids.len(), 2, "expected the card and the trigger to have click handlers");
            self.dom.runtime().handle_event("click", click(), ids[1]);
            let ids = click_ids(self.dom.render_immediate_to_vec());
            assert_eq!(ids.len(), 3, "expected backdrop, menu and item to have click handlers");
            ids
        }

        fn press(&mut self, el: ElementId) {
            self.dom.runtime().handle_event("touchstart", touch(), el);
        }
    }

    #[test]
    fn menu_item_click_does_not_reach_the_surrounding_card() {
        let mut card = Card::new();

        let ids = card.open_menu();
        card.dom.runtime().handle_event("click", click(), ids[2]);

        assert!(card.item.get(), "the item handler must fire — otherwise the dispatch missed it");
        assert!(!card.outer.get(), "a menu click must not bubble into the card's onclick");
    }

    #[test]
    fn trigger_click_does_not_reach_the_surrounding_card() {
        let mut card = Card::new();

        // The menu opening is the proof the trigger handler ran.
        card.open_menu();
        assert!(!card.outer.get(), "opening the menu must not bubble into the card's onclick");
    }

    /// The card prevent_defaults the click after a long press, so arming the timer from inside
    /// the menu would silently swallow the action.
    #[test]
    fn a_touch_on_a_menu_item_does_not_arm_the_cards_long_press() {
        let mut card = Card::new();

        let ids = card.open_menu();
        // [card, dropdown root] — the menu adds no touchstart handler of its own, so the item
        // button's id is the one the click listeners named.
        let guards = touch_ids(card.dom.rebuild_to_vec());
        assert_eq!(guards.len(), 2, "expected the card and the dropdown root to guard touchstart");

        card.press(ids[2]);

        assert!(!card.pressed.get(), "a press on a menu item must not arm the card's long press");
    }

    /// The scrim is `fixed inset-0`: while the menu is open it covers the whole viewport, but it
    /// is still a descendant of the card.
    #[test]
    fn a_touch_on_the_backdrop_does_not_arm_the_cards_long_press() {
        let mut card = Card::new();

        let ids = card.open_menu();
        card.press(ids[0]);

        assert!(!card.pressed.get(), "a press on the scrim must not arm the card's long press");
    }

    #[test]
    fn a_touch_outside_the_dropdown_still_arms_the_cards_long_press() {
        let mut card = Card::new();

        let ids = touch_ids(card.dom.rebuild_to_vec());
        card.press(ids[0]);

        assert!(card.pressed.get(), "the guard must be local to the dropdown, not disable long press");
    }
}
