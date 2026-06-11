use serde::{Deserialize, Serialize};
/// An anchor.
/// # Documentation
/// <https://core.telegram.org/bots/api#richtextanchor>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RichTextAnchor {
    /// The name of the anchor
    pub name: Box<str>,
}
impl RichTextAnchor {
    /// Creates a new `RichTextAnchor`.
    ///
    /// # Arguments
    /// * `name` - The name of the anchor
    #[must_use]
    pub fn new<T0: Into<Box<str>>>(name: T0) -> Self {
        Self {
            name: name.into(),
        }
    }

    /// The name of the anchor
    #[must_use]
    pub fn name<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.name = val.into();
        self
    }
}
