use serde::{Deserialize, Serialize};
/// This object represents a phone contact.
/// # Documentation
/// <https://core.telegram.org/bots/api#contact>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Contact {
    /// Contact's phone number
    pub phone_number: Box<str>,
    /// Contact's first name
    pub first_name: Box<str>,
    /// Contact's last name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<Box<str>>,
    /// Contact's user identifier in Telegram. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a 64-bit integer or double-precision float type are safe for storing this identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
    /// Additional data about the contact in the form of a `vCard`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vcard: Option<Box<str>>,
}
impl Contact {
    /// Creates a new `Contact`.
    ///
    /// # Arguments
    /// * `phone_number` - Contact's phone number
    /// * `first_name` - Contact's first name
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0: Into<Box<str>>, T1: Into<Box<str>>>(phone_number: T0, first_name: T1) -> Self {
        Self {
            phone_number: phone_number.into(),
            first_name: first_name.into(),
            last_name: None,
            user_id: None,
            vcard: None,
        }
    }

    /// Contact's phone number
    #[must_use]
    pub fn phone_number<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.phone_number = val.into();
        self
    }

    /// Contact's first name
    #[must_use]
    pub fn first_name<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.first_name = val.into();
        self
    }

    /// Contact's last name
    #[must_use]
    pub fn last_name<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.last_name = Some(val.into());
        self
    }

    /// Contact's last name
    #[must_use]
    pub fn last_name_option<T: Into<Box<str>>>(mut self, val: Option<T>) -> Self {
        self.last_name = val.map(Into::into);
        self
    }

    /// Contact's user identifier in Telegram. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a 64-bit integer or double-precision float type are safe for storing this identifier.
    #[must_use]
    pub fn user_id<T: Into<i64>>(mut self, val: T) -> Self {
        self.user_id = Some(val.into());
        self
    }

    /// Contact's user identifier in Telegram. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a 64-bit integer or double-precision float type are safe for storing this identifier.
    #[must_use]
    pub fn user_id_option<T: Into<i64>>(mut self, val: Option<T>) -> Self {
        self.user_id = val.map(Into::into);
        self
    }

    /// Additional data about the contact in the form of a `vCard`
    #[must_use]
    pub fn vcard<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.vcard = Some(val.into());
        self
    }

    /// Additional data about the contact in the form of a `vCard`
    #[must_use]
    pub fn vcard_option<T: Into<Box<str>>>(mut self, val: Option<T>) -> Self {
        self.vcard = val.map(Into::into);
        self
    }
}
