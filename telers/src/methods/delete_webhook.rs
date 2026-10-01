use crate::client::Bot;
/// Use this method to remove webhook integration if you decide to switch back to [`crate::methods::GetUpdates`]. Returns `true` on success.
/// # Documentation
/// <https://core.telegram.org/bots/api#deletewebhook>
/// # Returns
/// - `bool`
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deser", derive(deser::Serialize))]
#[cfg_attr(not(feature = "deser"), derive(serde::Serialize))]
pub struct DeleteWebhook {
    /// Pass `true` to drop all pending updates
    #[cfg_attr(not(feature = "deser"), serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "deser", deser(skip_serializing_if = Option::is_none))]
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
    pub fn drop_pending_updates<T: Into<bool>>(mut self, val: T) -> Self {
        self.drop_pending_updates = Some(val.into());
        self
    }

    /// Pass `true` to drop all pending updates
    #[must_use]
    pub fn drop_pending_updates_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.drop_pending_updates = val.map(Into::into);
        self
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
