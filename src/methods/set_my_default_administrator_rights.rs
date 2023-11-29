use super::base::{Request, TelegramMethod};

use crate::{client::Bot, types::ChatAdministratorRights};

use serde::Serialize;
use serde_with::skip_serializing_none;

/// Use this method to change the default administrator rights requested by the bot when it's added as an administrator to groups or channels. These rights will be suggested to users, but they are are free to modify the list before adding the bot.
/// # Documentation
/// <https://core.telegram.org/bots/api#setmydefaultadministratorrights>
/// # Returns
/// Returns `true` on success
#[skip_serializing_none]
#[derive(Debug, Default, Clone, Hash, PartialEq, Eq, Serialize)]
pub struct SetMyDefaultAdministratorRights {
    /// A JSON-serialized object describing new default administrator rights. If not specified, the default administrator rights will be cleared.
    pub rights: Option<ChatAdministratorRights>,
    /// Pass `true` to change the default administrator rights of the bot in channels. Otherwise, the default administrator rights of the bot for groups and supergroups will be changed.
    pub for_channels: Option<bool>,
}

impl SetMyDefaultAdministratorRights {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn rights(self, val: impl Into<ChatAdministratorRights>) -> Self {
        Self {
            rights: Some(val.into()),
            ..self
        }
    }

    #[must_use]
    pub fn for_channels(self, val: bool) -> Self {
        Self {
            for_channels: Some(val),
            ..self
        }
    }
}

impl SetMyDefaultAdministratorRights {
    #[must_use]
    pub fn rights_option(self, val: Option<impl Into<ChatAdministratorRights>>) -> Self {
        Self {
            rights: val.map(Into::into),
            ..self
        }
    }

    #[must_use]
    pub fn for_channels_option(self, val: Option<bool>) -> Self {
        Self {
            for_channels: val,
            ..self
        }
    }
}

impl TelegramMethod for SetMyDefaultAdministratorRights {
    type Method = Self;
    type Return = bool;

    fn build_request<Client>(&self, _bot: &Bot<Client>) -> Request<Self::Method> {
        Request::new("setMyDefaultAdministratorRights", self, None)
    }
}

impl AsRef<SetMyDefaultAdministratorRights> for SetMyDefaultAdministratorRights {
    fn as_ref(&self) -> &Self {
        self
    }
}
