//! DBNet text detection.
//!
//! Every constant below is lifted from the model's own `det.inference.yml` rather than chosen:
//! `thresh: 0.2`, `box_thresh: 0.45`, `unclip_ratio: 1.4`. Guessing them costs recall in ways that
//! look like a bad model.

use image::RgbImage;
use rten::Model;
use rten_imageproc::{
    find_contours, min_area_rect, BoundingRect, PointF, RetrievalMode, RotatedRect,
};
use rten_tensor::prelude::*;
use rten_tensor::{NdTensor, NdTensorView};

use crate::{prep, OcrError};

/// ImageNet, from `PreProcess.NormalizeImage` in `det.inference.yml`.
const MEAN: [f32; 3] = [0.485, 0.456, 0.406];
const STD: [f32; 3] = [0.229, 0.224, 0.225];

pub struct DetConfig {
    /// Floor on the **short** side, PaddleOCR's `limit_side_len` with `limit_type='min'`. 736 is
    /// the reference default for this model, and it means "scale a small photo up to it, pass a
    /// larger one through untouched" — not "shrink to it". Lower it first if a device is slow;
    /// that is now a deliberate, documented deviation rather than the accidental one it replaced.
    pub limit_side_len: u32,
    /// Ours, not the reference's: an upper bound so "never downscale" stays bounded for a caller
    /// handing `scan` an arbitrarily large photo. The capture path already caps at 1600, so in the
    /// app this never binds.
    pub max_long_edge: u32,
    pub bin_thresh: f32,
    pub box_thresh: f32,
    pub unclip_ratio: f32,
    /// Boxes thinner than this, on the model's own grid, are noise.
    pub min_side: f32,
    /// `max_candidates` in the model config. A blank or badly-lit photo can otherwise produce
    /// thousands of specks, each costing a recognition pass.
    pub max_boxes: usize,
}

impl Default for DetConfig {
    fn default() -> Self {
        DetConfig {
            limit_side_len: 736,
            max_long_edge: 2048,
            bin_thresh: 0.2,
            box_thresh: 0.45,
            unclip_ratio: 1.4,
            min_side: 3.0,
            max_boxes: 3000,
        }
    }
}

/// Text boxes in **source image** coordinates.
pub fn detect(
    model: &Model,
    src: &RgbImage,
    cfg: &DetConfig,
) -> Result<Vec<RotatedRect>, OcrError> {
    let boxed = prep::letterbox(src, cfg.limit_side_len, cfg.max_long_edge);
    let input = prep::to_nchw_bgr(&boxed.image, MEAN, STD);
    let output = model.run_one(input.view().into(), None).map_err(|_| OcrError::Model)?;
    let prob: NdTensor<f32, 4> = output.try_into().map_err(|_| OcrError::Model)?;

    let [_, _, h, w] = prob.shape();
    let map = prob.reshaped([h, w]);
    let boxes = postprocess(map.view(), cfg);

    // The letterbox pads only right and bottom, so undoing it is a plain divide.
    let inv = 1.0 / boxed.scale;
    Ok(boxes.into_iter().map(|r| scaled(&r, inv)).collect())
}

fn scaled(rect: &RotatedRect, factor: f32) -> RotatedRect {
    let c = rect.center();
    RotatedRect::new(
        PointF::from_yx(c.y * factor, c.x * factor),
        rect.up_axis(),
        rect.width() * factor,
        rect.height() * factor,
    )
}

fn postprocess(prob: NdTensorView<f32, 2>, cfg: &DetConfig) -> Vec<RotatedRect> {
    let [h, w] = prob.shape();
    let mask: Vec<bool> = prob.iter().map(|v| *v > cfg.bin_thresh).collect();
    let mask = NdTensor::from_data([h, w], mask);

    find_contours(mask.view(), RetrievalMode::External)
        .iter()
        .take(cfg.max_boxes)
        .filter_map(|poly| {
            let points: Vec<PointF> =
                poly.iter().map(|p| PointF::from_yx(p.y as f32, p.x as f32)).collect();
            let rect = min_area_rect(&points)?;
            if rect.width().min(rect.height()) < cfg.min_side {
                return None;
            }
            if box_score(prob, &rect) < cfg.box_thresh {
                return None;
            }
            Some(unclip(&rect, cfg.unclip_ratio))
        })
        .collect()
}

/// PaddleOCR's `box_score_fast`: the mean probability **inside the rotated box**, which the
/// reference gets from a `fillPoly` mask over the box's bounds.
///
/// Averaging over the axis-aligned bounds instead — the previous version — is the same number for
/// an upright line and a very different one for a tilted line: the corner triangles outside the
/// polygon are empty, and their area is `width × sin θ`, so the widest lines on the page lose the
/// most. A receipt's widest lines are its header. On the Le Refuge fixture "SAS LE REFUGE" sits at
/// 5 degrees and scored 0.43 over its bounds against 0.87 inside the polygon — dropped at 0.45,
/// and the title fell through to "TABLE 62". It is also the mechanism `tests/framing.rs` measured
/// and could not name: "3 degrees loses the top third of the receipt".
fn box_score(prob: NdTensorView<f32, 2>, rect: &RotatedRect) -> f32 {
    let [h, w] = prob.shape();
    let bounds = rect.bounding_rect();
    let x0 = (bounds.left().floor() as i64).clamp(0, w as i64 - 1) as usize;
    let x1 = (bounds.right().ceil() as i64).clamp(0, w as i64 - 1) as usize;
    let y0 = (bounds.top().floor() as i64).clamp(0, h as i64 - 1) as usize;
    let y1 = (bounds.bottom().ceil() as i64).clamp(0, h as i64 - 1) as usize;

    let mut sum = 0.0;
    let mut count = 0u32;
    for y in y0..=y1 {
        for x in x0..=x1 {
            if rect.contains(PointF::from_yx(y as f32 + 0.5, x as f32 + 0.5)) {
                sum += prob[[y, x]];
                count += 1;
            }
        }
    }
    if count == 0 {
        return 0.0;
    }
    sum / count as f32
}

/// DBNet shrinks its training targets, so every predicted box has to be grown back. `d = area *
/// ratio / perimeter` is the Vatti offset; for a rectangle, moving all four sides out by `d` is
/// exactly what the polygon clipper would produce, so growing the rect directly is not an
/// approximation — and no receipt box is non-convex.
fn unclip(rect: &RotatedRect, ratio: f32) -> RotatedRect {
    let (w, h) = (rect.width(), rect.height());
    let perimeter = 2.0 * (w + h);
    if perimeter <= 0.0 {
        return *rect;
    }
    let d = w * h * ratio / perimeter;
    rect.expanded(2.0 * d, 2.0 * d)
}

fn short_side(rect: &RotatedRect) -> f32 {
    rect.width().min(rect.height())
}

/// Boxes grouped into reading lines.
///
/// A receipt prints "TOTAL" and "12,90" as two separate detections on one visual line. The parser
/// anchors an amount to a keyword *on the same line*, so without this the anchor never sees its
/// number and every total falls back to the low-confidence path.
/// Grouping is by raw centre-y, which assumes an upright page.
///
/// That assumption holds to 10 degrees of tilt on the synthetic fixture (`tests/framing.rs`).
/// When the title used to vanish at 3 degrees, grouping was suspected and cleared: deriving a skew
/// angle from the boxes' own `up_axis` and grouping in a rotated frame gave byte-identical output.
/// The fault was `box_score` scoring the axis-aligned bounds of a tilted box.
pub fn group_lines(rects: &[RotatedRect]) -> Vec<Vec<RotatedRect>> {
    if rects.is_empty() {
        return Vec::new();
    }
    let mut heights: Vec<f32> = rects.iter().map(short_side).collect();
    heights.sort_by(|a, b| a.total_cmp(b));
    let tolerance = 0.6 * heights[heights.len() / 2].max(f32::EPSILON);

    let mut sorted = rects.to_vec();
    sorted.sort_by(|a, b| a.center().y.total_cmp(&b.center().y));

    let mut groups: Vec<Vec<RotatedRect>> = Vec::new();
    for rect in sorted {
        let y = rect.center().y;
        match groups.last_mut() {
            Some(group) if (y - mean_y(group)).abs() <= tolerance => group.push(rect),
            _ => groups.push(vec![rect]),
        }
    }
    for group in &mut groups {
        group.sort_by(|a, b| a.center().x.total_cmp(&b.center().x));
    }
    groups
}

fn mean_y(group: &[RotatedRect]) -> f32 {
    group.iter().map(|r| r.center().y).sum::<f32>() / group.len() as f32
}

/// The group's bounds in the source image, as fractions — what `parse::TextLine` wants.
pub fn group_bounds(group: &[RotatedRect], img_w: f32, img_h: f32) -> (f32, f32, f32) {
    let left = group.iter().map(|r| r.bounding_rect().left()).fold(f32::MAX, f32::min);
    let top = group.iter().map(|r| r.bounding_rect().top()).fold(f32::MAX, f32::min);
    let bottom = group.iter().map(|r| r.bounding_rect().bottom()).fold(f32::MIN, f32::max);
    (left / img_w, (top + bottom) / 2.0 / img_h, (bottom - top) / img_h)
}

#[cfg(test)]
mod tests;
