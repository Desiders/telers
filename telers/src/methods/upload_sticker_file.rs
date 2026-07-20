use crate::client::Bot;
use serde::Serialize;
/// Use this method to upload a file with a sticker for later use in the [`crate::methods::CreateNewStickerSet`], [`crate::methods::AddStickerToSet`], or [`crate::methods::ReplaceStickerInSet`] methods (the file can be used multiple times). Returns the uploaded File on success.
/// # Documentation
/// <https://core.telegram.org/bots/api#uploadstickerfile>
/// # Returns
/// - `crate::types::File`
#[derive(Clone, Debug, Serialize)]
pub struct UploadStickerFile {
    /// User identifier of sticker file owner
    pub user_id: i64,
    /// A file with the sticker in .WEBP, .PNG, .TGS, or .WEBM format. See <https://core.telegram.org/stickers> for technical requirements. More information on Sending Files: <https://core.telegram.org/bots/api#sending-files>
    pub sticker: crate::types::InputFile,
    /// Format of the sticker, must be one of `static`, `animated`, `video`
    pub sticker_format: Box<str>,
}
impl UploadStickerFile {
    /// Creates a new `UploadStickerFile`.
    ///
    /// # Arguments
    /// * `user_id` - User identifier of sticker file owner
    /// * `sticker` - A file with the sticker in .WEBP, .PNG, .TGS, or .WEBM format. See <https://core.telegram.org/stickers> for technical requirements. More information on Sending Files: <https://core.telegram.org/bots/api#sending-files>
    /// * `sticker_format` - Format of the sticker, must be one of `static`, `animated`, `video`
    #[must_use]
    pub fn new<T0: Into<i64>, T1: Into<crate::types::InputFile>, T2: Into<Box<str>>>(
        user_id: T0,
        sticker: T1,
        sticker_format: T2,
    ) -> Self {
        Self {
            user_id: user_id.into(),
            sticker: sticker.into(),
            sticker_format: sticker_format.into(),
        }
    }

    /// User identifier of sticker file owner
    #[must_use]
    pub fn user_id<T: Into<i64>>(mut self, val: T) -> Self {
        self.user_id = val.into();
        self
    }

    /// A file with the sticker in .WEBP, .PNG, .TGS, or .WEBM format. See <https://core.telegram.org/stickers> for technical requirements. More information on Sending Files: <https://core.telegram.org/bots/api#sending-files>
    #[must_use]
    pub fn sticker<T: Into<crate::types::InputFile>>(mut self, val: T) -> Self {
        self.sticker = val.into();
        self
    }

    /// Format of the sticker, must be one of `static`, `animated`, `video`
    #[must_use]
    pub fn sticker_format<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.sticker_format = val.into();
        self
    }
}
impl super::TelegramMethod for UploadStickerFile {
    type Method = Self;
    type Return = crate::types::File;

    fn build_request<Client>(mut self, _bot: &Bot<Client>) -> super::Request<Self::Method> {
        let mut files = vec![];
        super::prepare_file(&mut files, &mut self.sticker);
        super::Request::new("uploadStickerFile", self, Some(files))
    }
}
