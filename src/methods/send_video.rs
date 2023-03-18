use super::base::{prepare_file_with_value, Request, TelegramMethod};

use crate::{
    client::Bot,
    types::{ChatIdKind, InputFile, Message, MessageEntity, ReplyMarkup},
};

use serde::Serialize;
use serde_with::skip_serializing_none;
use std::collections::HashMap;

/// Use this method to send video files, Telegram clients support MPEG4 videos (other formats may be sent as [`crate::types::Document`]). Bots can currently send video files of up to 50 MB in size, this limit may be changed in the future.
/// # Documentation
/// <https://core.telegram.org/bots/api#sendvideo>
/// # Returns
/// On success, the sent [`Message`] is returned
#[skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize)]
pub struct SendVideo<'a> {
    /// Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
    pub chat_id: ChatIdKind,
    /// Unique identifier for the target message thread (topic) of the forum; for forum supergroups only
    pub message_thread_id: Option<i64>,
    /// Video to send. Pass a `file_id` as String to send a video that exists on the Telegram servers (recommended), pass an HTTP URL as a String for Telegram to get a video from the Internet, or upload a new one using `multipart/form-data`. See [`more information on Sending Files`](https://core.telegram.org/bots/api#sending-files).
    pub video: InputFile<'a>,
    /// Duration of sent video in seconds
    pub duration: Option<i64>,
    /// Video width
    pub width: Option<i64>,
    /// Video height
    pub height: Option<i64>,
    /// Thumbnail of the file sent; can be ignored if thumbnail generation for the file is supported server-side. The thumbnail should be in JPEG format and less than 200 kB in size. A thumbnail's width and height should not exceed 320. Ignored if the file is not uploaded using `multipart/form-data`. Thumbnails can't be reused and can be only uploaded as a new file, so you can pass `attach://<file_attach_name>` if the thumbnail was uploaded using `multipart/form-data` under <file_attach_name>. [`More information on Sending Files`](https://core.telegram.org/bots/api#sending-files).
    pub thumb: Option<InputFile<'a>>,
    /// Video caption (may also be used when resending videos by `file_id`), 0-1024 characters after entities parsing
    pub caption: Option<String>,
    /// Mode for parsing entities in the video caption. See [`formatting options`](https://core.telegram.org/bots/api#formatting-options) for more details.
    pub parse_mode: Option<String>,
    /// A JSON-serialized list of special entities that appear in the caption, which can be specified instead of `parse_mode`
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// Pass `True` if the video needs to be covered with a spoiler animation
    pub has_spoiler: Option<bool>,
    /// Pass `True`, if the uploaded video is suitable for streaming
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

impl<'a> SendVideo<'a> {
    #[must_use]
    pub fn new<C: Into<ChatIdKind>, A: Into<InputFile<'a>>>(chat_id: C, video: A) -> Self {
        Self {
            chat_id: chat_id.into(),
            message_thread_id: None,
            video: video.into(),
            duration: None,
            width: None,
            height: None,
            thumb: None,
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
    pub fn video<T: Into<InputFile<'a>>>(mut self, val: T) -> Self {
        self.video = val.into();
        self
    }

    #[must_use]
    pub fn duration(mut self, val: i64) -> Self {
        self.duration = Some(val);
        self
    }

    #[must_use]
    pub fn width(mut self, val: i64) -> Self {
        self.width = Some(val);
        self
    }

    #[must_use]
    pub fn height(mut self, val: i64) -> Self {
        self.height = Some(val);
        self
    }

    #[must_use]
    pub fn thumb<T: Into<InputFile<'a>>>(mut self, val: T) -> Self {
        self.thumb = Some(val.into());
        self
    }

    #[must_use]
    pub fn caption<T: Into<String>>(mut self, val: T) -> Self {
        self.caption = Some(val.into());
        self
    }

    #[must_use]
    pub fn parse_mode<T: Into<String>>(mut self, val: T) -> Self {
        self.parse_mode = Some(val.into());
        self
    }

    #[must_use]
    pub fn caption_entities(mut self, val: Vec<MessageEntity>) -> Self {
        self.caption_entities = Some(val);
        self
    }

    #[must_use]
    pub fn has_spoiler(mut self, val: bool) -> Self {
        self.has_spoiler = Some(val);
        self
    }

    #[must_use]
    pub fn supports_streaming(mut self, val: bool) -> Self {
        self.supports_streaming = Some(val);
        self
    }

    #[must_use]
    pub fn disable_notification(mut self, val: bool) -> Self {
        self.disable_notification = Some(val);
        self
    }

    #[must_use]
    pub fn protect_content(mut self, val: bool) -> Self {
        self.protect_content = Some(val);
        self
    }

    #[must_use]
    pub fn reply_to_message_id(mut self, val: i64) -> Self {
        self.reply_to_message_id = Some(val);
        self
    }

    #[must_use]
    pub fn allow_sending_without_reply(mut self, val: bool) -> Self {
        self.allow_sending_without_reply = Some(val);
        self
    }

    #[must_use]
    pub fn reply_markup<T: Into<ReplyMarkup>>(mut self, val: T) -> Self {
        self.reply_markup = Some(val.into());
        self
    }
}

impl<'a> SendVideo<'a> {
    #[must_use]
    pub fn message_thread_id_some(mut self, val: Option<i64>) -> Self {
        self.message_thread_id = val;
        self
    }

    #[must_use]
    pub fn duration_some(mut self, val: Option<i64>) -> Self {
        self.duration = val;
        self
    }

    #[must_use]
    pub fn width_some(mut self, val: Option<i64>) -> Self {
        self.width = val;
        self
    }

    #[must_use]
    pub fn height_some(mut self, val: Option<i64>) -> Self {
        self.height = val;
        self
    }

    #[must_use]
    pub fn thumb_some<T: Into<InputFile<'a>>>(mut self, val: Option<T>) -> Self {
        self.thumb = val.map(Into::into);
        self
    }

    #[must_use]
    pub fn caption_some<T: Into<String>>(mut self, val: Option<T>) -> Self {
        self.caption = val.map(Into::into);
        self
    }

    #[must_use]
    pub fn parse_mode_some<T: Into<String>>(mut self, val: Option<T>) -> Self {
        self.parse_mode = val.map(Into::into);
        self
    }

    #[must_use]
    pub fn caption_entities_some(mut self, val: Option<Vec<MessageEntity>>) -> Self {
        self.caption_entities = val;
        self
    }

    #[must_use]
    pub fn has_spoiler_some(mut self, val: Option<bool>) -> Self {
        self.has_spoiler = val;
        self
    }

    #[must_use]
    pub fn supports_streaming_some(mut self, val: Option<bool>) -> Self {
        self.supports_streaming = val;
        self
    }

    #[must_use]
    pub fn disable_notification_some(mut self, val: Option<bool>) -> Self {
        self.disable_notification = val;
        self
    }

    #[must_use]
    pub fn protect_content_some(mut self, val: Option<bool>) -> Self {
        self.protect_content = val;
        self
    }

    #[must_use]
    pub fn reply_to_message_id_some(mut self, val: Option<i64>) -> Self {
        self.reply_to_message_id = val;
        self
    }

    #[must_use]
    pub fn allow_sending_without_reply_some(mut self, val: Option<bool>) -> Self {
        self.allow_sending_without_reply = val;
        self
    }

    #[must_use]
    pub fn reply_markup_some<T: Into<ReplyMarkup>>(mut self, val: Option<T>) -> Self {
        self.reply_markup = val.map(Into::into);
        self
    }
}

impl<'a> TelegramMethod for SendVideo<'a> {
    type Method = Self;
    type Return = Message;

    fn build_request<Client>(&self, _bot: &Bot<Client>) -> Request<Self::Method> {
        let mut files = HashMap::new();
        prepare_file_with_value(&mut files, &self.video, "video");

        if let Some(file) = &self.thumb {
            prepare_file_with_value(&mut files, file, "thumb");
        }

        Request::new("sendVideo", self, Some(files))
    }
}
