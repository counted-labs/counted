//! Drag a bottom sheet down to dismiss it, as on iOS and Android. See docs/mobile-navigation.md,
//! "Sheet drag".
//!
//! JS only, like `overscroll.rs`: every event dispatched to Rust on a native mobile target is a
//! synchronous XHR over wry's bridge, and a `touchmove` per frame would pin the WebView. Nothing
//! crosses over at all - a committed drag clicks the sheet's own `.modal-backdrop`, the same exit
//! the Android back button takes (`navigation_sync.rs`), so it closes as a tap outside does.

#[cfg(any(target_os = "android", target_os = "ios"))]
use dioxus::prelude::*;

#[cfg(any(target_os = "android", target_os = "ios"))]
pub fn use_sheet_drag() {
    use_effect(|| {
        document::eval(SHEET_DRAG_JS);
    });
}

#[cfg(not(any(target_os = "android", target_os = "ios")))]
pub fn use_sheet_drag() {}

/// Listeners go on each `.modal-bottom` as it appears, not on `main`: `touchmove` has to be
/// non-passive to stop the sheet body's own rubber band while the sheet follows the finger, and a
/// non-passive listener on `main` would put every page scroll behind the main thread.
///
/// A touch becomes a drag only when it moves down, mostly vertically, and every scroller between the
/// finger and the box is at its top - the native rule, so a sheet scrolled into its content scrolls
/// back first. A release past 30% of the box's height, or a downward fling, dismisses.
///
/// State on the DOM, removed at rest: `data-sheet-drag` (`pulling` / `settling`) on the box and the
/// `.modal`, `--sheet-drag` on the box and `--sheet-scrim` on the `.modal`. `main.css` draws them.
#[cfg_attr(not(any(target_os = "android", target_os = "ios")), allow(dead_code))]
const SHEET_DRAG_JS: &str = r#"
(function () {
  if (window._countedSheetDrag) return;
  window._countedSheetDrag = true;

  var SLOP = 10;
  var DISMISS_RATIO = 0.3;
  var FLING = 0.5;
  var narrow = window.matchMedia('(max-width: 639.98px)');
  var g = null;

  function atTop(el, box) {
    for (; el; el = el.parentElement) {
      if (el.scrollTop > 0) return false;
      if (el === box) return true;
    }
    return true;
  }

  function clear(box, modal) {
    delete box.dataset.sheetDrag;
    box.style.removeProperty('--sheet-drag');
    delete modal.dataset.sheetDrag;
    modal.style.removeProperty('--sheet-scrim');
  }

  function settle(box, modal, to, done) {
    box.dataset.sheetDrag = 'settling';
    modal.dataset.sheetDrag = 'settling';
    box.style.setProperty('--sheet-drag', to + 'px');
    modal.style.setProperty('--sheet-scrim', to > 0 ? '0' : '1');
    var finished = false;
    function finish(e) {
      if (finished || (e && (e.target !== box || e.propertyName !== 'transform'))) return;
      finished = true;
      box.removeEventListener('transitionend', finish);
      done();
    }
    box.addEventListener('transitionend', finish);
    setTimeout(finish, 350);
  }

  // The sheet leaves first and closes after, so it never vanishes mid-screen. A sheet that is still
  // there once Rust has had its frame did not close on the backdrop click, and comes back.
  function dismiss(box, modal) {
    settle(box, modal, box.offsetHeight, function () {
      var backdrop = Array.from(modal.querySelectorAll('.modal-backdrop'))
        .find(function (b) { return b.closest('.modal') === modal; });
      if (backdrop) backdrop.click();
      setTimeout(function () {
        if (!modal.isConnected) return;
        settle(box, modal, 0, function () { clear(box, modal); });
      }, 300);
    });
  }

  function release(s, commit) {
    if (!s.dragging) return;
    if (commit) dismiss(s.box, s.modal);
    else settle(s.box, s.modal, 0, function () { clear(s.box, s.modal); });
  }

  function attach(modal) {
    modal.addEventListener('touchstart', function (e) {
      if (g && g.modal === modal) {
        var s = g;
        g = null;
        release(s, false);
      }
      if (!narrow.matches || e.touches.length !== 1 || !(e.target instanceof Element)) return;
      if (window._countedKeyboard && window._countedKeyboard.up()) return;
      var box = e.target.closest('.modal-box');
      if (!box || box.closest('.modal') !== modal || box.dataset.sheetDrag === 'settling') return;
      var t = e.touches[0];
      g = { modal: modal, box: box, target: e.target, x: t.clientX, y: t.clientY,
            lastY: t.clientY, lastT: e.timeStamp, v: 0, off: 0, dragging: false };
    }, { capture: true, passive: true });

    modal.addEventListener('touchmove', function (e) {
      if (!g || g.modal !== modal || e.touches.length !== 1) return;
      var t = e.touches[0];
      if (!g.dragging) {
        var dx = t.clientX - g.x, dy = t.clientY - g.y;
        if (dy <= 0 || Math.abs(dx) > dy || !atTop(g.target, g.box)) {
          if (Math.max(Math.abs(dx), Math.abs(dy)) >= SLOP) g = null;
          return;
        }
        if (e.cancelable) e.preventDefault();
        if (dy < SLOP) return;
        g.dragging = true;
        g.y = t.clientY;
        g.box.dataset.sheetDrag = 'pulling';
        g.modal.dataset.sheetDrag = 'pulling';
      }
      if (e.cancelable) e.preventDefault();
      g.off = Math.max(0, t.clientY - g.y);
      g.v = (t.clientY - g.lastY) / Math.max(1, e.timeStamp - g.lastT);
      g.lastY = t.clientY;
      g.lastT = e.timeStamp;
      g.box.style.setProperty('--sheet-drag', g.off + 'px');
      g.modal.style.setProperty('--sheet-scrim',
        Math.max(0, 1 - g.off / g.box.offsetHeight).toFixed(3));
    }, { capture: true, passive: false });

    function end(e) {
      if (!g || g.modal !== modal) return;
      var s = g;
      g = null;
      var flung = s.v > FLING && e.timeStamp - s.lastT < 100;
      release(s, e.type === 'touchend'
        && (s.off > s.box.offsetHeight * DISMISS_RATIO || (flung && s.off > SLOP)));
    }
    modal.addEventListener('touchend', end, { capture: true, passive: true });
    modal.addEventListener('touchcancel', end, { capture: true, passive: true });
  }

  function scan() {
    document.querySelectorAll('.modal-bottom').forEach(function (m) {
      if (m._countedSheetDrag) return;
      m._countedSheetDrag = true;
      attach(m);
      if (window._countedKeyboard) window._countedKeyboard.restyle();
    });
  }
  new MutationObserver(scan).observe(document.body, { childList: true, subtree: true });
  scan();
})();
"#;
