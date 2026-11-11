use std::sync::OnceLock;

static VERSION: OnceLock<&'static str> = OnceLock::new();

/// The store version of the phone apps, set once by the mobile entry point. The web build has no
/// release number of its own (`ui` and `web` stay 0.1.0), so it shows none.
pub fn set_app_version(version: &'static str) {
    let _ = VERSION.set(version);
}

pub fn app_version() -> Option<&'static str> {
    VERSION.get().copied()
}
