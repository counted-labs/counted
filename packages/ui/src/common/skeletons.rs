//! Placeholders shaped like the content they stand in for, so the page does not jump when it lands.
//! Each says "loading" once to a screen reader; the shapes themselves are hidden from it.

use dioxus::prelude::*;

use crate::tid;

#[component]
fn Loading(children: Element) -> Element {
    rsx! {
        div { role: "status", aria_label: tid!("loading"), class: "flex flex-col gap-3",
            div { class: "flex flex-col gap-3", aria_hidden: "true", {children} }
        }
    }
}

/// The expense page: the summary card, then the paid-by and for cards with their rows.
#[component]
pub fn ExpenseDetailSkeleton() -> Element {
    rsx! {
        Loading {
            div { class: "card bg-base-100 shadow-soft",
                div { class: "card-body p-4 gap-3",
                    div { class: "skeleton h-4 w-3/4" }
                    div { class: "flex items-center gap-3",
                        div { class: "skeleton size-11 shrink-0 rounded-2xl" }
                        div { class: "flex-1 flex flex-col gap-2",
                            div { class: "skeleton h-6 w-24" }
                            div { class: "skeleton h-3 w-1/2" }
                        }
                    }
                }
            }
            for rows in [1u8, 2] {
                div { class: "card bg-base-100 shadow-soft",
                    div { class: "card-body p-4 gap-3",
                        div { class: "skeleton h-3 w-16" }
                        for _ in 0..rows {
                            div { class: "flex items-center gap-3",
                                div { class: "skeleton h-10 w-10 rounded-full shrink-0" }
                                div { class: "skeleton h-4 flex-1" }
                                div { class: "skeleton h-4 w-16 shrink-0" }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// The recurring page: the totals strip, then rule rows.
#[component]
pub fn RuleListSkeleton() -> Element {
    rsx! {
        Loading {
            div { class: "skeleton h-16 w-full rounded-box" }
            for _ in 0..3u8 {
                div { class: "flex items-start gap-3 p-3 rounded-box shadow-soft bg-base-100",
                    div { class: "skeleton w-9 h-9 shrink-0 rounded-full" }
                    div { class: "flex-1 flex flex-col gap-2",
                        div { class: "flex justify-between gap-2",
                            div { class: "skeleton h-4 w-1/2" }
                            div { class: "skeleton h-4 w-16" }
                        }
                        div { class: "skeleton h-3 w-3/4" }
                    }
                }
            }
        }
    }
}

/// The charts page: the stat tiles, then a chart card.
#[component]
pub fn ChartsSkeleton() -> Element {
    rsx! {
        Loading {
            div { class: "grid grid-cols-2 gap-3",
                for _ in 0..2u8 {
                    div { class: "card bg-base-100 shadow-soft",
                        div { class: "card-body p-4 gap-2",
                            div { class: "skeleton h-3 w-16" }
                            div { class: "skeleton h-6 w-24" }
                        }
                    }
                }
            }
            div { class: "card bg-base-100 shadow-soft",
                div { class: "card-body p-4 gap-3",
                    div { class: "skeleton h-4 w-1/2" }
                    div { class: "skeleton h-48 w-full" }
                }
            }
        }
    }
}

/// The settings account card: a list of label and value rows.
#[component]
pub fn AccountCardSkeleton() -> Element {
    rsx! {
        Loading {
            div { class: "card bg-base-100 shadow-soft",
                div { class: "card-body gap-3",
                    for _ in 0..3u8 {
                        div { class: "flex flex-col gap-2 py-1",
                            div { class: "skeleton h-3 w-16" }
                            div { class: "skeleton h-4 w-3/4" }
                        }
                    }
                }
            }
        }
    }
}

/// A grouped list of `rows` rows, each a round avatar and two lines: Friends and payment methods.
#[component]
pub fn GroupedListSkeleton(rows: u8) -> Element {
    rsx! {
        Loading {
            div { class: "list-group",
                for _ in 0..rows {
                    div { class: "flex items-center gap-3 px-4 py-3",
                        div { class: "skeleton size-10 rounded-full shrink-0" }
                        div { class: "flex-1 flex flex-col gap-2",
                            div { class: "skeleton h-4 w-3/4" }
                            div { class: "skeleton h-3 w-1/2" }
                        }
                    }
                }
            }
        }
    }
}
