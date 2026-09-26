//! The native push seam. The mobile entry point installs a [`NativePush`] before launch; every
//! other target leaves it unset and the whole feature is invisible.
//!
//! The device token never crosses this seam as a value: the platform layer writes it to
//! `COUNTED_DATA_DIR/push_token` (iOS from the app-delegate callback, Android from the UnifiedPush
//! service), and [`read_push_token`] reads it back. One file, two platforms, no channel — and a
//! token that arrives while the app is asleep is still there at the next boot.

use std::sync::{Arc, OnceLock};

use shared::{PushPlatform, WebPushKeys};

pub const TOKEN_FILE: &str = "push_token";
/// Where Android's `CountedPushService` drops the proof from the server's verification push.
pub const PROOF_FILE: &str = "push_proof";

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

/// Reads and removes the pending verification proof. Removed whatever the verify call then does: a
/// proof works once, and one lost to a failed call is replaced by the server's next challenge.
pub fn take_push_proof() -> Option<String> {
    #[cfg(not(target_arch = "wasm32"))]
    {
        let path = super::persist::data_dir().join(PROOF_FILE);
        let proof = std::fs::read_to_string(&path).ok()?;
        let _ = std::fs::remove_file(&path);
        let proof = proof.trim();
        (!proof.is_empty()).then(|| proof.to_string())
    }
    #[cfg(target_arch = "wasm32")]
    {
        None
    }
}

/// The Web Push subscription Android's `CountedPushService` writes as JSON.
#[derive(serde::Deserialize)]
struct WebPushSubscription {
    endpoint: String,
    p256dh: String,
    auth: String,
}

/// Splits the file's contents into what `RegisterPushToken` carries: an Android subscription is
/// its endpoint plus keys, anything else (the iOS hex token) is the token alone.
pub fn parse_push_token(raw: &str) -> (String, Option<WebPushKeys>) {
    match serde_json::from_str::<WebPushSubscription>(raw) {
        Ok(sub) => (sub.endpoint, Some(WebPushKeys { p256dh: sub.p256dh, auth: sub.auth })),
        Err(_) => (raw.to_string(), None),
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
    fn android_subscription_splits_into_endpoint_and_keys() {
        let raw = r#"{"endpoint":"https://ntfy.sh/upX?up=1","p256dh":"BAbc","auth":"xyz"}"#;
        let (token, keys) = parse_push_token(raw);
        assert_eq!(token, "https://ntfy.sh/upX?up=1");
        assert_eq!(keys, Some(WebPushKeys { p256dh: "BAbc".into(), auth: "xyz".into() }));
    }

    #[test]
    fn ios_hex_token_is_the_token_alone() {
        let hex = "ab".repeat(32);
        assert_eq!(parse_push_token(&hex), (hex.clone(), None));
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

        assert_eq!(take_push_proof(), None);
        std::fs::write(dir.join(PROOF_FILE), "proof-1\n").unwrap();
        assert_eq!(take_push_proof().as_deref(), Some("proof-1"));
        assert_eq!(take_push_proof(), None, "a proof is taken once");

        std::env::remove_var("COUNTED_DATA_DIR");
        let _ = std::fs::remove_dir_all(&dir);
    }
}
