use serde::{Deserialize, Serialize};
/// A text with a link.
/// # Documentation
/// <https://core.telegram.org/bots/api#richtexturl>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RichTextUrl {
    /// The text
    pub text: Box<crate::types::RichText>,
    /// URL of the link
    pub url: Box<str>,
}
impl RichTextUrl {
    /// Creates a new `RichTextUrl`.
    ///
    /// # Arguments
    /// * `text` - The text
    /// * `url` - URL of the link
    #[must_use]
    pub fn new<T0: Into<crate::types::RichText>, T1: Into<Box<str>>>(text: T0, url: T1) -> Self {
        Self {
            text: Box::new(text.into()),
            url: url.into(),
        }
    }

    /// The text
    #[must_use]
    pub fn text<T: Into<crate::types::RichText>>(mut self, val: T) -> Self {
        self.text = Box::new(val.into());
        self
    }

    /// URL of the link
    #[must_use]
    pub fn url<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.url = val.into();
        self
    }
}
