use crate::client::Bot;
use serde::Serialize;
/// Use this method to remove webhook integration if you decide to switch back to getUpdates. Returns `true` on success.
/// # Documentation
/// <https://core.telegram.org/bots/api#deletewebhook>
/// # Returns
/// - `bool`
#[derive(Clone, Debug, Serialize)]
pub struct DeleteWebhook {
    /// Pass `true` to drop all pending updates
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drop_pending_updates: Option<bool>,
}
impl DeleteWebhook {
    /// Creates a new `DeleteWebhook`.
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new() -> Self {
        Self {
            drop_pending_updates: None,
        }
    }

    /// Pass `true` to drop all pending updates
    #[must_use]
    pub fn drop_pending_updates<T: Into<bool>>(self, val: T) -> Self {
        let mut this = self;
        this.drop_pending_updates = Some(val.into());
        this
    }

    /// Pass `true` to drop all pending updates
    #[must_use]
    pub fn drop_pending_updates_option<T: Into<bool>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.drop_pending_updates = val.map(Into::into);
        this
    }
}
impl Default for DeleteWebhook {
    fn default() -> Self {
        Self::new()
    }
}
impl super::TelegramMethod for DeleteWebhook {
    type Method = Self;
    type Return = bool;

    fn build_request<Client>(self, _bot: &Bot<Client>) -> super::Request<Self::Method> {
        super::Request::new("deleteWebhook", self, None)
    }
}
