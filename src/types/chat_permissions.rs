use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Describes actions that a non-administrator user is allowed to take in a chat.
/// # Documentation
/// <https://core.telegram.org/bots/api#chatpermissions>
#[skip_serializing_none]
#[derive(Default, Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct ChatPermissions {
    /// `True`, if the user is allowed to send text messages, contacts, locations and venues
    pub can_send_messages: Option<bool>,
    /// `True`, if the user is allowed to send audios, documents, photos, videos, video notes and voice notes, implies can_send_messages
    pub can_send_media_messages: Option<bool>,
    /// `True`, if the user is allowed to send polls, implies can_send_messages
    pub can_send_polls: Option<bool>,
    /// `True`, if the user is allowed to send animations, games, stickers and use inline bots, implies can_send_media_messages
    pub can_send_other_messages: Option<bool>,
    /// `True`, if the user is allowed to add web page previews to their messages, implies can_send_media_messages
    pub can_add_web_page_previews: Option<bool>,
    /// `True`, if the user is allowed to change the chat title, photo and other settings. Ignored in public supergroups
    pub can_change_info: Option<bool>,
    /// `True`, if the user is allowed to invite new users to the chat
    pub can_invite_users: Option<bool>,
    /// `True`, if the user is allowed to pin messages. Ignored in public supergroups
    pub can_pin_messages: Option<bool>,
    /// `True`, if the user is allowed to create forum topics. If omitted defaults to the value of can_pin_messages
    pub can_manage_topics: Option<bool>,
}

impl ChatPermissions {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn can_send_messages(self, val: bool) -> Self {
        Self {
            can_send_messages: Some(val),
            ..self
        }
    }

    #[must_use]
    pub fn can_send_media_messages(self, val: bool) -> Self {
        Self {
            can_send_media_messages: Some(val),
            ..self
        }
    }

    #[must_use]
    pub fn can_send_polls(self, val: bool) -> Self {
        Self {
            can_send_polls: Some(val),
            ..self
        }
    }

    #[must_use]
    pub fn can_send_other_messages(self, val: bool) -> Self {
        Self {
            can_send_other_messages: Some(val),
            ..self
        }
    }

    #[must_use]
    pub fn can_add_web_page_previews(self, val: bool) -> Self {
        Self {
            can_add_web_page_previews: Some(val),
            ..self
        }
    }

    #[must_use]
    pub fn can_change_info(self, val: bool) -> Self {
        Self {
            can_change_info: Some(val),
            ..self
        }
    }

    #[must_use]
    pub fn can_invite_users(self, val: bool) -> Self {
        Self {
            can_invite_users: Some(val),
            ..self
        }
    }

    #[must_use]
    pub fn can_pin_messages(self, val: bool) -> Self {
        Self {
            can_pin_messages: Some(val),
            ..self
        }
    }

    #[must_use]
    pub fn can_manage_topics(self, val: bool) -> Self {
        Self {
            can_manage_topics: Some(val),
            ..self
        }
    }
}
