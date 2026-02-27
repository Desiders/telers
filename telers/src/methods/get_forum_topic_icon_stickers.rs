use crate::client::Bot;
use serde::Serialize;
/// Use this method to get custom emoji stickers, which can be used as a forum topic icon by any user. Requires no parameters. Returns an Array of Sticker objects.
/// # Documentation
/// <https://core.telegram.org/bots/api#getforumtopiciconstickers>
/// # Returns
/// - `Box<[crate::types::Sticker]>`
#[derive(Clone, Debug, Serialize)]
pub struct GetForumTopicIconStickers {}
impl GetForumTopicIconStickers {
    /// Creates a new `GetForumTopicIconStickers`.
    #[must_use]
    pub const fn new() -> Self {
        Self {}
    }
}
impl Default for GetForumTopicIconStickers {
    fn default() -> Self {
        Self::new()
    }
}
impl super::TelegramMethod for GetForumTopicIconStickers {
    type Method = Self;
    type Return = Box<[crate::types::Sticker]>;

    fn build_request<Client>(self, _bot: &Bot<Client>) -> super::Request<Self::Method> {
        super::Request::new("getForumTopicIconStickers", self, None)
    }
}
