//! After a navigation the trigger is usually gone and focus falls to `<body>`, where a screen
//! reader says nothing and the next swipe starts at the top. Focus then goes to the new page's
//! `h1`, skipping the outgoing page that `page_transition.rs` keeps on screen as an `inert`
//! overlay. A focus that survived the navigation - a dock button - is left alone.
//!
//! JS twin for the native WebViews, `web_dom::focus_page_heading` for the browser build.

use std::cell::RefCell;
use std::rc::Rc;

use dioxus::prelude::*;

use crate::route::Route;

#[cfg(any(target_os = "android", target_os = "ios"))]
const FOCUS_HEADING_JS: &str = r#"
const active = document.activeElement;
if (active && active !== document.body && active.isConnected && !active.closest('[inert]')) return true;
const heading = Array.from(document.querySelectorAll('h1')).find(h => !h.closest('[inert]'));
if (!heading) return false;
if (!heading.hasAttribute('tabindex')) heading.setAttribute('tabindex', '-1');
heading.focus({ preventScroll: true });
return document.activeElement === heading;
"#;

#[cfg(any(target_arch = "wasm32", target_os = "android", target_os = "ios"))]
async fn focus_page_heading() -> bool {
    #[cfg(target_arch = "wasm32")]
    return crate::common::web_dom::focus_page_heading();
    #[cfg(any(target_os = "android", target_os = "ios"))]
    return document::eval(FOCUS_HEADING_JS).join::<bool>().await.unwrap_or(true);
}

/// Not on the first route: a fresh load is read from the top anyway.
pub fn use_route_focus(route: &Route) {
    let previous: Rc<RefCell<Option<Route>>> = use_hook(|| Rc::new(RefCell::new(None)));
    let changed = previous.borrow().as_ref().is_some_and(|p| p != route);
    *previous.borrow_mut() = Some(route.clone());

    #[cfg(any(target_arch = "wasm32", target_os = "android", target_os = "ios"))]
    if changed {
        spawn(async move {
            crate::common::next_paint().await;
            for _ in 0..20 {
                if focus_page_heading().await {
                    return;
                }
                crate::common::sleep(50).await;
            }
        });
    }
    #[cfg(not(any(target_arch = "wasm32", target_os = "android", target_os = "ios")))]
    let _ = changed;
}
