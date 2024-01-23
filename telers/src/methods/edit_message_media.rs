use super::base::{prepare_input_media, Request, TelegramMethod};

use crate::{
    client::Bot,
    types::{ChatIdKind, InlineKeyboardMarkup, InputMedia, MessageOrTrue},
};

use serde::Serialize;
use serde_with::skip_serializing_none;

/// Use this method to edit animation, audio, document, photo, or video messages. If a message is part of a message album, then it can be edited only to an audio for audio albums, only to a document for document albums and to a photo or a video otherwise. When an inline message is edited, a new file can't be uploaded; use a previously uploaded file via its `file_id` or specify a URL.
/// # Documentation
/// <https://core.telegram.org/bots/api#editmessagemedia>
/// # Returns
/// On success, if the edited message is not an inline message, the edited [`crate::types::Message`] is returned, otherwise `true` is returned
#[skip_serializing_none]
#[derive(Debug, Clone, Hash, PartialEq, Serialize)]
pub struct EditMessageMedia<'a> {
    /// Required if `inline_message_id` is not specified. Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
    pub chat_id: Option<ChatIdKind>,
    /// Required if `inline_message_id` is not specified. Identifier of the message to edit
    pub message_id: Option<i64>,
    /// Required if `chat_id` and `message_id` are not specified. Identifier of the inline message
    pub inline_message_id: Option<String>,
    /// A JSON-serialized object for a new media content of the message
    pub media: InputMedia<'a>,
    /// A JSON-serialized object for a new [inline keyboard](https://core.telegram.org/bots/features#inline-keyboards).
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

impl<'a> EditMessageMedia<'a> {
    #[must_use]
    pub fn new(media: impl Into<InputMedia<'a>>) -> Self {
        Self {
            chat_id: None,
            message_id: None,
            inline_message_id: None,
            media: media.into(),
            reply_markup: None,
        }
    }

    #[must_use]
    pub fn chat_id(self, val: impl Into<ChatIdKind>) -> Self {
        Self {
            chat_id: Some(val.into()),
            ..self
        }
    }

    #[must_use]
    pub fn message_id(self, val: i64) -> Self {
        Self {
            message_id: Some(val),
            ..self
        }
    }

    #[must_use]
    pub fn inline_message_id(self, val: impl Into<String>) -> Self {
        Self {
            inline_message_id: Some(val.into()),
            ..self
        }
    }

    #[must_use]
    pub fn media(self, val: impl Into<InputMedia<'a>>) -> Self {
        Self {
            media: val.into(),
            ..self
        }
    }

    #[must_use]
    pub fn reply_markup(self, val: impl Into<InlineKeyboardMarkup>) -> Self {
        Self {
            reply_markup: Some(val.into()),
            ..self
        }
    }
}

impl<'a> EditMessageMedia<'a> {
    #[must_use]
    pub fn chat_id_option(self, val: Option<impl Into<ChatIdKind>>) -> Self {
        Self {
            chat_id: val.map(Into::into),
            ..self
        }
    }

    #[must_use]
    pub fn message_id_option(self, val: Option<i64>) -> Self {
        Self {
            message_id: val,
            ..self
        }
    }

    #[must_use]
    pub fn inline_message_id_option(self, val: Option<impl Into<String>>) -> Self {
        Self {
            inline_message_id: val.map(Into::into),
            ..self
        }
    }

    #[must_use]
    pub fn reply_markup_option(self, val: Option<impl Into<InlineKeyboardMarkup>>) -> Self {
        Self {
            reply_markup: val.map(Into::into),
            ..self
        }
    }
}

impl TelegramMethod for EditMessageMedia<'_> {
    type Method = Self;
    type Return = MessageOrTrue;

    fn build_request<Client>(&self, _bot: &Bot<Client>) -> Request<Self::Method> {
        let mut files = vec![];
        prepare_input_media(&mut files, &self.media);

        Request::new("editMessageMedia", self, Some(files.into()))
    }
}

impl<'a> AsRef<EditMessageMedia<'a>> for EditMessageMedia<'a> {
    fn as_ref(&self) -> &Self {
        self
    }
}
