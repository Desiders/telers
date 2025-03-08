use super::base::{prepare_input_paid_media_group, Request, TelegramMethod};

use crate::{
    client::Bot,
    types::{ChatIdKind, InputPaidMedia, Message, MessageEntity, ReplyMarkup, ReplyParameters},
};

use serde::Serialize;
use serde_with::skip_serializing_none;

/// Use this method to send paid media.
/// # Documentation
/// <https://core.telegram.org/bots/api#sendpaidmedia>
/// # Returns
/// On success, the sent [`Message`] is returned
#[skip_serializing_none]
#[derive(Debug, Clone, Hash, PartialEq, Serialize)]
pub struct SendPaidMedia<'a> {
    /// Unique identifier of the business connection on behalf of which the message will be sent
    pub business_connection_id: Option<String>,
    /// Unique identifier for the target chat or username of the target channel (in the format `@channelusername`). If the chat is a channel, all Telegram Star proceeds from this media will be credited to the chat's balance. Otherwise, they will be credited to the bot's balance.
    pub chat_id: ChatIdKind,
    /// The number of Telegram Stars that must be paid to buy access to the media
    pub star_count: i64,
    /// A JSON-serialized array describing the media to be sent; up to 10 items
    pub media: Vec<InputPaidMedia<'a>>,
    /// Bot-defined paid media payload, 0-128 bytes. This will not be displayed to the user, use it for your internal processes.
    pub payload: Option<String>,
    /// Media caption, 0-1024 characters after entities parsing
    pub caption: Option<String>,
    /// Mode for parsing entities in the media caption. See [`formatting options`](https://core.telegram.org/bots/api#formatting-options) for more details.
    pub parse_mode: Option<String>,
    /// A JSON-serialized list of special entities that appear in the caption, which can be specified instead of `parse_mode`
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// Pass `true`, if the caption must be shown above the message media
    pub show_caption_above_media: Option<bool>,
    /// Sends the message [silently](https://telegram.org/blog/channels-2-0#silent-messages). Users will receive a notification with no sound
    pub disable_notification: Option<bool>,
    /// Protects the contents of the sent message from forwarding and saving
    pub protect_content: Option<bool>,
    /// Pass `true` to allow up to 1000 messages per second, ignoring [broadcasting limits](https://core.telegram.org/bots/faq#how-can-i-message-all-of-my-bot-39s-subscribers-at-once) for a fee of 0.1 Telegram Stars per message. The relevant Stars will be withdrawn from the bot's balance
    pub allow_paid_broadcast: Option<bool>,
    /// Description of the message to reply to
    pub reply_parameters: Option<ReplyParameters>,
    /// Additional interface options. A JSON-serialized object for an [inline keyboard](https://core.telegram.org/bots/features#inline-keyboards), [custom reply keyboard](https://core.telegram.org/bots/features#keyboards), instructions to remove reply keyboard or to force a reply from the user.
    pub reply_markup: Option<ReplyMarkup>,
}

impl<'a> SendPaidMedia<'a> {
    #[must_use]
    pub fn new<T, I>(chat_id: impl Into<ChatIdKind>, star_count: i64, media: I) -> Self
    where
        T: Into<InputPaidMedia<'a>>,
        I: IntoIterator<Item = T>,
    {
        Self {
            business_connection_id: None,
            chat_id: chat_id.into(),
            star_count,
            media: media.into_iter().map(Into::into).collect(),
            payload: None,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            show_caption_above_media: None,
            disable_notification: None,
            protect_content: None,
            allow_paid_broadcast: None,
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
    pub fn star_count(self, val: i64) -> Self {
        Self {
            star_count: val,
            ..self
        }
    }

    #[must_use]
    pub fn media_single(self, val: impl Into<InputPaidMedia<'a>>) -> Self {
        Self {
            media: self.media.into_iter().chain(Some(val.into())).collect(),
            ..self
        }
    }

    #[must_use]
    pub fn media<T, I>(self, val: I) -> Self
    where
        T: Into<InputPaidMedia<'a>>,
        I: IntoIterator<Item = T>,
    {
        Self {
            media: self
                .media
                .into_iter()
                .chain(val.into_iter().map(Into::into))
                .collect(),
            ..self
        }
    }

    #[must_use]
    pub fn payload(self, val: impl Into<String>) -> Self {
        Self {
            payload: Some(val.into()),
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
    pub fn show_caption_above_media(self, val: bool) -> Self {
        Self {
            show_caption_above_media: Some(val),
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

impl SendPaidMedia<'_> {
    #[must_use]
    pub fn business_connection_id_option(self, val: Option<impl Into<String>>) -> Self {
        Self {
            business_connection_id: val.map(Into::into),
            ..self
        }
    }

    #[must_use]
    pub fn payload_option(self, val: Option<impl Into<String>>) -> Self {
        Self {
            payload: val.map(Into::into),
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
    pub fn show_caption_above_media_option(self, val: Option<bool>) -> Self {
        Self {
            show_caption_above_media: val,
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

impl TelegramMethod for SendPaidMedia<'_> {
    type Method = Self;
    type Return = Message;

    fn build_request<Client>(&self, _bot: &Bot<Client>) -> Request<Self::Method> {
        let mut files = vec![];
        prepare_input_paid_media_group(&mut files, &self.media);

        Request::new("sendPaidMedia", self, Some(files.into()))
    }
}

impl<'a> AsRef<SendPaidMedia<'a>> for SendPaidMedia<'a> {
    fn as_ref(&self) -> &Self {
        self
    }
}
