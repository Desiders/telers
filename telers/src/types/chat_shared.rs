use serde::{Deserialize, Serialize};
/// This object contains information about a chat that was shared with the bot using a [`crate::types::KeyboardButtonRequestChat`] button.
/// # Documentation
/// <https://core.telegram.org/bots/api#chatshared>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChatShared {
    /// Identifier of the request
    pub request_id: i64,
    /// Identifier of the shared chat. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a 64-bit integer or double-precision float type are safe for storing this identifier. The bot may not have access to the chat and could be unable to use this identifier, unless the chat is already known to the bot by some other means.
    pub chat_id: i64,
    /// Title of the chat, if the title was requested by the bot.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<Box<str>>,
    /// Username of the chat, if the username was requested by the bot and available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<Box<str>>,
    /// Available sizes of the chat photo, if the photo was requested by the bot
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo: Option<Box<[crate::types::PhotoSize]>>,
}
impl ChatShared {
    /// Creates a new `ChatShared`.
    ///
    /// # Arguments
    /// * `request_id` - Identifier of the request
    /// * `chat_id` - Identifier of the shared chat. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a 64-bit integer or double-precision float type are safe for storing this identifier. The bot may not have access to the chat and could be unable to use this identifier, unless the chat is already known to the bot by some other means.
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0: Into<i64>, T1: Into<i64>>(request_id: T0, chat_id: T1) -> Self {
        Self {
            request_id: request_id.into(),
            chat_id: chat_id.into(),
            title: None,
            username: None,
            photo: None,
        }
    }

    /// Identifier of the request
    #[must_use]
    pub fn request_id<T: Into<i64>>(mut self, val: T) -> Self {
        self.request_id = val.into();
        self
    }

    /// Identifier of the shared chat. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a 64-bit integer or double-precision float type are safe for storing this identifier. The bot may not have access to the chat and could be unable to use this identifier, unless the chat is already known to the bot by some other means.
    #[must_use]
    pub fn chat_id<T: Into<i64>>(mut self, val: T) -> Self {
        self.chat_id = val.into();
        self
    }

    /// Title of the chat, if the title was requested by the bot.
    #[must_use]
    pub fn title<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.title = Some(val.into());
        self
    }

    /// Title of the chat, if the title was requested by the bot.
    #[must_use]
    pub fn title_option<T: Into<Box<str>>>(mut self, val: Option<T>) -> Self {
        self.title = val.map(Into::into);
        self
    }

    /// Username of the chat, if the username was requested by the bot and available.
    #[must_use]
    pub fn username<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.username = Some(val.into());
        self
    }

    /// Username of the chat, if the username was requested by the bot and available.
    #[must_use]
    pub fn username_option<T: Into<Box<str>>>(mut self, val: Option<T>) -> Self {
        self.username = val.map(Into::into);
        self
    }

    /// Available sizes of the chat photo, if the photo was requested by the bot
    ///
    /// # Notes
    /// Adds multiple elements.
    #[must_use]
    pub fn photos<T: Into<Box<[crate::types::PhotoSize]>>>(mut self, val: T) -> Self {
        self.photo = Some(
            self.photo
                .unwrap_or_default()
                .into_vec()
                .into_iter()
                .chain(val.into())
                .collect(),
        );
        self
    }

    /// Available sizes of the chat photo, if the photo was requested by the bot
    ///
    /// # Notes
    /// Adds a single element.
    #[must_use]
    pub fn photo<T: Into<crate::types::PhotoSize>>(mut self, val: T) -> Self {
        self.photo = Some(
            self.photo
                .unwrap_or_default()
                .into_vec()
                .into_iter()
                .chain(Some(val.into()))
                .collect(),
        );
        self
    }

    /// Available sizes of the chat photo, if the photo was requested by the bot
    ///
    /// # Notes
    /// Adds a single element.
    #[must_use]
    pub fn photo_option<T: Into<Box<[crate::types::PhotoSize]>>>(mut self, val: Option<T>) -> Self {
        self.photo = val.map(Into::into);
        self
    }
}
