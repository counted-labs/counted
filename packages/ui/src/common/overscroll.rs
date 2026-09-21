//! Elastic scrolling and pull-to-refresh for the mobile WebViews. See docs/mobile-scrolling.md.
//!
//! Every routed page scrolls inside its own `.app-container`, and a box whose content fits is not a
//! scroll node on either platform: iOS backs an overflow scroller with a `UIScrollView` whose
//! `alwaysBounceVertical` is never set, and Chromium sends the unused delta to the viewport. So a
//! one-item list does not move under the finger at all. The gesture below fills that gap — and on
//! Android replaces the WebView's own stretch, which only some versions draw for a non-root
//! scroller (`features::kOverscrollEffectOnNonRootScrollers`), so `main.css` switches it off there.
//!
//! All of it stays in JS: every event dispatched to Rust on a native mobile target is a synchronous
//! XHR over wry's bridge (`app_header.rs`, docs/mobile-navigation.md), and a `touchmove` per frame
//! would pin the WebView. Only the one "refresh" trigger crosses over, through the window hook the
//! `PullToRefresh` component installs.

#[cfg(any(target_os = "android", target_os = "ios"))]
use dioxus::prelude::*;

/// Set by `PullToRefresh` on mount; the gesture calls it once an armed pull is released.
pub const REFRESH_HOOK: &str = "_countedRefresh";
/// Set by the gesture; `PullToRefresh` calls it when the page's refresh has finished.
pub const DONE_HOOK: &str = "_countedPtrDone";

/// Damping applied to the finger's travel — 0.45 is close to UIScrollView's rubber band.
const DAMPING: &str = "0.45";
/// Pixels of damped travel that arm the refresh; `--ptr-pull` is the ratio to this.
const ARM_PX: &str = "64";

#[cfg(any(target_os = "android", target_os = "ios"))]
pub fn use_overscroll() {
    use_effect(|| {
        document::eval(&overscroll_js());
    });
}

#[cfg(not(any(target_os = "android", target_os = "ios")))]
pub fn use_overscroll() {}

#[cfg_attr(not(any(target_os = "android", target_os = "ios")), allow(dead_code))]
fn overscroll_js() -> String {
    let plat = if cfg!(target_os = "ios") { "ios" } else { "android" };
    OVERSCROLL_JS
        .replace("$PLAT", plat)
        .replace("$DAMPING", DAMPING)
        .replace("$ARM_PX", ARM_PX)
        .replace("$REFRESH_HOOK", REFRESH_HOOK)
        .replace("$DONE_HOOK", DONE_HOOK)
}

/// Passive capture-phase listeners on `main`: capture because the scroller is a descendant and the
/// touch may be stopped at its target (`DropdownButton` does), passive because nothing here prevents
/// a default — at an edge the box cannot scroll further anyway.
///
/// State written to the DOM, all of it removed at rest:
/// - `.app-container[data-overscroll]` + `--overscroll` — `pulling` while the finger is down,
///   `settling` for the spring back. `main.css` transforms the container's children off it; nothing
///   may stay transformed once settled, or the FAB and the dropdown scrims lose their viewport anchor
///   (`page_transition.rs`).
/// - `.ptr[data-state]` + `--ptr-pull` — `pulling` → `armed` → `refreshing` → `done`.
///
/// iOS keeps its native rubber band wherever the box is scrollable (`nativeBounce`): the JS bounce
/// on top of it would move the content twice.
#[cfg_attr(not(any(target_os = "android", target_os = "ios")), allow(dead_code))]
const OVERSCROLL_JS: &str = r#"
(function () {
  if (window._countedOverscroll) return;
  var main = document.querySelector('main');
  if (!main) return;
  window._countedOverscroll = true;

  var PLAT = '$PLAT';
  var DAMPING = $DAMPING;
  var ARM_PX = $ARM_PX;
  var EDGE_GUARD = 24;
  var MIN_SPIN_MS = 500;
  var reduce = window.matchMedia('(prefers-reduced-motion: reduce)');

  var g = null;

  function damp(dy) {
    var off = Math.min(Math.abs(dy) * DAMPING, 140);
    return dy < 0 ? -off : off;
  }

  function settle(scroller) {
    scroller.dataset.overscroll = 'settling';
    scroller.style.setProperty('--overscroll', '0px');
    var done = false;
    function finish(e) {
      if (done || (e && e.propertyName !== 'transform')) return;
      done = true;
      scroller.removeEventListener('transitionend', finish);
      delete scroller.dataset.overscroll;
      scroller.style.removeProperty('--overscroll');
    }
    scroller.addEventListener('transitionend', finish);
    setTimeout(finish, 350);
  }

  function clearPtr(ptr) {
    delete ptr.dataset.state;
    delete ptr.dataset.since;
    ptr.style.removeProperty('--ptr-pull');
  }

  main.addEventListener('touchstart', function (e) {
    g = null;
    if (e.touches.length !== 1 || !(e.target instanceof Element)) return;
    var t = e.touches[0];
    if (t.clientX < EDGE_GUARD || e.target.closest('.modal')) return;
    var scroller = e.target.closest('.app-container');
    if (!scroller) return;
    var ptr = scroller.querySelector(':scope > .ptr');
    if (ptr && ptr.dataset.state === 'refreshing') ptr = null;
    g = {
      scroller: scroller,
      ptr: ptr,
      startY: t.clientY,
      atTop: scroller.scrollTop <= 0,
      atBottom: scroller.scrollTop + scroller.clientHeight >= scroller.scrollHeight - 1,
      nativeBounce: PLAT === 'ios' && scroller.scrollHeight > scroller.clientHeight + 1,
      bouncing: false,
      armed: false
    };
  }, { capture: true, passive: true });

  main.addEventListener('touchmove', function (e) {
    if (!g || e.touches.length !== 1) return;
    var y = e.touches[0].clientY;
    // A drag that started mid-list and ran into an edge continues into the rubber band from
    // there, so the travel is re-based on the edge rather than on where the finger landed.
    if (!g.atTop && y > g.startY && g.scroller.scrollTop <= 0) { g.atTop = true; g.startY = y; }
    if (!g.atBottom && y < g.startY
        && g.scroller.scrollTop + g.scroller.clientHeight >= g.scroller.scrollHeight - 1) {
      g.atBottom = true; g.startY = y;
    }
    var dy = y - g.startY;
    var pullingDown = dy > 0 && g.atTop;
    var pullingUp = dy < 0 && g.atBottom;

    if (!g.nativeBounce && !reduce.matches && (pullingDown || pullingUp)) {
      g.bouncing = true;
      g.scroller.dataset.overscroll = 'pulling';
      g.scroller.style.setProperty('--overscroll', damp(dy) + 'px');
    } else if (g.bouncing) {
      g.bouncing = false;
      settle(g.scroller);
    }

    if (g.ptr) {
      if (pullingDown) {
        var progress = Math.min(damp(dy) / ARM_PX, 1.25);
        g.armed = progress >= 1;
        g.ptr.style.setProperty('--ptr-pull', progress.toFixed(3));
        g.ptr.dataset.state = g.armed ? 'armed' : 'pulling';
      } else if (g.ptr.dataset.state) {
        g.armed = false;
        clearPtr(g.ptr);
      }
    }
  }, { capture: true, passive: true });

  function end() {
    if (!g) return;
    var s = g;
    g = null;
    if (s.bouncing) settle(s.scroller);
    if (!s.ptr) return;
    if (s.armed) {
      s.ptr.dataset.state = 'refreshing';
      s.ptr.dataset.since = String(Date.now());
      s.ptr.style.setProperty('--ptr-pull', '1');
      if (window.$REFRESH_HOOK) window.$REFRESH_HOOK();
    } else {
      clearPtr(s.ptr);
    }
  }
  main.addEventListener('touchend', end, { capture: true, passive: true });
  main.addEventListener('touchcancel', end, { capture: true, passive: true });

  window.$DONE_HOOK = function () {
    var ptr = document.querySelector('.ptr[data-state="refreshing"]');
    if (!ptr) return;
    var since = Number(ptr.dataset.since) || 0;
    var wait = Math.max(0, since + MIN_SPIN_MS - Date.now());
    setTimeout(function () {
      if (ptr.dataset.state !== 'refreshing') return;
      ptr.dataset.state = 'done';
      var finished = false;
      function finish() {
        if (finished) return;
        finished = true;
        ptr.removeEventListener('transitionend', finish);
        if (ptr.dataset.state === 'done') clearPtr(ptr);
      }
      ptr.addEventListener('transitionend', finish);
      setTimeout(finish, 400);
    }, wait);
  };
})();
"#;
