use serde::{Deserialize, Serialize};
/// Media is a photo, available sizes of the photo
/// # Notes
/// This object represents a poll media from original field `photo`.
/// # Documentation
/// <https://core.telegram.org/bots/api#pollmedia>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PollMediaPhoto {
    /// Media is a photo, available sizes of the photo
    pub photo: Box<[crate::types::PhotoSize]>,
}
impl PollMediaPhoto {
    /// Creates a new `PollMediaPhoto`.
    ///
    /// # Arguments
    /// * `photo` - Media is a photo, available sizes of the photo
    #[must_use]
    pub fn new<T0Item: Into<crate::types::PhotoSize>, T0: IntoIterator<Item = T0Item>>(
        photo: T0,
    ) -> Self {
        Self {
            photo: photo.into_iter().map(Into::into).collect(),
        }
    }

    /// Media is a photo, available sizes of the photo
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

    /// Media is a photo, available sizes of the photo
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
}
