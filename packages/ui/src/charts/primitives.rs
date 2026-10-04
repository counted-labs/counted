use dioxus::prelude::*;

use super::svg::{area_path, column_path, compact, line_path, nice_scale, split_at, ticks, WIDTH};
use crate::common::{Mascot, MascotPose, MASCOT_EMPTY};
use crate::tid;

pub const AXIS_TEXT: &str = "font-size:10px;fill:var(--color-base-content);fill-opacity:.5";
pub const LABEL_TEXT: &str =
    "font-size:11px;font-weight:600;fill:var(--color-base-content);fill-opacity:.7";
pub const STRONG_TEXT: &str = "font-size:12px;font-weight:700;fill:var(--color-base-content)";
const GRID: &str = "var(--color-base-300)";
const INK: &str = "var(--color-base-content)";

#[component]
pub fn ChartCard(title: String, description: Option<String>, children: Element) -> Element {
    rsx! {
        section { class: "card bg-base-100 shadow-soft",
            div { class: "card-body p-0 gap-3",
                header { class: "flex flex-col gap-0.5",
                    h2 { class: "text-sm font-semibold", "{title}" }
                    if let Some(description) = description {
                        p { class: "text-xs text-base-content/70", "{description}" }
                    }
                }
                {children}
            }
        }
    }
}

#[component]
pub fn Swatch(color: &'static str) -> Element {
    rsx! {
        span {
            class: "w-2.5 h-2.5 rounded-[3px] inline-block shrink-0",
            style: "background: {color};",
        }
    }
}

#[component]
pub fn Note(text: String) -> Element {
    rsx! {
        p { class: "text-[11px] text-base-content/50", "{text}" }
    }
}

#[derive(Clone, PartialEq)]
pub struct Segment {
    pub label: String,
    pub active: bool,
    pub enabled: bool,
}

/// `button`, not `input type=radio`: main.css's unlayered `input { font-size: 16px !important }`
/// would outgrow the 360px row.
#[component]
pub fn Segmented(
    aria_label: String,
    segments: Vec<Segment>,
    on_select: EventHandler<usize>,
) -> Element {
    rsx! {
        div {
            role: "radiogroup",
            aria_label: "{aria_label}",
            class: "join w-full",
            for (i , s) in segments.into_iter().enumerate() {
                button {
                    r#type: "button",
                    role: "radio",
                    aria_checked: s.active,
                    disabled: !s.enabled,
                    class: if s.active { "join-item btn btn-primary flex-1 min-h-11 px-1 text-sm whitespace-nowrap" } else { "join-item btn flex-1 min-h-11 px-1 text-sm whitespace-nowrap" },
                    onclick: move |_| on_select.call(i),
                    "{s.label}"
                }
            }
        }
    }
}

#[component]
pub fn EmptyState() -> Element {
    rsx! {
        div { class: "flex flex-col items-center gap-2 py-12 text-base-content/70",
            Mascot { pose: MascotPose::Searching, size: MASCOT_EMPTY }
            span { class: "text-sm", {tid!("charts-nothing-to-show")} }
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct Column {
    pub label: String,
    pub value: f64,
    pub value_label: Option<String>,
}

#[component]
pub fn ColumnChart(
    columns: Vec<Column>,
    color: String,
    average: Option<(f64, String)>,
    aria_label: String,
) -> Element {
    const H: f64 = 170.0;
    const PAD_L: f64 = 30.0;
    const PAD_T: f64 = 18.0;
    const PAD_B: f64 = 20.0;
    let pad_r = if average.is_some() { 40.0 } else { 6.0 };
    let plot_w = WIDTH - PAD_L - pad_r;
    let plot_h = H - PAD_T - PAD_B;
    let max = columns.iter().map(|c| c.value).fold(0.0, f64::max);
    let (step, top) = nice_scale(max, 4.0);
    let y = move |v: f64| PAD_T + plot_h - v / top * plot_h;
    let band = plot_w / columns.len().max(1) as f64;
    let bar_w = 24f64.min(band - 4.0).max(2.0);
    let right = WIDTH - pad_r;

    rsx! {
        svg {
            view_box: "0 0 {WIDTH} {H}",
            class: "w-full h-auto overflow-visible",
            role: "img",
            "aria-label": "{aria_label}",
            for t in ticks(0.0, top, step) {
                line {
                    x1: "{PAD_L}",
                    x2: "{right}",
                    y1: "{y(t):.2}",
                    y2: "{y(t):.2}",
                    stroke: GRID,
                    stroke_width: "1",
                }
                text {
                    x: "{PAD_L - 6.0}",
                    y: "{y(t) + 3.0:.2}",
                    text_anchor: "end",
                    style: AXIS_TEXT,
                    "{compact(t)}"
                }
            }
            for (i , c) in columns.iter().enumerate() {
                {
                    let x = PAD_L + i as f64 * band + (band - bar_w) / 2.0;
                    let top_y = y(c.value);
                    let height = PAD_T + plot_h - top_y;
                    rsx! {
                        if c.value > 0.0 {
                            path { d: column_path(x, top_y, bar_w, height), fill: "{color}" }
                        }
                        if !c.label.is_empty() {
                            text {
                                x: "{x + bar_w / 2.0:.2}",
                                y: "{H - 5.0}",
                                text_anchor: "middle",
                                style: AXIS_TEXT,
                                "{c.label}"
                            }
                        }
                        if let Some(label) = &c.value_label {
                            text {
                                x: "{x + bar_w / 2.0:.2}",
                                y: "{top_y - 5.0:.2}",
                                text_anchor: "middle",
                                style: STRONG_TEXT,
                                "{label}"
                            }
                        }
                    }
                }
            }
            if let Some((value, label)) = average {
                line {
                    x1: "{PAD_L}",
                    x2: "{right + 2.0}",
                    y1: "{y(value):.2}",
                    y2: "{y(value):.2}",
                    stroke: INK,
                    stroke_opacity: ".55",
                    stroke_width: "1",
                }
                text {
                    x: "{right + 6.0}",
                    y: "{y(value) - 2.0:.2}",
                    style: LABEL_TEXT,
                    {tid!("charts-avg")}
                }
                text {
                    x: "{right + 6.0}",
                    y: "{y(value) + 10.0:.2}",
                    style: LABEL_TEXT,
                    "{label}"
                }
            }
        }
    }
}

/// `signed`: a balance, washed green above zero and red below. Otherwise a running total.
#[component]
pub fn LineChart(
    values: Vec<f64>,
    labels: Vec<String>,
    end_label: String,
    signed: bool,
    aria_label: String,
) -> Element {
    const H: f64 = 170.0;
    const PAD_L: f64 = 34.0;
    const PAD_R: f64 = 44.0;
    const PAD_T: f64 = 14.0;
    const PAD_B: f64 = 20.0;
    let plot_w = WIDTH - PAD_L - PAD_R;
    let plot_h = H - PAD_T - PAD_B;
    let max = values.iter().copied().fold(0.0, f64::max);
    let min = values.iter().copied().fold(0.0, f64::min);
    let (step, low, high) = if signed {
        let (step, _) = nice_scale(max - min, 4.0);
        (step, (min / step).floor() * step, (max / step).ceil().max(1.0) * step)
    } else {
        let (step, top) = nice_scale(max, 5.0);
        (step, 0.0, top)
    };
    let y = move |v: f64| PAD_T + plot_h - (v - low) / (high - low) * plot_h;
    let n = values.len();
    let x = move |i: usize| {
        if n > 1 {
            PAD_L + i as f64 / (n - 1) as f64 * plot_w
        } else {
            PAD_L + plot_w / 2.0
        }
    };
    let points: Vec<(f64, f64)> = values.iter().enumerate().map(|(i, v)| (x(i), y(*v))).collect();
    let base = y(0.0);
    let (above, below) = split_at(&points, base);
    let stroke = if signed { INK } else { "var(--color-primary)" };
    let end = points.last().copied();

    rsx! {
        svg {
            view_box: "0 0 {WIDTH} {H}",
            class: "w-full h-auto overflow-visible",
            role: "img",
            "aria-label": "{aria_label}",
            for t in ticks(low, high, step) {
                {
                    let tick = if signed && t > 0.0 {
                        format!("+{}", compact(t))
                    } else {
                        compact(t)
                    };
                    rsx! {
                        line {
                            x1: "{PAD_L}",
                            x2: "{PAD_L + plot_w}",
                            y1: "{y(t):.2}",
                            y2: "{y(t):.2}",
                            stroke: GRID,
                            stroke_width: "1",
                        }
                        text {
                            x: "{PAD_L - 6.0}",
                            y: "{y(t) + 3.0:.2}",
                            text_anchor: "end",
                            style: AXIS_TEXT,
                            "{tick}"
                        }
                    }
                }
            }
            if signed {
                for stretch in above {
                    path {
                        d: area_path(&stretch, base),
                        fill: "var(--color-success)",
                        fill_opacity: ".12",
                    }
                }
                for stretch in below {
                    path {
                        d: area_path(&stretch, base),
                        fill: "var(--color-error)",
                        fill_opacity: ".1",
                    }
                }
                line {
                    x1: "{PAD_L}",
                    x2: "{PAD_L + plot_w}",
                    y1: "{base:.2}",
                    y2: "{base:.2}",
                    stroke: INK,
                    stroke_opacity: ".55",
                    stroke_width: "1",
                }
            } else {
                path {
                    d: area_path(&points, base),
                    fill: "var(--color-primary)",
                    fill_opacity: ".1",
                }
            }
            path {
                d: line_path(&points),
                fill: "none",
                stroke,
                stroke_width: "2",
                stroke_linejoin: "round",
                stroke_linecap: "round",
            }
            for (i , label) in labels.iter().enumerate() {
                if !label.is_empty() {
                    text {
                        x: "{x(i):.2}",
                        y: "{H - 5.0}",
                        text_anchor: "middle",
                        style: AXIS_TEXT,
                        "{label}"
                    }
                }
            }
            if let Some((ex, ey)) = end {
                circle {
                    cx: "{ex:.2}",
                    cy: "{ey:.2}",
                    r: "4.5",
                    fill: stroke,
                    stroke: "var(--color-base-100)",
                    stroke_width: "2",
                }
                text {
                    x: "{ex + 9.0:.2}",
                    y: "{ey + 4.0:.2}",
                    style: STRONG_TEXT,
                    "{end_label}"
                }
            }
        }
    }
}
