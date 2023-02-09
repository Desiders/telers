use super::base::{prepare_file_with_value, Request, TelegramMethod};

use crate::{
    client::Bot,
    types::{InputFile, MaskPosition},
};

use serde::Serialize;
use serde_with::skip_serializing_none;
use std::collections::HashMap;

/// Use this method to add a new sticker to a set created by the bot. \
/// You **must** use exactly one of the fields `png_sticker`, `tgs_sticker`, or `webm_sticker`. Animated stickers can be added to animated sticker sets and only to them. Animated sticker sets can have up to 50 stickers. Static sticker sets can have up to 120 stickers.
/// # Documentation
/// <https://core.telegram.org/bots/api#addstickertoset>
/// # Returns
/// `True` on success
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AddStickerToSet<'a> {
    /// User identifier of sticker set owner
    pub user_id: i64,
    /// Sticker set name
    pub name: String,
    /// **PNG** image with the sticker, must be up to 512 kilobytes in size, dimensions must not exceed 512px, and either width or height must be exactly 512px. Pass a `file_id` as a String to send a file that already exists on the Telegram servers, pass an HTTP URL as a String for Telegram to get a file from the Internet, or upload a new one using `multipart/form-data`. [More information on Sending Files](https://core.telegram.org/bots/api#sending-files)
    pub png_sticker: Option<InputFile<'a>>,
    /// TGS animation with the sticker, uploaded using `multipart/form-data`. See <https://core.telegram.org/stickers#animated-sticker-requirements> for technical requirements
    pub tgs_sticker: Option<InputFile<'a>>,
    /// WEBM video with the sticker, uploaded using `multipart/form-data`. See <https://core.telegram.org/stickers#video-sticker-requirements> for technical requirements
    pub webm_sticker: Option<InputFile<'a>>,
    /// One or more emoji corresponding to the sticker
    pub emojis: String,
    /// A JSON-serialized object for position where the mask should be placed on faces
    pub mask_position: Option<MaskPosition>,
}

impl<'a> AddStickerToSet<'a> {
    #[must_use]
    pub fn new<T: Into<String>>(user_id: i64, name: T, emojis: T) -> Self {
        Self {
            user_id,
            name: name.into(),
            png_sticker: None,
            tgs_sticker: None,
            webm_sticker: None,
            emojis: emojis.into(),
            mask_position: None,
        }
    }

    #[must_use]
    pub fn user_id(mut self, val: i64) -> Self {
        self.user_id = val;
        self
    }

    #[must_use]
    pub fn name<T: Into<String>>(mut self, val: T) -> Self {
        self.name = val.into();
        self
    }

    #[must_use]
    pub fn png_sticker(mut self, val: InputFile<'a>) -> Self {
        self.png_sticker = Some(val);
        self
    }

    #[must_use]
    pub fn tgs_sticker(mut self, val: InputFile<'a>) -> Self {
        self.tgs_sticker = Some(val);
        self
    }

    #[must_use]
    pub fn webm_sticker(mut self, val: InputFile<'a>) -> Self {
        self.webm_sticker = Some(val);
        self
    }

    #[must_use]
    pub fn emojis<T: Into<String>>(mut self, val: T) -> Self {
        self.emojis = val.into();
        self
    }

    #[must_use]
    pub fn mask_position(mut self, val: MaskPosition) -> Self {
        self.mask_position = Some(val);
        self
    }
}

impl<'a> TelegramMethod for AddStickerToSet<'a> {
    type Method = Self;
    type Return = bool;

    fn build_request<Client>(&self, _bot: &Bot<Client>) -> Request<Self::Method> {
        let mut files = HashMap::new();

        if let Some(file) = &self.png_sticker {
            prepare_file_with_value(&mut files, file, "png_sticker");
        }
        if let Some(file) = &self.tgs_sticker {
            prepare_file_with_value(&mut files, file, "tgs_sticker");
        }
        if let Some(file) = &self.webm_sticker {
            prepare_file_with_value(&mut files, file, "webm_sticker");
        }

        Request::new("addStickerToSet", self, Some(files))
    }
}
