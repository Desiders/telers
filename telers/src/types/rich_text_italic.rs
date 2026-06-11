use serde::{Deserialize, Serialize};
/// An italicized text.
/// # Documentation
/// <https://core.telegram.org/bots/api#richtextitalic>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RichTextItalic {
    /// The text
    pub text: Box<crate::types::RichText>,
}
impl RichTextItalic {
    /// Creates a new `RichTextItalic`.
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
