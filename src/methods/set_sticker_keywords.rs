use super::base::{Request, TelegramMethod};

use crate::client::Bot;

use serde::Serialize;
use serde_with::skip_serializing_none;

/// Use this method to change search keywords assigned to a regular or custom emoji sticker. The sticker must belong to a sticker set created by the bot.
/// # Documentation
/// <https://core.telegram.org/bots/api#setstickerkeywords>
/// # Returns
/// Returns `True` on success
#[skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize)]
pub struct SetStickerKeywords {
    /// File identifier of the sticker
    pub sticker: String,
    /// A JSON-serialized list of 0-20 search keywords for the sticker with total length of up to 64 characters
    pub keywords: Vec<String>,
}

impl SetStickerKeywords {
    #[must_use]
    pub fn new(sticker: impl Into<String>) -> Self {
        Self {
            sticker: sticker.into(),
            keywords: vec![],
        }
    }

    #[must_use]
    pub fn sticker(self, val: impl Into<String>) -> Self {
        Self {
            sticker: val.into(),
            ..self
        }
    }

    #[must_use]
    pub fn keyword(self, val: impl Into<String>) -> Self {
        Self {
            keywords: self.keywords.into_iter().chain(Some(val.into())).collect(),
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
            keywords: self
                .keywords
                .into_iter()
                .chain(val.into_iter().map(Into::into))
                .collect(),
            ..self
        }
    }
}

impl TelegramMethod for SetStickerKeywords {
    type Method = Self;
    type Return = bool;

    fn build_request<Client>(&self, _bot: &Bot<Client>) -> Request<Self::Method> {
        Request::new("setStickerKeywords", self, None)
    }
}
