//! Which transformation loses the title?
//!
//! The title comes back empty on roughly half of real receipts while the amount and the date are
//! reliable. Three mechanisms could do that and they have nothing in common as fixes: the detector
//! never found the header, the recogniser turned it into something `is_title_candidate` rejects, or
//! the parser found it and dropped it on position. Guessing between them is how this feature has
//! already been wrong several times (see §11 of docs/plans/ocr-receipt-scan.md).
//!
//! So this deliberately breaks a fixture that currently works, one axis at a time, and prints every
//! `TextLine` the parser was handed. Whichever transform first turns the title to `None` names the
//! mechanism. It is `#[ignore]`d because it is an experiment, not a regression gate — and because
//! it runs the full pipeline a dozen times.
//!
//! `cargo test -p ocr --test framing -- --ignored --nocapture`
//!
//! # What it found
//!
//! **Framing was the whole bug, and it was in the parser.** At 30% background above the receipt the
//! header was still detected and recognised perfectly — "CARREFOUR MARKET" at confidence 1.00 — and
//! then discarded for landing at y=0.280 against a 0.25 cutoff measured against the *image* rather
//! than the receipt. 20% padding passed, so the fault was invisible on any tightly-shot fixture.
//! Fixed by measuring the zone against the detected text span; `background_above_the_receipt_does_
//! not_lose_the_title` is the gate, and 80% padding now passes.
//!
//! **Resolution and compression were not involved.** The title survives a 50% downscale and JPEG
//! quality 50. Worth knowing, because "the photo isn't sharp enough" is the intuitive explanation
//! and it is wrong — recognition crops are cut from the full-size source, so only detection recall
//! is ever at stake.
//!
//! **Rotation had a sharp threshold — 2 degrees perfect, 3 degrees lost the top third of the
//! receipt — and it was the box scorer.** At 3 degrees the header was not among the detections,
//! while "PAIN COMPLET 2,40" a few lines below still grouped correctly, so the fault was inside
//! detection; two attempts elsewhere failed and were reverted (grouping in a frame deskewed from the
//! boxes' own `up_axis`: byte-identical output; Otsu-based rectification: erratic). The Le Refuge
//! fixture named it: `box_score` averaged the probability map over the box's *axis-aligned bounds*
//! where PaddleOCR averages *inside the polygon*, and the empty corner triangles of a tilted box
//! grow with `width × sin θ` — so the widest lines, the header, were the first to drop under
//! `box_thresh`. Scored inside the polygon, 10 degrees keeps the title;
//! `a_tilted_receipt_does_not_lose_the_title` is the gate.

#![cfg(feature = "models")]

use image::codecs::jpeg::JpegEncoder;
use image::{ImageEncoder, Rgb, RgbImage};
use std::path::PathBuf;

const FIXTURE: &str = "synthetic-carrefour";
/// Well above the capture path's own quality, so compression is not a hidden variable in the other rows.
const QUALITY: u8 = 92;
/// Surface the receipt sits on. Paper is ~255, so this sets the paper-to-background contrast, which
/// matters for any outline-finding approach — an Otsu-based one was tried and failed on it.
const BACKGROUND: u8 = 140;

fn fixture() -> RgbImage {
    let path =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(format!("tests/fixtures/{FIXTURE}.jpg"));
    let bytes = std::fs::read(&path).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
    image::load_from_memory(&bytes).expect("fixture is not a decodable image").to_rgb8()
}

fn to_jpeg(img: &RgbImage, quality: u8) -> Vec<u8> {
    let mut buf = Vec::new();
    JpegEncoder::new_with_quality(&mut buf, quality)
        .write_image(img.as_raw(), img.width(), img.height(), image::ExtendedColorType::Rgb8)
        .expect("encode");
    buf
}

/// Background above the receipt — what every hand-held photo has and the fixture does not.
/// `TextLine.y` is a fraction of *image* height, so this is the one transform that directly probes
/// whether `TITLE_ZONE_Y` is measured against the wrong thing.
fn pad_top(img: &RgbImage, fraction: f32) -> RgbImage {
    let pad = (img.height() as f32 * fraction).round() as u32;
    let mut out = RgbImage::from_pixel(img.width(), img.height() + pad, Rgb([BACKGROUND; 3]));
    for (x, y, px) in img.enumerate_pixels() {
        out.put_pixel(x, y + pad, *px);
    }
    out
}

/// Bilinear rotation about the centre, canvas grown to fit. Intended to probe the y-axis that
/// `group_lines` and the title zone depend on, since `warp_rect` already de-rotates each crop. The
/// measurement said otherwise: what broke first was detection's box score, not either of those.
fn rotate(img: &RgbImage, degrees: f32) -> RgbImage {
    let (w, h) = (img.width() as f32, img.height() as f32);
    let (sin, cos) = degrees.to_radians().sin_cos();
    let out_w = (w * cos.abs() + h * sin.abs()).ceil().max(1.0);
    let out_h = (w * sin.abs() + h * cos.abs()).ceil().max(1.0);
    let mut out = RgbImage::from_pixel(out_w as u32, out_h as u32, Rgb([BACKGROUND; 3]));

    for oy in 0..out.height() {
        for ox in 0..out.width() {
            // Inverse-map the destination pixel back into the source.
            let dx = ox as f32 + 0.5 - out_w / 2.0;
            let dy = oy as f32 + 0.5 - out_h / 2.0;
            let sx = dx * cos + dy * sin + w / 2.0 - 0.5;
            let sy = -dx * sin + dy * cos + h / 2.0 - 0.5;
            if sx < 0.0 || sy < 0.0 || sx >= w - 1.0 || sy >= h - 1.0 {
                continue;
            }
            let (x0, y0) = (sx.floor() as u32, sy.floor() as u32);
            let (fx, fy) = (sx - x0 as f32, sy - y0 as f32);
            let mut px = [0.0f32; 3];
            for (i, p) in px.iter_mut().enumerate() {
                let p00 = img.get_pixel(x0, y0)[i] as f32;
                let p10 = img.get_pixel(x0 + 1, y0)[i] as f32;
                let p01 = img.get_pixel(x0, y0 + 1)[i] as f32;
                let p11 = img.get_pixel(x0 + 1, y0 + 1)[i] as f32;
                *p = p00 * (1.0 - fx) * (1.0 - fy)
                    + p10 * fx * (1.0 - fy)
                    + p01 * (1.0 - fx) * fy
                    + p11 * fx * fy;
            }
            out.put_pixel(ox, oy, Rgb([px[0] as u8, px[1] as u8, px[2] as u8]));
        }
    }
    out
}

fn scaled(img: &RgbImage, factor: f32) -> RgbImage {
    let w = ((img.width() as f32 * factor).round() as u32).max(1);
    let h = ((img.height() as f32 * factor).round() as u32).max(1);
    image::imageops::resize(img, w, h, image::imageops::FilterType::Triangle)
}

/// The regression gate the experiment below produced. Before the title zone was measured against
/// the detected text rather than the image, this failed at 30% and every larger padding: the header
/// was detected and recognised at confidence 1.00, then dropped for landing at y=0.280 against a
/// 0.25 cutoff that described the photo rather than the receipt. 20% passed, so the old behaviour
/// looked fine on any fixture shot tight.
#[test]
fn background_above_the_receipt_does_not_lose_the_title() {
    let base = fixture();
    for fraction in [0.3_f32, 0.5, 0.8] {
        let jpeg = to_jpeg(&pad_top(&base, fraction), QUALITY);
        let title = ocr::scan(&jpeg).ok().and_then(|r| r.title);
        assert_eq!(
            title.as_deref(),
            Some("Carrefour Market"),
            "{:.0}% background above the receipt lost the title",
            fraction * 100.0
        );
    }
}

/// Before `box_score` measured inside the polygon this failed from 3 degrees up: the header's
/// axis-aligned bounds were mostly empty and it scored under `box_thresh`.
#[test]
fn a_tilted_receipt_does_not_lose_the_title() {
    let base = fixture();
    for degrees in [3.0_f32, 5.0, 10.0] {
        let jpeg = to_jpeg(&rotate(&base, degrees), QUALITY);
        let title = ocr::scan(&jpeg).ok().and_then(|r| r.title);
        assert_eq!(title.as_deref(), Some("Carrefour Market"), "{degrees} degrees lost the title");
    }
}

#[test]
#[ignore = "experiment, not a gate — runs the whole pipeline a dozen times"]
fn which_transform_loses_the_title() {
    let base = fixture();
    println!("fixture {FIXTURE}: {}x{}", base.width(), base.height());

    let mut cases: Vec<(String, RgbImage)> = vec![("baseline".to_string(), base.clone())];
    for f in [0.2_f32, 0.3, 0.5, 0.8] {
        cases.push((format!("pad_top {:.0}%", f * 100.0), pad_top(&base, f)));
    }
    for d in [2.0_f32, 3.0, 5.0, 10.0] {
        cases.push((format!("rotate {d}deg"), rotate(&base, d)));
    }
    for s in [0.75_f32, 0.5] {
        cases.push((format!("scale {:.0}%", s * 100.0), scaled(&base, s)));
    }
    cases.push(("jpeg q50".to_string(), base.clone()));

    for (name, img) in &cases {
        let quality = if name == "jpeg q50" { 50 } else { QUALITY };
        let jpeg = to_jpeg(img, quality);
        let lines = ocr::text_lines(&jpeg).unwrap_or_default();
        let title = ocr::scan(&jpeg).ok().and_then(|r| r.title);

        println!(
            "\n=== {name} ({}x{}) -> title {:?}",
            img.width(),
            img.height(),
            title.as_deref().unwrap_or("<NONE>")
        );
        // Only the top of the page matters for the title, and printing every line of a receipt
        // buries it. `TITLE_ZONE_Y` is 0.25, so 0.45 shows what is just outside the window too.
        for l in lines.iter() {
            println!("    y={:.3} h={:.3} conf={:.2} {:?}", l.y, l.h, l.confidence, l.text);
        }
    }
}
