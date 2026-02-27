use crate::client::Bot;
use serde::Serialize;
/// Returns the list of gifts that can be sent by the bot to users and channel chats. Requires no parameters. Returns a Gifts object.
/// # Documentation
/// <https://core.telegram.org/bots/api#getavailablegifts>
/// # Returns
/// - `crate::types::Gifts`
#[derive(Clone, Debug, Serialize)]
pub struct GetAvailableGifts {}
impl GetAvailableGifts {
    /// Creates a new `GetAvailableGifts`.
    #[must_use]
    pub const fn new() -> Self {
        Self {}
    }
}
impl Default for GetAvailableGifts {
    fn default() -> Self {
        Self::new()
    }
}
impl super::TelegramMethod for GetAvailableGifts {
    type Method = Self;
    type Return = crate::types::Gifts;

    fn build_request<Client>(self, _bot: &Bot<Client>) -> super::Request<Self::Method> {
        super::Request::new("getAvailableGifts", self, None)
    }
}
