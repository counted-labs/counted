//! One expense row: swipe left to delete, right to edit; a long press opens both as a sheet.
//!
//! **One gesture, two implementations, by target.** On Android and iOS every event dispatched to
//! Rust costs a *synchronous* XHR to wry's custom protocol (`dioxus-interpreter-js`,
//! `handleVirtualdomEventSync` → `xhr.open(..., false)`), so a Dioxus `onpointermove` was a bridge
//! call and a re-render per frame of a swipe. There the row carries no pointer listener at all:
//! [`ROW_SWIPE_JS`], delegated on `document`, moves the foreground and the panels, and a committed
//! swipe clicks the row's hidden `.swipe-commit-*` button - the one event that reaches Rust. It
//! swipes as a UITableView cell does: a partial swipe holds the row open on its action, which a
//! tap fires; a tap elsewhere or a swipe back closes it; a swipe past half the row commits at once.
//! On the web an event is a wasm call, and the pointer handlers below stay in Rust with the same
//! constants (and their tests), committing past [`SWIPE_THRESHOLD`] with no hold: a finger in a
//! phone browser reports `pointerType` "touch" and swipes, a mouse never enters the state machine.
//! Two targets, never two renders of one page, so the split cannot break hydration.
//!
//! **The long press is boot.js's**, on every target: the foreground's `data-long-press` names the
//! hidden button it clicks after 500ms. It is also the screen reader's way to the swipe-only
//! actions, through the row's visually hidden actions button ([`RowActionsEntry`]).
//!
//! **`touch-action: pan-y` on the foreground**, on both: a vertical drag is claimed by the UA at
//! once with `pointercancel`, so a scroll never feeds the gesture.
//!
//! On the web the drag offset is a *per-row* signal - one on the list would re-render every row per
//! pointermove. Touch events cannot replace the pointer ones there: `TouchPoint::identifier()` is
//! not `pub` in dioxus-html 0.7.9, so a touch cannot be tracked across events.

#![cfg_attr(any(target_os = "android", target_os = "ios"), allow(dead_code))]

use dioxus::prelude::*;
use crate::tid;

use crate::common::{haptic, Haptic, RowActionsEntry, RowActionsSheet};
use crate::expenses::expense_search::Marked;
use crate::expenses::project_actions_sheet::Row;
use crate::icons::{PencilIcon, RepeatIcon, TrashIcon, ICON_ACTION, ICON_HEADER};

const EDIT_PANEL_CLASS: &str = "absolute inset-y-0 left-0 w-20 bg-info text-info-content flex items-center justify-center";
const DELETE_PANEL_CLASS: &str = "absolute inset-y-0 right-0 w-20 bg-error text-error-content flex items-center justify-center";

/// How far the finger must travel before a release commits the action.
const SWIPE_THRESHOLD: f64 = 72.0;
/// Movement before the gesture is claimed for an axis. Below it, this is still a tap.
const AXIS_SLOP: f64 = 10.0;
/// Gestures this close to the left edge belong to the iOS back-swipe strip (`page_transition.rs`).
const EDGE_GUARD: f64 = 24.0;
/// Never further than the panel behind it is wide (`w-20` = 5rem = 80px), plus overshoot so the
/// panel is fully uncovered at commit.
const MAX_PULL: f64 = 96.0;
/// Native: where a partial swipe holds the row open, the panel's `w-20`. Released past half of it,
/// the row opens; short of that it closes.
#[cfg_attr(not(any(target_os = "android", target_os = "ios")), allow(dead_code))]
const REVEAL_WIDTH: f64 = 80.0;
/// Native: the share of the row's width past which a release commits without holding open.
#[cfg_attr(not(any(target_os = "android", target_os = "ios")), allow(dead_code))]
const FULL_SWIPE: f64 = 0.5;

#[derive(Clone, Copy, PartialEq, Debug)]
enum Axis {
    /// Still within the slop - could yet become a tap, a scroll or a swipe.
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

/// Right is edit, left is delete - the sides the affordances are painted on.
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
/// fully transparent at rest - 22 000 unseen DOM nodes on a 2000-expense project. They exist only
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
    /// Always in `currency` - an expense entered in another currency was converted at write time.
    pub amount: f64,
    pub currency: String,
    /// `Some((amount, code))` when the expense was entered in another currency, shown under the
    /// total as what was actually paid. The rate itself is on the detail page, not here.
    pub source: Option<(f64, String)>,
    /// Payments no longer adding up to the total - badged, never blocked.
    pub inconsistent: bool,
    /// "Mes dettes" filter only: this user's share, shown under the total. Every other call site
    /// omits it.
    pub my_debt: Option<f64>,
    /// Typed search words, folded: their matches are marked in the name.
    #[props(default)]
    pub highlight: Vec<String>,
    /// The category label, when the search matched it and not the name.
    #[props(default)]
    pub matched_category: Option<String>,
    /// "Mes dépenses": the group a share comes from, badged ahead of the subtitle.
    #[props(default)]
    pub group: Option<String>,
    /// "Mes dépenses": under the amount, what it is a share of, or that it is not counted.
    #[props(default)]
    pub note: Option<String>,
    #[props(default)]
    pub recurring: bool,
    #[props(default)]
    pub estimate: bool,
    #[props(default)]
    pub income: bool,
    /// Added offline and not yet sent: badged, and inert until the replay gives it a server id.
    #[props(default)]
    pub pending: bool,
    pub can_edit: bool,
    pub can_delete: bool,
    /// A tap or Enter/Space - navigation is the parent's job, so this row needs no router.
    pub on_open: EventHandler<()>,
    pub on_edit: EventHandler<()>,
    pub on_delete: EventHandler<()>,
}

const FOREGROUND_CLASS: &str = "relative flex items-center gap-3 p-3 bg-base-100 select-none cursor-pointer hover:bg-base-200 row-press";

#[component]
pub fn ExpenseRow(props: ExpenseRowProps) -> Element {
    let on_open = props.on_open;
    let onkeydown = move |e: KeyboardEvent| {
        let key = e.key();
        if key == Key::Enter || key == Key::Character(" ".to_string()) {
            e.prevent_default();
            on_open.call(());
        }
    };
    let body = rsx! {
        span { class: "size-10 shrink-0 flex items-center justify-center text-2xl leading-none", aria_hidden: "true", "{props.emoji}" }
        div { class: "flex-1 min-w-0",
            p { class: "font-medium flex items-center gap-1.5 min-w-0",
                span { class: "line-clamp-2 wrap-anywhere",
                    Marked { text: props.name.clone(), words: props.highlight.clone() }
                }
                if props.recurring {
                    span { class: "shrink-0 text-primary", title: tid!("occurrence-recurring"), aria_label: tid!("occurrence-recurring"), RepeatIcon { size: 14 } }
                }
            }
            p { class: "text-xs text-base-content/70 flex items-center gap-1.5 min-w-0",
                if let Some(group) = props.group.clone() {
                    span { class: "badge badge-xs badge-soft badge-success font-semibold shrink-0 whitespace-nowrap", "{group}" }
                }
                span { class: "truncate",
                "{props.subtitle}"
                if let Some(category) = props.matched_category.clone() {
                    Marked { text: format!(" · {category}"), words: props.highlight.clone() }
                }
                }
            }
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
            // of it - one line or two.
            div { class: "flex flex-col items-end leading-tight",
                if props.estimate {
                    span { class: "badge badge-warning badge-soft badge-xs mb-0.5", {tid!("estimate-badge")} }
                }
                if props.pending {
                    span { class: "badge badge-warning badge-soft badge-xs mb-0.5", {tid!("expense-pending")} }
                }
                p { class: "text-sm font-semibold tabular-nums", if props.income { {crate::common::signed_amount(props.amount, &props.currency)} } else { {crate::common::amount(props.amount, &props.currency)} } }
                if let Some((source_amount, source_currency)) = props.source.clone() {
                    p { class: "text-xs text-base-content/60", {crate::common::amount(source_amount, &source_currency)} }
                }
                if let Some(note) = props.note.clone() {
                    p { class: "text-xs text-base-content/60", "{note}" }
                }
                if let Some(debt) = props.my_debt {
                    // Bare number - the label lives in the a11y name, not on screen.
                    p {
                        class: "text-xs text-success/70",
                        title: tid!("expense-your-share"),
                        aria_label: tid!("expense-your-share-value", amount: crate::common::number(debt), currency: props.currency.clone()),
                        {crate::common::amount(debt, &props.currency)}
                    }
                }
            }
        }
    };

    let mut actions_open = use_signal(|| false);
    let (on_edit, on_delete) = (props.on_edit, props.on_delete);
    let long_press = (props.can_edit || props.can_delete).then(|| format!("long-press-expense-{}", props.expense_id));
    let actions = rsx! {
        if let Some(id) = long_press.clone() {
            RowActionsEntry {
                long_press_id: id,
                label: tid!("member-actions", name: props.name.clone()),
                open: actions_open(),
                on_open: move |_| actions_open.set(true),
            }
        }
        if actions_open() {
            RowActionsSheet { title: props.name.clone(), on_close: move |_| actions_open.set(false),
                if props.can_edit {
                    Row {
                        id: "row-action-edit",
                        label: tid!("edit"),
                        onclick: move |_| {
                            actions_open.set(false);
                            on_edit.call(());
                        },
                        PencilIcon { size: ICON_HEADER }
                    }
                }
                if props.can_delete {
                    Row {
                        id: "row-action-delete",
                        label: tid!("delete"),
                        danger: true,
                        onclick: move |_| {
                            actions_open.set(false);
                            on_delete.call(());
                        },
                        TrashIcon { size: ICON_HEADER }
                    }
                }
            }
        }
    };
    swipe_row(&props, body, onkeydown, long_press, actions)
}

/// No pointer listener: [`ROW_SWIPE_JS`] drives the drag and clicks a hidden commit button, present
/// only for the directions the project status allows, which is also how the script reads them.
#[cfg(any(target_os = "android", target_os = "ios"))]
fn swipe_row(
    props: &ExpenseRowProps,
    body: Element,
    onkeydown: impl FnMut(KeyboardEvent) + 'static,
    long_press: Option<String>,
    actions: Element,
) -> Element {
    let (on_open, on_edit, on_delete) = (props.on_open, props.on_edit, props.on_delete);
    rsx! {
        li {
            id: "expense-row-{props.expense_id}",
            class: "relative overflow-hidden",
            if props.can_edit {
                button { r#type: "button", class: "swipe-commit-edit", hidden: true, tabindex: "-1", onclick: move |_| on_edit.call(()) }
            }
            if props.can_delete {
                button { r#type: "button", class: "swipe-commit-delete", hidden: true, tabindex: "-1", onclick: move |_| on_delete.call(()) }
            }
            div {
                class: "expense-swipe {FOREGROUND_CLASS}",
                style: "touch-action: pan-y;",
                role: "button",
                tabindex: "0",
                "data-long-press": long_press,
                onclick: move |_| on_open.call(()),
                onkeydown: onkeydown,
                {body}
            }
            {actions}
        }
    }
}

#[cfg(not(any(target_os = "android", target_os = "ios")))]
fn swipe_row(
    props: &ExpenseRowProps,
    body: Element,
    onkeydown: impl FnMut(KeyboardEvent) + 'static,
    long_press: Option<String>,
    actions: Element,
) -> Element {
    let mut drag: Signal<Option<Drag>> = use_signal(|| None);
    let mut pull = use_signal(|| 0.0f64);
    let mut settling = use_signal(|| false);
    // Read by `onclick`: the browser synthesises a click at the end of a swipe, and without this
    // the row would also navigate.
    let mut swiped = use_signal(|| false);
    // Whether the pull is currently past the threshold. Held so the haptic fires on the *crossing*
    // rather than on every move event past it - a buzz per frame is what "too much haptics" is.
    let mut armed = use_signal(|| false);

    let expense_id = props.expense_id;
    let can_edit = props.can_edit;
    let can_delete = props.can_delete;
    let (on_open, on_edit, on_delete) = (props.on_open, props.on_edit, props.on_delete);

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
            class: "relative overflow-hidden",
            if affordances_mounted(drag().is_some(), settling()) {
                div {
                    id: "swipe-edit-{expense_id}",
                    class: EDIT_PANEL_CLASS,
                    style: "opacity: {edit_opacity};",
                    aria_hidden: "true",
                    PencilIcon { size: ICON_ACTION }
                }
                div {
                    id: "swipe-delete-{expense_id}",
                    class: DELETE_PANEL_CLASS,
                    style: "opacity: {delete_opacity};",
                    aria_hidden: "true",
                    TrashIcon { size: ICON_ACTION }
                }
            }
            div {
                // `bg-base-100` is opaque - it is what hides the two panels at rest. `select-none`
                // stops a drag becoming a text selection; `touch-action: pan-y` keeps a scroll out
                // of the gesture (module docs).
                class: FOREGROUND_CLASS,
                style: "touch-action: pan-y; transform: translateX({offset}px); transition: {transition};",
                role: "button",
                tabindex: "0",
                "data-long-press": long_press,
                onpointerdown: move |e: PointerEvent| {
                    // The latch need only outlive its own gesture's synthesised click, already
                    // dispatched by the time a new one starts. Clearing here stops a swipe whose
                    // click landed on a modal from leaving the row needing two taps. Before the
                    // guards below: a gesture this row declines is still a new gesture.
                    swiped.set(false);
                    // A mouse never swipes: a desktop browser behaves exactly as before.
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
                        Some(SwipeAction::Edit) => on_edit.call(()),
                        Some(SwipeAction::Delete) => on_delete.call(()),
                        None => {}
                    }
                },
                // The UA claimed it for scrolling and sends nothing further for this pointer -
                // the only chance to put the row back.
                onpointercancel: move |_| release(),
                onclick: move |_| {
                    if swiped() {
                        swiped.set(false);
                        return;
                    }
                    on_open.call(());
                },
                onkeydown: onkeydown,
                {body}
            }
            {actions}
        }
    }
}

/// The swipe panels' one copy per list, which [`ROW_SWIPE_JS`] clones under a row for the length
/// of a gesture (a copy in every row was half of each row's DOM), and the script itself. Renders
/// nothing on the web, where the rows draw their own panels.
#[cfg(any(target_os = "android", target_os = "ios"))]
#[component]
pub fn ExpenseSwipe() -> Element {
    // Scope-bound, like `PullToRefresh`: the next list's mount rebinds the haptic hook.
    use_hook(|| {
        spawn(async move {
            let mut eval = document::eval(&format!(
                "window._countedSwipeConst = [{REVEAL_WIDTH}, {AXIS_SLOP}, {EDGE_GUARD}, {FULL_SWIPE}];\
                 window._countedSwipeArm = function () {{ dioxus.send(1); }};\
                 {ROW_SWIPE_JS}"
            ));
            while eval.recv::<i32>().await.is_ok() {
                haptic(Haptic::Medium);
            }
        });
    });
    rsx! {
        // Full width behind the row, the icon in the first 80px of its side: the row uncovers
        // more of the colour as it travels and the icon stays put, with nothing resized per frame.
        div { id: "expense-swipe-panels", hidden: true,
            div {
                class: "swipe-panel-edit {EDIT_PANEL_CLASS}",
                style: "right: 0; width: auto; justify-content: flex-start; padding-left: 30px;",
                aria_hidden: "true",
                PencilIcon { size: ICON_ACTION }
            }
            div {
                class: "swipe-panel-delete {DELETE_PANEL_CLASS}",
                style: "left: 0; width: auto; justify-content: flex-end; padding-right: 30px;",
                aria_hidden: "true",
                TrashIcon { size: ICON_ACTION }
            }
        }
    }
}

#[cfg(not(any(target_os = "android", target_os = "ios")))]
#[component]
pub fn ExpenseSwipe() -> Element {
    rsx! {}
}

/// The row swipe on Android and iOS, the `sheet_drag` way: passive pointer listeners on `document`,
/// delegated to `.expense-swipe`, so nothing is attached per row and the shared observer is not
/// needed. The dominant axis past the slop decides, a vertical one lets go, the left edge belongs to
/// the iOS back swipe, a forbidden direction (no `.swipe-commit-*` button) does not move, and the
/// pull stops at the row's width. A release past half the row clicks the commit button (crossing
/// that line calls `_countedSwipeArm`, a Medium haptic); past half the panel it holds the row open
/// at the panel's width, where a tap on the panel clicks the same button. While a row is open, a
/// press anywhere else closes it and its click is eaten, and a drag on it carries on from where it
/// rests. Clicks the browser synthesises after a swipe are swallowed in the capture phase, before
/// Dioxus's root listener sees them; script clicks (`isTrusted` false) never are. A long press
/// (boot.js, `_countedLongPressed`) ends the drag. The constants arrive in `_countedSwipeConst`.
#[cfg_attr(not(any(target_os = "android", target_os = "ios")), allow(dead_code))]
const ROW_SWIPE_JS: &str = r#"
(function () {
  if (window._countedRowSwipe) return;
  window._countedRowSwipe = true;

  var c = window._countedSwipeConst;
  var REVEAL = c[0], SLOP = c[1], EDGE = c[2], FULL = c[3];
  var g = null;
  var open = null;
  var tap = null;
  var swallow = null;

  function child(li, cls) { return li.querySelector(':scope > .' + cls); }

  function pullOf(s, raw) {
    if (!raw || !child(s.li, raw > 0 ? 'swipe-commit-edit' : 'swipe-commit-delete')) return 0;
    return Math.max(-s.w, Math.min(s.w, raw));
  }

  function full(s) { return Math.max(REVEAL * 1.5, s.w * FULL); }

  function unmount(li) {
    var e = child(li, 'swipe-panel-edit'), d = child(li, 'swipe-panel-delete');
    if (e) e.remove();
    if (d) d.remove();
  }

  function mount(li) {
    var t = document.getElementById('expense-swipe-panels');
    if (!t || child(li, 'swipe-panel-edit')) return;
    for (var i = t.children.length - 1; i >= 0; i--) li.insertBefore(t.children[i].cloneNode(true), li.firstChild);
  }

  function finishSettle(li) {
    if (!li._countedSettle) return;
    clearTimeout(li._countedSettle);
    li._countedSettle = null;
    var fg = child(li, 'expense-swipe');
    if (fg) fg.style.transition = '';
    unmount(li);
  }

  function paint(s, pull) {
    s.fg.style.transform = 'translateX(' + pull + 'px)';
    var e = child(s.li, 'swipe-panel-edit'), d = child(s.li, 'swipe-panel-delete');
    if (e) { e.style.opacity = Math.max(0, Math.min(1, pull / REVEAL)); e.style.visibility = pull > 0 ? 'visible' : 'hidden'; }
    if (d) { d.style.opacity = Math.max(0, Math.min(1, -pull / REVEAL)); d.style.visibility = pull < 0 ? 'visible' : 'hidden'; }
    var armed = Math.abs(pull) >= full(s);
    if (armed !== s.armed) {
      s.armed = armed;
      if (armed && window._countedSwipeArm) window._countedSwipeArm();
    }
  }

  function home(s) {
    if (!s.started) return;
    s.fg.style.transition = 'transform 180ms ease-out';
    s.fg.style.transform = '';
    s.li._countedSettle = setTimeout(function () { finishSettle(s.li); }, 200);
  }

  function hold(s, pull) {
    s.fg.style.transition = 'transform 180ms ease-out';
    s.armed = true;
    paint(s, pull);
    open = { fg: s.fg, li: s.li, w: s.w, pull: pull, started: true, armed: false };
  }

  function shut() {
    var o = open;
    open = null;
    if (o && o.li.isConnected) home(o);
  }

  function commit(li, pull) {
    var button = child(li, pull > 0 ? 'swipe-commit-edit' : 'swipe-commit-delete');
    if (button) button.click();
  }

  document.addEventListener('pointerdown', function (e) {
    swallow = null;
    tap = null;
    if (open && !open.li.isConnected) open = null;
    if (!(e.target instanceof Element)) return;
    if (open) {
      var panel = e.target.closest('.swipe-panel-edit, .swipe-panel-delete');
      if (panel && panel.parentElement === open.li) {
        tap = { id: e.pointerId, x: e.clientX, y: e.clientY };
        return;
      }
      if (e.pointerType === 'touch' && e.isPrimary && open.fg.contains(e.target)) {
        var o = open;
        open = null;
        o.fg.style.transition = 'none';
        g = { fg: o.fg, li: o.li, id: e.pointerId, x: e.clientX, y: e.clientY, w: o.w, base: o.pull,
              horizontal: false, started: true, armed: false };
        return;
      }
      shut();
      swallow = document;
      return;
    }
    if (e.pointerType !== 'touch' || !e.isPrimary) return;
    var fg = e.target.closest('.expense-swipe');
    if (!fg || e.clientX < EDGE) return;
    finishSettle(fg.parentElement);
    g = { fg: fg, li: fg.parentElement, id: e.pointerId, x: e.clientX, y: e.clientY, w: fg.offsetWidth, base: 0,
          horizontal: false, started: false, armed: false };
  }, { capture: true, passive: true });

  document.addEventListener('pointermove', function (e) {
    if (tap && e.pointerId === tap.id && Math.max(Math.abs(e.clientX - tap.x), Math.abs(e.clientY - tap.y)) >= SLOP) {
      tap = null;
      shut();
      return;
    }
    if (!g || e.pointerId !== g.id) return;
    var dx = e.clientX - g.x, dy = e.clientY - g.y;
    if (window._countedLongPressed) {
      var s = g;
      g = null;
      home(s);
      return;
    }
    if (!g.horizontal) {
      if (Math.max(Math.abs(dx), Math.abs(dy)) < SLOP) return;
      if (Math.abs(dx) <= Math.abs(dy)) {
        var v = g;
        g = null;
        home(v);
        return;
      }
      g.horizontal = true;
    }
    if (!g.started) {
      g.started = true;
      g.fg.style.transition = 'none';
      mount(g.li);
    }
    paint(g, pullOf(g, g.base + dx));
  }, { capture: true, passive: true });

  document.addEventListener('pointerup', function (e) {
    if (tap && e.pointerId === tap.id) {
      tap = null;
      var o = open;
      shut();
      if (o) commit(o.li, o.pull);
      swallow = document;
      return;
    }
    if (!g || e.pointerId !== g.id) return;
    var s = g;
    g = null;
    var dx = e.clientX - s.x;
    var pull = s.horizontal ? pullOf(s, s.base + dx) : 0;
    if (Math.abs(pull) >= full(s)) {
      home(s);
      commit(s.li, pull);
    } else if (Math.abs(pull) >= REVEAL / 2) {
      hold(s, pull > 0 ? REVEAL : -REVEAL);
    } else {
      home(s);
    }
    if (Math.abs(dx) >= SLOP || s.base) swallow = s.fg;
  }, { capture: true, passive: true });

  document.addEventListener('pointercancel', function (e) {
    if (tap && e.pointerId === tap.id) tap = null;
    if (!g || e.pointerId !== g.id) return;
    var s = g;
    g = null;
    home(s);
  }, { capture: true, passive: true });

  document.addEventListener('click', function (e) {
    if (!e.isTrusted) return;
    var s = swallow;
    swallow = null;
    if (!s) return;
    if (s === document || (e.target instanceof Node && s.contains(e.target))) {
      e.preventDefault();
      e.stopPropagation();
    }
  }, true);
})();
"#;

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
    /// runtime `assert!` on them is a tautology the optimiser deletes - the check would pass
    /// whatever the values were. These fail the build instead.
    const _: () = assert!(MAX_PULL >= SWIPE_THRESHOLD);
    const _: () = assert!(SWIPE_THRESHOLD > AXIS_SLOP);
    // Half the panel, the line between closing and holding open, has to be a swipe, not a tap.
    const _: () = assert!(REVEAL_WIDTH / 2.0 > AXIS_SLOP);
    const _: () = assert!(FULL_SWIPE > 0.0 && FULL_SWIPE < 1.0);

    /// boot.js finds the long press's button by the id the foreground names.
    #[test]
    fn boot_js_reads_the_long_press_attribute_the_row_renders() {
        let boot = crate::common::BOOT_JS_SOURCE;
        assert!(boot.contains("closest('[data-long-press]')"));
        assert!(boot.contains("getAttribute('data-long-press')"));
        assert!(boot.contains("window._countedLongPressed = true"));
    }

    fn row_with_actions(can_edit: bool, can_delete: bool) -> (VirtualDom, Log, ElementId, Vec<ElementId>) {
        let log: Log = Rc::new(RefCell::new(Vec::new()));
        let mut dom = VirtualDom::new_with_props(
            Harness,
            HarnessProps { log: log.clone(), can_edit, can_delete, my_debt: None },
        );
        let m = dom.rebuild_to_vec();
        let fg = listener_ids(&m, "pointerdown")[0];
        let others = listener_ids(&m, "click").into_iter().filter(|id| *id != fg).collect();
        (dom, log, fg, others)
    }

    /// The long press (and the hidden actions button) open a sheet holding exactly what the swipe
    /// allows, so the actions are reachable without a swipe.
    #[test]
    fn the_actions_sheet_holds_what_the_swipe_allows() {
        for (can_edit, can_delete) in [(true, true), (false, true), (true, false)] {
            let (_, _, _, openers) = row_with_actions(can_edit, can_delete);
            assert_eq!(openers.len(), 2, "the long press's button and the actions entry");
            for opener in openers {
                let (mut dom, _, _, _) = row_with_actions(can_edit, can_delete);
                dom.runtime().handle_event("click", click(), opener);
                let shown = texts(&dom.render_immediate_to_vec());
                assert_eq!(shown.iter().any(|t| t == "Edit"), can_edit, "{shown:?}");
                assert_eq!(shown.iter().any(|t| t == "Delete"), can_delete, "{shown:?}");
            }
        }
    }

    #[test]
    fn a_row_with_nothing_allowed_has_no_actions_entry() {
        let (_, _, _, openers) = row_with_actions(false, false);
        assert!(openers.is_empty(), "{openers:?}");
    }

    /// Each of the sheet's targets, pressed on a fresh row: edit and delete run once each, the close
    /// button and the backdrop run nothing, and nothing navigates.
    #[test]
    fn each_action_in_the_sheet_runs_its_handler() {
        let open_sheet = || {
            let (mut dom, log, _, openers) = row_with_actions(true, true);
            dom.runtime().handle_event("click", click(), openers[0]);
            let targets = listener_ids(&dom.render_immediate_to_vec(), "click");
            (dom, log, targets)
        };
        let (_, _, targets) = open_sheet();
        assert_eq!(targets.len(), 4, "close, edit, delete, backdrop");
        let mut fired = Vec::new();
        for i in 0..targets.len() {
            let (mut dom, log, targets) = open_sheet();
            dom.runtime().handle_event("click", click(), targets[i]);
            dom.render_immediate_to_vec();
            fired.extend(log.borrow().iter().copied());
        }
        fired.sort_by_key(|f| *f as u8);
        assert_eq!(fired, vec![Fired::Edit, Fired::Delete]);
    }

    #[test]
    fn the_panel_fades_in_over_the_travel_and_stops_at_full() {
        assert_eq!(affordance_opacity(0.0), 0.0);
        assert_eq!(affordance_opacity(SWIPE_THRESHOLD / 2.0), 0.5);
        assert_eq!(affordance_opacity(SWIPE_THRESHOLD), 1.0);
        assert_eq!(affordance_opacity(MAX_PULL), 1.0);
        // The opposite direction keeps this panel hidden.
        assert_eq!(affordance_opacity(-50.0), 0.0);
    }

    /// The native script finds rows, panels and commit buttons by these names, and nothing on
    /// Android or iOS would notice a rename: no Rust handler is left there.
    #[test]
    fn the_native_script_uses_the_names_the_markup_renders() {
        for name in [
            "'.expense-swipe'",
            "'swipe-commit-edit'",
            "'swipe-commit-delete'",
            "'expense-swipe-panels'",
            "'swipe-panel-edit'",
            "'swipe-panel-delete'",
            "window._countedSwipeConst",
            "window._countedSwipeArm",
            "window._countedLongPressed",
        ] {
            assert!(ROW_SWIPE_JS.contains(name), "{name} missing from ROW_SWIPE_JS");
        }
    }

    #[test]
    fn a_row_at_rest_carries_no_swipe_panels() {
        assert!(!affordances_mounted(false, false), "half the row's DOM, invisible at rest");
    }

    #[test]
    fn the_panels_are_mounted_for_the_whole_gesture_including_the_glide_home() {
        assert!(affordances_mounted(true, false), "a live drag");
        assert!(affordances_mounted(false, true), "still settling - the row is mid-glide");
        assert!(affordances_mounted(true, true));
    }

    // The pure functions say what a `dx` means; these say the handlers wire the four pointer
    // events to them correctly - where the bugs are (missed `pointer_id` guard, a `pointercancel`
    // that forgets to drop the drag).

    /// Everything the row reported, in order - `Open` included, since "a swipe must not also
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

        // No `row.tap()` here - the user cancelled the modal, which ate the synthesised click.
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

    /// Once vertical has won, a later horizontal wander must not resurrect the swipe - the
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
        texts(&dom.rebuild_to_vec()).into_iter().filter(|t| t.contains("€")).collect()
    }

    #[test]
    fn without_a_share_the_row_shows_one_amount() {
        assert_eq!(money_texts(None), vec!["€24.00"]);
    }

    #[test]
    fn a_share_is_rendered_next_to_the_total() {
        let money = money_texts(Some(8.0));
        assert_eq!(money.len(), 2, "{money:?}");
        assert!(money.contains(&"€24.00".to_string()), "{money:?}");
        assert!(money.contains(&"€8.00".to_string()), "{money:?}");
    }
}
