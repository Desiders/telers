use super::{InputFile, MaskPosition};

use serde::Serialize;
use serde_with::skip_serializing_none;

/// This object describes a sticker to be added to a sticker set.
/// # Documentation
/// <https://core.telegram.org/bots/api#inputsticker>
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct InputSticker<'a> {
    /// The added sticker. Pass a `file_id` as a String to send a file that already exists on the Telegram servers, pass an HTTP URL as a String for Telegram to get a file from the Internet, upload a new one using `multipart/form-data`, or pass `attach://<file_attach_name>` to upload a new one using `multipart/form-data` under <file_attach_name> name. Animated and video stickers can't be uploaded via HTTP URL. [`More information on Sending Files`](https://core.telegram.org/bots/api#sending-files).
    pub sticker: InputFile<'a>,
    /// List of 1-20 emoji associated with the sticker
    pub emoji_list: Vec<String>,
    /// Position where the mask should be placed on faces. For `mask` stickers only.
    pub mask_position: Option<MaskPosition>,
    /// List of 0-20 search keywords for the sticker with total length of up to 64 characters. For `regular` and `custom_emoji` stickers only.
    pub keywords: Option<Vec<String>>,
}

impl<'a> InputSticker<'a> {
    #[must_use]
    pub fn new(sticker: impl Into<InputFile<'a>>) -> Self {
        Self {
            sticker: sticker.into(),
            emoji_list: Vec::new(),
            mask_position: None,
            keywords: None,
        }
    }

    #[must_use]
    pub fn sticker(self, val: impl Into<InputFile<'a>>) -> Self {
        Self {
            sticker: val.into(),
            ..self
        }
    }

    #[must_use]
    pub fn emoji<T>(self, val: T) -> Self
    where
        T: Into<String>,
    {
        Self {
            emoji_list: self
                .emoji_list
                .into_iter()
                .chain(Some(val.into()))
                .collect(),
            ..self
        }
    }

    #[must_use]
    pub fn emoji_list<T, I>(self, val: I) -> Self
    where
        T: Into<String>,
        I: IntoIterator<Item = T>,
    {
        Self {
            emoji_list: self
                .emoji_list
                .into_iter()
                .chain(val.into_iter().map(Into::into))
                .collect(),
            ..self
        }
    }

    /// Alias to [`InputSticker::emoji_list`] method
    #[must_use]
    pub fn emojis<T, I>(self, val: I) -> Self
    where
        T: Into<String>,
        I: IntoIterator<Item = T>,
    {
        self.emoji_list(val)
    }

    #[must_use]
    pub fn mask_position(self, val: MaskPosition) -> Self {
        Self {
            mask_position: Some(val),
            ..self
        }
    }

    #[must_use]
    pub fn keyword<T>(self, val: T) -> Self
    where
        T: Into<String>,
    {
        Self {
            keywords: Some(
                self.keywords
                    .unwrap_or_default()
                    .into_iter()
                    .chain(Some(val.into()))
                    .collect(),
            ),
            ..self
        }
    }

    #[must_use]
    pub fn keywords<T, I>(self, val: I) -> Self
    where
        T: Into<String>,
        I: IntoIterator<Item = T>,
    {
        Self {
            keywords: Some(
                self.keywords
                    .unwrap_or_default()
                    .into_iter()
                    .chain(val.into_iter().map(Into::into))
                    .collect(),
            ),
            ..self
        }
    }
}
