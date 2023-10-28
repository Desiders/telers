use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Represents the [`content`](https://core.telegram.org/bots/api#inputmessagecontent) of a contact message to be sent as the result of an inline query.
/// # Documentation
/// <https://core.telegram.org/bots/api#inputcontactmessagecontent>
#[skip_serializing_none]
#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub struct InputContactMessageContent {
    /// Contact's phone number
    pub phone_number: String,
    /// Contact's first name
    pub first_name: String,
    /// Contact's last name
    pub last_name: Option<String>,
    /// Additional data about the contact in the form of a [`vCard`](https://en.wikipedia.org/wiki/VCard), 0-2048 bytes
    pub vcard: Option<String>,
}

impl InputContactMessageContent {
    #[must_use]
    pub fn new(phone_number: impl Into<String>, first_name: impl Into<String>) -> Self {
        Self {
            phone_number: phone_number.into(),
            first_name: first_name.into(),
            last_name: None,
            vcard: None,
        }
    }

    #[must_use]
    pub fn phone_number(self, val: impl Into<String>) -> Self {
        Self {
            phone_number: val.into(),
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

    #[must_use]
    pub fn vcard(self, val: impl Into<String>) -> Self {
        Self {
            vcard: Some(val.into()),
            ..self
        }
    }
}
