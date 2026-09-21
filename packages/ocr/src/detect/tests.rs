use rten_imageproc::Vec2;

use super::*;

fn upright(cx: f32, cy: f32, w: f32, h: f32) -> RotatedRect {
    RotatedRect::new(PointF::from_yx(cy, cx), Vec2::from_xy(0.0, 1.0), w, h)
}

/// A probability map with bright rectangles painted on it, so the post-process can be tested with
/// no model in the loop.
fn map_with(blobs: &[(usize, usize, usize, usize, f32)], h: usize, w: usize) -> NdTensor<f32, 2> {
    let mut data = vec![0.0f32; h * w];
    for (x0, y0, x1, y1, value) in blobs {
        for y in *y0..*y1 {
            for x in *x0..*x1 {
                data[y * w + x] = *value;
            }
        }
    }
    NdTensor::from_data([h, w], data)
}

#[test]
fn two_blobs_become_two_boxes() {
    let map = map_with(&[(10, 10, 40, 20, 0.9), (10, 40, 40, 50, 0.9)], 100, 100);
    let boxes = postprocess(map.view(), &DetConfig::default());
    assert_eq!(boxes.len(), 2);
}

#[test]
fn a_blob_below_the_binarisation_threshold_is_invisible() {
    let map = map_with(&[(10, 10, 40, 20, 0.15)], 100, 100);
    assert!(postprocess(map.view(), &DetConfig::default()).is_empty());
}

/// Above the 0.2 binarisation threshold but below the 0.45 box score: the shape exists, it just is
/// not confidently text.
#[test]
fn a_faint_blob_is_dropped_by_the_box_score() {
    let map = map_with(&[(10, 10, 40, 20, 0.3)], 100, 100);
    assert!(postprocess(map.view(), &DetConfig::default()).is_empty());
}

/// The bug this pins: a long line at a few degrees of tilt. Its axis-aligned bounds are mostly
/// empty corner triangles, and averaging over those halved the score of a header that was in fact
/// detected at 0.87 — "SAS LE REFUGE", 5 degrees, dropped for scoring 0.43 against 0.45.
#[test]
fn a_tilted_line_is_scored_inside_its_own_polygon() {
    let (h, w) = (200usize, 400usize);
    let mut data = vec![0.0f32; h * w];
    let tilt = 0.1_f32;
    for x in 20..380usize {
        let centre = 100.0 + (x as f32 - 200.0) * tilt;
        for y in (centre - 5.0) as usize..(centre + 5.0) as usize {
            data[y * w + x] = 0.9;
        }
    }
    let map = NdTensor::from_data([h, w], data);
    let boxes = postprocess(map.view(), &DetConfig::default());
    assert_eq!(boxes.len(), 1);
}

#[test]
fn a_sliver_thinner_than_the_minimum_side_is_dropped() {
    let map = map_with(&[(10, 10, 60, 11, 0.9)], 100, 100);
    let cfg = DetConfig { unclip_ratio: 0.0, ..DetConfig::default() };
    assert!(postprocess(map.view(), &cfg).is_empty());
}

#[test]
fn unclip_grows_the_box_by_the_vatti_offset() {
    // 30x10: area 300, perimeter 80, d = 300 * 1.4 / 80 = 5.25, so each side grows by 2d = 10.5.
    let grown = unclip(&upright(50.0, 50.0, 30.0, 10.0), 1.4);
    assert!((grown.width() - 40.5).abs() < 1e-3, "width {}", grown.width());
    assert!((grown.height() - 20.5).abs() < 1e-3, "height {}", grown.height());
    assert_eq!(grown.center().x, 50.0, "unclip is centred");
}

#[test]
fn unclip_of_a_degenerate_box_is_a_no_op() {
    let rect = upright(5.0, 5.0, 0.0, 0.0);
    assert_eq!(unclip(&rect, 1.4).width(), 0.0);
}

// ─── reading order ───────────────────────────────────────────────────────────

/// The bug this pins: "TOTAL" and "12,90" are separate detections, and the parser only anchors an
/// amount to a keyword on the *same* line.
#[test]
fn two_boxes_on_one_visual_line_become_one_group() {
    let boxes = [upright(20.0, 100.0, 40.0, 10.0), upright(90.0, 101.0, 30.0, 10.0)];
    let groups = group_lines(&boxes);
    assert_eq!(groups.len(), 1);
    assert_eq!(groups[0].len(), 2);
}

#[test]
fn separate_lines_stay_separate() {
    let boxes = [
        upright(20.0, 100.0, 40.0, 10.0),
        upright(20.0, 130.0, 40.0, 10.0),
        upright(20.0, 160.0, 40.0, 10.0),
    ];
    assert_eq!(group_lines(&boxes).len(), 3);
}

#[test]
fn a_group_is_ordered_left_to_right() {
    let boxes = [upright(90.0, 100.0, 30.0, 10.0), upright(20.0, 100.0, 40.0, 10.0)];
    let groups = group_lines(&boxes);
    assert_eq!(groups[0][0].center().x, 20.0);
    assert_eq!(groups[0][1].center().x, 90.0);
}

#[test]
fn groups_are_ordered_top_to_bottom() {
    let boxes = [upright(20.0, 160.0, 40.0, 10.0), upright(20.0, 100.0, 40.0, 10.0)];
    let groups = group_lines(&boxes);
    assert_eq!(groups[0][0].center().y, 100.0);
    assert_eq!(groups[1][0].center().y, 160.0);
}

#[test]
fn no_boxes_is_no_groups() {
    assert!(group_lines(&[]).is_empty());
}

#[test]
fn group_bounds_are_fractions_of_the_image() {
    let group = vec![upright(100.0, 50.0, 40.0, 20.0)];
    let (x, y, h) = group_bounds(&group, 200.0, 100.0);
    assert!((x - 0.4).abs() < 1e-6, "x {x}");
    assert!((y - 0.5).abs() < 1e-6, "y {y}");
    assert!((h - 0.2).abs() < 1e-6, "h {h}");
}
