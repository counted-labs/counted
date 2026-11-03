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

/// Sets `<html data-theme>` and the native chrome. On mobile the head script defines
/// `countedTheme`, which owns the `prefers-color-scheme` listener.
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
    haptic(Haptic::Light);
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
