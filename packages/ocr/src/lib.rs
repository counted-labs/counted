//! On-device receipt OCR. Runs entirely in the app process — no image and no recognised text ever
//! reaches the network. See `docs/plans/ocr-receipt-scan.md`.
//!
//! The pipeline modules are behind the `models` feature so `parse` — which holds the logic worth
//! testing — builds and tests in a checkout whose Git LFS blobs were never pulled.

pub mod parse;

#[cfg(feature = "models")]
mod detect;
#[cfg(feature = "models")]
mod engine;
#[cfg(feature = "models")]
mod prep;
#[cfg(feature = "models")]
mod recognise;

#[cfg(all(test, feature = "models"))]
mod bench;

pub use parse::{parse_receipt, Receipt, TextLine};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OcrError {
    /// The bytes are not a JPEG the decoder accepts.
    Decode,
    /// A model failed to load or to run. Only reachable on a corrupt binary.
    Model,
    /// Ran, and found no text at all.
    Empty,
}

/// Lines below this are noise the parser is better off not seeing.
#[cfg(feature = "models")]
const MIN_LINE_CONFIDENCE: f32 = 0.5;

/// Decode, detect, recognise, parse. The whole feature.
///
/// Blocking and CPU-bound for one to three seconds — the caller must keep it off the platform UI
/// thread, which on both phones is also the Dioxus event loop.
#[cfg(feature = "models")]
pub fn scan(jpeg: &[u8]) -> Result<Receipt, OcrError> {
    let lines = text_lines(jpeg)?;
    if lines.is_empty() {
        return Err(OcrError::Empty);
    }
    Ok(parse_receipt(&lines, chrono::Local::now().date_naive()))
}

/// Everything the parser sees, before it sees it.
///
/// Public for diagnostics: when a field comes back empty, the only question that matters is whether
/// the line was never detected, recognised as something else, or found and then rejected by the
/// parser — and those three have nothing in common as fixes. `tests/framing.rs` is the caller.
#[cfg(feature = "models")]
pub fn text_lines(jpeg: &[u8]) -> Result<Vec<TextLine>, OcrError> {
    let engine = engine::shared()?;
    let image = prep::decode(jpeg)?;
    let boxes = detect::detect(&engine.det, &image, &detect::DetConfig::default())?;

    let (w, h) = (image.width() as f32, image.height() as f32);
    let mut lines = Vec::new();
    for group in detect::group_lines(&boxes) {
        let parts = recognise::recognise(&engine.rec, &engine.charset, &image, &group);
        let text = parts
            .iter()
            .map(|p| p.text.as_str())
            .filter(|t| !t.is_empty())
            .collect::<Vec<_>>()
            .join(" ");
        if text.is_empty() {
            continue;
        }
        let confidence = parts.iter().map(|p| p.confidence).sum::<f32>() / parts.len() as f32;
        if confidence < MIN_LINE_CONFIDENCE {
            continue;
        }
        let (x, y, line_h) = detect::group_bounds(&group, w, h);
        lines.push(TextLine { text, x, y, h: line_h, confidence });
    }
    Ok(lines)
}
