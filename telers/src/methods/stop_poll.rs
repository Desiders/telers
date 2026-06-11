use crate::client::Bot;
use serde::Serialize;
/// Use this method to stop a poll which was sent by the bot. On success, the stopped Poll is returned.
/// # Documentation
/// <https://core.telegram.org/bots/api#stoppoll>
/// # Returns
/// - `crate::types::Poll`
#[derive(Clone, Debug, Serialize)]
pub struct StopPoll {
    /// Unique identifier of the business connection on behalf of which the message to be edited was sent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<Box<str>>,
    /// Unique identifier for the target chat or username of the target bot, supergroup or channel in the format @username
    pub chat_id: crate::types::ChatIdKind,
    /// Identifier of the original message with the poll
    pub message_id: i64,
    /// A JSON-serialized object for a new message inline keyboard
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<crate::types::InlineKeyboardMarkup>,
}
impl StopPoll {
    /// Creates a new `StopPoll`.
    ///
    /// # Arguments
    /// * `chat_id` - Unique identifier for the target chat or username of the target bot, supergroup or channel in the format @username
    /// * `message_id` - Identifier of the original message with the poll
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0: Into<crate::types::ChatIdKind>, T1: Into<i64>>(
        chat_id: T0,
        message_id: T1,
    ) -> Self {
        Self {
            business_connection_id: None,
            chat_id: chat_id.into(),
            message_id: message_id.into(),
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

    /// Unique identifier for the target chat or username of the target bot, supergroup or channel in the format @username
    #[must_use]
    pub fn chat_id<T: Into<crate::types::ChatIdKind>>(mut self, val: T) -> Self {
        self.chat_id = val.into();
        self
    }

    /// Identifier of the original message with the poll
    #[must_use]
    pub fn message_id<T: Into<i64>>(mut self, val: T) -> Self {
        self.message_id = val.into();
        self
    }

    /// A JSON-serialized object for a new message inline keyboard
    #[must_use]
    pub fn reply_markup<T: Into<crate::types::InlineKeyboardMarkup>>(mut self, val: T) -> Self {
        self.reply_markup = Some(val.into());
        self
    }

    /// A JSON-serialized object for a new message inline keyboard
    #[must_use]
    pub fn reply_markup_option<T: Into<crate::types::InlineKeyboardMarkup>>(
        mut self,
        val: Option<T>,
    ) -> Self {
        self.reply_markup = val.map(Into::into);
        self
    }
}
impl super::TelegramMethod for StopPoll {
    type Method = Self;
    type Return = crate::types::Poll;

    fn build_request<Client>(self, _bot: &Bot<Client>) -> super::Request<Self::Method> {
        super::Request::new("stopPoll", self, None)
    }
}
