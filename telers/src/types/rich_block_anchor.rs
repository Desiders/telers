use serde::{Deserialize, Serialize};
/// A block with an anchor, corresponding to the HTML tag <`a`> with the attribute name.
/// # Documentation
/// <https://core.telegram.org/bots/api#richblockanchor>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RichBlockAnchor {
    /// The name of the anchor
    pub name: Box<str>,
}
impl RichBlockAnchor {
    /// Creates a new `RichBlockAnchor`.
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
