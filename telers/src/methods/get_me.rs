use super::base::{Request, TelegramMethod};

use crate::{client::Bot, types::User};

use serde::Serialize;

/// A simple method for testing your bot's authentication token. Requires no parameters.
/// # Documentation
/// <https://core.telegram.org/bots/api#getme>
/// # Returns
/// Returns basic information about the bot in form of a [`User`] object
#[derive(Debug, Default, Clone, Hash, PartialEq, Eq, Serialize)]
pub struct GetMe {}

impl GetMe {
    #[must_use]
    pub const fn new() -> Self {
        Self {}
    }
}

impl TelegramMethod for GetMe {
    type Method = Self;
    type Return = User;

    fn build_request<Client>(&self, _bot: &Bot<Client>) -> Request<Self::Method> {
        Request::new("getMe", self, None)
    }
}

impl AsRef<GetMe> for GetMe {
    fn as_ref(&self) -> &Self {
        self
    }
}
