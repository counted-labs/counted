//! Receipt scanning: the camera, the transport, and the platform hooks the OCR crate is reached
//! through. See `docs/plans/ocr-receipt-scan.md`.
//!
//! Nothing here decodes an image or runs a model — `packages/ocr` does that, and `ui` must not
//! depend on it (31 MB of weights would land in the wasm bundle and the server binary). The
//! platform half is installed by `packages/mobile/src/main.rs`.

use std::sync::{Arc, Mutex, OnceLock};

/// What OCR found, in the shapes the expense form takes. `date` is `%Y-%m-%d` because that is what
/// `ExpenseForm`'s `date_str` signal holds.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ScanFields {
    pub title: Option<String>,
    pub amount: Option<f64>,
    pub date: Option<String>,
    /// False when the amount came from the bottom-of-page fallback rather than a keyword anchor.
    pub amount_confident: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScanError {
    /// No scanner installed — web, desktop, the server binary.
    Unsupported,
    /// The user backed out of the camera or the picker. Not a failure, and never shown.
    Cancelled,
    /// The WebView handed back something that is not a decodable image.
    Capture,
    /// The models ran and found nothing usable.
    Unreadable,
}

/// Where the photo comes from. Only the `capture` attribute differs, and it is decisive: with it,
/// wry's Android chooser launches `ACTION_IMAGE_CAPTURE` and nothing else, and WKWebView opens the
/// camera directly; without it, Android gets the document picker only and iOS its own sheet. No
/// single input yields both on Android, so the choice is made in the UI, before the input exists.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScanSource {
    Camera,
    Library,
}

/// A key, not a sentence — DOCUMENTATION §11.1. `Cancelled` and `Unsupported` have no key because
/// neither is ever surfaced: backing out of the camera is not an error, and a platform with no
/// scanner never renders the button.
pub fn scan_error_key(error: ScanError) -> &'static str {
    match error {
        ScanError::Unreadable => "scan-error-unreadable",
        _ => "scan-error-capture",
    }
}

/// The platform half of scanning, installed once by the mobile entry point.
///
/// Three functions rather than one because the temp-file purge is asymmetric. On Android the
/// capture directory is the app's alone and `purge` can empty it; on iOS WKWebView drops its copy
/// into a shared `NSTemporaryDirectory()`, so only entries that appeared *during* the pick may be
/// removed — `arm` is where iOS takes that snapshot and where Android does nothing.
#[derive(Clone)]
pub struct NativeScan {
    /// Blocking and CPU-bound. Called on a worker thread, never on the event loop.
    pub recognise: Arc<dyn Fn(Vec<u8>) -> Result<ScanFields, ScanError> + Send + Sync>,
    pub arm: Arc<dyn Fn() + Send + Sync>,
    pub purge: Arc<dyn Fn() + Send + Sync>,
}

static NATIVE: OnceLock<NativeScan> = OnceLock::new();

/// Installed before `launch`, exactly like `set_haptics_player`. Later calls are ignored.
///
/// A process-global rather than a Dioxus context: `packages/desktop/src/main.rs` provides no
/// native-capability contexts at all, so a `use_context` in `ExpensesTab` — a main-path component,
/// not a rarely-opened modal — would panic at render there. The only value stored is a function
/// the entry point owns for the life of the process.
pub fn set_native_scan(scan: NativeScan) {
    let _ = NATIVE.set(scan);
}

/// The render gate for the Scan button. False on web, desktop and the server binary, because
/// nothing there installs a scanner.
pub fn scanner_available() -> bool {
    NATIVE.get().is_some()
}

/// Base64 sent per message. The bridge is `window.ipc.postMessage`, and a whole 1600px JPEG is
/// ~340-540 KB of base64 — a few messages rather than one very large one.
const CHUNK_BYTES: usize = 262_144;
/// Ceiling on a reassembled capture. A bound, not a threat model: the page that produces these is
/// ours, and the phone would run out of memory long before anything interesting happened.
const MAX_CAPTURE_BYTES: usize = 6 * 1024 * 1024;
/// Long edge the photo is downscaled to before transport. The detector wants ~960, but recognition
/// crops come off the source and receipt print is small.
const MAX_EDGE: u32 = 1600;
/// Higher than a photo would need. Thermal receipt print is thin, low-contrast and exactly the
/// spatial frequency JPEG ringing destroys, and the detector cannot recover a stroke the codec
/// removed. The extra few hundred KB stay far inside `MAX_CAPTURE_BYTES`.
const JPEG_QUALITY: f32 = 0.92;

/// Opens the camera or the picker and streams the photo back, base64 in chunks.
///
/// **The input is created in JS and appended to `document.body`, outside the Dioxus root, and that
/// is load-bearing.** dioxus-interpreter's `native.js` installs a window click listener that
/// intercepts every `input[type=file]`, calls `preventDefault()` and routes to `__file_dialog`
/// (dioxus-desktop `protocol.rs` -> `file_upload.rs`), which calls `rfd::FileDialog` — and `rfd`
/// has no Android or iOS backend, so a file input written in RSX does nothing at all on a phone.
/// That listener only fires when `getTargetId(target)` is non-null, and `getTargetId` walks
/// `parentNode` looking for `data-dioxus-id`; an element with no such ancestor is skipped and the
/// WebView's own chooser runs. See `docs/dioxus-overrides.md`.
///
/// The bytes come back over `dioxus.send()` and **not** a custom protocol: on Android wry
/// intercepts through `RustWebViewClient.shouldInterceptRequest`, and Android's
/// `WebResourceRequest` carries no request body, so a POST would arrive empty.
///
/// Dioxus runs this as the body of an async function, so the top-level `await` is what suspends it
/// — same contract as `sleep_js` and `copy_js`.
fn capture_js(source: ScanSource) -> String {
    let capture = match source {
        ScanSource::Camera => "input.capture = 'environment';",
        ScanSource::Library => "",
    };
    format!(
        r#"
        const CHUNK = {CHUNK_BYTES}, MAX_EDGE = {MAX_EDGE}, QUALITY = {JPEG_QUALITY};
        return await new Promise((resolve) => {{
            const input = document.createElement('input');
            input.type = 'file';
            input.accept = 'image/*';
            {capture}
            input.style.cssText = 'position:fixed;left:-10000px;opacity:0';
            document.body.appendChild(input);

            let settled = false, picking = false;
            const done = (status) => {{
                if (settled) return;
                settled = true;
                input.remove();
                dioxus.send({{ kind: 'end', value: status }});
                resolve(status);
            }};

            input.addEventListener('change', () => {{
                const file = input.files && input.files[0];
                if (!file) return done('cancel');
                picking = true;
                const url = URL.createObjectURL(file);
                const img = new Image();
                img.onerror = () => {{ URL.revokeObjectURL(url); done('capture'); }};
                img.onload = () => {{
                    URL.revokeObjectURL(url);
                    const long = Math.max(img.width, img.height);
                    const s = long > MAX_EDGE ? MAX_EDGE / long : 1;
                    const c = document.createElement('canvas');
                    c.width = Math.round(img.width * s);
                    c.height = Math.round(img.height * s);
                    c.getContext('2d').drawImage(img, 0, 0, c.width, c.height);
                    const data = c.toDataURL('image/jpeg', QUALITY);
                    c.width = 0; c.height = 0;
                    const b64 = data.slice(data.indexOf(',') + 1);
                    for (let i = 0; i < b64.length; i += CHUNK) {{
                        dioxus.send({{ kind: 'chunk', value: b64.slice(i, i + CHUNK) }});
                    }}
                    done('ok');
                }};
                img.src = url;
            }});

            // Chromium fires `cancel` since 113, but the Android System WebView on an older device
            // does not, and with no second signal the spinner would hang forever.
            input.addEventListener('cancel', () => done('cancel'));
            // Guarded on `picking`, and that guard is load-bearing: `focus` fires the moment the
            // picker closes, but a 12MP gallery photo can take longer than the timeout to decode,
            // scale and re-encode. Unguarded, the timer wins the race and silently discards a
            // photo the user did choose. Once `change` has a file, that handler owns the outcome.
            window.addEventListener('focus', () => setTimeout(() => {{
                if (!picking) done('cancel');
            }}, 1500), {{ once: true }});

            input.click();
        }});
    "#
    )
}

#[derive(Debug, Clone, serde::Deserialize)]
pub(crate) struct CaptureMsg {
    pub kind: String,
    pub value: String,
}

/// Reassembles the chunk stream. Pure, so the protocol is testable without a WebView.
#[derive(Default)]
pub(crate) struct Assembler {
    base64: String,
}

impl Assembler {
    /// `Some` once the stream terminates: the decoded JPEG, or why not.
    pub(crate) fn push(&mut self, msg: &CaptureMsg) -> Option<Result<Vec<u8>, ScanError>> {
        match msg.kind.as_str() {
            "chunk" => {
                if self.base64.len() + msg.value.len() > MAX_CAPTURE_BYTES {
                    return Some(Err(ScanError::Capture));
                }
                self.base64.push_str(&msg.value);
                None
            }
            "end" => Some(match msg.value.as_str() {
                "cancel" => Err(ScanError::Cancelled),
                "ok" if self.base64.is_empty() => Err(ScanError::Capture),
                "ok" => {
                    use base64::Engine;
                    base64::engine::general_purpose::STANDARD
                        .decode(&self.base64)
                        .map_err(|_| ScanError::Capture)
                }
                _ => Err(ScanError::Capture),
            }),
            _ => Some(Err(ScanError::Capture)),
        }
    }
}

/// Opens the camera or the picker, reads the photo, runs OCR. The whole feature, from the UI's
/// point of view.
pub async fn capture_and_scan(source: ScanSource) -> Result<ScanFields, ScanError> {
    let Some(native) = NATIVE.get() else {
        return Err(ScanError::Unsupported);
    };

    (native.arm)();
    let captured = capture_jpeg(source).await;
    // Unconditional, and before the `?`: the platform's own copy of the photo goes whether the scan
    // succeeded, failed, or the user backed out of the camera. That promise is in the privacy
    // policy, and it is not conditional on the happy path.
    (native.purge)();
    let jpeg = captured?;

    recognise_off_thread(native, jpeg).await
}

#[cfg(not(target_arch = "wasm32"))]
async fn capture_jpeg(source: ScanSource) -> Result<Vec<u8>, ScanError> {
    let mut eval = dioxus::prelude::document::eval(&capture_js(source));
    let mut assembler = Assembler::default();
    while let Ok(msg) = eval.recv::<CaptureMsg>().await {
        if let Some(done) = assembler.push(&msg) {
            return done;
        }
    }
    Err(ScanError::Capture)
}

/// Web installs no scanner, so this is unreachable — but `ui` is compiled for wasm and the eval
/// path above is CSP-blocked there (DOCUMENTATION §8).
#[cfg(target_arch = "wasm32")]
async fn capture_jpeg(_source: ScanSource) -> Result<Vec<u8>, ScanError> {
    Err(ScanError::Unsupported)
}

/// Inference takes one to three seconds, and on both phones the Dioxus event loop **is** the
/// platform UI thread — running it inline is an ANR on Android and a watchdog kill on iOS.
///
/// Polling rather than a channel: a oneshot would cost two new dependencies for a wait that is
/// invisible next to the inference itself.
#[cfg(not(target_arch = "wasm32"))]
async fn recognise_off_thread(
    native: &'static NativeScan,
    jpeg: Vec<u8>,
) -> Result<ScanFields, ScanError> {
    type Slot = Arc<Mutex<Option<Result<ScanFields, ScanError>>>>;
    let slot: Slot = Arc::new(Mutex::new(None));

    let out = slot.clone();
    let recognise = native.recognise.clone();
    std::thread::spawn(move || {
        // The slot is this thread's only way to report back, so a panic must still fill it. An
        // index out of bounds anywhere in the model would otherwise leave it `None` and spin the
        // loop below forever — the user sees a spinner that never stops instead of an error, and
        // the button stays disabled for the life of the process.
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| recognise(jpeg)))
            .unwrap_or(Err(ScanError::Unreadable));
        if let Ok(mut guard) = out.lock() {
            *guard = Some(result);
        }
    });

    loop {
        if let Some(done) = slot.lock().ok().and_then(|mut g| g.take()) {
            return done;
        }
        crate::common::sleep(POLL_MS).await;
    }
}

#[cfg(not(target_arch = "wasm32"))]
const POLL_MS: u32 = 120;

#[cfg(target_arch = "wasm32")]
async fn recognise_off_thread(
    _native: &'static NativeScan,
    _jpeg: Vec<u8>,
) -> Result<ScanFields, ScanError> {
    Err(ScanError::Unsupported)
}

#[cfg(test)]
mod tests;
