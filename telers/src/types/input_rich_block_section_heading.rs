use serde::{Deserialize, Serialize};
/// A section heading, corresponding to the HTML tags <`h1`>, <`h2`>, <`h3`>, <`h4`>, <`h5`>, or <`h6`>.
/// # Documentation
/// <https://core.telegram.org/bots/api#inputrichblocksectionheading>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InputRichBlockSectionHeading {
    /// Text of the block
    pub text: Box<crate::types::RichText>,
    /// Relative size of the text font; 1-6, 1 is the largest, 6 is the smallest
    pub size: u8,
}
impl InputRichBlockSectionHeading {
    /// Creates a new `InputRichBlockSectionHeading`.
    ///
    /// # Arguments
    /// * `text` - Text of the block
    /// * `size` - Relative size of the text font; 1-6, 1 is the largest, 6 is the smallest
    #[must_use]
    pub fn new<T0: Into<crate::types::RichText>, T1: Into<u8>>(text: T0, size: T1) -> Self {
        Self {
            text: Box::new(text.into()),
            size: size.into(),
        }
    }

    /// Text of the block
    #[must_use]
    pub fn text<T: Into<crate::types::RichText>>(mut self, val: T) -> Self {
        self.text = Box::new(val.into());
        self
    }

    /// Relative size of the text font; 1-6, 1 is the largest, 6 is the smallest
    #[must_use]
    pub fn size<T: Into<u8>>(mut self, val: T) -> Self {
        self.size = val.into();
        self
    }
}
