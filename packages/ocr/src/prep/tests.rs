use image::Rgb;
use rten_imageproc::{PointF, RotatedRect, Vec2};
use rten_tensor::prelude::*;

use super::*;

fn solid(w: u32, h: u32, colour: [u8; 3]) -> RgbImage {
    RgbImage::from_pixel(w, h, Rgb(colour))
}

/// The regression this function was rewritten for. `limit_side_len` is a floor on the **short**
/// side, so a photo already above it is passed through at native size — PaddleOCR's `'min'` mode
/// sets `ratio = 1.` in that branch. Read as a ceiling on the long edge, this same 1200x1600 input
/// used to be detected at 720x960: 36% of the pixels, at the one stage that decides recall.
#[test]
fn letterbox_does_not_resize_a_photo_above_the_floor() {
    let src = solid(1200, 1600, [255, 255, 255]);
    let out = letterbox(&src, 736, 2048);
    assert_eq!(out.scale, 1.0, "min(1200,1600) is above 736, so nothing should be resized");
    // Only padded up to the stride: 1200 -> 1216, and 1600 is already a multiple of 32.
    assert_eq!((out.image.width(), out.image.height()), (1216, 1600));
    // The original content stays at the origin, so mapping a box back is a plain divide.
    assert_eq!(out.image.get_pixel(0, 0).0, [255, 255, 255]);
    assert_eq!(out.image.get_pixel(1215, 0).0, [0, 0, 0], "padding is black, and on the right");
}

#[test]
fn letterbox_scales_a_small_photo_up_to_the_floor() {
    let out = letterbox(&solid(100, 50, [255, 255, 255]), 736, 2048);
    assert!((out.scale - 14.72).abs() < 1e-4, "scale was {}", out.scale);
    assert_eq!((out.image.width(), out.image.height()), (1472, 736), "short side lands on 736");
}

/// `max_long_edge` has no counterpart in the reference: it is the bound that keeps "never
/// downscale" from meaning "decode a 50 MP photo at full size" for a public entry point.
#[test]
fn letterbox_caps_an_enormous_photo_on_the_long_edge() {
    let out = letterbox(&solid(5000, 1000, [1, 2, 3]), 736, 2048);
    assert!((out.scale - 0.4096).abs() < 1e-4, "scale was {}", out.scale);
    assert_eq!(out.image.width(), 2048, "the long edge is clamped");
}

#[test]
fn letterbox_output_is_always_a_multiple_of_the_stride() {
    for (w, h) in [(1, 1), (33, 65), (1999, 1001), (960, 960), (4000, 3000)] {
        let out = letterbox(&solid(w, h, [1, 2, 3]), 736, 2048);
        assert_eq!(out.image.width() % STRIDE, 0, "{w}x{h}");
        assert_eq!(out.image.height() % STRIDE, 0, "{w}x{h}");
    }
}

/// The whole point of this function: channel 0 must be blue.
#[test]
fn to_nchw_is_bgr_not_rgb() {
    let src = solid(1, 1, [255, 0, 0]); // pure red
    let t = to_nchw_bgr(&src, [0.0, 0.0, 0.0], [1.0, 1.0, 1.0]);
    assert_eq!(t.shape(), [1, 3, 1, 1]);
    assert_eq!(t[[0, 0, 0, 0]], 0.0, "channel 0 is blue");
    assert_eq!(t[[0, 1, 0, 0]], 0.0, "channel 1 is green");
    assert_eq!(t[[0, 2, 0, 0]], 1.0, "channel 2 is red");
}

#[test]
fn to_nchw_applies_mean_and_std() {
    let src = solid(1, 1, [255, 255, 255]);
    let t = to_nchw_bgr(&src, [0.5, 0.5, 0.5], [0.5, 0.5, 0.5]);
    assert!((t[[0, 0, 0, 0]] - 1.0).abs() < 1e-6);
}

fn upright(cx: f32, cy: f32, w: f32, h: f32) -> RotatedRect {
    RotatedRect::new(PointF::from_yx(cy, cx), Vec2::from_xy(0.0, 1.0), w, h)
}

#[test]
fn warp_of_an_axis_aligned_rect_is_a_plain_crop() {
    let mut src = solid(40, 20, [0, 0, 0]);
    for x in 10..20 {
        for y in 5..15 {
            src.put_pixel(x, y, Rgb([255, 255, 255]));
        }
    }
    // The white square, exactly.
    let out = warp_rect(&src, &upright(15.0, 10.0, 10.0, 10.0), 10);
    assert_eq!((out.width(), out.height()), (10, 10));
    assert_eq!(out.get_pixel(5, 5).0, [255, 255, 255]);
}

#[test]
fn warp_output_height_is_exact_and_width_follows_the_aspect() {
    let src = solid(200, 100, [128, 128, 128]);
    let out = warp_rect(&src, &upright(100.0, 50.0, 80.0, 20.0), 48);
    assert_eq!(out.height(), 48);
    assert_eq!(out.width(), 192, "80/20 aspect at 48px tall");
}

/// A box whose long side runs vertically still comes out as a horizontal strip.
#[test]
fn warp_uses_the_long_side_as_the_reading_direction() {
    let src = solid(100, 200, [7, 7, 7]);
    let out = warp_rect(&src, &upright(50.0, 100.0, 20.0, 80.0), 48);
    assert_eq!(out.height(), 48);
    assert_eq!(out.width(), 192);
}

fn fixture_jpeg() -> Vec<u8> {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/synthetic-carrefour.jpg");
    std::fs::read(path).unwrap()
}

/// The same fixture with its SOF0 frame header rewritten to declare another size, so every other
/// segment stays valid and the decoder gets as far as the dimension check.
fn fixture_declaring(w: u16, h: u16) -> Vec<u8> {
    let mut jpeg = fixture_jpeg();
    let sof = jpeg.windows(2).position(|m| m == [0xFF, 0xC0]).expect("baseline SOF0 marker");
    jpeg[sof + 5..sof + 7].copy_from_slice(&h.to_be_bytes());
    jpeg[sof + 7..sof + 9].copy_from_slice(&w.to_be_bytes());
    jpeg
}

/// The header alone can ask for 65535x65535, which `to_rgb8` would answer with a ~12.9 GB
/// allocation. The reader must refuse it before a single pixel is allocated: the error is the
/// limit's, not a decode failure further down.
#[test]
fn decode_refuses_a_header_that_declares_a_huge_image() {
    assert_eq!(decode(&fixture_declaring(65535, 65535)), Err(OcrError::Decode));
    assert_eq!(decode(&fixture_declaring(MAX_DECODE_SIDE as u16 + 1, 100)), Err(OcrError::Decode));

    let mut reader = image::ImageReader::new(std::io::Cursor::new(fixture_declaring(65535, 65535)));
    reader.set_format(image::ImageFormat::Jpeg);
    let mut limits = image::Limits::default();
    limits.max_image_width = Some(MAX_DECODE_SIDE);
    limits.max_image_height = Some(MAX_DECODE_SIDE);
    reader.limits(limits);
    let err = reader.decode().expect_err("refused").to_string();
    assert!(err.contains("limit"), "expected a limits error, got: {err}");
}

#[test]
fn decode_still_reads_a_real_jpeg() {
    let img = decode(&fixture_jpeg()).unwrap();
    assert!(img.width() > 0 && img.height() > 0);
}
