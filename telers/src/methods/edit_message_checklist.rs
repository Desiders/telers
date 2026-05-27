use crate::client::Bot;
use serde::Serialize;
/// Use this method to edit a checklist on behalf of a connected business account. On success, the edited Message is returned.
/// # Documentation
/// <https://core.telegram.org/bots/api#editmessagechecklist>
/// # Returns
/// - `crate::types::Message`
#[derive(Clone, Debug, Serialize)]
pub struct EditMessageChecklist {
    /// Unique identifier of the business connection on behalf of which the message will be sent
    pub business_connection_id: Box<str>,
    /// Unique identifier for the target chat or username of the target bot in the format @username
    pub chat_id: crate::types::ChatIdKind,
    /// Unique identifier for the target message
    pub message_id: i64,
    /// A JSON-serialized object for the new checklist
    pub checklist: crate::types::InputChecklist,
    /// A JSON-serialized object for the new inline keyboard for the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<crate::types::InlineKeyboardMarkup>,
}
impl EditMessageChecklist {
    /// Creates a new `EditMessageChecklist`.
    ///
    /// # Arguments
    /// * `business_connection_id` - Unique identifier of the business connection on behalf of which the message will be sent
    /// * `chat_id` - Unique identifier for the target chat or username of the target bot in the format @username
    /// * `message_id` - Unique identifier for the target message
    /// * `checklist` - A JSON-serialized object for the new checklist
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<
        T0: Into<Box<str>>,
        T1: Into<crate::types::ChatIdKind>,
        T2: Into<i64>,
        T3: Into<crate::types::InputChecklist>,
    >(
        business_connection_id: T0,
        chat_id: T1,
        message_id: T2,
        checklist: T3,
    ) -> Self {
        Self {
            business_connection_id: business_connection_id.into(),
            chat_id: chat_id.into(),
            message_id: message_id.into(),
            checklist: checklist.into(),
            reply_markup: None,
        }
    }

    /// Unique identifier of the business connection on behalf of which the message will be sent
    #[must_use]
    pub fn business_connection_id<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.business_connection_id = val.into();
        self
    }

    /// Unique identifier for the target chat or username of the target bot in the format @username
    #[must_use]
    pub fn chat_id<T: Into<crate::types::ChatIdKind>>(mut self, val: T) -> Self {
        self.chat_id = val.into();
        self
    }

    /// Unique identifier for the target message
    #[must_use]
    pub fn message_id<T: Into<i64>>(mut self, val: T) -> Self {
        self.message_id = val.into();
        self
    }

    /// A JSON-serialized object for the new checklist
    #[must_use]
    pub fn checklist<T: Into<crate::types::InputChecklist>>(mut self, val: T) -> Self {
        self.checklist = val.into();
        self
    }

    /// A JSON-serialized object for the new inline keyboard for the message
    #[must_use]
    pub fn reply_markup<T: Into<crate::types::InlineKeyboardMarkup>>(mut self, val: T) -> Self {
        self.reply_markup = Some(val.into());
        self
    }

    /// A JSON-serialized object for the new inline keyboard for the message
    #[must_use]
    pub fn reply_markup_option<T: Into<crate::types::InlineKeyboardMarkup>>(
        mut self,
        val: Option<T>,
    ) -> Self {
        self.reply_markup = val.map(Into::into);
        self
    }
}
impl super::TelegramMethod for EditMessageChecklist {
    type Method = Self;
    type Return = crate::types::Message;

    fn build_request<Client>(self, _bot: &Bot<Client>) -> super::Request<Self::Method> {
        super::Request::new("editMessageChecklist", self, None)
    }
}
