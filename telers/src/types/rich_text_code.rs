use serde::{Deserialize, Serialize};
/// A monowidth text.
/// # Documentation
/// <https://core.telegram.org/bots/api#richtextcode>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RichTextCode {
    /// The text
    pub text: Box<crate::types::RichText>,
}
impl RichTextCode {
    /// Creates a new `RichTextCode`.
    ///
    /// # Arguments
    /// * `text` - The text
    #[must_use]
    pub fn new<T0: Into<crate::types::RichText>>(text: T0) -> Self {
        Self {
            text: Box::new(text.into()),
        }
    }

    /// The text
    #[must_use]
    pub fn text<T: Into<crate::types::RichText>>(mut self, val: T) -> Self {
        self.text = Box::new(val.into());
        self
    }
}
