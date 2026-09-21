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
