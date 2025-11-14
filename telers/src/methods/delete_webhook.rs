use super::base::{Request, TelegramMethod};

use crate::client::Bot;

use serde::Serialize;

/// Use this method to remove webhook integration if you decide to switch back to [`super::GetUpdates`].
/// # Documentation
/// <https://core.telegram.org/bots/api#deletewebhook>
/// # Returns
/// On success, `true` is returned
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize)]
pub struct DeleteWebhook {}

impl DeleteWebhook {
    #[inline]
    #[must_use]
    pub const fn new() -> Self {
        Self {}
    }
}

impl TelegramMethod for DeleteWebhook {
    type Method = Self;
    type Return = bool;

    fn build_request<Client>(&self, _bot: &Bot<Client>) -> Request<'_, Self::Method> {
        Request::new("deleteWebhook", self, None)
    }
}

impl AsRef<DeleteWebhook> for DeleteWebhook {
    fn as_ref(&self) -> &Self {
        self
    }
}
