use serde::{Deserialize, Serialize};
/// A footer, corresponding to the HTML tag <`footer`>.
/// # Documentation
/// <https://core.telegram.org/bots/api#inputrichblockfooter>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InputRichBlockFooter {
    /// Text of the block
    pub text: Box<crate::types::RichText>,
}
impl InputRichBlockFooter {
    /// Creates a new `InputRichBlockFooter`.
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
