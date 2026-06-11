use serde::{Deserialize, Serialize};
/// A text with a phone number.
/// # Documentation
/// <https://core.telegram.org/bots/api#richtextphonenumber>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RichTextPhoneNumber {
    /// The text
    pub text: Box<crate::types::RichText>,
    /// The phone number
    pub phone_number: Box<str>,
}
impl RichTextPhoneNumber {
    /// Creates a new `RichTextPhoneNumber`.
    ///
    /// # Arguments
    /// * `text` - The text
    /// * `phone_number` - The phone number
    #[must_use]
    pub fn new<T0: Into<crate::types::RichText>, T1: Into<Box<str>>>(
        text: T0,
        phone_number: T1,
    ) -> Self {
        Self {
            text: Box::new(text.into()),
            phone_number: phone_number.into(),
        }
    }

    /// The text
    #[must_use]
    pub fn text<T: Into<crate::types::RichText>>(mut self, val: T) -> Self {
        self.text = Box::new(val.into());
        self
    }

    /// The phone number
    #[must_use]
    pub fn phone_number<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.phone_number = val.into();
        self
    }
}
