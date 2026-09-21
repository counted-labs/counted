//! Enter moves focus to the next text field on mobile.
//!
//! The soft keyboard's action key fires a plain Enter, which the browser turns into implicit form
//! submission — so Enter in `#expense-name` submitted a half-filled expense. One `keydown`
//! listener on `document` walks the text-like inputs of the enclosing form (or dialog) and focuses
//! the next one that accepts focus; when none does, the default stays and the last field still
//! submits. Selects, dates, checkboxes and radios are never targets: on a phone they are tapped,
//! and focusing them only closes the keyboard.
//!
//! Two twins of the same algorithm, kept in step by hand: the JS below runs through
//! `document::eval` on the native mobile targets, and `web_dom::enter_advances_focus` is the
//! web-sys copy for the browser build, which must never eval (DOCUMENTATION.md §8). Both decide
//! synchronously inside the trusted keydown — `preventDefault` has to land before the event ends,
//! and iOS drops the keyboard when `focus()` is deferred out of the gesture. Desktop keeps the
//! standard Enter-submits.

#[cfg(any(target_os = "android", target_os = "ios"))]
pub fn use_enter_advances_focus() {
    dioxus::prelude::use_effect(|| {
        dioxus::prelude::document::eval(
            r#"
            (function() {
                if (window._countedEnterNext) return;
                window._countedEnterNext = true;
                const TEXT = ['text', 'email', 'password', 'number', 'tel', 'url', 'search'];
                const textLike = el => el.tagName === 'TEXTAREA'
                    || (el.tagName === 'INPUT' && TEXT.includes((el.getAttribute('type') || 'text').toLowerCase()));
                document.addEventListener('keydown', function(e) {
                    if (e.key !== 'Enter' || e.defaultPrevented || e.isComposing) return;
                    const from = e.target;
                    if (!from || from.tagName !== 'INPUT' || !textLike(from)) return;
                    const scope = from.closest('form, [role="dialog"]') || document;
                    const fields = Array.from(scope.querySelectorAll('input, textarea')).filter(textLike);
                    for (const next of fields.slice(fields.indexOf(from) + 1)) {
                        if (next.hasAttribute('readonly')) continue;
                        next.focus();
                        if (document.activeElement === next) { e.preventDefault(); return; }
                    }
                });
            })();
            "#,
        );
    });
}

#[cfg(target_arch = "wasm32")]
pub fn use_enter_advances_focus() {
    dioxus::prelude::use_effect(|| {
        if crate::common::is_mobile() {
            crate::common::web_dom::enter_advances_focus();
        }
    });
}

#[cfg(not(any(target_os = "android", target_os = "ios", target_arch = "wasm32")))]
pub fn use_enter_advances_focus() {}
