use super::base::{prepare_file, Request, TelegramMethod};

use crate::{
    client::Bot,
    types::{ChatIdKind, InputFile, Message, MessageEntity, ReplyMarkup},
};

use serde::Serialize;
use serde_with::skip_serializing_none;

/// Use this method to send animation files (GIF or H.264/MPEG-4 AVC video without sound). Bots can currently send animation files of up to 50 MB in size, this limit may be changed in the future.
/// # Documentation
/// <https://core.telegram.org/bots/api#sendanimation>
/// # Returns
/// On success, the sent [`Message`] is returned
#[skip_serializing_none]
#[derive(Debug, Clone, Hash, PartialEq, Serialize)]
pub struct SendAnimation<'a> {
    /// Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
    pub chat_id: ChatIdKind,
    /// Unique identifier for the target message thread (topic) of the forum; for forum supergroups only
    pub message_thread_id: Option<i64>,
    /// Animation to send. Pass a `file_id` as String to send a animation that exists on the Telegram servers (recommended), pass an HTTP URL as a String for Telegram to get a Animation from the Internet, or upload a new one using `multipart/form-data`. See [`more information on Sending Files`](https://core.telegram.org/bots/api#sending-files).
    pub animation: InputFile<'a>,
    /// Duration of sent animation in seconds
    pub duration: Option<i64>,
    /// Animation width
    pub width: Option<i64>,
    /// Animation height
    pub height: Option<i64>,
    /// Thumbnail of the file sent; can be ignored if thumbnail generation for the file is supported server-side. The thumbnail should be in JPEG format and less than 200 kB in size. A thumbnail's width and height should not exceed 320. Ignored if the file is not uploaded using `multipart/form-data`. Thumbnails can't be reused and can be only uploaded as a new file, so you can pass `attach://<file_attach_name>` if the thumbnail was uploaded using `multipart/form-data` under <file_attach_name>. [`More information on Sending Files`](https://core.telegram.org/bots/api#sending-files).
    pub thumbnail: Option<InputFile<'a>>,
    /// Animation caption (may also be used when resending animations by `file_id`), 0-1024 characters after entities parsing
    pub caption: Option<String>,
    /// Mode for parsing entities in the animation caption. See [`formatting options`](https://core.telegram.org/bots/api#formatting-options) for more details.
    pub parse_mode: Option<String>,
    /// A JSON-serialized list of special entities that appear in the caption, which can be specified instead of `parse_mode`
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// Pass `True` if the animation needs to be covered with a spoiler animation
    pub has_spoiler: Option<bool>,
    /// Pass `True`, if the uploaded animation is suitable for streaming
    pub supports_streaming: Option<bool>,
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

impl<'a> SendAnimation<'a> {
    #[must_use]
    pub fn new(chat_id: impl Into<ChatIdKind>, animation: impl Into<InputFile<'a>>) -> Self {
        Self {
            chat_id: chat_id.into(),
            message_thread_id: None,
            animation: animation.into(),
            duration: None,
            width: None,
            height: None,
            thumbnail: None,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            has_spoiler: None,
            supports_streaming: None,
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
    pub fn animation(self, val: impl Into<InputFile<'a>>) -> Self {
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
    pub fn thumb(self, val: impl Into<InputFile<'a>>) -> Self {
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

impl<'a> SendAnimation<'a> {
    #[must_use]
    pub fn message_thread_id_option(self, val: Option<i64>) -> Self {
        Self {
            message_thread_id: val,
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
    pub fn thumbnail_option(self, val: Option<impl Into<InputFile<'a>>>) -> Self {
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

impl<'a> TelegramMethod for SendAnimation<'a> {
    type Method = Self;
    type Return = Message;

    fn build_request<Client>(&self, _bot: &Bot<Client>) -> Request<Self::Method> {
        let mut files = vec![];
        prepare_file(&mut files, &self.animation);

        if let Some(file) = &self.thumbnail {
            prepare_file(&mut files, file);
        }

        Request::new("sendAnimation", self, Some(files.into()))
    }
}

impl<'a> AsRef<SendAnimation<'a>> for SendAnimation<'a> {
    fn as_ref(&self) -> &Self {
        self
    }
}
