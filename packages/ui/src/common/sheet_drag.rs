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
/// A touch becomes a drag down only when it moves down, mostly vertically, and every scroller
/// between the finger and the box is at its top - the native rule, so a sheet scrolled into its
/// content scrolls back first. A release past 30% of the box's height, or a downward fling,
/// dismisses. A drag up starts only where nothing under the finger can scroll further down, and
/// rubber-bands past the top (UIKit's curve, `h * (1 - 1 / (d * 0.55 / h + 1))`) and settles back.
///
/// **Detents.** A `.modal` marked `data-sheet-detents` (the participants and currency sheets) opens
/// at a medium stop, half the viewport, when its full height is at least 48px more: `main.css`
/// sizes the box from `data-sheet-detent="medium"` and `--sheet-medium`, so its footer stays on
/// screen. Any drag up from there expands it, scroller or not (UIKit's
/// `prefersScrollingExpandsWhenScrolledToEdge`), and so does focusing a field in it. During the
/// drag the box is at its full height,
/// offset by the difference so its top edge does not move; a release goes to the nearer stop, a
/// fling to the next one, and past 30% of the remaining height below medium it dismisses.
///
/// A `.modal` marked `data-sheet-locked` cannot be dismissed: it follows the finger down on the same
/// rubber band and always settles back. One marked `data-sheet-dirty` (a changed form,
/// `expense_form.rs` and the project forms) resists the same way, and a release that would have
/// dismissed clicks its backdrop instead, which asks to discard while the sheet settles back.
///
/// State on the DOM, removed at rest: `data-sheet-drag` (`pulling` / `settling`) on the box and the
/// `.modal`, `--sheet-drag` on the box and `--sheet-scrim` on the `.modal`. `main.css` draws them.
///
/// **Slide-out.** Every close (✕, backdrop, Android back, Done, a write that closes the form)
/// unmounts the sheet in Rust at once, so the removed `.modal-bottom` is caught in the shared
/// observer's `removedNodes` and put back on `body` for the length of `sheet-out`, the way
/// `page_transition.rs` keeps an outgoing page. The node itself, not a clone, so typed values stay
/// on screen; stripped of every `data-dioxus-id` (Dioxus recycles freed ids) and every `id` (no
/// duplicate for `getElementById` or a test), `inert`, and `sheet-closing` instead of `modal-open`,
/// so `navigation_sync.rs`'s top sheet and Android back never see it. A timer removes it when
/// `animationend` never comes (a hidden tab). Skipped: a sheet the drag already took off-screen
/// (`data-sheet-gone`), one whose parent left with its page (a route change), wide screens and
/// Reduce Motion. Its own removal is ignored, or it would be adopted again.
#[cfg_attr(not(any(target_os = "android", target_os = "ios")), allow(dead_code))]
const SHEET_DRAG_JS: &str = r#"
(function () {
  if (window._countedSheetDrag) return;
  window._countedSheetDrag = true;

  var SLOP = 10;
  var DISMISS_RATIO = 0.3;
  var FLING = 0.5;
  var RUBBER = 0.55;
  var MEDIUM = 0.5;
  var MIN_DETENT = 48;
  var narrow = window.matchMedia('(max-width: 639.98px)');
  var g = null;

  function atTop(el, box) {
    for (; el; el = el.parentElement) {
      if (el.scrollTop > 0) return false;
      if (el === box) return true;
    }
    return true;
  }

  function atBottom(el, box) {
    for (; el; el = el.parentElement) {
      if (el.scrollHeight - el.clientHeight - el.scrollTop > 1) {
        var o = getComputedStyle(el).overflowY;
        if (o === 'auto' || o === 'scroll') return false;
      }
      if (el === box) return true;
    }
    return true;
  }

  function band(d, h) {
    return h * (1 - 1 / (d * RUBBER / h + 1));
  }

  function mediumHeight() {
    return Math.round(document.documentElement.clientHeight * MEDIUM);
  }

  function setMedium(box) {
    box.dataset.sheetDetent = 'medium';
    box.style.setProperty('--sheet-medium', mediumHeight() + 'px');
  }

  function setLarge(box) {
    delete box.dataset.sheetDetent;
    box.style.removeProperty('--sheet-medium');
  }

  // The medium stop's offset below the full height, read with the box at its full height; null
  // when the sheet has no detents or is too short for one.
  function midStop(modal, box) {
    if (!modal.hasAttribute('data-sheet-detents')) return null;
    var mid = box.offsetHeight - mediumHeight();
    return mid >= MIN_DETENT ? mid : null;
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
    modal.style.setProperty('--sheet-scrim', to > 0 && to >= box.offsetHeight ? '0' : '1');
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

  // Settles on a stop; at the medium one the box then takes its medium height in the same frame
  // the offset goes, so its top edge stays where it is.
  function rest(box, modal, to, mid) {
    settle(box, modal, to, function () {
      clear(box, modal);
      if (mid !== null && to === mid && to > 0) setMedium(box);
    });
  }

  // The sheet leaves first and closes after, so it never vanishes mid-screen. A sheet that is still
  // there once Rust has had its frame did not close on the backdrop click, and comes back.
  // `data-sheet-gone` tells the slide-out below the sheet is already off-screen.
  function dismiss(box, modal) {
    settle(box, modal, box.offsetHeight, function () {
      modal.setAttribute('data-sheet-gone', '');
      clickBackdrop(modal);
      setTimeout(function () {
        if (!modal.isConnected) return;
        modal.removeAttribute('data-sheet-gone');
        settle(box, modal, 0, function () { clear(box, modal); });
      }, 300);
    });
  }

  function clickBackdrop(modal) {
    var backdrop = Array.from(modal.querySelectorAll('.modal-backdrop'))
      .find(function (b) { return b.closest('.modal') === modal; });
    if (backdrop) backdrop.click();
  }

  function cancel(s) {
    if (s.dragging) rest(s.box, s.modal, s.base, s.mid);
  }

  function initDetent(modal) {
    var box = modal.querySelector(':scope > .modal-box');
    if (box && narrow.matches && midStop(modal, box) !== null) setMedium(box);
  }

  function expand(box) {
    var modal = box.parentElement;
    var medium = box.offsetHeight;
    setLarge(box);
    var mid = box.offsetHeight - medium;
    if (!(mid > 0) || !modal) return;
    box.dataset.sheetDrag = 'pulling';
    modal.dataset.sheetDrag = 'pulling';
    box.style.setProperty('--sheet-drag', mid + 'px');
    void getComputedStyle(box).transform;
    rest(box, modal, 0, null);
  }

  document.addEventListener('focusin', function (e) {
    var el = e.target;
    var box = el instanceof Element ? el.closest('.modal-box[data-sheet-detent="medium"]') : null;
    if (box && window._countedTextLike && window._countedTextLike(el)) expand(box);
  }, true);

  function attach(modal) {
    modal.addEventListener('touchstart', function (e) {
      if (g && g.modal === modal) {
        var s = g;
        g = null;
        cancel(s);
      }
      if (!narrow.matches || e.touches.length !== 1 || !(e.target instanceof Element)) return;
      if (window._countedKeyboard && window._countedKeyboard.up()) return;
      var box = e.target.closest('.modal-box');
      if (!box || box.closest('.modal') !== modal || box.dataset.sheetDrag === 'settling') return;
      var t = e.touches[0];
      g = { modal: modal, box: box, target: e.target, x: t.clientX, y: t.clientY,
            lastY: t.clientY, lastT: e.timeStamp, v: 0, raw: 0, base: 0, mid: null, dragging: false,
            locked: modal.hasAttribute('data-sheet-locked') || modal.hasAttribute('data-sheet-dirty'),
            guarded: !modal.hasAttribute('data-sheet-locked') && modal.hasAttribute('data-sheet-dirty') };
    }, { capture: true, passive: true });

    modal.addEventListener('touchmove', function (e) {
      if (!g || g.modal !== modal || e.touches.length !== 1) return;
      var t = e.touches[0];
      if (!g.dragging) {
        var dx = t.clientX - g.x, dy = t.clientY - g.y;
        var medium = g.box.dataset.sheetDetent === 'medium';
        var down = dy > 0 && Math.abs(dx) <= dy && atTop(g.target, g.box);
        var up = dy < 0 && Math.abs(dx) <= -dy && (medium || atBottom(g.target, g.box));
        if (!down && !up) {
          if (Math.max(Math.abs(dx), Math.abs(dy)) >= SLOP) g = null;
          return;
        }
        if (e.cancelable) e.preventDefault();
        if (Math.abs(dy) < SLOP) return;
        g.dragging = true;
        g.y = t.clientY;
        if (medium) setLarge(g.box);
        g.mid = midStop(g.modal, g.box);
        g.base = medium && g.mid !== null ? g.mid : 0;
        g.box.dataset.sheetDrag = 'pulling';
        g.modal.dataset.sheetDrag = 'pulling';
      }
      if (e.cancelable) e.preventDefault();
      var h = g.box.offsetHeight, low = g.mid !== null ? g.mid : 0;
      var raw = g.base + t.clientY - g.y, off;
      if (raw < 0) off = -band(-raw, h);
      else if (g.locked && raw > g.base) off = g.base + band(raw - g.base, h);
      else off = raw;
      g.raw = raw;
      g.v = (t.clientY - g.lastY) / Math.max(1, e.timeStamp - g.lastT);
      g.lastY = t.clientY;
      g.lastT = e.timeStamp;
      g.box.style.setProperty('--sheet-drag', off + 'px');
      g.modal.style.setProperty('--sheet-scrim',
        Math.min(1, Math.max(0, 1 - Math.max(0, off - low) / Math.max(1, h - low))).toFixed(3));
    }, { capture: true, passive: false });

    function end(e) {
      if (!g || g.modal !== modal) return;
      var s = g;
      g = null;
      if (!s.dragging) return;
      var h = s.box.offsetHeight, recent = e.timeStamp - s.lastT < 100;
      var to;
      if (e.type !== 'touchend') to = s.base;
      else if (s.v < -FLING && recent) to = 0;
      else if (s.v > FLING && recent && s.raw - s.base > SLOP) to = s.mid !== null && s.raw < s.mid ? s.mid : 'dismiss';
      else if (s.mid !== null) to = s.raw > s.mid + (h - s.mid) * DISMISS_RATIO ? 'dismiss' : (s.raw < s.mid / 2 ? 0 : s.mid);
      else to = s.raw > h * DISMISS_RATIO ? 'dismiss' : 0;
      if (to !== 'dismiss') {
        rest(s.box, s.modal, to, s.mid);
      } else if (s.locked) {
        rest(s.box, s.modal, s.mid !== null ? s.mid : 0, s.mid);
        if (s.guarded) clickBackdrop(s.modal);
      } else {
        dismiss(s.box, s.modal);
      }
    }
    modal.addEventListener('touchend', end, { capture: true, passive: true });
    modal.addEventListener('touchcancel', end, { capture: true, passive: true });
    initDetent(modal);
  }

  function scan() {
    document.querySelectorAll('.modal-bottom').forEach(function (m) {
      if (m._countedSheetDrag) return;
      m._countedSheetDrag = true;
      attach(m);
      if (window._countedKeyboard) window._countedKeyboard.restyle();
    });
  }

  var reduce = window.matchMedia('(prefers-reduced-motion: reduce)');
  var scrolled = new Map();
  document.addEventListener('scroll', function (e) {
    var el = e.target;
    if (el instanceof Element && el.closest('.modal-bottom')) scrolled.set(el, el.scrollTop);
  }, { capture: true, passive: true });

  function slideOut(old) {
    var box = old.querySelector(':scope > .modal-box');
    if (!box) return;
    [old].concat(Array.from(old.querySelectorAll('[data-dioxus-id], [id]'))).forEach(function (el) {
      el.removeAttribute('data-dioxus-id');
      el.removeAttribute('id');
    });
    clear(box, old);
    old.classList.remove('modal-open');
    old.classList.add('sheet-closing');
    if (document.querySelector('.modal.modal-open')) old.classList.add('sheet-closing-stacked');
    old.inert = true;
    old.setAttribute('aria-hidden', 'true');
    document.body.appendChild(old);
    scrolled.forEach(function (top, el) { if (old.contains(el)) el.scrollTop = top; });
    var removed = false;
    function remove(e) {
      if (removed || (e && e.target !== box)) return;
      removed = true;
      old.remove();
    }
    box.addEventListener('animationend', remove);
    setTimeout(remove, 400);
  }

  function closed(records) {
    if (!reduce.matches && narrow.matches) {
      for (var i = 0; i < records.length; i++) {
        var host = records[i].target;
        if (!(host instanceof Element) || !host.isConnected || host.closest('.page-exiting')) continue;
        var removed = records[i].removedNodes;
        for (var j = 0; j < removed.length; j++) {
          var old = removed[j];
          if (!(old instanceof Element) || !old.classList.contains('modal-bottom')) continue;
          if (!old.classList.contains('modal-open') || old.hasAttribute('data-sheet-gone')) continue;
          slideOut(old);
        }
      }
    }
    scrolled.forEach(function (_, el) { if (!el.isConnected) scrolled.delete(el); });
  }

  window._countedObserve(function (records) {
    closed(records);
    scan();
  });
  scan();
})();
"#;
