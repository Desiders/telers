use crate::client::Bot;
use serde::Serialize;
/// Use this method to get current webhook status. Requires no parameters. On success, returns a [`crate::types::WebhookInfo`] object. If the bot is using getUpdates, will return an object with the url field empty.
/// # Documentation
/// <https://core.telegram.org/bots/api#getwebhookinfo>
/// # Returns
/// - `crate::types::WebhookInfo`
#[derive(Clone, Debug, Serialize)]
pub struct GetWebhookInfo {}
impl GetWebhookInfo {
    /// Creates a new `GetWebhookInfo`.
    #[must_use]
    pub const fn new() -> Self {
        Self {}
    }
}
impl Default for GetWebhookInfo {
    fn default() -> Self {
        Self::new()
    }
}
impl super::TelegramMethod for GetWebhookInfo {
    type Method = Self;
    type Return = crate::types::WebhookInfo;

    fn build_request<Client>(self, _bot: &Bot<Client>) -> super::Request<Self::Method> {
        super::Request::new("getWebhookInfo", self, None)
    }
}
