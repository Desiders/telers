use serde::{Deserialize, Serialize};
/// This object represents a supergroup chat.
/// # Notes
/// This object represents a chat from original chat type `supergroup`.
/// # Documentation
/// <https://core.telegram.org/bots/api#chat>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChatSupergroup {
    /// Unique identifier for this chat. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a signed 64-bit integer or double-precision float type are safe for storing this identifier.
    pub id: i64,
    /// Title, for supergroups, channels and group chats
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<Box<str>>,
    /// Username, for private chats, supergroups and channels if available
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<Box<str>>,
    /// `true`, if the supergroup chat is a forum (has topics enabled)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_forum: Option<bool>,
    /// `true`, if the chat is the direct messages chat of a channel
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_direct_messages: Option<bool>,
}
impl ChatSupergroup {
    /// Creates a new `ChatSupergroup`.
    ///
    /// # Arguments
    /// * `id` - Unique identifier for this chat. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a signed 64-bit integer or double-precision float type are safe for storing this identifier.
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0: Into<i64>>(id: T0) -> Self {
        Self {
            id: id.into(),
            title: None,
            username: None,
            is_forum: None,
            is_direct_messages: None,
        }
    }

    /// Unique identifier for this chat. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a signed 64-bit integer or double-precision float type are safe for storing this identifier.
    #[must_use]
    pub fn id<T: Into<i64>>(mut self, val: T) -> Self {
        self.id = val.into();
        self
    }

    /// Title, for supergroups, channels and group chats
    #[must_use]
    pub fn title<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.title = Some(val.into());
        self
    }

    /// Title, for supergroups, channels and group chats
    #[must_use]
    pub fn title_option<T: Into<Box<str>>>(mut self, val: Option<T>) -> Self {
        self.title = val.map(Into::into);
        self
    }

    /// Username, for private chats, supergroups and channels if available
    #[must_use]
    pub fn username<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.username = Some(val.into());
        self
    }

    /// Username, for private chats, supergroups and channels if available
    #[must_use]
    pub fn username_option<T: Into<Box<str>>>(mut self, val: Option<T>) -> Self {
        self.username = val.map(Into::into);
        self
    }

    /// `true`, if the supergroup chat is a forum (has topics enabled)
    #[must_use]
    pub fn is_forum<T: Into<bool>>(mut self, val: T) -> Self {
        self.is_forum = Some(val.into());
        self
    }

    /// `true`, if the supergroup chat is a forum (has topics enabled)
    #[must_use]
    pub fn is_forum_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.is_forum = val.map(Into::into);
        self
    }

    /// `true`, if the chat is the direct messages chat of a channel
    #[must_use]
    pub fn is_direct_messages<T: Into<bool>>(mut self, val: T) -> Self {
        self.is_direct_messages = Some(val.into());
        self
    }

    /// `true`, if the chat is the direct messages chat of a channel
    #[must_use]
    pub fn is_direct_messages_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.is_direct_messages = val.map(Into::into);
        self
    }
}
