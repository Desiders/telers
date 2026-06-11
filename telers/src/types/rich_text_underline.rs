use serde::{Deserialize, Serialize};
/// An underlined text.
/// # Documentation
/// <https://core.telegram.org/bots/api#richtextunderline>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RichTextUnderline {
    /// The text
    pub text: Box<crate::types::RichText>,
}
impl RichTextUnderline {
    /// Creates a new `RichTextUnderline`.
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
