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
    /// Unique identifier for the target chat
    pub chat_id: i64,
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
    /// * `chat_id` - Unique identifier for the target chat
    /// * `checklist` - A JSON-serialized object for the checklist to send
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0: Into<Box<str>>, T1: Into<i64>, T2: Into<crate::types::InputChecklist>>(
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
    pub fn business_connection_id<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.business_connection_id = val.into();
        this
    }

    /// Unique identifier for the target chat
    #[must_use]
    pub fn chat_id<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.chat_id = val.into();
        this
    }

    /// A JSON-serialized object for the checklist to send
    #[must_use]
    pub fn checklist<T: Into<crate::types::InputChecklist>>(self, val: T) -> Self {
        let mut this = self;
        this.checklist = val.into();
        this
    }

    /// Sends the message silently. Users will receive a notification with no sound.
    #[must_use]
    pub fn disable_notification<T: Into<bool>>(self, val: T) -> Self {
        let mut this = self;
        this.disable_notification = Some(val.into());
        this
    }

    /// Sends the message silently. Users will receive a notification with no sound.
    #[must_use]
    pub fn disable_notification_option<T: Into<bool>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.disable_notification = val.map(Into::into);
        this
    }

    /// Protects the contents of the sent message from forwarding and saving
    #[must_use]
    pub fn protect_content<T: Into<bool>>(self, val: T) -> Self {
        let mut this = self;
        this.protect_content = Some(val.into());
        this
    }

    /// Protects the contents of the sent message from forwarding and saving
    #[must_use]
    pub fn protect_content_option<T: Into<bool>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.protect_content = val.map(Into::into);
        this
    }

    /// Unique identifier of the message effect to be added to the message
    #[must_use]
    pub fn message_effect_id<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.message_effect_id = Some(val.into());
        this
    }

    /// Unique identifier of the message effect to be added to the message
    #[must_use]
    pub fn message_effect_id_option<T: Into<Box<str>>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.message_effect_id = val.map(Into::into);
        this
    }

    /// A JSON-serialized object for description of the message to reply to
    #[must_use]
    pub fn reply_parameters<T: Into<crate::types::ReplyParameters>>(self, val: T) -> Self {
        let mut this = self;
        this.reply_parameters = Some(val.into());
        this
    }

    /// A JSON-serialized object for description of the message to reply to
    #[must_use]
    pub fn reply_parameters_option<T: Into<crate::types::ReplyParameters>>(
        self,
        val: Option<T>,
    ) -> Self {
        let mut this = self;
        this.reply_parameters = val.map(Into::into);
        this
    }

    /// A JSON-serialized object for an inline keyboard
    #[must_use]
    pub fn reply_markup<T: Into<crate::types::InlineKeyboardMarkup>>(self, val: T) -> Self {
        let mut this = self;
        this.reply_markup = Some(val.into());
        this
    }

    /// A JSON-serialized object for an inline keyboard
    #[must_use]
    pub fn reply_markup_option<T: Into<crate::types::InlineKeyboardMarkup>>(
        self,
        val: Option<T>,
    ) -> Self {
        let mut this = self;
        this.reply_markup = val.map(Into::into);
        this
    }
}
impl super::TelegramMethod for SendChecklist {
    type Method = Self;
    type Return = crate::types::Message;

    fn build_request<Client>(self, _bot: &Bot<Client>) -> super::Request<Self::Method> {
        super::Request::new("sendChecklist", self, None)
    }
}
