use dioxus::prelude::*;

use crate::categories::category_label;
use crate::charts::money;
use crate::common::{Avatar, Mascot, MascotPose, SizeClass};
use crate::tid;

const LOCK_PATH: &str = "M16.5 10.5V6.75a4.5 4.5 0 1 0-9 0v3.75m-.75 11.25h10.5a2.25 2.25 0 0 0 2.25-2.25v-6.75a2.25 2.25 0 0 0-2.25-2.25H6.75a2.25 2.25 0 0 0-2.25 2.25v6.75a2.25 2.25 0 0 0 2.25 2.25Z";
const LINK_PATH: &str = "M13.19 8.688a4.5 4.5 0 0 1 1.242 7.244l-4.5 4.5a4.5 4.5 0 0 1-6.364-6.364l1.757-1.757m13.35-.622 1.757-1.757a4.5 4.5 0 0 0-6.364-6.364l-4.5 4.5a4.5 4.5 0 0 0 1.242 7.244";
const CHECK_PATH: &str = "m4.5 12.75 6 6 9-13.5";
const PLUS_PATH: &str = "M12 4.5v15m7.5-7.5h-15";

#[component]
pub(super) fn Icon(path: &'static str, class: &'static str) -> Element {
    rsx! {
        svg {
            class,
            xmlns: "http://www.w3.org/2000/svg",
            "aria-hidden": "true",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "2",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            path { d: path }
        }
    }
}

#[component]
pub fn PromiseArt() -> Element {
    rsx! {
        div { class: "size-56 rounded-full bg-base-100/70 flex items-center justify-center",
            Mascot { pose: MascotPose::Secure, size: 150 }
        }
    }
}

#[component]
pub fn LinkArt() -> Element {
    rsx! {
        div { class: "w-70 max-w-[80%] -mt-10 rounded-2xl bg-base-100 p-4 shadow-lg",
            p { class: "font-display text-lg font-bold truncate", {tid!("welcome-demo-project")} }
            div { class: "mt-3 flex -space-x-2",
                Avatar {
                    initials: "C".to_string(),
                    color_class: "bg-primary text-primary-content".to_string(),
                    size: SizeClass::W10,
                }
                Avatar {
                    initials: "L".to_string(),
                    color_class: "bg-info text-info-content".to_string(),
                    size: SizeClass::W10,
                }
                Avatar {
                    initials: "M".to_string(),
                    color_class: "bg-warning text-warning-content".to_string(),
                    size: SizeClass::W10,
                }
                div { class: "size-10 rounded-full border-2 border-dashed border-secondary bg-base-200 flex items-center justify-center",
                    Icon { path: PLUS_PATH, class: "size-4 text-base-content/70" }
                }
            }
            div { class: "mt-3 flex items-center gap-2 rounded-xl bg-base-200 py-2 ps-3 pe-2",
                Icon { path: LINK_PATH, class: "size-4 shrink-0 text-base-content/70" }
                span { class: "flex-1 min-w-0 truncate text-sm text-base-content/70", "counted.fr/…" }
                span { class: "rounded-lg bg-neutral px-2.5 py-1 text-xs font-semibold text-neutral-content",
                    {tid!("share-link")}
                }
            }
        }
        div { class: "absolute bottom-4 end-5 [@media(max-height:700px)]:hidden",
            Mascot { pose: MascotPose::Mail, size: 110 }
        }
    }
}

#[component]
fn LockedTag(label: String, class: &'static str) -> Element {
    rsx! {
        span { class: "absolute {class} inline-flex items-center gap-1.5 rounded-full bg-base-100 px-3 py-2 text-sm font-semibold shadow-md",
            Icon { path: LOCK_PATH, class: "size-3.5 text-primary" }
            {label}
        }
    }
}

#[component]
pub fn PrivateArt() -> Element {
    rsx! {
        Mascot { pose: MascotPose::Locked, size: 190 }
        LockedTag { label: tid!("welcome-private-names"), class: "start-7 top-[15%]" }
        LockedTag { label: tid!("welcome-private-amounts"), class: "end-6 top-[35%]" }
        LockedTag { label: tid!("welcome-private-projects"), class: "start-11 bottom-[15%]" }
    }
}

#[component]
fn FilledField(label: String, value: String) -> Element {
    rsx! {
        div { class: "flex items-center gap-2 rounded-xl bg-base-100 px-2.5 py-2 shadow-sm",
            div { class: "flex-1 min-w-0",
                p { class: "text-xs text-base-content/70 truncate", {label} }
                p { class: "text-sm font-semibold truncate", {value} }
            }
            Icon { path: CHECK_PATH, class: "size-4 shrink-0 text-primary" }
        }
    }
}

#[component]
pub fn ScanArt() -> Element {
    rsx! {
        div { class: "flex items-center gap-4 px-5",
            div { class: "relative shrink-0 p-2.5",
                span { class: "absolute start-0 top-0 size-4.5 rounded-tl-md border-s-3 border-t-3 border-primary" }
                span { class: "absolute end-0 top-0 size-4.5 rounded-tr-md border-e-3 border-t-3 border-primary" }
                span { class: "absolute start-0 bottom-0 size-4.5 rounded-bl-md border-s-3 border-b-3 border-primary" }
                span { class: "absolute end-0 bottom-0 size-4.5 rounded-br-md border-e-3 border-b-3 border-primary" }
                div { class: "w-28 -rotate-3 rounded bg-base-100 p-3 shadow-lg flex flex-col gap-2",
                    div { class: "h-1.5 w-3/4 self-center rounded-full bg-base-300" }
                    div { class: "h-1 w-full rounded-full bg-base-200" }
                    div { class: "h-1 w-5/6 rounded-full bg-base-200" }
                    div { class: "h-1 w-11/12 rounded-full bg-base-200" }
                    div { class: "h-1 w-2/3 rounded-full bg-base-200" }
                    div { class: "mt-1 h-2 w-1/2 self-end rounded-full bg-primary" }
                }
            }
            div { class: "flex w-40 min-w-0 flex-col gap-2",
                FilledField { label: tid!("field-amount"), value: money(43.0, "EUR") }
                FilledField { label: tid!("field-date"), value: tid!("date-today") }
                FilledField { label: tid!("expense-category"), value: category_label("Nourriture") }
            }
        }
    }
}
