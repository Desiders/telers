use super::base::{Request, TelegramMethod};

use crate::{client::Bot, types::Sticker};

use serde::Serialize;
use serde_with::skip_serializing_none;

/// Use this method to get information about custom emoji stickers by their identifiers.
/// # Documentation
/// <https://core.telegram.org/bots/api#getcustomemojistickers>
/// # Returns
/// Returns an Array of [`Sticker`] objects
#[skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize)]
pub struct GetCustomEmojiStickers {
    /// List of custom emoji identifiers. At most 200 custom emoji identifiers can be specified.
    pub custom_emoji_ids: Vec<String>,
}

impl GetCustomEmojiStickers {
    #[must_use]
    pub fn new<T, I>(custom_emoji_ids: I) -> Self
    where
        T: Into<String>,
        I: IntoIterator<Item = T>,
    {
        Self {
            custom_emoji_ids: custom_emoji_ids.into_iter().map(Into::into).collect(),
        }
    }

    #[must_use]
    pub fn custom_emoji_id(self, val: impl Into<String>) -> Self {
        Self {
            custom_emoji_ids: self
                .custom_emoji_ids
                .into_iter()
                .chain(Some(val.into()))
                .collect(),
        }
    }

    #[must_use]
    pub fn custom_emoji_ids<T, I>(self, val: I) -> Self
    where
        T: Into<String>,
        I: IntoIterator<Item = T>,
    {
        Self {
            custom_emoji_ids: self
                .custom_emoji_ids
                .into_iter()
                .chain(val.into_iter().map(Into::into))
                .collect(),
        }
    }
}

impl TelegramMethod for GetCustomEmojiStickers {
    type Method = Self;
    type Return = Vec<Sticker>;

    fn build_request<Client>(&self, _bot: &Bot<Client>) -> Request<Self::Method> {
        Request::new("getCustomEmojiStickers", self, None)
    }
}
