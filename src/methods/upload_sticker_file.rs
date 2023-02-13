use super::base::{prepare_file_with_value, Request, TelegramMethod};

use crate::{
    client::Bot,
    types::{File, InputFile},
};

use serde::Serialize;
use serde_with::skip_serializing_none;
use std::collections::HashMap;

/// Use this method to upload a .PNG file with a sticker for later use in [`CreateNewStickerSet`](crate::methods::CreateNewStickerSet) and [`AddStickerToSet`](crate::methods::AddStickerToSet) methods (can be used multiple times)
/// # Documentation
/// <https://core.telegram.org/bots/api#uploadstickerfile>
/// # Returns
/// Returns the uploaded [`File`] on success
#[skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize)]
pub struct UploadStickerFile<'a> {
    /// User identifier of sticker file owner
    pub user_id: i64,
    /// `PNG` image with the sticker, must be up to 512 kilobytes in size, dimensions must not exceed 512px, and either width or height must be exactly 512px. [More info on Sending Files »](https://core.telegram.org/bots/api#sending-files)
    pub png_sticker: InputFile<'a>,
}

impl<'a> UploadStickerFile<'a> {
    #[must_use]
    pub fn new<T: Into<InputFile<'a>>>(user_id: i64, png_sticker: T) -> Self {
        Self {
            user_id,
            png_sticker: png_sticker.into(),
        }
    }

    #[must_use]
    pub fn user_id(mut self, val: i64) -> Self {
        self.user_id = val;
        self
    }

    #[must_use]
    pub fn png_sticker<T: Into<InputFile<'a>>>(mut self, val: T) -> Self {
        self.png_sticker = val.into();
        self
    }
}

impl<'a> TelegramMethod for UploadStickerFile<'a> {
    type Method = Self;
    type Return = File;

    fn build_request<Client>(&self, _bot: &Bot<Client>) -> Request<Self::Method> {
        let mut files = HashMap::new();
        prepare_file_with_value(&mut files, &self.png_sticker, "png_sticker");

        Request::new("uploadStickerFile", self, Some(files))
    }
}
