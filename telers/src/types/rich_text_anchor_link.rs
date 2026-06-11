use serde::{Deserialize, Serialize};
/// A link to an anchor.
/// # Documentation
/// <https://core.telegram.org/bots/api#richtextanchorlink>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RichTextAnchorLink {
    /// The link text
    pub text: Box<crate::types::RichText>,
    /// The name of the anchor. If the name is empty, then the link brings back to the top of the message.
    pub anchor_name: Box<str>,
}
impl RichTextAnchorLink {
    /// Creates a new `RichTextAnchorLink`.
    ///
    /// # Arguments
    /// * `text` - The link text
    /// * `anchor_name` - The name of the anchor. If the name is empty, then the link brings back to the top of the message.
    #[must_use]
    pub fn new<T0: Into<crate::types::RichText>, T1: Into<Box<str>>>(
        text: T0,
        anchor_name: T1,
    ) -> Self {
        Self {
            text: Box::new(text.into()),
            anchor_name: anchor_name.into(),
        }
    }

    /// The link text
    #[must_use]
    pub fn text<T: Into<crate::types::RichText>>(mut self, val: T) -> Self {
        self.text = Box::new(val.into());
        self
    }

    /// The name of the anchor. If the name is empty, then the link brings back to the top of the message.
    #[must_use]
    pub fn anchor_name<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.anchor_name = val.into();
        self
    }
}
