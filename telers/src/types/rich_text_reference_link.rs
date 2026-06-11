use serde::{Deserialize, Serialize};
/// A link to a reference.
/// # Documentation
/// <https://core.telegram.org/bots/api#richtextreferencelink>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RichTextReferenceLink {
    /// The link text
    pub text: Box<crate::types::RichText>,
    /// The name of the reference
    pub reference_name: Box<str>,
}
impl RichTextReferenceLink {
    /// Creates a new `RichTextReferenceLink`.
    ///
    /// # Arguments
    /// * `text` - The link text
    /// * `reference_name` - The name of the reference
    #[must_use]
    pub fn new<T0: Into<crate::types::RichText>, T1: Into<Box<str>>>(
        text: T0,
        reference_name: T1,
    ) -> Self {
        Self {
            text: Box::new(text.into()),
            reference_name: reference_name.into(),
        }
    }

    /// The link text
    #[must_use]
    pub fn text<T: Into<crate::types::RichText>>(mut self, val: T) -> Self {
        self.text = Box::new(val.into());
        self
    }

    /// The name of the reference
    #[must_use]
    pub fn reference_name<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.reference_name = val.into();
        self
    }
}
