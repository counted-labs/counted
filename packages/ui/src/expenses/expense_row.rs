//! One expense row: swipe left to delete, right to edit. Separate from `expenses_tab` because the
//! drag offset must be a *per-row* signal — one on the list re-renders every row per pointermove.
//!
//! **Why pointer events are affordable.** On mobile each dispatched event costs a *synchronous*
//! XHR to wry's custom protocol (`dioxus-interpreter-js`, `handleVirtualdomEventSync` →
//! `xhr.open(..., false)`) — why `ProjectCard` refuses `ontouchmove`. `touch-action: pan-y` on the
//! foreground is the escape: a vertical drag is claimed by the UA at once, firing `pointercancel`,
//! so scrolling costs a handful of events rather than one per frame. Horizontal drags are never
//! claimed, so moves flow only for the ~300ms of a deliberate swipe.
//!
//! Touch events cannot do this: `TouchPoint::identifier()` is not `pub` in dioxus-html 0.7.9, so a
//! touch cannot be tracked across events. `PointerData::pointer_id()` can. It also gates the web
//! build out — `pointer_type()` is `"mouse"` there, and a mouse never enters the state machine.

use dioxus::prelude::*;
use crate::tid;

use crate::common::{haptic, Avatar, Haptic, SizeClass};
use crate::icons::{PencilIcon, RepeatIcon, TrashIcon, ICON_ACTION};

/// How far the finger must travel before a release commits the action.
const SWIPE_THRESHOLD: f64 = 72.0;
/// Movement before the gesture is claimed for an axis. Below it, this is still a tap.
const AXIS_SLOP: f64 = 10.0;
/// Gestures this close to the left edge belong to the OS back-swipe — not enabled in any build
/// today (docs/mobile-navigation.md), but a row must not eat it the day it is.
const EDGE_GUARD: f64 = 24.0;
/// Never further than the panel behind it is wide (`w-20` = 5rem = 80px), plus overshoot so the
/// panel is fully uncovered at commit.
const MAX_PULL: f64 = 96.0;

#[derive(Clone, Copy, PartialEq, Debug)]
enum Axis {
    /// Still within the slop — could yet become a tap, a scroll or a swipe.
    Undecided,
    /// Ours: the row follows the finger.
    Horizontal,
    /// The UA's: it is a scroll, so let go of it.
    Abandoned,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub(crate) enum SwipeAction {
    Edit,
    Delete,
}

#[derive(Clone, Copy)]
struct Drag {
    start_x: f64,
    start_y: f64,
    pointer_id: i32,
    axis: Axis,
}

/// The dominant axis wins: a mostly-downward diagonal drag is a scroll, not a swipe.
fn decide_axis(dx: f64, dy: f64) -> Axis {
    if dx.abs().max(dy.abs()) < AXIS_SLOP {
        Axis::Undecided
    } else if dx.abs() > dy.abs() {
        Axis::Horizontal
    } else {
        Axis::Abandoned
    }
}

/// A pull towards an action the project status forbids does not move at all, so the affordance
/// never appears and the release cannot commit.
fn clamp_pull(dx: f64, can_edit: bool, can_delete: bool) -> f64 {
    let allowed = if dx > 0.0 { can_edit } else { can_delete };
    if !allowed {
        return 0.0;
    }
    dx.clamp(-MAX_PULL, MAX_PULL)
}

/// Right is edit, left is delete — the sides the affordances are painted on.
fn swipe_action(dx: f64, can_edit: bool, can_delete: bool) -> Option<SwipeAction> {
    let pull = clamp_pull(dx, can_edit, can_delete);
    if pull >= SWIPE_THRESHOLD {
        Some(SwipeAction::Edit)
    } else if pull <= -SWIPE_THRESHOLD {
        Some(SwipeAction::Delete)
    } else {
        None
    }
}

/// Fades in over the travel to the threshold, so full colour reads as "release now and it fires".
fn affordance_opacity(pull: f64) -> f64 {
    (pull / SWIPE_THRESHOLD).clamp(0.0, 1.0)
}

/// Whether the two swipe panels are in the tree at all. They are 11 of the row's 22 elements and
/// fully transparent at rest — 22 000 unseen DOM nodes on a 2000-expense project. They exist only
/// while a gesture is live; `settling` keeps them through the glide home so the row is never seen
/// sliding over an empty gutter.
fn affordances_mounted(dragging: bool, settling: bool) -> bool {
    dragging || settling
}

#[derive(PartialEq, Props, Clone)]
pub struct ExpenseRowProps {
    pub expense_id: i32,
    pub name: String,
    /// Already-formatted "Dépense payée par Alice" line.
    pub subtitle: String,
    pub emoji: String,
    /// Always in `currency` — an expense entered in another currency was converted at write time.
    pub amount: f64,
    pub currency: String,
    /// `Some((amount, code))` when the expense was entered in another currency, shown under the
    /// total as what was actually paid. The rate itself is on the detail page, not here.
    pub source: Option<(f64, String)>,
    /// Payments no longer adding up to the total — badged, never blocked.
    pub inconsistent: bool,
    /// "Mes dettes" filter only: this user's share, shown under the total. Every other call site
    /// omits it.
    pub my_debt: Option<f64>,
    #[props(default)]
    pub recurring: bool,
    #[props(default)]
    pub estimate: bool,
    pub can_edit: bool,
    pub can_delete: bool,
    /// A tap or Enter/Space — navigation is the parent's job, so this row needs no router.
    pub on_open: EventHandler<()>,
    pub on_edit: EventHandler<()>,
    pub on_delete: EventHandler<()>,
}

#[component]
pub fn ExpenseRow(props: ExpenseRowProps) -> Element {
    let mut drag: Signal<Option<Drag>> = use_signal(|| None);
    let mut pull = use_signal(|| 0.0f64);
    let mut settling = use_signal(|| false);
    // Read by `onclick`: the browser synthesises a click at the end of a swipe, and without this
    // the row would also navigate.
    let mut swiped = use_signal(|| false);
    // Whether the pull is currently past the threshold. Held so the haptic fires on the *crossing*
    // rather than on every move event past it — a buzz per frame is what "too much haptics" is.
    let mut armed = use_signal(|| false);

    let expense_id = props.expense_id;
    let can_edit = props.can_edit;
    let can_delete = props.can_delete;

    // Release everything the gesture holds and let the row glide home.
    let mut release = move || {
        drag.set(None);
        settling.set(true);
        pull.set(0.0);
        armed.set(false);
    };

    // Called on every accepted horizontal move. The haptic marks the moment the release would
    // start doing something, which is the only feedback a row swipe can give before it commits.
    let mut track_arm = move |dx: f64| {
        let now = swipe_action(dx, can_edit, can_delete).is_some();
        if now != armed() {
            if now {
                haptic(Haptic::Medium);
            }
            armed.set(now);
        }
    };

    let offset = pull();
    let transition = if settling() { "transform 180ms ease-out" } else { "none" };
    let edit_opacity = affordance_opacity(offset);
    let delete_opacity = affordance_opacity(-offset);

    rsx! {
        li {
            id: "expense-row-{expense_id}",
            // Elevation on the clipping wrapper: `overflow-hidden` would cut it off the
            // foreground, and the foreground is what slides.
            class: "relative overflow-hidden rounded-box shadow-soft",
            if affordances_mounted(drag().is_some(), settling()) {
                div {
                    id: "swipe-edit-{expense_id}",
                    class: "absolute inset-y-0 left-0 w-20 bg-info text-info-content flex items-center justify-center",
                    style: "opacity: {edit_opacity};",
                    aria_hidden: "true",
                    PencilIcon { size: ICON_ACTION }
                }
                div {
                    id: "swipe-delete-{expense_id}",
                    class: "absolute inset-y-0 right-0 w-20 bg-error text-error-content flex items-center justify-center",
                    style: "opacity: {delete_opacity};",
                    aria_hidden: "true",
                    TrashIcon { size: ICON_ACTION }
                }
            }
            div {
                // `bg-base-100` is opaque — it is what hides the two panels at rest. `select-none`
                // stops a drag becoming a text selection; `touch-action: pan-y` is the whole
                // performance story (module docs).
                class: "relative flex items-center gap-3 p-3 bg-base-100 select-none cursor-pointer hover:bg-base-200 transition-colors",
                style: "touch-action: pan-y; transform: translateX({offset}px); transition: {transition};",
                role: "button",
                tabindex: "0",
                onpointerdown: move |e: PointerEvent| {
                    // The latch need only outlive its own gesture's synthesised click, already
                    // dispatched by the time a new one starts. Clearing here stops a swipe whose
                    // click landed on a modal from leaving the row needing two taps. Before the
                    // guards below: a gesture this row declines is still a new gesture.
                    swiped.set(false);
                    // A mouse never swipes: the web build has to behave exactly as before.
                    if e.pointer_type() != "touch" || !e.is_primary() {
                        return;
                    }
                    let p = e.client_coordinates();
                    if p.x < EDGE_GUARD {
                        return;
                    }
                    settling.set(false);
                    drag.set(
                        Some(Drag {
                            start_x: p.x,
                            start_y: p.y,
                            pointer_id: e.pointer_id(),
                            axis: Axis::Undecided,
                        }),
                    );
                },
                onpointermove: move |e: PointerEvent| {
                    let Some(d) = drag() else { return };
                    if e.pointer_id() != d.pointer_id {
                        return;
                    }
                    let p = e.client_coordinates();
                    let (dx, dy) = (p.x - d.start_x, p.y - d.start_y);
                    match d.axis {
                        Axis::Horizontal => {
                            pull.set(clamp_pull(dx, can_edit, can_delete));
                            track_arm(dx);
                        }
                        Axis::Undecided => {
                            match decide_axis(dx, dy) {
                                Axis::Undecided => {}
                                Axis::Horizontal => {
                                    drag.set(Some(Drag { axis: Axis::Horizontal, ..d }));
                                    pull.set(clamp_pull(dx, can_edit, can_delete));
                                    track_arm(dx);
                                }
                                Axis::Abandoned => drag.set(None),
                            }
                        }
                        Axis::Abandoned => {}
                    }
                },
                onpointerup: move |e: PointerEvent| {
                    let Some(d) = drag() else { return };
                    if e.pointer_id() != d.pointer_id {
                        return;
                    }
                    let dx = e.client_coordinates().x - d.start_x;
                    release();
                    if dx.abs() >= AXIS_SLOP {
                        swiped.set(true);
                    }
                    match swipe_action(dx, can_edit, can_delete) {
                        Some(SwipeAction::Edit) => props.on_edit.call(()),
                        Some(SwipeAction::Delete) => props.on_delete.call(()),
                        None => {}
                    }
                },
                // The UA claimed it for scrolling and sends nothing further for this pointer —
                // the only chance to put the row back.
                onpointercancel: move |_| release(),
                onclick: move |_| {
                    if swiped() {
                        swiped.set(false);
                        return;
                    }
                    props.on_open.call(());
                },
                onkeydown: move |e| {
                    let key = e.key();
                    if key == Key::Enter || key == Key::Character(" ".to_string()) {
                        e.prevent_default();
                        props.on_open.call(());
                    }
                },
                Avatar {
                    initials: props.emoji.clone(),
                    color_class: "bg-transparent".to_string(),
                    size: SizeClass::W10,
                }
                div { class: "flex-1 min-w-0",
                    p { class: "font-medium flex items-center gap-1.5 min-w-0",
                        span { class: "truncate", "{props.name}" }
                        if props.recurring {
                            span { class: "shrink-0 text-primary", title: tid!("occurrence-recurring"), aria_label: tid!("occurrence-recurring"), RepeatIcon { size: 14 } }
                        }
                    }
                    p { class: "text-xs text-base-content/70", "{props.subtitle}" }
                }
                div { class: "shrink-0 flex items-center gap-2",
                    if props.inconsistent {
                        span {
                            class: "badge badge-warning badge-sm",
                            title: tid!("expense-inconsistent-amounts"),
                            aria_label: tid!("expense-inconsistent-amounts"),
                            "⚠"
                        }
                    }
                    // A flex *item*, so `items-center` above centres the badge against the whole
                    // of it — one line or two.
                    div { class: "flex flex-col items-end leading-tight",
                        if props.estimate {
                            span { class: "badge badge-warning badge-soft badge-xs mb-0.5", {tid!("estimate-badge")} }
                        }
                        p { class: "text-sm font-semibold", "{props.amount:.2} {props.currency}" }
                        if let Some((source_amount, source_currency)) = props.source.clone() {
                            p { class: "text-xs text-base-content/60", "{source_amount:.2} {source_currency}" }
                        }
                        if let Some(debt) = props.my_debt {
                            // Bare number — the label lives in the a11y name, not on screen.
                            p {
                                class: "text-xs text-success/70",
                                title: tid!("expense-your-share"),
                                aria_label: tid!("expense-your-share-value", amount: format!("{debt:.2}"), currency: props.currency.clone()),
                                "{debt:.2} {props.currency}"
                            }
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::test_dom::{click, listener_ids, pointer, texts};
    use dioxus::dioxus_core::ElementId;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn a_move_inside_the_slop_decides_nothing() {
        assert_eq!(decide_axis(4.0, 4.0), Axis::Undecided);
        assert_eq!(decide_axis(-9.0, 0.0), Axis::Undecided);
    }

    #[test]
    fn a_dominantly_horizontal_move_is_ours() {
        assert_eq!(decide_axis(30.0, 5.0), Axis::Horizontal);
        assert_eq!(decide_axis(-30.0, 5.0), Axis::Horizontal);
    }

    #[test]
    fn a_dominantly_vertical_move_belongs_to_the_scroller() {
        assert_eq!(decide_axis(5.0, 30.0), Axis::Abandoned);
        assert_eq!(decide_axis(-5.0, -30.0), Axis::Abandoned);
    }

    /// A 45° drag is ambiguous and the scroller is the safer owner: a lost swipe costs a retry, a
    /// stolen scroll makes the list feel broken.
    #[test]
    fn a_perfect_diagonal_goes_to_the_scroller() {
        assert_eq!(decide_axis(30.0, 30.0), Axis::Abandoned);
    }

    #[test]
    fn the_pull_is_capped_in_both_directions() {
        assert_eq!(clamp_pull(500.0, true, true), MAX_PULL);
        assert_eq!(clamp_pull(-500.0, true, true), -MAX_PULL);
        assert_eq!(clamp_pull(40.0, true, true), 40.0);
    }

    #[test]
    fn a_forbidden_direction_does_not_move() {
        assert_eq!(clamp_pull(80.0, false, true), 0.0);
        assert_eq!(clamp_pull(-80.0, true, false), 0.0);
        // The other direction is unaffected.
        assert_eq!(clamp_pull(-80.0, false, true), -80.0);
        assert_eq!(clamp_pull(80.0, true, false), 80.0);
    }

    #[test]
    fn a_short_swipe_commits_nothing() {
        assert_eq!(swipe_action(SWIPE_THRESHOLD - 1.0, true, true), None);
        assert_eq!(swipe_action(-SWIPE_THRESHOLD + 1.0, true, true), None);
        assert_eq!(swipe_action(0.0, true, true), None);
    }

    #[test]
    fn right_edits_and_left_deletes() {
        assert_eq!(swipe_action(SWIPE_THRESHOLD, true, true), Some(SwipeAction::Edit));
        assert_eq!(swipe_action(-SWIPE_THRESHOLD, true, true), Some(SwipeAction::Delete));
        assert_eq!(swipe_action(300.0, true, true), Some(SwipeAction::Edit));
        assert_eq!(swipe_action(-300.0, true, true), Some(SwipeAction::Delete));
    }

    #[test]
    fn a_forbidden_action_never_commits() {
        assert_eq!(swipe_action(300.0, false, true), None);
        assert_eq!(swipe_action(-300.0, true, false), None);
    }

    /// The clamp must never sit below the commit distance, or a full-strength swipe never fires.
    ///
    /// `const` assertions rather than a `#[test]`: all three are compile-time constants, so a
    /// runtime `assert!` on them is a tautology the optimiser deletes — the check would pass
    /// whatever the values were. These fail the build instead.
    const _: () = assert!(MAX_PULL >= SWIPE_THRESHOLD);
    const _: () = assert!(SWIPE_THRESHOLD > AXIS_SLOP);

    #[test]
    fn the_panel_fades_in_over_the_travel_and_stops_at_full() {
        assert_eq!(affordance_opacity(0.0), 0.0);
        assert_eq!(affordance_opacity(SWIPE_THRESHOLD / 2.0), 0.5);
        assert_eq!(affordance_opacity(SWIPE_THRESHOLD), 1.0);
        assert_eq!(affordance_opacity(MAX_PULL), 1.0);
        // The opposite direction keeps this panel hidden.
        assert_eq!(affordance_opacity(-50.0), 0.0);
    }

    #[test]
    fn a_row_at_rest_carries_no_swipe_panels() {
        assert!(!affordances_mounted(false, false), "half the row's DOM, invisible at rest");
    }

    #[test]
    fn the_panels_are_mounted_for_the_whole_gesture_including_the_glide_home() {
        assert!(affordances_mounted(true, false), "a live drag");
        assert!(affordances_mounted(false, true), "still settling — the row is mid-glide");
        assert!(affordances_mounted(true, true));
    }

    // The pure functions say what a `dx` means; these say the handlers wire the four pointer
    // events to them correctly — where the bugs are (missed `pointer_id` guard, a `pointercancel`
    // that forgets to drop the drag).

    /// Everything the row reported, in order — `Open` included, since "a swipe must not also
    /// navigate" is the likeliest regression.
    #[derive(Debug, PartialEq, Clone, Copy)]
    enum Fired {
        Open,
        Edit,
        Delete,
    }

    type Log = Rc<RefCell<Vec<Fired>>>;

    #[derive(Props, Clone, PartialEq)]
    struct HarnessProps {
        log: Log,
        can_edit: bool,
        can_delete: bool,
        my_debt: Option<f64>,
    }

    #[component]
    fn Harness(props: HarnessProps) -> Element {
        // The row translates its labels, and `tid!` panics with no provider above it.
        crate::i18n::use_test_i18n();
        let (open, edit, del) = (props.log.clone(), props.log.clone(), props.log.clone());
        rsx! {
            ExpenseRow {
                expense_id: 1,
                name: "Pizza".to_string(),
                subtitle: "Dépense payée par Alice".to_string(),
                emoji: "🍕".to_string(),
                amount: 24.0,
                currency: "EUR".to_string(),
                inconsistent: false,
                my_debt: props.my_debt,
                can_edit: props.can_edit,
                can_delete: props.can_delete,
                on_open: move |_| open.borrow_mut().push(Fired::Open),
                on_edit: move |_| edit.borrow_mut().push(Fired::Edit),
                on_delete: move |_| del.borrow_mut().push(Fired::Delete),
            }
        }
    }

    /// The foreground div carries all six handlers, so every listener list is that one element.
    struct Row {
        dom: VirtualDom,
        el: ElementId,
        log: Log,
    }

    impl Row {
        fn new(can_edit: bool, can_delete: bool) -> Self {
            let log: Log = Rc::new(RefCell::new(Vec::new()));
            let mut dom = VirtualDom::new_with_props(
                Harness,
                HarnessProps { log: log.clone(), can_edit, can_delete, my_debt: None },
            );
            let m = dom.rebuild_to_vec();
            let ids = listener_ids(&m, "pointerdown");
            assert_eq!(ids.len(), 1, "one swipe surface per row: {ids:?}");
            Self { dom, el: ids[0], log }
        }

        fn send(&mut self, name: &str, x: f64, y: f64) {
            self.dom.runtime().handle_event(name, pointer(x, y, "touch", 7), self.el);
        }

        fn tap(&mut self) {
            self.dom.runtime().handle_event("click", click(), self.el);
        }

        fn fired(&self) -> Vec<Fired> {
            self.log.borrow().clone()
        }
    }

    #[test]
    fn a_full_left_swipe_asks_to_delete() {
        let mut row = Row::new(true, true);
        row.send("pointerdown", 200.0, 100.0);
        row.send("pointermove", 160.0, 100.0);
        row.send("pointermove", 100.0, 102.0);
        row.send("pointerup", 100.0, 102.0);
        assert_eq!(row.fired(), vec![Fired::Delete]);
    }

    #[test]
    fn a_full_right_swipe_asks_to_edit() {
        let mut row = Row::new(true, true);
        row.send("pointerdown", 100.0, 100.0);
        row.send("pointermove", 140.0, 100.0);
        row.send("pointermove", 200.0, 98.0);
        row.send("pointerup", 200.0, 98.0);
        assert_eq!(row.fired(), vec![Fired::Edit]);
    }

    /// The click the browser synthesises at the end of a swipe must not also open the expense.
    #[test]
    fn a_swipe_does_not_also_navigate() {
        let mut row = Row::new(true, true);
        row.send("pointerdown", 200.0, 100.0);
        row.send("pointermove", 100.0, 100.0);
        row.send("pointerup", 100.0, 100.0);
        row.tap();
        assert_eq!(row.fired(), vec![Fired::Delete], "the trailing click must be swallowed");
        // ...and only the one click. The next tap is a real tap again.
        row.tap();
        assert_eq!(row.fired(), vec![Fired::Delete, Fired::Open]);
    }

    /// The real shape of a swipe-delete: the confirm modal opens on top, so the trailing click
    /// lands on *it*. The latch must not survive past its own gesture, or the next real tap is
    /// swallowed too.
    #[test]
    fn a_swipe_whose_click_landed_elsewhere_leaves_the_row_tappable() {
        let mut row = Row::new(true, true);
        row.send("pointerdown", 200.0, 100.0);
        row.send("pointermove", 100.0, 100.0);
        row.send("pointerup", 100.0, 100.0);
        assert_eq!(row.fired(), vec![Fired::Delete]);

        // No `row.tap()` here — the user cancelled the modal, which ate the synthesised click.
        row.send("pointerdown", 200.0, 100.0);
        row.send("pointerup", 200.0, 100.0);
        row.tap();
        assert_eq!(row.fired(), vec![Fired::Delete, Fired::Open], "one tap must be enough");
    }

    #[test]
    fn a_plain_tap_still_opens_the_expense() {
        let mut row = Row::new(true, true);
        row.send("pointerdown", 200.0, 100.0);
        row.send("pointerup", 201.0, 100.0);
        row.tap();
        assert_eq!(row.fired(), vec![Fired::Open]);
    }

    /// The scroll case: the UA claims the gesture, sends `pointercancel`, then nothing further.
    #[test]
    fn a_vertical_drag_scrolls_and_commits_nothing() {
        let mut row = Row::new(true, true);
        row.send("pointerdown", 200.0, 300.0);
        row.send("pointermove", 202.0, 240.0);
        row.send("pointercancel", 202.0, 240.0);
        assert_eq!(row.fired(), vec![]);
    }

    /// Once vertical has won, a later horizontal wander must not resurrect the swipe — the
    /// diagonal flick down a long list.
    #[test]
    fn a_gesture_lost_to_the_scroller_never_comes_back() {
        let mut row = Row::new(true, true);
        row.send("pointerdown", 200.0, 300.0);
        row.send("pointermove", 205.0, 250.0);
        row.send("pointermove", 60.0, 250.0);
        row.send("pointerup", 60.0, 250.0);
        assert_eq!(row.fired(), vec![]);
    }

    #[test]
    fn a_short_swipe_commits_nothing_and_still_opens() {
        let mut row = Row::new(true, true);
        row.send("pointerdown", 200.0, 100.0);
        row.send("pointermove", 170.0, 100.0);
        row.send("pointerup", 170.0, 100.0);
        assert_eq!(row.fired(), vec![]);
        // It moved past the tap slop, so the trailing click is still suppressed.
        row.tap();
        assert_eq!(row.fired(), vec![]);
    }

    /// A left-edge gesture is the OS back-swipe's. Never claim it.
    #[test]
    fn a_swipe_from_the_left_edge_is_left_alone() {
        let mut row = Row::new(true, true);
        row.send("pointerdown", EDGE_GUARD - 1.0, 100.0);
        row.send("pointermove", 200.0, 100.0);
        row.send("pointerup", 200.0, 100.0);
        assert_eq!(row.fired(), vec![]);
    }

    /// A mouse is not a finger: the web build must be untouched by this feature.
    #[test]
    fn a_mouse_drag_is_ignored() {
        let mut row = Row::new(true, true);
        row.dom.runtime().handle_event("pointerdown", pointer(200.0, 100.0, "mouse", 1), row.el);
        row.dom.runtime().handle_event("pointermove", pointer(100.0, 100.0, "mouse", 1), row.el);
        row.dom.runtime().handle_event("pointerup", pointer(100.0, 100.0, "mouse", 1), row.el);
        assert_eq!(row.fired(), vec![]);
        row.tap();
        assert_eq!(row.fired(), vec![Fired::Open], "a mouse click still opens the expense");
    }

    /// A second finger landing mid-swipe must not hijack the drag it did not start.
    #[test]
    fn a_second_pointer_cannot_steal_the_gesture() {
        let mut row = Row::new(true, true);
        row.send("pointerdown", 200.0, 100.0);
        // Pointer 9 is not the one that went down; its events belong to nobody here.
        row.dom.runtime().handle_event("pointermove", pointer(20.0, 100.0, "touch", 9), row.el);
        row.dom.runtime().handle_event("pointerup", pointer(20.0, 100.0, "touch", 9), row.el);
        assert_eq!(row.fired(), vec![]);
        // The original finger is still in charge.
        row.send("pointermove", 100.0, 100.0);
        row.send("pointerup", 100.0, 100.0);
        assert_eq!(row.fired(), vec![Fired::Delete]);
    }

    #[test]
    fn a_closed_project_swipes_neither_way_when_both_are_forbidden() {
        let mut row = Row::new(false, false);
        row.send("pointerdown", 200.0, 100.0);
        row.send("pointermove", 60.0, 100.0);
        row.send("pointerup", 60.0, 100.0);
        assert_eq!(row.fired(), vec![]);

        let mut row = Row::new(false, false);
        row.send("pointerdown", 100.0, 100.0);
        row.send("pointermove", 260.0, 100.0);
        row.send("pointerup", 260.0, 100.0);
        assert_eq!(row.fired(), vec![]);
    }

    /// The two directions are gated independently: a closed project still deletes, archived not.
    #[test]
    fn forbidding_edit_leaves_delete_working() {
        let mut row = Row::new(false, true);
        row.send("pointerdown", 200.0, 100.0);
        row.send("pointermove", 60.0, 100.0);
        row.send("pointerup", 60.0, 100.0);
        assert_eq!(row.fired(), vec![Fired::Delete]);
    }

    fn money_texts(my_debt: Option<f64>) -> Vec<String> {
        let mut dom = VirtualDom::new_with_props(
            Harness,
            HarnessProps {
                log: Rc::new(RefCell::new(Vec::new())),
                can_edit: true,
                can_delete: true,
                my_debt,
            },
        );
        // Creation order is not document order for the dynamic `if`, so assert on the set.
        texts(&dom.rebuild_to_vec()).into_iter().filter(|t| t.contains("EUR")).collect()
    }

    #[test]
    fn without_a_share_the_row_shows_one_amount() {
        assert_eq!(money_texts(None), vec!["24.00 EUR"]);
    }

    #[test]
    fn a_share_is_rendered_next_to_the_total() {
        let money = money_texts(Some(8.0));
        assert_eq!(money.len(), 2, "{money:?}");
        assert!(money.contains(&"24.00 EUR".to_string()), "{money:?}");
        assert!(money.contains(&"8.00 EUR".to_string()), "{money:?}");
    }
}
