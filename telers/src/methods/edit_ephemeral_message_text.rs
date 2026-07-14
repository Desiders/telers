use crate::client::Bot;
use serde::Serialize;
/// Use this method to edit an ephemeral text message. Note that it is not guaranteed that the user will receive the message edit event, especially if they are offline. On success, `true` is returned.
/// # Documentation
/// <https://core.telegram.org/bots/api#editephemeralmessagetext>
/// # Returns
/// - `bool`
#[derive(Clone, Debug, Serialize)]
pub struct EditEphemeralMessageText {
    /// Unique identifier for the target chat or username of the target supergroup in the format @username
    pub chat_id: crate::types::ChatIdKind,
    /// Identifier of the user who received the message
    pub receiver_user_id: i64,
    /// Identifier of the ephemeral message to edit
    pub ephemeral_message_id: i64,
    /// New text of the message, 1-4096 characters after entity parsing
    pub text: Box<str>,
    /// Mode for parsing entities in the message text. See formatting options for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<Box<str>>,
    /// A JSON-serialized list of special entities that appear in message text, which can be specified instead of `parse_mode`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entities: Option<Box<[crate::types::MessageEntity]>>,
    /// Link preview generation options for the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_preview_options: Option<crate::types::LinkPreviewOptions>,
    /// A JSON-serialized object for an inline keyboard
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<crate::types::InlineKeyboardMarkup>,
}
impl EditEphemeralMessageText {
    /// Creates a new `EditEphemeralMessageText`.
    ///
    /// # Arguments
    /// * `chat_id` - Unique identifier for the target chat or username of the target supergroup in the format @username
    /// * `receiver_user_id` - Identifier of the user who received the message
    /// * `ephemeral_message_id` - Identifier of the ephemeral message to edit
    /// * `text` - New text of the message, 1-4096 characters after entity parsing
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<
        T0: Into<crate::types::ChatIdKind>,
        T1: Into<i64>,
        T2: Into<i64>,
        T3: Into<Box<str>>,
    >(
        chat_id: T0,
        receiver_user_id: T1,
        ephemeral_message_id: T2,
        text: T3,
    ) -> Self {
        Self {
            chat_id: chat_id.into(),
            receiver_user_id: receiver_user_id.into(),
            ephemeral_message_id: ephemeral_message_id.into(),
            text: text.into(),
            parse_mode: None,
            entities: None,
            link_preview_options: None,
            reply_markup: None,
        }
    }

    /// Unique identifier for the target chat or username of the target supergroup in the format @username
    #[must_use]
    pub fn chat_id<T: Into<crate::types::ChatIdKind>>(mut self, val: T) -> Self {
        self.chat_id = val.into();
        self
    }

    /// Identifier of the user who received the message
    #[must_use]
    pub fn receiver_user_id<T: Into<i64>>(mut self, val: T) -> Self {
        self.receiver_user_id = val.into();
        self
    }

    /// Identifier of the ephemeral message to edit
    #[must_use]
    pub fn ephemeral_message_id<T: Into<i64>>(mut self, val: T) -> Self {
        self.ephemeral_message_id = val.into();
        self
    }

    /// New text of the message, 1-4096 characters after entity parsing
    #[must_use]
    pub fn text<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.text = val.into();
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
impl super::TelegramMethod for EditEphemeralMessageText {
    type Method = Self;
    type Return = bool;

    fn build_request<Client>(self, _bot: &Bot<Client>) -> super::Request<Self::Method> {
        super::Request::new("editEphemeralMessageText", self, None)
    }
}
