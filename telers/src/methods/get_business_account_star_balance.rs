use super::base::{Request, TelegramMethod};
use crate::{client::Bot, types::StarAmount};

use serde::Serialize;

/// Returns the amount of Telegram Stars owned by a managed business account. Requires the `can_view_gifts_and_stars` business bot right.
/// # Documentation
/// <https://core.telegram.org/bots/api#getbusinessaccountstarbalance>
/// # Returns
/// On success, [`StarAmount`] is returned
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize)]
pub struct GetBusinessAccountStarBalance {
    /// Unique identifier of the business connection
    pub business_connection_id: String,
}

impl GetBusinessAccountStarBalance {
    #[must_use]
    pub fn new(business_connection_id: impl Into<String>) -> Self {
        Self {
            business_connection_id: business_connection_id.into(),
        }
    }

    #[must_use]
    pub fn business_connection_id(self, val: impl Into<String>) -> Self {
        Self {
            business_connection_id: val.into(),
        }
    }
}

impl TelegramMethod for GetBusinessAccountStarBalance {
    type Method = Self;
    type Return = StarAmount;

    fn build_request<Client>(self, _bot: &Bot<Client>) -> Request<Self::Method> {
        Request::new("getBusinessAccountStarBalance", self, None)
    }
}

impl AsRef<GetBusinessAccountStarBalance> for GetBusinessAccountStarBalance {
    fn as_ref(&self) -> &Self {
        self
    }
}
