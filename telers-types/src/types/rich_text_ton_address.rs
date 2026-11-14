/// An address in the TON blockchain.
/// # Documentation
/// <https://core.telegram.org/bots/api#richtexttonaddress>
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deser", derive(deser::Serialize, deser::Deserialize))]
#[cfg_attr(not(feature = "deser"), derive(serde::Serialize, serde::Deserialize))]
pub struct RichTextTonAddress {
    /// Text of the address
    pub text: Box<crate::types::RichText>,
    /// The address
    pub address: Box<str>,
}
impl RichTextTonAddress {
    /// Creates a new `RichTextTonAddress`.
    ///
    /// # Arguments
    /// * `text` - Text of the address
    /// * `address` - The address
    #[must_use]
    pub fn new<T0: Into<crate::types::RichText>, T1: Into<Box<str>>>(
        text: T0,
        address: T1,
    ) -> Self {
        Self {
            text: Box::new(text.into()),
            address: address.into(),
        }
    }

    /// Text of the address
    #[must_use]
    pub fn text<T: Into<crate::types::RichText>>(mut self, val: T) -> Self {
        self.text = Box::new(val.into());
        self
    }

    /// The address
    #[must_use]
    pub fn address<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.address = val.into();
        self
    }
}
