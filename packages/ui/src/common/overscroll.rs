//! Elastic scrolling and pull-to-refresh for the mobile WebViews. See docs/mobile-scrolling.md.
//!
//! Every routed page scrolls inside its own `.app-container`, and a box whose content fits is not a
//! scroll node on either platform: iOS backs an overflow scroller with a `UIScrollView` whose
//! `alwaysBounceVertical` is never set, and Chromium sends the unused delta to the viewport. So a
//! one-item list does not move under the finger at all. The gesture below fills that gap - and on
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

/// Damping applied to the finger's travel - 0.45 is close to UIScrollView's rubber band.
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
/// touch may be stopped at its target (the speed dial does), passive because nothing here prevents
/// a default - at an edge the box cannot scroll further anyway.
///
/// State written to the DOM, all of it removed at rest:
/// - `.app-container[data-overscroll]` + `--overscroll` - `pulling` while the finger is down,
///   `settling` for the spring back, `holding` while an iOS refresh keeps the content down.
/// - `[data-band]` on what moves: every child of the scroller, except that a `.fixed` element is
///   skipped and one holding a fixed element is descended into (two levels) rather than moved, so
///   the FAB and the scrims keep their viewport anchor (a transform makes an ancestor their
///   containing block, `page_transition.rs`). Only transforms move: nothing animates `top`.
/// - Android instead: `--overscroll-scale` on the container and `--oy` on each moved element, the
///   M3 stretch (`scaleY` from the pulled edge, nothing translated). A refreshable page does not
///   stretch on a pull down: the content stays still under the refresh disc, as in M3.
/// - `.ptr[data-state]` + `--ptr-pull` - `pulling` → `armed` → `refreshing` → `done`.
///
/// iOS refreshes the moment the pull arms, with the finger still down, as `UIRefreshControl` does,
/// and on release the content settles to 48px instead of 0 and holds there (UIKit's
/// `contentInset`) until the refresh is done. Android refreshes on release, as M3's does.
///
/// iOS keeps its native rubber band wherever the box is scrollable (`nativeBounce`): the JS bounce
/// on top of it would move the content twice. Only the hold is the script's there.
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
  var HOLD_PX = 48;
  var EDGE_GUARD = 24;
  var MIN_SPIN_MS = 500;
  var reduce = window.matchMedia('(prefers-reduced-motion: reduce)');

  var g = null;

  function damp(dy) {
    var off = Math.min(Math.abs(dy) * DAMPING, 140);
    return dy < 0 ? -off : off;
  }

  function movers(scroller) {
    var out = [];
    (function walk(el, depth) {
      for (var c = el.firstElementChild; c; c = c.nextElementSibling) {
        if (c.classList.contains('fixed')) continue;
        if (PLAT === 'android' && c.classList.contains('ptr')) continue;
        if (c.querySelector('.fixed')) { if (depth < 2) walk(c, depth + 1); continue; }
        out.push(c);
      }
    })(scroller, 0);
    return out;
  }

  function mark(scroller) {
    if (scroller._countedBand) return;
    scroller._countedBand = movers(scroller);
    scroller._countedBand.forEach(function (el) { el.setAttribute('data-band', ''); });
  }

  function unmark(scroller) {
    (scroller._countedBand || []).forEach(function (el) {
      el.removeAttribute('data-band');
      el.style.removeProperty('--oy');
    });
    scroller._countedBand = null;
  }

  // Android stretches from the pulled edge. Each moved element scales about that edge (`--oy`, the
  // edge in its own coordinates), so together they stretch as one block. Read once per direction.
  function anchor(scroller, down) {
    var box = scroller.getBoundingClientRect();
    var edge = down ? box.top : box.bottom;
    scroller._countedBand.forEach(function (el) {
      el.style.setProperty('--oy', (edge - el.getBoundingClientRect().top) + 'px');
    });
  }

  function stretch(scroller, dy) {
    return (1 + Math.abs(damp(dy)) / Math.max(scroller.clientHeight, 1) * 0.5).toFixed(4);
  }

  function settle(scroller, hold) {
    var token = scroller._countedSettle = {};
    mark(scroller);
    scroller.dataset.overscroll = 'settling';
    scroller.style.setProperty('--overscroll', (hold || 0) + 'px');
    scroller.style.setProperty('--overscroll-scale', '1');
    var done = false;
    function finish(e) {
      if (done || (e && e.propertyName !== 'transform')) return;
      done = true;
      scroller.removeEventListener('transitionend', finish);
      if (scroller._countedSettle !== token) return;
      if (hold) { scroller.dataset.overscroll = 'holding'; return; }
      delete scroller.dataset.overscroll;
      scroller.style.removeProperty('--overscroll');
      scroller.style.removeProperty('--overscroll-scale');
      unmark(scroller);
    }
    scroller.addEventListener('transitionend', finish);
    setTimeout(finish, 350);
  }

  function clearPtr(ptr) {
    delete ptr.dataset.state;
    delete ptr.dataset.since;
    ptr.style.removeProperty('--ptr-pull');
  }

  function refresh(ptr) {
    ptr.dataset.state = 'refreshing';
    ptr.dataset.since = String(Date.now());
    ptr.style.setProperty('--ptr-pull', '1');
    if (window.$REFRESH_HOOK) window.$REFRESH_HOOK();
  }

  main.addEventListener('touchstart', function (e) {
    g = null;
    if (e.touches.length !== 1 || !(e.target instanceof Element)) return;
    var t = e.touches[0];
    if (t.clientX < EDGE_GUARD || e.target.closest('.modal')) return;
    var scroller = e.target.closest('.app-container');
    if (!scroller || scroller.dataset.overscroll === 'holding') return;
    var ptr = scroller.querySelector(':scope > .ptr');
    var refreshable = !!ptr;
    if (ptr && ptr.dataset.state === 'refreshing') ptr = null;
    g = {
      scroller: scroller,
      ptr: ptr,
      refreshable: refreshable,
      down: null,
      startY: t.clientY,
      atTop: scroller.scrollTop <= 0,
      atBottom: scroller.scrollTop + scroller.clientHeight >= scroller.scrollHeight - 1,
      nativeBounce: PLAT === 'ios' && scroller.scrollHeight > scroller.clientHeight + 1,
      bouncing: false,
      armed: false,
      fired: false
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

    // M3 keeps the content still while the refresh disc comes down.
    var android = PLAT === 'android';
    var stillForDisc = android && pullingDown && g.refreshable;

    if (!g.nativeBounce && !reduce.matches && !stillForDisc && (pullingDown || pullingUp)) {
      g.bouncing = true;
      mark(g.scroller);
      g.scroller.dataset.overscroll = 'pulling';
      if (android) {
        if (g.down !== pullingDown) {
          g.down = pullingDown;
          g.scroller.style.setProperty('--overscroll-scale', '1');
          anchor(g.scroller, pullingDown);
        }
        g.scroller.style.setProperty('--overscroll-scale', stretch(g.scroller, dy));
      } else {
        g.scroller.style.setProperty('--overscroll', damp(dy) + 'px');
      }
    } else if (g.bouncing) {
      g.bouncing = false;
      g.down = null;
      settle(g.scroller);
    }

    if (g.ptr && !g.fired) {
      if (pullingDown) {
        var progress = Math.min(damp(dy) / ARM_PX, 1.25);
        g.armed = progress >= 1;
        g.ptr.style.setProperty('--ptr-pull', progress.toFixed(3));
        g.ptr.dataset.state = g.armed ? 'armed' : 'pulling';
        if (g.armed && !android) {
          g.fired = true;
          refresh(g.ptr);
        }
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
    var holding = s.fired && s.ptr.dataset.state === 'refreshing';
    if (holding) settle(s.scroller, HOLD_PX);
    else if (s.bouncing) settle(s.scroller);
    if (!s.ptr || s.fired) return;
    if (s.armed) refresh(s.ptr);
    else clearPtr(s.ptr);
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
      var scroller = ptr.parentElement;
      if (scroller && (scroller.dataset.overscroll === 'holding' || scroller.dataset.overscroll === 'settling')
          && !(g && g.scroller === scroller)) {
        settle(scroller);
      }
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
