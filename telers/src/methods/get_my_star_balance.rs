use crate::client::Bot;
use serde::Serialize;
/// A method to get the current Telegram Stars balance of the bot. Requires no parameters. On success, returns a [`StarAmount`] object.
/// # Documentation
/// <https://core.telegram.org/bots/api#getmystarbalance>
/// # Returns
/// - `crate::types::StarAmount`
#[derive(Clone, Debug, Serialize)]
pub struct GetMyStarBalance {}
impl GetMyStarBalance {
    /// Creates a new `GetMyStarBalance`.
    #[must_use]
    pub const fn new() -> Self {
        Self {}
    }
}
impl Default for GetMyStarBalance {
    fn default() -> Self {
        Self::new()
    }
}
impl super::TelegramMethod for GetMyStarBalance {
    type Method = Self;
    type Return = crate::types::StarAmount;

    fn build_request<Client>(self, _bot: &Bot<Client>) -> super::Request<Self::Method> {
        super::Request::new("getMyStarBalance", self, None)
    }
}
