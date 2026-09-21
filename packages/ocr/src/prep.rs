//! Image preprocessing shared by detection and recognition.
//!
//! **Channel order is BGR, not RGB.** Both `det.inference.yml` and `rec.inference.yml` declare
//! `DecodeImage: img_mode: BGR`, so the networks were trained on OpenCV's byte order and the
//! normalisation constants line up against it. Feeding RGB costs accuracy silently — nothing
//! errors, the text just comes out worse.

use image::{imageops::FilterType, RgbImage};
use rten_tensor::NdTensor;

use crate::OcrError;

/// DBNet's stride: the detector's input must be a multiple of it in both dimensions.
const STRIDE: u32 = 32;

/// A JPEG header can declare 65535x65535 and `to_rgb8` on that is ~12.9 GB from a file of a few
/// hundred bytes. The capture path already downscales honest photos before they get here
/// (`ui::common::scan`), so these only ever bite a crafted file.
const MAX_DECODE_SIDE: u32 = 8192;
const MAX_DECODE_ALLOC: u64 = 256 * 1024 * 1024;

pub fn decode(jpeg: &[u8]) -> Result<RgbImage, OcrError> {
    let mut reader = image::ImageReader::new(std::io::Cursor::new(jpeg));
    reader.set_format(image::ImageFormat::Jpeg);
    let mut limits = image::Limits::default();
    limits.max_image_width = Some(MAX_DECODE_SIDE);
    limits.max_image_height = Some(MAX_DECODE_SIDE);
    limits.max_alloc = Some(MAX_DECODE_ALLOC);
    reader.limits(limits);
    reader.decode().map(|img| img.to_rgb8()).map_err(|_| OcrError::Decode)
}

pub struct Letterboxed {
    pub image: RgbImage,
    /// Source pixels per letterboxed pixel: divide a box coordinate by this to map it back.
    pub scale: f32,
}

/// Resizes for detection, then pads right and bottom to a multiple of the stride. Padding only on
/// those two sides is what lets the inverse mapping be a plain divide.
///
/// `limit_side_len` is a **floor on the short side**, which is what PaddleOCR means by it. Its
/// `DetResizeForTest` defaults to `limit_side_len=736, limit_type='min'`, and `'min'` reads:
///
/// ```text
/// if min(h, w) < limit_side_len: ratio = limit_side_len / <shorter side>
/// else:                          ratio = 1.
/// ```
///
/// — so a photo larger than the floor is passed through untouched. This function previously read
/// the constant as a *ceiling on the long edge*, which is the opposite axis and the opposite
/// direction: a 1200x1600 photo was detected at 720x960, 36% of the pixels, with every glyph 1.67x
/// smaller. Detection is where recall is decided, and recognition crops come from the full-size
/// source, so that cost fell entirely on lines the detector never found.
///
/// `max_long_edge` has no equivalent in the reference and is ours: `scan` is a public entry point,
/// and "never downscale" needs an upper bound before someone hands it a 50 MP photo.
pub fn letterbox(src: &RgbImage, limit_side_len: u32, max_long_edge: u32) -> Letterboxed {
    let (w, h) = (src.width(), src.height());
    let (short, long) = (w.min(h) as f32, w.max(h) as f32);

    let mut scale = if short < limit_side_len as f32 { limit_side_len as f32 / short } else { 1.0 };
    if long * scale > max_long_edge as f32 {
        scale = max_long_edge as f32 / long;
    }

    let target_w = ((w as f32 * scale).round() as u32).max(1);
    let target_h = ((h as f32 * scale).round() as u32).max(1);

    let resized = if target_w == w && target_h == h {
        src.clone()
    } else {
        image::imageops::resize(src, target_w, target_h, FilterType::Triangle)
    };

    let padded_w = target_w.div_ceil(STRIDE) * STRIDE;
    let padded_h = target_h.div_ceil(STRIDE) * STRIDE;
    if padded_w == target_w && padded_h == target_h {
        return Letterboxed { image: resized, scale };
    }
    let mut padded = RgbImage::new(padded_w, padded_h);
    image::imageops::replace(&mut padded, &resized, 0, 0);
    Letterboxed { image: padded, scale }
}

/// NCHW f32 in BGR order, `(x/255 - mean) / std` per channel.
pub fn to_nchw_bgr(img: &RgbImage, mean: [f32; 3], std: [f32; 3]) -> NdTensor<f32, 4> {
    let (w, h) = (img.width() as usize, img.height() as usize);
    let mut out = NdTensor::<f32, 4>::zeros([1, 3, h, w]);
    for (x, y, px) in img.enumerate_pixels() {
        let bgr = [px[2], px[1], px[0]];
        for (c, value) in bgr.iter().enumerate() {
            out[[0, c, y as usize, x as usize]] = (*value as f32 / 255.0 - mean[c]) / std[c];
        }
    }
    out
}

/// Bilinear sample, clamped at the edges.
fn sample(src: &RgbImage, x: f32, y: f32) -> [f32; 3] {
    let (w, h) = (src.width() as i64, src.height() as i64);
    let x0 = x.floor() as i64;
    let y0 = y.floor() as i64;
    let (fx, fy) = (x - x0 as f32, y - y0 as f32);
    let at = |px: i64, py: i64| {
        let px = px.clamp(0, w - 1) as u32;
        let py = py.clamp(0, h - 1) as u32;
        src.get_pixel(px, py).0
    };
    let (p00, p10, p01, p11) = (at(x0, y0), at(x0 + 1, y0), at(x0, y0 + 1), at(x0 + 1, y0 + 1));
    let mut out = [0.0; 3];
    for c in 0..3 {
        let top = p00[c] as f32 * (1.0 - fx) + p10[c] as f32 * fx;
        let bottom = p01[c] as f32 * (1.0 - fx) + p11[c] as f32 * fx;
        out[c] = top * (1.0 - fy) + bottom * fy;
    }
    out
}

/// The axes of a text box: the long one is the reading direction.
///
/// Derived from the rect's own basis rather than the corner order, because `corners()` makes no
/// promise about which corner is which. Both axes are then flipped to point right and down, which
/// is what stops a near-horizontal line coming out mirrored or upside down. There is no
/// text-direction classifier in this pipeline — receipts are printed upright, and a rotated one is
/// a retake, not a code path.
fn axes(rect: &rten_imageproc::RotatedRect) -> ([f32; 2], f32, [f32; 2], f32) {
    let up = rect.up_axis();
    let right = [up.y, -up.x];
    let up = [up.x, up.y];

    let (mut long_dir, long_len, mut short_dir, short_len) = if rect.width() >= rect.height() {
        (right, rect.width(), up, rect.height())
    } else {
        (up, rect.height(), right, rect.width())
    };
    if long_dir[0] < 0.0 {
        long_dir = [-long_dir[0], -long_dir[1]];
    }
    if short_dir[1] < 0.0 {
        short_dir = [-short_dir[0], -short_dir[1]];
    }
    (long_dir, long_len, short_dir, short_len)
}

/// PaddleOCR's `get_rotate_crop_image`: the rect's content resampled into an upright strip
/// `out_h` tall, so the recogniser never sees a rotated box.
pub fn warp_rect(src: &RgbImage, rect: &rten_imageproc::RotatedRect, out_h: u32) -> RgbImage {
    let (long_dir, long_len, short_dir, short_len) = axes(rect);
    let aspect = if short_len > 0.0 { long_len / short_len } else { 1.0 };
    let out_w = ((out_h as f32 * aspect).round() as u32).clamp(1, 4096);

    let centre = rect.center();
    let mut out = RgbImage::new(out_w, out_h);
    for oy in 0..out_h {
        let v = (oy as f32 + 0.5) / out_h as f32 - 0.5;
        for ox in 0..out_w {
            let u = (ox as f32 + 0.5) / out_w as f32 - 0.5;
            let x = centre.x + u * long_len * long_dir[0] + v * short_len * short_dir[0];
            let y = centre.y + u * long_len * long_dir[1] + v * short_len * short_dir[1];
            let px = sample(src, x, y);
            out.put_pixel(ox, oy, image::Rgb([px[0] as u8, px[1] as u8, px[2] as u8]));
        }
    }
    out
}

#[cfg(test)]
mod tests;
