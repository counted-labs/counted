use dioxus::prelude::*;
use crate::tid;
use shared::{PaymentMethod, ProjectStatus, ReimbursementSuggestion, User};

use crate::{
    common::{
        copy_text, initials, user_color_class, Avatar, CheckMarkIllustration, Flash, ProjectKey,
    },
    crypto::{decrypt_user, DecryptedUser},
    icons::RightArrowIcon,
    payment_methods::{method_display_name, method_kind_name},
};

#[derive(PartialEq, Props, Clone)]
pub struct ReimbursementsTabProps {
    pub suggestions: Vec<ReimbursementSuggestion>,
    pub users: Vec<User>,
    pub currency: String,
    pub project_status: ProjectStatus,
    /// The participant this device is. `None` renders every row the same, with no way to pay.
    pub stored_user_id: Option<i32>,
    pub on_reimburse: EventHandler<ReimbursementSuggestion>,
}

#[component]
pub fn ReimbursementsTab(props: ReimbursementsTabProps) -> Element {
    let key_ctx = use_context::<ProjectKey>().0;

    if props.suggestions.is_empty() {
        return rsx! {
            div { class: "flex flex-col items-center gap-2 py-12 text-base-content/70",
                CheckMarkIllustration {}
                span { class: "font-bold text-base-content", {tid!("reimbursements-empty-title")} }
                span { class: "text-sm text-center",
                    {tid!("reimbursements-empty-hint")}
                }
            }
        };
    }

    let is_mine = |s: &ReimbursementSuggestion| Some(s.user_id_debtor) == props.stored_user_id;
    let mine: Vec<ReimbursementSuggestion> =
        props.suggestions.iter().filter(|s| is_mine(s)).cloned().collect();
    let others: Vec<ReimbursementSuggestion> =
        props.suggestions.iter().filter(|s| !is_mine(s)).cloned().collect();
    let editable = props.project_status != ProjectStatus::Archived;
    let key = key_ctx();

    let row = |suggestion: &ReimbursementSuggestion, highlighted: bool| {
        let find = |id: i32| props.users.iter().find(|u| u.id == id);
        // "?" covers all three misses: unknown user, no key, undecryptable payload.
        let decrypted = |u: Option<&User>| -> Option<DecryptedUser> {
            u.zip(key).and_then(|(u, k)| decrypt_user(&k, u).ok())
        };
        let debtor = decrypted(find(suggestion.user_id_debtor));
        let payer = decrypted(find(suggestion.user_id_payer));
        let name_or_unknown = |u: &Option<DecryptedUser>| {
            u.as_ref().map(|d| d.name.clone()).unwrap_or_else(|| "?".to_string())
        };
        let debtor_name = name_or_unknown(&debtor);
        let payer_name = name_or_unknown(&payer);
        let debtor_initials = initials(&debtor_name);
        let payer_initials = initials(&payer_name);
        let debtor_color = debtor.as_ref().map(|u| user_color_class(u.id)).unwrap_or("bg-neutral");
        let payer_color = payer.as_ref().map(|u| user_color_class(u.id)).unwrap_or("bg-neutral");
        let amount = format!("{:.2} {}", suggestion.amount, props.currency);
        let suggestion_clone = suggestion.clone();
        let payer_methods = payer.as_ref().map(|p| p.payment_methods.clone()).unwrap_or_default();
        // The claimant's own name, not the participant's: the methods are exactly as
        // trustworthy as the claim, and this is what lets the debtor notice a stranger
        // behind a friend's participant.
        let shared_by =
            payer.as_ref().and_then(|p| p.claim_name.clone()).unwrap_or_else(|| payer_name.clone());
        let card_class = if highlighted {
            "bg-primary/5 border border-primary/30 rounded-box shadow-soft p-4 flex items-center gap-3"
        } else {
            "bg-base-100 rounded-box shadow-soft p-4 flex items-center gap-3"
        };
        rsx! {
            li { class: "{card_class}",
                div { class: "flex items-center gap-1 shrink-0",
                    Avatar { initials: debtor_initials, color_class: debtor_color.to_string() }
                    RightArrowIcon {}
                    Avatar { initials: payer_initials, color_class: payer_color.to_string() }
                }

                div { class: "flex-1 min-w-0",
                    p { class: "text-sm font-medium truncate", {tid!("reimbursement-owes", debtor: debtor_name.clone(), creditor: payer_name.clone())} }
                    p { class: "text-xs font-semibold text-base-content/70 uppercase", "{amount}" }
                }
                if editable && highlighted && !payer_methods.is_empty() {
                    PayButton {
                        id: format!("reimbursement-pay-{}-{}", suggestion.user_id_debtor, suggestion.user_id_payer),
                        creditor_name: payer_name.clone(),
                        shared_by,
                        amount: amount.clone(),
                        methods: payer_methods,
                    }
                }
                if editable {
                    button {
                        r#type: "button",
                        class: "btn btn-sm btn-primary",
                        title: tid!("reimbursement-add"),
                        onclick: move |_| props.on_reimburse.call(suggestion_clone.clone()),
                        {tid!("reimbursement-record")}
                    }
                }
            }
        }
    };

    rsx! {
        div { class: "flex flex-col gap-4",
            if !mine.is_empty() {
                section { class: "flex flex-col gap-2",
                    h3 { class: "text-xs font-semibold uppercase tracking-wide text-primary px-1",
                        {tid!("reimbursements-mine-title")}
                    }
                    ul { class: "flex flex-col gap-3",
                        for suggestion in mine.iter() {
                            {row(suggestion, true)}
                        }
                    }
                }
            }
            if !others.is_empty() {
                section { class: "flex flex-col gap-2",
                    if !mine.is_empty() {
                        h3 { class: "text-xs font-semibold uppercase tracking-wide text-base-content/60 px-1",
                            {tid!("reimbursements-others-title")}
                        }
                    }
                    ul { class: "flex flex-col gap-3",
                        for suggestion in others.iter() {
                            {row(suggestion, false)}
                        }
                    }
                }
            }
        }
    }
}

#[derive(PartialEq, Props, Clone)]
struct PayButtonProps {
    id: String,
    creditor_name: String,
    shared_by: String,
    amount: String,
    methods: Vec<PaymentMethod>,
}

/// Opens the creditor's shared payment methods in a modal, each with a copy button. Copy only:
/// no wallet exposes a link a third-party app may build (see docs/research/payment-handoff.md),
/// so the debtor pays in their own app and comes back to record the transfer. A modal rather
/// than a dropdown because the trigger sits mid-row on a phone, where an anchored menu has no
/// side to open on.
#[component]
fn PayButton(props: PayButtonProps) -> Element {
    let mut open = use_signal(|| false);
    let mut flash = use_context::<Signal<Option<Flash>>>();

    rsx! {
        button {
            id: props.id.clone(),
            r#type: "button",
            class: "btn btn-sm btn-soft btn-primary",
            onclick: move |_| open.set(true),
            {tid!("reimbursement-pay-with")}
        }
        if open() {
            div { class: "modal modal-open modal-bottom sm:modal-middle", role: "dialog",
                div { class: "modal-box max-w-sm p-0 flex flex-col",
                    div { class: "px-6 pt-5 pb-4 border-b border-base-200",
                        h3 { class: "font-bold text-lg font-display",
                            {tid!("reimbursement-pay-title", name: props.creditor_name.clone())}
                        }
                        p { class: "text-2xl font-bold text-primary", "{props.amount}" }
                    }
                    div { class: "px-6 py-4 flex flex-col gap-3",
                        div { role: "alert", class: "alert alert-warning alert-soft text-sm",
                            {tid!("reimbursement-pay-shared-by", name: props.shared_by.clone())}
                        }
                        ul { class: "list bg-base-200 rounded-box",
                            for method in props.methods.iter() {
                                {
                                    let value = method.value.clone();
                                    let name = method_display_name(method);
                                    let kind = method_kind_name(method);
                                    let label = method.label.clone().filter(|l| kind.is_some() && !l.trim().is_empty());
                                    rsx! {
                                        li { class: "list-row items-center",
                                            div { class: "list-col-grow min-w-0 flex flex-col gap-0.5",
                                                span { class: "text-xs font-semibold text-base-content/70",
                                                    {kind.unwrap_or_else(|| name.clone())}
                                                    if let Some(label) = label {
                                                        span { class: "font-normal", " · {label}" }
                                                    }
                                                }
                                                span { class: "font-mono text-sm break-all select-all", "{method.value}" }
                                            }
                                            button {
                                                r#type: "button",
                                                class: "btn btn-sm btn-ghost",
                                                aria_label: tid!("payment-method-copy", name: name.clone()),
                                                onclick: move |_| {
                                                    let value = value.clone();
                                                    spawn(async move {
                                                        if copy_text(&value).await {
                                                            flash.set(Some(Flash::ok(tid!("payment-method-copied"))));
                                                        } else {
                                                            flash.set(Some(Flash::err(tid!("payment-method-copy-failed"))));
                                                        }
                                                    });
                                                },
                                                {tid!("copy")}
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    div { class: "flex justify-end px-6 py-4 border-t border-base-200",
                        button {
                            r#type: "button",
                            class: "btn",
                            onclick: move |_| open.set(false),
                            {tid!("close")}
                        }
                    }
                }
                div { class: "modal-backdrop", onclick: move |_| open.set(false) }
            }
        }
    }
}
