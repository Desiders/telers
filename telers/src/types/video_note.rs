use serde::{Deserialize, Serialize};
/// This object represents a video message.
/// # Documentation
/// <https://core.telegram.org/bots/api#videonote>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VideoNote {
    /// Identifier for this file, which can be used to download or reuse the file
    pub file_id: Box<str>,
    /// Unique identifier for this file, which is supposed to be the same over time and for different bots. Can't be used to download or reuse the file.
    pub file_unique_id: Box<str>,
    /// Video width and height (diameter of the video message) as defined by the sender
    pub length: i64,
    /// Duration of the video in seconds as defined by the sender
    pub duration: i64,
    /// Video thumbnail
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<crate::types::PhotoSize>,
    /// File size in bytes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<i64>,
}
impl VideoNote {
    /// Creates a new `VideoNote`.
    ///
    /// # Arguments
    /// * `file_id` - Identifier for this file, which can be used to download or reuse the file
    /// * `file_unique_id` - Unique identifier for this file, which is supposed to be the same over time and for different bots. Can't be used to download or reuse the file.
    /// * `length` - Video width and height (diameter of the video message) as defined by the sender
    /// * `duration` - Duration of the video in seconds as defined by the sender
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0: Into<Box<str>>, T1: Into<Box<str>>, T2: Into<i64>, T3: Into<i64>>(
        file_id: T0,
        file_unique_id: T1,
        length: T2,
        duration: T3,
    ) -> Self {
        Self {
            file_id: file_id.into(),
            file_unique_id: file_unique_id.into(),
            length: length.into(),
            duration: duration.into(),
            thumbnail: None,
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

    /// Video width and height (diameter of the video message) as defined by the sender
    #[must_use]
    pub fn length<T: Into<i64>>(mut self, val: T) -> Self {
        self.length = val.into();
        self
    }

    /// Duration of the video in seconds as defined by the sender
    #[must_use]
    pub fn duration<T: Into<i64>>(mut self, val: T) -> Self {
        self.duration = val.into();
        self
    }

    /// Video thumbnail
    #[must_use]
    pub fn thumbnail<T: Into<crate::types::PhotoSize>>(mut self, val: T) -> Self {
        self.thumbnail = Some(val.into());
        self
    }

    /// Video thumbnail
    #[must_use]
    pub fn thumbnail_option<T: Into<crate::types::PhotoSize>>(mut self, val: Option<T>) -> Self {
        self.thumbnail = val.map(Into::into);
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
impl crate::types::FileIdGetter for VideoNote {
    fn file_id(&self) -> &str {
        &self.file_id
    }
}
