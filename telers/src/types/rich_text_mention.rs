use serde::{Deserialize, Serialize};
/// A mention by a username.
/// # Documentation
/// <https://core.telegram.org/bots/api#richtextmention>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RichTextMention {
    /// The text
    pub text: Box<crate::types::RichText>,
    /// The username
    pub username: Box<str>,
}
impl RichTextMention {
    /// Creates a new `RichTextMention`.
    ///
    /// # Arguments
    /// * `text` - The text
    /// * `username` - The username
    #[must_use]
    pub fn new<T0: Into<crate::types::RichText>, T1: Into<Box<str>>>(
        text: T0,
        username: T1,
    ) -> Self {
        Self {
            text: Box::new(text.into()),
            username: username.into(),
        }
    }

    /// The text
    #[must_use]
    pub fn text<T: Into<crate::types::RichText>>(mut self, val: T) -> Self {
        self.text = Box::new(val.into());
        self
    }

    /// The username
    #[must_use]
    pub fn username<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.username = val.into();
        self
    }
}
