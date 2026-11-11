//! The on-screen keyboard and the bottom sheets: their inset, revealing the focused field, and
//! dismissing the keyboard by touching above it. See docs/styling.md, "Pitfall: a `position:
//! fixed` sheet and the on-screen keyboard", docs/plans/sheet-keyboard.md and, for the iOS
//! prediction, docs/plans/ios-sheets-keyboard.md.
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
/// sheet with it. It runs again when the box's content changes under a focused field (a
/// participant row added above it), from the shared observer.
///
/// Motion: Android's keyboard slides for 285 ms at 1x while the resize reports its end in one step.
/// `MainActivity.kt` hands `motion` the slide's duration and curve just before that resize, which
/// then animates the open sheet instead of snapping it: the box by the share its growth takes, the
/// rows below its scroller by the rest. Web Animations transforms, so a busy main thread cannot
/// stall them, and the layout is committed so that a late frame lands where the pixels already
/// are. The two arrive in either order (the signal first in 8 of 9 moves on a Fairphone 3), so once
/// a signal has been seen a show or hide waits up to 60 ms for it. With 0 ms (animations off) or no
/// signal at all (Android 10 and older) the sheet snaps, as before.
/// See docs/plans/keyboard-motion.md.
///
/// iOS has its signal before the keyboard moves: `UIKeyboardWillChangeFrameNotification`, which
/// `packages/mobile/src/main.rs` forwards as `willChange(kb, ms, curve, …)`. It becomes a
/// **prediction**: the end layout is applied at once and the same glide carries the pixels, with
/// a curve of the app's own (UIKit's is private). A prediction is kept apart from the measured
/// pair: `up()` reads the measured one only, so no gesture arms on a guess; when the `resize`
/// confirms it the prediction is adopted and the glide finishes, when it differs the measured
/// value wins at once, and one no event confirms is reverted after `ms + 150`.
///
/// WebKit's own reveal cannot be laid out around: on an iPhone every focus of a text field centres
/// it in the band above the keyboard, visible or not (`_zoomToFocusRect` with `forceScroll`, which
/// is `requiresAccessoryView`, true for text fields on the phone idiom; measured as a 212 px pan
/// for a field at 478 px in a 566 px band), and the one thing it honours is `preventScroll` on the
/// focus call. A later `focus({ preventScroll: true })` is too late: the tap's own focus has
/// already queued the reveal, and WebKit's mouse path refocuses an already focused field without
/// the flag. So on a phone the first tap on a sheet field is focused by this script: the
/// `mousedown`, whose default is that focus, is cancelled, and `focus({ preventScroll: true })`
/// runs inside the same gesture, so the keyboard rises and the page never pans. The caret lands at
/// the end, as in a native field; a tap on a field already focused is left to WebKit. A tap on a
/// `label` around the field is resolved through `label.control`, and the label's click is
/// cancelled so its activation never focuses again. Script focuses into a sheet field carry the
/// same flag (`enter_next.rs`). Everything iOS-specific is behind
/// `window._countedKeyboardFlags`, which a diagnostic build drives at runtime; the defaults are in
/// `flag()`'s call sites. `window._countedKeyboardTrace`, when defined, receives every step.
///
/// A sheet still sliding in (`sheet-in` in `main.css`) is styled without a glide: the slide's
/// `translate: 0 100%` is a share of the box's own height, so the rect the glide measures moves
/// with the padding it adds. A field focused by script inside it keeps that focus: it arrives
/// through a native eval, which WebKit counts as a user gesture, so the keyboard rises with the
/// slide. Holding it to the slide's end was measured wrong on 2026-11-10: the blur cancelled a
/// keyboard already rising at every retry of the amount field's mount focus, and the refocus at
/// the slide's end came from this script, outside a gesture, so it landed with no keyboard.
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
  var SIGNAL_WAIT = 60;
  var REVERT_GRACE = 150;
  var SAMPLES = 17;
  var INTERACTIVE = 'input, select, textarea, button, a, label, summary, [contenteditable], '
    + '[tabindex]:not([tabindex="-1"]), [role=button], [role=radio], [role=tab], [role=option], '
    + '[role=menuitem], [role=switch], [role=checkbox], [role=link]';
  var narrow = window.matchMedia('(max-width: 639.98px)');
  var reduce = window.matchMedia('(prefers-reduced-motion: reduce)');
  var ios = document.documentElement.dataset.plat === 'ios';
  var phone = ios && Math.min(screen.width, screen.height) < 600;
  var probe = document.createElement('div');
  probe.style.cssText = 'position:fixed;top:0;bottom:0;left:0;width:0;visibility:hidden;pointer-events:none';
  document.body.appendChild(probe);
  var kb = 0, vvo = 0, pred = null, touching = false, g = null, spec = null, glides = [], signalled = false, waiting = null, raf = 0, revealQueued = false;

  function flag(name, dflt) { var f = window._countedKeyboardFlags; return f && f[name] !== undefined ? f[name] : dflt; }
  function trace(ev, detail) { var t = window._countedKeyboardTrace; if (t) t(ev, detail === undefined ? '' : String(detail)); }
  function textLike(el) { return !!el && !!window._countedTextLike && window._countedTextLike(el); }
  function gestures() { return !!flag('gestures', true); }
  function boxes() { return document.querySelectorAll('.modal > .modal-box'); }

  function up() { return kb > 0 || vvo > 0; }
  function eff() { return pred ? [pred.kb, 0] : [kb, vvo]; }

  function entering(box) {
    return !!box.getAnimations && box.getAnimations().some(function (a) {
      return a.animationName === 'sheet-in' && a.playState === 'running';
    });
  }

  function motion(ms, curve) {
    signalled = true;
    spec = { ms: ms, curve: curve, at: Date.now() };
    if (waiting) apply();
  }

  function bezier(x1, y1, x2, y2) {
    function at(a, b, t) { return 3 * a * (1 - t) * (1 - t) * t + 3 * b * (1 - t) * t * t + t * t * t; }
    return function (x) {
      var t = x;
      for (var i = 0; i < 8; i++) {
        var d = at(x1, x2, t) - x;
        if (Math.abs(d) < 1e-4) break;
        var slope = 3 * (1 - t) * (1 - t) * x1 + 6 * (1 - t) * t * (x2 - x1) + 3 * t * t * (1 - x2);
        if (!slope) break;
        t -= d / slope;
      }
      return at(y1, y2, t);
    };
  }
  function spring(t) {
    var w = 7, end = 1 - Math.exp(-w) * (1 + w);
    return (1 - Math.exp(-w * t) * (1 + w * t)) / end;
  }
  var PRESETS = {
    spring: { f: spring, ms: 500 },
    bezA: { f: bezier(0.38, 0.7, 0.125, 1), ms: 0 },
    bezB: { f: bezier(0.25, 0.1, 0.25, 1), ms: 0 }
  };
  function sample(f) {
    var out = [];
    for (var i = 0; i < SAMPLES; i++) out.push(f(i / (SAMPLES - 1)));
    return out;
  }
  function signalSpec(ms) {
    if (reduce.matches) return null;
    var preset = PRESETS[flag('curve', 'spring')] || PRESETS.spring;
    var dur = Number(flag('ms', 0)) || preset.ms || ms;
    return dur > 0 ? { ms: dur, curve: sample(preset.f) } : null;
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

  function style(box, k, o) {
    if (!(k > 0 || o > 0)) {
      box.style.removeProperty('max-height');
      box.style.removeProperty('padding-bottom');
      trace('style', box.id + ' rest');
      return;
    }
    box.style.maxHeight = 'calc(100dvh - ' + o + 'px - var(--sat) - var(--offline-bar) - var(--sab) - 1rem)';
    if (narrow.matches && box.parentElement && box.parentElement.classList.contains('modal-bottom')) {
      box.style.paddingBottom = 'max(calc(var(--sab) - ' + o + 'px), ' + k + 'px)';
    } else {
      box.style.removeProperty('padding-bottom');
    }
    trace('style', box.id + ' ' + k + ' ' + o);
  }

  function restyle() {
    var e = eff();
    boxes().forEach(function (box) {
      if (!box._countedGlide) style(box, e[0], e[1]);
    });
  }

  // The pixel motion of `box` from its layout at `k0` to its layout at `k1`: the layout is
  // committed, the pixels ride a WAAPI transform.
  function glide(box, k0, k1, s) {
    var modal = box.parentElement;
    if (!modal || !modal.classList.contains('modal-open') || !modal.classList.contains('modal-bottom') || !narrow.matches || entering(box)) {
      style(box, k1, 0);
      return;
    }
    var sab = parseFloat(getComputedStyle(box).getPropertyValue('--sab')) || 0;
    var f0 = Math.max(sab, k0), span = Math.max(sab, k1) - f0;
    var p0 = parseFloat(getComputedStyle(box).paddingBottom), top0 = box.getBoundingClientRect().top;
    style(box, k1, 0);
    var d = parseFloat(getComputedStyle(box).paddingBottom) - p0;
    if (!span || !(Math.abs(d) >= 1)) return;
    var top1 = box.getBoundingClientRect().top;
    var grow = d > 0 ? Math.min(d, Math.max(0, top0 - top1)) : Math.min(-d, Math.max(0, top1 - top0));
    var rest = Math.abs(d) - grow;
    var foot = rest > 0.5 ? footer(box) : [];
    if (rest > 0.5 && !foot.length) return;
    var committed = d;
    if (d > 0 && rest > 0.5) {
      committed = grow;
      box.style.paddingBottom = (p0 + grow) + 'px';
    }
    var cb = d > 0 ? -grow : grow, cf = -committed;
    var bg = getComputedStyle(box).backgroundColor, saved = [], anims = [];
    function frames(curve) {
      var n = curve.length, kfBox = [], kfFoot = [];
      for (var i = 0; i < n; i++) {
        var w = (Math.max(sab, k0 + (k1 - k0) * curve[i]) - f0) / span * d;
        var tb = (d > 0 ? -Math.min(w, grow) : Math.max(0, -w - rest)) - cb;
        kfBox.push({ transform: 'translateY(' + tb + 'px)', offset: i / (n - 1) });
        kfFoot.push({ transform: 'translateY(' + (-w - cf - tb) + 'px)', offset: i / (n - 1) });
      }
      return [kfBox, kfFoot];
    }
    function stop() {
      anims.forEach(function (a) { a.cancel(); });
      anims = [];
    }
    var x = { over: false };
    x.run = function (sp) {
      stop();
      var kf = frames(sp.curve), opts = { duration: sp.ms, easing: 'linear', fill: 'forwards' };
      anims = [box.animate(kf[0], opts)];
      foot.forEach(function (el) {
        if (!saved.some(function (sv) { return sv[0] === el; })) saved.push([el, el.style.backgroundColor]);
        el.style.backgroundColor = bg;
        anims.push(el.animate(kf[1], opts));
      });
      var start = document.timeline.currentTime;
      anims.forEach(function (a) { a.startTime = start; });
      anims[0].onfinish = x.done;
      trace('glide', box.id + ' ' + k0 + '>' + k1 + ' d=' + d.toFixed(1) + ' ' + sp.ms + 'ms');
    };
    x.done = function () {
      if (x.over) return;
      x.over = true;
      stop();
      saved.forEach(function (sv) { sv[0].style.backgroundColor = sv[1]; });
      box._countedGlide = null;
      var e = eff();
      style(box, e[0], e[1]);
      trace('glide end', box.id);
      if (up()) reveal();
    };
    box._countedGlide = x;
    glides.push(x);
    x.run(s);
  }

  function arm(p, s) {
    clearTimeout(p.timer);
    p.timer = setTimeout(function () { if (pred === p) endPrediction(true); }, Math.max((s && s.ms) || 0, 250) + REVERT_GRACE);
  }

  function endPrediction(revert) {
    if (!pred) return;
    var p = pred;
    pred = null;
    clearTimeout(p.timer);
    if (revert) {
      settle();
      restyle();
      trace('revert', p.source + ' ' + p.kb);
    } else {
      trace('adopt', p.source + ' ' + p.kb);
    }
  }

  function predict(k1, source, s) {
    var k0 = eff()[0];
    if (pred) {
      var p = pred;
      if (Math.abs(p.kb - k1) <= 1) {
        if (source === 'signal') p.source = 'signal';
        arm(p, s);
        trace('predict same', source + ' ' + k1);
        return;
      }
      pred = null;
      clearTimeout(p.timer);
      settle();
    }
    pred = { kb: k1, source: source, at: Date.now(), timer: null };
    trace('predict', source + ' ' + k0 + '>' + k1 + (s ? ' ' + s.ms + 'ms' : ' snap'));
    boxes().forEach(function (box) {
      if (box._countedGlide) box._countedGlide.done();
      if (s && s.ms > 0) glide(box, k0, k1, s); else style(box, k1, 0);
    });
    arm(pred, s);
  }

  function willChange(k, ms, curve, isLocal, appState, nativeMs) {
    trace('willChange', k + ' ' + ms + 'ms c' + curve + ' local=' + isLocal + ' ' + appState + ' n=' + nativeMs);
    if (!ios || !flag('signal', true) || isLocal === false || (appState && appState !== 'active')) return;
    if (!(k >= MIN_KEYBOARD)) k = 0;
    if (vvo) {
      if (k === 0 && !(pred && pred.hide)) predictRest(ms);
      return;
    }
    if (!pred && k === kb) return;
    predict(k, 'signal', signalSpec(ms));
    if (k > 0) reveal();
  }

  // A hide announced while the page is panned: the pan decays over the keyboard's duration and
  // the sheet is put at rest at once, under it, instead of being re-cut at every step.
  function predictRest(ms) {
    if (pred) {
      clearTimeout(pred.timer);
      pred = null;
    }
    settle();
    pred = { kb: 0, source: 'signal', hide: true, at: Date.now(), timer: null };
    trace('predict', 'rest from pan ' + kb + '/' + vvo);
    boxes().forEach(function (box) { style(box, 0, 0); });
    arm(pred, { ms: ms || 250 });
  }

  function tapped(e) {
    var el = e.target instanceof Element ? e.target.closest(INTERACTIVE) : null;
    var field = el && el.tagName === 'LABEL' ? el.control : el;
    if (!field || !textLike(field) || field.disabled || field.readOnly || !field.closest('.modal-box')) return null;
    return field;
  }

  function measure() {
    var layout = probe.getBoundingClientRect().height, visual = vv.height, off = vv.offsetTop;
    if (!isFinite(layout) || !isFinite(visual) || !isFinite(off)) return [0, 0];
    var raw = layout - visual;
    var k = raw < MIN_KEYBOARD ? 0 : Math.max(0, raw - off);
    return [Math.round(k * 10) / 10, off > 0 ? Math.round(off * 10) / 10 : 0];
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
      var r = a.getBoundingClientRect(), s = el.getBoundingClientRect(), before = el.scrollTop;
      if (r.bottom + MARGIN > s.bottom) el.scrollTop += Math.min(r.bottom + MARGIN - s.bottom, r.top - MARGIN - s.top);
      else if (r.top - MARGIN < s.top) el.scrollTop -= s.top - (r.top - MARGIN);
      if (el.scrollTop !== before) trace('reveal', a.id + ' ' + before.toFixed(0) + '>' + el.scrollTop.toFixed(0));
    }
  }

  function fresh() { return spec && Date.now() - spec.at < 250 ? spec : null; }

  function update() {
    var m = measure();
    if (m[0] === kb && m[1] === vvo) return;
    if (signalled && !fresh() && !m[1] && !vvo && (kb === 0) !== (m[0] === 0)) {
      if (!waiting) waiting = setTimeout(apply, SIGNAL_WAIT);
      return;
    }
    apply();
  }

  function schedule() {
    if (!ios || !flag('batch', true)) { update(); return; }
    if (!raf) raf = requestAnimationFrame(function () { raf = 0; update(); });
  }

  function apply() {
    if (waiting) clearTimeout(waiting);
    waiting = null;
    var m = measure();
    if (m[0] === kb && m[1] === vvo) return;
    var k0 = kb, s = fresh();
    spec = null;
    trace('measure', m[0] + ' ' + m[1]);
    if (pred && pred.hide) {
      kb = m[0];
      vvo = m[1];
      if (!up()) { endPrediction(false); return; }
      if (kb === 0) return;
      endPrediction(true);
      reveal();
      return;
    }
    if (pred && m[1] === 0 && Math.abs(m[0] - pred.kb) <= 1) {
      kb = m[0];
      vvo = 0;
      endPrediction(false);
      if (up()) reveal();
      return;
    }
    if (pred) endPrediction(true);
    settle();
    kb = m[0];
    vvo = m[1];
    if (s && s.ms > 0 && !vvo) {
      boxes().forEach(function (box) { glide(box, k0, kb, s); });
    } else {
      restyle();
    }
    if (up()) reveal();
  }

  function typing() {
    var a = document.activeElement;
    return textLike(a) ? a : null;
  }

  function dismiss(field) {
    if (document.activeElement !== field) return;
    trace('dismiss', field.id);
    var dialog = field.closest('[role=dialog]');
    if (dialog) dialog.focus({ preventScroll: true });
    if (document.activeElement === field) field.blur();
  }

  vv.addEventListener('resize', schedule);
  vv.addEventListener('scroll', schedule);
  narrow.addEventListener('change', restyle);
  document.addEventListener('focusin', function () { requestAnimationFrame(reveal); }, true);

  if (phone) document.addEventListener('mousedown', function (e) {
    var field = tapped(e);
    if (!field || document.activeElement === field) return;
    e.preventDefault();
    field.focus({ preventScroll: true });
    trace('own', field.id + ' kept=' + (document.activeElement === field));
  }, true);

  if (phone) document.addEventListener('click', function (e) {
    var field = tapped(e);
    if (field && document.activeElement === field && !field.contains(e.target)) e.preventDefault();
  }, true);

  if (window._countedObserve) window._countedObserve(function (records) {
    if (revealQueued || !up()) return;
    var a = document.activeElement, box = a && a.closest ? a.closest('.modal-box') : null;
    if (!box || !textLike(a)) return;
    for (var i = 0; i < records.length; i++) {
      if (records[i].type === 'childList' && box.contains(records[i].target)) {
        revealQueued = true;
        requestAnimationFrame(function () { revealQueued = false; reveal(); });
        return;
      }
    }
  });

  document.addEventListener('touchstart', function (e) {
    touching = true;
    g = null;
    var field = typing();
    if (!gestures() || !up() || !field || e.touches.length !== 1 || !(e.target instanceof Element)) return;
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
    var s = g;
    g = null;
    if (e.type !== 'touchend' || !s || s.moved || s.target.closest(INTERACTIVE)) return;
    dismiss(s.field);
  }
  document.addEventListener('touchend', end, { capture: true, passive: true });
  document.addEventListener('touchcancel', end, { capture: true, passive: true });

  window._countedKeyboard = {
    up: up, reveal: reveal, restyle: restyle, motion: motion, willChange: willChange, gestures: gestures,
    ping: function (nativeMs) { trace('ping', 'n=' + nativeMs); },
    state: function () { return { kb: kb, vvo: vvo, predicted: pred ? pred.kb : null, source: pred ? pred.source : null }; }
  };
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
        assert!(!KEYBOARD_JS.contains("passive: false"));
        let touch = KEYBOARD_JS.matches("addEventListener('touch").count();
        assert_eq!(touch, 4);
        assert_eq!(KEYBOARD_JS.matches("passive: true").count(), touch);
        assert_eq!(KEYBOARD_JS.matches("preventDefault").count(), 2, "only the owned tap cancels a default");
    }

    #[test]
    fn the_owned_focus_never_scrolls() {
        assert!(KEYBOARD_JS.contains("if (phone) document.addEventListener('mousedown'"));
        assert!(!KEYBOARD_JS.contains(".focus()"));
        assert!(KEYBOARD_JS.matches("focus({ preventScroll: true })").count() >= 2);
    }

    #[test]
    fn a_script_focus_in_a_sliding_sheet_is_kept() {
        assert!(KEYBOARD_JS.contains("addEventListener('focusin', function () { requestAnimationFrame(reveal); }, true)"));
        assert_eq!(KEYBOARD_JS.matches(".blur()").count(), 1, "only dismiss() blurs");
        assert!(!KEYBOARD_JS.contains("animationend"));
    }

    #[test]
    fn the_prediction_never_arms_a_gesture() {
        let up = KEYBOARD_JS.find("function up()").expect("up() is defined");
        let line = &KEYBOARD_JS[up..KEYBOARD_JS[up..].find('\n').unwrap() + up];
        assert!(!line.contains("pred"), "up() reads the measured pair only: {line}");
    }

    #[test]
    fn ios_behaviour_is_gated_and_switchable() {
        assert!(KEYBOARD_JS.contains("dataset.plat === 'ios'"));
        for flag in ["'signal'", "'batch'", "'gestures'", "'curve'"] {
            assert!(KEYBOARD_JS.contains(&format!("flag({flag}")), "{flag} is a runtime switch");
        }
    }

    #[test]
    fn never_reads_a_field_value_or_storage() {
        assert!(!KEYBOARD_JS.contains(".value"));
        assert!(!KEYBOARD_JS.contains("localStorage"));
    }
}
