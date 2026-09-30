//! The participant block shared by the create and the edit modal — see `docs/plans/friends.md` §11.

mod participants_editor;
pub mod participants_service;
mod use_friend_list;

pub use participants_editor::{Avatar, ParticipantsEditor};
pub use use_friend_list::{use_friend_list, use_my_display_name, FriendList};
