use super::base::{Request, TelegramMethod};

use crate::client::Bot;

use serde::Serialize;
use serde_with::skip_serializing_none;

/// Use this method to set the title of a created sticker set.
/// # Documentation
/// <https://core.telegram.org/bots/api#setstickersettitle>
/// # Returns
/// Returns `true` on success
#[skip_serializing_none]
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize)]
pub struct SetStickerSetTitle {
    /// Sticker set name
    pub name: String,
    /// Sticker set title, 1-64 characters
    pub title: String,
}

impl SetStickerSetTitle {
    #[must_use]
    pub fn new(name: impl Into<String>, title: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            title: title.into(),
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
    pub fn title(self, val: impl Into<String>) -> Self {
        Self {
            title: val.into(),
            ..self
        }
    }
}

impl TelegramMethod for SetStickerSetTitle {
    type Method = Self;
    type Return = bool;

    fn build_request<Client>(&self, _bot: &Bot<Client>) -> Request<Self::Method> {
        Request::new("setStickerSetTitle", self, None)
    }
}

impl AsRef<SetStickerSetTitle> for SetStickerSetTitle {
    fn as_ref(&self) -> &Self {
        self
    }
}
