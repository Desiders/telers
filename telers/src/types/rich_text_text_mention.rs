use serde::{Deserialize, Serialize};
/// A mention of a Telegram user by their identifier.
/// # Documentation
/// <https://core.telegram.org/bots/api#richtexttextmention>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RichTextTextMention {
    /// The text
    pub text: Box<crate::types::RichText>,
    /// The mentioned user
    pub user: Box<crate::types::User>,
}
impl RichTextTextMention {
    /// Creates a new `RichTextTextMention`.
    ///
    /// # Arguments
    /// * `text` - The text
    /// * `user` - The mentioned user
    #[must_use]
    pub fn new<T0: Into<crate::types::RichText>, T1: Into<crate::types::User>>(
        text: T0,
        user: T1,
    ) -> Self {
        Self {
            text: Box::new(text.into()),
            user: Box::new(user.into()),
        }
    }

    /// The text
    #[must_use]
    pub fn text<T: Into<crate::types::RichText>>(mut self, val: T) -> Self {
        self.text = Box::new(val.into());
        self
    }

    /// The mentioned user
    #[must_use]
    pub fn user<T: Into<crate::types::User>>(mut self, val: T) -> Self {
        self.user = Box::new(val.into());
        self
    }
}
