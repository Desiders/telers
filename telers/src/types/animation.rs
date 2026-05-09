use serde::{Deserialize, Serialize};
/// This object represents an animation file (GIF or H.264/MPEG-4 AVC video without sound).
/// # Documentation
/// <https://core.telegram.org/bots/api#animation>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Animation {
    /// Identifier for this file, which can be used to download or reuse the file
    pub file_id: Box<str>,
    /// Unique identifier for this file, which is supposed to be the same over time and for different bots. Can't be used to download or reuse the file.
    pub file_unique_id: Box<str>,
    /// Video width as defined by the sender
    pub width: i64,
    /// Video height as defined by the sender
    pub height: i64,
    /// Duration of the video in seconds as defined by the sender
    pub duration: i64,
    /// Animation thumbnail as defined by the sender
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<crate::types::PhotoSize>,
    /// Original animation filename as defined by the sender
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_name: Option<Box<str>>,
    /// MIME type of the file as defined by the sender
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<Box<str>>,
    /// File size in bytes. It can be bigger than 2^31 and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a signed 64-bit integer or double-precision float type are safe for storing this value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<i64>,
}
impl Animation {
    /// Creates a new `Animation`.
    ///
    /// # Arguments
    /// * `file_id` - Identifier for this file, which can be used to download or reuse the file
    /// * `file_unique_id` - Unique identifier for this file, which is supposed to be the same over time and for different bots. Can't be used to download or reuse the file.
    /// * `width` - Video width as defined by the sender
    /// * `height` - Video height as defined by the sender
    /// * `duration` - Duration of the video in seconds as defined by the sender
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<
        T0: Into<Box<str>>,
        T1: Into<Box<str>>,
        T2: Into<i64>,
        T3: Into<i64>,
        T4: Into<i64>,
    >(
        file_id: T0,
        file_unique_id: T1,
        width: T2,
        height: T3,
        duration: T4,
    ) -> Self {
        Self {
            file_id: file_id.into(),
            file_unique_id: file_unique_id.into(),
            width: width.into(),
            height: height.into(),
            duration: duration.into(),
            thumbnail: None,
            file_name: None,
            mime_type: None,
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

    /// Video width as defined by the sender
    #[must_use]
    pub fn width<T: Into<i64>>(mut self, val: T) -> Self {
        self.width = val.into();
        self
    }

    /// Video height as defined by the sender
    #[must_use]
    pub fn height<T: Into<i64>>(mut self, val: T) -> Self {
        self.height = val.into();
        self
    }

    /// Duration of the video in seconds as defined by the sender
    #[must_use]
    pub fn duration<T: Into<i64>>(mut self, val: T) -> Self {
        self.duration = val.into();
        self
    }

    /// Animation thumbnail as defined by the sender
    #[must_use]
    pub fn thumbnail<T: Into<crate::types::PhotoSize>>(mut self, val: T) -> Self {
        self.thumbnail = Some(val.into());
        self
    }

    /// Animation thumbnail as defined by the sender
    #[must_use]
    pub fn thumbnail_option<T: Into<crate::types::PhotoSize>>(mut self, val: Option<T>) -> Self {
        self.thumbnail = val.map(Into::into);
        self
    }

    /// Original animation filename as defined by the sender
    #[must_use]
    pub fn file_name<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.file_name = Some(val.into());
        self
    }

    /// Original animation filename as defined by the sender
    #[must_use]
    pub fn file_name_option<T: Into<Box<str>>>(mut self, val: Option<T>) -> Self {
        self.file_name = val.map(Into::into);
        self
    }

    /// MIME type of the file as defined by the sender
    #[must_use]
    pub fn mime_type<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.mime_type = Some(val.into());
        self
    }

    /// MIME type of the file as defined by the sender
    #[must_use]
    pub fn mime_type_option<T: Into<Box<str>>>(mut self, val: Option<T>) -> Self {
        self.mime_type = val.map(Into::into);
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
