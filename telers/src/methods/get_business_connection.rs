use super::base::{Request, TelegramMethod};

use crate::{client::Bot, types::BusinessConnection};

use serde::Serialize;
use serde_with::skip_serializing_none;

/// Use this method to get information about the connection of the bot with a business account
/// # Documentation
/// <https://core.telegram.org/bots/api#getbusinessconnection>
/// # Returns
/// Returns [`BusinessConnection`] on success
#[skip_serializing_none]
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize)]
pub struct GetBusinessConnection {
    /// Unique identifier of the business connection
    pub business_connection_id: String,
}

impl GetBusinessConnection {
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

impl TelegramMethod for GetBusinessConnection {
    type Method = Self;
    type Return = BusinessConnection;

    fn build_request<Client>(&self, _bot: &Bot<Client>) -> Request<'_, Self::Method> {
        Request::new("getBusinessConnection", self, None)
    }
}

impl AsRef<GetBusinessConnection> for GetBusinessConnection {
    fn as_ref(&self) -> &Self {
        self
    }
}
