use serde::{Deserialize, Serialize};
/// A preformatted text block, corresponding to the nested HTML tags <`pre`> and <`code`>.
/// # Documentation
/// <https://core.telegram.org/bots/api#inputrichblockpreformatted>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InputRichBlockPreformatted {
    /// Text of the block
    pub text: Box<crate::types::RichText>,
    /// The programming language of the text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<Box<str>>,
}
impl InputRichBlockPreformatted {
    /// Creates a new `InputRichBlockPreformatted`.
    ///
    /// # Arguments
    /// * `text` - Text of the block
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0: Into<crate::types::RichText>>(text: T0) -> Self {
        Self {
            text: Box::new(text.into()),
            language: None,
        }
    }

    /// Text of the block
    #[must_use]
    pub fn text<T: Into<crate::types::RichText>>(mut self, val: T) -> Self {
        self.text = Box::new(val.into());
        self
    }

    /// The programming language of the text
    #[must_use]
    pub fn language<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.language = Some(val.into());
        self
    }

    /// The programming language of the text
    #[must_use]
    pub fn language_option<T: Into<Box<str>>>(mut self, val: Option<T>) -> Self {
        self.language = val.map(Into::into);
        self
    }
}
