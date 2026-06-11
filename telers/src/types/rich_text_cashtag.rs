use serde::{Deserialize, Serialize};
/// A cashtag.
/// # Documentation
/// <https://core.telegram.org/bots/api#richtextcashtag>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RichTextCashtag {
    /// The text
    pub text: Box<crate::types::RichText>,
    /// The cashtag
    pub cashtag: Box<str>,
}
impl RichTextCashtag {
    /// Creates a new `RichTextCashtag`.
    ///
    /// # Arguments
    /// * `text` - The text
    /// * `cashtag` - The cashtag
    #[must_use]
    pub fn new<T0: Into<crate::types::RichText>, T1: Into<Box<str>>>(
        text: T0,
        cashtag: T1,
    ) -> Self {
        Self {
            text: Box::new(text.into()),
            cashtag: cashtag.into(),
        }
    }

    /// The text
    #[must_use]
    pub fn text<T: Into<crate::types::RichText>>(mut self, val: T) -> Self {
        self.text = Box::new(val.into());
        self
    }

    /// The cashtag
    #[must_use]
    pub fn cashtag<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.cashtag = val.into();
        self
    }
}
