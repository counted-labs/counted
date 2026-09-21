//! The two PP-OCRv6 models and the character set they decode against.
//!
//! `load_static_slice` is zero-copy: the weights stay in the mapped binary image and are never
//! heap-allocated. `asset!()` could not do this — on Android a bundled asset lives inside the APK
//! and is reachable only through AssetManager over JNI, never `std::fs`.

use std::sync::OnceLock;

use rten::Model;

use crate::OcrError;

static DET: &[u8] = include_bytes!("../models/det.onnx");
static REC: &[u8] = include_bytes!("../models/rec.onnx");
/// `PostProcess.character_dict` from the model repo's `inference.yml`, one Unicode scalar per
/// line, in order. Extracted once and committed; no entry contains a newline, which is what makes
/// the format safe.
static CHARSET: &str = include_str!("../models/charset.txt");

pub struct Engine {
    pub det: Model,
    pub rec: Model,
    pub charset: Vec<char>,
}

/// CTC class index to character. The recogniser emits `charset.len() + 2` classes — measured, not
/// assumed: `rec.inference.yml` carries no `use_space_char` key, but the model's output width is
/// 18710 against an 18708-entry dictionary, so PaddleOCR's space *is* appended. Getting this wrong
/// costs every space in the output and shifts nothing else, which is the hardest kind of bug to
/// notice.
pub fn decode_index(charset: &[char], class: usize) -> Option<char> {
    match class {
        0 => None,
        c if c <= charset.len() => Some(charset[c - 1]),
        c if c == charset.len() + 1 => Some(' '),
        _ => None,
    }
}

/// Built once, on the first scan.
pub fn shared() -> Result<&'static Engine, OcrError> {
    static ENGINE: OnceLock<Option<Engine>> = OnceLock::new();
    ENGINE.get_or_init(|| Engine::load().ok()).as_ref().ok_or(OcrError::Model)
}

impl Engine {
    fn load() -> Result<Self, OcrError> {
        Ok(Engine {
            det: Model::load_static_slice(DET).map_err(|_| OcrError::Model)?,
            rec: Model::load_static_slice(REC).map_err(|_| OcrError::Model)?,
            charset: parse_charset(CHARSET),
        })
    }
}

fn parse_charset(raw: &str) -> Vec<char> {
    raw.lines().filter_map(|line| line.chars().next()).collect()
}

#[cfg(test)]
mod tests;
