use super::base::{prepare_file_with_value, Request, TelegramMethod};

use crate::{
    client::Bot,
    types::{ChatIdKind, InputFile, Message, ReplyMarkup},
};

use serde::Serialize;
use serde_with::skip_serializing_none;
use std::collections::HashMap;

/// As of [v.4.0](https://telegram.org/blog/video-messages-and-telescope), Telegram clients support rounded square MPEG4 videos of up to 1 minute long. Use this method to send video messages.
/// # Documentation
/// <https://core.telegram.org/bots/api#sendvideonote>
/// # Returns
/// On success, the sent [`Message`] is returned
#[skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize)]
pub struct SendVideoNote<'a> {
    /// Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
    pub chat_id: ChatIdKind,
    /// Unique identifier for the target message thread (topic) of the forum; for forum supergroups only
    pub message_thread_id: Option<i64>,
    /// Video note to send. Pass a `file_id` as String to send a video note that exists on the Telegram servers (recommended) or upload a new video using `multipart/form-data`. See [`more information on Sending Files`](https://core.telegram.org/bots/api#sending-files). Sending video notes by a URL is currently unsupported.
    pub video_note: InputFile<'a>,
    /// Duration of sent video in seconds
    pub duration: Option<i64>,
    /// Video width and height, i.e. diameter of the video message
    pub length: Option<i64>,
    /// Thumbnail of the file sent; can be ignored if thumbnail generation for the file is supported server-side. The thumbnail should be in JPEG format and less than 200 kB in size. A thumbnail's width and height should not exceed 320. Ignored if the file is not uploaded using `multipart/form-data`. Thumbnails can't be reused and can be only uploaded as a new file, so you can pass `attach://<file_attach_name>` if the thumbnail was uploaded using `multipart/form-data` under <file_attach_name>. [`More information on Sending Files`](https://core.telegram.org/bots/api#sending-files).
    pub thumbnail: Option<InputFile<'a>>,
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

impl<'a> SendVideoNote<'a> {
    #[must_use]
    pub fn new(chat_id: impl Into<ChatIdKind>, video_note: impl Into<InputFile<'a>>) -> Self {
        Self {
            chat_id: chat_id.into(),
            message_thread_id: None,
            video_note: video_note.into(),
            duration: None,
            length: None,
            thumbnail: None,
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
    pub fn video_note(self, val: impl Into<InputFile<'a>>) -> Self {
        Self {
            video_note: val.into(),
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
    pub fn length(self, val: i64) -> Self {
        Self {
            length: Some(val),
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

impl<'a> SendVideoNote<'a> {
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
    pub fn length_option(self, val: Option<i64>) -> Self {
        Self {
            length: val,
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

impl<'a> TelegramMethod for SendVideoNote<'a> {
    type Method = Self;
    type Return = Message;

    fn build_request<Client>(&self, _bot: &Bot<Client>) -> Request<Self::Method> {
        let mut files = HashMap::new();
        prepare_file_with_value(&mut files, &self.video_note, "video_note");

        if let Some(file) = &self.thumbnail {
            prepare_file_with_value(&mut files, file, "thumbnail");
        }

        Request::new("sendVideoNote", self, Some(files))
    }
}
