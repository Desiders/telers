use super::base::{Request, TelegramMethod};

use crate::client::Bot;

use serde::Serialize;
use serde_with::skip_serializing_none;

/// Use this method to remove webhook integration if you decide to switch back to [`super::GetUpdates`].
/// # Documentation
/// <https://core.telegram.org/bots/api#deletewebhook>
/// # Returns
/// On success, `true` is returned
#[skip_serializing_none]
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize)]
pub struct DeleteWebhook {
    /// Pass `true` to drop all pending updates
    pub drop_pending_updates: Option<bool>,
}

impl DeleteWebhook {
    #[inline]
    #[must_use]
    pub const fn new() -> Self {
        Self {
            drop_pending_updates: None,
        }
    }

    #[must_use]
    pub fn drop_pending_updates(self, val: bool) -> Self {
        Self {
            drop_pending_updates: Some(val),
        }
    }
}

impl DeleteWebhook {
    #[must_use]
    pub fn drop_pending_updates_option(self, val: Option<bool>) -> Self {
        Self {
            drop_pending_updates: val,
        }
    }
}

impl Default for DeleteWebhook {
    fn default() -> Self {
        Self::new()
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
