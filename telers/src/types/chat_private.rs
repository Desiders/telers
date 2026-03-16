use serde::{Deserialize, Serialize};
/// This object represents a private chat.
/// # Notes
/// This object represents a chat from original chat type `private`.
/// # Documentation
/// <https://core.telegram.org/bots/api#chat>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChatPrivate {
    /// Unique identifier for this chat. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a signed 64-bit integer or double-precision float type are safe for storing this identifier.
    pub id: i64,
    /// Username, for private chats, supergroups and channels if available
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<Box<str>>,
    /// First name of the other party in a private chat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<Box<str>>,
    /// Last name of the other party in a private chat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<Box<str>>,
    /// `true`, if the chat is the direct messages chat of a channel
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_direct_messages: Option<bool>,
}
impl ChatPrivate {
    /// Creates a new `ChatPrivate`.
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
            username: None,
            first_name: None,
            last_name: None,
            is_direct_messages: None,
        }
    }

    /// Unique identifier for this chat. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a signed 64-bit integer or double-precision float type are safe for storing this identifier.
    #[must_use]
    pub fn id<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.id = val.into();
        this
    }

    /// Username, for private chats, supergroups and channels if available
    #[must_use]
    pub fn username<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.username = Some(val.into());
        this
    }

    /// Username, for private chats, supergroups and channels if available
    #[must_use]
    pub fn username_option<T: Into<Box<str>>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.username = val.map(Into::into);
        this
    }

    /// First name of the other party in a private chat
    #[must_use]
    pub fn first_name<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.first_name = Some(val.into());
        this
    }

    /// First name of the other party in a private chat
    #[must_use]
    pub fn first_name_option<T: Into<Box<str>>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.first_name = val.map(Into::into);
        this
    }

    /// Last name of the other party in a private chat
    #[must_use]
    pub fn last_name<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.last_name = Some(val.into());
        this
    }

    /// Last name of the other party in a private chat
    #[must_use]
    pub fn last_name_option<T: Into<Box<str>>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.last_name = val.map(Into::into);
        this
    }

    /// `true`, if the chat is the direct messages chat of a channel
    #[must_use]
    pub fn is_direct_messages<T: Into<bool>>(self, val: T) -> Self {
        let mut this = self;
        this.is_direct_messages = Some(val.into());
        this
    }

    /// `true`, if the chat is the direct messages chat of a channel
    #[must_use]
    pub fn is_direct_messages_option<T: Into<bool>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.is_direct_messages = val.map(Into::into);
        this
    }
}
