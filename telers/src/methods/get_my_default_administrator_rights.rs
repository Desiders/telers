use crate::client::Bot;
use serde::Serialize;
/// Use this method to get the current default administrator rights of the bot. Returns [`crate::types::ChatAdministratorRights`] on success.
/// # Documentation
/// <https://core.telegram.org/bots/api#getmydefaultadministratorrights>
/// # Returns
/// - `crate::types::ChatAdministratorRights`
#[derive(Clone, Debug, Serialize)]
pub struct GetMyDefaultAdministratorRights {
    /// Pass `true` to get default administrator rights of the bot in channels. Otherwise, default administrator rights of the bot for groups and supergroups will be returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub for_channels: Option<bool>,
}
impl GetMyDefaultAdministratorRights {
    /// Creates a new `GetMyDefaultAdministratorRights`.
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new() -> Self {
        Self {
            for_channels: None,
        }
    }

    /// Pass `true` to get default administrator rights of the bot in channels. Otherwise, default administrator rights of the bot for groups and supergroups will be returned.
    #[must_use]
    pub fn for_channels<T: Into<bool>>(mut self, val: T) -> Self {
        self.for_channels = Some(val.into());
        self
    }

    /// Pass `true` to get default administrator rights of the bot in channels. Otherwise, default administrator rights of the bot for groups and supergroups will be returned.
    #[must_use]
    pub fn for_channels_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.for_channels = val.map(Into::into);
        self
    }
}
impl Default for GetMyDefaultAdministratorRights {
    fn default() -> Self {
        Self::new()
    }
}
impl super::TelegramMethod for GetMyDefaultAdministratorRights {
    type Method = Self;
    type Return = crate::types::ChatAdministratorRights;

    fn build_request<Client>(self, _bot: &Bot<Client>) -> super::Request<Self::Method> {
        super::Request::new("getMyDefaultAdministratorRights", self, None)
    }
}
