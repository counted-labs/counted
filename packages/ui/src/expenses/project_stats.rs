use dioxus::prelude::*;
use crate::tid;

#[derive(PartialEq, Props, Clone)]
pub struct ProjectStatsProps {
    pub global_total: f64,
    /// `None` until this device has picked which participant it is - the tile is hidden and the
    /// grid collapses to one column.
    pub user_total: Option<f64>,
    pub currency: String,
}

#[component]
pub fn ProjectStats(props: ProjectStatsProps) -> Element {
    let currency = props.currency;
    rsx! {
        div {
            class: "stats bg-base-100 shadow-soft w-full flex-shrink-0",
            style: if props.user_total.is_some() { "grid-template-columns: 1fr 1fr;" } else { "grid-template-columns: 1fr;" },
            div { class: "stat",
                div { class: "stat-title text-xs", {tid!("stats-total-expenses")} }
                div { class: "stat-value text-xl font-display text-gradient-brand", {crate::common::number(props.global_total)} }
                div { class: "stat-desc", "{currency}" }
            }
            if let Some(user_total) = props.user_total {
                div { class: "stat",
                    div { class: "stat-title text-xs", {tid!("stats-my-expenses")} }
                    div { class: "stat-value text-xl font-display text-primary", {crate::common::number(user_total)} }
                    div { class: "stat-desc", "{currency}" }
                }
            }
        }
    }
}
