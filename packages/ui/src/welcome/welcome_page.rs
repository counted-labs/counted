use dioxus::prelude::*;
use crate::tid;

use crate::route::Route;

use super::welcome_art::{Icon, LinkArt, PrivateArt, PromiseArt, ScanArt};
use super::welcome_flow::{use_welcome_flow, WelcomeStep};

const GLOBE_PATH: &str = "M21.75 12a9.75 9.75 0 1 1-19.5 0 9.75 9.75 0 0 1 19.5 0zM2.5 12h19M12 2.25c2.5 2.6 3.9 6.1 3.9 9.75S14.5 19.15 12 21.75c-2.5-2.6-3.9-6.1-3.9-9.75S9.5 4.85 12 2.25z";
const NO_ADS_PATH: &str = "M18.364 18.364A9 9 0 0 0 5.636 5.636m12.728 12.728A9 9 0 0 1 5.636 5.636m12.728 12.728L5.636 5.636";
const NO_TRACKERS_PATH: &str = "M3.98 8.223A10.477 10.477 0 0 0 1.934 12C3.226 16.338 7.244 19.5 12 19.5c.993 0 1.953-.138 2.863-.395M6.228 6.228A10.451 10.451 0 0 1 12 4.5c4.756 0 8.773 3.162 10.065 7.498a10.522 10.522 0 0 1-4.293 5.774M6.228 6.228 3 3m3.228 3.228 3.65 3.65m7.894 7.894L21 21m-3.228-3.228-3.65-3.65m0 0a3 3 0 1 0-4.243-4.243m4.242 4.242L9.88 9.88";

#[component]
pub fn WelcomePage() -> Element {
    let flow = use_welcome_flow();
    let position = flow.position();
    let total = flow.total();

    rsx! {
        div { class: "min-h-dvh safe-page flex flex-col bg-base-100 pb-6",
            div { class: "flex h-11 items-center gap-4 ps-6 pe-4 mt-3",
                div {
                    class: "flex flex-1 gap-1.5",
                    role: "progressbar",
                    aria_valuemin: "1",
                    aria_valuemax: "{total}",
                    aria_valuenow: "{position}",
                    aria_label: tid!("welcome-step", current: position as i64, total: total as i64),
                    for i in 0..total {
                        span { class: if i < position { "h-1 flex-1 rounded-full bg-primary" } else { "h-1 flex-1 rounded-full bg-primary/20" } }
                    }
                }
                if !flow.is_last() {
                    button {
                        id: "welcome-skip",
                        r#type: "button",
                        class: "btn btn-ghost btn-sm min-h-11 text-base-content/70",
                        onclick: move |_| flow.skip(),
                        {tid!("welcome-skip")}
                    }
                }
            }

            div {
                class: "mx-4 mt-1 flex min-h-56 max-h-[22.5rem] flex-1 overflow-hidden rounded-[1.75rem] bg-primary/10 [@media(max-height:700px)]:min-h-36",
                aria_hidden: "true",
                div { class: "relative flex flex-1 items-center justify-center [@media(max-height:700px)]:scale-[0.65]",
                    match flow.step() {
                        WelcomeStep::Promise => rsx! { PromiseArt {} },
                        WelcomeStep::Link => rsx! { LinkArt {} },
                        WelcomeStep::Private => rsx! { PrivateArt {} },
                        WelcomeStep::Scan => rsx! { ScanArt {} },
                    }
                }
            }

            div { class: "px-6 pt-7 [@media(max-height:700px)]:pt-5",
                match flow.step() {
                    WelcomeStep::Promise => rsx! {
                        StepTitle { {tid!("welcome-title")} }
                        StepBody { {tid!("welcome-subtitle")} }
                        p { class: "mt-4 text-sm font-semibold", {tid!("welcome-note")} }
                    },
                    WelcomeStep::Link => rsx! {
                        StepTitle { {tid!("welcome-link-title")} }
                        StepBody { {tid!("welcome-link-body")} }
                        p { class: "mt-3.5 text-sm text-base-content/70", {tid!("welcome-link-account")} }
                    },
                    WelcomeStep::Private => rsx! {
                        StepTitle { {tid!("welcome-private-title")} }
                        StepBody { {tid!("welcome-private-body")} }
                    },
                    WelcomeStep::Scan => rsx! {
                        StepTitle { {tid!("welcome-scan-title")} }
                        StepBody { {tid!("welcome-scan-body")} }
                    },
                }
            }

            div { class: "mt-auto flex flex-col items-center gap-3.5 px-6 pt-6",
                if flow.is_last() {
                    div { class: "flex flex-wrap justify-center gap-2 pb-0.5",
                        TrustChip { path: GLOBE_PATH, {tid!("welcome-eu-title")} }
                        TrustChip { path: NO_ADS_PATH, {tid!("welcome-no-ads")} }
                        TrustChip { path: NO_TRACKERS_PATH, {tid!("welcome-no-trackers")} }
                    }
                    button {
                        // E2E dismisses onboarding through this. Fixed id, not translated copy:
                        // `welcome-start` is French in the shipping locale and changes freely.
                        id: "welcome-start",
                        r#type: "button",
                        class: "btn btn-primary btn-lg btn-block",
                        onclick: move |_| flow.finish(),
                        {tid!("welcome-start")}
                    }
                    Link {
                        id: "welcome-how-it-works",
                        to: Route::PrivacyPage {},
                        class: "text-sm text-base-content/70 underline underline-offset-3",
                        {tid!("welcome-how-it-works")}
                    }
                } else {
                    button {
                        id: "welcome-next",
                        r#type: "button",
                        class: "btn btn-primary btn-lg btn-block",
                        onclick: move |_| flow.next(),
                        {tid!("welcome-next")}
                    }
                }
            }
        }
    }
}

#[component]
fn StepTitle(children: Element) -> Element {
    rsx! {
        h1 { class: "font-display text-3xl font-extrabold leading-[1.12] tracking-tight [@media(max-height:700px)]:text-[1.625rem]", {children} }
    }
}

#[component]
fn StepBody(children: Element) -> Element {
    rsx! {
        p { class: "mt-3 text-lg leading-normal text-base-content/70 [@media(max-height:700px)]:text-base", {children} }
    }
}

#[component]
fn TrustChip(path: &'static str, children: Element) -> Element {
    rsx! {
        span { class: "inline-flex items-center gap-1.5 rounded-full bg-base-200 px-3 py-1.5 text-xs font-semibold",
            Icon { path, class: "size-3.5 text-primary" }
            {children}
        }
    }
}
