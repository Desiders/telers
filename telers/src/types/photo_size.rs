use serde::{Deserialize, Serialize};
/// This object represents one size of a photo or a file / sticker thumbnail.
/// # Documentation
/// <https://core.telegram.org/bots/api#photosize>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PhotoSize {
    /// Identifier for this file, which can be used to download or reuse the file
    pub file_id: Box<str>,
    /// Unique identifier for this file, which is supposed to be the same over time and for different bots. Can't be used to download or reuse the file.
    pub file_unique_id: Box<str>,
    /// Photo width
    pub width: i64,
    /// Photo height
    pub height: i64,
    /// File size in bytes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<i64>,
}
impl PhotoSize {
    /// Creates a new `PhotoSize`.
    ///
    /// # Arguments
    /// * `file_id` - Identifier for this file, which can be used to download or reuse the file
    /// * `file_unique_id` - Unique identifier for this file, which is supposed to be the same over time and for different bots. Can't be used to download or reuse the file.
    /// * `width` - Photo width
    /// * `height` - Photo height
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0: Into<Box<str>>, T1: Into<Box<str>>, T2: Into<i64>, T3: Into<i64>>(
        file_id: T0,
        file_unique_id: T1,
        width: T2,
        height: T3,
    ) -> Self {
        Self {
            file_id: file_id.into(),
            file_unique_id: file_unique_id.into(),
            width: width.into(),
            height: height.into(),
            file_size: None,
        }
    }

    /// Identifier for this file, which can be used to download or reuse the file
    #[must_use]
    pub fn file_id<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.file_id = val.into();
        self
    }

    /// Unique identifier for this file, which is supposed to be the same over time and for different bots. Can't be used to download or reuse the file.
    #[must_use]
    pub fn file_unique_id<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.file_unique_id = val.into();
        self
    }

    /// Photo width
    #[must_use]
    pub fn width<T: Into<i64>>(mut self, val: T) -> Self {
        self.width = val.into();
        self
    }

    /// Photo height
    #[must_use]
    pub fn height<T: Into<i64>>(mut self, val: T) -> Self {
        self.height = val.into();
        self
    }

    /// File size in bytes
    #[must_use]
    pub fn file_size<T: Into<i64>>(mut self, val: T) -> Self {
        self.file_size = Some(val.into());
        self
    }

    /// File size in bytes
    #[must_use]
    pub fn file_size_option<T: Into<i64>>(mut self, val: Option<T>) -> Self {
        self.file_size = val.map(Into::into);
        self
    }
}
impl crate::types::FileIdGetter for PhotoSize {
    fn file_id(&self) -> &str {
        &self.file_id
    }
}
