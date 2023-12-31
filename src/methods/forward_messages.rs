use super::base::{Request, TelegramMethod};

use crate::{
    client::Bot,
    types::{ChatIdKind, MessageId},
};

use serde::Serialize;
use serde_with::skip_serializing_none;

/// Use this method to forward multiple messages of any kind. If some of the specified messages can't be found or forwarded, they are skipped. Service messages and messages with protected content can't be forwarded. Album grouping is kept for forwarded messages.
/// # Documentation
/// <https://core.telegram.org/bots/api#forwardmessages>
/// # Returns
/// On success, an array of [`MessageId`] of the sent messages is returned.
#[skip_serializing_none]
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize)]
pub struct ForwardMessages {
    /// Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
    pub chat_id: ChatIdKind,
    /// Unique identifier for the target message thread (topic) of the forum; for forum supergroups only
    pub message_thread_id: Option<i64>,
    /// Unique identifier for the chat where the original messages were sent (or channel username in the format `@channelusername`)
    pub from_chat_id: ChatIdKind,
    /// Identifiers of 1-100 messages in the chat `from_chat_id` to forward. The identifiers must be specified in a strictly increasing order.
    pub message_ids: Vec<i64>,
    /// Sends the messages [silently](https://telegram.org/blog/channels-2-0#silent-messages). Users will receive a notification with no sound.
    pub disable_notification: Option<bool>,
    /// Protects the contents of the forwarded messages from forwarding and saving
    pub protect_content: Option<bool>,
}

impl ForwardMessages {
    #[must_use]
    pub fn new(
        chat_id: impl Into<ChatIdKind>,
        from_chat_id: impl Into<ChatIdKind>,
        message_ids: impl IntoIterator<Item = i64>,
    ) -> Self {
        Self {
            chat_id: chat_id.into(),
            message_thread_id: None,
            from_chat_id: from_chat_id.into(),
            message_ids: message_ids.into_iter().collect(),
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
    pub fn from_chat_id(self, val: impl Into<ChatIdKind>) -> Self {
        Self {
            from_chat_id: val.into(),
            ..self
        }
    }

    #[must_use]
    pub fn message_id(self, val: i64) -> Self {
        Self {
            message_ids: self.message_ids.into_iter().chain(Some(val)).collect(),
            ..self
        }
    }

    #[must_use]
    pub fn message_ids(self, val: impl IntoIterator<Item = i64>) -> Self {
        Self {
            message_ids: self.message_ids.into_iter().chain(val).collect(),
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
}

impl ForwardMessages {
    #[must_use]
    pub fn message_thread_id_option(self, val: Option<i64>) -> Self {
        Self {
            message_thread_id: val,
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

impl TelegramMethod for ForwardMessages {
    type Method = Self;
    type Return = MessageId;

    fn build_request<Client>(&self, _bot: &Bot<Client>) -> Request<Self::Method> {
        Request::new("forwardMessages", self, None)
    }
}

impl AsRef<ForwardMessages> for ForwardMessages {
    fn as_ref(&self) -> &Self {
        self
    }
}
