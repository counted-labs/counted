use dioxus::prelude::*;

#[derive(PartialEq, Props, Clone)]
pub struct CalloutComponentProps {
    callout_type: CalloutComponentTypes,
    error_message: String,
}

#[derive(PartialEq, Clone)]
pub enum CalloutComponentTypes {
    Success,
    Warning,
    Info,
    Error,
}

#[component]
pub fn CalloutComponent(props: CalloutComponentProps) -> Element {
    let callout_type = match props.callout_type {
        CalloutComponentTypes::Success => "success",
        CalloutComponentTypes::Warning => "warning",
        CalloutComponentTypes::Info => "info",
        CalloutComponentTypes::Error => "error",
    };

    rsx! {
        div { role: "alert", class: "alert alert-{callout_type} p-3 mt-2 mb-2",
            svg {
                xmlns: "http://www.w3.org/2000/svg",
                "aria-hidden": "true",
                class: "h-6 w-6 shrink-0 stroke-current",
                fill: "none",
                view_box: "0 0 24 24",
                path {
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                    stroke_width: "2",
                    d: "M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z",
                }
            }
            span { "{props.error_message}" }
        }
    }
}
