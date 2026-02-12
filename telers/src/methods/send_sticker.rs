use super::base::{prepare_file, Request, TelegramMethod};

use crate::{
    client::Bot,
    types::{
        ChatIdKind, InputFile, Message, ReplyMarkup, ReplyParameters, SuggestedPostParameters,
    },
};

use serde::Serialize;
use serde_with::skip_serializing_none;

/// Use this method to send static .WEBP, [animated](https://telegram.org/blog/animated-stickers) .TGS, or [video](https://telegram.org/blog/video-stickers-better-reactions) .WEBM stickers.
/// # Documentation
/// <https://core.telegram.org/bots/api#sendsticker>
/// # Returns
/// On success, the sent [`Message`] is returned
#[skip_serializing_none]
#[derive(Debug, Clone, Hash, PartialEq, Serialize)]
pub struct SendSticker<'a> {
    /// Unique identifier of the business connection on behalf of which the message will be sent
    pub business_connection_id: Option<String>,
    /// Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
    pub chat_id: ChatIdKind,
    /// Unique identifier for the target message thread (topic) of a forum; for forum supergroups and private chats of bots with forum topic mode enabled only
    pub message_thread_id: Option<i64>,
    /// Identifier of the direct messages topic to which the message will be sent; required if the message is sent to a direct messages chat
    pub direct_messages_topic_id: Option<i64>,
    /// Sticker to send. Pass a `file_id` as String to send a file that exists on the Telegram servers (recommended), pass an HTTP URL as a String for Telegram to get a .WEBP sticker from the Internet, or upload a new .WEBP, .TGS, or .WEBM sticker using multipart/form-data. [More info on Sending Files »](https://core.telegram.org/bots/api#sending-files). Video and animated stickers can't be sent via an HTTP URL.
    pub sticker: InputFile<'a>,
    /// Emoji associated with the sticker; only for just uploaded stickers
    pub emoji: Option<String>,
    /// Sends the message [silently](https://telegram.org/blog/channels-2-0#silent-messages). Users will receive a notification with no sound
    pub disable_notification: Option<bool>,
    /// Protects the contents of the sent message from forwarding and saving
    pub protect_content: Option<bool>,
    /// Pass `true` to allow up to 1000 messages per second, ignoring [broadcasting limits](https://core.telegram.org/bots/faq#how-can-i-message-all-of-my-bot-39s-subscribers-at-once) for a fee of 0.1 Telegram Stars per message. The relevant Stars will be withdrawn from the bot's balance
    pub allow_paid_broadcast: Option<bool>,
    /// Unique identifier of the message effect to be added to the message; for private chats only
    pub message_effect_id: Option<String>,
    /// A JSON-serialized object containing the parameters of the suggested post to send; for direct messages chats only. If the message is sent as a reply to another suggested post, then that suggested post is automatically declined.
    pub suggested_post_parameters: Option<SuggestedPostParameters>,
    /// Description of the message to reply to
    pub reply_parameters: Option<ReplyParameters>,
    /// Additional interface options. A JSON-serialized object for an [inline keyboard](https://core.telegram.org/bots/features#inline-keyboards), [custom reply keyboard](https://core.telegram.org/bots/features#keyboards), instructions to remove reply keyboard or to force a reply from the user.
    pub reply_markup: Option<ReplyMarkup>,
}

impl<'a> SendSticker<'a> {
    #[must_use]
    pub fn new(chat_id: impl Into<ChatIdKind>, sticker: impl Into<InputFile<'a>>) -> Self {
        Self {
            business_connection_id: None,
            chat_id: chat_id.into(),
            message_thread_id: None,
            direct_messages_topic_id: None,
            sticker: sticker.into(),
            emoji: None,
            disable_notification: None,
            protect_content: None,
            allow_paid_broadcast: None,
            message_effect_id: None,
            suggested_post_parameters: None,
            reply_parameters: None,
            reply_markup: None,
        }
    }

    #[must_use]
    pub fn business_connection_id(self, val: impl Into<String>) -> Self {
        Self {
            business_connection_id: Some(val.into()),
            ..self
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
    pub fn sticker(self, val: impl Into<InputFile<'a>>) -> Self {
        Self {
            sticker: val.into(),
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
    pub fn allow_paid_broadcast(self, val: bool) -> Self {
        Self {
            allow_paid_broadcast: Some(val),
            ..self
        }
    }

    #[must_use]
    pub fn message_effect_id(self, val: impl Into<String>) -> Self {
        Self {
            message_effect_id: Some(val.into()),
            ..self
        }
    }

    #[must_use]
    pub fn suggested_post_parameters(self, val: SuggestedPostParameters) -> Self {
        Self {
            suggested_post_parameters: Some(val),
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

impl SendSticker<'_> {
    #[must_use]
    pub fn business_connection_id_option(self, val: Option<impl Into<String>>) -> Self {
        Self {
            business_connection_id: val.map(Into::into),
            ..self
        }
    }

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
    pub fn allow_paid_broadcast_option(self, val: Option<bool>) -> Self {
        Self {
            allow_paid_broadcast: val,
            ..self
        }
    }

    #[must_use]
    pub fn message_effect_id_option(self, val: Option<impl Into<String>>) -> Self {
        Self {
            message_effect_id: val.map(Into::into),
            ..self
        }
    }

    #[must_use]
    pub fn suggested_post_parameters_option(self, val: Option<SuggestedPostParameters>) -> Self {
        Self {
            suggested_post_parameters: val,
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

impl TelegramMethod for SendSticker<'_> {
    type Method = Self;
    type Return = Message;

    fn build_request<Client>(&self, _bot: &Bot<Client>) -> Request<'_, Self::Method> {
        let mut files = vec![];
        prepare_file(&mut files, &self.sticker);

        Request::new("sendSticker", self, Some(files.into()))
    }
}

impl<'a> AsRef<SendSticker<'a>> for SendSticker<'a> {
    fn as_ref(&self) -> &Self {
        self
    }
}
