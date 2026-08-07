use serde::{Deserialize, Serialize};
/// Describes a keyboard button to be used by a user of a Mini App.
/// # Documentation
/// <https://core.telegram.org/bots/api#preparedkeyboardbutton>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PreparedKeyboardButton {
    /// Unique identifier of the keyboard button
    pub id: Box<str>,
}
impl PreparedKeyboardButton {
    /// Creates a new `PreparedKeyboardButton`.
    ///
    /// # Arguments
    /// * `id` - Unique identifier of the keyboard button
    #[must_use]
    pub fn new<T0: Into<Box<str>>>(id: T0) -> Self {
        Self { id: id.into() }
    }

    /// Unique identifier of the keyboard button
    #[must_use]
    pub fn id<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.id = val.into();
        self
    }
}
