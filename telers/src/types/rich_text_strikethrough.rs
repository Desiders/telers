use serde::{Deserialize, Serialize};
/// A strikethrough text.
/// # Documentation
/// <https://core.telegram.org/bots/api#richtextstrikethrough>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RichTextStrikethrough {
    /// The text
    pub text: Box<crate::types::RichText>,
}
impl RichTextStrikethrough {
    /// Creates a new `RichTextStrikethrough`.
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
