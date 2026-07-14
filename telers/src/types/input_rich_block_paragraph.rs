use serde::{Deserialize, Serialize};
/// A text paragraph, corresponding to the HTML tag <`p`>.
/// # Documentation
/// <https://core.telegram.org/bots/api#inputrichblockparagraph>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InputRichBlockParagraph {
    /// Text of the block
    pub text: Box<crate::types::RichText>,
}
impl InputRichBlockParagraph {
    /// Creates a new `InputRichBlockParagraph`.
    ///
    /// # Arguments
    /// * `text` - Text of the block
    #[must_use]
    pub fn new<T0: Into<crate::types::RichText>>(text: T0) -> Self {
        Self {
            text: Box::new(text.into()),
        }
    }

    /// Text of the block
    #[must_use]
    pub fn text<T: Into<crate::types::RichText>>(mut self, val: T) -> Self {
        self.text = Box::new(val.into());
        self
    }
}
