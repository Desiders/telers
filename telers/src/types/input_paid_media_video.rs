use serde::{Deserialize, Serialize};
/// The paid media to send is a video.
/// # Documentation
/// <https://core.telegram.org/bots/api#inputpaidmediavideo>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InputPaidMediaVideo {
    /// File to send. Pass a `file_id` to send a file that exists on the Telegram servers (recommended), pass an HTTP URL for Telegram to get a file from the Internet, or pass `attach://<file_attach_name>` to upload a new one using multipart/form-data under <`file_attach_name`> name. More information on Sending Files: <https://core.telegram.org/bots/api#sending-files>
    pub media: crate::types::InputFile,
    /// Thumbnail of the file sent; can be ignored if thumbnail generation for the file is supported server-side. The thumbnail should be in JPEG format and less than 200 kB in size. A thumbnail's width and height should not exceed 320. Ignored if the file is not uploaded using multipart/form-data. Thumbnails can't be reused and can be only uploaded as a new file, so you can pass `attach://<file_attach_name>` if the thumbnail was uploaded using multipart/form-data under <`file_attach_name`>. More information on Sending Files: <https://core.telegram.org/bots/api#sending-files>
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<crate::types::InputFile>,
    /// Cover for the video in the message. Pass a `file_id` to send a file that exists on the Telegram servers (recommended), pass an HTTP URL for Telegram to get a file from the Internet, or pass `attach://<file_attach_name>` to upload a new one using multipart/form-data under <`file_attach_name`> name. More information on Sending Files: <https://core.telegram.org/bots/api#sending-files>
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cover: Option<crate::types::InputFile>,
    /// Start timestamp for the video in the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_timestamp: Option<i64>,
    /// Video width
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i64>,
    /// Video height
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i64>,
    /// Video duration in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
    /// Pass `true` if the uploaded video is suitable for streaming
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_streaming: Option<bool>,
}
impl InputPaidMediaVideo {
    /// Creates a new `InputPaidMediaVideo`.
    ///
    /// # Arguments
    /// * `media` - File to send. Pass a `file_id` to send a file that exists on the Telegram servers (recommended), pass an HTTP URL for Telegram to get a file from the Internet, or pass `attach://<file_attach_name>` to upload a new one using multipart/form-data under <`file_attach_name`> name. More information on Sending Files: <https://core.telegram.org/bots/api#sending-files>
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0: Into<crate::types::InputFile>>(media: T0) -> Self {
        Self {
            media: media.into(),
            thumbnail: None,
            cover: None,
            start_timestamp: None,
            width: None,
            height: None,
            duration: None,
            supports_streaming: None,
        }
    }

    /// File to send. Pass a `file_id` to send a file that exists on the Telegram servers (recommended), pass an HTTP URL for Telegram to get a file from the Internet, or pass `attach://<file_attach_name>` to upload a new one using multipart/form-data under <`file_attach_name`> name. More information on Sending Files: <https://core.telegram.org/bots/api#sending-files>
    #[must_use]
    pub fn media<T: Into<crate::types::InputFile>>(mut self, val: T) -> Self {
        self.media = val.into();
        self
    }

    /// Thumbnail of the file sent; can be ignored if thumbnail generation for the file is supported server-side. The thumbnail should be in JPEG format and less than 200 kB in size. A thumbnail's width and height should not exceed 320. Ignored if the file is not uploaded using multipart/form-data. Thumbnails can't be reused and can be only uploaded as a new file, so you can pass `attach://<file_attach_name>` if the thumbnail was uploaded using multipart/form-data under <`file_attach_name`>. More information on Sending Files: <https://core.telegram.org/bots/api#sending-files>
    #[must_use]
    pub fn thumbnail<T: Into<crate::types::InputFile>>(mut self, val: T) -> Self {
        self.thumbnail = Some(val.into());
        self
    }

    /// Thumbnail of the file sent; can be ignored if thumbnail generation for the file is supported server-side. The thumbnail should be in JPEG format and less than 200 kB in size. A thumbnail's width and height should not exceed 320. Ignored if the file is not uploaded using multipart/form-data. Thumbnails can't be reused and can be only uploaded as a new file, so you can pass `attach://<file_attach_name>` if the thumbnail was uploaded using multipart/form-data under <`file_attach_name`>. More information on Sending Files: <https://core.telegram.org/bots/api#sending-files>
    #[must_use]
    pub fn thumbnail_option<T: Into<crate::types::InputFile>>(mut self, val: Option<T>) -> Self {
        self.thumbnail = val.map(Into::into);
        self
    }

    /// Cover for the video in the message. Pass a `file_id` to send a file that exists on the Telegram servers (recommended), pass an HTTP URL for Telegram to get a file from the Internet, or pass `attach://<file_attach_name>` to upload a new one using multipart/form-data under <`file_attach_name`> name. More information on Sending Files: <https://core.telegram.org/bots/api#sending-files>
    #[must_use]
    pub fn cover<T: Into<crate::types::InputFile>>(mut self, val: T) -> Self {
        self.cover = Some(val.into());
        self
    }

    /// Cover for the video in the message. Pass a `file_id` to send a file that exists on the Telegram servers (recommended), pass an HTTP URL for Telegram to get a file from the Internet, or pass `attach://<file_attach_name>` to upload a new one using multipart/form-data under <`file_attach_name`> name. More information on Sending Files: <https://core.telegram.org/bots/api#sending-files>
    #[must_use]
    pub fn cover_option<T: Into<crate::types::InputFile>>(mut self, val: Option<T>) -> Self {
        self.cover = val.map(Into::into);
        self
    }

    /// Start timestamp for the video in the message
    #[must_use]
    pub fn start_timestamp<T: Into<i64>>(mut self, val: T) -> Self {
        self.start_timestamp = Some(val.into());
        self
    }

    /// Start timestamp for the video in the message
    #[must_use]
    pub fn start_timestamp_option<T: Into<i64>>(mut self, val: Option<T>) -> Self {
        self.start_timestamp = val.map(Into::into);
        self
    }

    /// Video width
    #[must_use]
    pub fn width<T: Into<i64>>(mut self, val: T) -> Self {
        self.width = Some(val.into());
        self
    }

    /// Video width
    #[must_use]
    pub fn width_option<T: Into<i64>>(mut self, val: Option<T>) -> Self {
        self.width = val.map(Into::into);
        self
    }

    /// Video height
    #[must_use]
    pub fn height<T: Into<i64>>(mut self, val: T) -> Self {
        self.height = Some(val.into());
        self
    }

    /// Video height
    #[must_use]
    pub fn height_option<T: Into<i64>>(mut self, val: Option<T>) -> Self {
        self.height = val.map(Into::into);
        self
    }

    /// Video duration in seconds
    #[must_use]
    pub fn duration<T: Into<i64>>(mut self, val: T) -> Self {
        self.duration = Some(val.into());
        self
    }

    /// Video duration in seconds
    #[must_use]
    pub fn duration_option<T: Into<i64>>(mut self, val: Option<T>) -> Self {
        self.duration = val.map(Into::into);
        self
    }

    /// Pass `true` if the uploaded video is suitable for streaming
    #[must_use]
    pub fn supports_streaming<T: Into<bool>>(mut self, val: T) -> Self {
        self.supports_streaming = Some(val.into());
        self
    }

    /// Pass `true` if the uploaded video is suitable for streaming
    #[must_use]
    pub fn supports_streaming_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.supports_streaming = val.map(Into::into);
        self
    }
}
