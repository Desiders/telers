use crate::client::Bot;
use serde::Serialize;
/// Use this method to remove a message from the list of pinned messages in a chat. In private chats and channel direct messages chats, all messages can be unpinned. Conversely, the bot must be an administrator with the '`can_pin_messages`' right or the '`can_edit_messages`' right to unpin messages in groups and channels respectively. Returns `true` on success.
/// # Documentation
/// <https://core.telegram.org/bots/api#unpinchatmessage>
/// # Returns
/// - `bool`
#[derive(Clone, Debug, Serialize)]
pub struct UnpinChatMessage {
    /// Unique identifier of the business connection on behalf of which the message will be unpinned
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<Box<str>>,
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: crate::types::ChatIdKind,
    /// Identifier of the message to unpin. Required if `business_connection_id` is specified. If not specified, the most recent pinned message (by sending date) will be unpinned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<i64>,
}
impl UnpinChatMessage {
    /// Creates a new `UnpinChatMessage`.
    ///
    /// # Arguments
    /// * `chat_id` - Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0: Into<crate::types::ChatIdKind>>(chat_id: T0) -> Self {
        Self {
            business_connection_id: None,
            chat_id: chat_id.into(),
            message_id: None,
        }
    }

    /// Unique identifier of the business connection on behalf of which the message will be unpinned
    #[must_use]
    pub fn business_connection_id<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.business_connection_id = Some(val.into());
        this
    }

    /// Unique identifier of the business connection on behalf of which the message will be unpinned
    #[must_use]
    pub fn business_connection_id_option<T: Into<Box<str>>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.business_connection_id = val.map(Into::into);
        this
    }

    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    #[must_use]
    pub fn chat_id<T: Into<crate::types::ChatIdKind>>(self, val: T) -> Self {
        let mut this = self;
        this.chat_id = val.into();
        this
    }

    /// Identifier of the message to unpin. Required if `business_connection_id` is specified. If not specified, the most recent pinned message (by sending date) will be unpinned.
    #[must_use]
    pub fn message_id<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.message_id = Some(val.into());
        this
    }

    /// Identifier of the message to unpin. Required if `business_connection_id` is specified. If not specified, the most recent pinned message (by sending date) will be unpinned.
    #[must_use]
    pub fn message_id_option<T: Into<i64>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.message_id = val.map(Into::into);
        this
    }
}
impl super::TelegramMethod for UnpinChatMessage {
    type Method = Self;
    type Return = bool;

    fn build_request<Client>(self, _bot: &Bot<Client>) -> super::Request<Self::Method> {
        super::Request::new("unpinChatMessage", self, None)
    }
}
