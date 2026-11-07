//! What every dialog needs from the DOM and cannot say in markup: focus moves in on open, the page
//! behind becomes `inert` (Chromium does not prune it for `aria-modal`, so TalkBack swiped straight
//! into it), Escape anywhere clicks the dialog's own `.modal-backdrop` - the same dismissal as
//! Android Back in `navigation_sync.rs`, so a dialog without a backdrop stays open - and focus
//! returns to the opener on close. Live regions are never made inert, so a toast is still read.
//! The iOS keyboard proxy (`data-keyboard-proxy`, `KEYBOARD_PROXY_SCRIPT` in the mobile crate) is
//! neither made inert nor robbed of the focus: it holds the keypad up until the amount field takes
//! over, which the web build never needs.
//!
//! Two twins kept in step by hand, as in `enter_next.rs`: the JS below for the native WebViews,
//! `web_dom::{enter_modal, leave_modal}` for the browser build, which must never eval.

use dioxus::prelude::*;

/// daisyUI fades the modal's `visibility` in over 0.3s and `focus()` on a still-hidden element is
/// silently dropped, so the first attempts fail by design.
const FOCUS_TRIES: usize = 20;
const FOCUS_RETRY_MS: u32 = 50;

#[cfg(any(target_os = "android", target_os = "ios"))]
const MODAL_JS: &str = r#"
if (!window._countedModals) {
  const open = [];
  const isLive = el => el.hasAttribute('aria-live') || ['alert', 'status'].includes(el.getAttribute('role'));
  document.addEventListener('keydown', e => {
    if (e.key !== 'Escape' || e.defaultPrevented || !open.length) return;
    const dialog = document.getElementById(open[open.length - 1].id);
    const backdrop = dialog && dialog.querySelector(':scope > .modal-backdrop');
    if (backdrop) { e.preventDefault(); backdrop.click(); }
  });
  window._countedModals = {
    enter(id) {
      const dialog = document.getElementById(id);
      if (!dialog) return false;
      const proxied = !!document.activeElement && document.activeElement.hasAttribute('data-keyboard-proxy');
      if (!open.some(m => m.id === id)) {
        const inerted = [];
        for (let el = dialog; el !== document.body && el.parentElement; el = el.parentElement) {
          for (const sibling of el.parentElement.children) {
            if (sibling !== el && !sibling.inert && !isLive(sibling) && !sibling.hasAttribute('data-keyboard-proxy')) { sibling.inert = true; inerted.push(sibling); }
          }
        }
        open.push({ id, opener: proxied ? null : document.activeElement, inerted });
      }
      if (proxied) return true;
      if (!dialog.contains(document.activeElement)) dialog.focus();
      return dialog.contains(document.activeElement);
    },
    leave(id) {
      const i = open.map(m => m.id).lastIndexOf(id);
      if (i < 0) return;
      const [m] = open.splice(i, 1);
      m.inerted.forEach(el => { el.inert = false; });
      if (m.opener && m.opener.isConnected) m.opener.focus();
    },
  };
}
"#;

async fn enter_modal(id: &'static str) -> bool {
    #[cfg(target_arch = "wasm32")]
    return crate::common::web_dom::enter_modal(id);
    #[cfg(any(target_os = "android", target_os = "ios"))]
    return document::eval(&format!(
        "{MODAL_JS} return window._countedModals.enter({});",
        serde_json::to_string(id).unwrap_or_default()
    ))
    .join::<bool>()
    .await
    .unwrap_or(true);
    #[cfg(not(any(target_arch = "wasm32", target_os = "android", target_os = "ios")))]
    {
        let _ = id;
        true
    }
}

fn leave_modal(id: &'static str) {
    #[cfg(target_arch = "wasm32")]
    crate::common::web_dom::leave_modal(id);
    #[cfg(any(target_os = "android", target_os = "ios"))]
    document::eval(&format!(
        "{MODAL_JS} window._countedModals.leave({});",
        serde_json::to_string(id).unwrap_or_default()
    ));
    #[cfg(not(any(target_arch = "wasm32", target_os = "android", target_os = "ios")))]
    let _ = id;
}

/// For the element carrying `role="dialog"`, which needs this `id` and `tabindex: "-1"`.
pub fn use_modal_focus(dialog_id: &'static str) {
    use_effect(move || {
        spawn(async move {
            for _ in 0..FOCUS_TRIES {
                if enter_modal(dialog_id).await {
                    return;
                }
                crate::common::sleep(FOCUS_RETRY_MS).await;
            }
        });
    });
    use_drop(move || leave_modal(dialog_id));
}
