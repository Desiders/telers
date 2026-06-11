use serde::{Deserialize, Serialize};
/// A hashtag.
/// # Documentation
/// <https://core.telegram.org/bots/api#richtexthashtag>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RichTextHashtag {
    /// The text
    pub text: Box<crate::types::RichText>,
    /// The hashtag
    pub hashtag: Box<str>,
}
impl RichTextHashtag {
    /// Creates a new `RichTextHashtag`.
    ///
    /// # Arguments
    /// * `text` - The text
    /// * `hashtag` - The hashtag
    #[must_use]
    pub fn new<T0: Into<crate::types::RichText>, T1: Into<Box<str>>>(
        text: T0,
        hashtag: T1,
    ) -> Self {
        Self {
            text: Box::new(text.into()),
            hashtag: hashtag.into(),
        }
    }

    /// The text
    #[must_use]
    pub fn text<T: Into<crate::types::RichText>>(mut self, val: T) -> Self {
        self.text = Box::new(val.into());
        self
    }

    /// The hashtag
    #[must_use]
    pub fn hashtag<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.hashtag = val.into();
        self
    }
}
