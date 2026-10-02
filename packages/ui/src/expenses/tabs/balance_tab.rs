use dioxus::prelude::*;
use shared::{User, UserSummary};

use crate::common::{initials, user_color_class, Avatar, Mascot, MascotPose, ProjectKey, SizeClass};
use crate::decrypted::user_name_opt;
use crate::expenses::helpers::balance_groups::{balance_groups, BalanceRow};
use crate::icons::{CheckIcon, ICON_INLINE};
use crate::tid;

#[derive(PartialEq, Props, Clone)]
pub struct BalanceTabProps {
    pub summary: UserSummary,
    pub users: Vec<User>,
    pub currency: String,
    pub stored_user_id: Option<i32>,
}

#[component]
pub fn BalanceTab(props: BalanceTabProps) -> Element {
    let key = use_context::<ProjectKey>().0();
    let groups = balance_groups(&props.users, &props.summary.summary);
    let name_of = |user: &User| user_name_opt(key.as_ref(), user);
    let settled_names = groups.settled.iter().map(name_of).collect::<Vec<_>>().join(", ");

    let group = |label: String, total: f64, rows: &[BalanceRow], positive: bool| {
        let tone = if positive { "text-success" } else { "text-error" };
        let bar = if positive { "bg-success" } else { "bg-error" };
        let total = format!("{total:.2} {}", props.currency);
        rsx! {
            section { class: "flex flex-col gap-2",
                h3 { class: "flex items-baseline justify-between px-1 text-xs font-semibold uppercase tracking-wide text-base-content/70",
                    span { "{label}" }
                    span { class: "normal-case tracking-normal tabular-nums {tone}", "{total}" }
                }
                ul { class: "bg-base-100 rounded-box shadow-soft overflow-hidden divide-y divide-base-200",
                    for row in rows.iter() {
                        {
                            let name = name_of(&row.user);
                            let is_me = Some(row.user.id) == props.stored_user_id;
                            let sign = if positive { "+" } else { "" };
                            let amount = format!("{sign}{:.2} {}", row.balance, props.currency);
                            rsx! {
                                li { class: if is_me { "flex items-center gap-3 px-4 py-2.5 bg-primary/5" } else { "flex items-center gap-3 px-4 py-2.5" },
                                    Avatar {
                                        initials: initials(&name),
                                        color_class: user_color_class(row.user.id).to_string(),
                                    }
                                    div { class: "flex-1 min-w-0 flex flex-col gap-1.5",
                                        div { class: "flex items-center gap-1.5 min-w-0",
                                            span { class: "text-sm truncate", "{name}" }
                                            if is_me {
                                                span { class: "badge badge-soft badge-primary badge-xs font-semibold shrink-0",
                                                    {tid!("participants-you-badge")}
                                                }
                                            }
                                        }
                                        div { class: "h-1 rounded-full bg-base-200", aria_hidden: "true",
                                            div {
                                                class: "h-full min-w-1 rounded-full {bar}",
                                                style: "width: {row.bar_percent:.1}%",
                                            }
                                        }
                                    }
                                    span { class: "text-sm font-semibold tabular-nums whitespace-nowrap {tone}", "{amount}" }
                                }
                            }
                        }
                    }
                }
            }
        }
    };

    rsx! {
        div { class: "flex flex-col gap-4",
            if groups.all_settled() {
                div { class: "flex flex-col items-center gap-2 pt-8 pb-2 text-base-content/70",
                    Mascot { pose: MascotPose::Settled }
                    span { class: "font-bold text-base-content", {tid!("reimbursements-empty-title")} }
                }
            }
            if !groups.gets_back.is_empty() {
                {group(tid!("balance-gets-back"), groups.gets_back_total, &groups.gets_back, true)}
            }
            if !groups.owes.is_empty() {
                {group(tid!("balance-owes"), groups.owes_total, &groups.owes, false)}
            }
            if !groups.settled.is_empty() {
                section { class: "flex flex-col gap-2",
                    h3 { class: "px-1 text-xs font-semibold uppercase tracking-wide text-base-content/70",
                        {tid!("balance-settled")}
                    }
                    div { class: "flex items-center gap-3 px-4 py-2.5 bg-base-100 rounded-box shadow-soft",
                        div { class: "avatar-group -space-x-2 shrink-0",
                            for user in groups.settled.iter().take(5) {
                                Avatar {
                                    initials: initials(&name_of(user)),
                                    color_class: user_color_class(user.id).to_string(),
                                    size: SizeClass::W6,
                                }
                            }
                        }
                        span { class: "flex-1 min-w-0 truncate text-sm text-base-content/70", "{settled_names}" }
                        span { class: "text-primary shrink-0",
                            CheckIcon { size: ICON_INLINE }
                        }
                    }
                }
            }
        }
    }
}
