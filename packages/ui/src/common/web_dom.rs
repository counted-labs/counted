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

/// A finger, not a mouse: a field focused by script there raises the on-screen keyboard.
pub fn coarse_pointer() -> bool {
    web_sys::window()
        .and_then(|w| w.match_media("(pointer: coarse)").ok().flatten())
        .is_some_and(|q| q.matches())
}

fn paint_theme(theme: crate::common::Theme) {
    let dark = theme.is_dark(system_dark_query().is_some_and(|q| q.matches()));
    let Some(doc) = document() else { return };
    if let Some(el) = doc.document_element() {
        let _ = el.set_attribute("data-theme", crate::common::data_theme(dark));
    }
    if let Ok(Some(meta)) = doc.query_selector("meta[name=\"theme-color\"]") {
        let _ = meta.set_attribute("content", crate::common::theme_color(dark));
    }
}

/// `<html data-theme>` and the `theme-color` meta, plus a `prefers-color-scheme` listener installed
/// on the first call and leaked, which repaints while the choice is System.
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
    observer: Option<web_sys::MutationObserver>,
    _on_mutation: wasm_bindgen::closure::Closure<dyn Fn()>,
}

thread_local! {
    static OPEN_MODALS: std::cell::RefCell<Vec<OpenModal>> = const { std::cell::RefCell::new(Vec::new()) };
    static ESCAPE_LISTENER: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
}

const CONTROLS: &str = "a[href], button, input, select, textarea, [tabindex]";

/// A toast or a status line stays readable behind a dialog; a banner with buttons does not stay
/// reachable.
fn stays_live(el: &web_sys::Element) -> bool {
    let live = el.has_attribute("aria-live")
        || matches!(el.get_attribute("role").as_deref(), Some("alert") | Some("status"));
    live && el.query_selector(CONTROLS).ok().flatten().is_none()
}

/// A dialog mounting beside the top one is about to take over, and inerts the rest itself.
fn holds_dialog(el: &web_sys::Element) -> bool {
    el.matches("[aria-modal=\"true\"]").unwrap_or(false)
        || el.query_selector("[aria-modal=\"true\"]").ok().flatten().is_some()
}

fn inert_outside(dialog: &web_sys::Element, spare_dialogs: bool) -> Vec<web_sys::Element> {
    let body: Option<web_sys::Element> = document().and_then(|d| d.body()).map(Into::into);
    let mut inerted = Vec::new();
    let mut el = dialog.clone();
    while Some(&el) != body.as_ref() {
        let Some(parent) = el.parent_element() else { break };
        let mut sibling = parent.first_element_child();
        while let Some(s) = sibling {
            if s != el && !s.has_attribute("inert") && !stays_live(&s) && !(spare_dialogs && holds_dialog(&s)) {
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
        let inerted = inert_outside(&dialog, false);
        let on_mutation = wasm_bindgen::closure::Closure::<dyn Fn()>::new(move || keep_modal(id));
        let observer = web_sys::MutationObserver::new(on_mutation.as_ref().unchecked_ref()).ok();
        if let (Some(observer), Some(body)) = (&observer, doc.body()) {
            let options = web_sys::MutationObserverInit::new();
            options.set_child_list(true);
            options.set_subtree(true);
            let _ = observer.observe_with_options(&body, &options);
        }
        OPEN_MODALS.with_borrow_mut(|m| {
            m.push(OpenModal { id, opener, inerted, observer, _on_mutation: on_mutation })
        });
    }
    let focus_inside = |doc: &web_sys::Document| {
        dialog.contains(doc.active_element().as_ref().map(|el| el.as_ref()))
    };
    if !focus_inside(&doc) {
        let _ = dialog.unchecked_ref::<web_sys::HtmlElement>().focus();
    }
    focus_inside(&doc)
}

fn is_top_modal(id: &'static str) -> bool {
    OPEN_MODALS.with_borrow(|m| m.last().map(|m| m.id)) == Some(id)
}

/// Web twin of the observer callback in `modal_focus.rs`: the page mounted new content beside the
/// top dialog, which is made inert too, or swapped content under the focus, which the dialog takes
/// back. The focus is checked a task later: between a field's blur and the next one's focus the
/// active element is the body, and a re-render in that gap would otherwise steal the focus.
fn keep_modal(id: &'static str) {
    let Some(dialog) = document().and_then(|d| d.get_element_by_id(id)) else { return };
    OPEN_MODALS.with_borrow_mut(|m| {
        if let Some(top) = m.last_mut().filter(|top| top.id == id) {
            top.inerted.extend(inert_outside(&dialog, true));
        }
    });
    wasm_bindgen_futures::spawn_local(async move {
        sleep_ms(0).await;
        let Some(doc) = document() else { return };
        let body: Option<web_sys::Element> = doc.body().map(Into::into);
        let active = doc.active_element();
        if is_top_modal(id) && (active.is_none() || active == body) && dialog.is_connected() {
            let _ = dialog.unchecked_ref::<web_sys::HtmlElement>().focus();
        }
    });
}

/// Web twin of `leave` in `modal_focus.rs`.
pub fn leave_modal(id: &'static str) {
    let Some((modal, was_top)) = OPEN_MODALS.with_borrow_mut(|m| {
        let i = m.iter().rposition(|m| m.id == id)?;
        let was_top = i + 1 == m.len();
        Some((m.remove(i), was_top))
    }) else {
        return;
    };
    if let Some(observer) = &modal.observer {
        observer.disconnect();
    }
    for el in &modal.inerted {
        let _ = el.remove_attribute("inert");
    }
    let doc = document();
    OPEN_MODALS.with_borrow_mut(|m| {
        let Some(top) = m.last_mut() else { return };
        if let Some(dialog) = doc.as_ref().and_then(|d| d.get_element_by_id(top.id)) {
            top.inerted.extend(inert_outside(&dialog, false));
        }
    });
    if let Some(opener) = modal.opener.filter(|o| was_top && o.is_connected()) {
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

/// Web twin of `focus_field` in `select_focused.rs`.
pub fn focus_field(id: &str) {
    let Some(el) = document().and_then(|d| d.get_element_by_id(id)) else { return };
    let options = web_sys::FocusOptions::new();
    options.set_prevent_scroll(true);
    let _ = el.unchecked_ref::<web_sys::HtmlElement>().focus_with_options(&options);
}

/// Resolves the promise a granted Web Lock waits on, which releases the lock: at the end of the
/// work, or when the future holding it is dropped before.
pub struct LockRelease(js_sys::Function);

impl Drop for LockRelease {
    fn drop(&mut self) {
        let _ = self.0.call0(&JsValue::NULL);
    }
}

/// `navigator.locks.request(name, () => held)` once granted. `None` where the browser has no Web
/// Locks or refuses the request, so the caller can fall back on a rule of its own.
pub async fn acquire_lock(name: &str) -> Option<LockRelease> {
    let locks = js_sys::Reflect::get(&web_sys::window()?.navigator(), &JsValue::from_str("locks")).ok().filter(JsValue::is_object)?;
    let request: js_sys::Function = js_sys::Reflect::get(&locks, &JsValue::from_str("request")).ok()?.dyn_into().ok()?;
    let mut release = None;
    let held = js_sys::Promise::new(&mut |resolve, _| release = Some(resolve));
    let release = LockRelease(release?);
    let mut grant = None;
    let granted = js_sys::Promise::new(&mut |resolve, _| grant = Some(resolve));
    let grant = grant?;
    let on_grant = {
        let grant = grant.clone();
        wasm_bindgen::closure::Closure::once_into_js(move |_lock: JsValue| -> JsValue {
            let _ = grant.call1(&JsValue::NULL, &JsValue::TRUE);
            held.into()
        })
    };
    let requested = request.call2(&locks, &JsValue::from_str(name), &on_grant).ok()?;
    let on_refused = wasm_bindgen::closure::Closure::once_into_js(move |_e: JsValue| {
        let _ = grant.call1(&JsValue::NULL, &JsValue::FALSE);
    });
    if let Some(catch) = js_sys::Reflect::get(&requested, &JsValue::from_str("catch")).ok().and_then(|c| c.dyn_into::<js_sys::Function>().ok()) {
        let _ = catch.call1(&requested, &on_refused);
    }
    let answer = JsFuture::from(granted).await.ok()?;
    (answer.as_bool() == Some(true)).then_some(release)
}

thread_local! {
    static PAGE_GONE: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
    static LOCK_EPOCH: std::cell::Cell<u64> = const { std::cell::Cell::new(0) };
    static ON_PAGE_HIDE: std::cell::RefCell<Option<js_sys::Function>> = const { std::cell::RefCell::new(None) };
    static WATCHING_PAGE: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
}

/// True from `pagehide` until a `pageshow` brings the page back from the back-forward cache.
pub fn page_gone() -> bool {
    PAGE_GONE.with(|g| g.get())
}

/// Moves on at every `pagehide`: a lock taken under an older epoch was let go, even if the page is
/// shown again since, and its holder must take it again before it sends (SPEC §15.9.5 Q2e).
pub fn lock_epoch() -> u64 {
    LOCK_EPOCH.with(|e| e.get())
}

pub fn page_visible() -> bool {
    document().is_some_and(|d| !d.hidden())
}

/// Lets `release`'s lock go on `pagehide` (SPEC §15.9.5 Q2e), marks the page gone and moves the
/// lock epoch on, so that the session it was taken for sends nothing more.
pub fn release_on_page_hide(release: &LockRelease) {
    ON_PAGE_HIDE.with_borrow_mut(|f| *f = Some(release.0.clone()));
    if WATCHING_PAGE.replace(true) {
        return;
    }
    let Some(window) = web_sys::window() else { return };
    let on_hide = wasm_bindgen::closure::Closure::<dyn Fn()>::new(|| {
        PAGE_GONE.with(|g| g.set(true));
        LOCK_EPOCH.with(|e| e.set(e.get() + 1));
        if let Some(release) = ON_PAGE_HIDE.with_borrow_mut(Option::take) {
            let _ = release.call0(&JsValue::NULL);
        }
    });
    let on_show = wasm_bindgen::closure::Closure::<dyn Fn()>::new(|| PAGE_GONE.with(|g| g.set(false)));
    let _ = window.add_event_listener_with_callback("pagehide", on_hide.as_ref().unchecked_ref());
    let _ = window.add_event_listener_with_callback("pageshow", on_show.as_ref().unchecked_ref());
    on_hide.forget();
    on_show.forget();
}

/// Calls `on_show` when the page comes back from the back-forward cache (`pageshow` with
/// `persisted`). Leaked like [`on_tab_visible`]: it belongs to the app shell.
pub fn on_page_restored(on_show: impl Fn() + 'static) {
    let Some(window) = web_sys::window() else { return };
    let handler = wasm_bindgen::closure::Closure::<dyn Fn(JsValue)>::new(move |event: JsValue| {
        if js_sys::Reflect::get(&event, &JsValue::from_str("persisted")).ok().and_then(|p| p.as_bool()) == Some(true) {
            on_show();
        }
    });
    let _ = window.add_event_listener_with_callback("pageshow", handler.as_ref().unchecked_ref());
    handler.forget();
}

/// Calls `on_watched` with the new value of each `storage` event another tab raises for `watched`,
/// as it arrives, `None` when the key was removed or storage cleared, and `on_change` once per burst
/// of events for any key, `quiet_ms` after the first, with the key of each (`None` for a `clear()`).
/// Leaked like [`on_tab_visible`]: it belongs to the app shell.
pub fn on_storage(quiet_ms: i32, watched: &'static str, on_watched: impl Fn(Option<String>) + 'static, on_change: impl Fn(Vec<Option<String>>) + 'static) {
    use std::cell::RefCell;
    use std::rc::Rc;
    let Some(window) = web_sys::window() else { return };
    let burst: Rc<RefCell<Option<Vec<Option<String>>>>> = Rc::new(RefCell::new(None));
    let on_change: Rc<dyn Fn(Vec<Option<String>>)> = Rc::new(on_change);
    let handler = wasm_bindgen::closure::Closure::<dyn Fn(JsValue)>::new(move |event: JsValue| {
        let key = js_sys::Reflect::get(&event, &JsValue::from_str("key")).ok().and_then(|k| k.as_string());
        match key.as_deref() {
            Some(key) if key == watched => on_watched(js_sys::Reflect::get(&event, &JsValue::from_str("newValue")).ok().and_then(|v| v.as_string())),
            Some(_) => {}
            None => on_watched(None),
        }
        let first = {
            let mut burst = burst.borrow_mut();
            let first = burst.is_none();
            burst.get_or_insert_with(Vec::new).push(key);
            first
        };
        if !first {
            return;
        }
        let (burst, on_change) = (burst.clone(), on_change.clone());
        let flush = wasm_bindgen::closure::Closure::once_into_js(move || {
            let keys = burst.borrow_mut().take().unwrap_or_default();
            on_change(keys);
        });
        if let Some(window) = web_sys::window() {
            let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(flush.unchecked_ref(), quiet_ms);
        }
    });
    let _ = window.add_event_listener_with_callback("storage", handler.as_ref().unchecked_ref());
    handler.forget();
}
