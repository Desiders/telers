use serde::{Deserialize, Serialize};
/// A subscript text.
/// # Documentation
/// <https://core.telegram.org/bots/api#richtextsubscript>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RichTextSubscript {
    /// The text
    pub text: Box<crate::types::RichText>,
}
impl RichTextSubscript {
    /// Creates a new `RichTextSubscript`.
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
