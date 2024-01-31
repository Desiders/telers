use super::base::{Request, TelegramMethod};

use crate::{
    client::Bot,
    types::{ChatIdKind, MessageEntity, MessageId, ReplyMarkup, ReplyParameters},
};

use serde::Serialize;
use serde_with::skip_serializing_none;

/// Use this method to copy messages of any kind. Service messages and invoice messages can't be copied. A quiz [`poll`](crate::types::Poll) can be copied only if the value of the field `correct_option_id` is known to the bot. The method is analogous to the method [`ForwardMessage`](crate::methods::ForwardMessage), but the copied message doesn't have a link to the original message.
/// # Documentation
/// <https://core.telegram.org/bots/api#copymessage>
/// # Returns
/// Returns the [`MessageId`] of the sent message on success
#[skip_serializing_none]
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize)]
pub struct CopyMessage {
    /// Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
    pub chat_id: ChatIdKind,
    /// Unique identifier for the target message thread (topic) of the forum; for forum supergroups only
    pub message_thread_id: Option<i64>,
    /// Unique identifier for the chat where the original message was sent (or channel username in the format `@channelusername`)
    pub from_chat_id: ChatIdKind,
    /// Message identifier in the chat specified in `from_chat_id`
    pub message_id: i64,
    /// New caption for media, 0-1024 characters after entities parsing. If not specified, the original caption is kept
    pub caption: Option<String>,
    /// Mode for parsing entities in the new caption. See [formatting options](https://core.telegram.org/bots/api#formatting-options) for more details.
    pub parse_mode: Option<String>,
    /// A JSON-serialized list of special entities that appear in the new caption, which can be specified instead of `parse_mode`
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// Sends the message [silently](https://telegram.org/blog/channels-2-0#silent-messages). Users will receive a notification with no sound
    pub disable_notification: Option<bool>,
    /// Protects the contents of the sent message from forwarding and saving
    pub protect_content: Option<bool>,
    /// Description of the message to reply to
    pub reply_parameters: Option<ReplyParameters>,
    /// Additional interface options. A JSON-serialized object for an [inline keyboard](https://core.telegram.org/bots/features#inline-keyboards), [custom reply keyboard](https://core.telegram.org/bots/features#keyboards), instructions to remove reply keyboard or to force a reply from the user.
    pub reply_markup: Option<ReplyMarkup>,
}

impl CopyMessage {
    #[must_use]
    pub fn new(
        chat_id: impl Into<ChatIdKind>,
        from_chat_id: impl Into<ChatIdKind>,
        message_id: i64,
    ) -> Self {
        Self {
            chat_id: chat_id.into(),
            message_thread_id: None,
            from_chat_id: from_chat_id.into(),
            message_id,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            disable_notification: None,
            protect_content: None,
            reply_parameters: None,
            reply_markup: None,
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
            message_id: val,
            ..self
        }
    }

    #[must_use]
    pub fn caption(self, val: impl Into<String>) -> Self {
        Self {
            caption: Some(val.into()),
            ..self
        }
    }

    #[must_use]
    pub fn parse_mode(self, val: impl Into<String>) -> Self {
        Self {
            parse_mode: Some(val.into()),
            ..self
        }
    }

    #[must_use]
    pub fn caption_entity(self, val: MessageEntity) -> Self {
        Self {
            caption_entities: Some(
                self.caption_entities
                    .unwrap_or_default()
                    .into_iter()
                    .chain(Some(val))
                    .collect(),
            ),
            ..self
        }
    }

    #[must_use]
    pub fn caption_entities(self, val: impl IntoIterator<Item = MessageEntity>) -> Self {
        Self {
            caption_entities: Some(
                self.caption_entities
                    .unwrap_or_default()
                    .into_iter()
                    .chain(val)
                    .collect(),
            ),
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
    pub fn reply_parameters(self, val: ReplyParameters) -> Self {
        Self {
            reply_parameters: Some(val),
            ..self
        }
    }

    #[must_use]
    pub fn reply_markup(self, val: impl Into<ReplyMarkup>) -> Self {
        Self {
            reply_markup: Some(val.into()),
            ..self
        }
    }
}

impl CopyMessage {
    #[must_use]
    pub fn message_thread_id_option(self, val: Option<i64>) -> Self {
        Self {
            message_thread_id: val,
            ..self
        }
    }

    #[must_use]
    pub fn caption_option(self, val: Option<impl Into<String>>) -> Self {
        Self {
            caption: val.map(Into::into),
            ..self
        }
    }

    #[must_use]
    pub fn parse_mode_option(self, val: Option<impl Into<String>>) -> Self {
        Self {
            parse_mode: val.map(Into::into),
            ..self
        }
    }

    #[must_use]
    pub fn caption_entities_option(
        self,
        val: Option<impl IntoIterator<Item = MessageEntity>>,
    ) -> Self {
        Self {
            caption_entities: val.map(|val| {
                self.caption_entities
                    .unwrap_or_default()
                    .into_iter()
                    .chain(val)
                    .collect()
            }),
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

    #[must_use]
    pub fn reply_parameters_option(self, val: Option<ReplyParameters>) -> Self {
        Self {
            reply_parameters: val,
            ..self
        }
    }

    #[must_use]
    pub fn reply_markup_option(self, val: Option<impl Into<ReplyMarkup>>) -> Self {
        Self {
            reply_markup: val.map(Into::into),
            ..self
        }
    }
}

impl TelegramMethod for CopyMessage {
    type Method = Self;
    type Return = MessageId;

    fn build_request<Client>(&self, _bot: &Bot<Client>) -> Request<Self::Method> {
        Request::new("copyMessage", self, None)
    }
}

impl AsRef<CopyMessage> for CopyMessage {
    fn as_ref(&self) -> &Self {
        self
    }
}
