use crate::client::Bot;
use serde::Serialize;
/// Use this method to edit animation, audio, document, live photo, photo, or video messages, or to replace a text or a rich message with a media. If a message is part of a message album, then it can be edited only to an audio for audio albums, only to a document for document albums and to a photo, a live photo, or a video otherwise. When an inline message is edited, a new file can't be uploaded; use a previously uploaded file via its `file_id` or specify a URL. On success, if the edited message is not an inline message, the edited Message is returned, otherwise `true` is returned. Note that business messages that were not sent by the bot and do not contain an inline keyboard can only be edited within 48 hours from the time they were sent.
/// # Documentation
/// <https://core.telegram.org/bots/api#editmessagemedia>
/// # Returns
/// - `crate::types::Message`
/// - `bool`
#[derive(Clone, Debug, Serialize)]
pub struct EditMessageMedia {
    /// Unique identifier of the business connection on behalf of which the message to be edited was sent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<Box<str>>,
    /// Required if `inline_message_id` is not specified. Unique identifier for the target chat or username of the target bot, supergroup or channel in the format @username.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<crate::types::ChatIdKind>,
    /// Required if `inline_message_id` is not specified. Identifier of the message to edit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<i64>,
    /// Required if `chat_id` and `message_id` are not specified. Identifier of the inline message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<Box<str>>,
    /// A JSON-serialized object for a new media content of the message
    pub media: crate::types::InputMedia,
    /// A JSON-serialized object for a new inline keyboard
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<crate::types::InlineKeyboardMarkup>,
}
impl EditMessageMedia {
    /// Creates a new `EditMessageMedia`.
    ///
    /// # Arguments
    /// * `media` - A JSON-serialized object for a new media content of the message
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0: Into<crate::types::InputMedia>>(media: T0) -> Self {
        Self {
            business_connection_id: None,
            chat_id: None,
            message_id: None,
            inline_message_id: None,
            media: media.into(),
            reply_markup: None,
        }
    }

    /// Unique identifier of the business connection on behalf of which the message to be edited was sent
    #[must_use]
    pub fn business_connection_id<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.business_connection_id = Some(val.into());
        self
    }

    /// Unique identifier of the business connection on behalf of which the message to be edited was sent
    #[must_use]
    pub fn business_connection_id_option<T: Into<Box<str>>>(mut self, val: Option<T>) -> Self {
        self.business_connection_id = val.map(Into::into);
        self
    }

    /// Required if `inline_message_id` is not specified. Unique identifier for the target chat or username of the target bot, supergroup or channel in the format @username.
    #[must_use]
    pub fn chat_id<T: Into<crate::types::ChatIdKind>>(mut self, val: T) -> Self {
        self.chat_id = Some(val.into());
        self
    }

    /// Required if `inline_message_id` is not specified. Unique identifier for the target chat or username of the target bot, supergroup or channel in the format @username.
    #[must_use]
    pub fn chat_id_option<T: Into<crate::types::ChatIdKind>>(mut self, val: Option<T>) -> Self {
        self.chat_id = val.map(Into::into);
        self
    }

    /// Required if `inline_message_id` is not specified. Identifier of the message to edit.
    #[must_use]
    pub fn message_id<T: Into<i64>>(mut self, val: T) -> Self {
        self.message_id = Some(val.into());
        self
    }

    /// Required if `inline_message_id` is not specified. Identifier of the message to edit.
    #[must_use]
    pub fn message_id_option<T: Into<i64>>(mut self, val: Option<T>) -> Self {
        self.message_id = val.map(Into::into);
        self
    }

    /// Required if `chat_id` and `message_id` are not specified. Identifier of the inline message.
    #[must_use]
    pub fn inline_message_id<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.inline_message_id = Some(val.into());
        self
    }

    /// Required if `chat_id` and `message_id` are not specified. Identifier of the inline message.
    #[must_use]
    pub fn inline_message_id_option<T: Into<Box<str>>>(mut self, val: Option<T>) -> Self {
        self.inline_message_id = val.map(Into::into);
        self
    }

    /// A JSON-serialized object for a new media content of the message
    #[must_use]
    pub fn media<T: Into<crate::types::InputMedia>>(mut self, val: T) -> Self {
        self.media = val.into();
        self
    }

    /// A JSON-serialized object for a new inline keyboard
    #[must_use]
    pub fn reply_markup<T: Into<crate::types::InlineKeyboardMarkup>>(mut self, val: T) -> Self {
        self.reply_markup = Some(val.into());
        self
    }

    /// A JSON-serialized object for a new inline keyboard
    #[must_use]
    pub fn reply_markup_option<T: Into<crate::types::InlineKeyboardMarkup>>(
        mut self,
        val: Option<T>,
    ) -> Self {
        self.reply_markup = val.map(Into::into);
        self
    }
}
impl super::TelegramMethod for EditMessageMedia {
    type Method = Self;
    type Return = crate::Either<crate::types::Message, bool>;

    fn build_request<Client>(mut self, _bot: &Bot<Client>) -> super::Request<Self::Method> {
        let mut files = vec![];
        super::prepare_input_media(&mut files, &mut self.media);
        super::Request::new("editMessageMedia", self, Some(files))
    }
}
