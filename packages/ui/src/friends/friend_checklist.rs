use dioxus::prelude::*;
use crate::tid;
use shared::Friend;
use uuid::Uuid;

#[derive(PartialEq, Props, Clone)]
pub struct FriendChecklistProps {
    pub friends: Vec<Friend>,
    pub selected: Signal<Vec<Uuid>>,
    pub disabled: bool,
}

/// One checkbox per friend. A friend with no public key cannot be sent a key, so their row is
/// shown disabled with "Not ready yet".
#[component]
pub fn FriendChecklist(props: FriendChecklistProps) -> Element {
    let mut selected = props.selected;
    let mut toggle = move |id: Uuid| {
        let mut list = selected.write();
        match list.iter().position(|x| *x == id) {
            Some(i) => {
                list.remove(i);
            }
            None => list.push(id),
        }
    };

    rsx! {
        ul { class: "list",
            for f in props.friends.iter() {
                {
                    let id = f.account_id;
                    let has_key = f.public_key.is_some();
                    rsx! {
                        li { class: "list-row items-center",
                            label { class: "flex items-center gap-3 cursor-pointer min-w-0",
                                input {
                                    r#type: "checkbox",
                                    class: "checkbox checkbox-primary checkbox-sm",
                                    disabled: !has_key || props.disabled,
                                    checked: selected().contains(&id),
                                    onchange: move |_| toggle(id),
                                }
                                span { class: "truncate", "{f.email}" }
                            }
                            if !has_key {
                                span { class: "badge badge-soft badge-warning badge-xs", {tid!("friends-no-key")} }
                            }
                        }
                    }
                }
            }
        }
    }
}
