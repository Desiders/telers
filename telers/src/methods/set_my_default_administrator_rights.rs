use crate::client::Bot;
use serde::Serialize;
/// Use this method to change the default administrator rights requested by the bot when it's added as an administrator to groups or channels. These rights will be suggested to users, but they are free to modify the list before adding the bot. Returns `true` on success.
/// # Documentation
/// <https://core.telegram.org/bots/api#setmydefaultadministratorrights>
/// # Returns
/// - `bool`
#[derive(Clone, Debug, Serialize)]
pub struct SetMyDefaultAdministratorRights {
    /// A JSON-serialized object describing new default administrator rights. If not specified, the default administrator rights will be cleared.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rights: Option<crate::types::ChatAdministratorRights>,
    /// Pass `true` to change the default administrator rights of the bot in channels. Otherwise, the default administrator rights of the bot for groups and supergroups will be changed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub for_channels: Option<bool>,
}
impl SetMyDefaultAdministratorRights {
    /// Creates a new `SetMyDefaultAdministratorRights`.
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new() -> Self {
        Self {
            rights: None,
            for_channels: None,
        }
    }

    /// A JSON-serialized object describing new default administrator rights. If not specified, the default administrator rights will be cleared.
    #[must_use]
    pub fn rights<T: Into<crate::types::ChatAdministratorRights>>(mut self, val: T) -> Self {
        self.rights = Some(val.into());
        self
    }

    /// A JSON-serialized object describing new default administrator rights. If not specified, the default administrator rights will be cleared.
    #[must_use]
    pub fn rights_option<T: Into<crate::types::ChatAdministratorRights>>(
        mut self,
        val: Option<T>,
    ) -> Self {
        self.rights = val.map(Into::into);
        self
    }

    /// Pass `true` to change the default administrator rights of the bot in channels. Otherwise, the default administrator rights of the bot for groups and supergroups will be changed.
    #[must_use]
    pub fn for_channels<T: Into<bool>>(mut self, val: T) -> Self {
        self.for_channels = Some(val.into());
        self
    }

    /// Pass `true` to change the default administrator rights of the bot in channels. Otherwise, the default administrator rights of the bot for groups and supergroups will be changed.
    #[must_use]
    pub fn for_channels_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.for_channels = val.map(Into::into);
        self
    }
}
impl Default for SetMyDefaultAdministratorRights {
    fn default() -> Self {
        Self::new()
    }
}
impl super::TelegramMethod for SetMyDefaultAdministratorRights {
    type Method = Self;
    type Return = bool;

    fn build_request<Client>(self, _bot: &Bot<Client>) -> super::Request<Self::Method> {
        super::Request::new("setMyDefaultAdministratorRights", self, None)
    }
}
