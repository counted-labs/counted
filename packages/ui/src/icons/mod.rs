use dioxus::prelude::*;

// Icon sizes, named after where they are used. Every icon takes an optional
// `size` prop that defaults to `ICON_ACTION`.

/// Bottom navigation dock — large touch targets.
pub const ICON_NAV: u32 = 26;
/// Page header / navbar actions (back, settings, history).
pub const ICON_HEADER: u32 = 22;
/// Buttons, form fields, list rows. Default.
pub const ICON_ACTION: u32 = 20;
/// Inline with text (alerts, badges, timeline bullets).
pub const ICON_INLINE: u32 = 16;

#[component]
pub fn BackArrowIcon(#[props(default = ICON_ACTION)] size: u32) -> Element {
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            "aria-hidden": "true",
            width: "{size}",
            height: "{size}",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "2",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            polyline { points: "15 18 9 12 15 6" }
        }
    }
}

#[component]
pub fn RightArrowIcon(#[props(default = ICON_ACTION)] size: u32) -> Element {
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            "aria-hidden": "true",
            width: "{size}",
            height: "{size}",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "1.5",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            polyline { points: "12 18 18 12 6 12 18 12 12 6" }
        }
    }
}

#[component]
pub fn TrashIcon(#[props(default = ICON_ACTION)] size: u32) -> Element {
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            "aria-hidden": "true",
            width: "{size}",
            height: "{size}",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "2",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            path { stroke: "none", d: "M0 0h24v24H0z", fill: "none" }
            path { d: "M4 7l16 0" }
            path { d: "M10 11l0 6" }
            path { d: "M14 11l0 6" }
            path { d: "M5 7l1 12a2 2 0 0 0 2 2h8a2 2 0 0 0 2 -2l1 -12" }
            path { d: "M9 7v-3a1 1 0 0 1 1 -1h4a1 1 0 0 1 1 1v3" }
        }
    }
}

#[component]
pub fn UserIcon(#[props(default = ICON_ACTION)] size: u32) -> Element {
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            "aria-hidden": "true",
            width: "{size}",
            height: "{size}",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "2",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            path { d: "M19 21v-2a4 4 0 0 0-4-4H9a4 4 0 0 0-4 4v2" }
            circle { cx: "12", cy: "7", r: "4" }
        }
    }
}

#[component]
pub fn SettingsIcon(#[props(default = ICON_ACTION)] size: u32) -> Element {
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            "aria-hidden": "true",
            width: "{size}",
            height: "{size}",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "2",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            path { d: "M12.22 2h-.44a2 2 0 0 0-2 2v.18a2 2 0 0 1-1 1.73l-.43.25a2 2 0 0 1-2 0l-.15-.08a2 2 0 0 0-2.73.73l-.22.38a2 2 0 0 0 .73 2.73l.15.1a2 2 0 0 1 1 1.72v.51a2 2 0 0 1-1 1.74l-.15.09a2 2 0 0 0-.73 2.73l.22.38a2 2 0 0 0 2.73.73l.15-.08a2 2 0 0 1 2 0l.43.25a2 2 0 0 1 1 1.73V20a2 2 0 0 0 2 2h.44a2 2 0 0 0 2-2v-.18a2 2 0 0 1 1-1.73l.43-.25a2 2 0 0 1 2 0l.15.08a2 2 0 0 0 2.73-.73l.22-.39a2 2 0 0 0-.73-2.73l-.15-.08a2 2 0 0 1-1-1.74v-.5a2 2 0 0 1 1-1.74l.15-.09a2 2 0 0 0 .73-2.73l-.22-.38a2 2 0 0 0-2.73-.73l-.15.08a2 2 0 0 1-2 0l-.43-.25a2 2 0 0 1-1-1.73V4a2 2 0 0 0-2-2z" }
            circle { cx: "12", cy: "12", r: "3" }
        }
    }
}

#[component]
pub fn BurgerIcon(#[props(default = ICON_ACTION)] size: u32) -> Element {
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            "aria-hidden": "true",
            width: "{size}",
            height: "{size}",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "2",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            path { d: "M3 12h18M3 6h18M3 18h18" }
        }
    }
}

#[component]
pub fn DollarIcon(#[props(default = ICON_ACTION)] size: u32) -> Element {
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            "aria-hidden": "true",
            width: "{size}",
            height: "{size}",
            view_box: "0 0 24 24",
            fill: "none",
            path {
                fill_rule: "evenodd",
                clip_rule: "evenodd",
                d: "M12 2.75C6.89137 2.75 2.75 6.89137 2.75 12C2.75 17.1086 6.89137 21.25 12 21.25C17.1086 21.25 21.25 17.1086 21.25 12C21.25 6.89137 17.1086 2.75 12 2.75ZM1.25 12C1.25 6.06294 6.06294 1.25 12 1.25C17.9371 1.25 22.75 6.06294 22.75 12C22.75 17.9371 17.9371 22.75 12 22.75C6.06294 22.75 1.25 17.9371 1.25 12ZM12 5.25C12.4142 5.25 12.75 5.58579 12.75 6V6.31673C14.3804 6.60867 15.75 7.83361 15.75 9.5C15.75 9.91421 15.4142 10.25 15 10.25C14.5858 10.25 14.25 9.91421 14.25 9.5C14.25 8.82154 13.6859 8.10339 12.75 7.84748V11.3167C14.3804 11.6087 15.75 12.8336 15.75 14.5C15.75 16.1664 14.3804 17.3913 12.75 17.6833V18C12.75 18.4142 12.4142 18.75 12 18.75C11.5858 18.75 11.25 18.4142 11.25 18V17.6833C9.61957 17.3913 8.25 16.1664 8.25 14.5C8.25 14.0858 8.58579 13.75 9 13.75C9.41421 13.75 9.75 14.0858 9.75 14.5C9.75 15.1785 10.3141 15.8966 11.25 16.1525V12.6833C9.61957 12.3913 8.25 11.1664 8.25 9.5C8.25 7.83361 9.61957 6.60867 11.25 6.31673V6C11.25 5.58579 11.5858 5.25 12 5.25ZM11.25 7.84748C10.3141 8.10339 9.75 8.82154 9.75 9.5C9.75 10.1785 10.3141 10.8966 11.25 11.1525V7.84748ZM12.75 12.8475V16.1525C13.6859 15.8966 14.25 15.1785 14.25 14.5C14.25 13.8215 13.6859 13.1034 12.75 12.8475Z",
                fill: "currentColor",
            }
        }
    }
}

#[component]
pub fn PlusIcon(#[props(default = ICON_ACTION)] size: u32) -> Element {
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            "aria-hidden": "true",
            width: "{size}",
            height: "{size}",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "2.5",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            path { d: "M12 5v14M5 12h14" }
        }
    }
}

/// The open state of a control whose closed state is `PlusIcon` — same stroke weight on purpose,
/// so the two do not appear to change thickness when a FAB toggles between them.
#[component]
pub fn CloseIcon(#[props(default = ICON_ACTION)] size: u32) -> Element {
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            "aria-hidden": "true",
            width: "{size}",
            height: "{size}",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "2.5",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            path { d: "M18 6 6 18M6 6l12 12" }
        }
    }
}

#[component]
pub fn PencilIcon(#[props(default = ICON_ACTION)] size: u32) -> Element {
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            "aria-hidden": "true",
            width: "{size}",
            height: "{size}",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "2",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            path { d: "M17 3a2.85 2.85 0 1 1 4 4L7.5 20.5 2 22l1.5-5.5Z" }
        }
    }
}

#[component]
pub fn HistoryIcon(#[props(default = ICON_ACTION)] size: u32) -> Element {
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            "aria-hidden": "true",
            width: "{size}",
            height: "{size}",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "2",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            path { d: "M3 12a9 9 0 1 0 9-9 9.75 9.75 0 0 0-6.74 2.74L3 8" }
            path { d: "M3 3v5h5" }
            path { d: "M12 7v5l4 2" }
        }
    }
}

#[component]
pub fn HomeIcon(#[props(default = ICON_ACTION)] size: u32) -> Element {
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            "aria-hidden": "true",
            width: "{size}",
            height: "{size}",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "2",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            path { d: "M3 9.5 12 3l9 6.5" }
            path { d: "M5 10v10a1 1 0 0 0 1 1h12a1 1 0 0 0 1-1V10" }
            path { d: "M9 21v-6h6v6" }
        }
    }
}

#[component]
pub fn LockIcon(#[props(default = ICON_ACTION)] size: u32) -> Element {
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            "aria-hidden": "true",
            width: "{size}",
            height: "{size}",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "2",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            rect { x: "3", y: "11", width: "18", height: "11", rx: "2" }
            path { d: "M7 11V7a5 5 0 0 1 10 0v4" }
        }
    }
}

#[component]
pub fn ChartIcon(#[props(default = ICON_ACTION)] size: u32) -> Element {
    rsx! {
        svg {
            class: "icon icon-tabler icons-tabler-outline icon-tabler-chart-pie-2",
            xmlns: "http://www.w3.org/2000/svg",
            "aria-hidden": "true",
            width: "{size}",
            height: "{size}",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "2",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            path { d: "M0 0h24v24H0z", fill: "none", stroke: "none" }
            path { d: "M12 3v9h9" }
            path { d: "M3 12a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" }
        }
    }
}

#[component]
pub fn LinkIcon(#[props(default = ICON_ACTION)] size: u32) -> Element {
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            "aria-hidden": "true",
            width: "{size}",
            height: "{size}",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "2",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            path { d: "M10 13a5 5 0 0 0 7.54.54l3-3a5 5 0 0 0-7.07-7.07l-1.72 1.71" }
            path { d: "M14 11a5 5 0 0 0-7.54-.54l-3 3a5 5 0 0 0 7.07 7.07l1.71-1.71" }
        }
    }
}

#[component]
pub fn ShareIcon(#[props(default = ICON_ACTION)] size: u32) -> Element {
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            "aria-hidden": "true",
            width: "{size}",
            height: "{size}",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "2",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            circle { cx: "18", cy: "5", r: "3" }
            circle { cx: "6", cy: "12", r: "3" }
            circle { cx: "18", cy: "19", r: "3" }
            line { x1: "8.59", y1: "13.51", x2: "15.42", y2: "17.49" }
            line { x1: "15.41", y1: "6.51", x2: "8.59", y2: "10.49" }
        }
    }
}

#[component]
pub fn ImportIcon(#[props(default = ICON_ACTION)] size: u32) -> Element {
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            "aria-hidden": "true",
            width: "{size}",
            height: "{size}",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "2",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            path { d: "M12 3v12" }
            path { d: "m8 11 4 4 4-4" }
            path { d: "M8 5H4a2 2 0 0 0-2 2v10a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2V7a2 2 0 0 0-2-2h-4" }
        }
    }
}

/// The Scan-a-receipt trigger. Same stroke weight as `PlusIcon`, which it sits beside.
#[component]
pub fn CameraIcon(#[props(default = ICON_ACTION)] size: u32) -> Element {
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            "aria-hidden": "true",
            width: "{size}",
            height: "{size}",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "2.5",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            path { stroke: "none", d: "M0 0h24v24H0z", fill: "none" }
            path { d: "M5 7h1a2 2 0 0 0 2 -2a1 1 0 0 1 1 -1h6a1 1 0 0 1 1 1a2 2 0 0 0 2 2h1a2 2 0 0 1 2 2v9a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2v-9a2 2 0 0 1 2 -2" }
            path { d: "M12 17a3 3 0 1 0 0 -6a3 3 0 0 0 0 6" }
        }
    }
}

#[component]
pub fn BellIcon(#[props(default = ICON_ACTION)] size: u32) -> Element {
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            "aria-hidden": "true",
            width: "{size}",
            height: "{size}",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "2",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            path { d: "M18 8A6 6 0 0 0 6 8c0 7-3 9-3 9h18s-3-2-3-9" }
            path { d: "M13.73 21a2 2 0 0 1-3.46 0" }
        }
    }
}
