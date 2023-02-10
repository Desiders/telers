use super::base::{Request, TelegramMethod};

use crate::{
    client::Bot,
    types::{ChatIdKind, Message},
};

use serde::Serialize;
use serde_with::skip_serializing_none;

/// Use this method to forward messages of any kind. Service messages can't be forwarded.
/// # Documentation
/// <https://core.telegram.org/bots/api#forwardmessage>
/// # Returns
/// On success, the sent [`Message`] is returned
#[skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize)]
pub struct ForwardMessage {
    /// Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
    pub chat_id: ChatIdKind,
    /// Unique identifier for the target message thread (topic) of the forum; for forum supergroups only
    pub message_thread_id: Option<i64>,
    /// Unique identifier for the chat where the original message was sent (or channel username in the format `@channelusername`)
    pub from_chat_id: ChatIdKind,
    /// Message identifier in the chat specified in `from_chat_id`
    pub message_id: i64,
    /// Sends the message [silently](https://telegram.org/blog/channels-2-0#silent-messages). Users will receive a notification with no sound
    pub disable_notification: bool,
    /// Protects the contents of the sent message from forwarding and saving
    pub protect_content: bool,
}

impl ForwardMessage {
    #[must_use]
    pub fn new<T: Into<ChatIdKind>>(
        chat_id: T,
        from_chat_id: T,
        message_id: i64,
        disable_notification: bool,
        protect_content: bool,
    ) -> Self {
        Self {
            chat_id: chat_id.into(),
            message_thread_id: None,
            from_chat_id: from_chat_id.into(),
            message_id,
            disable_notification,
            protect_content,
        }
    }

    #[must_use]
    pub fn chat_id<T: Into<ChatIdKind>>(mut self, val: T) -> Self {
        self.chat_id = val.into();
        self
    }

    #[must_use]
    pub fn message_thread_id(mut self, val: i64) -> Self {
        self.message_thread_id = Some(val);
        self
    }

    #[must_use]
    pub fn from_chat_id<T: Into<ChatIdKind>>(mut self, val: T) -> Self {
        self.from_chat_id = val.into();
        self
    }

    #[must_use]
    pub fn message_id(mut self, val: i64) -> Self {
        self.message_id = val;
        self
    }

    #[must_use]
    pub fn disable_notification(mut self, val: bool) -> Self {
        self.disable_notification = val;
        self
    }

    #[must_use]
    pub fn protect_content(mut self, val: bool) -> Self {
        self.protect_content = val;
        self
    }
}

impl TelegramMethod for ForwardMessage {
    type Method = Self;
    type Return = Message;

    fn build_request<Client>(&self, _bot: &Bot<Client>) -> Request<Self::Method> {
        Request::new("forwardMessage", self, None)
    }
}
