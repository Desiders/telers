/// A block with an anchor, corresponding to the HTML tag <`a`> with the attribute name.
/// # Documentation
/// <https://core.telegram.org/bots/api#inputrichblockanchor>
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deser", derive(deser::Serialize, deser::Deserialize))]
#[cfg_attr(not(feature = "deser"), derive(serde::Serialize, serde::Deserialize))]
pub struct InputRichBlockAnchor {
    /// The name of the anchor
    pub name: Box<str>,
}
impl InputRichBlockAnchor {
    /// Creates a new `InputRichBlockAnchor`.
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
