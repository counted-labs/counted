use dioxus::prelude::*;

use crate::icons::ChevronRightIcon;
use crate::route::Route;

const ROW: &str = "flex items-center gap-3 w-full min-h-11 px-4 py-2.5 text-left";

/// One row of a `ul.list-group` (native-feel.css). With `to` it navigates and ends on a chevron;
/// with `onclick` it is a button, and `long_press` names the hidden button boot.js clicks on a long
/// press, which runs the same action with a Medium haptic (the mark sits on the button itself, as
/// boot.js ignores a press on a control inside the marked element); with neither it is a plain row
/// that may hold its own controls in `trailing`, so no interactive element ever nests inside another.
/// `children` is the leading tile or avatar.
#[component]
pub fn ListRow(
    #[props(into, default)] id: Option<String>,
    title: String,
    #[props(default)] subtitle: Option<String>,
    #[props(default)] subtitle_class: &'static str,
    #[props(default)] subtitle_sr: Option<String>,
    #[props(default)] title_class: &'static str,
    #[props(default)] trailing: Option<Element>,
    #[props(default)] to: Option<Route>,
    #[props(default)] onclick: Option<EventHandler<()>>,
    #[props(default)] chevron: bool,
    #[props(default)] centred: bool,
    #[props(into, default)] long_press: Option<String>,
    #[props(default)] disabled: bool,
    #[props(into, default)] class: String,
    children: Element,
) -> Element {
    let navigates = to.is_some() || chevron;
    let align = if centred { "justify-center text-center" } else { "" };
    let body = rsx! {
        {children}
        span { class: "flex-1 min-w-0 flex flex-col {align}",
            span { class: "break-words {title_class}", "{title}" }
            if let Some(subtitle) = subtitle {
                span { class: "text-sm break-words {subtitle_class}", aria_hidden: subtitle_sr.is_some().then_some("true"), "{subtitle}" }
            }
            if let Some(subtitle_sr) = subtitle_sr {
                span { class: "sr-only", "{subtitle_sr}" }
            }
        }
        {trailing.unwrap_or_else(|| rsx! {})}
        if navigates {
            span { class: "list-chevron", aria_hidden: "true", ChevronRightIcon {} }
        }
    };

    match (to, onclick) {
        (Some(to), _) => rsx! {
            li { class,
                Link { id, to, class: "{ROW} row-press", {body} }
            }
        },
        (None, Some(onclick)) => {
            let hidden_id = long_press.clone();
            rsx! {
                li { class,
                    button {
                        id,
                        "data-long-press": long_press,
                        r#type: "button",
                        class: "{ROW} row-press disabled:opacity-50",
                        disabled,
                        onclick: move |_| {
                            crate::common::haptic(crate::common::Haptic::Light);
                            onclick.call(());
                        },
                        {body}
                    }
                    if let Some(hidden_id) = hidden_id {
                        button {
                            id: hidden_id,
                            r#type: "button",
                            hidden: true,
                            tabindex: "-1",
                            onclick: move |_| {
                                crate::common::haptic(crate::common::Haptic::Medium);
                                onclick.call(());
                            },
                        }
                    }
                }
            }
        }
        (None, None) => rsx! {
            li { class, id,
                div { class: "{ROW}", {body} }
            }
        },
    }
}

/// The square tile a settings row leads with: an icon on one of the avatar tints.
#[component]
pub fn RowTile(tint: &'static str, children: Element) -> Element {
    rsx! {
        span { class: "size-8 rounded-lg grid place-items-center shrink-0 {tint}", aria_hidden: "true", {children} }
    }
}

/// The footnote under a group, outside it, as an iOS table footer.
#[component]
pub fn ListFooter(#[props(into, default)] id: Option<String>, children: Element) -> Element {
    rsx! {
        p { id, class: "px-4 pt-1.5 text-xs text-base-content/70", {children} }
    }
}
