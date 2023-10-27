use super::base::{Request, TelegramMethod};

use crate::client::Bot;

use serde::Serialize;
use serde_with::skip_serializing_none;

/// Use this method to set the thumbnail of a custom emoji sticker set.
/// # Documentation
/// <https://core.telegram.org/bots/api#setcustomemojistickersetthumbnail>
/// # Returns
/// Returns `True` on success
#[skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize)]
pub struct SetCustomEmojiStickerSetThumbnail {
    /// Sticker set name
    pub name: String,
    /// Custom emoji identifier of a sticker from the sticker set; pass an empty string to drop the thumbnail and use the first sticker as the thumbnail.
    pub custom_emoji_id: Option<String>,
}

impl SetCustomEmojiStickerSetThumbnail {
    #[must_use]
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            custom_emoji_id: None,
        }
    }

    #[must_use]
    pub fn name(self, val: impl Into<String>) -> Self {
        Self {
            name: val.into(),
            ..self
        }
    }

    #[must_use]
    pub fn custom_emoji_id(self, val: impl Into<String>) -> Self {
        Self {
            custom_emoji_id: Some(val.into()),
            ..self
        }
    }
}

impl SetCustomEmojiStickerSetThumbnail {
    #[must_use]
    pub fn custom_emoji_id_option(self, val: Option<impl Into<String>>) -> Self {
        Self {
            custom_emoji_id: val.map(Into::into),
            ..self
        }
    }
}

impl TelegramMethod for SetCustomEmojiStickerSetThumbnail {
    type Method = Self;
    type Return = bool;

    fn build_request<Client>(&self, _bot: &Bot<Client>) -> Request<Self::Method> {
        Request::new("setCustomEmojiStickerSetThumbnail", self, None)
    }
}

impl AsRef<SetCustomEmojiStickerSetThumbnail> for SetCustomEmojiStickerSetThumbnail {
    fn as_ref(&self) -> &Self {
        self
    }
}
