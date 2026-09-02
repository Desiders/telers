use serde::{Deserialize, Serialize};
/// A button.
/// # Documentation
/// <https://core.telegram.org/bots/api#richtextbutton>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RichTextButton {
    /// The button
    pub button: crate::types::RichMessageButton,
}
impl RichTextButton {
    /// Creates a new `RichTextButton`.
    ///
    /// # Arguments
    /// * `button` - The button
    #[must_use]
    pub fn new<T0: Into<crate::types::RichMessageButton>>(button: T0) -> Self {
        Self {
            button: button.into(),
        }
    }

    /// The button
    #[must_use]
    pub fn button<T: Into<crate::types::RichMessageButton>>(mut self, val: T) -> Self {
        self.button = val.into();
        self
    }
}
