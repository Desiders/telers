use serde::{Deserialize, Serialize};
/// A block with a `Thinking...` placeholder, corresponding to the custom HTML tag <tg-thinking>. The block may be used only in sendRichMessageDraft, therefore it can't be received in messages. See <https://t.me/addemoji/AIActions> for examples of custom emoji that are recommended for usage in the block.
/// # Documentation
/// <https://core.telegram.org/bots/api#richblockthinking>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RichBlockThinking {
    /// Text of the block. See <https://t.me/addemoji/AIActions> for examples of custom emoji that are recommended for usage in the block.
    pub text: Box<crate::types::RichText>,
}
impl RichBlockThinking {
    /// Creates a new `RichBlockThinking`.
    ///
    /// # Arguments
    /// * `text` - Text of the block. See <https://t.me/addemoji/AIActions> for examples of custom emoji that are recommended for usage in the block.
    #[must_use]
    pub fn new<T0: Into<crate::types::RichText>>(text: T0) -> Self {
        Self {
            text: Box::new(text.into()),
        }
    }

    /// Text of the block. See <https://t.me/addemoji/AIActions> for examples of custom emoji that are recommended for usage in the block.
    #[must_use]
    pub fn text<T: Into<crate::types::RichText>>(mut self, val: T) -> Self {
        self.text = Box::new(val.into());
        self
    }
}
