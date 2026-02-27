use serde::{Deserialize, Serialize};
/// Represents a link to a sticker stored on the Telegram servers. By default, this sticker will be sent by the user. Alternatively, you can use `input_message_content` to send a message with the specified content instead of the sticker.
/// # Documentation
/// <https://core.telegram.org/bots/api#inlinequeryresultcachedsticker>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InlineQueryResultCachedSticker {
    /// Unique identifier for this result, 1-64 bytes
    pub id: Box<str>,
    /// A valid file identifier of the sticker
    pub sticker_file_id: Box<str>,
    /// Inline keyboard attached to the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<crate::types::InlineKeyboardMarkup>,
    /// Content of the message to be sent instead of the sticker
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<crate::types::InputMessageContent>,
}
impl InlineQueryResultCachedSticker {
    /// Creates a new `InlineQueryResultCachedSticker`.
    ///
    /// # Arguments
    /// * `id` - Unique identifier for this result, 1-64 bytes
    /// * `sticker_file_id` - A valid file identifier of the sticker
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0: Into<Box<str>>, T1: Into<Box<str>>>(id: T0, sticker_file_id: T1) -> Self {
        Self {
            id: id.into(),
            sticker_file_id: sticker_file_id.into(),
            reply_markup: None,
            input_message_content: None,
        }
    }

    /// Unique identifier for this result, 1-64 bytes
    #[must_use]
    pub fn id<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.id = val.into();
        this
    }

    /// A valid file identifier of the sticker
    #[must_use]
    pub fn sticker_file_id<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.sticker_file_id = val.into();
        this
    }

    /// Inline keyboard attached to the message
    #[must_use]
    pub fn reply_markup<T: Into<crate::types::InlineKeyboardMarkup>>(self, val: T) -> Self {
        let mut this = self;
        this.reply_markup = Some(val.into());
        this
    }

    /// Inline keyboard attached to the message
    #[must_use]
    pub fn reply_markup_option<T: Into<crate::types::InlineKeyboardMarkup>>(
        self,
        val: Option<T>,
    ) -> Self {
        let mut this = self;
        this.reply_markup = val.map(Into::into);
        this
    }

    /// Content of the message to be sent instead of the sticker
    #[must_use]
    pub fn input_message_content<T: Into<crate::types::InputMessageContent>>(self, val: T) -> Self {
        let mut this = self;
        this.input_message_content = Some(val.into());
        this
    }

    /// Content of the message to be sent instead of the sticker
    #[must_use]
    pub fn input_message_content_option<T: Into<crate::types::InputMessageContent>>(
        self,
        val: Option<T>,
    ) -> Self {
        let mut this = self;
        this.input_message_content = val.map(Into::into);
        this
    }
}
