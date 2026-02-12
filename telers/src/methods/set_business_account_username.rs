use super::base::{Request, TelegramMethod};

use crate::client::Bot;

use serde::Serialize;
use serde_with::skip_serializing_none;

/// Changes the username of a managed business account. Requires the `can_change_username` business bot right.
/// # Documentation
/// <https://core.telegram.org/bots/api#setbusinessaccountusername>
/// # Returns
/// On success, `true` is returned
#[skip_serializing_none]
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize)]
pub struct SetBusinessAccountUsername {
    /// Unique identifier of the business connection
    pub business_connection_id: String,
    /// The new value of the username for the business account; 0-32 characters
    pub username: Option<String>,
}

impl SetBusinessAccountUsername {
    #[must_use]
    pub fn new(business_connection_id: impl Into<String>) -> Self {
        Self {
            business_connection_id: business_connection_id.into(),
            username: None,
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
    pub fn username(self, val: impl Into<String>) -> Self {
        Self {
            username: Some(val.into()),
            ..self
        }
    }
}

impl SetBusinessAccountUsername {
    #[must_use]
    pub fn username_option(self, val: Option<impl Into<String>>) -> Self {
        Self {
            username: val.map(Into::into),
            ..self
        }
    }
}

impl TelegramMethod for SetBusinessAccountUsername {
    type Method = Self;
    type Return = bool;

    fn build_request<Client>(self, _bot: &Bot<Client>) -> Request<Self::Method> {
        Request::new("setBusinessAccountUsername", self, None)
    }
}

impl AsRef<SetBusinessAccountUsername> for SetBusinessAccountUsername {
    fn as_ref(&self) -> &Self {
        self
    }
}
