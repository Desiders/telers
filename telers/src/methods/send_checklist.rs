use crate::client::Bot;
use serde::Serialize;
/// Use this method to send a checklist on behalf of a connected business account. On success, the sent Message is returned.
/// # Documentation
/// <https://core.telegram.org/bots/api#sendchecklist>
/// # Returns
/// - `crate::types::Message`
#[derive(Clone, Debug, Serialize)]
pub struct SendChecklist {
    /// Unique identifier of the business connection on behalf of which the message will be sent
    pub business_connection_id: Box<str>,
    /// Unique identifier for the target chat or username of the target bot in the format @username
    pub chat_id: crate::types::ChatIdKind,
    /// A JSON-serialized object for the checklist to send
    pub checklist: crate::types::InputChecklist,
    /// Sends the message silently. Users will receive a notification with no sound.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    /// Protects the contents of the sent message from forwarding and saving
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,
    /// Unique identifier of the message effect to be added to the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_effect_id: Option<Box<str>>,
    /// A JSON-serialized object for description of the message to reply to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_parameters: Option<crate::types::ReplyParameters>,
    /// A JSON-serialized object for an inline keyboard
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<crate::types::InlineKeyboardMarkup>,
}
impl SendChecklist {
    /// Creates a new `SendChecklist`.
    ///
    /// # Arguments
    /// * `business_connection_id` - Unique identifier of the business connection on behalf of which the message will be sent
    /// * `chat_id` - Unique identifier for the target chat or username of the target bot in the format @username
    /// * `checklist` - A JSON-serialized object for the checklist to send
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<
        T0: Into<Box<str>>,
        T1: Into<crate::types::ChatIdKind>,
        T2: Into<crate::types::InputChecklist>,
    >(
        business_connection_id: T0,
        chat_id: T1,
        checklist: T2,
    ) -> Self {
        Self {
            business_connection_id: business_connection_id.into(),
            chat_id: chat_id.into(),
            checklist: checklist.into(),
            disable_notification: None,
            protect_content: None,
            message_effect_id: None,
            reply_parameters: None,
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

    /// A JSON-serialized object for the checklist to send
    #[must_use]
    pub fn checklist<T: Into<crate::types::InputChecklist>>(mut self, val: T) -> Self {
        self.checklist = val.into();
        self
    }

    /// Sends the message silently. Users will receive a notification with no sound.
    #[must_use]
    pub fn disable_notification<T: Into<bool>>(mut self, val: T) -> Self {
        self.disable_notification = Some(val.into());
        self
    }

    /// Sends the message silently. Users will receive a notification with no sound.
    #[must_use]
    pub fn disable_notification_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.disable_notification = val.map(Into::into);
        self
    }

    /// Protects the contents of the sent message from forwarding and saving
    #[must_use]
    pub fn protect_content<T: Into<bool>>(mut self, val: T) -> Self {
        self.protect_content = Some(val.into());
        self
    }

    /// Protects the contents of the sent message from forwarding and saving
    #[must_use]
    pub fn protect_content_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.protect_content = val.map(Into::into);
        self
    }

    /// Unique identifier of the message effect to be added to the message
    #[must_use]
    pub fn message_effect_id<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.message_effect_id = Some(val.into());
        self
    }

    /// Unique identifier of the message effect to be added to the message
    #[must_use]
    pub fn message_effect_id_option<T: Into<Box<str>>>(mut self, val: Option<T>) -> Self {
        self.message_effect_id = val.map(Into::into);
        self
    }

    /// A JSON-serialized object for description of the message to reply to
    #[must_use]
    pub fn reply_parameters<T: Into<crate::types::ReplyParameters>>(mut self, val: T) -> Self {
        self.reply_parameters = Some(val.into());
        self
    }

    /// A JSON-serialized object for description of the message to reply to
    #[must_use]
    pub fn reply_parameters_option<T: Into<crate::types::ReplyParameters>>(
        mut self,
        val: Option<T>,
    ) -> Self {
        self.reply_parameters = val.map(Into::into);
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
impl super::TelegramMethod for SendChecklist {
    type Method = Self;
    type Return = crate::types::Message;

    fn build_request<Client>(self, _bot: &Bot<Client>) -> super::Request<Self::Method> {
        super::Request::new("sendChecklist", self, None)
    }
}
