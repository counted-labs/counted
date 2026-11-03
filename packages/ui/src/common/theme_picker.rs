use dioxus::prelude::*;

use super::theme::{choose_theme, use_theme_choice, Theme};
use crate::icons::{DeviceIcon, MoonIcon, SunIcon, ICON_HEADER};
use crate::tid;

#[component]
pub fn ThemePicker() -> Element {
    let choice = use_theme_choice();

    rsx! {
        div { class: "flex flex-col gap-2",
            span { id: "theme-label", class: "label", {tid!("theme")} }
            div {
                role: "radiogroup",
                aria_labelledby: "theme-label",
                class: "tabs tabs-box bg-base-300 shadow-none grid grid-flow-col auto-cols-fr",
                for theme in Theme::ALL {
                    label {
                        class: if choice() == theme {
                            "tab tab-active h-16 min-w-0 flex-col gap-1 text-sm font-semibold"
                        } else {
                            "tab h-16 min-w-0 flex-col gap-1 text-sm text-base-content/70"
                        },
                        input {
                            id: "theme-{theme.as_str()}",
                            r#type: "radio",
                            name: "theme",
                            checked: choice() == theme,
                            oninput: move |_| choose_theme(choice, theme),
                        }
                        match theme {
                            Theme::Light => rsx! { SunIcon { size: ICON_HEADER } },
                            Theme::Dark => rsx! { MoonIcon { size: ICON_HEADER } },
                            Theme::System => rsx! { DeviceIcon { size: ICON_HEADER } },
                        }
                        {tid!(theme.label_key())}
                    }
                }
            }
            p { class: "label",
                if choice() == Theme::System {
                    {tid!("theme-hint-system")}
                } else {
                    {tid!("theme-hint-device")}
                }
            }
        }
    }
}
