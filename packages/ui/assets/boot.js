(function () {
  var dark = matchMedia('(prefers-color-scheme: dark)');
  var chosen;
  function stored() {
    try {
      return JSON.parse(localStorage.getItem('counted_theme'));
    } catch (e) {
      return null;
    }
  }
  function paint() {
    var t = chosen || stored();
    var isDark = t === 'dark' || (t === 'system' && dark.matches);
    document.documentElement.dataset.theme = isDark ? 'counted-dark' : 'counted';
    var meta = document.querySelector('meta[name="theme-color"]');
    if (meta) meta.content = isDark ? '#1B221F' : '#F4F8F6';
  }
  window.countedTheme = function (t) {
    chosen = t;
    paint();
  };
  dark.addEventListener('change', paint);
  paint();
  document.addEventListener('touchstart', function () {}, { passive: true });
  // Capture, because `scroll` does not bubble. Scoped to the element that scrolled: mid-transition
  // the exiting page overlay holds a header too, and only the live one may change.
  // On a tab root the large title hands over to the bar's own: once a frame, `--large-title-out`
  // says how far it has slid under the bar (0 to 1) and `.is-collapsed` that it is all the way.
  // A hidden large title (every platform but iOS) has no height and is left alone.
  var titleFrame = 0;
  document.addEventListener('scroll', function (e) {
    var t = e.target;
    if (!t || t.nodeType !== 1) return;
    var bar = t.querySelector('.nav-material');
    if (!bar) return;
    bar.classList.toggle('is-scrolled', t.scrollTop > 2);
    if (titleFrame || !bar.hasAttribute('data-large')) return;
    titleFrame = requestAnimationFrame(function () {
      titleFrame = 0;
      var large = t.querySelector('.large-title');
      var r = large && large.getBoundingClientRect();
      if (!r || !r.height) return;
      var out = Math.min(1, Math.max(0, (bar.getBoundingClientRect().bottom - r.top) / r.height));
      out = Math.round(out * 100) / 100;
      if (bar._titleOut === out) return;
      bar._titleOut = out;
      bar.style.setProperty('--large-title-out', out);
      large.style.setProperty('--large-title-out', out);
      bar.classList.toggle('is-collapsed', out === 1);
    });
  }, { capture: true, passive: true });
  // The iOS toast is a banner: it follows a drag (down only grudgingly) and a release 24px up
  // dismisses it through its own click, the one event Rust sees. `transform`, because the CSS
  // entrance and exit own `translate`.
  var banner = null;
  document.addEventListener('pointerdown', function (e) {
    if (document.documentElement.dataset.plat !== 'ios' || !(e.target instanceof Element)) return;
    var el = e.target.closest('.app-toast:not([data-leaving])');
    if (!el) return;
    banner = { el: el, id: e.pointerId, y: e.clientY };
    el.style.transition = 'none';
  }, { passive: true });
  document.addEventListener('pointermove', function (e) {
    if (!banner || e.pointerId !== banner.id) return;
    var dy = e.clientY - banner.y;
    banner.el.style.transform = 'translateY(' + (dy < 0 ? dy : dy / 4) + 'px)';
  }, { passive: true });
  function dropBanner(e) {
    if (!banner || e.pointerId !== banner.id) return;
    var b = banner;
    banner = null;
    b.el.style.transition = '';
    if (e.type === 'pointerup' && e.clientY - b.y < -24) {
      b.el.click();
    } else {
      b.el.style.transition = 'transform 200ms ease-out, translate 200ms ease-in, opacity 200ms ease-in';
      b.el.style.transform = '';
    }
  }
  document.addEventListener('pointerup', dropBanner, { passive: true });
  document.addEventListener('pointercancel', dropBanner, { passive: true });
  // Long press on a row: `data-long-press` names a hidden button. A finger held 500ms within 10px
  // clicks it, the one event Rust sees (a Medium haptic and the row's actions sheet). The click the
  // release may then synthesise is eaten, and `_countedLongPressed` tells the row swipe to let go.
  // A control inside the row keeps its own press.
  var press = null;
  window._countedLongPressed = false;
  function dropPress() {
    if (press) clearTimeout(press.timer);
    press = null;
  }
  document.addEventListener('pointerdown', function (e) {
    dropPress();
    window._countedLongPressed = false;
    if (e.pointerType !== 'touch' || !e.isPrimary || !(e.target instanceof Element)) return;
    var row = e.target.closest('[data-long-press]');
    if (!row) return;
    var control = e.target.closest('button, a, input, select, textarea, label');
    if (control && control !== row && row.contains(control)) return;
    var target = document.getElementById(row.getAttribute('data-long-press'));
    if (!target) return;
    press = { id: e.pointerId, x: e.clientX, y: e.clientY, timer: setTimeout(function () {
      press = null;
      target.click();
      window._countedLongPressed = true;
    }, 500) };
  }, { capture: true, passive: true });
  document.addEventListener('pointermove', function (e) {
    if (press && e.pointerId === press.id && Math.max(Math.abs(e.clientX - press.x), Math.abs(e.clientY - press.y)) > 10) dropPress();
  }, { capture: true, passive: true });
  document.addEventListener('pointerup', dropPress, { capture: true, passive: true });
  document.addEventListener('pointercancel', dropPress, { capture: true, passive: true });
  document.addEventListener('click', function (e) {
    if (!window._countedLongPressed || !e.isTrusted) return;
    window._countedLongPressed = false;
    e.preventDefault();
    e.stopPropagation();
  }, true);
  document.addEventListener('contextmenu', function (e) {
    if (press || window._countedLongPressed) e.preventDefault();
  }, true);
})();
