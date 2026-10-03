/// A quotation with centered text, loosely corresponding to the HTML tag <`aside`>.
/// # Documentation
/// <https://core.telegram.org/bots/api#inputrichblockpullquotation>
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deser", derive(deser::Serialize, deser::Deserialize))]
#[cfg_attr(not(feature = "deser"), derive(serde::Serialize, serde::Deserialize))]
pub struct InputRichBlockPullQuotation {
    /// Text of the block
    pub text: Box<crate::types::RichText>,
    /// Credit of the block
    #[cfg_attr(not(feature = "deser"), serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "deser", deser(skip_serializing_if = Option::is_none))]
    pub credit: Option<Box<crate::types::RichText>>,
}
impl InputRichBlockPullQuotation {
    /// Creates a new `InputRichBlockPullQuotation`.
    ///
    /// # Arguments
    /// * `text` - Text of the block
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0: Into<crate::types::RichText>>(text: T0) -> Self {
        Self {
            text: Box::new(text.into()),
            credit: None,
        }
    }

    /// Text of the block
    #[must_use]
    pub fn text<T: Into<crate::types::RichText>>(mut self, val: T) -> Self {
        self.text = Box::new(val.into());
        self
    }

    /// Credit of the block
    #[must_use]
    pub fn credit<T: Into<crate::types::RichText>>(mut self, val: T) -> Self {
        self.credit = Some(Box::new(val.into()));
        self
    }

    /// Credit of the block
    #[must_use]
    pub fn credit_option<T: Into<crate::types::RichText>>(mut self, val: Option<T>) -> Self {
        self.credit = val.map(|val| Box::new(val.into()));
        self
    }
}
