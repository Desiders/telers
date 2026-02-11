use super::base::{Request, TelegramMethod};
use crate::{client::Bot, types::StarAmount};

use serde::Serialize;

/// A method to get the current Telegram Stars balance of the bot.
/// # Documentation
/// <https://core.telegram.org/bots/api#getmystarbalance>
/// # Returns
/// On success, [`StarAmount`] is returned
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize)]
pub struct GetMyStarBalance {}

impl GetMyStarBalance {
    #[must_use]
    pub fn new() -> Self {
        Self {}
    }
}

impl TelegramMethod for GetMyStarBalance {
    type Method = Self;
    type Return = StarAmount;

    fn build_request<Client>(&self, _bot: &Bot<Client>) -> Request<'_, Self::Method> {
        Request::new("getMyStarBalance", self, None)
    }
}

impl AsRef<GetMyStarBalance> for GetMyStarBalance {
    fn as_ref(&self) -> &Self {
        self
    }
}
