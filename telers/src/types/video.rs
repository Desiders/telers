use serde::{Deserialize, Serialize};
/// This object represents a video file.
/// # Documentation
/// <https://core.telegram.org/bots/api#video>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Video {
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
    /// Video thumbnail
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<crate::types::PhotoSize>,
    /// Available sizes of the cover of the video in the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cover: Option<Box<[crate::types::PhotoSize]>>,
    /// Timestamp in seconds from which the video will play in the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_timestamp: Option<i64>,
    /// List of available qualities of the video
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qualities: Option<Box<[crate::types::VideoQuality]>>,
    /// Original filename as defined by the sender
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_name: Option<Box<str>>,
    /// MIME type of the file as defined by the sender
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<Box<str>>,
    /// File size in bytes. It can be bigger than 2^31 and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a signed 64-bit integer or double-precision float type are safe for storing this value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<i64>,
}
impl Video {
    /// Creates a new `Video`.
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
            cover: None,
            start_timestamp: None,
            qualities: None,
            file_name: None,
            mime_type: None,
            file_size: None,
        }
    }

    /// Identifier for this file, which can be used to download or reuse the file
    #[must_use]
    pub fn file_id<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.file_id = val.into();
        this
    }

    /// Unique identifier for this file, which is supposed to be the same over time and for different bots. Can't be used to download or reuse the file.
    #[must_use]
    pub fn file_unique_id<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.file_unique_id = val.into();
        this
    }

    /// Video width as defined by the sender
    #[must_use]
    pub fn width<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.width = val.into();
        this
    }

    /// Video height as defined by the sender
    #[must_use]
    pub fn height<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.height = val.into();
        this
    }

    /// Duration of the video in seconds as defined by the sender
    #[must_use]
    pub fn duration<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.duration = val.into();
        this
    }

    /// Video thumbnail
    #[must_use]
    pub fn thumbnail<T: Into<crate::types::PhotoSize>>(self, val: T) -> Self {
        let mut this = self;
        this.thumbnail = Some(val.into());
        this
    }

    /// Video thumbnail
    #[must_use]
    pub fn thumbnail_option<T: Into<crate::types::PhotoSize>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.thumbnail = val.map(Into::into);
        this
    }

    /// Available sizes of the cover of the video in the message
    ///
    /// # Notes
    /// Adds multiple elements.
    #[must_use]
    pub fn covers<T: Into<Box<[crate::types::PhotoSize]>>>(self, val: T) -> Self {
        let mut this = self;
        this.cover = Some(
            this.cover
                .unwrap_or_default()
                .into_vec()
                .into_iter()
                .chain(val.into())
                .collect(),
        );
        this
    }

    /// Available sizes of the cover of the video in the message
    ///
    /// # Notes
    /// Adds a single element.
    #[must_use]
    pub fn cover<T: Into<crate::types::PhotoSize>>(self, val: T) -> Self {
        let mut this = self;
        this.cover = Some(
            this.cover
                .unwrap_or_default()
                .into_vec()
                .into_iter()
                .chain(Some(val.into()))
                .collect(),
        );
        this
    }

    /// Available sizes of the cover of the video in the message
    ///
    /// # Notes
    /// Adds a single element.
    #[must_use]
    pub fn cover_option<T: Into<Box<[crate::types::PhotoSize]>>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.cover = val.map(Into::into);
        this
    }

    /// Timestamp in seconds from which the video will play in the message
    #[must_use]
    pub fn start_timestamp<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.start_timestamp = Some(val.into());
        this
    }

    /// Timestamp in seconds from which the video will play in the message
    #[must_use]
    pub fn start_timestamp_option<T: Into<i64>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.start_timestamp = val.map(Into::into);
        this
    }

    /// List of available qualities of the video
    ///
    /// # Notes
    /// Adds multiple elements.
    #[must_use]
    pub fn qualities<T: Into<Box<[crate::types::VideoQuality]>>>(self, val: T) -> Self {
        let mut this = self;
        this.qualities = Some(
            this.qualities
                .unwrap_or_default()
                .into_vec()
                .into_iter()
                .chain(val.into())
                .collect(),
        );
        this
    }

    /// List of available qualities of the video
    ///
    /// # Notes
    /// Adds a single element.
    #[must_use]
    pub fn quality<T: Into<crate::types::VideoQuality>>(self, val: T) -> Self {
        let mut this = self;
        this.qualities = Some(
            this.qualities
                .unwrap_or_default()
                .into_vec()
                .into_iter()
                .chain(Some(val.into()))
                .collect(),
        );
        this
    }

    /// List of available qualities of the video
    ///
    /// # Notes
    /// Adds a single element.
    #[must_use]
    pub fn qualities_option<T: Into<Box<[crate::types::VideoQuality]>>>(
        self,
        val: Option<T>,
    ) -> Self {
        let mut this = self;
        this.qualities = val.map(Into::into);
        this
    }

    /// Original filename as defined by the sender
    #[must_use]
    pub fn file_name<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.file_name = Some(val.into());
        this
    }

    /// Original filename as defined by the sender
    #[must_use]
    pub fn file_name_option<T: Into<Box<str>>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.file_name = val.map(Into::into);
        this
    }

    /// MIME type of the file as defined by the sender
    #[must_use]
    pub fn mime_type<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.mime_type = Some(val.into());
        this
    }

    /// MIME type of the file as defined by the sender
    #[must_use]
    pub fn mime_type_option<T: Into<Box<str>>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.mime_type = val.map(Into::into);
        this
    }

    /// File size in bytes. It can be bigger than 2^31 and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a signed 64-bit integer or double-precision float type are safe for storing this value.
    #[must_use]
    pub fn file_size<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.file_size = Some(val.into());
        this
    }

    /// File size in bytes. It can be bigger than 2^31 and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a signed 64-bit integer or double-precision float type are safe for storing this value.
    #[must_use]
    pub fn file_size_option<T: Into<i64>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.file_size = val.map(Into::into);
        this
    }
}
