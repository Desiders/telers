use serde::{Deserialize, Serialize};
/// This object describes the paid media to be sent. Currently, it can be one of
/// - [`InputPaidMediaPhoto`]
/// - [`InputPaidMediaVideo`]
/// # Documentation
/// <https://core.telegram.org/bots/api#inputpaidmedia>
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum InputPaidMedia {
    Photo(crate::types::InputPaidMediaPhoto),
    Video(crate::types::InputPaidMediaVideo),
}
impl InputPaidMedia {
    /// Helper method for field `cover`.
    ///
    /// Cover for the video in the message. Pass a `file_id` to send a file that exists on the Telegram servers (recommended), pass an HTTP URL for Telegram to get a file from the Internet, or pass `attach://<file_attach_name>` to upload a new one using multipart/form-data under <`file_attach_name`> name. More information on Sending Files: <https://core.telegram.org/bots/api#sending-files>
    #[must_use]
    pub fn cover(&self) -> Option<&crate::types::InputFile> {
        match self {
            Self::Video(val) => val.cover.as_ref(),
            Self::Photo(_) => None,
        }
    }

    /// Helper method for field `duration`.
    ///
    /// Video duration in seconds
    #[must_use]
    pub fn duration(&self) -> Option<i64> {
        match self {
            Self::Video(val) => val.duration,
            Self::Photo(_) => None,
        }
    }

    /// Helper method for field `height`.
    ///
    /// Video height
    #[must_use]
    pub fn height(&self) -> Option<i64> {
        match self {
            Self::Video(val) => val.height,
            Self::Photo(_) => None,
        }
    }

    /// Helper method for field `media`.
    ///
    /// File to send. Pass a `file_id` to send a file that exists on the Telegram servers (recommended), pass an HTTP URL for Telegram to get a file from the Internet, or pass `attach://<file_attach_name>` to upload a new one using multipart/form-data under <`file_attach_name`> name. More information on Sending Files: <https://core.telegram.org/bots/api#sending-files>
    #[must_use]
    pub fn media(&self) -> &crate::types::InputFile {
        match self {
            Self::Photo(val) => &val.media,
            Self::Video(val) => &val.media,
        }
    }

    /// Helper method for field `start_timestamp`.
    ///
    /// Start timestamp for the video in the message
    #[must_use]
    pub fn start_timestamp(&self) -> Option<i64> {
        match self {
            Self::Video(val) => val.start_timestamp,
            Self::Photo(_) => None,
        }
    }

    /// Helper method for field `supports_streaming`.
    ///
    /// Pass `true` if the uploaded video is suitable for streaming
    #[must_use]
    pub fn supports_streaming(&self) -> Option<bool> {
        match self {
            Self::Video(val) => val.supports_streaming,
            Self::Photo(_) => None,
        }
    }

    /// Helper method for field `thumbnail`.
    ///
    /// Thumbnail of the file sent; can be ignored if thumbnail generation for the file is supported server-side. The thumbnail should be in JPEG format and less than 200 kB in size. A thumbnail's width and height should not exceed 320. Ignored if the file is not uploaded using multipart/form-data. Thumbnails can't be reused and can be only uploaded as a new file, so you can pass `attach://<file_attach_name>` if the thumbnail was uploaded using multipart/form-data under <`file_attach_name`>. More information on Sending Files: <https://core.telegram.org/bots/api#sending-files>
    #[must_use]
    pub fn thumbnail(&self) -> Option<&crate::types::InputFile> {
        match self {
            Self::Video(val) => val.thumbnail.as_ref(),
            Self::Photo(_) => None,
        }
    }

    /// Helper method for field `width`.
    ///
    /// Video width
    #[must_use]
    pub fn width(&self) -> Option<i64> {
        match self {
            Self::Video(val) => val.width,
            Self::Photo(_) => None,
        }
    }
}
impl From<crate::types::InputPaidMediaPhoto> for InputPaidMedia {
    fn from(val: crate::types::InputPaidMediaPhoto) -> Self {
        Self::Photo(val)
    }
}
impl TryFrom<InputPaidMedia> for crate::types::InputPaidMediaPhoto {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: InputPaidMedia) -> Result<Self, Self::Error> {
        match val {
            InputPaidMedia::Photo(inner) => Ok(inner),
            InputPaidMedia::Video(_) => Err(Self::Error::new(
                stringify!(InputPaidMedia),
                stringify!(InputPaidMediaPhoto),
            )),
        }
    }
}
impl From<crate::types::InputPaidMediaVideo> for InputPaidMedia {
    fn from(val: crate::types::InputPaidMediaVideo) -> Self {
        Self::Video(val)
    }
}
impl TryFrom<InputPaidMedia> for crate::types::InputPaidMediaVideo {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: InputPaidMedia) -> Result<Self, Self::Error> {
        match val {
            InputPaidMedia::Video(inner) => Ok(inner),
            InputPaidMedia::Photo(_) => Err(Self::Error::new(
                stringify!(InputPaidMedia),
                stringify!(InputPaidMediaVideo),
            )),
        }
    }
}
