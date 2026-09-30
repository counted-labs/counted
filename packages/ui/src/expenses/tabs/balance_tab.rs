use dioxus::prelude::*;
use shared::{User, UserSummary};

use crate::common::{initials, user_color_class, Avatar, ProjectKey};
use crate::decrypted::user_name_opt;

#[derive(PartialEq, Props, Clone)]
pub struct BalanceTabProps {
    pub summary: UserSummary,
    pub users: Vec<User>,
    pub currency: String,
}

#[component]
pub fn BalanceTab(props: BalanceTabProps) -> Element {
    let key_ctx = use_context::<ProjectKey>().0;

    let mut entries: Vec<(User, f64)> = props
        .users
        .iter()
        .map(|u| {
            let balance = props.summary.summary.get(&u.id).copied().unwrap_or(0.0);
            (u.clone(), balance)
        })
        .collect();

    // Sort ascending (most negative first)
    entries.sort_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

    let max_abs = entries.iter().map(|(_, b)| b.abs()).fold(1.0_f64, f64::max);

    let currency = props.currency.clone();

    rsx! {
        ul { class: "flex flex-col gap-2",
            for (user , balance) in entries {
                {
                    let width = ((balance.abs() / max_abs) * 100.0).max(2.0);
                    let (amount_class, bar_class, bar_position) = if balance > 0.0 {
                        ("text-success", "progress-success", "")
                    } else if balance < 0.0 {
                        ("text-error", "progress-error", "text-right")
                    } else {
                        ("", "", "")
                    };
                    let sign = if balance > 0.0 { "+" } else { "" };
                    let curr = currency.clone();
                    rsx! {
                        li {
                            class: "grid items-center gap-1 shadow-soft p-2 rounded-box bg-base-100",
                            style: "grid-template-columns: min-content 1fr 1fr 1fr; min-height: 56px",
                            {
                                let name = user_name_opt(key_ctx().as_ref(), &user);
                                rsx! {
                                    Avatar {
                                        initials: initials(&name),
                                        color_class: user_color_class(user.id).to_string(),
                                    }
                                    span { class: "self-center text-sm ml-1 text-left truncate", "{name}" }
                                }
                            }
                            div { class: "self-center w-20 text-left ml-1",
                                span { class: "text-sm {amount_class}", "{sign}{balance:.2} {curr}" }
                            }
                            div { class: "flex",
                                if balance > 0.0 {
                                    div { class: "flex-1" }
                                }
                                div { class: "flex-1 {bar_position}",
                                    // Decorative. The magnitude lives in the inline width, not in
                                    // value/max, so AT would read "100%" for every participant —
                                    // and the real amount is already in the span above.
                                    progress {
                                        class: "progress {bar_class}",
                                        style: "width: {width:.0}%",
                                        value: "100",
                                        max: "100",
                                        aria_hidden: "true",
                                    }
                                }
                                if balance <= 0.0 {
                                    div { class: "flex-1" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
