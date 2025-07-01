use super::base::{Request, TelegramMethod};

use crate::client::Bot;

use serde::Serialize;
use serde_with::skip_serializing_none;

/// Changes the first and last name of a managed business account. Requires the `can_change_name` business bot right.
/// # Documentation
/// <https://core.telegram.org/bots/api#setbusinessaccountname>
/// # Returns
/// On success, `true` is returned
#[skip_serializing_none]
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize)]
pub struct SetBusinessAccountName {
    /// Unique identifier of the business connection
    pub business_connection_id: String,
    /// The new value of the first name for the business account; 1-64 characters
    pub first_name: String,
    /// The new value of the last name for the business account; 0-64 characters
    pub last_name: Option<String>,
}

impl SetBusinessAccountName {
    #[must_use]
    pub fn new(business_connection_id: impl Into<String>, first_name: impl Into<String>) -> Self {
        Self {
            business_connection_id: business_connection_id.into(),
            first_name: first_name.into(),
            last_name: None,
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
    pub fn first_name(self, val: impl Into<String>) -> Self {
        Self {
            first_name: val.into(),
            ..self
        }
    }

    #[must_use]
    pub fn last_name(self, val: impl Into<String>) -> Self {
        Self {
            last_name: Some(val.into()),
            ..self
        }
    }
}

impl SetBusinessAccountName {
    #[must_use]
    pub fn last_name_option(self, val: Option<impl Into<String>>) -> Self {
        Self {
            last_name: val.map(Into::into),
            ..self
        }
    }
}

impl TelegramMethod for SetBusinessAccountName {
    type Method = Self;
    type Return = bool;

    fn build_request<Client>(&self, _bot: &Bot<Client>) -> Request<Self::Method> {
        Request::new("setBusinessAccountName", self, None)
    }
}

impl AsRef<SetBusinessAccountName> for SetBusinessAccountName {
    fn as_ref(&self) -> &Self {
        self
    }
}
