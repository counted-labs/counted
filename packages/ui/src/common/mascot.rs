//! The Counted mascot: a glazed coin box built on the brand hexagon.
//!
//! The box is a flat-top regular hexagonal prism (circumradius 80, centred on 150,110) seen from
//! 30 degrees above, so the top hexagon projects at half height and the front face is the flat
//! rectangle 110..190 × 144.6..244.6 the face is drawn on. The open lid is the same prism turned
//! 105 degrees about the back rim edge (110..190 at y 75.4), which is why the two share that edge.
//! Every face is stroked in its own paint at 16 with round joins: that is what rounds the corners.
//!
//! Poses differ only in the lid, the face and the object beside the box — never a second copy of
//! the box.

use dioxus::prelude::*;

/// Onboarding hero and other full-width slots.
pub const MASCOT_HERO: u32 = 160;
/// Empty and success states inside a page. Default.
pub const MASCOT_EMPTY: u32 = 120;
/// Compact slots — a single empty chart, a card.
pub const MASCOT_INLINE: u32 = 72;

const INK: &str = "#2E3236";

#[derive(PartialEq, Clone, Copy)]
pub enum MascotPose {
    Searching,
    Settled,
    Empty,
    Mail,
    Locked,
    Offline,
    Secure,
}

#[component]
pub fn Mascot(pose: MascotPose, #[props(default = MASCOT_EMPTY)] size: u32) -> Element {
    // charts_page renders up to five empty charts at once, so fixed gradient ids would collide
    // and every instance after the first would take the first one's stops. The scope id is unique
    // per instance and stable across renders.
    let id = use_hook(|| format!("counted-mascot-{}", dioxus::core::current_scope_id().0));

    let lid_open = matches!(
        pose,
        MascotPose::Settled | MascotPose::Empty | MascotPose::Secure
    );
    // A closed box has nothing above its lid, so it is cropped rather than left floating in the
    // space the open lid needs.
    let (view_box, height) = if lid_open {
        ("0 -80 300 350", size * 350 / 300)
    } else {
        ("0 40 300 230", size * 230 / 300)
    };
    let style = if pose == MascotPose::Offline {
        "filter: grayscale(1); opacity: 0.7"
    } else {
        ""
    };

    let eyes = |cy: &'static str| {
        rsx! {
            ellipse { cx: "-16", cy, rx: "5", ry: "6.5" }
            ellipse { cx: "16", cy, rx: "5", ry: "6.5" }
        }
    };

    let face = match pose {
        MascotPose::Searching | MascotPose::Offline => rsx! {
            {eyes("0")}
            path {
                d: "M-5 14 H5",
                stroke: INK,
                stroke_width: "3.5",
                stroke_linecap: "round",
            }
        },
        MascotPose::Settled => rsx! {
            path {
                d: "M-21 2 Q-16 -5 -11 2 M11 2 Q16 -5 21 2 M-7 10 Q0 19 7 10",
                fill: "none",
                stroke: INK,
                stroke_width: "3.5",
                stroke_linecap: "round",
            }
        },
        MascotPose::Locked => rsx! {
            path {
                d: "M-21 -1 Q-16 4 -11 -1 M11 -1 Q16 4 21 -1 M-5 12 Q0 16 5 12",
                fill: "none",
                stroke: INK,
                stroke_width: "3.5",
                stroke_linecap: "round",
            }
        },
        MascotPose::Empty => rsx! {
            {eyes("-3")}
            ellipse { cx: "0", cy: "14", rx: "3.2", ry: "3.8" }
        },
        MascotPose::Mail | MascotPose::Secure => rsx! {
            {eyes("0")}
            path {
                d: "M-6 12 Q0 18 6 12",
                fill: "none",
                stroke: INK,
                stroke_width: "3.5",
                stroke_linecap: "round",
            }
        },
    };

    let object = match pose {
        MascotPose::Searching => rsx! {
            path {
                d: "M250 217 L234 246",
                stroke: "#006940",
                stroke_width: "11",
                stroke_linecap: "round",
            }
            circle {
                cx: "262",
                cy: "196",
                r: "24",
                fill: "#EAF6F2",
                fill_opacity: "0.85",
                stroke: "url(#{id}-brand)",
                stroke_width: "8",
            }
            path {
                d: "M249 190 Q253 179 264 178",
                fill: "none",
                stroke: "#FFFFFF",
                stroke_width: "4",
                stroke_linecap: "round",
            }
        },
        MascotPose::Settled => rsx! {
            circle {
                cx: "266",
                cy: "120",
                r: "17",
                fill: "url(#{id}-brand)",
                stroke: "#FFFFFF",
                stroke_width: "3",
            }
            path {
                d: "M258 120 L264 126 L275 114",
                fill: "none",
                stroke: "#FFFFFF",
                stroke_width: "4",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        },
        MascotPose::Empty => rsx! {
            path {
                d: "M150 58 V96",
                stroke: "#9AA6AE",
                stroke_width: "2.5",
                stroke_linecap: "round",
                stroke_dasharray: "2 6",
            }
            ellipse {
                cx: "150",
                cy: "34",
                rx: "13",
                ry: "16",
                fill: "url(#{id}-gold)",
                stroke: "#C4841D",
                stroke_width: "2",
            }
            ellipse {
                cx: "150",
                cy: "34",
                rx: "7.5",
                ry: "10",
                fill: "none",
                stroke: "#FBD978",
                stroke_width: "2",
            }
        },
        MascotPose::Mail => rsx! {
            rect {
                x: "238",
                y: "168",
                width: "54",
                height: "40",
                rx: "7",
                fill: "#FFFFFF",
                stroke: "url(#{id}-brand)",
                stroke_width: "5",
            }
            path {
                d: "M243 174 L265 191 L287 174",
                fill: "none",
                stroke: "url(#{id}-brand)",
                stroke_width: "5",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
        },
        // Drawn flat, then sheared onto the right face's plane.
        MascotPose::Locked => rsx! {
            g { transform: "matrix(0.6 -0.52 0 1 210 180)",
                path {
                    d: "M-11 -4 V-12 A11 11 0 0 1 11 -12 V-4",
                    fill: "none",
                    stroke: "#006940",
                    stroke_width: "6",
                    stroke_linecap: "round",
                }
                rect {
                    x: "-17",
                    y: "-7",
                    width: "34",
                    height: "30",
                    rx: "7",
                    fill: "url(#{id}-brand)",
                    stroke: "#FFFFFF",
                    stroke_width: "2.5",
                }
                circle { cx: "0", cy: "5", r: "4", fill: "#FFFFFF" }
                path {
                    d: "M0 7 V14",
                    stroke: "#FFFFFF",
                    stroke_width: "3.5",
                    stroke_linecap: "round",
                }
            }
        },
        MascotPose::Offline => rsx! {
            circle { cx: "250", cy: "184", r: "14", fill: "#C9D1D8" }
            circle { cx: "268", cy: "174", r: "18", fill: "#C9D1D8" }
            circle { cx: "284", cy: "186", r: "12", fill: "#C9D1D8" }
            rect {
                x: "250",
                y: "184",
                width: "34",
                height: "14",
                fill: "#C9D1D8",
            }
            path {
                d: "M244 204 L290 156",
                stroke: "#FFFFFF",
                stroke_width: "10",
                stroke_linecap: "round",
            }
            path {
                d: "M244 204 L290 156",
                stroke: "#7C8A96",
                stroke_width: "5",
                stroke_linecap: "round",
            }
        },
        MascotPose::Secure => rsx! {},
    };

    rsx! {
        svg {
            width: "{size}",
            height: "{height}",
            view_box,
            style,
            fill: "none",
            xmlns: "http://www.w3.org/2000/svg",
            "aria-hidden": "true",
            "focusable": "false",
            defs {
                linearGradient { id: "{id}-brand", x1: "0", y1: "0", x2: "1", y2: "1",
                    stop { offset: "0", stop_color: "#00A169" }
                    stop { offset: "1", stop_color: "#00BEB7" }
                }
                linearGradient { id: "{id}-front", x1: "0", y1: "0", x2: "0", y2: "1",
                    stop { offset: "0", stop_color: "#FFFFFF" }
                    stop { offset: "1", stop_color: "#E9EDF0" }
                }
                linearGradient { id: "{id}-left", x1: "0", y1: "0", x2: "0", y2: "1",
                    stop { offset: "0", stop_color: "#F8FAFB" }
                    stop { offset: "1", stop_color: "#E1E6EA" }
                }
                linearGradient { id: "{id}-right", x1: "0", y1: "0", x2: "1", y2: "1",
                    stop { offset: "0", stop_color: "#D6DDE3" }
                    stop { offset: "1", stop_color: "#C0CAD2" }
                }
                linearGradient { id: "{id}-top", x1: "0", y1: "0", x2: "1", y2: "1",
                    stop { offset: "0", stop_color: "#FFFFFF" }
                    stop { offset: "1", stop_color: "#EEF2F4" }
                }
                linearGradient { id: "{id}-band", x1: "0", y1: "0", x2: "1", y2: "0",
                    stop { offset: "0", stop_color: "#F1F4F6" }
                    stop { offset: "1", stop_color: "#CBD4DB" }
                }
                linearGradient { id: "{id}-inside", x1: "0", y1: "0", x2: "0", y2: "1",
                    stop { offset: "0", stop_color: "#86CDB6" }
                    stop { offset: "1", stop_color: "#E0F4EC" }
                }
                linearGradient { id: "{id}-gold", x1: "0", y1: "0", x2: "1", y2: "0",
                    stop { offset: "0", stop_color: "#B57418" }
                    stop { offset: "0.35", stop_color: "#F2C04E" }
                    stop { offset: "0.6", stop_color: "#FBD978" }
                    stop { offset: "1", stop_color: "#C4841D" }
                }
                radialGradient { id: "{id}-shadow",
                    stop { offset: "0", stop_color: "#1F2A2E", stop_opacity: "0.3" }
                    stop { offset: "1", stop_color: "#1F2A2E", stop_opacity: "0" }
                }
            }
            ellipse {
                cx: "150",
                cy: "250",
                rx: "124",
                ry: "22",
                fill: "url(#{id}-shadow)",
            }
            if lid_open {
                polygon {
                    points: "70,4.8 110,-62.1 190,-62.1 230,4.8 230,8.4 190,75.4 110,75.4 70,8.4",
                    fill: "url(#{id}-band)",
                    stroke: "url(#{id}-band)",
                    stroke_width: "16",
                    stroke_linejoin: "round",
                }
                polygon {
                    points: "230,8.4 190,-58.5 110,-58.5 70,8.4 110,75.4 190,75.4",
                    fill: "url(#{id}-top)",
                    stroke: "url(#{id}-top)",
                    stroke_width: "12",
                    stroke_linejoin: "round",
                }
                polygon {
                    points: "217.2,8.4 183.6,-47.8 116.4,-47.8 82.8,8.4 116.4,64.7 183.6,64.7",
                    fill: "url(#{id}-inside)",
                    stroke: "url(#{id}-inside)",
                    stroke_width: "5",
                    stroke_linejoin: "round",
                }
            }
            polygon {
                points: "70,110 110,144.6 110,244.6 70,210",
                fill: "url(#{id}-left)",
                stroke: "url(#{id}-left)",
                stroke_width: "16",
                stroke_linejoin: "round",
            }
            polygon {
                points: "190,144.6 230,110 230,210 190,244.6",
                fill: "url(#{id}-right)",
                stroke: "url(#{id}-right)",
                stroke_width: "16",
                stroke_linejoin: "round",
            }
            polygon {
                points: "110,144.6 190,144.6 190,244.6 110,244.6",
                fill: "url(#{id}-front)",
                stroke: "url(#{id}-front)",
                stroke_width: "16",
                stroke_linejoin: "round",
            }
            path {
                d: "M109 154 V236",
                stroke: "#FFFFFF",
                stroke_opacity: "0.75",
                stroke_width: "3",
                stroke_linecap: "round",
            }
            if lid_open {
                polygon {
                    points: "230,110 190,144.6 110,144.6 70,110 110,75.4 190,75.4",
                    fill: "url(#{id}-top)",
                    stroke: "url(#{id}-top)",
                    stroke_width: "16",
                    stroke_linejoin: "round",
                }
                polygon {
                    points: "214,110 182,137.7 118,137.7 86,110 118,82.3 182,82.3",
                    fill: "url(#{id}-inside)",
                    stroke: "url(#{id}-inside)",
                    stroke_width: "5",
                    stroke_linejoin: "round",
                }
                // The opening's front edge is level at y 137.7, so the stack simply stops there
                // instead of needing a clip path.
                if pose != MascotPose::Empty {
                    rect {
                        x: "122",
                        y: "62",
                        width: "56",
                        height: "75.7",
                        fill: "url(#{id}-gold)",
                    }
                    path {
                        d: "M122 70 A28 11 0 0 0 178 70 M122 78 A28 11 0 0 0 178 78 M122 86 A28 11 0 0 0 178 86 M122 94 A28 11 0 0 0 178 94 M122 102 A28 11 0 0 0 178 102 M122 110 A28 11 0 0 0 178 110 M122 118 A28 11 0 0 0 178 118",
                        fill: "none",
                        stroke: "#9A5E12",
                        stroke_opacity: "0.4",
                        stroke_width: "1.5",
                    }
                    ellipse {
                        cx: "150",
                        cy: "62",
                        rx: "28",
                        ry: "11",
                        fill: "#F7D06A",
                        stroke: "#D39A2C",
                        stroke_width: "1.5",
                    }
                    ellipse {
                        cx: "150",
                        cy: "62",
                        rx: "19",
                        ry: "7",
                        fill: "none",
                        stroke: "#D39A2C",
                        stroke_width: "2",
                    }
                }
            } else {
                polygon {
                    points: "70,98 110,132.6 190,132.6 230,98 230,110 190,144.6 110,144.6 70,110",
                    fill: "url(#{id}-band)",
                    stroke: "url(#{id}-band)",
                    stroke_width: "16",
                    stroke_linejoin: "round",
                }
                polygon {
                    points: "230,98 190,132.6 110,132.6 70,98 110,63.4 190,63.4",
                    fill: "url(#{id}-top)",
                    stroke: "url(#{id}-top)",
                    stroke_width: "16",
                    stroke_linejoin: "round",
                }
                // Centred on its ink rather than its box: with the right edge missing, the five
                // edges' mean sits 30.5 grid units left of 256, i.e. 3.7 at this scale.
                g { transform: "translate(153.7 98) scale(0.12 0.06) translate(-256 -256)",
                    path {
                        d: "M408.42 168 L256 80 L103.58 168 L103.58 344 L256 432 L408.42 344",
                        fill: "none",
                        stroke: "url(#{id}-brand)",
                        stroke_width: "64",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                    }
                }
            }
            g { transform: "translate(150 190)", fill: INK, {face} }
            {object}
        }
    }
}
