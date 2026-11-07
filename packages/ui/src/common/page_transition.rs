use dioxus::prelude::*;

use crate::route::Route;

/// Which way a navigation moves through the app's hierarchy. Drives the CSS in
/// `packages/mobile/assets/main.css`, per `html[data-plat]` (set by the mobile head script); see
/// docs/mobile-navigation.md.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NavDirection {
    Forward,
    Back,
    Lateral,
}

impl NavDirection {
    pub fn as_str(self) -> &'static str {
        match self {
            NavDirection::Forward => "fwd",
            NavDirection::Back => "back",
            NavDirection::Lateral => "lateral",
        }
    }
}

/// How deep a route sits in the app's hierarchy.
///
/// Direction is inferred from this rather than signalled at each navigator call site: the back
/// arrow, the hardware back button, every `push` and the dock's `replace` tabs then all agree
/// without a single one of them knowing about animations. The match is exhaustive on purpose - a
/// new route must not silently default to a depth.
pub fn route_depth(route: &Route) -> u8 {
    match route {
        Route::ProjectsPage {}
        | Route::ChartsPage {}
        | Route::SettingsPage {}
        | Route::LoginPage {}
        | Route::VerifyEmailPage { .. }
        | Route::ResetPage { .. }
        | Route::DemoPage { .. }
        | Route::NotFoundPage { .. } => 0,
        Route::ExpensesPage { .. }
        | Route::FriendsPage {}
        | Route::RegisterPage {}
        | Route::ForgotPasswordPage {}
        | Route::HelpPage {}
        | Route::PrivacyPage {}
        | Route::TermsPage {}
        | Route::LegalNoticePage {} => 1,
        Route::PaymentPage { .. }
        | Route::ProjectHistoryPage { .. }
        | Route::RecurringPage { .. }
        | Route::RecurringRulePage { .. }
        | Route::LicensesPage {} => 2,
    }
}

/// Equal depth is `Lateral`, not "no animation": those are the dock's top-level destinations, which
/// Material moves with a fade-through and iOS does not move at all.
pub fn nav_direction(prev: &Route, next: &Route) -> NavDirection {
    match route_depth(next).cmp(&route_depth(prev)) {
        std::cmp::Ordering::Greater => NavDirection::Forward,
        std::cmp::Ordering::Less => NavDirection::Back,
        std::cmp::Ordering::Equal => NavDirection::Lateral,
    }
}

/// The animated shell every routed page renders inside, on Android and iOS.
///
/// Two things make it work, neither obvious:
///
/// **It is keyed by the route.** A CSS animation only runs when its element is created, and the
/// shell is the only node whose creation lines up exactly with a navigation - `.app-container`
/// cannot carry the animation because `ExpensesPage` swaps between two different `.app-container`
/// roots (key-missing vs. loaded) and would replay its entrance mid-view. A changed key on a
/// one-item fragment makes `diff_keyed_middle` remove the old node and create the new one, which is
/// both the animation trigger and the removal the exit observer below listens for.
///
/// **It carries no `position` and no `will-change`.** The transform during the animation makes it a
/// containing block for `position: fixed` descendants - the `SpeedDialFab`, the dropdown scrims -
/// which is what makes them slide with the page. Anything that made that permanent would re-anchor
/// them to the shell forever. It is full-height and opaque (`main.css`) for the same descendants:
/// their `bottom:` resolves from the shell's edge while it is transformed, and the incoming shell
/// has to cover the dimmed exit overlay beneath it.
#[cfg(any(target_os = "android", target_os = "ios"))]
#[component]
pub fn PageTransition() -> Element {
    let route = use_route::<Route>();

    // Not a signal: writing one during render would loop, and `use_route` already re-runs this body
    // on every navigation. `None` is the first mount - a cold start or a deep link opened cold must
    // not animate under the splash, and "none" matches no CSS rule.
    let previous: std::rc::Rc<std::cell::RefCell<Option<Route>>> =
        use_hook(|| std::rc::Rc::new(std::cell::RefCell::new(None)));

    let dir = {
        let previous = previous.borrow();
        match previous.as_ref() {
            None => "none",
            Some(prev) => nav_direction(prev, &route).as_str(),
        }
    };
    *previous.borrow_mut() = Some(route.clone());

    use_effect(|| {
        document::eval(EXIT_ANIMATION_JS);
    });

    rsx! {
        for k in [route.to_string()] {
            div {
                key: "{k}",
                class: "page-shell w-full flex flex-col items-center",
                "data-nav": dir,
                "data-route": "{k}",
                Outlet::<Route> {}
            }
        }
    }
}

/// Dioxus has no exit lifecycle: by the time any Rust code runs, the outgoing page's DOM is already
/// gone. A push/pop needs it on screen for the length of the animation, so it is caught here as a
/// `removedNodes` entry and put back as a throwaway overlay.
///
/// Three details are load-bearing:
///
/// - **`data-dioxus-id` is stripped from the whole subtree.** The interpreter's `getTargetId` climbs
///   the DOM looking for that attribute, and Dioxus recycles ids it has freed - a stale one left in
///   this tree could route an event to the wrong scope. `inert` and `pointer-events: none` are the
///   other two layers.
/// - **DOM order, never `z-index`, decides who paints on top.** New page over old on push, old over
///   new on pop, exactly like UIKit. A `z-index` on the shell would make it a permanent stacking
///   context and trap the in-page FAB (`z-40`) and every modal under the dock (`z-50`).
/// - **`scrollTop` is restored.** Detaching a node destroys its layout box and zeroes it, so a page
///   left mid-list would visibly snap to the top as it slides away. The live page gets its own
///   back too: each `.app-container` offset is kept per `data-route` and `data-scroll-key` (the
///   project page's tab), and put back in this same microtask, before paint, when a page comes
///   back or a tab is switched; a push starts at the top.
/// - **An overlay's own removal is ignored.** It still carries `page-shell`, so without the
///   `page-exiting` guard the observer adopts it as a fresh outgoing page: re-inserting it restarts
///   the animation and schedules another `animationend` and another 600ms timer, each of which
///   removes it and is adopted again. The pending timers then double every 600ms and pin the
///   WebView's JS thread within seconds - and since Dioxus applies its mutations from that same
///   thread, the Rust side keeps rendering into a DOM that never updates.
///
/// **Android predictive back** (native-feel P4.2): a shell the system back gesture shrank
/// (`data-back-progress`, written by `_countedBackProgress` in the mobile head script from
/// `MainActivity`'s progress) leaves as `data-nav="predictive"`, a fade that keeps its inline scale,
/// rather than the shared-axis slide, whose keyframes would snap it back to full size first.
///
/// **The iOS swipe back is interactive** (native-feel P4.1). WebKit's own swipe
/// (`allowsBackForwardNavigationGestures`) stays off: it slid a snapshot of the current page over a
/// same-URL history entry, then the CSS pop played on top. Instead:
///
/// - **A push keeps the parent.** The outgoing page of an iOS push is not thrown away when its
///   animation ends: it is parked (`page-parent`, hidden, its element `id`s stripped so nothing
///   resolves into it, SVG gradients excepted) in a stack that follows history - a push adds one, a
///   back drops the top, a tab switch or Reduce Motion drops all, and it holds two at most (the
///   app is two levels deep).
/// - **A 24px strip on the left edge** (`.swipe-edge`, `touch-action: none`, its own listeners, the
///   `touchmove` one non-passive) is shown only while a back is armed (`_routeArmed`) and no sheet is
///   open; `navigation_sync.rs`'s `_syncBack` calls `_countedEdgeSync` on every change. A drag that
///   starts mostly horizontal moves the live shell with the finger (it then holds its fixed
///   descendants, like the CSS pop) over the top parked page (`page-peek`) at UIKit's pop geometry:
///   the parent from -25% to 0, its dim from 0.1 to 0. A tap on the strip is handed to what lies
///   under it.
/// - **Past half the width or on a rightward flick (> 0.5 px/ms) it commits**: the slide finishes
///   in JS, then `_countedSwipeBack` is stamped and `history.back()` takes the same path as Android
///   back (`navigation_sync.rs`'s popstate, `back` to the router). The shell that navigation
///   produces gets `data-nav="none"` here, before its first style pass, and no overlay; the parked
///   copy, which sits over it, fades out in 150ms, so a stale copy hands over to the fresh page
///   without a jump. A stamp older than a second is ignored. Otherwise it springs back.
/// - Reduce Motion: no drag, a committed swipe just goes back.
///
/// The mutations arrive inside `requestAnimationFrame` (the interpreter's `rafEdits`), and this
/// callback is a microtask after that flush, so the overlay lands in the same frame - no flash. It
/// rides the one shared observer (`window._countedObserve`, the mobile head script), which watches
/// the whole document, so it keeps only the records whose target is `main`.
#[cfg(any(target_os = "android", target_os = "ios"))]
const EXIT_ANIMATION_JS: &str = r#"
if (!window._countedExitAnimRegistered) {
  window._countedExitAnimRegistered = true;
  requestAnimationFrame(function () {
    var host = document.querySelector('main');
    if (!host) { window._countedExitAnimRegistered = false; return; }

    var plat = document.documentElement.dataset.plat || 'android';
    var lastScroll = null;
    var offsets = {};
    var shown = null;
    var parents = [];
    function scrollKey(el) {
      var shell = el.closest('.page-shell');
      return shell ? shell.getAttribute('data-route') + '#' + (el.getAttribute('data-scroll-key') || '') : null;
    }
    document.addEventListener('scroll', function (e) {
      if (!(e.target instanceof Element)) return;
      lastScroll = { el: e.target, top: e.target.scrollTop };
      if (e.target.classList.contains('app-container') && !e.target.closest('.page-exiting')) {
        offsets[scrollKey(e.target)] = e.target.scrollTop;
      }
    }, { capture: true, passive: true });

    function liveShell() {
      return host.querySelector(':scope > .page-shell:not(.page-exiting)');
    }

    function restoreScroll() {
      var live = liveShell();
      var scroller = live && live.querySelector('.app-container');
      if (!scroller) return;
      var key = scrollKey(scroller);
      if (shown && shown.el === scroller && shown.key === key) return;
      var pushed = (!shown || shown.shell !== live) && live.getAttribute('data-nav') === 'fwd';
      shown = { el: scroller, key: key, shell: live };
      if (pushed) delete offsets[key];
      else scroller.scrollTop = offsets[key] || 0;
    }

    var reduce = window.matchMedia('(prefers-reduced-motion: reduce)');

    function dropParents() {
      parents.forEach(function (p) { p.remove(); });
      parents = [];
    }

    function park(node) {
      if (!node.isConnected || parents.indexOf(node) < 0) { node.remove(); return; }
      if (node.classList.contains('page-parent')) return;
      node.classList.add('page-parent');
      var ids = node.querySelectorAll('[id]');
      for (var k = 0; k < ids.length; k++) if (!ids[k].closest('svg')) ids[k].removeAttribute('id');
    }

    function fadeOut(node) {
      node.classList.add('page-fading');
      node.addEventListener('transitionend', function (e) { if (e.target === node) node.remove(); });
      setTimeout(function () { node.remove(); }, 250);
    }

    window._countedObserve(function (records) {
      restoreScroll();
      if (reduce.matches) { if (parents.length) dropParents(); return; }
      for (var i = 0; i < records.length; i++) {
        if (records[i].target !== host) continue;
        var removed = records[i].removedNodes;
        for (var j = 0; j < removed.length; j++) {
          var old = removed[j];
          if (!(old instanceof Element) || !old.classList.contains('page-shell')) continue;
          if (old.classList.contains('page-exiting')) continue;

          var live = liveShell();
          if (!live) continue;
          var swiped = window._countedSwipeBack;
          window._countedSwipeBack = 0;
          if (swiped && Date.now() - swiped < 1000) {
            live.setAttribute('data-nav', 'none');
            var peek = parents.pop();
            if (peek) fadeOut(peek);
            continue;
          }
          var dir = live.getAttribute('data-nav') || 'none';
          if (dir === 'back') { var top = parents.pop(); if (top) top.remove(); }
          if (dir === 'none' || dir === 'lateral') dropParents();
          if (dir === 'none') continue;
          if (plat === 'ios' && dir === 'lateral') continue;
          var keep = plat === 'ios' && dir === 'fwd';

          host.querySelectorAll(':scope > .page-exiting').forEach(function (n) {
            if (parents.indexOf(n) < 0) n.remove();
          });

          old.removeAttribute('data-dioxus-id');
          var ids = old.querySelectorAll('[data-dioxus-id]');
          for (var k = 0; k < ids.length; k++) ids[k].removeAttribute('data-dioxus-id');
          old.inert = true;
          old.setAttribute('aria-hidden', 'true');
          old.setAttribute('data-nav', old.hasAttribute('data-back-progress') ? 'predictive' : dir);
          old.classList.add('page-exiting');

          if (dir === 'back') host.appendChild(old); else host.insertBefore(old, live);
          if (lastScroll && old.contains(lastScroll.el)) lastScroll.el.scrollTop = lastScroll.top;
          if (keep) {
            parents.push(old);
            if (parents.length > 2) parents.shift().remove();
          }

          (function (node, keep) {
            function done(e) {
              if (e && e.target !== node) return;
              if (keep) park(node); else node.remove();
            }
            node.addEventListener('animationend', done);
            setTimeout(done, 600);
          })(old, keep);
        }
      }
    });

    if (plat !== 'ios') return;

    var edge = document.createElement('div');
    edge.className = 'swipe-edge';
    edge.setAttribute('aria-hidden', 'true');
    document.body.appendChild(edge);
    window._countedEdgeSync = function () {
      edge.hidden = !window._routeArmed || !!document.querySelector('.modal.modal-open');
    };
    window._countedEdgeSync();

    var sw = null;

    function follow(s, x) {
      var p = Math.min(1, x / s.w);
      s.live.style.transform = 'translateX(' + x + 'px)';
      if (s.peek) {
        s.peek.style.transform = 'translateX(' + (-0.25 * s.w * (1 - p)) + 'px)';
        s.peek.style.setProperty('--swipe-dim', (0.1 * (1 - p)).toFixed(3));
      }
    }

    function begin(s) {
      if (!window._routeArmed || !window._dioxusBackArmed || window._dioxusSyntheticBack) return false;
      if (document.querySelector('.modal.modal-open')) return false;
      s.live = liveShell();
      if (!s.live) return false;
      s.peek = parents.length ? parents[parents.length - 1] : null;
      s.w = document.documentElement.clientWidth;
      if (reduce.matches) return true;
      s.live.classList.add('page-swiping');
      if (s.peek) s.peek.classList.add('page-peek');
      follow(s, 0);
      return true;
    }

    function reset(s) {
      s.live.classList.remove('page-swiping');
      s.live.style.removeProperty('transform');
      if (s.peek) {
        s.peek.classList.remove('page-peek');
        s.peek.style.removeProperty('transform');
        s.peek.style.removeProperty('--swipe-dim');
      }
    }

    function goBack() {
      window._countedSwipeBack = Date.now();
      window.history.back();
    }

    function finish(s, commit) {
      if (reduce.matches) { if (commit) goBack(); return; }
      var nodes = s.peek ? [s.live, s.peek] : [s.live];
      nodes.forEach(function (n) { n.classList.add('page-swipe-settling'); });
      follow(s, commit ? s.w : 0);
      var settled = false;
      function done(e) {
        if (settled || (e && (e.target !== s.live || e.propertyName !== 'transform'))) return;
        settled = true;
        s.live.removeEventListener('transitionend', done);
        nodes.forEach(function (n) { n.classList.remove('page-swipe-settling'); });
        if (!commit) { reset(s); return; }
        goBack();
        setTimeout(function () { if (s.live.isConnected) reset(s); }, 1000);
      }
      s.live.addEventListener('transitionend', done);
      setTimeout(done, 400);
    }

    function forwardTap(x, y) {
      edge.hidden = true;
      var el = document.elementFromPoint(x, y);
      window._countedEdgeSync();
      if (!el) return;
      if (el.matches('input, textarea, select')) el.focus(); else el.click();
    }

    edge.addEventListener('touchstart', function (e) {
      sw = null;
      if (e.touches.length !== 1) return;
      var t = e.touches[0];
      sw = { x: t.clientX, y: t.clientY, lastX: t.clientX, lastT: e.timeStamp, v: 0, dx: 0, drag: false, dead: false };
    }, { passive: true });

    edge.addEventListener('touchmove', function (e) {
      if (!sw || e.touches.length !== 1) return;
      if (e.cancelable) e.preventDefault();
      if (sw.dead) return;
      var t = e.touches[0], dx = t.clientX - sw.x, dy = t.clientY - sw.y;
      if (!sw.drag) {
        if (Math.abs(dx) < 8 && Math.abs(dy) < 8) return;
        if (dx <= Math.abs(dy) || !begin(sw)) { sw.dead = true; return; }
        sw.drag = true;
      }
      sw.dx = Math.max(0, dx);
      sw.v = (t.clientX - sw.lastX) / Math.max(1, e.timeStamp - sw.lastT);
      sw.lastX = t.clientX;
      sw.lastT = e.timeStamp;
      if (!reduce.matches) follow(sw, sw.dx);
    }, { passive: false });

    function end(e) {
      var s = sw;
      sw = null;
      if (!s) return;
      if (!s.drag) {
        if (!s.dead && e.type === 'touchend') forwardTap(s.x, s.y);
        return;
      }
      finish(s, e.type === 'touchend'
        && (s.dx > s.w * 0.5 || (s.v > 0.5 && e.timeStamp - s.lastT < 100)));
    }
    edge.addEventListener('touchend', end, { passive: true });
    edge.addEventListener('touchcancel', end, { passive: true });
  });
}
"#;

/// No-op passthrough for web and desktop - no wrapper, no keyed remount, nothing changes.
#[cfg(not(any(target_os = "android", target_os = "ios")))]
#[component]
pub fn PageTransition() -> Element {
    rsx! {
        Outlet::<Route> {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[test]
    fn opening_a_project_goes_forward() {
        let id = Uuid::new_v4();
        assert_eq!(
            nav_direction(&Route::ProjectsPage {}, &Route::ExpensesPage { project_id: id }),
            NavDirection::Forward
        );
    }

    #[test]
    fn opening_an_expense_goes_forward() {
        let id = Uuid::new_v4();
        assert_eq!(
            nav_direction(
                &Route::ExpensesPage { project_id: id },
                &Route::PaymentPage { project_id: id, expense_id: 42 }
            ),
            NavDirection::Forward
        );
    }

    #[test]
    fn leaving_an_expense_goes_back() {
        let id = Uuid::new_v4();
        assert_eq!(
            nav_direction(
                &Route::PaymentPage { project_id: id, expense_id: 42 },
                &Route::ExpensesPage { project_id: id }
            ),
            NavDirection::Back
        );
    }

    /// The header arrow and the Android hardware back button both land here, and both must produce
    /// the same animation - neither of them tells us it is a back press.
    #[test]
    fn leaving_a_project_goes_back() {
        let id = Uuid::new_v4();
        assert_eq!(
            nav_direction(&Route::ExpensesPage { project_id: id }, &Route::ProjectsPage {}),
            NavDirection::Back
        );
    }

    /// Everything the dock reaches is a top-level destination, whichever way you cycle.
    #[test]
    fn dock_tabs_are_lateral() {
        assert_eq!(
            nav_direction(&Route::ProjectsPage {}, &Route::ChartsPage {}),
            NavDirection::Lateral
        );
        assert_eq!(
            nav_direction(&Route::SettingsPage {}, &Route::ProjectsPage {}),
            NavDirection::Lateral
        );
        assert_eq!(
            nav_direction(&Route::ChartsPage {}, &Route::SettingsPage {}),
            NavDirection::Lateral
        );
    }

    #[test]
    fn settings_to_a_legal_page_goes_forward_and_back() {
        assert_eq!(
            nav_direction(&Route::SettingsPage {}, &Route::PrivacyPage {}),
            NavDirection::Forward
        );
        assert_eq!(
            nav_direction(&Route::PrivacyPage {}, &Route::SettingsPage {}),
            NavDirection::Back
        );
    }

    #[test]
    fn licenses_are_a_step_past_the_legal_notice() {
        assert_eq!(
            nav_direction(&Route::LegalNoticePage {}, &Route::LicensesPage {}),
            NavDirection::Forward
        );
    }

    #[test]
    fn registering_is_a_step_past_login() {
        assert_eq!(
            nav_direction(&Route::LoginPage {}, &Route::RegisterPage {}),
            NavDirection::Forward
        );
    }

    /// A deep link landing on another project while one is open. The shell remounts (the key is the
    /// URL, which changed), and a slide either way would be a lie about the hierarchy.
    #[test]
    fn switching_project_is_lateral() {
        assert_eq!(
            nav_direction(
                &Route::ExpensesPage { project_id: Uuid::new_v4() },
                &Route::ExpensesPage { project_id: Uuid::new_v4() }
            ),
            NavDirection::Lateral
        );
    }

    #[test]
    fn project_history_sits_beside_an_expense() {
        let id = Uuid::new_v4();
        assert_eq!(route_depth(&Route::ProjectHistoryPage { project_id: id }), 2);
        assert_eq!(
            nav_direction(
                &Route::ExpensesPage { project_id: id },
                &Route::ProjectHistoryPage { project_id: id }
            ),
            NavDirection::Forward
        );
    }

    /// The catch-all is where a bad URL lands; it must not read as a step into the app.
    #[test]
    fn not_found_is_top_level() {
        assert_eq!(route_depth(&Route::NotFoundPage { segments: vec!["zzz".to_string()] }), 0);
    }
}
