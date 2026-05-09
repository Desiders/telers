use serde::{Deserialize, Serialize};
/// Represents the content of a contact message to be sent as the result of an inline query.
/// # Documentation
/// <https://core.telegram.org/bots/api#inputcontactmessagecontent>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InputContactMessageContent {
    /// Contact's phone number
    pub phone_number: Box<str>,
    /// Contact's first name
    pub first_name: Box<str>,
    /// Contact's last name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<Box<str>>,
    /// Additional data about the contact in the form of a vCard, 0-2048 bytes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vcard: Option<Box<str>>,
}
impl InputContactMessageContent {
    /// Creates a new `InputContactMessageContent`.
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

    /// Additional data about the contact in the form of a vCard, 0-2048 bytes
    #[must_use]
    pub fn vcard<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.vcard = Some(val.into());
        self
    }

    /// Additional data about the contact in the form of a vCard, 0-2048 bytes
    #[must_use]
    pub fn vcard_option<T: Into<Box<str>>>(mut self, val: Option<T>) -> Self {
        self.vcard = val.map(Into::into);
        self
    }
}
