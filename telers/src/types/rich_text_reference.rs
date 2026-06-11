use serde::{Deserialize, Serialize};
/// A reference.
/// # Documentation
/// <https://core.telegram.org/bots/api#richtextreference>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RichTextReference {
    /// Text of the reference
    pub text: Box<crate::types::RichText>,
    /// The name of the reference
    pub name: Box<str>,
}
impl RichTextReference {
    /// Creates a new `RichTextReference`.
    ///
    /// # Arguments
    /// * `text` - Text of the reference
    /// * `name` - The name of the reference
    #[must_use]
    pub fn new<T0: Into<crate::types::RichText>, T1: Into<Box<str>>>(text: T0, name: T1) -> Self {
        Self {
            text: Box::new(text.into()),
            name: name.into(),
        }
    }

    /// Text of the reference
    #[must_use]
    pub fn text<T: Into<crate::types::RichText>>(mut self, val: T) -> Self {
        self.text = Box::new(val.into());
        self
    }

    /// The name of the reference
    #[must_use]
    pub fn name<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.name = val.into();
        self
    }
}
