use serde::{Deserialize, Serialize};
/// This object represents an inline keyboard button that copies specified text to the clipboard.
/// # Documentation
/// <https://core.telegram.org/bots/api#copytextbutton>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CopyTextButton {
    /// The text to be copied to the clipboard; 1-256 characters
    pub text: Box<str>,
}
impl CopyTextButton {
    /// Creates a new `CopyTextButton`.
    ///
    /// # Arguments
    /// * `text` - The text to be copied to the clipboard; 1-256 characters
    #[must_use]
    pub fn new<T0: Into<Box<str>>>(text: T0) -> Self {
        Self { text: text.into() }
    }

    /// The text to be copied to the clipboard; 1-256 characters
    #[must_use]
    pub fn text<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.text = val.into();
        self
    }
}
