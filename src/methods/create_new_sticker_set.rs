use super::base::{prepare_file_with_value, Request, TelegramMethod};

use crate::{
    client::Bot,
    types::{InputFile, MaskPosition},
};

use serde::Serialize;
use serde_with::skip_serializing_none;
use std::collections::HashMap;

/// Use this method to create a new sticker set owned by a user. The bot will be able to edit the sticker set thus created. You `must` use exactly one of the fields `png_sticker`, `tgs_sticker`, or `webm_sticker`.
/// # Documentation
/// <https://core.telegram.org/bots/api#createnewstickerset>
/// # Returns
/// Returns `True` on success
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CreateNewStickerSet<'a> {
    /// User identifier of created sticker set owner
    pub user_id: i64,
    /// Short name of sticker set, to be used in `t.me/addstickers/` URLs (e.g., animals). Can contain only english letters, digits and underscores. Must begin with a letter, can't contain consecutive underscores and must end in `_by_<bot username>`. `<bot_username>` is case insensitive. 1-64 characters.
    pub name: String,
    /// Sticker set title, 1-64 characters
    pub title: String,
    /// `PNG` image with the sticker, must be up to 512 kilobytes in size, dimensions must not exceed 512px, and either width or height must be exactly 512px. Pass a `file_id` as a String to send a file that already exists on the Telegram servers, pass an HTTP URL as a String for Telegram to get a file from the Internet, or upload a new one using multipart/form-data. [More info on Sending Files »](https://core.telegram.org/bots/api#sending-files)
    pub png_sticker: Option<InputFile<'a>>,
    /// `TGS` animation with the sticker, uploaded using multipart/form-data. See [https://core.telegram.org/animated_stickers#technical-requirements](https://core.telegram.org/animated_stickers#technical-requirements) for technical requirements.
    pub tgs_sticker: Option<InputFile<'a>>,
    /// `Webm` image with the sticker, must be up to 512 kilobytes in size, dimensions must not exceed 512px, and either width or height must be exactly 512px. Pass a `file_id` as a String to send a file that already exists on the Telegram servers, pass an HTTP URL as a String for Telegram to get a file from the Internet, or upload a new one using multipart/form-data. [More info on Sending Files »](https://core.telegram.org/bots/api#sending-files)
    pub webm_sticker: Option<InputFile<'a>>,
    /// Type of stickers in the set, pass `regular` or `mask`. Custom emoji sticker sets can't be created via the Bot API at the moment. By default, a regular sticker set is created.
    pub sticker_type: Option<String>,
    /// One or more emoji corresponding to the sticker
    pub emojis: String,
    /// A JSON-serialized object for position where the mask should be placed on faces
    pub mask_position: Option<MaskPosition>,
}

impl<'a> CreateNewStickerSet<'a> {
    #[must_use]
    pub fn new<T: Into<String>>(user_id: i64, name: T, title: T, emojis: T) -> Self {
        Self {
            user_id,
            name: name.into(),
            title: title.into(),
            emojis: emojis.into(),
            png_sticker: None,
            tgs_sticker: None,
            webm_sticker: None,
            sticker_type: None,
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
    pub fn title<T: Into<String>>(mut self, val: T) -> Self {
        self.title = val.into();
        self
    }

    #[must_use]
    pub fn png_sticker<T: Into<InputFile<'a>>>(mut self, val: T) -> Self {
        self.png_sticker = Some(val.into());
        self
    }

    #[must_use]
    pub fn tgs_sticker<T: Into<InputFile<'a>>>(mut self, val: T) -> Self {
        self.tgs_sticker = Some(val.into());
        self
    }

    #[must_use]
    pub fn webm_sticker<T: Into<InputFile<'a>>>(mut self, val: T) -> Self {
        self.webm_sticker = Some(val.into());
        self
    }

    #[must_use]
    pub fn sticker_type<T: Into<String>>(mut self, val: T) -> Self {
        self.sticker_type = Some(val.into());
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

impl<'a> CreateNewStickerSet<'a> {
    #[must_use]
    pub fn png_sticker_some<T: Into<InputFile<'a>>>(mut self, val: Option<T>) -> Self {
        self.png_sticker = val.map(Into::into);
        self
    }

    #[must_use]
    pub fn tgs_sticker_some<T: Into<InputFile<'a>>>(mut self, val: Option<T>) -> Self {
        self.tgs_sticker = val.map(Into::into);
        self
    }

    #[must_use]
    pub fn webm_sticker_some<T: Into<InputFile<'a>>>(mut self, val: Option<T>) -> Self {
        self.webm_sticker = val.map(Into::into);
        self
    }

    #[must_use]
    pub fn sticker_type_some<T: Into<String>>(mut self, val: Option<T>) -> Self {
        self.sticker_type = val.map(Into::into);
        self
    }

    #[must_use]
    pub fn mask_position_some(mut self, val: Option<MaskPosition>) -> Self {
        self.mask_position = val;
        self
    }
}

impl<'a> TelegramMethod for CreateNewStickerSet<'a> {
    type Method = Self;
    type Return = bool;

    fn build_request<Client>(&self, _bot: &Bot<Client>) -> Request<Self::Method> {
        let mut files = HashMap::new();
        if let Some(png_sticker) = &self.png_sticker {
            prepare_file_with_value(&mut files, png_sticker, "png_sticker");
        }
        if let Some(tgs_sticker) = &self.tgs_sticker {
            prepare_file_with_value(&mut files, tgs_sticker, "tgs_sticker");
        }

        Request::new("createNewStickerSet", self, Some(files))
    }
}
