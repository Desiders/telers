use super::base::{Request, TelegramMethod};
use crate::client::Bot;

use serde::Serialize;

/// Transfers Telegram Stars from the business account balance to the bot's balance. Requires the `can_transfer_stars` business bot right.
/// # Documentation
/// <https://core.telegram.org/bots/api#transferbusinessaccountstars>
/// # Returns
/// On success, `true` is returned
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize)]
pub struct TransferBusinessAccountStars {
    /// Unique identifier of the business connection
    pub business_connection_id: String,
    /// Number of Telegram Stars to transfer; 1-10000
    pub star_count: u16,
}

impl TransferBusinessAccountStars {
    #[must_use]
    pub fn new(business_connection_id: impl Into<String>, star_count: u16) -> Self {
        Self {
            business_connection_id: business_connection_id.into(),
            star_count,
        }
    }

    #[must_use]
    pub fn business_connection_id(self, val: impl Into<String>) -> Self {
        Self {
            business_connection_id: val.into(),
            ..self
        }
    }

    #[must_use]
    pub fn star_count(self, val: u16) -> Self {
        Self {
            star_count: val,
            ..self
        }
    }
}

impl TelegramMethod for TransferBusinessAccountStars {
    type Method = Self;
    type Return = bool;

    fn build_request<Client>(self, _bot: &Bot<Client>) -> Request<Self::Method> {
        Request::new("transferBusinessAccountStars", self, None)
    }
}

impl AsRef<TransferBusinessAccountStars> for TransferBusinessAccountStars {
    fn as_ref(&self) -> &Self {
        self
    }
}
