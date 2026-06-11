use crate::client::Bot;
use serde::Serialize;
/// Use this method to edit text, rich and game messages. On success, if the edited message is not an inline message, the edited Message is returned, otherwise `true` is returned. Note that business messages that were not sent by the bot and do not contain an inline keyboard can only be edited within 48 hours from the time they were sent.
/// # Documentation
/// <https://core.telegram.org/bots/api#editmessagetext>
/// # Returns
/// - `crate::types::Message`
/// - `bool`
#[derive(Clone, Debug, Serialize)]
pub struct EditMessageText {
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
    /// New text of the message, 1-4096 characters after entity parsing; required if `rich_message` isn't specified
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<Box<str>>,
    /// Mode for parsing entities in the message text. See formatting options for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<Box<str>>,
    /// A JSON-serialized list of special entities that appear in message text, which can be specified instead of `parse_mode`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entities: Option<Box<[crate::types::MessageEntity]>>,
    /// Link preview generation options for the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_preview_options: Option<crate::types::LinkPreviewOptions>,
    /// New rich content of the message; required if text isn't specified
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rich_message: Option<crate::types::InputRichMessage>,
    /// A JSON-serialized object for an inline keyboard
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<crate::types::InlineKeyboardMarkup>,
}
impl EditMessageText {
    /// Creates a new `EditMessageText`.
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new() -> Self {
        Self {
            business_connection_id: None,
            chat_id: None,
            message_id: None,
            inline_message_id: None,
            text: None,
            parse_mode: None,
            entities: None,
            link_preview_options: None,
            rich_message: None,
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

    /// New text of the message, 1-4096 characters after entity parsing; required if `rich_message` isn't specified
    #[must_use]
    pub fn text<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.text = Some(val.into());
        self
    }

    /// New text of the message, 1-4096 characters after entity parsing; required if `rich_message` isn't specified
    #[must_use]
    pub fn text_option<T: Into<Box<str>>>(mut self, val: Option<T>) -> Self {
        self.text = val.map(Into::into);
        self
    }

    /// Mode for parsing entities in the message text. See formatting options for more details.
    #[must_use]
    pub fn parse_mode<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.parse_mode = Some(val.into());
        self
    }

    /// Mode for parsing entities in the message text. See formatting options for more details.
    #[must_use]
    pub fn parse_mode_option<T: Into<Box<str>>>(mut self, val: Option<T>) -> Self {
        self.parse_mode = val.map(Into::into);
        self
    }

    /// A JSON-serialized list of special entities that appear in message text, which can be specified instead of `parse_mode`
    ///
    /// # Notes
    /// Adds multiple elements.
    #[must_use]
    pub fn entities<TItem: Into<crate::types::MessageEntity>, T: IntoIterator<Item = TItem>>(
        mut self,
        val: T,
    ) -> Self {
        self.entities = Some(
            self.entities
                .unwrap_or_default()
                .into_vec()
                .into_iter()
                .chain(val.into_iter().map(Into::into))
                .collect(),
        );
        self
    }

    /// A JSON-serialized list of special entities that appear in message text, which can be specified instead of `parse_mode`
    ///
    /// # Notes
    /// Adds a single element.
    #[must_use]
    pub fn entity<T: Into<crate::types::MessageEntity>>(mut self, val: T) -> Self {
        self.entities = Some(
            self.entities
                .unwrap_or_default()
                .into_vec()
                .into_iter()
                .chain(Some(val.into()))
                .collect(),
        );
        self
    }

    /// A JSON-serialized list of special entities that appear in message text, which can be specified instead of `parse_mode`
    ///
    /// # Notes
    /// Adds multiple elements.
    #[must_use]
    pub fn entities_option<
        TItem: Into<crate::types::MessageEntity>,
        T: IntoIterator<Item = TItem>,
    >(
        mut self,
        val: Option<T>,
    ) -> Self {
        self.entities = val.map(|v| v.into_iter().map(Into::into).collect());
        self
    }

    /// Link preview generation options for the message
    #[must_use]
    pub fn link_preview_options<T: Into<crate::types::LinkPreviewOptions>>(
        mut self,
        val: T,
    ) -> Self {
        self.link_preview_options = Some(val.into());
        self
    }

    /// Link preview generation options for the message
    #[must_use]
    pub fn link_preview_options_option<T: Into<crate::types::LinkPreviewOptions>>(
        mut self,
        val: Option<T>,
    ) -> Self {
        self.link_preview_options = val.map(Into::into);
        self
    }

    /// New rich content of the message; required if text isn't specified
    #[must_use]
    pub fn rich_message<T: Into<crate::types::InputRichMessage>>(mut self, val: T) -> Self {
        self.rich_message = Some(val.into());
        self
    }

    /// New rich content of the message; required if text isn't specified
    #[must_use]
    pub fn rich_message_option<T: Into<crate::types::InputRichMessage>>(
        mut self,
        val: Option<T>,
    ) -> Self {
        self.rich_message = val.map(Into::into);
        self
    }

    /// A JSON-serialized object for an inline keyboard
    #[must_use]
    pub fn reply_markup<T: Into<crate::types::InlineKeyboardMarkup>>(mut self, val: T) -> Self {
        self.reply_markup = Some(val.into());
        self
    }

    /// A JSON-serialized object for an inline keyboard
    #[must_use]
    pub fn reply_markup_option<T: Into<crate::types::InlineKeyboardMarkup>>(
        mut self,
        val: Option<T>,
    ) -> Self {
        self.reply_markup = val.map(Into::into);
        self
    }
}
impl Default for EditMessageText {
    fn default() -> Self {
        Self::new()
    }
}
impl super::TelegramMethod for EditMessageText {
    type Method = Self;
    type Return = crate::Either<crate::types::Message, bool>;

    fn build_request<Client>(self, _bot: &Bot<Client>) -> super::Request<Self::Method> {
        super::Request::new("editMessageText", self, None)
    }
}
