use serde::{Deserialize, Serialize};
/// A block with a photo, corresponding to the HTML tag <`photo`>.
/// # Documentation
/// <https://core.telegram.org/bots/api#richblockphoto>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RichBlockPhoto {
    /// Available sizes of the photo
    pub photo: Box<[crate::types::PhotoSize]>,
    /// `true`, if the media preview is covered by a spoiler animation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_spoiler: Option<bool>,
    /// Caption of the block
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<crate::types::RichBlockCaption>,
}
impl RichBlockPhoto {
    /// Creates a new `RichBlockPhoto`.
    ///
    /// # Arguments
    /// * `photo` - Available sizes of the photo
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0Item: Into<crate::types::PhotoSize>, T0: IntoIterator<Item = T0Item>>(
        photo: T0,
    ) -> Self {
        Self {
            photo: photo.into_iter().map(Into::into).collect(),
            has_spoiler: None,
            caption: None,
        }
    }

    /// Available sizes of the photo
    ///
    /// # Notes
    /// Adds multiple elements.
    #[must_use]
    pub fn photos<T: Into<Box<[crate::types::PhotoSize]>>>(mut self, val: T) -> Self {
        self.photo = self
            .photo
            .into_vec()
            .into_iter()
            .chain(val.into())
            .collect();
        self
    }

    /// Available sizes of the photo
    ///
    /// # Notes
    /// Adds a single element.
    #[must_use]
    pub fn photo<T: Into<crate::types::PhotoSize>>(mut self, val: T) -> Self {
        self.photo = self
            .photo
            .into_vec()
            .into_iter()
            .chain(Some(val.into()))
            .collect();
        self
    }

    /// `true`, if the media preview is covered by a spoiler animation
    #[must_use]
    pub fn has_spoiler<T: Into<bool>>(mut self, val: T) -> Self {
        self.has_spoiler = Some(val.into());
        self
    }

    /// `true`, if the media preview is covered by a spoiler animation
    #[must_use]
    pub fn has_spoiler_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.has_spoiler = val.map(Into::into);
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
