use super::base::{Request, TelegramMethod};

use crate::{client::Bot, types::StarTransactions};

use serde::Serialize;
use serde_with::skip_serializing_none;

/// Returns the bot's Telegram Star transactions in chronological order.
/// # Documentation
/// <https://core.telegram.org/bots/api#getstartransactions>
/// # Returns
/// On success, returns a [`StarTransactions`] object.
#[skip_serializing_none]
#[derive(Debug, Default, Clone, Hash, PartialEq, Eq, Serialize)]
pub struct GetStarTransactions {
    /// Number of transactions to skip in the response
    pub offset: Option<i64>,
    /// The maximum number of transactions to be retrieved. Values between 1-100 are accepted. Defaults to 100.
    pub limit: Option<u8>,
}

impl GetStarTransactions {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn offset(self, val: i64) -> Self {
        Self {
            offset: Some(val),
            ..self
        }
    }

    #[must_use]
    pub fn limit(self, val: u8) -> Self {
        Self {
            limit: Some(val),
            ..self
        }
    }
}

impl GetStarTransactions {
    #[must_use]
    pub fn offset_option(self, val: Option<i64>) -> Self {
        Self {
            offset: val,
            ..self
        }
    }

    #[must_use]
    pub fn limit_option(self, val: Option<u8>) -> Self {
        Self { limit: val, ..self }
    }
}

impl TelegramMethod for GetStarTransactions {
    type Method = Self;
    type Return = StarTransactions;

    fn build_request<Client>(&self, _bot: &Bot<Client>) -> Request<'_, Self::Method> {
        Request::new("getStarTransactions", self, None)
    }
}

impl AsRef<GetStarTransactions> for GetStarTransactions {
    fn as_ref(&self) -> &Self {
        self
    }
}
