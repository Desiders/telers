use serde::{Deserialize, Serialize};
/// This object represents a video file of a specific quality.
/// # Documentation
/// <https://core.telegram.org/bots/api#videoquality>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VideoQuality {
    /// Identifier for this file, which can be used to download or reuse the file
    pub file_id: Box<str>,
    /// Unique identifier for this file, which is supposed to be the same over time and for different bots. Can't be used to download or reuse the file.
    pub file_unique_id: Box<str>,
    /// Video width
    pub width: i64,
    /// Video height
    pub height: i64,
    /// Codec that was used to encode the video, for example, `h264`, `h265`, or `av01`
    pub codec: Box<str>,
    /// File size in bytes. It can be bigger than 2^31 and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a signed 64-bit integer or double-precision float type are safe for storing this value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<i64>,
}
impl VideoQuality {
    /// Creates a new `VideoQuality`.
    ///
    /// # Arguments
    /// * `file_id` - Identifier for this file, which can be used to download or reuse the file
    /// * `file_unique_id` - Unique identifier for this file, which is supposed to be the same over time and for different bots. Can't be used to download or reuse the file.
    /// * `width` - Video width
    /// * `height` - Video height
    /// * `codec` - Codec that was used to encode the video, for example, `h264`, `h265`, or `av01`
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<
        T0: Into<Box<str>>,
        T1: Into<Box<str>>,
        T2: Into<i64>,
        T3: Into<i64>,
        T4: Into<Box<str>>,
    >(
        file_id: T0,
        file_unique_id: T1,
        width: T2,
        height: T3,
        codec: T4,
    ) -> Self {
        Self {
            file_id: file_id.into(),
            file_unique_id: file_unique_id.into(),
            width: width.into(),
            height: height.into(),
            codec: codec.into(),
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

    /// Video width
    #[must_use]
    pub fn width<T: Into<i64>>(mut self, val: T) -> Self {
        self.width = val.into();
        self
    }

    /// Video height
    #[must_use]
    pub fn height<T: Into<i64>>(mut self, val: T) -> Self {
        self.height = val.into();
        self
    }

    /// Codec that was used to encode the video, for example, `h264`, `h265`, or `av01`
    #[must_use]
    pub fn codec<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.codec = val.into();
        self
    }

    /// File size in bytes. It can be bigger than 2^31 and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a signed 64-bit integer or double-precision float type are safe for storing this value.
    #[must_use]
    pub fn file_size<T: Into<i64>>(mut self, val: T) -> Self {
        self.file_size = Some(val.into());
        self
    }

    /// File size in bytes. It can be bigger than 2^31 and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a signed 64-bit integer or double-precision float type are safe for storing this value.
    #[must_use]
    pub fn file_size_option<T: Into<i64>>(mut self, val: Option<T>) -> Self {
        self.file_size = val.map(Into::into);
        self
    }
}
