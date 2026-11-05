/// A block with a general file, corresponding to the custom HTML tag `<tg-document>`.
/// # Documentation
/// <https://core.telegram.org/bots/api#inputrichblockdocument>
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deser", derive(deser::Serialize, deser::Deserialize))]
#[cfg_attr(not(feature = "deser"), derive(serde::Serialize, serde::Deserialize))]
pub struct InputRichBlockDocument {
    /// The document. Caption is ignored.
    pub document: crate::types::InputMediaDocument,
    /// Caption of the block
    #[cfg_attr(not(feature = "deser"), serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "deser", deser(skip_serializing_if = Option::is_none))]
    pub caption: Option<crate::types::RichBlockCaption>,
}
impl InputRichBlockDocument {
    /// Creates a new `InputRichBlockDocument`.
    ///
    /// # Arguments
    /// * `document` - The document. Caption is ignored.
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0: Into<crate::types::InputMediaDocument>>(document: T0) -> Self {
        Self {
            document: document.into(),
            caption: None,
        }
    }

    /// The document. Caption is ignored.
    #[must_use]
    pub fn document<T: Into<crate::types::InputMediaDocument>>(mut self, val: T) -> Self {
        self.document = val.into();
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
