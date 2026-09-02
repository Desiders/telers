use serde::{Deserialize, Serialize};
/// A block quotation, corresponding to the HTML tag <`blockquote`> with custom attribute `expandable`.
/// # Documentation
/// <https://core.telegram.org/bots/api#richblockexpandableblockquotation>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RichBlockExpandableBlockQuotation {
    /// Content of the block
    pub text: Box<crate::types::RichText>,
    /// Credit of the block
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credit: Option<Box<crate::types::RichText>>,
}
impl RichBlockExpandableBlockQuotation {
    /// Creates a new `RichBlockExpandableBlockQuotation`.
    ///
    /// # Arguments
    /// * `text` - Content of the block
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

    /// Content of the block
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
