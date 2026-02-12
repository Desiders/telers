use super::base::{Request, TelegramMethod};

use crate::client::Bot;

use serde::Serialize;
use serde_with::skip_serializing_none;

/// Changes the bio of a managed business account. Requires the `can_change_bio` business bot right.
/// # Documentation
/// <https://core.telegram.org/bots/api#setbusinessaccountbio>
/// # Returns
/// On success, `true` is returned
#[skip_serializing_none]
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize)]
pub struct SetBusinessAccountBio {
    /// Unique identifier of the business connection
    pub business_connection_id: String,
    /// The new value of the bio for the business account; 0-140 characters
    pub bio: Option<String>,
}

impl SetBusinessAccountBio {
    #[must_use]
    pub fn new(business_connection_id: impl Into<String>) -> Self {
        Self {
            business_connection_id: business_connection_id.into(),
            bio: None,
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
    pub fn bio(self, val: impl Into<String>) -> Self {
        Self {
            bio: Some(val.into()),
            ..self
        }
    }
}

impl SetBusinessAccountBio {
    #[must_use]
    pub fn bio_option(self, val: Option<impl Into<String>>) -> Self {
        Self {
            bio: val.map(Into::into),
            ..self
        }
    }
}

impl TelegramMethod for SetBusinessAccountBio {
    type Method = Self;
    type Return = bool;

    fn build_request<Client>(self, _bot: &Bot<Client>) -> Request<Self::Method> {
        Request::new("setBusinessAccountBio", self, None)
    }
}

impl AsRef<SetBusinessAccountBio> for SetBusinessAccountBio {
    fn as_ref(&self) -> &Self {
        self
    }
}
