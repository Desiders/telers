use super::base::{Request, TelegramMethod};

use crate::{client::Bot, types::Sticker};

use serde::Serialize;
use serde_with::skip_serializing_none;

/// Use this method to get custom emoji stickers, which can be used as a forum topic icon by any user. Requires no parameters.
/// # Documentation
/// <https://core.telegram.org/bots/api#getforumtopiciconstickers>
/// # Returns
/// Returns an Array of [`Sticker`] objects
#[skip_serializing_none]
#[derive(Default, Clone, Debug, Eq, Hash, PartialEq, Serialize)]
pub struct GetForumTopicIconStickers {}

impl GetForumTopicIconStickers {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

impl TelegramMethod for GetForumTopicIconStickers {
    type Method = Self;
    type Return = Vec<Sticker>;

    fn build_request<Client>(&self, _bot: &Bot<Client>) -> Request<Self::Method> {
        Request::new("getForumTopicIconStickers", self, None)
    }
}
