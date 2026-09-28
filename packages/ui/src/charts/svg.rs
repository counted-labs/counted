use std::f64::consts::PI;

pub const WIDTH: f64 = 328.0;

/// A round step giving about `count` gridlines up to `max`, and the top of the axis.
pub fn nice_scale(max: f64, count: f64) -> (f64, f64) {
    if max <= 0.0 {
        return (1.0, 1.0);
    }
    let raw = max / count;
    let pow = 10f64.powf(raw.log10().floor());
    let step = [1.0, 2.0, 2.5, 5.0, 10.0].iter().map(|m| m * pow).find(|s| *s >= raw).unwrap_or(10.0 * pow);
    (step, (max / step).ceil() * step)
}

pub fn ticks(low: f64, high: f64, step: f64) -> Vec<f64> {
    let n = ((high - low) / step).round() as i64;
    (0..=n).map(|i| low + i as f64 * step).collect()
}

/// `1k`, `1.5k`, `250`: axis labels only, the marks carry the exact values.
pub fn compact(value: f64) -> String {
    let abs = value.abs();
    let sign = if value < 0.0 { "−" } else { "" };
    if abs >= 1000.0 {
        let k = abs / 1000.0;
        let text = if (k - k.round()).abs() < 0.05 { format!("{}", k.round()) } else { format!("{k:.1}") };
        format!("{sign}{text}k")
    } else if abs.fract() > 0.05 {
        format!("{sign}{abs:.1}")
    } else {
        format!("{sign}{}", abs.round())
    }
}

/// A column growing from the baseline: 4px rounded top, square foot.
pub fn column_path(x: f64, y: f64, w: f64, h: f64) -> String {
    let r = 4f64.min(h).min(w / 2.0);
    format!(
        "M{x:.2},{b:.2}V{yr:.2}Q{x:.2},{y:.2} {xr:.2},{y:.2}H{xw_r:.2}Q{xw:.2},{y:.2} {xw:.2},{yr:.2}V{b:.2}Z",
        b = y + h,
        yr = y + r,
        xr = x + r,
        xw_r = x + w - r,
        xw = x + w,
    )
}

/// One ring segment between `start` and `end`, as fractions of the full turn from 12 o'clock.
pub fn donut_path(center: f64, outer: f64, inner: f64, start: f64, end: f64) -> String {
    if end - start >= 0.9999 {
        return format!(
            "M{l:.2},{center} A{outer},{outer} 0 1 1 {r:.2},{center} A{outer},{outer} 0 1 1 {l:.2},{center}Z \
             M{li:.2},{center} A{inner},{inner} 0 1 0 {ri:.2},{center} A{inner},{inner} 0 1 0 {li:.2},{center}Z",
            l = center - outer,
            r = center + outer,
            li = center - inner,
            ri = center + inner,
        );
    }
    let a0 = start * 2.0 * PI - PI / 2.0;
    let a1 = end * 2.0 * PI - PI / 2.0;
    let large = if end - start > 0.5 { 1 } else { 0 };
    let p = |r: f64, a: f64| (center + r * a.cos(), center + r * a.sin());
    let (x0, y0) = p(outer, a0);
    let (x1, y1) = p(outer, a1);
    let (x2, y2) = p(inner, a1);
    let (x3, y3) = p(inner, a0);
    format!(
        "M{x0:.2},{y0:.2} A{outer},{outer} 0 {large} 1 {x1:.2},{y1:.2} L{x2:.2},{y2:.2} \
         A{inner},{inner} 0 {large} 0 {x3:.2},{y3:.2}Z"
    )
}

pub fn line_path(points: &[(f64, f64)]) -> String {
    points
        .iter()
        .enumerate()
        .map(|(i, (x, y))| format!("{}{x:.2},{y:.2}", if i == 0 { "M" } else { "L" }))
        .collect()
}

/// The area between the line and `base`, closed on the baseline.
pub fn area_path(points: &[(f64, f64)], base: f64) -> String {
    match (points.first(), points.last()) {
        (Some((x0, _)), Some((xn, _))) => format!("{}L{xn:.2},{base:.2}L{x0:.2},{base:.2}Z", line_path(points)),
        _ => String::new(),
    }
}

/// The line split where it crosses `base`, into the stretches above and below it (screen y grows
/// downward, so "above" is `y < base`). Each stretch is closed on the baseline by [`area_path`].
pub fn split_at(points: &[(f64, f64)], base: f64) -> (Vec<Vec<(f64, f64)>>, Vec<Vec<(f64, f64)>>) {
    let mut above: Vec<Vec<(f64, f64)>> = vec![];
    let mut below: Vec<Vec<(f64, f64)>> = vec![];
    let mut current: Vec<(f64, f64)> = vec![];
    let mut current_above: Option<bool> = None;
    for (i, &(x, y)) in points.iter().enumerate() {
        let is_above = y <= base;
        if let (Some(was_above), Some(&(px, py))) = (current_above, i.checked_sub(1).and_then(|j| points.get(j))) {
            if was_above != is_above {
                let t = (base - py) / (y - py);
                let crossing = (px + (x - px) * t, base);
                current.push(crossing);
                let done = std::mem::replace(&mut current, vec![crossing]);
                if was_above { above.push(done) } else { below.push(done) }
            }
        }
        current.push((x, y));
        current_above = Some(is_above);
    }
    if let Some(was_above) = current_above {
        if was_above { above.push(current) } else { below.push(current) }
    }
    (above, below)
}

/// Every `k`-th label so that at most `max` show.
pub fn label_every(count: usize, max: usize) -> usize {
    count.div_ceil(max.max(1)).max(1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nice_scale_rounds_up_to_a_clean_top() {
        assert_eq!(nice_scale(520.0, 4.0), (200.0, 600.0));
        assert_eq!(nice_scale(402.0, 4.0), (200.0, 600.0));
        assert_eq!(nice_scale(8969.0, 5.0), (2000.0, 10000.0));
        assert_eq!(nice_scale(0.0, 4.0), (1.0, 1.0));
    }

    #[test]
    fn ticks_include_both_ends() {
        assert_eq!(ticks(-100.0, 400.0, 100.0), [-100.0, 0.0, 100.0, 200.0, 300.0, 400.0]);
    }

    #[test]
    fn compact_labels() {
        assert_eq!(compact(0.0), "0");
        assert_eq!(compact(600.0), "600");
        assert_eq!(compact(2000.0), "2k");
        assert_eq!(compact(2500.0), "2.5k");
        assert_eq!(compact(-100.0), "−100");
    }

    #[test]
    fn compact_keeps_a_fractional_step() {
        assert_eq!(compact(2.5), "2.5");
        assert_eq!(compact(7.5), "7.5");
        assert_eq!(compact(-2.5), "−2.5");
    }

    #[test]
    fn a_tiny_column_does_not_invert_its_radius() {
        let d = column_path(0.0, 99.0, 20.0, 1.0);
        assert!(d.starts_with("M0.00,100.00V100.00"), "{d}");
    }

    #[test]
    fn a_full_donut_draws_a_ring() {
        assert_eq!(donut_path(90.0, 84.0, 54.0, 0.0, 1.0).matches('M').count(), 2);
    }

    #[test]
    fn split_at_cuts_on_the_crossing() {
        let points = [(0.0, 10.0), (10.0, 30.0), (20.0, 10.0)];
        let (above, below) = split_at(&points, 20.0);
        assert_eq!(above.len(), 2);
        assert_eq!(below, vec![vec![(5.0, 20.0), (10.0, 30.0), (15.0, 20.0)]]);
        assert_eq!(above[0], vec![(0.0, 10.0), (5.0, 20.0)]);
    }

    #[test]
    fn split_at_with_no_crossing_is_one_stretch() {
        let (above, below) = split_at(&[(0.0, 1.0), (1.0, 2.0)], 5.0);
        assert_eq!((above.len(), below.len()), (1, 0));
    }

    #[test]
    fn label_every_thins_long_axes() {
        assert_eq!(label_every(10, 12), 1);
        assert_eq!(label_every(30, 12), 3);
        assert_eq!(label_every(0, 12), 1);
    }
}
