//! Direct DOM bindings for the web build — the CSP-safe replacement for `document::eval`.
//!
//! dioxus-web implements `document::eval` (and `document::Title`) with `new Function(...)`.
//! The app's CSP allows `'wasm-unsafe-eval'` but not `'unsafe-eval'` (nginx/nginx.conf), so the
//! constructor throws an `EvalError`; the exception unwinds the wasm stack through whatever was
//! running and leaves the app frozen. Web code calls these bindings instead — no eval, no CSP
//! relaxation. Native targets (mobile WebView, desktop) keep using eval: no CSP applies there.

use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;

fn document() -> Option<web_sys::Document> {
    web_sys::window().and_then(|w| w.document())
}

/// `document.title = title`
pub fn set_title(title: &str) {
    if let Some(d) = document() {
        d.set_title(title);
    }
}

/// `document.documentElement.lang = lang`
pub fn set_html_lang(lang: &str) {
    if let Some(el) = document().and_then(|d| d.document_element()) {
        let _ = el.set_attribute("lang", lang);
    }
}

/// `document.cookie = "name=value; …"`.
///
/// Only ever used for the language preference, which the *server* has to know before the client
/// can speak — localStorage is invisible to SSR, so a stored choice alone would render the wrong
/// language until hydration. Not HttpOnly by necessity (the client writes it). `Secure` is set
/// unconditionally because this is the generic cookie writer and the next caller inherits its
/// default; browsers accept a `Secure` cookie from `http://localhost` (a secure context) but drop
/// it on other plain-http origins such as the Android emulator's `10.0.2.2`, where SSR then falls
/// back to `Accept-Language` — a dev-only cost.
pub fn set_cookie(name: &str, value: &str, max_age: i64) {
    let Some(doc) = document().map(|d| d.unchecked_into::<web_sys::HtmlDocument>()) else {
        return;
    };
    let _ = doc.set_cookie(&format!(
        "{name}={value}; Path=/; Max-Age={max_age}; SameSite=Lax; Secure"
    ));
}

/// `document.querySelector('meta[name=…]').content`
pub fn meta_content(name: &str) -> Option<String> {
    document()?
        .query_selector(&format!("meta[name=\"{name}\"]"))
        .ok()??
        .get_attribute("content")
}

/// `(await fetch(url)).text()`, `None` on any failure including a non-2xx status.
pub async fn fetch_text(url: &str) -> Option<String> {
    let window = web_sys::window()?;
    let response: web_sys::Response =
        JsFuture::from(window.fetch_with_str(url)).await.ok()?.dyn_into().ok()?;
    if !response.ok() {
        return None;
    }
    JsFuture::from(response.text().ok()?).await.ok()?.as_string()
}

/// `history.replaceState(null, '', '#fragment')` — keeps the key in the URL without navigating.
pub fn replace_state_hash(fragment: &str) {
    if let Some(h) = web_sys::window().and_then(|w| w.history().ok()) {
        let _ = h.replace_state_with_url(&JsValue::NULL, "", Some(&format!("#{fragment}")));
    }
}

/// Calls `on_visible` every time the tab is foregrounded. The listener is leaked deliberately: it
/// belongs to the app shell, which outlives every page, and there is nothing to detach it from.
pub fn on_tab_visible(on_visible: impl Fn() + 'static) {
    let Some(doc) = document() else { return };
    let handler = wasm_bindgen::closure::Closure::<dyn Fn()>::new(move || {
        let visible = document().map(|d| !d.hidden()).unwrap_or(false);
        if visible {
            on_visible();
        }
    });
    let _ = doc.add_event_listener_with_callback(
        "visibilitychange",
        handler.as_ref().unchecked_ref(),
    );
    handler.forget();
}

const TEXT_INPUT_TYPES: [&str; 7] = ["text", "email", "password", "number", "tel", "url", "search"];

fn text_like(el: &web_sys::Element) -> bool {
    match el.tag_name().as_str() {
        "TEXTAREA" => true,
        "INPUT" => {
            let kind = el.get_attribute("type").unwrap_or_else(|| "text".into()).to_lowercase();
            TEXT_INPUT_TYPES.contains(&kind.as_str())
        }
        _ => false,
    }
}

/// Web twin of the keydown listener in `enter_next.rs` — same algorithm, same guards; change both.
pub fn enter_advances_focus() {
    let Some(doc) = document() else { return };
    let handler = wasm_bindgen::closure::Closure::<dyn Fn(web_sys::KeyboardEvent)>::new({
        let doc = doc.clone();
        move |e: web_sys::KeyboardEvent| {
            if e.key() != "Enter" || e.default_prevented() || e.is_composing() {
                return;
            }
            let Some(from) = e.target().and_then(|t| t.dyn_into::<web_sys::Element>().ok()) else {
                return;
            };
            if from.tag_name() != "INPUT" || !text_like(&from) {
                return;
            }
            let Some(scope) = from
                .closest("form, [role=\"dialog\"]")
                .ok()
                .flatten()
                .or_else(|| doc.document_element())
            else {
                return;
            };
            let Ok(nodes) = scope.query_selector_all("input, textarea") else { return };
            let fields: Vec<web_sys::Element> = (0..nodes.length())
                .filter_map(|i| nodes.item(i)?.dyn_into::<web_sys::Element>().ok())
                .filter(text_like)
                .collect();
            let Some(pos) = fields.iter().position(|f| f == &from) else { return };
            for next in &fields[pos + 1..] {
                if next.has_attribute("readonly") {
                    continue;
                }
                let _ = next.unchecked_ref::<web_sys::HtmlElement>().focus();
                if doc.active_element().as_ref() == Some(next) {
                    e.prevent_default();
                    return;
                }
            }
        }
    });
    let _ = doc.add_event_listener_with_callback("keydown", handler.as_ref().unchecked_ref());
    handler.forget();
}

/// `navigator.clipboard`, but only when `method` really exists on it. web-sys' bindings are not
/// `catch`: on a non-secure origin `clipboard` is undefined, and Firefox has `writeText` but no
/// `readText` — either way the call throws a TypeError through wasm and freezes the app, which is
/// the whole failure this module exists to avoid.
fn clipboard_with(method: &str) -> Option<web_sys::Clipboard> {
    let nav = web_sys::window()?.navigator();
    let clipboard = js_sys::Reflect::get(&nav, &JsValue::from_str("clipboard")).ok()?;
    let f = js_sys::Reflect::get(&clipboard, &JsValue::from_str(method)).ok()?;
    f.is_function().then(|| clipboard.unchecked_into())
}

/// `navigator.clipboard.writeText`. False when the browser refuses (denied permission, no
/// secure context) — callers surface it, the link stays selectable by hand.
pub async fn copy_to_clipboard(text: &str) -> bool {
    let Some(clipboard) = clipboard_with("writeText") else {
        return false;
    };
    JsFuture::from(clipboard.write_text(text)).await.is_ok()
}

/// `navigator.clipboard.readText`. `None` when the browser refuses.
pub async fn read_clipboard() -> Option<String> {
    let clipboard = clipboard_with("readText")?;
    JsFuture::from(clipboard.read_text()).await.ok()?.as_string()
}

/// The navigator only when `navigator.share` is a function, checked through `Reflect` for the
/// same reason as [`clipboard_with`]: Firefox on desktop and Chrome on Linux have no Web Share.
fn navigator_with_share() -> Option<web_sys::Navigator> {
    let nav = web_sys::window()?.navigator();
    let share = js_sys::Reflect::get(&nav, &JsValue::from_str("share")).ok()?;
    share.is_function().then_some(nav)
}

/// `navigator.share({ url })`. The user dismissing the sheet rejects with `AbortError` and counts
/// as handled — only a sheet that could not open is a failure.
pub async fn share_url(url: &str) -> bool {
    let Some(nav) = navigator_with_share() else {
        return false;
    };
    let data = web_sys::ShareData::new();
    data.set_url(url);
    match JsFuture::from(nav.share_with_data(&data)).await {
        Ok(_) => true,
        Err(e) => js_sys::Reflect::get(&e, &JsValue::from_str("name"))
            .ok()
            .and_then(|n| n.as_string())
            .is_some_and(|n| n == "AbortError"),
    }
}

/// Saves `content` as a file: blob + a synthetic anchor click, then release the object URL.
pub fn download(content: &str, filename: &str, mime_type: &str) {
    let Some(doc) = document() else { return };
    let parts = js_sys::Array::of1(&JsValue::from_str(content));
    let options = web_sys::BlobPropertyBag::new();
    options.set_type(mime_type);
    let Ok(blob) = web_sys::Blob::new_with_str_sequence_and_options(&parts, &options) else {
        return;
    };
    let Ok(url) = web_sys::Url::create_object_url_with_blob(&blob) else { return };
    if let Ok(anchor) = doc.create_element("a").map(|e| e.unchecked_into::<web_sys::HtmlAnchorElement>())
    {
        anchor.set_href(&url);
        anchor.set_download(filename);
        anchor.click();
    }
    let _ = web_sys::Url::revoke_object_url(&url);
}

/// `await new Promise(r => setTimeout(r, ms))`.
pub async fn sleep_ms(ms: u32) {
    gloo_timers::future::TimeoutFuture::new(ms).await;
}

/// Resolves once the browser has painted. `requestAnimationFrame` fires *before* paint, so the
/// macrotask hop after it is what lands once the frame is on screen. A hidden tab never fires rAF —
/// skip the wait there rather than hang until it is foregrounded.
pub async fn next_paint() {
    let hidden = document().map(|d| d.hidden()).unwrap_or(true);
    if let (Some(win), false) = (web_sys::window(), hidden) {
        let promise = js_sys::Promise::new(&mut |resolve, _| {
            let _ = win.request_animation_frame(&resolve);
        });
        let _ = JsFuture::from(promise).await;
    }
    gloo_timers::future::TimeoutFuture::new(0).await;
}
