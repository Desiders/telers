use super::base::{prepare_file_with_value, Request, TelegramMethod};

use crate::{client::Bot, types::InputFile};

use serde::Serialize;
use serde_with::skip_serializing_none;
use std::collections::HashMap;

/// Use this method to set the thumbnail of a sticker set. Animated thumbnails can be set for animated sticker sets only. Video thumbnails can be set only for video sticker sets only.
/// # Documentation
/// <https://core.telegram.org/bots/api#setstickersetthumb>
/// # Returns
/// Returns `True` on success
#[skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize)]
pub struct SetStickerSetThumb<'a> {
    /// Sticker set name
    pub name: String,
    /// User identifier of the sticker set owner
    pub user_id: i64,
    /// A *PNG* image with the thumbnail, must be up to 128 kilobytes in size and have width and height exactly 100px, or a *TGS* animation with the thumbnail up to 32 kilobytes in size; see https://core.telegram.org/stickers#animated-sticker-requirements for animated sticker technical requirements, or a *WEBM* video with the thumbnail up to 32 kilobytes in size; see https://core.telegram.org/stickers#video-sticker-requirements for video sticker technical requirements. Pass a `file_id` as a String to send a file that already exists on the Telegram servers, pass an HTTP URL as a String for Telegram to get a file from the Internet, or upload a new one using multipart/form-data. [More info on Sending Files »](https://core.telegram.org/bots/api#sending-files). Animated sticker set thumbnails can't be uploaded via HTTP URL.
    pub thumb: Option<InputFile<'a>>,
}

impl<'a> SetStickerSetThumb<'a> {
    #[must_use]
    pub fn new<S: Into<String>>(name: S, user_id: i64) -> Self {
        Self {
            name: name.into(),
            user_id,
            thumb: None,
        }
    }

    #[must_use]
    pub fn name<T: Into<String>>(mut self, val: T) -> Self {
        self.name = val.into();
        self
    }

    #[must_use]
    pub fn user_id(mut self, val: i64) -> Self {
        self.user_id = val;
        self
    }

    #[must_use]
    pub fn thumb<T: Into<InputFile<'a>>>(mut self, val: T) -> Self {
        self.thumb = Some(val.into());
        self
    }
}

impl TelegramMethod for SetStickerSetThumb<'_> {
    type Method = Self;
    type Return = bool;

    fn build_request<Client>(&self, _bot: &Bot<Client>) -> Request<Self::Method> {
        let mut files = HashMap::new();
        if let Some(thumb) = &self.thumb {
            prepare_file_with_value(&mut files, thumb, "thumb");
        }

        Request::new("setStickerSetThumb", self, Some(files))
    }
}
