use serde::{Deserialize, Serialize};
/// A block with a general file, corresponding to the custom HTML tag `<tg-document>`.
/// # Documentation
/// <https://core.telegram.org/bots/api#richblockdocument>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RichBlockDocument {
    /// The document
    pub document: Box<crate::types::Document>,
    /// Caption of the block
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<crate::types::RichBlockCaption>,
}
impl RichBlockDocument {
    /// Creates a new `RichBlockDocument`.
    ///
    /// # Arguments
    /// * `document` - The document
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0: Into<crate::types::Document>>(document: T0) -> Self {
        Self {
            document: Box::new(document.into()),
            caption: None,
        }
    }

    /// The document
    #[must_use]
    pub fn document<T: Into<crate::types::Document>>(mut self, val: T) -> Self {
        self.document = Box::new(val.into());
        self
    }

    /// Caption of the block
    #[must_use]
    pub fn caption<T: Into<crate::types::RichBlockCaption>>(mut self, val: T) -> Self {
        self.caption = Some(val.into());
        self
    }

    /// Caption of the block
    #[must_use]
    pub fn caption_option<T: Into<crate::types::RichBlockCaption>>(
        mut self,
        val: Option<T>,
    ) -> Self {
        self.caption = val.map(Into::into);
        self
    }
}
