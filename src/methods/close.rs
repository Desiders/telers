use super::base::{Request, TelegramMethod};

use crate::{client::Bot, types::User};

use serde::Serialize;

/// Use this method to close the bot instance before moving it from one local server to another. You need to delete the webhook before calling this method to ensure that the bot isn't launched again after server restart. The method will return error 429 in the first 10 minutes after the bot is launched. Requires no parameters.
/// # Documentation
/// <https://core.telegram.org/bots/api#close>
/// # Returns
/// Returns `True` on success
#[derive(Default, Clone, Debug, Eq, Hash, PartialEq, Serialize)]
pub struct Close {}

impl Close {
    pub fn new() -> Self {
        Self::default()
    }
}

impl TelegramMethod for Close {
    type Method = Self;
    type Return = bool;

    fn build_request<Client>(&self, _bot: &Bot<Client>) -> Request<Self::Method> {
        Request::new("close", self, None)
    }
}

impl AsRef<Close> for Close {
    fn as_ref(&self) -> &Self {
        self
    }
}
