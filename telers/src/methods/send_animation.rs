use super::base::{prepare_file, Request, TelegramMethod};

use crate::{
    client::Bot,
    types::{
        ChatIdKind, InputFile, Message, MessageEntity, ReplyMarkup, ReplyParameters,
        SuggestedPostParameters,
    },
};

use serde::Serialize;
use serde_with::skip_serializing_none;

/// Use this method to send animation files (GIF or H.264/MPEG-4 AVC video without sound). Bots can currently send animation files of up to 50 MB in size, this limit may be changed in the future.
/// # Documentation
/// <https://core.telegram.org/bots/api#sendanimation>
/// # Returns
/// On success, the sent [`Message`] is returned
#[skip_serializing_none]
#[derive(Debug, Hash, PartialEq, Serialize)]
pub struct SendAnimation {
    /// Unique identifier of the business connection on behalf of which the message will be sent
    pub business_connection_id: Option<String>,
    /// Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
    pub chat_id: ChatIdKind,
    /// Unique identifier for the target message thread (topic) of a forum; for forum supergroups and private chats of bots with forum topic mode enabled only
    pub message_thread_id: Option<i64>,
    /// Identifier of the direct messages topic to which the message will be sent; required if the message is sent to a direct messages chat
    pub direct_messages_topic_id: Option<i64>,
    /// Animation to send. Pass a `file_id` as String to send a animation that exists on the Telegram servers (recommended), pass an HTTP URL as a String for Telegram to get a Animation from the Internet, or upload a new one using `multipart/form-data`. See [`more information on Sending Files`](https://core.telegram.org/bots/api#sending-files).
    pub animation: InputFile,
    /// Duration of sent animation in seconds
    pub duration: Option<i64>,
    /// Animation width
    pub width: Option<i64>,
    /// Animation height
    pub height: Option<i64>,
    /// Thumbnail of the file sent; can be ignored if thumbnail generation for the file is supported server-side. The thumbnail should be in JPEG format and less than 200 kB in size. A thumbnail's width and height should not exceed 320. Ignored if the file is not uploaded using `multipart/form-data`. Thumbnails can't be reused and can be only uploaded as a new file, so you can pass `attach://<file_attach_name>` if the thumbnail was uploaded using `multipart/form-data` under `<file_attach_name>`. [`More information on Sending Files`](https://core.telegram.org/bots/api#sending-files).
    pub thumbnail: Option<InputFile>,
    /// Animation caption (may also be used when resending animations by `file_id`), 0-1024 characters after entities parsing
    pub caption: Option<String>,
    /// Mode for parsing entities in the animation caption. See [`formatting options`](https://core.telegram.org/bots/api#formatting-options) for more details.
    pub parse_mode: Option<String>,
    /// A JSON-serialized list of special entities that appear in the caption, which can be specified instead of `parse_mode`
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// Pass `true`, if the caption must be shown above the message media
    pub show_caption_above_media: Option<bool>,
    /// Pass `true` if the animation needs to be covered with a spoiler animation
    pub has_spoiler: Option<bool>,
    /// Pass `true`, if the uploaded animation is suitable for streaming
    pub supports_streaming: Option<bool>,
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

impl SendAnimation {
    #[must_use]
    pub fn new(chat_id: impl Into<ChatIdKind>, animation: impl Into<InputFile>) -> Self {
        Self {
            business_connection_id: None,
            chat_id: chat_id.into(),
            message_thread_id: None,
            direct_messages_topic_id: None,
            animation: animation.into(),
            duration: None,
            width: None,
            height: None,
            thumbnail: None,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            show_caption_above_media: None,
            has_spoiler: None,
            supports_streaming: None,
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
    pub fn animation(self, val: impl Into<InputFile>) -> Self {
        Self {
            animation: val.into(),
            ..self
        }
    }

    #[must_use]
    pub fn duration(self, val: i64) -> Self {
        Self {
            duration: Some(val),
            ..self
        }
    }

    #[must_use]
    pub fn width(self, val: i64) -> Self {
        Self {
            width: Some(val),
            ..self
        }
    }

    #[must_use]
    pub fn height(self, val: i64) -> Self {
        Self {
            height: Some(val),
            ..self
        }
    }

    #[must_use]
    pub fn thumbnail(self, val: impl Into<InputFile>) -> Self {
        Self {
            thumbnail: Some(val.into()),
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
    pub fn has_spoiler(self, val: bool) -> Self {
        Self {
            has_spoiler: Some(val),
            ..self
        }
    }

    #[must_use]
    pub fn supports_streaming(self, val: bool) -> Self {
        Self {
            supports_streaming: Some(val),
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

impl SendAnimation {
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
    pub fn duration_option(self, val: Option<i64>) -> Self {
        Self {
            duration: val,
            ..self
        }
    }

    #[must_use]
    pub fn width_option(self, val: Option<i64>) -> Self {
        Self { width: val, ..self }
    }

    #[must_use]
    pub fn height_option(self, val: Option<i64>) -> Self {
        Self {
            height: val,
            ..self
        }
    }

    #[must_use]
    pub fn thumbnail_option(self, val: Option<impl Into<InputFile>>) -> Self {
        Self {
            thumbnail: val.map(Into::into),
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
    pub fn has_spoiler_option(self, val: Option<bool>) -> Self {
        Self {
            has_spoiler: val,
            ..self
        }
    }

    #[must_use]
    pub fn supports_streaming_option(self, val: Option<bool>) -> Self {
        Self {
            supports_streaming: val,
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

impl TelegramMethod for SendAnimation {
    type Method = Self;
    type Return = Message;

    fn build_request<Client>(mut self, _bot: &Bot<Client>) -> Request<Self::Method> {
        let mut files = vec![];
        prepare_file(&mut files, &mut self.animation);

        if let Some(file) = &mut self.thumbnail {
            prepare_file(&mut files, file);
        }

        Request::new("sendAnimation", self, Some(files))
    }
}

impl AsRef<SendAnimation> for SendAnimation {
    fn as_ref(&self) -> &Self {
        self
    }
}
