//! The on-screen keyboard and the bottom sheets: their inset, revealing the focused field, and
//! dismissing the keyboard by touching above it. See docs/styling.md, "Pitfall: a `position:
//! fixed` sheet and the on-screen keyboard", and docs/plans/sheet-keyboard.md.
//!
//! JS only, applied in the `visualViewport` event itself. It used to cross to Rust and come back as
//! an eval writing `--kb`/`--vvo` on `<html>`: every keyboard step reached the sheet 30-190 ms late
//! and restyled the whole document (measured on a Fairphone 3), so the sheet jumped after the
//! keyboard instead of with it.

#[cfg(any(target_os = "android", target_os = "ios"))]
use dioxus::prelude::*;

#[cfg(any(target_os = "android", target_os = "ios"))]
pub fn use_keyboard() {
    use_effect(|| {
        document::eval(KEYBOARD_JS);
    });
}

#[cfg(not(any(target_os = "android", target_os = "ios")))]
pub fn use_keyboard() {}

/// Neither platform resizes the layout viewport for the keyboard - WKWebView scrolls the page, an
/// edge-to-edge Android window is never resized - so a `position: fixed` sheet has two edges to
/// correct, read off `visualViewport`:
///
/// - `kb`, the strip hidden at the bottom: `layout - visual - offsetTop`, measured against a fixed
///   `top: 0; bottom: 0` probe, never `innerHeight` (iOS 26 reports it net of the scroll). Below
///   80px the shrink is a URL bar or rounding, not a keyboard, judged before the subtraction: a
///   pan can absorb the whole keyboard and must not read as none.
/// - `vvo`, the visible band's scroll: WebKit pans by the whole keyboard when the field starts
///   below its top, leaving `kb` 0 while the sheet's top edge sits `vvo` above the screen.
///
/// They go into inline styles on the open boxes that keep `var()` for everything else, so the
/// offline bar, Dynamic Type and the safe areas stay live, and are removed at rest so `main.css`
/// is the only resting source. A box mounting with the keyboard up is styled from `sheet_drag.rs`'s
/// scan, through `restyle`.
///
/// `reveal` scrolls the focused field into view inside its box's scrollers only. `scrollIntoView`
/// also scrolls the root, which on these windows pans the visual viewport and drags the whole
/// sheet with it.
///
/// Motion: Android's keyboard slides for 285 ms at 1x while the resize reports its end in one step.
/// `MainActivity.kt` hands `motion` the slide's duration and curve just before that resize, which
/// then animates the open sheet instead of snapping it: the box by the share its growth takes, the
/// rows below its scroller by the rest. Web Animations transforms, so a busy main thread cannot
/// stall them, and the layout is committed so that a late frame lands where the pixels already
/// are. The two arrive in either order (the signal first in 8 of 9 moves on a Fairphone 3), so once
/// a signal has been seen a show or hide waits up to 60 ms for it. With 0 ms (animations off) or no
/// signal at all (iOS, Android 10 and older) the sheet snaps, as before.
/// See docs/plans/keyboard-motion.md.
///
/// A sheet still sliding in (`sheet-in` in `main.css`) is styled without a glide: the slide's
/// `translate: 0 100%` is a share of the box's own height, so the rect the glide measures moves
/// with the padding it adds. A field focused by script inside it is blurred and focused again at
/// the slide's end, so the keyboard rises over a sheet at rest; a focus within 500 ms of a touch
/// is the user's and is kept.
///
/// Dismissal is `.onDrag` plus tap-outside, only while `up()`: a field focused outside a gesture
/// has the focus but, on iOS, no keyboard. A touch that starts outside the focused field and moves
/// 8px mostly vertically, or a tap on something not interactive, moves the focus off it - to its
/// dialog when there is one, so a screen reader stays inside.
#[cfg_attr(not(any(target_os = "android", target_os = "ios")), allow(dead_code))]
const KEYBOARD_JS: &str = r#"
(function () {
  if (window._countedKeyboard) return;
  var vv = window.visualViewport;
  if (!vv) return;

  var MIN_KEYBOARD = 80;
  var SLOP = 8;
  var MARGIN = 20;
  var INTERACTIVE = 'input, select, textarea, button, a, label, summary, [contenteditable], '
    + '[tabindex]:not([tabindex="-1"]), [role=button], [role=radio], [role=tab], [role=option], '
    + '[role=menuitem], [role=switch], [role=checkbox], [role=link]';
  var narrow = window.matchMedia('(max-width: 639.98px)');
  var probe = document.createElement('div');
  probe.style.cssText = 'position:fixed;top:0;bottom:0;left:0;width:0;visibility:hidden;pointer-events:none';
  document.body.appendChild(probe);
  var kb = 0, vvo = 0, touching = false, lastTouch = 0, g = null, spec = null, glides = [], signalled = false, waiting = null;

  function up() { return kb > 0 || vvo > 0; }

  function entering(box) {
    return !!box.getAnimations && box.getAnimations().some(function (a) {
      return a.animationName === 'sheet-in' && a.playState === 'running';
    });
  }

  function afterSlide(box, then) {
    var ran = false;
    function run(e) {
      if (ran || (e && e.target !== box)) return;
      ran = true;
      box.removeEventListener('animationend', run);
      box.removeEventListener('animationcancel', run);
      then();
    }
    box.addEventListener('animationend', run);
    box.addEventListener('animationcancel', run);
    setTimeout(run, 700);
  }

  function holdFocus(e) {
    var field = e.target, box = field.closest ? field.closest('.modal-box') : null;
    if (!box || Date.now() - lastTouch < 500 || !window._countedTextLike || !window._countedTextLike(field) || !entering(box)) return false;
    field.blur();
    afterSlide(box, function () {
      var a = document.activeElement;
      if (field.isConnected && (!a || a === document.body || a === box.closest('[role=dialog]'))) field.focus({ preventScroll: true });
    });
    return true;
  }

  function motion(ms, curve) {
    signalled = true;
    spec = { ms: ms, curve: curve, at: Date.now() };
    if (waiting) apply();
  }

  function scroller(box) {
    var level = [box];
    for (var depth = 0; depth < 6 && level.length; depth++) {
      var next = [];
      for (var i = 0; i < level.length; i++) {
        for (var c = level[i].firstElementChild; c; c = c.nextElementSibling) {
          var o = getComputedStyle(c).overflowY;
          if (o === 'auto' || o === 'scroll') return c;
          next.push(c);
        }
      }
      level = next;
    }
    return null;
  }

  function footer(box) {
    var out = [];
    for (var el = scroller(box); el && el !== box; el = el.parentElement) {
      for (var n = el.nextElementSibling; n; n = n.nextElementSibling) out.push(n);
    }
    return out;
  }

  function settle() {
    var running = glides;
    glides = [];
    running.forEach(function (x) { x.done(); });
  }

  function glide(box, k0, s) {
    var modal = box.parentElement;
    if (!modal || !modal.classList.contains('modal-open') || !modal.classList.contains('modal-bottom') || !narrow.matches || entering(box)) {
      style(box);
      return;
    }
    var sab = parseFloat(getComputedStyle(box).getPropertyValue('--sab')) || 0;
    var f0 = Math.max(sab, k0), span = Math.max(sab, kb) - f0;
    var p0 = parseFloat(getComputedStyle(box).paddingBottom), top0 = box.getBoundingClientRect().top;
    style(box);
    var d = parseFloat(getComputedStyle(box).paddingBottom) - p0;
    if (!span || !(Math.abs(d) >= 1)) return;
    var top1 = box.getBoundingClientRect().top;
    var g = d > 0 ? Math.min(d, Math.max(0, top0 - top1)) : Math.min(-d, Math.max(0, top1 - top0));
    var rest = Math.abs(d) - g;
    var foot = rest > 0.5 ? footer(box) : [];
    if (rest > 0.5 && !foot.length) return;
    var committed = d;
    if (d > 0 && rest > 0.5) {
      committed = g;
      box.style.paddingBottom = (p0 + g) + 'px';
    }
    var cb = d > 0 ? -g : g, cf = -committed, n = s.curve.length, kfBox = [], kfFoot = [];
    for (var i = 0; i < n; i++) {
      var w = (Math.max(sab, k0 + (kb - k0) * s.curve[i]) - f0) / span * d;
      var tb = (d > 0 ? -Math.min(w, g) : Math.max(0, -w - rest)) - cb;
      kfBox.push({ transform: 'translateY(' + tb + 'px)', offset: i / (n - 1) });
      kfFoot.push({ transform: 'translateY(' + (-w - cf - tb) + 'px)', offset: i / (n - 1) });
    }
    var opts = { duration: s.ms, easing: 'linear', fill: 'forwards' };
    var anims = [box.animate(kfBox, opts)], bg = getComputedStyle(box).backgroundColor, saved = [];
    foot.forEach(function (el) {
      saved.push([el, el.style.backgroundColor]);
      el.style.backgroundColor = bg;
      anims.push(el.animate(kfFoot, opts));
    });
    var start = document.timeline.currentTime;
    anims.forEach(function (a) { a.startTime = start; });
    var x = { done: function () {
      if (x.over) return;
      x.over = true;
      anims.forEach(function (a) { a.cancel(); });
      saved.forEach(function (sv) { sv[0].style.backgroundColor = sv[1]; });
      box._countedGlide = null;
      style(box);
      if (up()) reveal();
    } };
    box._countedGlide = x;
    glides.push(x);
    anims[0].onfinish = x.done;
  }

  function measure() {
    var layout = probe.getBoundingClientRect().height, visual = vv.height, off = vv.offsetTop;
    if (!isFinite(layout) || !isFinite(visual) || !isFinite(off)) return [0, 0];
    var raw = layout - visual;
    var k = raw < MIN_KEYBOARD ? 0 : Math.max(0, raw - off);
    return [Math.round(k * 10) / 10, off > 0 ? Math.round(off * 10) / 10 : 0];
  }

  function style(box) {
    if (!up()) {
      box.style.removeProperty('max-height');
      box.style.removeProperty('padding-bottom');
      return;
    }
    box.style.maxHeight = 'calc(100dvh - ' + vvo + 'px - var(--sat) - var(--offline-bar) - var(--sab) - 1rem)';
    if (narrow.matches && box.parentElement && box.parentElement.classList.contains('modal-bottom')) {
      box.style.paddingBottom = 'max(calc(var(--sab) - ' + vvo + 'px), ' + kb + 'px)';
    } else {
      box.style.removeProperty('padding-bottom');
    }
  }

  function restyle() {
    document.querySelectorAll('.modal > .modal-box').forEach(function (box) {
      if (!box._countedGlide) style(box);
    });
  }

  function reveal() {
    if (touching) return;
    var a = document.activeElement;
    var box = a && a.closest ? a.closest('.modal-box') : null;
    if (!box || a === box) return;
    for (var el = a.parentElement; el && el !== box; el = el.parentElement) {
      if (el.scrollHeight <= el.clientHeight) continue;
      var overflow = getComputedStyle(el).overflowY;
      if (overflow !== 'auto' && overflow !== 'scroll') continue;
      var r = a.getBoundingClientRect(), s = el.getBoundingClientRect();
      if (r.bottom + MARGIN > s.bottom) el.scrollTop += Math.min(r.bottom + MARGIN - s.bottom, r.top - MARGIN - s.top);
      else if (r.top - MARGIN < s.top) el.scrollTop -= s.top - (r.top - MARGIN);
    }
  }

  function fresh() { return spec && Date.now() - spec.at < 250 ? spec : null; }

  function update() {
    var m = measure();
    if (m[0] === kb && m[1] === vvo) return;
    if (signalled && !fresh() && !m[1] && !vvo && (kb === 0) !== (m[0] === 0)) {
      if (!waiting) waiting = setTimeout(apply, 60);
      return;
    }
    apply();
  }

  function apply() {
    if (waiting) clearTimeout(waiting);
    waiting = null;
    var m = measure();
    if (m[0] === kb && m[1] === vvo) return;
    var k0 = kb, s = fresh();
    spec = null;
    settle();
    kb = m[0];
    vvo = m[1];
    if (s && s.ms > 0 && !vvo) {
      document.querySelectorAll('.modal > .modal-box').forEach(function (box) { glide(box, k0, s); });
    } else {
      restyle();
    }
    if (up()) reveal();
  }

  function typing() {
    var a = document.activeElement;
    return a && window._countedTextLike && window._countedTextLike(a) ? a : null;
  }

  function dismiss(field) {
    if (document.activeElement !== field) return;
    var dialog = field.closest('[role=dialog]');
    if (dialog) dialog.focus({ preventScroll: true });
    if (document.activeElement === field) field.blur();
  }

  vv.addEventListener('resize', update);
  vv.addEventListener('scroll', update);
  narrow.addEventListener('change', restyle);
  document.addEventListener('focusin', function (e) {
    if (!holdFocus(e)) requestAnimationFrame(reveal);
  }, true);

  document.addEventListener('touchstart', function (e) {
    touching = true;
    g = null;
    var field = typing();
    if (!up() || !field || e.touches.length !== 1 || !(e.target instanceof Element)) return;
    if ((field.closest('label, .input') || field).contains(e.target)) return;
    var t = e.touches[0];
    g = { field: field, target: e.target, x: t.clientX, y: t.clientY, moved: false };
  }, { capture: true, passive: true });

  document.addEventListener('touchmove', function (e) {
    if (!g || g.moved || e.touches.length !== 1) return;
    var t = e.touches[0];
    var dx = Math.abs(t.clientX - g.x), dy = Math.abs(t.clientY - g.y);
    if (dx >= SLOP && dx > dy) { g = null; return; }
    if (dy < SLOP || dx > dy) return;
    g.moved = true;
    dismiss(g.field);
  }, { capture: true, passive: true });

  function end(e) {
    touching = e.touches.length > 0;
    lastTouch = Date.now();
    var s = g;
    g = null;
    if (e.type !== 'touchend' || !s || s.moved || s.target.closest(INTERACTIVE)) return;
    dismiss(s.field);
  }
  document.addEventListener('touchend', end, { capture: true, passive: true });
  document.addEventListener('touchcancel', end, { capture: true, passive: true });

  window._countedKeyboard = { up: up, reveal: reveal, restyle: restyle, motion: motion };
  update();
})();
"#;

#[cfg(test)]
mod tests {
    use super::KEYBOARD_JS;

    #[test]
    fn never_scrolls_through_the_root() {
        assert!(!KEYBOARD_JS.contains("scrollIntoView"));
    }

    #[test]
    fn writes_no_custom_property_on_the_root() {
        assert!(!KEYBOARD_JS.contains("documentElement.style"));
        assert!(!KEYBOARD_JS.contains("setProperty('--"));
    }

    #[test]
    fn never_writes_an_inline_transition() {
        assert!(!KEYBOARD_JS.contains("transition"));
    }

    #[test]
    fn touch_listeners_never_block_scrolling() {
        assert!(!KEYBOARD_JS.contains("preventDefault"));
        assert!(!KEYBOARD_JS.contains("passive: false"));
    }
}
