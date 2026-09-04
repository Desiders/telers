use serde::{Deserialize, Serialize};
/// This object represents an audio file to be treated as music by the Telegram clients.
/// # Documentation
/// <https://core.telegram.org/bots/api#audio>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Audio {
    /// Identifier for this file, which can be used to download or reuse the file
    pub file_id: Box<str>,
    /// Unique identifier for this file, which is supposed to be the same over time and for different bots. Can't be used to download or reuse the file.
    pub file_unique_id: Box<str>,
    /// Duration of the audio in seconds as defined by the sender
    pub duration: i64,
    /// Performer of the audio as defined by the sender or by audio tags
    #[serde(skip_serializing_if = "Option::is_none")]
    pub performer: Option<Box<str>>,
    /// Title of the audio as defined by the sender or by audio tags
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<Box<str>>,
    /// Original filename as defined by the sender
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_name: Option<Box<str>>,
    /// MIME type of the file as defined by the sender
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<Box<str>>,
    /// File size in bytes. It can be bigger than 2^31 and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a signed 64-bit integer or double-precision float type are safe for storing this value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<i64>,
    /// Thumbnail of the album cover to which the music file belongs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<crate::types::PhotoSize>,
}
impl Audio {
    /// Creates a new `Audio`.
    ///
    /// # Arguments
    /// * `file_id` - Identifier for this file, which can be used to download or reuse the file
    /// * `file_unique_id` - Unique identifier for this file, which is supposed to be the same over time and for different bots. Can't be used to download or reuse the file.
    /// * `duration` - Duration of the audio in seconds as defined by the sender
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0: Into<Box<str>>, T1: Into<Box<str>>, T2: Into<i64>>(
        file_id: T0,
        file_unique_id: T1,
        duration: T2,
    ) -> Self {
        Self {
            file_id: file_id.into(),
            file_unique_id: file_unique_id.into(),
            duration: duration.into(),
            performer: None,
            title: None,
            file_name: None,
            mime_type: None,
            file_size: None,
            thumbnail: None,
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

    /// Duration of the audio in seconds as defined by the sender
    #[must_use]
    pub fn duration<T: Into<i64>>(mut self, val: T) -> Self {
        self.duration = val.into();
        self
    }

    /// Performer of the audio as defined by the sender or by audio tags
    #[must_use]
    pub fn performer<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.performer = Some(val.into());
        self
    }

    /// Performer of the audio as defined by the sender or by audio tags
    #[must_use]
    pub fn performer_option<T: Into<Box<str>>>(mut self, val: Option<T>) -> Self {
        self.performer = val.map(Into::into);
        self
    }

    /// Title of the audio as defined by the sender or by audio tags
    #[must_use]
    pub fn title<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.title = Some(val.into());
        self
    }

    /// Title of the audio as defined by the sender or by audio tags
    #[must_use]
    pub fn title_option<T: Into<Box<str>>>(mut self, val: Option<T>) -> Self {
        self.title = val.map(Into::into);
        self
    }

    /// Original filename as defined by the sender
    #[must_use]
    pub fn file_name<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.file_name = Some(val.into());
        self
    }

    /// Original filename as defined by the sender
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

    /// Thumbnail of the album cover to which the music file belongs
    #[must_use]
    pub fn thumbnail<T: Into<crate::types::PhotoSize>>(mut self, val: T) -> Self {
        self.thumbnail = Some(val.into());
        self
    }

    /// Thumbnail of the album cover to which the music file belongs
    #[must_use]
    pub fn thumbnail_option<T: Into<crate::types::PhotoSize>>(mut self, val: Option<T>) -> Self {
        self.thumbnail = val.map(Into::into);
        self
    }
}
impl crate::types::FileIdGetter for Audio {
    fn file_id(&self) -> &str {
        &self.file_id
    }
}
