use super::*;

fn msg(kind: &str, value: &str) -> CaptureMsg {
    CaptureMsg { kind: kind.into(), value: value.into() }
}

fn b64(bytes: &[u8]) -> String {
    use base64::Engine;
    base64::engine::general_purpose::STANDARD.encode(bytes)
}

// ─── the chunk protocol ──────────────────────────────────────────────────────

#[test]
fn chunks_reassemble_in_order() {
    let jpeg = vec![0xFF, 0xD8, 0xFF, 0xE0, 1, 2, 3, 4, 5, 6];
    let encoded = b64(&jpeg);
    let (head, tail) = encoded.split_at(4);

    let mut a = Assembler::default();
    assert!(a.push(&msg("chunk", head)).is_none(), "a chunk never terminates the stream");
    assert!(a.push(&msg("chunk", tail)).is_none());
    assert_eq!(a.push(&msg("end", "ok")), Some(Ok(jpeg)));
}

#[test]
fn a_single_chunk_is_enough() {
    let mut a = Assembler::default();
    a.push(&msg("chunk", &b64(b"hello")));
    assert_eq!(a.push(&msg("end", "ok")), Some(Ok(b"hello".to_vec())));
}

/// Backing out of the camera is not a failure and must never reach a toast.
#[test]
fn a_cancel_end_is_not_an_error_to_show() {
    let mut a = Assembler::default();
    assert_eq!(a.push(&msg("end", "cancel")), Some(Err(ScanError::Cancelled)));
}

#[test]
fn an_end_without_chunks_is_a_capture_error() {
    let mut a = Assembler::default();
    assert_eq!(a.push(&msg("end", "ok")), Some(Err(ScanError::Capture)));
}

#[test]
fn invalid_base64_is_a_capture_error() {
    let mut a = Assembler::default();
    a.push(&msg("chunk", "not base64 at all !!!"));
    assert_eq!(a.push(&msg("end", "ok")), Some(Err(ScanError::Capture)));
}

#[test]
fn an_oversized_stream_is_rejected() {
    let mut a = Assembler::default();
    let big = "A".repeat(CHUNK_BYTES);
    let mut rejected = None;
    for _ in 0..40 {
        if let Some(done) = a.push(&msg("chunk", &big)) {
            rejected = Some(done);
            break;
        }
    }
    assert_eq!(rejected, Some(Err(ScanError::Capture)), "the ceiling must stop the stream");
}

#[test]
fn an_unknown_message_kind_ends_the_stream() {
    let mut a = Assembler::default();
    assert_eq!(a.push(&msg("nonsense", "")), Some(Err(ScanError::Capture)));
}

/// The JS sends `{kind, value}` and nothing else; if that shape drifts, the stream silently stops.
#[test]
fn the_wire_shape_deserialises() {
    let parsed: CaptureMsg = serde_json::from_str(r#"{"kind":"chunk","value":"AAAA"}"#).unwrap();
    assert_eq!(parsed.kind, "chunk");
    assert_eq!(parsed.value, "AAAA");
}

// ─── the capture script ──────────────────────────────────────────────────────

/// The dodge this whole module exists for: dioxus intercepts `input[type=file]` clicks for any
/// element inside its root and routes them to `rfd`, which does nothing on a phone.
#[test]
fn the_input_is_appended_outside_the_dioxus_root() {
    let js = capture_js(ScanSource::Camera);
    assert!(js.contains("document.body.appendChild(input)"), "{js}");
    assert!(!js.contains("getElementById"), "must not attach inside the app's own tree");
}

#[test]
fn the_camera_source_asks_for_the_camera() {
    let js = capture_js(ScanSource::Camera);
    assert!(js.contains("input.accept = 'image/*'"), "{js}");
    assert!(js.contains("input.capture = 'environment'"), "{js}");
}

/// `capture` is decisive on Android: with it wry launches the camera intent and nothing else, so
/// the library source must leave it off to reach the document picker at all.
#[test]
fn the_library_source_leaves_capture_off() {
    let js = capture_js(ScanSource::Library);
    assert!(js.contains("input.accept = 'image/*'"), "{js}");
    assert!(!js.contains("input.capture"), "{js}");
}

/// Dioxus runs the script as an async function *body*; without a top-level await the caller
/// resumes immediately and the photo never arrives.
#[test]
fn the_script_awaits_at_the_top_level() {
    let js = capture_js(ScanSource::Camera);
    assert!(js.contains("return await new Promise"), "{js}");
}

#[test]
fn the_script_carries_the_transport_constants() {
    let js = capture_js(ScanSource::Camera);
    assert!(js.contains(&CHUNK_BYTES.to_string()), "chunk size missing: {js}");
    assert!(js.contains(&MAX_EDGE.to_string()), "downscale edge missing: {js}");
}

/// Three cancel paths, because the Android System WebView on older devices fires none of the first
/// two and the UI would sit on a spinner forever.
#[test]
fn cancellation_has_a_fallback_signal() {
    let js = capture_js(ScanSource::Camera);
    assert!(js.contains("'cancel'"), "{js}");
    assert!(js.contains("addEventListener('cancel'"), "{js}");
    assert!(js.contains("window.addEventListener('focus'"), "{js}");
}

#[test]
fn the_photo_is_downscaled_before_transport() {
    let js = capture_js(ScanSource::Camera);
    assert!(js.contains("toDataURL('image/jpeg'"), "{js}");
    assert!(js.contains("drawImage"), "{js}");
}

// ─── installation ────────────────────────────────────────────────────────────

/// Web, desktop and the server binary install nothing, and every call site must stay safe. The
/// mobile entry point is the only caller of `set_native_scan`, so this holds in the test binary.
#[test]
fn no_scanner_is_installed_by_default() {
    assert!(!scanner_available());
}

#[test]
fn every_shown_error_has_a_key() {
    for error in [ScanError::Capture, ScanError::Unreadable] {
        assert!(!scan_error_key(error).is_empty());
    }
}
