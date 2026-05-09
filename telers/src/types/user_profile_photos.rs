use serde::{Deserialize, Serialize};
/// This object represent a user's profile pictures.
/// # Documentation
/// <https://core.telegram.org/bots/api#userprofilephotos>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserProfilePhotos {
    /// Total number of profile pictures the target user has
    pub total_count: i64,
    /// Requested profile pictures (in up to 4 sizes each)
    pub photos: Box<[Box<[crate::types::PhotoSize]>]>,
}
impl UserProfilePhotos {
    /// Creates a new `UserProfilePhotos`.
    ///
    /// # Arguments
    /// * `total_count` - Total number of profile pictures the target user has
    /// * `photos` - Requested profile pictures (in up to 4 sizes each)
    #[must_use]
    pub fn new<
        T0: Into<i64>,
        T1Item: Into<Box<[crate::types::PhotoSize]>>,
        T1: IntoIterator<Item = T1Item>,
    >(
        total_count: T0,
        photos: T1,
    ) -> Self {
        Self {
            total_count: total_count.into(),
            photos: photos.into_iter().map(Into::into).collect(),
        }
    }

    /// Total number of profile pictures the target user has
    #[must_use]
    pub fn total_count<T: Into<i64>>(mut self, val: T) -> Self {
        self.total_count = val.into();
        self
    }

    /// Requested profile pictures (in up to 4 sizes each)
    ///
    /// # Notes
    /// Adds multiple elements.
    #[must_use]
    pub fn photos<T: Into<Box<[Box<[crate::types::PhotoSize]>]>>>(mut self, val: T) -> Self {
        self.photos = self
            .photos
            .into_vec()
            .into_iter()
            .chain(val.into())
            .collect();
        self
    }

    /// Requested profile pictures (in up to 4 sizes each)
    ///
    /// # Notes
    /// Adds a single element.
    #[must_use]
    pub fn photo<T: Into<Box<[crate::types::PhotoSize]>>>(mut self, val: T) -> Self {
        self.photos = self
            .photos
            .into_vec()
            .into_iter()
            .chain(Some(val.into()))
            .collect();
        self
    }
}
