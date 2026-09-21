//! The native push seam. The mobile entry point installs a [`NativePush`] before launch; every
//! other target leaves it unset and the whole feature is invisible.
//!
//! The device token never crosses this seam as a value: the platform layer writes it to
//! `COUNTED_DATA_DIR/push_token` (iOS from the app-delegate callback, Android from the Firebase
//! service), and [`read_push_token`] reads it back. One file, two platforms, no channel — and a
//! token that arrives while the app is asleep is still there at the next boot.

use std::sync::{Arc, OnceLock};

use shared::PushPlatform;

pub const TOKEN_FILE: &str = "push_token";

pub struct NativePush {
    pub platform: PushPlatform,
    /// Asks the OS for permission and for a token; the token lands in the file, asynchronously.
    pub request: Arc<dyn Fn() + Send + Sync + 'static>,
}

static NATIVE: OnceLock<NativePush> = OnceLock::new();

/// Same construction as `set_haptics_player`: once, before launch, later calls ignored.
pub fn set_native_push(push: NativePush) {
    let _ = NATIVE.set(push);
}

pub fn native_push() -> Option<&'static NativePush> {
    NATIVE.get()
}

#[cfg(not(target_arch = "wasm32"))]
pub fn token_path() -> std::path::PathBuf {
    super::persist::data_dir().join(TOKEN_FILE)
}

/// `None` when no token has been delivered yet, or on a target that has no file to read.
pub fn read_push_token() -> Option<String> {
    #[cfg(not(target_arch = "wasm32"))]
    {
        let token = std::fs::read_to_string(token_path()).ok()?;
        let token = token.trim();
        (!token.is_empty()).then(|| token.to_string())
    }
    #[cfg(target_arch = "wasm32")]
    {
        None
    }
}

/// Sibling tmp file and rename, so a kill mid-write leaves the previous token rather than an empty
/// file. Called by the platform layer, which owns the token's arrival.
#[cfg(not(target_arch = "wasm32"))]
pub fn write_push_token(token: &str) -> std::io::Result<()> {
    let path = token_path();
    let tmp = path.with_extension("tmp");
    std::fs::write(&tmp, token)?;
    std::fs::rename(&tmp, &path)
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use super::*;

    #[test]
    fn no_platform_installed_means_no_push() {
        assert!(native_push().is_none());
    }

    #[test]
    fn token_round_trips_through_the_data_dir() {
        let _guard = crate::common::env_lock();
        let dir = std::env::temp_dir().join(format!("counted-push-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        std::env::set_var("COUNTED_DATA_DIR", dir.to_str().unwrap());

        assert_eq!(read_push_token(), None);
        write_push_token("abc123").unwrap();
        assert_eq!(read_push_token().as_deref(), Some("abc123"));
        write_push_token("  ").unwrap();
        assert_eq!(read_push_token(), None);

        std::env::remove_var("COUNTED_DATA_DIR");
        let _ = std::fs::remove_dir_all(&dir);
    }
}
