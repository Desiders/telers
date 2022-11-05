use serde::{Deserialize, Serialize};

/// Describes actions that a non-administrator user is allowed to take in a chat.
/// <https://core.telegram.org/bots/api#chatpermissions>
#[derive(Default, Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct ChatPermissions {
    /// *Optional*. `True`, if the user is allowed to send text messages, contacts, locations and venues
    pub can_send_messages: Option<bool>,
    /// *Optional*. `True`, if the user is allowed to send audios, documents, photos, videos, video notes and voice notes, implies can_send_messages
    pub can_send_media_messages: Option<bool>,
    /// *Optional*. `True`, if the user is allowed to send polls, implies can_send_messages
    pub can_send_polls: Option<bool>,
    /// *Optional*. `True`, if the user is allowed to send animations, games, stickers and use inline bots, implies can_send_media_messages
    pub can_send_other_messages: Option<bool>,
    /// *Optional*. `True`, if the user is allowed to add web page previews to their messages, implies can_send_media_messages
    pub can_add_web_page_previews: Option<bool>,
    /// *Optional*. `True`, if the user is allowed to change the chat title, photo and other settings. Ignored in public supergroups
    pub can_change_info: Option<bool>,
    /// *Optional*. `True`, if the user is allowed to invite new users to the chat
    pub can_invite_users: Option<bool>,
    /// *Optional*. `True`, if the user is allowed to pin messages. Ignored in public supergroups
    pub can_pin_messages: Option<bool>,
    /// *Optional*. `True`, if the user is allowed to create forum topics. If omitted defaults to the value of can_pin_messages
    pub can_manage_topics: Option<bool>,
}
