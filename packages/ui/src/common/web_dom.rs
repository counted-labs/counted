//! Direct DOM bindings for the web build - the CSP-safe replacement for `document::eval`.
//!
//! dioxus-web implements `document::eval` (and `document::Title`) with `new Function(...)`.
//! The app's CSP allows `'wasm-unsafe-eval'` but not `'unsafe-eval'` (nginx/nginx.conf), so the
//! constructor throws an `EvalError`; the exception unwinds the wasm stack through whatever was
//! running and leaves the app frozen. Web code calls these bindings instead - no eval, no CSP
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

thread_local! {
    static THEME: std::cell::Cell<Option<crate::common::Theme>> = const { std::cell::Cell::new(None) };
}

fn system_dark_query() -> Option<web_sys::MediaQueryList> {
    web_sys::window()?.match_media("(prefers-color-scheme: dark)").ok().flatten()
}

fn paint_theme(theme: crate::common::Theme) {
    let system_dark = system_dark_query().is_some_and(|q| q.matches());
    if let Some(el) = document().and_then(|d| d.document_element()) {
        let _ = el.set_attribute("data-theme", crate::common::data_theme(theme.is_dark(system_dark)));
    }
}

/// `<html data-theme>`, plus a `prefers-color-scheme` listener installed on the first call and
/// leaked, which repaints while the choice is System.
pub fn set_theme(theme: crate::common::Theme) {
    let first = THEME.with(|t| t.replace(Some(theme))).is_none();
    paint_theme(theme);
    if !first {
        return;
    }
    let Some(query) = system_dark_query() else { return };
    let handler = wasm_bindgen::closure::Closure::<dyn Fn()>::new(|| {
        if let Some(theme) = THEME.with(|t| t.get()) {
            paint_theme(theme);
        }
    });
    let _ = query.add_event_listener_with_callback("change", handler.as_ref().unchecked_ref());
    handler.forget();
}

/// `document.cookie = "name=value; …"`.
///
/// Only ever used for the language preference, which the *server* has to know before the client
/// can speak - localStorage is invisible to SSR, so a stored choice alone would render the wrong
/// language until hydration. Not HttpOnly by necessity (the client writes it). `Secure` is set
/// unconditionally because this is the generic cookie writer and the next caller inherits its
/// default; browsers accept a `Secure` cookie from `http://localhost` (a secure context) but drop
/// it on other plain-http origins such as the Android emulator's `10.0.2.2`, where SSR then falls
/// back to `Accept-Language` - a dev-only cost.
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

/// `history.replaceState` to the same path and query without the `#fragment`, so the key a share
/// link carried leaves the address bar and the history entry once the store holds it.
pub fn clear_hash() {
    let Some(w) = web_sys::window() else { return };
    let loc = w.location();
    let url = format!("{}{}", loc.pathname().unwrap_or_default(), loc.search().unwrap_or_default());
    if let Ok(h) = w.history() {
        let _ = h.replace_state_with_url(&JsValue::NULL, "", Some(&url));
    }
}

/// `location.assign(url)`: a full load, so every context starts again from the stores.
pub fn assign_location(url: &str) {
    if let Some(w) = web_sys::window() {
        let _ = w.location().assign(url);
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

/// Web twin of the keydown listener in `enter_next.rs` - same algorithm, same guards; change both.
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
/// `readText` - either way the call throws a TypeError through wasm and freezes the app, which is
/// the whole failure this module exists to avoid.
fn clipboard_with(method: &str) -> Option<web_sys::Clipboard> {
    let nav = web_sys::window()?.navigator();
    let clipboard = js_sys::Reflect::get(&nav, &JsValue::from_str("clipboard")).ok()?;
    let f = js_sys::Reflect::get(&clipboard, &JsValue::from_str(method)).ok()?;
    f.is_function().then(|| clipboard.unchecked_into())
}

/// `navigator.clipboard.writeText`. False when the browser refuses (denied permission, no
/// secure context) - callers surface it, the link stays selectable by hand.
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
/// as handled - only a sheet that could not open is a failure.
pub async fn share_url(url: &str) -> bool {
    let data = web_sys::ShareData::new();
    data.set_url(url);
    share(&data).await
}

/// `navigator.share({ text })`, for a message rather than a link: `url` would refuse it.
pub async fn share_message(text: &str) -> bool {
    let data = web_sys::ShareData::new();
    data.set_text(text);
    share(&data).await
}

async fn share(data: &web_sys::ShareData) -> bool {
    let Some(nav) = navigator_with_share() else {
        return false;
    };
    match JsFuture::from(nav.share_with_data(data)).await {
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

/// Selects the focused input's text once the tap that focused it has placed its caret.
pub async fn select_focused_input() {
    sleep_ms(0).await;
    let Some(input) = document()
        .and_then(|d| d.active_element())
        .and_then(|el| el.dyn_into::<web_sys::HtmlInputElement>().ok())
    else {
        return;
    };
    let len = input.value().encode_utf16().count() as u32;
    let _ = input.set_selection_range(0, len);
}

/// Resolves once the browser has painted. `requestAnimationFrame` fires *before* paint, so the
/// macrotask hop after it is what lands once the frame is on screen. A hidden tab never fires rAF -
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

/// Whether a form field holds the focus - a button that opened a modal does not count.
pub fn a_field_has_focus() -> bool {
    document()
        .and_then(|d| d.active_element())
        .is_some_and(|el| matches!(el.tag_name().as_str(), "INPUT" | "TEXTAREA" | "SELECT"))
}

struct OpenModal {
    id: &'static str,
    opener: Option<web_sys::HtmlElement>,
    inerted: Vec<web_sys::Element>,
}

thread_local! {
    static OPEN_MODALS: std::cell::RefCell<Vec<OpenModal>> = const { std::cell::RefCell::new(Vec::new()) };
    static ESCAPE_LISTENER: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
}

fn is_live_region(el: &web_sys::Element) -> bool {
    el.has_attribute("aria-live")
        || matches!(el.get_attribute("role").as_deref(), Some("alert") | Some("status"))
}

fn inert_outside(dialog: &web_sys::Element) -> Vec<web_sys::Element> {
    let body: Option<web_sys::Element> = document().and_then(|d| d.body()).map(Into::into);
    let mut inerted = Vec::new();
    let mut el = dialog.clone();
    while Some(&el) != body.as_ref() {
        let Some(parent) = el.parent_element() else { break };
        let mut sibling = parent.first_element_child();
        while let Some(s) = sibling {
            if s != el && !s.has_attribute("inert") && !is_live_region(&s) {
                let _ = s.set_attribute("inert", "");
                inerted.push(s.clone());
            }
            sibling = s.next_element_sibling();
        }
        el = parent;
    }
    inerted
}

/// Web twin of the Escape listener in `modal_focus.rs`.
fn listen_for_escape(doc: &web_sys::Document) {
    if ESCAPE_LISTENER.replace(true) {
        return;
    }
    let handler = wasm_bindgen::closure::Closure::<dyn Fn(web_sys::KeyboardEvent)>::new(
        move |e: web_sys::KeyboardEvent| {
            if e.key() != "Escape" || e.default_prevented() {
                return;
            }
            let Some(top) = OPEN_MODALS.with_borrow(|m| m.last().map(|m| m.id)) else { return };
            let backdrop = document()
                .and_then(|d| d.get_element_by_id(top))
                .and_then(|dialog| dialog.query_selector(":scope > .modal-backdrop").ok().flatten());
            if let Some(backdrop) = backdrop {
                e.prevent_default();
                backdrop.unchecked_ref::<web_sys::HtmlElement>().click();
            }
        },
    );
    let _ = doc.add_event_listener_with_callback("keydown", handler.as_ref().unchecked_ref());
    handler.forget();
}

/// Web twin of `enter` in `modal_focus.rs`. True once the focus is inside the dialog.
pub fn enter_modal(id: &'static str) -> bool {
    let Some(doc) = document() else { return true };
    let Some(dialog) = doc.get_element_by_id(id) else { return false };
    if !OPEN_MODALS.with_borrow(|m| m.iter().any(|m| m.id == id)) {
        listen_for_escape(&doc);
        let opener = doc.active_element().and_then(|e| e.dyn_into::<web_sys::HtmlElement>().ok());
        let inerted = inert_outside(&dialog);
        OPEN_MODALS.with_borrow_mut(|m| m.push(OpenModal { id, opener, inerted }));
    }
    let focus_inside = |doc: &web_sys::Document| {
        dialog.contains(doc.active_element().as_ref().map(|el| el.as_ref()))
    };
    if !focus_inside(&doc) {
        let _ = dialog.unchecked_ref::<web_sys::HtmlElement>().focus();
    }
    focus_inside(&doc)
}

/// Web twin of `leave` in `modal_focus.rs`.
pub fn leave_modal(id: &'static str) {
    let Some(modal) = OPEN_MODALS.with_borrow_mut(|m| {
        m.iter().rposition(|m| m.id == id).map(|i| m.remove(i))
    }) else {
        return;
    };
    for el in modal.inerted {
        let _ = el.remove_attribute("inert");
    }
    if let Some(opener) = modal.opener.filter(|o| o.is_connected()) {
        let _ = opener.focus();
    }
}

/// Web twin of `FOCUS_HEADING_JS` in `route_focus.rs`. True once there is nothing left to do.
pub fn focus_page_heading() -> bool {
    let Some(doc) = document() else { return true };
    let outside_inert = |el: &web_sys::Element| el.closest("[inert]").ok().flatten().is_none();
    let body: Option<web_sys::Element> = doc.body().map(Into::into);
    if let Some(active) = doc.active_element() {
        if Some(&active) != body.as_ref() && active.is_connected() && outside_inert(&active) {
            return true;
        }
    }
    let Ok(headings) = doc.query_selector_all("h1") else { return true };
    let Some(heading) = (0..headings.length())
        .filter_map(|i| headings.item(i)?.dyn_into::<web_sys::Element>().ok())
        .find(|h| outside_inert(h))
    else {
        return false;
    };
    if !heading.has_attribute("tabindex") {
        let _ = heading.set_attribute("tabindex", "-1");
    }
    let options = web_sys::FocusOptions::new();
    options.set_prevent_scroll(true);
    let _ = heading.unchecked_ref::<web_sys::HtmlElement>().focus_with_options(&options);
    doc.active_element().as_ref() == Some(&heading)
}
