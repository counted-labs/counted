//! Translations. Every user-facing string goes through `tid!`, never a literal in `rsx!`.
//!
//! `tid!` renders the message id itself when a key is missing: visible, but the app keeps running.
//! A panic on the wasm render path would unwind through the renderer and freeze the app — the
//! same failure mode `web_dom` exists to avoid.
//!
//! The Dioxus glue is ours (context, `tid!`, bundle rebuild on switch) rather than a crate's:
//! the wasm bundle embeds only `en.ftl` and fetches the other locales as hashed, immutable assets
//! when they are needed, and that needs a context a locale can be added to after it was created.
//! Native targets (server, mobile) embed every locale — the server renders any of them, and the
//! phone must work offline.
//!
//! Adding a language is one line in `locales!` plus the `.ftl` file. Every file must be complete:
//! `en.ftl` is the per-message fallback, and the tests below hold every locale to its id set.

use std::collections::HashMap;

use dioxus::fullstack::FullstackContext;
use dioxus::prelude::*;
pub use fluent::FluentArgs;
use fluent::{FluentBundle, FluentResource};
use unic_langid::{langid, LanguageIdentifier};

use crate::common::{read_from_ls, write_to_ls, LocalStorageState};

/// One list, three tables: the picker's `(code, endonym)` pairs, the hashed asset the web build
/// fetches, and the embedded text native builds carry. A missing file is a compile error.
///
/// The label is the language's name *in that language* — a French speaker looking for German scans
/// for "Deutsch", not "Allemand". Never translate this column. Ordered by endonym, Latin script
/// first, so the `<select>` reads as one alphabet then the other.
macro_rules! locales {
    ($( $code:literal => $endonym:literal, $path:literal ),* $(,)?) => {
        /// Every language the app ships, as (BCP-47 code, endonym).
        pub const SUPPORTED: &[(&str, &str)] = &[ $( ($code, $endonym) ),* ];

        const ASSETS: &[(&str, Asset)] = &[ $( ($code, asset!($path)) ),* ];

        #[cfg(not(target_arch = "wasm32"))]
        fn embedded(code: &str) -> Option<&'static str> {
            match code {
                $( $code => Some(include_str!(concat!("..", $path))), )*
                _ => None,
            }
        }
    };
}

locales! {
    "bs" => "Bosanski", "/i18n/bs.ftl",
    "cs" => "Čeština", "/i18n/cs.ftl",
    "da" => "Dansk", "/i18n/da.ftl",
    "de" => "Deutsch", "/i18n/de.ftl",
    "et" => "Eesti", "/i18n/et.ftl",
    "en" => "English", "/i18n/en.ftl",
    "es" => "Español", "/i18n/es.ftl",
    "fr" => "Français", "/i18n/fr.ftl",
    "ga" => "Gaeilge", "/i18n/ga.ftl",
    "hr" => "Hrvatski", "/i18n/hr.ftl",
    "is" => "Íslenska", "/i18n/is.ftl",
    "it" => "Italiano", "/i18n/it.ftl",
    "lv" => "Latviešu", "/i18n/lv.ftl",
    "lt" => "Lietuvių", "/i18n/lt.ftl",
    "hu" => "Magyar", "/i18n/hu.ftl",
    "mt" => "Malti", "/i18n/mt.ftl",
    "nl" => "Nederlands", "/i18n/nl.ftl",
    "no" => "Norsk", "/i18n/no.ftl",
    "pl" => "Polski", "/i18n/pl.ftl",
    "pt" => "Português", "/i18n/pt.ftl",
    "ro" => "Română", "/i18n/ro.ftl",
    "sq" => "Shqip", "/i18n/sq.ftl",
    "sk" => "Slovenčina", "/i18n/sk.ftl",
    "sl" => "Slovenščina", "/i18n/sl.ftl",
    "sr" => "Srpski", "/i18n/sr.ftl",
    "fi" => "Suomi", "/i18n/fi.ftl",
    "sv" => "Svenska", "/i18n/sv.ftl",
    "tr" => "Türkçe", "/i18n/tr.ftl",
    "bg" => "Български", "/i18n/bg.ftl",
    "mk" => "Македонски", "/i18n/mk.ftl",
    "uk" => "Українська", "/i18n/uk.ftl",
    "el" => "Ελληνικά", "/i18n/el.ftl",
}

/// The locale every other one falls back to, per message. `en.ftl` is the only file the wasm
/// bundle embeds: a key missing there is the only kind that surfaces to a user as a raw key.
pub const FALLBACK: &str = "en";

#[cfg(any(target_arch = "wasm32", test))]
const FALLBACK_FTL: &str = include_str!("../i18n/en.ftl");

/// The cookie that lets the *server* pick the right locale before the client has said anything.
/// Functional only — no identifier, no tracking. See docs/rgpd-compliance.md.
pub const LANG_COOKIE: &str = "counted_lang";

/// The `<meta>` the server writes so the wasm knows which locale to fetch before it renders.
pub const LANG_META: &str = "counted-lang";

const COOKIE_MAX_AGE: i64 = 31_536_000;

fn langid_of(code: &str) -> LanguageIdentifier {
    code.parse().unwrap_or(langid!("en"))
}

/// The hashed, immutable URL of a locale file — `/assets/de-dxh….ftl`. Content change, new hash.
pub fn locale_asset(code: &str) -> Option<Asset> {
    ASSETS.iter().find(|(c, _)| *c == code).map(|(_, a)| *a)
}

/// Translate `id` (or `id.attribute`) with `args`, or `None` when the id is unknown to both the
/// active locale and the fallback, or when Fluent reports an error formatting it.
///
/// Reactive: reads the bundle signal, so a component calling this re-renders on a switch.
fn translate(id: &str, args: Option<&FluentArgs>) -> Option<String> {
    let i18n = i18n();
    let bundle = i18n.bundle.read();
    let (message_id, attribute) = match id.split_once('.') {
        Some((m, a)) => (m, Some(a)),
        None => (id, None),
    };
    let message = bundle.get_message(message_id)?;
    let pattern = match attribute {
        Some(a) => message.get_attribute(a)?.value(),
        None => message.value()?,
    };
    let mut errors = vec![];
    let text = bundle.format_pattern(pattern, args, &mut errors).into_owned();
    errors.is_empty().then_some(text)
}

/// `translate`, rendering the id itself when it cannot be resolved. What `tid!` expands to.
pub fn translate_or_id(id: &str, args: Option<&FluentArgs>) -> String {
    translate(id, args).unwrap_or_else(|| id.to_string())
}

/// `tid!("id")` / `tid!("id", name: value, …)` — the only way copy reaches the UI.
#[macro_export]
macro_rules! tid {
    ($id:expr $(, $name:ident : $value:expr)+ $(,)?) => {{
        let mut args = $crate::i18n::FluentArgs::new();
        $( args.set(stringify!($name), $value); )+
        $crate::i18n::translate_or_id($id, Some(&args))
    }};
    ($id:expr $(,)?) => {
        $crate::i18n::translate_or_id($id, None)
    };
}

/// The i18n context: the active language, the locale texts known so far, and the bundle built
/// from the active one over the fallback. `Copy`, like every Dioxus context of signals.
#[derive(Clone, Copy)]
pub struct I18n {
    lang: Signal<LanguageIdentifier>,
    resources: Signal<HashMap<LanguageIdentifier, &'static str>>,
    bundle: Signal<FluentBundle<FluentResource>>,
}

impl I18n {
    fn new(lang: LanguageIdentifier, resources: HashMap<LanguageIdentifier, &'static str>) -> Self {
        let bundle = build_bundle(&lang, &resources);
        Self {
            lang: Signal::new(lang),
            resources: Signal::new(resources),
            bundle: Signal::new(bundle),
        }
    }

    pub fn language(&self) -> LanguageIdentifier {
        self.lang.read().clone()
    }

    pub fn has_locale(&self, id: &LanguageIdentifier) -> bool {
        self.resources.peek().contains_key(id)
    }

    /// Register a locale's text. Rebuilds the bundle only when it changes what renders now.
    pub fn add_locale(&mut self, id: LanguageIdentifier, ftl: &'static str) {
        let renders_now = id == *self.lang.peek() || id == langid!("en");
        self.resources.write().insert(id, ftl);
        if renders_now {
            self.rebuild();
        }
    }

    pub fn set_language(&mut self, id: LanguageIdentifier) {
        self.lang.set(id);
        self.rebuild();
    }

    fn rebuild(&mut self) {
        let bundle = build_bundle(&self.lang.peek(), &self.resources.peek());
        self.bundle.set(bundle);
    }
}

/// The fallback first, then the active language from its bare subtag up to its full identifier
/// (`de`, then `de-Latn`, `de-AT`, variants), each overriding what came before.
fn build_bundle(
    lang: &LanguageIdentifier,
    resources: &HashMap<LanguageIdentifier, &'static str>,
) -> FluentBundle<FluentResource> {
    let mut bundle = FluentBundle::new(vec![lang.clone()]);
    let mut add = |id: &LanguageIdentifier| {
        if let Some(ftl) = resources.get(id) {
            let resource = FluentResource::try_new((*ftl).to_string()).unwrap_or_else(|(res, e)| {
                tracing::error!("{id}.ftl has syntax errors: {e:?}");
                res
            });
            bundle.add_resource_overriding(resource);
        }
    };
    add(&langid!("en"));
    let (language, script, region, variants) = lang.clone().into_parts();
    add(&LanguageIdentifier::from_parts(language, None, None, &[]));
    add(&LanguageIdentifier::from_parts(language, script, None, &[]));
    add(&LanguageIdentifier::from_parts(language, script, region, &[]));
    if !variants.is_empty() {
        add(lang);
    }
    bundle
}

pub fn i18n() -> I18n {
    consume_context()
}

/// Registers the i18n context. Called from `use_app_contexts`, before any other hook.
///
/// The initial locale is resolved **on the server** and carried into hydration by
/// `use_server_cached`, so the SSR markup and the first wasm render agree. Deciding it client-side
/// from localStorage would diverge from the served HTML and break hydration — the same trap
/// `onboarding_seen` and `show_archived` are documented against.
/// Returns the code it resolved, so the caller can tell whether a stored preference differs from
/// it without subscribing to the locale signal (which would make the reconciling effect re-run).
pub fn use_init_locale() -> String {
    let initial = use_server_cached(resolve_request_lang);

    let config = initial.clone();
    use_context_provider(move || I18n::new(langid_of(&config), initial_resources()));

    initial
}

/// What the context starts with. Native: every locale. Wasm: English plus whatever
/// [`preload_request_locale`] fetched before launch.
fn initial_resources() -> HashMap<LanguageIdentifier, &'static str> {
    let mut resources = HashMap::new();
    #[cfg(not(target_arch = "wasm32"))]
    for (code, _) in SUPPORTED {
        if let Some(ftl) = embedded(code) {
            resources.insert(langid_of(code), ftl);
        }
    }
    #[cfg(target_arch = "wasm32")]
    {
        resources.insert(langid!("en"), FALLBACK_FTL);
        if let Some((code, ftl)) = PRELOADED.take() {
            resources.insert(langid_of(&code), ftl);
        }
    }
    resources
}

#[cfg(target_arch = "wasm32")]
thread_local! {
    static PRELOADED: std::cell::Cell<Option<(String, &'static str)>> = const { std::cell::Cell::new(None) };
}

/// Fetch the locale the server rendered in, **before** `dioxus::launch`, so the first wasm render
/// speaks the same language as the SSR markup it hydrates. The server names it in a `<meta>`, and
/// `LocaleHead` also emits a `<link rel="preload">` for it, so by the time the wasm runs this the
/// browser usually has it already — a cache hit, no round trip.
///
/// English needs no fetch (embedded). On failure the app launches anyway, in English: the asset
/// server that just delivered the wasm failing one request later is not worth a blank page.
#[cfg(target_arch = "wasm32")]
pub async fn preload_request_locale() {
    use crate::common::web_dom;
    let Some(code) = web_dom::meta_content(LANG_META) else { return };
    if code == FALLBACK {
        return;
    }
    let Some(asset) = locale_asset(&code) else { return };
    match web_dom::fetch_text(&asset.to_string()).await {
        Some(text) => PRELOADED.set(Some((code, Box::leak(text.into_boxed_str())))),
        None => tracing::warn!("could not fetch the {code} locale; rendering in English"),
    }
}

/// Head elements the web entry point renders: the `<meta>` the pre-launch fetch reads, and a
/// preload of the same asset so the browser fetches it alongside the wasm. Reactive on the
/// locale, so after a switch the next reload's head is already right too (the cookie decides the
/// server side; this only mirrors it).
#[component]
pub fn LocaleHead() -> Element {
    let lang = current_lang();
    let asset = (lang != FALLBACK).then(|| locale_asset(&lang)).flatten();
    rsx! {
        document::Meta { name: LANG_META, content: lang }
        if let Some(asset) = asset {
            document::Link { rel: "preload", r#as: "fetch", crossorigin: "anonymous", href: asset }
        }
    }
}

/// The active language code. Reactive — reading it re-runs the caller on a language change.
pub fn current_lang() -> String {
    i18n().language().language.to_string()
}

/// Switch language: the signal (repaints everything), localStorage (survives a reload) and the
/// cookie (lets the *next* SSR render get it right, so there is no flash of the wrong language).
///
/// Must be called from a component scope — `i18n()` consumes context.
pub fn set_language(code: &str) {
    let Some(code) = supported(code) else {
        return;
    };

    apply_language(code);

    let mut state = read_from_ls();
    state.language = Some(code.to_string());
    write_to_ls(&state);
    if let Some(mut ls) = try_consume_context::<Signal<LocalStorageState>>() {
        ls.set(state);
    }

    #[cfg(target_arch = "wasm32")]
    crate::common::web_dom::set_cookie(LANG_COOKIE, code, COOKIE_MAX_AGE);
    #[cfg(not(target_arch = "wasm32"))]
    let _ = COOKIE_MAX_AGE;
}

/// Switch the rendered language **without recording a choice**.
///
/// Used when the language was inferred rather than picked. Persisting an inference would freeze it:
/// the device's language could change and the app would keep the old one forever, having stored
/// something the user never chose.
///
/// On the web build a locale the wasm has not seen yet is fetched first; the switch happens when
/// it lands, and a failed fetch (offline) leaves the current language rendered. Native has every
/// locale embedded, so the switch is immediate there.
fn apply_language(code: &'static str) {
    let mut i18n = i18n();
    let id = langid_of(code);
    if i18n.has_locale(&id) {
        i18n.set_language(id);
    } else {
        #[cfg(target_arch = "wasm32")]
        spawn(async move {
            let Some(asset) = locale_asset(code) else { return };
            match crate::common::web_dom::fetch_text(&asset.to_string()).await {
                Some(text) => {
                    i18n.add_locale(id.clone(), Box::leak(text.into_boxed_str()));
                    i18n.set_language(id);
                }
                None => tracing::warn!("could not fetch the {code} locale; keeping the current one"),
            }
        });
    }
}

/// [`apply_language`] for a code that came from inference rather than from the user. Validated
/// against `SUPPORTED` like `set_language`, and deliberately not persisted.
pub fn apply_inferred_language(code: &str) {
    if let Some(code) = supported(code) {
        apply_language(code);
    }
}

/// The device's preferred language, for the targets that have no HTTP request to read it from.
///
/// Mobile and desktop run in a WebView, so `navigator.language` is the system locale as the OS
/// reports it — one mechanism for android, ios and desktop. Off-web `document::eval` is the
/// sanctioned way to reach the DOM (the CSP that forbids it applies to the browser build only, and
/// `use_app_contexts` already evals for connectivity events).
///
/// Async, so unlike `Accept-Language` on web this cannot feed the initial config; it is applied
/// post-mount. These targets have no SSR, so there is no served markup for it to diverge from.
#[cfg(not(target_arch = "wasm32"))]
pub async fn device_lang() -> Option<&'static str> {
    let tag = dioxus::document::eval("return navigator.language || '';")
        .await
        .ok()?
        .as_str()?
        .to_string();
    // The header parser does exactly what is needed here too: take the primary subtag, so `de-AT`
    // and `pt-BR` land on `de` and `pt`, and return None for a language we do not ship.
    lang_from_accept(&tag)
}

/// The locale to render this request in.
///
/// No `cfg` gate: `packages/ui` has no `server` feature, and needs none. `FullstackContext::current`
/// is `None` off-server, so this returns the fallback on wasm (where `use_server_cached` hands back
/// the hydrated value instead) and on mobile/desktop (where there is no SSR to diverge from, and
/// the stored preference is applied post-mount).
fn resolve_request_lang() -> String {
    let Some(ctx) = FullstackContext::current() else {
        return FALLBACK.to_string();
    };
    let parts = ctx.parts_mut();
    let header = |name: &str| parts.headers.get(name).and_then(|v| v.to_str().ok());

    let picked = header("cookie")
        .and_then(lang_from_cookie)
        .or_else(|| header("accept-language").and_then(lang_from_accept));

    picked.unwrap_or(FALLBACK).to_string()
}

fn supported(code: &str) -> Option<&'static str> {
    SUPPORTED.iter().find(|(c, _)| c.eq_ignore_ascii_case(code)).map(|(c, _)| *c)
}

/// The `counted_lang` value out of a raw `Cookie` header, if it names a language we ship.
fn lang_from_cookie(header: &str) -> Option<&'static str> {
    header
        .split(';')
        .filter_map(|pair| pair.split_once('='))
        .find(|(k, _)| k.trim() == LANG_COOKIE)
        .and_then(|(_, v)| supported(v.trim()))
}

/// The first language of an `Accept-Language` header that we ship, matched on the primary subtag
/// so `de-AT` and `pt-BR` land on `de` and `pt`. Browsers say `nb`/`nn` for Norwegian; we ship it
/// as `no`.
///
/// Entries are taken in the order sent. Browsers already order them by preference, and re-sorting
/// on `q` would only matter for a hand-written header.
fn lang_from_accept(header: &str) -> Option<&'static str> {
    header
        .split(',')
        .filter_map(|entry| entry.split(';').next())
        .filter_map(|tag| tag.trim().split(['-', '_']).next())
        .map(|primary| match primary {
            "nb" | "nn" => "no",
            other => other,
        })
        .find_map(supported)
}

/// Provides an i18n context for a component test.
///
/// `tid!` resolves through `consume_context` and **panics without a provider** — in the app that
/// cannot happen (every entry point's root calls `use_app_contexts`), but a bare `VirtualDom` in a
/// test has nothing above it. This skips `use_server_cached`, which needs a fullstack render
/// context that a bare `VirtualDom` also lacks.
#[cfg(test)]
pub(crate) fn use_test_i18n() {
    use_context_provider(|| I18n::new(langid!("en"), HashMap::from([(langid!("en"), FALLBACK_FTL)])));
}

/// The message ids defined by one locale file, read off disk.
///
/// Test-only, and `pub(crate)` on purpose: the scanner guard below only sees `tid!("literal")`
/// call sites, so any module that keys `tid!` off a table or an enum must assert its own keys
/// against this — see `project_status_menu::tests`.
#[cfg(all(test, not(target_arch = "wasm32")))]
pub(crate) fn locale_ids(code: &str) -> std::collections::BTreeSet<String> {
    let text = locale_text(code);
    // Only a top-level `id = …` line defines a message: comments start with `#`, attributes with
    // `.`, and continuation lines (select variants included) are indented.
    text.lines()
        .filter(|l| !l.starts_with(['#', ' ', '\t', '.']) && !l.trim().is_empty())
        .filter_map(|l| l.split_once('='))
        .map(|(id, _)| id.trim().to_string())
        .filter(|id| !id.is_empty())
        .collect()
}

#[cfg(all(test, not(target_arch = "wasm32")))]
fn locale_text(code: &str) -> String {
    let path =
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("i18n").join(format!("{code}.ftl"));
    std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{} is missing: {e}", path.display()))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `shared::UI_LANGS` is the copy `api` validates the language counters against, and it cannot
    /// import this crate. A locale added to `locales!` and not there would be counted as `other`.
    #[test]
    fn shared_ui_langs_lists_exactly_the_shipped_locales() {
        let shipped: std::collections::BTreeSet<&str> =
            SUPPORTED.iter().map(|(code, _)| *code).collect();
        let shared: std::collections::BTreeSet<&str> = shared::UI_LANGS.iter().copied().collect();
        assert_eq!(shipped, shared);
    }

    #[test]
    fn cookie_wins_when_it_names_a_shipped_language() {
        assert_eq!(lang_from_cookie("counted_lang=de"), Some("de"));
        assert_eq!(lang_from_cookie("session_id=abc; counted_lang=it; x=1"), Some("it"));
        assert_eq!(lang_from_cookie(" counted_lang = pt "), Some("pt"));
    }

    /// A cookie is client-controlled. Anything that is not one of ours must fall through to the
    /// header rather than reach `langid_of` and silently become English with a locale nobody ships.
    #[test]
    fn cookie_naming_an_unshipped_or_junk_language_is_ignored() {
        assert_eq!(lang_from_cookie("counted_lang=xx"), None);
        assert_eq!(lang_from_cookie("counted_lang="), None);
        assert_eq!(lang_from_cookie("other=fr"), None);
        assert_eq!(lang_from_cookie(""), None);
        assert_eq!(lang_from_cookie("counted_lang=../../etc/passwd"), None);
    }

    #[test]
    fn accept_language_matches_on_the_primary_subtag() {
        assert_eq!(lang_from_accept("de-AT,de;q=0.9,en;q=0.8"), Some("de"));
        assert_eq!(lang_from_accept("pt-BR"), Some("pt"));
        assert_eq!(lang_from_accept("en-US,en;q=0.9"), Some("en"));
        assert_eq!(lang_from_accept("sr-Latn-RS"), Some("sr"));
    }

    /// Norwegian browsers send Bokmål or Nynorsk; the app ships one Norwegian file.
    #[test]
    fn norwegian_variants_land_on_no() {
        assert_eq!(lang_from_accept("nb-NO,nb;q=0.9,en;q=0.8"), Some("no"));
        assert_eq!(lang_from_accept("nn"), Some("no"));
        assert_eq!(lang_from_accept("no"), Some("no"));
    }

    /// The header is sent in preference order, so the first language we actually ship wins even
    /// when languages we do not ship come first.
    #[test]
    fn accept_language_skips_languages_we_do_not_ship() {
        assert_eq!(lang_from_accept("ja,ko;q=0.9,es;q=0.8"), Some("es"));
        assert_eq!(lang_from_accept("ja,ko"), None);
        assert_eq!(lang_from_accept(""), None);
    }

    #[test]
    fn fallback_is_a_shipped_language() {
        assert!(supported(FALLBACK).is_some());
    }

    /// `device_lang` reuses the header parser on a `navigator.language` value, which is a single
    /// tag rather than a comma-separated list. Same parser, different input shape — worth pinning,
    /// because the WebView is the only place mobile and desktop can infer a language from.
    #[test]
    fn a_navigator_language_tag_resolves_like_a_header() {
        assert_eq!(lang_from_accept("fr-FR"), Some("fr"));
        assert_eq!(lang_from_accept("pt-BR"), Some("pt"));
        assert_eq!(lang_from_accept("nl"), Some("nl"));
        // A device set to something we do not ship leaves the English fallback in place rather
        // than guessing.
        assert_eq!(lang_from_accept("ja-JP"), None);
        assert_eq!(lang_from_accept(""), None);
    }

    /// An inferred language must not be recorded as a choice, or the app would keep it after the
    /// device's own language changed. Only `set_language` writes.
    #[test]
    fn inference_is_validated_against_supported() {
        assert!(supported("fr").is_some());
        assert!(supported("ja").is_none());
    }

    #[test]
    fn supported_codes_are_unique_and_parse_as_language_identifiers() {
        for (code, label) in SUPPORTED {
            assert_eq!(langid_of(code).language.as_str(), *code, "{code} must parse to itself");
            assert!(!label.is_empty(), "{code} has no endonym");
            assert_eq!(
                SUPPORTED.iter().filter(|(c, _)| c == code).count(),
                1,
                "{code} is listed twice"
            );
        }
    }

    /// The bundle logic, without a Dioxus runtime: the active locale overrides the fallback per
    /// message, and what it lacks still comes from English.
    #[test]
    fn active_locale_overrides_the_fallback_per_message() {
        let resources = HashMap::from([
            (langid!("en"), "hello = Hello\nbye = Bye"),
            (langid!("fr"), "hello = Bonjour"),
        ]);
        let bundle = build_bundle(&langid!("fr"), &resources);
        let text = |id: &str| {
            let msg = bundle.get_message(id).unwrap();
            bundle.format_pattern(msg.value().unwrap(), None, &mut vec![]).into_owned()
        };
        assert_eq!(text("hello"), "Bonjour");
        assert_eq!(text("bye"), "Bye");
    }

    #[test]
    fn a_region_variant_resolves_through_its_language() {
        let resources = HashMap::from([
            (langid!("en"), "hello = Hello"),
            (langid!("de"), "hello = Hallo"),
        ]);
        let bundle = build_bundle(&langid!("de-AT"), &resources);
        let msg = bundle.get_message("hello").unwrap();
        assert_eq!(bundle.format_pattern(msg.value().unwrap(), None, &mut vec![]), "Hallo");
    }

    /// `tid!` on a missing id renders the id, and a resolved one renders the copy — through a
    /// real component, since `translate` consumes context.
    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn tid_renders_copy_or_the_bare_id() {
        #[component]
        fn Probe() -> Element {
            use_test_i18n();
            rsx! {
                p { {tid!("loading")} }
                p { {tid!("no-such-key")} }
                p { {tid!("offline-pending", count: 2)} }
            }
        }
        let mut dom = VirtualDom::new(Probe);
        let texts = crate::common::test_dom::texts(&dom.rebuild_to_vec());
        assert!(texts.iter().any(|t| t == "Loading…"), "{texts:?}");
        assert!(texts.iter().any(|t| t == "no-such-key"), "{texts:?}");
        assert!(texts.iter().any(|t| t.contains("2") && t.contains("pending")), "{texts:?}");
    }

    /// A locale added after the context exists renders once selected, and a key it lacks still
    /// falls back — the web build relies on this to switch to a locale it just fetched.
    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn a_locale_added_at_runtime_renders_once_selected() {
        #[component]
        fn Probe() -> Element {
            use_test_i18n();
            let mut i18n = i18n();
            use_hook(move || {
                i18n.add_locale(langid!("xx"), "loading = Chargement…");
                i18n.set_language(langid!("xx"));
            });
            rsx! {
                p { {tid!("loading")} }
                p { {tid!("cancel")} }
            }
        }
        let mut dom = VirtualDom::new(Probe);
        let texts = crate::common::test_dom::texts(&dom.rebuild_to_vec());
        assert!(texts.iter().any(|t| t == "Chargement…"), "{texts:?}");
        assert!(texts.iter().any(|t| t == "Cancel"), "{texts:?}");
    }

    /// The keys are strings, so nothing in the type system stops a typo. These read the `.ftl`
    /// files and the crate source off disk and close that gap: a mistyped key fails the build
    /// instead of rendering as itself in production.
    #[cfg(not(target_arch = "wasm32"))]
    mod ftl {
        use super::*;
        use std::collections::BTreeSet;
        use std::path::{Path, PathBuf};

        use super::super::locale_ids as ids;

        fn crate_dir() -> PathBuf {
            PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        }

        /// Every `.rs` under `src`, except this file: the scanner below quotes its own needle and
        /// its doc comments, so including it would report those as missing keys.
        fn rust_sources(dir: &Path, out: &mut Vec<PathBuf>) {
            for entry in std::fs::read_dir(dir).expect("readable src dir").flatten() {
                let path = entry.path();
                if path.is_dir() {
                    rust_sources(&path, out);
                } else if path.extension().is_some_and(|e| e == "rs")
                    && path.file_name().is_some_and(|f| f != "i18n.rs")
                {
                    out.push(path);
                }
            }
        }

        /// Every `tid!("…")` literal in the crate. A key built at runtime would slip past this —
        /// don't build keys at runtime.
        fn used_keys() -> BTreeSet<String> {
            let mut files = Vec::new();
            rust_sources(&crate_dir().join("src"), &mut files);

            let mut keys = BTreeSet::new();
            for file in files {
                let text = std::fs::read_to_string(&file).expect("readable source");
                for (_, after) in text.match_indices("tid!(").map(|(i, m)| (i, &text[i + m.len()..]))
                {
                    let Some(open) = after.find('"') else { continue };
                    if after[..open].contains(')') {
                        continue;
                    }
                    let rest = &after[open + 1..];
                    let Some(close) = rest.find('"') else { continue };
                    keys.insert(rest[..close].to_string());
                }
            }
            keys
        }

        /// Every string literal inside an `rsx! { … }` block, with the line it sits on, the text
        /// before it (on its line, or the previous line when it opens one) and the text after it
        /// on its line. `#[cfg(test)]` modules are skipped: harnesses label their fixtures freely.
        fn rsx_literals(text: &str) -> Vec<(usize, String, String, String)> {
            let text = text.split("#[cfg(test)]").next().unwrap_or("");
            let bytes = text.as_bytes();
            let mut out = Vec::new();
            let mut depth = 0usize;
            let mut i = 0;
            while i < bytes.len() {
                let rest = &text[i..];
                if rest.starts_with("//") {
                    i += rest.find('\n').unwrap_or(rest.len());
                    continue;
                }
                if depth == 0 {
                    if rest.starts_with("rsx! {") || rest.starts_with("rsx!{") {
                        depth = 1;
                        i += rest.find('{').unwrap() + 1;
                    } else {
                        i += rest.chars().next().unwrap().len_utf8();
                    }
                    continue;
                }
                match bytes[i] {
                    b'{' => depth += 1,
                    b'}' => depth -= 1,
                    b'"' => {
                        let mut j = i + 1;
                        while j < bytes.len() && bytes[j] != b'"' {
                            j += if bytes[j] == b'\\' { 2 } else { 1 };
                        }
                        let line_start = text[..i].rfind('\n').map_or(0, |n| n + 1);
                        let line_end = text[j..].find('\n').map_or(text.len(), |n| j + n);
                        let mut before = text[line_start..i].trim();
                        if before.is_empty() {
                            before = text[..line_start].trim_end().lines().last().unwrap_or("").trim();
                        }
                        out.push((
                            text[..i].matches('\n').count() + 1,
                            text[i + 1..j].to_string(),
                            before.to_string(),
                            text[j + 1..line_end].trim().to_string(),
                        ));
                        i = j;
                    }
                    _ => {}
                }
                i += 1;
            }
            out
        }

        /// A literal is a text node when rsx would render it as one: alone on its line, or after
        /// the attributes and closing the element. Attribute values (`class: "…"`), macro
        /// arguments (`tid!("…")`) and `if c { "…" } else { "…" }` values are not.
        fn is_text_node(before: &str, after: &str) -> bool {
            if !(after.is_empty() || after.starts_with('}')) {
                return false;
            }
            if before.ends_with([',', '}', '"']) {
                return true;
            }
            let Some(head) = before.strip_suffix('{') else { return false };
            let head = head.trim_end();
            !head.ends_with("else") && !head.split_whitespace().any(|w| w == "if")
        }

        /// What remains of a literal once `{ … }` interpolations are removed — copy is what has
        /// letters left, `"{amount:.0}"` and `"{percent:.0}%"` have none.
        fn without_interpolations(literal: &str) -> String {
            let mut out = String::new();
            let mut depth = 0usize;
            for c in literal.chars() {
                match c {
                    '{' => depth += 1,
                    '}' => depth = depth.saturating_sub(1),
                    _ if depth == 0 => out.push(c),
                    _ => {}
                }
            }
            out
        }

        /// Proper nouns and identifiers that render as themselves in every language.
        const NOT_COPY: &[&str] = &[
            "Counted",
            "contact@counted.fr",
            "www.hetzner.com",
            "github.com/counted-labs/counted",
            "Hetzner Online GmbH",
            "Hetzner Online GmbH ",
            "Scaleway TEM ",
            "Bunq / Tricount ",
            "Grafana Labs ",
            "Jonathan Bosi.",
            "CNIL",
            "HttpOnly",
            "SameSite=Lax",
            "Secure",
            "counted_lang",
            "zero-knowledge",
        ];

        /// Copy that never went through `tid!` renders in whatever language it was typed in,
        /// for every user. This is the guard for that.
        #[test]
        fn no_hardcoded_copy_in_rsx() {
            let mut files = Vec::new();
            rust_sources(&crate_dir().join("src"), &mut files);

            let mut offenders = Vec::new();
            for file in files {
                let text = std::fs::read_to_string(&file).expect("readable source");
                for (line, literal, before, after) in rsx_literals(&text) {
                    if is_text_node(&before, &after)
                        && without_interpolations(&literal).chars().any(char::is_alphabetic)
                        && !NOT_COPY.contains(&literal.as_str())
                    {
                        let path = file.strip_prefix(crate_dir()).unwrap_or(&file).display();
                        offenders.push(format!("{path}:{line}: {literal:?}"));
                    }
                }
            }
            assert!(offenders.is_empty(), "copy in rsx! that does not go through tid!:\n{}", offenders.join("\n"));
        }

        /// The guard that makes string keys safe. `tid!` renders an unknown key as itself, which
        /// is quiet in a test suite and loud in production.
        #[test]
        fn every_key_used_in_the_source_exists_in_the_fallback_locale() {
            let available = ids(FALLBACK);
            let missing: Vec<_> = used_keys().difference(&available).cloned().collect();
            assert!(missing.is_empty(), "keys used in rsx! but absent from {FALLBACK}.ftl: {missing:?}");
        }

        /// The legal, terms and privacy pages exist in English and French only, on purpose: they
        /// are not machine-translated, and every other locale renders them in English per message
        /// until a translation has been reviewed. See the `### Legal` header in `en.ftl`.
        fn is_legal(id: &str) -> bool {
            id.starts_with("legal-") || id.starts_with("terms-") || id.starts_with("privacy-")
        }

        /// Every locale defines exactly the fallback's ids (legal pages aside, above). A missing one
        /// would render in English for that language's users; an extra one is a typo or a key
        /// renamed everywhere but here, and would silently never render.
        #[test]
        fn every_locale_defines_the_same_messages_as_the_fallback() {
            let fallback = ids(FALLBACK);
            for (code, _) in SUPPORTED.iter().filter(|(c, _)| *c != FALLBACK) {
                let locale = ids(code);
                let missing: Vec<_> = fallback
                    .difference(&locale)
                    .filter(|id| *code == "fr" || !is_legal(id))
                    .cloned()
                    .collect();
                let orphans: Vec<_> = locale.difference(&fallback).cloned().collect();
                assert!(missing.is_empty(), "{code}.ftl lacks: {missing:?}");
                assert!(orphans.is_empty(), "{code}.ftl defines unknown messages: {orphans:?}");
                if *code != "fr" {
                    let legal: Vec<_> = locale.iter().filter(|id| is_legal(id)).collect();
                    assert!(legal.is_empty(), "{code}.ftl translates legal copy, which is en/fr only: {legal:?}");
                }
            }
        }

        /// A syntax error in a locale file used to panic at the first render on that locale. Fluent
        /// now keeps what parses, so this is where it gets caught.
        #[test]
        fn every_locale_parses_as_fluent() {
            for (code, _) in SUPPORTED {
                if let Err((_, errors)) = FluentResource::try_new(locale_text(code)) {
                    panic!("{code}.ftl has syntax errors: {errors:?}");
                }
            }
        }

        /// The plural messages must name every category the language's CLDR rules produce, or one
        /// count renders the `*[other]` form in a language where that form is wrong.
        #[test]
        fn plural_messages_cover_the_language_cardinal_categories() {
            use intl_pluralrules::{PluralCategory, PluralRuleType, PluralRules};
            let plural_ids = ["offline-pending", "charts-expense-count", "charts-my-share-skipped", "invite-sent"];
            for (code, _) in SUPPORTED {
                let rules = PluralRules::create(langid_of(code), PluralRuleType::CARDINAL)
                    .unwrap_or_else(|e| panic!("no plural rules for {code}: {e}"));
                let text = locale_text(code);
                for id in plural_ids {
                    let start = text.find(&format!("{id} =")).unwrap_or_else(|| panic!("{code}.ftl lacks {id}"));
                    let body: String = text[start..]
                        .lines()
                        .skip(1)
                        .take_while(|l| l.starts_with([' ', '\t']))
                        .collect::<Vec<_>>()
                        .join("\n");
                    // The counts are integers; every integer category shows up below 1000.
                    let categories: BTreeSet<&str> = (0..1000)
                        .map(|n| match rules.select(n).unwrap() {
                            PluralCategory::ZERO => "zero",
                            PluralCategory::ONE => "one",
                            PluralCategory::TWO => "two",
                            PluralCategory::FEW => "few",
                            PluralCategory::MANY => "many",
                            PluralCategory::OTHER => "other",
                        })
                        .collect();
                    for name in categories {
                        assert!(body.contains(&format!("[{name}]")), "{code}.ftl: {id} lacks the [{name}] variant");
                    }
                }
            }
        }
    }
}
