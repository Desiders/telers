use crate::client::Bot;
use serde::Serialize;
/// Use this method to add a message to the list of pinned messages in a chat. In private chats and channel direct messages chats, all non-service messages can be pinned. Conversely, the bot must be an administrator with the '`can_pin_messages`' right or the '`can_edit_messages`' right to pin messages in groups and channels respectively. Returns `true` on success.
/// # Documentation
/// <https://core.telegram.org/bots/api#pinchatmessage>
/// # Returns
/// - `bool`
#[derive(Clone, Debug, Serialize)]
pub struct PinChatMessage {
    /// Unique identifier of the business connection on behalf of which the message will be pinned
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<Box<str>>,
    /// Unique identifier for the target chat or username of the target channel in the format @username
    pub chat_id: crate::types::ChatIdKind,
    /// Identifier of a message to pin
    pub message_id: i64,
    /// Pass `true` if it is not necessary to send a notification to all chat members about the new pinned message. Notifications are always disabled in channels and private chats.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
}
impl PinChatMessage {
    /// Creates a new `PinChatMessage`.
    ///
    /// # Arguments
    /// * `chat_id` - Unique identifier for the target chat or username of the target channel in the format @username
    /// * `message_id` - Identifier of a message to pin
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
            disable_notification: None,
        }
    }

    /// Unique identifier of the business connection on behalf of which the message will be pinned
    #[must_use]
    pub fn business_connection_id<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.business_connection_id = Some(val.into());
        this
    }

    /// Unique identifier of the business connection on behalf of which the message will be pinned
    #[must_use]
    pub fn business_connection_id_option<T: Into<Box<str>>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.business_connection_id = val.map(Into::into);
        this
    }

    /// Unique identifier for the target chat or username of the target channel in the format @username
    #[must_use]
    pub fn chat_id<T: Into<crate::types::ChatIdKind>>(self, val: T) -> Self {
        let mut this = self;
        this.chat_id = val.into();
        this
    }

    /// Identifier of a message to pin
    #[must_use]
    pub fn message_id<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.message_id = val.into();
        this
    }

    /// Pass `true` if it is not necessary to send a notification to all chat members about the new pinned message. Notifications are always disabled in channels and private chats.
    #[must_use]
    pub fn disable_notification<T: Into<bool>>(self, val: T) -> Self {
        let mut this = self;
        this.disable_notification = Some(val.into());
        this
    }

    /// Pass `true` if it is not necessary to send a notification to all chat members about the new pinned message. Notifications are always disabled in channels and private chats.
    #[must_use]
    pub fn disable_notification_option<T: Into<bool>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.disable_notification = val.map(Into::into);
        this
    }
}
impl super::TelegramMethod for PinChatMessage {
    type Method = Self;
    type Return = bool;

    fn build_request<Client>(self, _bot: &Bot<Client>) -> super::Request<Self::Method> {
        super::Request::new("pinChatMessage", self, None)
    }
}
