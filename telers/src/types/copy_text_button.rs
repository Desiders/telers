use serde::{Deserialize, Serialize};

/// This object represents an inline keyboard button that copies specified text to the clipboard.
/// # Documentation
/// <https://core.telegram.org/bots/api#copytextbutton>
#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub struct CopyTextButton {
    /// The text to be copied to the clipboard; 1-256 characters
    pub text: String,
}

impl CopyTextButton {
    #[must_use]
    pub fn new(text: impl Into<String>) -> Self {
        Self { text: text.into() }
    }

    #[must_use]
    pub fn text(self, val: impl Into<String>) -> Self {
        Self { text: val.into() }
    }
}
