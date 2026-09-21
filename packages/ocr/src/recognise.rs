//! SVTR text recognition with greedy CTC decoding.
//!
//! Contract from `rec.inference.yml`: `RecResizeImg.image_shape: [3, 48, 320]`, `img_mode: BGR`,
//! `PostProcess.name: CTCLabelDecode`.

use image::RgbImage;
use rten::Model;
use rten_imageproc::RotatedRect;
use rten_tensor::prelude::*;
use rten_tensor::{NdTensor, NdTensorView};

use crate::engine::decode_index;
use crate::prep;

/// Fixed by the model: the recogniser's input height is not dynamic.
pub const REC_H: u32 = 48;
/// Crops wider than this are squashed to fit rather than truncated. Losing the right-hand end of a
/// line loses the amount, which is the one thing that matters here.
///
/// `rec.inference.yml` names 320 as the nominal width, but the width axis is dynamic and padding
/// every crop to a fixed 320 measured four times slower than padding to what the crop actually
/// needs — most receipt lines warp to ~105-140px.
const MAX_W: usize = 1024;
/// Recognition runs one crop at a time.
///
/// Counter-intuitive, and measured rather than assumed: batching eight crops made recognition
/// **4.6x slower** (21.9s against 4.7s for the same 26 crops), because a batch must be padded to
/// its widest member and this model's 18710-class output projection is then paid for every padded
/// timestep. One crop per run pays for exactly the width it has, and needs no sorting.
const BATCH: usize = 1;

/// Symmetric, so channel order does not affect the arithmetic — but the model was trained on BGR
/// features, so `to_nchw_bgr` still matters.
const MEAN: [f32; 3] = [0.5, 0.5, 0.5];
const STD: [f32; 3] = [0.5, 0.5, 0.5];

#[derive(Debug, Clone, PartialEq)]
pub struct Recognised {
    pub text: String,
    /// Mean probability of the winning class at each emitting timestep.
    pub confidence: f32,
}

pub fn recognise(
    model: &Model,
    charset: &[char],
    src: &RgbImage,
    boxes: &[RotatedRect],
) -> Vec<Recognised> {
    let mut out = Vec::with_capacity(boxes.len());
    for chunk in boxes.chunks(BATCH) {
        match run_batch(model, charset, src, chunk) {
            Some(mut batch) => out.append(&mut batch),
            // A failed batch loses those lines rather than the whole scan: a receipt missing one
            // line still yields a total.
            None => out.extend(chunk.iter().map(|_| Recognised {
                text: String::new(),
                confidence: 0.0,
            })),
        }
    }
    out
}

fn run_batch(
    model: &Model,
    charset: &[char],
    src: &RgbImage,
    boxes: &[RotatedRect],
) -> Option<Vec<Recognised>> {
    let crops: Vec<RgbImage> = boxes
        .iter()
        .map(|rect| {
            let crop = prep::warp_rect(src, rect, REC_H);
            if crop.width() as usize > MAX_W {
                image::imageops::resize(
                    &crop,
                    MAX_W as u32,
                    REC_H,
                    image::imageops::FilterType::Triangle,
                )
            } else {
                crop
            }
        })
        .collect();

    // The width axis is dynamic, so the batch is padded to its widest member and no further. The
    // stride-of-8 rounding keeps the CTC timestep count integral.
    let width = crops.iter().map(|c| c.width() as usize).max()?.next_multiple_of(8).max(8);

    // Zero after normalisation is mid-grey, which is exactly what PaddleOCR's `resize_norm_img`
    // pads with — so an unfilled tail is neutral, not black.
    let mut input = NdTensor::<f32, 4>::zeros([crops.len(), 3, REC_H as usize, width]);
    for (n, crop) in crops.iter().enumerate() {
        let strip = prep::to_nchw_bgr(crop, MEAN, STD);
        for c in 0..3 {
            for y in 0..REC_H as usize {
                for x in 0..crop.width() as usize {
                    input[[n, c, y, x]] = strip[[0, c, y, x]];
                }
            }
        }
    }

    let output = model.run_one(input.view().into(), None).ok()?;
    let logits: NdTensor<f32, 3> = output.try_into().ok()?;
    Some((0..boxes.len()).map(|n| ctc_greedy(logits.view(), n, charset)).collect())
}

/// Greedy CTC: argmax per timestep, collapse runs of the same class, drop the blank.
///
/// The model already emits probabilities — the spike measured each timestep summing to 1 — so the
/// winning value is used as a confidence directly, with no softmax here.
pub fn ctc_greedy(logits: NdTensorView<f32, 3>, n: usize, charset: &[char]) -> Recognised {
    let [_, steps, classes] = logits.shape();
    let mut text = String::new();
    let mut total = 0.0;
    let mut emitted = 0u32;
    let mut previous = usize::MAX;

    for t in 0..steps {
        let mut best = 0usize;
        let mut best_value = f32::MIN;
        for c in 0..classes {
            let value = logits[[n, t, c]];
            if value > best_value {
                best_value = value;
                best = c;
            }
        }
        if best != previous {
            if let Some(ch) = decode_index(charset, best) {
                text.push(ch);
                total += best_value;
                emitted += 1;
            }
        }
        previous = best;
    }

    let confidence = if emitted == 0 { 0.0 } else { total / emitted as f32 };
    Recognised { text: text.trim().to_string(), confidence }
}

#[cfg(test)]
mod tests;
