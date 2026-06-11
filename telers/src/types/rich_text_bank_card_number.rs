use serde::{Deserialize, Serialize};
/// A text with a bank card number.
/// # Documentation
/// <https://core.telegram.org/bots/api#richtextbankcardnumber>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RichTextBankCardNumber {
    /// The text
    pub text: Box<crate::types::RichText>,
    /// The bank card number
    pub bank_card_number: Box<str>,
}
impl RichTextBankCardNumber {
    /// Creates a new `RichTextBankCardNumber`.
    ///
    /// # Arguments
    /// * `text` - The text
    /// * `bank_card_number` - The bank card number
    #[must_use]
    pub fn new<T0: Into<crate::types::RichText>, T1: Into<Box<str>>>(
        text: T0,
        bank_card_number: T1,
    ) -> Self {
        Self {
            text: Box::new(text.into()),
            bank_card_number: bank_card_number.into(),
        }
    }

    /// The text
    #[must_use]
    pub fn text<T: Into<crate::types::RichText>>(mut self, val: T) -> Self {
        self.text = Box::new(val.into());
        self
    }

    /// The bank card number
    #[must_use]
    pub fn bank_card_number<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.bank_card_number = val.into();
        self
    }
}
