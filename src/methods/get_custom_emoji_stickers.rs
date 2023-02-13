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
    pub fn new<T: Into<String>>(custom_emoji_ids: Vec<T>) -> Self {
        Self {
            custom_emoji_ids: custom_emoji_ids.into_iter().map(Into::into).collect(),
        }
    }

    #[must_use]
    pub fn custom_emoji_ids<T: Into<String>>(mut self, val: Vec<T>) -> Self {
        self.custom_emoji_ids = val.into_iter().map(Into::into).collect();
        self
    }

    #[must_use]
    pub fn custom_emoji_id<T: Into<String>>(mut self, val: T) -> Self {
        self.custom_emoji_ids.push(val.into());
        self
    }
}

impl TelegramMethod for GetCustomEmojiStickers {
    type Method = Self;
    type Return = Vec<Sticker>;

    fn build_request<Client>(&self, _bot: &Bot<Client>) -> Request<Self::Method> {
        Request::new("getCustomEmojiStickers", self, None)
    }
}
