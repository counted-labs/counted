use dioxus::prelude::*;

#[derive(PartialEq, Clone, Copy)]
pub enum SizeClass {
    W6,
    W8,
    W10,
    W12,
}

impl SizeClass {
    pub fn as_class(&self) -> &'static str {
        match self {
            SizeClass::W6 => "w-6",
            SizeClass::W8 => "w-8",
            SizeClass::W10 => "w-10",
            SizeClass::W12 => "w-12",
        }
    }
}

#[derive(PartialEq, Props, Clone)]
pub struct AvatarProps {
    initials: String,
    color_class: Option<String>,
    #[props(default = SizeClass::W8)]
    size: SizeClass,
}

#[component]
pub fn Avatar(props: AvatarProps) -> Element {
    let color = props.color_class.as_deref().unwrap_or("bg-primary-content");
    let size = props.size.as_class();

    rsx! {
        // Decorative: initials (or the expense emoji) duplicate a name that is always rendered
        // next to it, and read as letter salad otherwise.
        div { class: "avatar avatar-placeholder", aria_hidden: "true",
            div { class: "{color} {size} rounded-full ring-2 ring-base-100",
                span { class: "text-xs", "{props.initials}" }
            }
        }
    }
}
