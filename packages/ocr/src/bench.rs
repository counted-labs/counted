//! Stage timings, `#[ignore]`d so CI never pays for them.
//!
//! `cargo test --release -p ocr -- --ignored --nocapture`. Release matters: a debug build is ~18x
//! slower and tells you nothing about the phone.

use std::time::Instant;

use super::*;

fn fixture() -> Vec<u8> {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/synthetic-carrefour.jpg");
    std::fs::read(path).unwrap()
}

#[test]
#[ignore = "timing, not correctness"]
fn stage_timings() {
    let jpeg = fixture();
    let engine = engine::shared().unwrap();

    let t = Instant::now();
    let image = prep::decode(&jpeg).unwrap();
    println!("decode        {:>10?}  {}x{}", t.elapsed(), image.width(), image.height());

    for limit_side_len in [736u32, 640, 960] {
        let cfg = detect::DetConfig { limit_side_len, ..detect::DetConfig::default() };

        let t = Instant::now();
        let boxes = detect::detect(&engine.det, &image, &cfg).unwrap();
        let detect_time = t.elapsed();

        let groups = detect::group_lines(&boxes);
        let t = Instant::now();
        let mut recognised = 0;
        for group in &groups {
            recognised += recognise::recognise(&engine.rec, &engine.charset, &image, group).len();
        }
        let recognise_time = t.elapsed();

        println!(
            "limit_side_len {limit_side_len:>4}  detect {detect_time:>10?}  recognise {recognise_time:>10?}  \
             {} boxes, {} lines, {recognised} crops",
            boxes.len(),
            groups.len()
        );
    }
}

/// Splits recognition into the part that is our code and the part that is the model, because the
/// remedy is completely different depending on which dominates.
#[test]
#[ignore = "timing, not correctness"]
fn recognition_breakdown() {
    let image = prep::decode(&fixture()).unwrap();
    let engine = engine::shared().unwrap();
    let boxes = detect::detect(&engine.det, &image, &detect::DetConfig::default()).unwrap();

    let t = Instant::now();
    let crops: Vec<_> =
        boxes.iter().map(|r| prep::warp_rect(&image, r, recognise::REC_H)).collect();
    println!("warp {} crops   {:?}", crops.len(), t.elapsed());
    let widths: Vec<u32> = crops.iter().map(|c| c.width()).collect();
    println!("crop widths     {widths:?}");

    let t = Instant::now();
    let _ = prep::to_nchw_bgr(&crops[0], [0.5; 3], [0.5; 3]);
    println!("to_nchw one     {:?}", t.elapsed());

    let t = Instant::now();
    let _ = recognise::recognise(&engine.rec, &engine.charset, &image, &boxes);
    println!("recognise all   {:?}", t.elapsed());

    println!("--- lines as the parser sees them ---");
    for line in text_lines(&fixture()).unwrap() {
        println!(
            "  y={:.3} h={:.3} conf={:.2}  {:?}",
            line.y, line.h, line.confidence, line.text
        );
    }
}
