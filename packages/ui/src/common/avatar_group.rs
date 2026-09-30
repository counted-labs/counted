use dioxus::prelude::*;
use shared::User;

use crate::common::avatar::SizeClass;
use crate::common::{initials, user_color_class, Avatar};
use crate::decrypted::user_name_opt;

#[derive(PartialEq, Props, Clone)]
pub struct AvatarGroupProps {
    users: Vec<User>,
    #[props(default)]
    encryption_key: Option<[u8; 32]>,
    #[props(default = SizeClass::W8)]
    size: SizeClass,
}

#[component]
pub fn AvatarGroup(props: AvatarGroupProps) -> Element {
    const MAX: usize = 4;
    let shown: Vec<&User> = props.users.iter().take(MAX).collect();
    let overflow = props.users.len().saturating_sub(MAX);
    let size = props.size;

    rsx! {
        div { class: "avatar-group -space-x-3",
            for user in shown {
                {
                    let name = user_name_opt(props.encryption_key.as_ref(), user);
                    rsx! {
                        Avatar {
                            initials: initials(&name),
                            color_class: user_color_class(user.id).to_string(),
                            size,
                        }
                    }
                }
            }
            if overflow > 0 {
                div { class: "avatar avatar-placeholder",
                    div { class: "w-8 bg-base-200 text-base-content/70 rounded-full ring-2 ring-base-100",
                        span { class: "text-xs font-medium", "+{overflow}" }
                    }
                }
            }
        }
    }
}
