//! The colour theme, a choice of this device alone: it is never synced to the account, so the
//! server learns nothing from it. Light unless chosen otherwise.
//!
//! Applied as `data-theme` on `<html>`, so the page, its `color-scheme` and every sheet follow it.
//! "System" is resolved on the client from `prefers-color-scheme`, live: the CSS keys on the two
//! daisyUI themes only and never on the media query, so a theme chosen here always wins over the
//! system's.

use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, OnceLock};

use super::persist::{read_json, write_json};
use crate::common::{haptic, Haptic};

const THEME_KEY: &str = "counted_theme";

/// `assets/boot.js`, the one blocking script: `data-theme` (and the web's `theme-color` meta) from
/// the stored choice before first paint, `window.countedTheme` with the System listener, the
/// passive `touchstart` that gives Safari its `:active`, and the capture-phase `scroll` listener
/// that toggles `.nav-material.is-scrolled` (no Dioxus `onscroll`: a bridge call per frame on
/// mobile). The web loads it same-origin with `document::Script` (CSP `'self'`); mobile inlines
/// the same source in its head script, where the theme comes from Rust and `countedTheme` is called
/// right after. It reads the store `write_json` writes here, so the tests pin that format.
/// Unminified, so the served bytes are the source's and `BOOT_JS_INTEGRITY` can pin them: nginx's
/// `Integrity-Policy` refuses a script without `integrity`, and a no-cors one even with it, so the
/// web tag also carries `crossorigin` (docs/dioxus-overrides.md §17).
pub const BOOT_JS: Asset = asset!("/assets/boot.js", AssetOptions::js().with_minify(false));
pub const BOOT_JS_INTEGRITY: &str = "sha384-jqiXaidSyOC/hi2UeimuyHvBYQUhye6tso8U1hbQ58gMbh6Zj9YLnc5EYJOi36x7";
/// `assets/native-feel.css`, linked by web and mobile between tailwind.css and their main.css.
pub const NATIVE_FEEL_CSS: Asset = asset!("/assets/native-feel.css");
pub const BOOT_JS_SOURCE: &str = include_str!("../../assets/boot.js");

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Debug, Default)]
#[serde(rename_all = "lowercase")]
pub enum Theme {
    #[default]
    Light,
    Dark,
    System,
}

impl Theme {
    pub const ALL: [Theme; 3] = [Theme::Light, Theme::Dark, Theme::System];

    pub fn as_str(self) -> &'static str {
        match self {
            Theme::Light => "light",
            Theme::Dark => "dark",
            Theme::System => "system",
        }
    }

    pub fn is_dark(self, system_dark: bool) -> bool {
        match self {
            Theme::Light => false,
            Theme::Dark => true,
            Theme::System => system_dark,
        }
    }

    pub fn label_key(self) -> &'static str {
        match self {
            Theme::Light => "theme-light",
            Theme::Dark => "theme-dark",
            Theme::System => "theme-system",
        }
    }
}

pub fn data_theme(dark: bool) -> &'static str {
    if dark {
        "counted-dark"
    } else {
        "counted"
    }
}

/// The `theme-color` meta of the web build (Safari's toolbar, Android's status bar): `base-200`
/// in sRGB, the page's ground, the same as Android's `windowBackground`.
pub fn theme_color(dark: bool) -> &'static str {
    if dark {
        "#1B221F"
    } else {
        "#F4F8F6"
    }
}

/// A store of its own rather than a field of `LocalStorageState`: a value a newer build wrote fails
/// to parse here alone and reads as Light, instead of failing the blob that holds the keys.
pub fn stored_theme() -> Theme {
    read_json(THEME_KEY)
}

pub type NativeTheme = Arc<dyn Fn(Theme) + Send + Sync + 'static>;

static NATIVE: OnceLock<NativeTheme> = OnceLock::new();

/// The status bar's icons follow the system on Android and iOS, not the page, so the mobile entry
/// point installs this to keep them readable on the chosen theme. Unset elsewhere.
pub fn set_native_theme(apply: NativeTheme) {
    let _ = NATIVE.set(apply);
}

/// Sets `<html data-theme>` and the native chrome. On mobile `countedTheme` (boot.js, in the head
/// script) owns the `prefers-color-scheme` listener.
pub fn apply_theme(theme: Theme) {
    #[cfg(target_arch = "wasm32")]
    crate::common::web_dom::set_theme(theme);
    #[cfg(not(target_arch = "wasm32"))]
    document::eval(&format!("window.countedTheme && window.countedTheme('{}');", theme.as_str()));
    if let Some(apply) = NATIVE.get() {
        apply(theme);
    }
}

/// The picker's selection. Light on the first render, the stored choice once mounted: the server
/// cannot read this device's store, and the first client render has to match what it served.
pub fn use_theme_choice() -> Signal<Theme> {
    let mut choice = use_signal(Theme::default);
    use_effect(move || choice.set(stored_theme()));
    choice
}

pub fn choose_theme(mut choice: Signal<Theme>, theme: Theme) {
    haptic(Haptic::Selection);
    choice.set(theme);
    write_json(THEME_KEY, "theme", &theme);
    apply_theme(theme);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn light_is_the_default() {
        assert_eq!(Theme::default(), Theme::Light);
        assert_eq!(serde_json::from_str::<Theme>("\"system\"").unwrap(), Theme::System);
        assert_eq!(serde_json::to_string(&Theme::Dark).unwrap(), "\"dark\"");
    }

    #[test]
    fn boot_js_reads_the_store_this_module_writes() {
        assert!(BOOT_JS_SOURCE.contains(&format!("JSON.parse(localStorage.getItem('{THEME_KEY}'))")));
        for theme in [Theme::Dark, Theme::System] {
            assert_eq!(serde_json::to_string(&theme).unwrap(), format!("\"{}\"", theme.as_str()));
            assert!(BOOT_JS_SOURCE.contains(&format!("t === '{}'", theme.as_str())), "{theme:?}");
        }
        assert!(BOOT_JS_SOURCE.contains(&format!("'{}' : '{}'", data_theme(true), data_theme(false))));
    }

    #[test]
    fn boot_js_integrity_pins_its_bytes() {
        use base64::Engine;
        use sha2::Digest;
        let digest = sha2::Sha384::digest(BOOT_JS_SOURCE.as_bytes());
        let pin = format!("sha384-{}", base64::engine::general_purpose::STANDARD.encode(digest));
        assert_eq!(pin, BOOT_JS_INTEGRITY, "boot.js changed: update BOOT_JS_INTEGRITY");
    }

    #[test]
    fn every_theme_color_source_agrees() {
        assert!(BOOT_JS_SOURCE.contains(&format!("'{}' : '{}'", theme_color(true), theme_color(false))));
        let light = format!("\"{}\"", theme_color(false));
        assert!(include_str!("../../../web/index.html").contains(&format!("name=\"theme-color\" content={light}")));
        let manifest = include_str!("../../../web/assets/pwa/manifest.webmanifest");
        assert!(manifest.contains(&format!("\"theme_color\": {light}")));
        assert!(manifest.contains(&format!("\"background_color\": {light}")));
    }

    #[test]
    fn only_system_follows_the_device() {
        for system_dark in [false, true] {
            assert!(!Theme::Light.is_dark(system_dark));
            assert!(Theme::Dark.is_dark(system_dark));
            assert_eq!(Theme::System.is_dark(system_dark), system_dark);
        }
    }

    #[test]
    fn a_theme_from_a_newer_build_reads_as_light() {
        let _guard = crate::common::env_lock();
        let dir = std::env::temp_dir().join(format!("counted-theme-{}", uuid::Uuid::new_v4()));
        std::fs::create_dir_all(&dir).unwrap();
        std::env::set_var("COUNTED_DATA_DIR", &dir);
        std::fs::write(dir.join(format!("{THEME_KEY}.json")), "\"midnight\"").unwrap();

        assert_eq!(stored_theme(), Theme::Light);

        std::env::remove_var("COUNTED_DATA_DIR");
        let _ = std::fs::remove_dir_all(&dir);
    }
}
