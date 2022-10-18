use super::User;

use serde::{Deserialize, Serialize};

/// Represents a `chat member <https://core.telegram.org/bots/api#chatmember>`_ that is under certain restrictions in the chat. Supergroups only.
/// <https://core.telegram.org/bots/api#chatmemberrestricted>_
#[allow(clippy::struct_excessive_bools)]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct ChatMemberRestricted {
    /// The member's status in the chat, always 'restricted'
    #[serde(default = "restricted")]
    pub status: String,
    /// Information about the user
    pub user: User,
    /// :code:`True`, if the user is a member of the chat at the moment of the request
    pub is_member: bool,
    /// :code:`True`, if the user is allowed to change the chat title, photo and other settings
    pub can_change_info: bool,
    /// :code:`True`, if the user is allowed to invite new users to the chat
    pub can_invite_users: bool,
    /// :code:`True`, if the user is allowed to pin messages
    pub can_pin_messages: bool,
    /// :code:`True`, if the user is allowed to send text messages, contacts, locations and venues
    pub can_send_messages: bool,
    /// :code:`True`, if the user is allowed to send audios, documents, photos, videos, video notes and voice notes
    pub can_send_media_messages: bool,
    /// :code:`True`, if the user is allowed to send polls
    pub can_send_polls: bool,
    /// :code:`True`, if the user is allowed to send animations, games, stickers and use inline bots
    pub can_send_other_messages: bool,
    /// :code:`True`, if the user is allowed to add web page previews to their messages
    pub can_add_web_page_previews: bool,
    /// Date when restrictions will be lifted for this user; unix time. If 0, then the user is restricted forever
    pub until_date: i64,
}

fn restricted() -> String {
    "restricted".to_string()
}
