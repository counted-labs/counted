use dioxus::prelude::*;
use crate::tid;

use crate::common::{update_ls, LocalStorageState, Mascot, MascotPose, MASCOT_HERO};
use crate::route::Route;

#[component]
pub fn WelcomePage() -> Element {
    let ls_ctx = use_context::<Signal<LocalStorageState>>();

    let on_start = move |_| {
        update_ls(ls_ctx, |state| state.onboarding_seen = true);
    };

    rsx! {
        div { class: "min-h-dvh safe-page flex flex-col items-center justify-center p-6 bg-base-100",
            div { class: "flex flex-col items-center gap-8 max-w-sm w-full",

                // Header
                div { class: "flex flex-col items-center gap-3 text-center",
                    Mascot { pose: MascotPose::Secure, size: MASCOT_HERO }
                    h1 { class: "text-xl font-bold", {tid!("welcome-title")} }
                    p { class: "text-sm text-base-content/70", {tid!("welcome-subtitle")} }
                }

                // Feature cards
                div { class: "flex flex-col gap-3 w-full",

                    // E2EE + zero access
                    div { class: "bg-base-200 rounded-2xl p-4 flex gap-3 items-start",
                        div { class: "w-9 h-9 rounded-xl bg-info/15 flex items-center justify-center shrink-0 mt-0.5",
                            svg {
                                class: "w-5 h-5 text-info",
                                xmlns: "http://www.w3.org/2000/svg",
                                "aria-hidden": "true",
                                view_box: "0 0 24 24",
                                fill: "none",
                                stroke: "currentColor",
                                stroke_width: "1.5",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                path { d: "M3.98 8.223A10.477 10.477 0 0 0 1.934 12C3.226 16.338 7.244 19.5 12 19.5c.993 0 1.953-.138 2.863-.395M6.228 6.228A10.451 10.451 0 0 1 12 4.5c4.756 0 8.773 3.162 10.065 7.498a10.522 10.522 0 0 1-4.293 5.774M6.228 6.228 3 3m3.228 3.228 3.65 3.65m7.894 7.894L21 21m-3.228-3.228-3.65-3.65m0 0a3 3 0 1 0-4.243-4.243m4.242 4.242L9.88 9.88" }
                            }
                        }
                        div {
                            p { class: "font-semibold text-sm", {tid!("welcome-e2ee-title")} }
                            p { class: "text-xs text-base-content/70 mt-1",
                                {tid!("welcome-e2ee-body")}
                            }
                            p { class: "text-xs text-base-content/70 mt-1",
                                {tid!("welcome-e2ee-note")}
                            }
                        }
                    }

                    // European stack
                    div { class: "bg-base-200 rounded-2xl p-4 flex gap-3 items-start",
                        div { class: "w-9 h-9 rounded-xl bg-primary/15 flex items-center justify-center shrink-0 mt-0.5",
                            svg {
                                class: "w-5 h-5 text-primary",
                                xmlns: "http://www.w3.org/2000/svg",
                                "aria-hidden": "true",
                                view_box: "0 0 24 24",
                                fill: "none",
                                stroke: "currentColor",
                                stroke_width: "1.5",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                path { d: "M21.75 12a9.75 9.75 0 1 1-19.5 0 9.75 9.75 0 0 1 19.5 0zM2.5 12h19M12 2.25c2.5 2.6 3.9 6.1 3.9 9.75S14.5 19.15 12 21.75c-2.5-2.6-3.9-6.1-3.9-9.75S9.5 4.85 12 2.25z" }
                            }
                        }
                        div {
                            p { class: "font-semibold text-sm", {tid!("welcome-eu-title")} }
                            p { class: "text-xs text-base-content/70 mt-1",
                                {tid!("welcome-eu-body")}
                            }
                        }
                    }

                    // Privacy notice
                    div { class: "bg-base-200 rounded-2xl p-4 flex gap-3 items-start",
                        div { class: "w-9 h-9 rounded-xl bg-warning/15 flex items-center justify-center shrink-0 mt-0.5",
                            svg {
                                class: "w-5 h-5 text-warning",
                                xmlns: "http://www.w3.org/2000/svg",
                                "aria-hidden": "true",
                                view_box: "0 0 24 24",
                                fill: "none",
                                stroke: "currentColor",
                                stroke_width: "1.5",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                path { d: "M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z" }
                            }
                        }
                        div {
                            p { class: "font-semibold text-sm", {tid!("welcome-noads-title")} }
                            p { class: "text-xs text-base-content/70 mt-1",
                                {tid!("welcome-noads-body")}
                            }
                        }
                    }
                }

                div { class: "flex flex-col items-center gap-3 w-full",
                    button {
                        // E2E dismisses onboarding through this. Fixed id, not translated copy:
                        // `welcome-start` is French in the shipping locale and changes freely.
                        id: "welcome-start",
                        r#type: "button",
                        class: "btn btn-primary btn-block mt-2",
                        onclick: on_start,
                        {tid!("welcome-start")}
                    }
                    Link {
                        id: "welcome-how-it-works",
                        to: Route::PrivacyPage {},
                        class: "text-xs text-base-content/70 underline",
                        {tid!("welcome-how-it-works")}
                    }
                }
            }
        }
    }
}
