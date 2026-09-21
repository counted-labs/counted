use dioxus::prelude::*;

use crate::route::Route;

/// Which way a navigation moves through the app's hierarchy. Drives the CSS in
/// `packages/mobile/assets/main.css`; see docs/mobile-navigation.md.
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
/// without a single one of them knowing about animations. The match is exhaustive on purpose — a
/// new route must not silently default to a depth.
pub fn route_depth(route: &Route) -> u8 {
    match route {
        Route::ProjectsPage {}
        | Route::ChartsPage {}
        | Route::SettingsPage {}
        | Route::LoginPage {}
        | Route::VerifyEmailPage { .. }
        | Route::NotFoundPage { .. } => 0,
        Route::ExpensesPage { .. }
        | Route::FriendsPage {}
        | Route::RegisterPage {}
        | Route::HelpPage {}
        | Route::PrivacyPage {}
        | Route::TermsPage {}
        | Route::LegalNoticePage {} => 1,
        Route::PaymentPage { .. } | Route::ProjectHistoryPage { .. } => 2,
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
/// shell is the only node whose creation lines up exactly with a navigation — `.app-container`
/// cannot carry the animation because `ExpensesPage` swaps between two different `.app-container`
/// roots (key-missing vs. loaded) and would replay its entrance mid-view. A changed key on a
/// one-item fragment makes `diff_keyed_middle` remove the old node and create the new one, which is
/// both the animation trigger and the removal the exit observer below listens for.
///
/// **It carries no `position` and no `will-change`.** The transform during the animation makes it a
/// containing block for `position: fixed` descendants — the `SpeedDialFab`, the dropdown scrims —
/// which is what makes them slide with the page. Anything that made that permanent would re-anchor
/// them to the shell forever. It is full-height and opaque (`main.css`) for the same descendants:
/// their `bottom:` resolves from the shell's edge while it is transformed, and the incoming shell
/// has to cover the dimmed exit overlay beneath it.
#[cfg(any(target_os = "android", target_os = "ios"))]
#[component]
pub fn PageTransition() -> Element {
    let route = use_route::<Route>();

    // Not a signal: writing one during render would loop, and `use_route` already re-runs this body
    // on every navigation. `None` is the first mount — a cold start or a deep link opened cold must
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

    let plat = if cfg!(target_os = "ios") { "ios" } else { "android" };

    use_effect(|| {
        document::eval(EXIT_ANIMATION_JS);
    });

    rsx! {
        for k in [route.to_string()] {
            div {
                key: "{k}",
                class: "page-shell w-full flex flex-col items-center",
                "data-nav": dir,
                "data-plat": plat,
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
///   the DOM looking for that attribute, and Dioxus recycles ids it has freed — a stale one left in
///   this tree could route an event to the wrong scope. `inert` and `pointer-events: none` are the
///   other two layers.
/// - **DOM order, never `z-index`, decides who paints on top.** New page over old on push, old over
///   new on pop, exactly like UIKit. A `z-index` on the shell would make it a permanent stacking
///   context and trap the in-page FAB (`z-40`) and every modal under the dock (`z-50`).
/// - **`scrollTop` is restored.** Detaching a node destroys its layout box and zeroes it, so a page
///   left mid-list would visibly snap to the top as it slides away.
/// - **An overlay's own removal is ignored.** It still carries `page-shell`, so without the
///   `page-exiting` guard the observer adopts it as a fresh outgoing page: re-inserting it restarts
///   the animation and schedules another `animationend` and another 600ms timer, each of which
///   removes it and is adopted again. The pending timers then double every 600ms and pin the
///   WebView's JS thread within seconds — and since Dioxus applies its mutations from that same
///   thread, the Rust side keeps rendering into a DOM that never updates.
///
/// The mutations arrive inside `requestAnimationFrame` (the interpreter's `rafEdits`), and this
/// callback is a microtask after that flush, so the overlay lands in the same frame — no flash.
#[cfg(any(target_os = "android", target_os = "ios"))]
const EXIT_ANIMATION_JS: &str = r#"
if (!window._countedExitAnimRegistered) {
  window._countedExitAnimRegistered = true;
  requestAnimationFrame(function () {
    var host = document.querySelector('main');
    if (!host) { window._countedExitAnimRegistered = false; return; }

    var lastScroll = null;
    document.addEventListener('scroll', function (e) {
      if (e.target instanceof Element) lastScroll = { el: e.target, top: e.target.scrollTop };
    }, { capture: true, passive: true });

    var reduce = window.matchMedia('(prefers-reduced-motion: reduce)');

    new MutationObserver(function (records) {
      if (reduce.matches) return;
      for (var i = 0; i < records.length; i++) {
        var removed = records[i].removedNodes;
        for (var j = 0; j < removed.length; j++) {
          var old = removed[j];
          if (!(old instanceof Element) || !old.classList.contains('page-shell')) continue;
          if (old.classList.contains('page-exiting')) continue;

          var live = host.querySelector(':scope > .page-shell:not(.page-exiting)');
          if (!live) continue;
          var dir = live.getAttribute('data-nav') || 'none';
          var plat = live.getAttribute('data-plat') || 'android';
          if (dir === 'none') continue;
          if (plat === 'ios' && dir === 'lateral') continue;

          var stale = host.querySelector('.page-exiting');
          if (stale) stale.remove();

          old.removeAttribute('data-dioxus-id');
          var ids = old.querySelectorAll('[data-dioxus-id]');
          for (var k = 0; k < ids.length; k++) ids[k].removeAttribute('data-dioxus-id');
          old.inert = true;
          old.setAttribute('aria-hidden', 'true');
          old.setAttribute('data-nav', dir);
          old.classList.add('page-exiting');

          if (dir === 'back') host.appendChild(old); else host.insertBefore(old, live);
          if (lastScroll && old.contains(lastScroll.el)) lastScroll.el.scrollTop = lastScroll.top;

          (function (node) {
            node.addEventListener('animationend', function (e) {
              if (e.target === node) node.remove();
            });
            setTimeout(function () { node.remove(); }, 600);
          })(old);
        }
      }
    }).observe(host, { childList: true });
  });
}
"#;

/// No-op passthrough for web and desktop — no wrapper, no keyed remount, nothing changes.
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
    /// the same animation — neither of them tells us it is a back press.
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
