use super::base::{Request, TelegramMethod};

use crate::{client::Bot, types::Gifts};

use serde::Serialize;

/// Returns the list of gifts that can be sent by the bot to users. Requires no parameters.
/// # Documentation
/// <https://core.telegram.org/bots/api#getavailablegifts>
/// # Returns
/// Returns [`Gifts`] on success
#[derive(Debug, Default, Clone, Hash, PartialEq, Eq, Serialize)]
pub struct GetAvailableGifts {}

impl GetAvailableGifts {
    #[must_use]
    pub const fn new() -> Self {
        Self {}
    }
}

impl TelegramMethod for GetAvailableGifts {
    type Method = Self;
    type Return = Gifts;

    fn build_request<Client>(&self, _bot: &Bot<Client>) -> Request<'_, Self::Method> {
        Request::new("getAvailableGifts", self, None)
    }
}

impl AsRef<GetAvailableGifts> for GetAvailableGifts {
    fn as_ref(&self) -> &Self {
        self
    }
}
