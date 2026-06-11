use serde::{Deserialize, Serialize};
/// A text with an email address.
/// # Documentation
/// <https://core.telegram.org/bots/api#richtextemailaddress>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RichTextEmailAddress {
    /// The text
    pub text: Box<crate::types::RichText>,
    /// The email address
    pub email_address: Box<str>,
}
impl RichTextEmailAddress {
    /// Creates a new `RichTextEmailAddress`.
    ///
    /// # Arguments
    /// * `text` - The text
    /// * `email_address` - The email address
    #[must_use]
    pub fn new<T0: Into<crate::types::RichText>, T1: Into<Box<str>>>(
        text: T0,
        email_address: T1,
    ) -> Self {
        Self {
            text: Box::new(text.into()),
            email_address: email_address.into(),
        }
    }

    /// The text
    #[must_use]
    pub fn text<T: Into<crate::types::RichText>>(mut self, val: T) -> Self {
        self.text = Box::new(val.into());
        self
    }

    /// The email address
    #[must_use]
    pub fn email_address<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.email_address = val.into();
        self
    }
}
