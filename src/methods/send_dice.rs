use super::base::{Request, TelegramMethod};

use crate::{
    client::Bot,
    types::{ChatIdKind, Message, ReplyMarkup},
};

use serde::Serialize;
use serde_with::skip_serializing_none;

/// Use this method to send an animated emoji that will display a random value.
/// # Documentation
/// <https://core.telegram.org/bots/api#senddice>
/// # Returns
/// On success, the sent [`Message`] is returned
#[skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize)]
pub struct SendDice {
    /// Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
    pub chat_id: ChatIdKind,
    /// Unique identifier for the target message thread (topic) of the forum; for forum supergroups only
    pub message_thread_id: Option<i64>,
    /// Emoji on which the dice throw animation is based. Currently, must be one of `🎲`, `🎯`, `🏀`, `⚽`, `🎳`, or `🎰`. Dice can have values 1-6 for `🎲`, `🎯` and `🎳`, values 1-5 for `🏀` and `⚽`, and values 1-64 for `🎰`. Defaults to `🎲`
    pub emoji: Option<String>,
    /// Sends the message [silently](https://telegram.org/blog/channels-2-0#silent-messages). Users will receive a notification with no sound
    pub disable_notification: Option<bool>,
    /// Protects the contents of the sent message from forwarding and saving
    pub protect_content: Option<bool>,
    /// If the message is a reply, ID of the original message
    pub reply_to_message_id: Option<i64>,
    /// Pass `True`, if the message should be sent even if the specified replied-to message is not found
    pub allow_sending_without_reply: Option<bool>,
    /// Additional interface options. A JSON-serialized object for an [inline keyboard](https://core.telegram.org/bots/features#inline-keyboards), [custom reply keyboard](https://core.telegram.org/bots/features#keyboards), instructions to remove reply keyboard or to force a reply from the user.
    pub reply_markup: Option<ReplyMarkup>,
}

impl SendDice {
    #[must_use]
    pub fn new(chat_id: impl Into<ChatIdKind>) -> Self {
        Self {
            chat_id: chat_id.into(),
            message_thread_id: None,
            emoji: None,
            disable_notification: None,
            protect_content: None,
            reply_to_message_id: None,
            allow_sending_without_reply: None,
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
    pub fn emoji(self, val: impl Into<String>) -> Self {
        Self {
            emoji: Some(val.into()),
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
    pub fn reply_to_message_id(self, val: i64) -> Self {
        Self {
            reply_to_message_id: Some(val),
            ..self
        }
    }

    #[must_use]
    pub fn allow_sending_without_reply(self, val: bool) -> Self {
        Self {
            allow_sending_without_reply: Some(val),
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

impl SendDice {
    #[must_use]
    pub fn message_thread_id_option(self, val: Option<i64>) -> Self {
        Self {
            message_thread_id: val,
            ..self
        }
    }

    #[must_use]
    pub fn emoji_option(self, val: Option<impl Into<String>>) -> Self {
        Self {
            emoji: val.map(Into::into),
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
    pub fn reply_to_message_id_option(self, val: Option<i64>) -> Self {
        Self {
            reply_to_message_id: val,
            ..self
        }
    }

    #[must_use]
    pub fn allow_sending_without_reply_option(self, val: Option<bool>) -> Self {
        Self {
            allow_sending_without_reply: val,
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

impl TelegramMethod for SendDice {
    type Method = Self;
    type Return = Message;

    fn build_request<Client>(&self, _bot: &Bot<Client>) -> Request<Self::Method> {
        Request::new("sendDice", self, None)
    }
}
