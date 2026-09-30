use std::sync::{Arc, OnceLock};

/// Opens the platform share sheet with `text`, true when the sheet was shown.
pub type NativeSharer = Arc<dyn Fn(&str) -> bool + Send + Sync + 'static>;

static SHARER: OnceLock<NativeSharer> = OnceLock::new();

/// Installs the platform sharer. The mobile entry point calls this once before launch; every other
/// native target leaves it unset, so the Share action falls back to the clipboard. Later calls
/// are ignored.
pub fn set_native_sharer(sharer: NativeSharer) {
    let _ = SHARER.set(sharer);
}

/// Opens the share sheet with `text`. False when there is no sheet (`navigator.share` absent on
/// web, no sharer installed elsewhere) or it could not be shown; the user dismissing it is not a
/// failure. Same process-global shape as `haptics`, for the same reason.
pub async fn share_text(text: &str) -> bool {
    #[cfg(target_arch = "wasm32")]
    {
        super::web_dom::share_url(text).await
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        SHARER.get().map(|share| share(text)).unwrap_or(false)
    }
}

/// Hands a URL the webview cannot open to the OS, true when something took it.
///
/// The native interpreter routes every link through `webbrowser::open`, which on Android and iOS
/// opens http(s) only and fails anything else — so a `mailto:` tap did nothing.
pub type NativeOpener = Arc<dyn Fn(&str) -> bool + Send + Sync + 'static>;

static OPENER: OnceLock<NativeOpener> = OnceLock::new();

pub fn set_native_opener(opener: NativeOpener) {
    let _ = OPENER.set(opener);
}

pub fn open_external(url: &str) -> bool {
    OPENER.get().map(|open| open(url)).unwrap_or(false)
}

/// Writes `bytes` as `filename` where the platform can share it, and opens the share sheet on the
/// file. The platform picks the directory: Android can only share what its FileProvider exposes.
pub type NativeFileSharer =
    Arc<dyn Fn(&str, &[u8], &str) -> Result<(), String> + Send + Sync + 'static>;

static FILE_SHARER: OnceLock<NativeFileSharer> = OnceLock::new();

pub fn set_native_file_sharer(sharer: NativeFileSharer) {
    let _ = FILE_SHARER.set(sharer);
}

pub fn share_file(filename: &str, bytes: &[u8], mime: &str) -> Result<(), String> {
    match FILE_SHARER.get() {
        Some(share) => share(filename, bytes, mime),
        None => Err("no share sheet on this platform".into()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::future::Future;
    use std::task::{Context, Poll, Waker};

    /// The SSR server binary installs nothing, and the action must fall through to the clipboard
    /// there. Off wasm32 the future never awaits, so one poll resolves it.
    #[test]
    fn share_without_a_sharer_returns_false() {
        let fut = std::pin::pin!(share_text("x"));
        let polled = fut.poll(&mut Context::from_waker(Waker::noop()));
        assert_eq!(polled, Poll::Ready(false));
    }
}
