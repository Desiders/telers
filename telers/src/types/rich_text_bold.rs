use serde::{Deserialize, Serialize};
/// A bold text.
/// # Documentation
/// <https://core.telegram.org/bots/api#richtextbold>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RichTextBold {
    /// The text
    pub text: Box<crate::types::RichText>,
}
impl RichTextBold {
    /// Creates a new `RichTextBold`.
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
