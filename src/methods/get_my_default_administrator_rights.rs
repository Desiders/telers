use super::base::{Request, TelegramMethod};

use crate::{client::Bot, types::ChatAdministratorRights};

use serde::Serialize;
use serde_with::skip_serializing_none;

/// Use this method to get the current default administrator rights of the bot.
/// # Documentation
/// <https://core.telegram.org/bots/api#getmydefaultadministratorrights>
/// # Returns
/// Returns [`ChatAdministratorRights`] on success
#[skip_serializing_none]
#[derive(Default, Clone, Debug, Eq, Hash, PartialEq, Serialize)]
pub struct GetMyDefaultAdministratorRights {
    /// Pass `True` to get default administrator rights of the bot in channels. Otherwise, default administrator rights of the bot for groups and supergroups will be returned.
    pub for_channels: Option<bool>,
}

impl GetMyDefaultAdministratorRights {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn for_channels(self, val: bool) -> Self {
        Self {
            for_channels: Some(val),
        }
    }
}

impl GetMyDefaultAdministratorRights {
    #[must_use]
    pub fn for_channels_option(self, val: Option<bool>) -> Self {
        Self { for_channels: val }
    }
}

impl TelegramMethod for GetMyDefaultAdministratorRights {
    type Method = Self;
    type Return = ChatAdministratorRights;

    fn build_request<Client>(&self, _bot: &Bot<Client>) -> Request<Self::Method> {
        Request::new("getMyDefaultAdministratorRights", self, None)
    }
}
