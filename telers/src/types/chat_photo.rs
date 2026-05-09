use serde::{Deserialize, Serialize};
/// This object represents a chat photo.
/// # Documentation
/// <https://core.telegram.org/bots/api#chatphoto>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChatPhoto {
    /// File identifier of small (160x160) chat photo. This `file_id` can be used only for photo download and only for as long as the photo is not changed.
    pub small_file_id: Box<str>,
    /// Unique file identifier of small (160x160) chat photo, which is supposed to be the same over time and for different bots. Can't be used to download or reuse the file.
    pub small_file_unique_id: Box<str>,
    /// File identifier of big (640x640) chat photo. This `file_id` can be used only for photo download and only for as long as the photo is not changed.
    pub big_file_id: Box<str>,
    /// Unique file identifier of big (640x640) chat photo, which is supposed to be the same over time and for different bots. Can't be used to download or reuse the file.
    pub big_file_unique_id: Box<str>,
}
impl ChatPhoto {
    /// Creates a new `ChatPhoto`.
    ///
    /// # Arguments
    /// * `small_file_id` - File identifier of small (160x160) chat photo. This `file_id` can be used only for photo download and only for as long as the photo is not changed.
    /// * `small_file_unique_id` - Unique file identifier of small (160x160) chat photo, which is supposed to be the same over time and for different bots. Can't be used to download or reuse the file.
    /// * `big_file_id` - File identifier of big (640x640) chat photo. This `file_id` can be used only for photo download and only for as long as the photo is not changed.
    /// * `big_file_unique_id` - Unique file identifier of big (640x640) chat photo, which is supposed to be the same over time and for different bots. Can't be used to download or reuse the file.
    #[must_use]
    pub fn new<T0: Into<Box<str>>, T1: Into<Box<str>>, T2: Into<Box<str>>, T3: Into<Box<str>>>(
        small_file_id: T0,
        small_file_unique_id: T1,
        big_file_id: T2,
        big_file_unique_id: T3,
    ) -> Self {
        Self {
            small_file_id: small_file_id.into(),
            small_file_unique_id: small_file_unique_id.into(),
            big_file_id: big_file_id.into(),
            big_file_unique_id: big_file_unique_id.into(),
        }
    }

    /// File identifier of small (160x160) chat photo. This `file_id` can be used only for photo download and only for as long as the photo is not changed.
    #[must_use]
    pub fn small_file_id<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.small_file_id = val.into();
        self
    }

    /// Unique file identifier of small (160x160) chat photo, which is supposed to be the same over time and for different bots. Can't be used to download or reuse the file.
    #[must_use]
    pub fn small_file_unique_id<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.small_file_unique_id = val.into();
        self
    }

    /// File identifier of big (640x640) chat photo. This `file_id` can be used only for photo download and only for as long as the photo is not changed.
    #[must_use]
    pub fn big_file_id<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.big_file_id = val.into();
        self
    }

    /// Unique file identifier of big (640x640) chat photo, which is supposed to be the same over time and for different bots. Can't be used to download or reuse the file.
    #[must_use]
    pub fn big_file_unique_id<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.big_file_unique_id = val.into();
        self
    }
}
