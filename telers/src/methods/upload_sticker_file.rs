use super::base::{prepare_file, Request, TelegramMethod};

use crate::{
    client::Bot,
    types::{File, InputFile},
};

use serde::Serialize;
use serde_with::skip_serializing_none;

/// Use this method to upload a .PNG file with a sticker for later use in [`CreateNewStickerSet`](crate::methods::CreateNewStickerSet) and [`AddStickerToSet`](crate::methods::AddStickerToSet) methods (can be used multiple times)
/// # Documentation
/// <https://core.telegram.org/bots/api#uploadstickerfile>
/// # Returns
/// Returns the uploaded [`File`] on success
#[skip_serializing_none]
#[derive(Debug, Hash, PartialEq, Serialize)]
pub struct UploadStickerFile {
    /// User identifier of sticker file owner
    pub user_id: i64,
    /// A file with the sticker in `.WEBP`, `.PNG`, `.TGS`, or `.WEBM` format. See <https://core.telegram.org/stickers> for technical requirements. [More info on Sending Files »](https://core.telegram.org/bots/api#sending-files)
    pub sticker: InputFile,
    /// Format of the sticker, must be one of `static`, `animated`, `video`
    pub sticker_format: Option<String>,
}

impl UploadStickerFile {
    #[must_use]
    pub fn new(user_id: i64, sticker: impl Into<InputFile>) -> Self {
        Self {
            user_id,
            sticker: sticker.into(),
            sticker_format: None,
        }
    }

    #[must_use]
    pub fn user_id(self, val: i64) -> Self {
        Self {
            user_id: val,
            ..self
        }
    }

    #[must_use]
    pub fn sticker(self, val: impl Into<InputFile>) -> Self {
        Self {
            sticker: val.into(),
            ..self
        }
    }

    #[must_use]
    pub fn sticker_format(self, val: impl Into<String>) -> Self {
        Self {
            sticker_format: Some(val.into()),
            ..self
        }
    }

    /// Alias to [`UploadStickerFile::sticker_format`] method
    #[must_use]
    pub fn format(self, val: impl Into<String>) -> Self {
        self.sticker_format(val)
    }
}

impl UploadStickerFile {
    #[must_use]
    pub fn sticker_format_option(self, val: Option<impl Into<String>>) -> Self {
        Self {
            sticker_format: val.map(Into::into),
            ..self
        }
    }

    /// Alias to [`UploadStickerFile::sticker_format_option`] method
    #[must_use]
    pub fn format_option(self, val: Option<impl Into<String>>) -> Self {
        self.sticker_format_option(val)
    }
}

impl TelegramMethod for UploadStickerFile {
    type Method = Self;
    type Return = File;

    fn build_request<Client>(mut self, _bot: &Bot<Client>) -> Request<Self::Method> {
        let mut files = vec![];
        prepare_file(&mut files, &mut self.sticker);

        Request::new("uploadStickerFile", self, Some(files))
    }
}

impl AsRef<UploadStickerFile> for UploadStickerFile {
    fn as_ref(&self) -> &Self {
        self
    }
}
