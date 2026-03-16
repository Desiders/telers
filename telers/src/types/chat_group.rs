use serde::{Deserialize, Serialize};
/// This object represents a group chat.
/// # Notes
/// This object represents a chat from original chat type `group`.
/// # Documentation
/// <https://core.telegram.org/bots/api#chat>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChatGroup {
    /// Unique identifier for this chat. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a signed 64-bit integer or double-precision float type are safe for storing this identifier.
    pub id: i64,
    /// `true`, if the chat is the direct messages chat of a channel
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_direct_messages: Option<bool>,
}
impl ChatGroup {
    /// Creates a new `ChatGroup`.
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
