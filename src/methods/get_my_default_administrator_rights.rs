use super::base::{Request, TelegramMethod};

use crate::{client::Bot, types::ChatAdministratorRights};

use serde::Serialize;
use serde_with::skip_serializing_none;

/// Use this method to get the current default administrator rights of the bot.
/// # Documentation
/// <https://core.telegram.org/bots/api#getmydefaultadministratorrights>
/// # Returns
/// Returns [`ChatAdministratorRights`] on success.
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
    pub fn for_channels(mut self, val: bool) -> Self {
        self.for_channels = Some(val);
        self
    }
}

impl TelegramMethod for GetMyDefaultAdministratorRights {
    type Method = Self;
    type Return = ChatAdministratorRights;

    fn build_request(&self, _: &Bot) -> Request<Self::Method> {
        Request::new("getMyDefaultAdministratorRights", self, None)
    }
}