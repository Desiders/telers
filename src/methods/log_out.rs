use super::base::{Request, TelegramMethod};

use crate::client::Bot;

use serde::Serialize;

/// Use this method to log out from the cloud Bot API server before launching the bot locally. You must log out the bot before running it locally, otherwise there is no guarantee that the bot will receive updates. After a successful call, you can immediately log in on a local server, but will not be able to log in back to the cloud Bot API server for 10 minutes. Requires no parameters.
/// # Documentation
/// <https://core.telegram.org/bots/api#logout>
/// # Returns
/// Returns `True` on success
#[derive(Default, Clone, Debug, Eq, Hash, PartialEq, Serialize)]
pub struct LogOut {}

impl LogOut {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

impl TelegramMethod for LogOut {
    type Method = Self;
    type Return = bool;

    fn build_request(&self, _bot: &Bot) -> Request<Self::Method> {
        Request::new("logOut", self, None)
    }
}
