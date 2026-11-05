/// A block with a photo, corresponding to the HTML tag <`img`>.
/// # Documentation
/// <https://core.telegram.org/bots/api#inputrichblockphoto>
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deser", derive(deser::Serialize, deser::Deserialize))]
#[cfg_attr(not(feature = "deser"), derive(serde::Serialize, serde::Deserialize))]
pub struct InputRichBlockPhoto {
    /// The photo. Caption is ignored.
    pub photo: crate::types::InputMediaPhoto,
    /// Caption of the block
    #[cfg_attr(not(feature = "deser"), serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "deser", deser(skip_serializing_if = Option::is_none))]
    pub caption: Option<crate::types::RichBlockCaption>,
}
impl InputRichBlockPhoto {
    /// Creates a new `InputRichBlockPhoto`.
    ///
    /// # Arguments
    /// * `photo` - The photo. Caption is ignored.
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0: Into<crate::types::InputMediaPhoto>>(photo: T0) -> Self {
        Self {
            photo: photo.into(),
            caption: None,
        }
    }

    /// The photo. Caption is ignored.
    #[must_use]
    pub fn photo<T: Into<crate::types::InputMediaPhoto>>(mut self, val: T) -> Self {
        self.photo = val.into();
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
