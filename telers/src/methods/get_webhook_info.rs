use super::base::{Request, TelegramMethod};

use crate::{client::Bot, types::WebhookInfo};

use serde::Serialize;

/// Use this method to get current webhook status. Requires no parameters.
/// # Documentation
/// <https://core.telegram.org/bots/api#getwebhookinfo>
/// # Returns
/// Returns [`WebhookInfo`] on success. If the bot is using [`super::GetUpdates`], will return an object with the `url` field empty.
#[derive(Debug, Default, Clone, Hash, PartialEq, Eq, Serialize)]
pub struct GetWebhookInfo {}

impl GetWebhookInfo {
    #[must_use]
    pub const fn new() -> Self {
        Self {}
    }
}

impl TelegramMethod for GetWebhookInfo {
    type Method = Self;
    type Return = WebhookInfo;

    fn build_request<Client>(&self, _bot: &Bot<Client>) -> Request<'_, Self::Method> {
        Request::new("getWebhookInfo", self, None)
    }
}

impl AsRef<GetWebhookInfo> for GetWebhookInfo {
    fn as_ref(&self) -> &Self {
        self
    }
}
