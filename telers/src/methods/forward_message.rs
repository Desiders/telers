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
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize)]
pub struct ForwardMessage {
    /// Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
    pub chat_id: ChatIdKind,
    /// Unique identifier for the target message thread (topic) of a forum; for forum supergroups and private chats of bots with forum topic mode enabled only
    pub message_thread_id: Option<i64>,
    /// Identifier of the direct messages topic to which the message will be forwarded; required if the message is forwarded to a direct messages chat
    pub direct_messages_topic_id: Option<i64>,
    /// Unique identifier for the chat where the original message was sent (or channel username in the format `@channelusername`)
    pub from_chat_id: ChatIdKind,
    /// New start timestamp for the forwarded video in the message
    pub video_start_timestamp: Option<i64>,
    /// Sends the message [silently](https://telegram.org/blog/channels-2-0#silent-messages). Users will receive a notification with no sound.
    pub disable_notification: Option<bool>,
    /// Protects the contents of the sent message from forwarding and saving
    pub protect_content: Option<bool>,
    /// Message identifier in the chat specified in `from_chat_id`
    pub message_id: i64,
}

impl ForwardMessage {
    #[must_use]
    pub fn new(
        chat_id: impl Into<ChatIdKind>,
        from_chat_id: impl Into<ChatIdKind>,
        message_id: i64,
    ) -> Self {
        Self {
            chat_id: chat_id.into(),
            message_thread_id: None,
            direct_messages_topic_id: None,
            from_chat_id: from_chat_id.into(),
            video_start_timestamp: None,
            message_id,
            disable_notification: None,
            protect_content: None,
        }
    }

    #[must_use]
    pub fn chat_id(self, val: impl Into<ChatIdKind>) -> Self {
        Self {
            chat_id: val.into(),
            ..self
        }
    }

    #[must_use]
    pub fn message_thread_id(self, val: i64) -> Self {
        Self {
            message_thread_id: Some(val),
            ..self
        }
    }

    #[must_use]
    pub fn direct_messages_topic_id(self, val: i64) -> Self {
        Self {
            direct_messages_topic_id: Some(val),
            ..self
        }
    }

    #[must_use]
    pub fn from_chat_id(self, val: impl Into<ChatIdKind>) -> Self {
        Self {
            from_chat_id: val.into(),
            ..self
        }
    }

    #[must_use]
    pub fn video_start_timestamp(self, val: i64) -> Self {
        Self {
            video_start_timestamp: Some(val),
            ..self
        }
    }

    #[must_use]
    pub fn disable_notification(self, val: bool) -> Self {
        Self {
            disable_notification: Some(val),
            ..self
        }
    }

    #[must_use]
    pub fn protect_content(self, val: bool) -> Self {
        Self {
            protect_content: Some(val),
            ..self
        }
    }

    #[must_use]
    pub fn message_id(self, val: i64) -> Self {
        Self {
            message_id: val,
            ..self
        }
    }
}

impl ForwardMessage {
    #[must_use]
    pub fn message_thread_id_option(self, val: Option<i64>) -> Self {
        Self {
            message_thread_id: val,
            ..self
        }
    }

    #[must_use]
    pub fn direct_messages_topic_id_option(self, val: Option<i64>) -> Self {
        Self {
            direct_messages_topic_id: val,
            ..self
        }
    }

    #[must_use]
    pub fn video_start_timestamp_option(self, val: Option<i64>) -> Self {
        Self {
            video_start_timestamp: val,
            ..self
        }
    }

    #[must_use]
    pub fn disable_notification_option(self, val: Option<bool>) -> Self {
        Self {
            disable_notification: val,
            ..self
        }
    }

    #[must_use]
    pub fn protect_content_option(self, val: Option<bool>) -> Self {
        Self {
            protect_content: val,
            ..self
        }
    }
}

impl TelegramMethod for ForwardMessage {
    type Method = Self;
    type Return = Message;

    fn build_request<Client>(self, _bot: &Bot<Client>) -> Request<Self::Method> {
        Request::new("forwardMessage", self, None)
    }
}

impl AsRef<ForwardMessage> for ForwardMessage {
    fn as_ref(&self) -> &Self {
        self
    }
}
