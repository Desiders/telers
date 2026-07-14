use crate::client::Bot;
use serde::Serialize;
/// Use this method to edit the media of an ephemeral message. Note that it is not guaranteed that the user will receive the message edit event, especially if they are offline. On success, `true` is returned.
/// # Documentation
/// <https://core.telegram.org/bots/api#editephemeralmessagemedia>
/// # Returns
/// - `bool`
#[derive(Clone, Debug, Serialize)]
pub struct EditEphemeralMessageMedia {
    /// Unique identifier for the target chat or username of the target supergroup in the format @username
    pub chat_id: crate::types::ChatIdKind,
    /// Identifier of the user who received the message
    pub receiver_user_id: i64,
    /// Identifier of the ephemeral message to edit
    pub ephemeral_message_id: i64,
    /// A JSON-serialized object for the new media content of the message. A new file can't be uploaded; use a previously uploaded file via its `file_id` or specify a URL.
    pub media: crate::types::InputMedia,
    /// A JSON-serialized object for an inline keyboard
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<crate::types::InlineKeyboardMarkup>,
}
impl EditEphemeralMessageMedia {
    /// Creates a new `EditEphemeralMessageMedia`.
    ///
    /// # Arguments
    /// * `chat_id` - Unique identifier for the target chat or username of the target supergroup in the format @username
    /// * `receiver_user_id` - Identifier of the user who received the message
    /// * `ephemeral_message_id` - Identifier of the ephemeral message to edit
    /// * `media` - A JSON-serialized object for the new media content of the message. A new file can't be uploaded; use a previously uploaded file via its `file_id` or specify a URL.
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<
        T0: Into<crate::types::ChatIdKind>,
        T1: Into<i64>,
        T2: Into<i64>,
        T3: Into<crate::types::InputMedia>,
    >(
        chat_id: T0,
        receiver_user_id: T1,
        ephemeral_message_id: T2,
        media: T3,
    ) -> Self {
        Self {
            chat_id: chat_id.into(),
            receiver_user_id: receiver_user_id.into(),
            ephemeral_message_id: ephemeral_message_id.into(),
            media: media.into(),
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

    /// A JSON-serialized object for the new media content of the message. A new file can't be uploaded; use a previously uploaded file via its `file_id` or specify a URL.
    #[must_use]
    pub fn media<T: Into<crate::types::InputMedia>>(mut self, val: T) -> Self {
        self.media = val.into();
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
impl super::TelegramMethod for EditEphemeralMessageMedia {
    type Method = Self;
    type Return = bool;

    fn build_request<Client>(mut self, _bot: &Bot<Client>) -> super::Request<Self::Method> {
        let mut files = vec![];
        super::prepare_input_media(&mut files, &mut self.media);
        super::Request::new("editEphemeralMessageMedia", self, Some(files))
    }
}
