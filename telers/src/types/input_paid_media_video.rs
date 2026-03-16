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
    pub fn media<T: Into<crate::types::InputFile>>(self, val: T) -> Self {
        let mut this = self;
        this.media = val.into();
        this
    }

    /// Thumbnail of the file sent; can be ignored if thumbnail generation for the file is supported server-side. The thumbnail should be in JPEG format and less than 200 kB in size. A thumbnail's width and height should not exceed 320. Ignored if the file is not uploaded using multipart/form-data. Thumbnails can't be reused and can be only uploaded as a new file, so you can pass `attach://<file_attach_name>` if the thumbnail was uploaded using multipart/form-data under <`file_attach_name`>. More information on Sending Files: <https://core.telegram.org/bots/api#sending-files>
    #[must_use]
    pub fn thumbnail<T: Into<crate::types::InputFile>>(self, val: T) -> Self {
        let mut this = self;
        this.thumbnail = Some(val.into());
        this
    }

    /// Thumbnail of the file sent; can be ignored if thumbnail generation for the file is supported server-side. The thumbnail should be in JPEG format and less than 200 kB in size. A thumbnail's width and height should not exceed 320. Ignored if the file is not uploaded using multipart/form-data. Thumbnails can't be reused and can be only uploaded as a new file, so you can pass `attach://<file_attach_name>` if the thumbnail was uploaded using multipart/form-data under <`file_attach_name`>. More information on Sending Files: <https://core.telegram.org/bots/api#sending-files>
    #[must_use]
    pub fn thumbnail_option<T: Into<crate::types::InputFile>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.thumbnail = val.map(Into::into);
        this
    }

    /// Cover for the video in the message. Pass a `file_id` to send a file that exists on the Telegram servers (recommended), pass an HTTP URL for Telegram to get a file from the Internet, or pass `attach://<file_attach_name>` to upload a new one using multipart/form-data under <`file_attach_name`> name. More information on Sending Files: <https://core.telegram.org/bots/api#sending-files>
    #[must_use]
    pub fn cover<T: Into<crate::types::InputFile>>(self, val: T) -> Self {
        let mut this = self;
        this.cover = Some(val.into());
        this
    }

    /// Cover for the video in the message. Pass a `file_id` to send a file that exists on the Telegram servers (recommended), pass an HTTP URL for Telegram to get a file from the Internet, or pass `attach://<file_attach_name>` to upload a new one using multipart/form-data under <`file_attach_name`> name. More information on Sending Files: <https://core.telegram.org/bots/api#sending-files>
    #[must_use]
    pub fn cover_option<T: Into<crate::types::InputFile>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.cover = val.map(Into::into);
        this
    }

    /// Start timestamp for the video in the message
    #[must_use]
    pub fn start_timestamp<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.start_timestamp = Some(val.into());
        this
    }

    /// Start timestamp for the video in the message
    #[must_use]
    pub fn start_timestamp_option<T: Into<i64>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.start_timestamp = val.map(Into::into);
        this
    }

    /// Video width
    #[must_use]
    pub fn width<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.width = Some(val.into());
        this
    }

    /// Video width
    #[must_use]
    pub fn width_option<T: Into<i64>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.width = val.map(Into::into);
        this
    }

    /// Video height
    #[must_use]
    pub fn height<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.height = Some(val.into());
        this
    }

    /// Video height
    #[must_use]
    pub fn height_option<T: Into<i64>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.height = val.map(Into::into);
        this
    }

    /// Video duration in seconds
    #[must_use]
    pub fn duration<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.duration = Some(val.into());
        this
    }

    /// Video duration in seconds
    #[must_use]
    pub fn duration_option<T: Into<i64>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.duration = val.map(Into::into);
        this
    }

    /// Pass `true` if the uploaded video is suitable for streaming
    #[must_use]
    pub fn supports_streaming<T: Into<bool>>(self, val: T) -> Self {
        let mut this = self;
        this.supports_streaming = Some(val.into());
        this
    }

    /// Pass `true` if the uploaded video is suitable for streaming
    #[must_use]
    pub fn supports_streaming_option<T: Into<bool>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.supports_streaming = val.map(Into::into);
        this
    }
}
