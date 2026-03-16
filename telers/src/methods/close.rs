use crate::client::Bot;
use serde::Serialize;
/// Use this method to close the bot instance before moving it from one local server to another. You need to delete the webhook before calling this method to ensure that the bot isn't launched again after server restart. The method will return error 429 in the first 10 minutes after the bot is launched. Returns `true` on success. Requires no parameters.
/// # Documentation
/// <https://core.telegram.org/bots/api#close>
/// # Returns
/// - `bool`
#[derive(Clone, Debug, Serialize)]
pub struct Close {}
impl Close {
    /// Creates a new `Close`.
    #[must_use]
    pub const fn new() -> Self {
        Self {}
    }
}
impl Default for Close {
    fn default() -> Self {
        Self::new()
    }
}
impl super::TelegramMethod for Close {
    type Method = Self;
    type Return = bool;

    fn build_request<Client>(self, _bot: &Bot<Client>) -> super::Request<Self::Method> {
        super::Request::new("close", self, None)
    }
}
