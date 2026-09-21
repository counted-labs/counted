//! The gate spike (docs/plans/ocr-receipt-scan.md §8, step 0). Nothing downstream of these is
//! worth writing until they pass: they are what proves PP-OCRv6 runs on rten at all, and that the
//! charset extraction lines up with what the recogniser actually emits.

use rten_tensor::prelude::*;
use rten_tensor::NdTensor;

use super::*;

/// A checkout without git-lfs compiles fine and then fails every model test with an unreadable
/// rten error. This turns that into one sentence.
#[test]
fn the_weights_are_not_lfs_pointers() {
    assert!(
        !DET.starts_with(b"version https://git-lfs"),
        "packages/ocr/models holds LFS pointer files - run `git lfs pull`"
    );
    assert!(DET.len() > 9_000_000, "det.onnx is {} bytes", DET.len());
    assert!(REC.len() > 20_000_000, "rec.onnx is {} bytes", REC.len());
}

#[test]
fn the_charset_survived_extraction() {
    let charset = parse_charset(CHARSET);
    assert_eq!(charset.len(), 18708);
    assert_eq!(charset[0], '!');
    // U+3000 IDEOGRAPHIC SPACE. It is the one entry a naive trim would drop, and dropping it
    // would shift every CTC index after it.
    assert_eq!(charset[1748], '\u{3000}');
}

#[test]
fn both_models_load() {
    let engine = Engine::load().expect("PP-OCRv6 must load in rten");
    println!("det inputs {:?} outputs {:?}", engine.det.input_ids(), engine.det.output_ids());
    println!("rec inputs {:?} outputs {:?}", engine.rec.input_ids(), engine.rec.output_ids());
    println!("det input shape {:?}", engine.det.input_shape(0));
    println!("rec input shape {:?}", engine.rec.input_shape(0));
}

#[test]
fn det_produces_a_probability_map() {
    let engine = Engine::load().unwrap();
    let input = NdTensor::<f32, 4>::zeros([1, 3, 960, 960]);
    let output = engine.det.run_one(input.view().into(), None).expect("det must run");
    let out: NdTensor<f32, 4> = output.try_into().expect("det output is NCHW f32");
    println!("det output shape {:?}", out.shape());
    assert_eq!(out.shape()[0], 1);
    assert_eq!(out.shape()[1], 1, "DBNet emits a single-channel probability map");
    assert_eq!(out.shape()[2], 960);
    assert_eq!(out.shape()[3], 960);
}

/// Also the charset check: `C` is blank plus the dictionary, so a mis-extracted dictionary shows
/// up here rather than as silently garbled text.
#[test]
fn rec_produces_ctc_logits_matching_the_charset() {
    let engine = Engine::load().unwrap();
    let input = NdTensor::<f32, 4>::zeros([1, 3, 48, 320]);
    let output = engine.rec.run_one(input.view().into(), None).expect("rec must run");
    let out: NdTensor<f32, 3> = output.try_into().expect("rec output is [N, T, C] f32");
    println!("rec output shape {:?}, charset {}", out.shape(), engine.charset.len());
    assert_eq!(out.shape()[0], 1);
    // Blank, the 18708 dictionary entries, and the appended space. Measured on 2026-09-04.
    assert_eq!(out.shape()[2], engine.charset.len() + 2);
    assert_eq!(decode_index(&engine.charset, 0), None, "class 0 is the CTC blank");
    assert_eq!(decode_index(&engine.charset, 1), Some('!'));
    assert_eq!(decode_index(&engine.charset, engine.charset.len() + 1), Some(' '));
}

/// The width the recogniser is actually run at. `inference.yml` declares dynamic shapes up to
/// [8, 3, 48, 3200], so a batch must be accepted.
#[test]
fn rec_accepts_a_batch() {
    let engine = Engine::load().unwrap();
    let input = NdTensor::<f32, 4>::zeros([8, 3, 48, 320]);
    let output = engine.rec.run_one(input.view().into(), None).expect("rec must batch");
    let out: NdTensor<f32, 3> = output.try_into().unwrap();
    assert_eq!(out.shape()[0], 8);
}
