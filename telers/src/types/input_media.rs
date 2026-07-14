use serde::{Deserialize, Serialize};
/// This object represents the content of a media message to be sent. It should be one of
/// - [`crate::types::InputMediaAnimation`]
/// - [`crate::types::InputMediaAudio`]
/// - [`crate::types::InputMediaDocument`]
/// - [`crate::types::InputMediaLivePhoto`]
/// - [`crate::types::InputMediaPhoto`]
/// - [`crate::types::InputMediaVideo`]
/// # Documentation
/// <https://core.telegram.org/bots/api#inputmedia>
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum InputMedia {
    Animation(crate::types::InputMediaAnimation),
    Audio(crate::types::InputMediaAudio),
    Document(crate::types::InputMediaDocument),
    LivePhoto(crate::types::InputMediaLivePhoto),
    Photo(crate::types::InputMediaPhoto),
    Video(crate::types::InputMediaVideo),
}
impl InputMedia {
    /// Helper method for field `caption`.
    ///
    /// # Variants
    /// - `InputMediaAnimation`. Caption of the animation to be sent, 0-1024 characters after entities parsing
    /// - `InputMediaAudio`. Caption of the audio to be sent, 0-1024 characters after entities parsing
    /// - `InputMediaDocument`. Caption of the document to be sent, 0-1024 characters after entities parsing
    /// - `InputMediaLivePhoto`. Caption of the live photo to be sent, 0-1024 characters after entities parsing
    /// - `InputMediaPhoto`. Caption of the photo to be sent, 0-1024 characters after entities parsing
    /// - `InputMediaVideo`. Caption of the video to be sent, 0-1024 characters after entities parsing
    #[must_use]
    pub fn caption(&self) -> Option<&str> {
        match self {
            Self::Animation(val) => val.caption.as_deref(),
            Self::Audio(val) => val.caption.as_deref(),
            Self::Document(val) => val.caption.as_deref(),
            Self::LivePhoto(val) => val.caption.as_deref(),
            Self::Photo(val) => val.caption.as_deref(),
            Self::Video(val) => val.caption.as_deref(),
        }
    }

    /// Helper method for field `caption_entities`.
    ///
    /// List of special entities that appear in the caption, which can be specified instead of `parse_mode`
    #[must_use]
    pub fn caption_entities(&self) -> Option<&[crate::types::MessageEntity]> {
        match self {
            Self::Animation(val) => val.caption_entities.as_deref(),
            Self::Audio(val) => val.caption_entities.as_deref(),
            Self::Document(val) => val.caption_entities.as_deref(),
            Self::LivePhoto(val) => val.caption_entities.as_deref(),
            Self::Photo(val) => val.caption_entities.as_deref(),
            Self::Video(val) => val.caption_entities.as_deref(),
        }
    }

    /// Helper method for field `cover`.
    ///
    /// Cover for the video in the message. Pass a `file_id` to send a file that exists on the Telegram servers (recommended), pass an HTTP URL for Telegram to get a file from the Internet, or pass `attach://<file_attach_name>` to upload a new one using multipart/form-data under <`file_attach_name`> name. More information on Sending Files: <https://core.telegram.org/bots/api#sending-files>
    #[must_use]
    pub fn cover(&self) -> Option<&crate::types::InputFile> {
        match self {
            Self::Video(val) => val.cover.as_ref(),
            _ => None,
        }
    }

    /// Helper method for field `disable_content_type_detection`.
    ///
    /// Disables automatic server-side content type detection for files uploaded using multipart/form-data. Always `true`, if the document is sent as part of an album.
    #[must_use]
    pub fn disable_content_type_detection(&self) -> Option<bool> {
        match self {
            Self::Document(val) => val.disable_content_type_detection,
            _ => None,
        }
    }

    /// Helper method for field `duration`.
    ///
    /// # Variants
    /// - `InputMediaAnimation`. Animation duration in seconds
    /// - `InputMediaAudio`. Duration of the audio in seconds
    /// - `InputMediaVideo`. Video duration in seconds
    #[must_use]
    pub fn duration(&self) -> Option<i64> {
        match self {
            Self::Animation(val) => val.duration,
            Self::Audio(val) => val.duration,
            Self::Video(val) => val.duration,
            _ => None,
        }
    }

    /// Helper method for field `has_spoiler`.
    ///
    /// # Variants
    /// - `InputMediaAnimation`. Pass `true` if the animation needs to be covered with a spoiler animation
    /// - `InputMediaLivePhoto`. Pass `true` if the live photo needs to be covered with a spoiler animation
    /// - `InputMediaPhoto`. Pass `true` if the photo needs to be covered with a spoiler animation
    /// - `InputMediaVideo`. Pass `true` if the video needs to be covered with a spoiler animation
    #[must_use]
    pub fn has_spoiler(&self) -> Option<bool> {
        match self {
            Self::Animation(val) => val.has_spoiler,
            Self::LivePhoto(val) => val.has_spoiler,
            Self::Photo(val) => val.has_spoiler,
            Self::Video(val) => val.has_spoiler,
            _ => None,
        }
    }

    /// Helper method for field `height`.
    ///
    /// # Variants
    /// - `InputMediaAnimation`. Animation height
    /// - `InputMediaVideo`. Video height
    #[must_use]
    pub fn height(&self) -> Option<i64> {
        match self {
            Self::Animation(val) => val.height,
            Self::Video(val) => val.height,
            _ => None,
        }
    }

    /// Helper method for field `media`.
    ///
    /// # Variants
    /// - `InputMediaAnimation`, `InputMediaAudio`, `InputMediaDocument`, `InputMediaPhoto`, `InputMediaVideo`. File to send. Pass a `file_id` to send a file that exists on the Telegram servers (recommended), pass an HTTP URL for Telegram to get a file from the Internet, or pass `attach://<file_attach_name>` to upload a new one using multipart/form-data under <`file_attach_name`> name. More information on Sending Files: <https://core.telegram.org/bots/api#sending-files>
    /// - `InputMediaLivePhoto`. Video of the live photo to send. Pass a `file_id` to send a file that exists on the Telegram servers (recommended) or pass `attach://<file_attach_name>` to upload a new one using multipart/form-data under <`file_attach_name`> name. More information on Sending Files: <https://core.telegram.org/bots/api#sending-files>. Sending live photos by a URL is currently unsupported.
    #[must_use]
    pub fn media(&self) -> &crate::types::InputFile {
        match self {
            Self::Animation(val) => &val.media,
            Self::Audio(val) => &val.media,
            Self::Document(val) => &val.media,
            Self::LivePhoto(val) => &val.media,
            Self::Photo(val) => &val.media,
            Self::Video(val) => &val.media,
        }
    }

    /// Helper method for field `parse_mode`.
    ///
    /// # Variants
    /// - `InputMediaAnimation`. Mode for parsing entities in the animation caption. See formatting options for more details.
    /// - `InputMediaAudio`. Mode for parsing entities in the audio caption. See formatting options for more details.
    /// - `InputMediaDocument`. Mode for parsing entities in the document caption. See formatting options for more details.
    /// - `InputMediaLivePhoto`. Mode for parsing entities in the live photo caption. See formatting options for more details.
    /// - `InputMediaPhoto`. Mode for parsing entities in the photo caption. See formatting options for more details.
    /// - `InputMediaVideo`. Mode for parsing entities in the video caption. See formatting options for more details.
    #[must_use]
    pub fn parse_mode(&self) -> Option<&str> {
        match self {
            Self::Animation(val) => val.parse_mode.as_deref(),
            Self::Audio(val) => val.parse_mode.as_deref(),
            Self::Document(val) => val.parse_mode.as_deref(),
            Self::LivePhoto(val) => val.parse_mode.as_deref(),
            Self::Photo(val) => val.parse_mode.as_deref(),
            Self::Video(val) => val.parse_mode.as_deref(),
        }
    }

    /// Helper method for field `performer`.
    ///
    /// Performer of the audio
    #[must_use]
    pub fn performer(&self) -> Option<&str> {
        match self {
            Self::Audio(val) => val.performer.as_deref(),
            _ => None,
        }
    }

    /// Helper method for field `photo`.
    ///
    /// The static photo to send. Pass a `file_id` to send a file that exists on the Telegram servers (recommended) or pass `attach://<file_attach_name>` to upload a new one using multipart/form-data under <`file_attach_name`> name. More information on Sending Files: <https://core.telegram.org/bots/api#sending-files>. Sending live photos by a URL is currently unsupported.
    #[must_use]
    pub fn photo(&self) -> Option<&crate::types::InputFile> {
        match self {
            Self::LivePhoto(val) => Some(&val.photo),
            _ => None,
        }
    }

    /// Helper method for field `show_caption_above_media`.
    ///
    /// Pass `true` if the caption must be shown above the message media
    #[must_use]
    pub fn show_caption_above_media(&self) -> Option<bool> {
        match self {
            Self::Animation(val) => val.show_caption_above_media,
            Self::LivePhoto(val) => val.show_caption_above_media,
            Self::Photo(val) => val.show_caption_above_media,
            Self::Video(val) => val.show_caption_above_media,
            _ => None,
        }
    }

    /// Helper method for field `start_timestamp`.
    ///
    /// Start timestamp for the video in the message
    #[must_use]
    pub fn start_timestamp(&self) -> Option<i64> {
        match self {
            Self::Video(val) => val.start_timestamp,
            _ => None,
        }
    }

    /// Helper method for field `supports_streaming`.
    ///
    /// Pass `true` if the uploaded video is suitable for streaming
    #[must_use]
    pub fn supports_streaming(&self) -> Option<bool> {
        match self {
            Self::Video(val) => val.supports_streaming,
            _ => None,
        }
    }

    /// Helper method for field `thumbnail`.
    ///
    /// Thumbnail of the file sent; can be ignored if thumbnail generation for the file is supported server-side. The thumbnail should be in JPEG format and less than 200 kB in size. A thumbnail's width and height should not exceed 320. Ignored if the file is not uploaded using multipart/form-data. Thumbnails can't be reused and can be only uploaded as a new file, so you can pass `attach://<file_attach_name>` if the thumbnail was uploaded using multipart/form-data under <`file_attach_name`>. More information on Sending Files: <https://core.telegram.org/bots/api#sending-files>
    #[must_use]
    pub fn thumbnail(&self) -> Option<&crate::types::InputFile> {
        match self {
            Self::Animation(val) => val.thumbnail.as_ref(),
            Self::Audio(val) => val.thumbnail.as_ref(),
            Self::Document(val) => val.thumbnail.as_ref(),
            Self::Video(val) => val.thumbnail.as_ref(),
            _ => None,
        }
    }

    /// Helper method for field `title`.
    ///
    /// Title of the audio
    #[must_use]
    pub fn title(&self) -> Option<&str> {
        match self {
            Self::Audio(val) => val.title.as_deref(),
            _ => None,
        }
    }

    /// Helper method for field `width`.
    ///
    /// # Variants
    /// - `InputMediaAnimation`. Animation width
    /// - `InputMediaVideo`. Video width
    #[must_use]
    pub fn width(&self) -> Option<i64> {
        match self {
            Self::Animation(val) => val.width,
            Self::Video(val) => val.width,
            _ => None,
        }
    }
}
impl From<crate::types::InputMediaAnimation> for InputMedia {
    fn from(val: crate::types::InputMediaAnimation) -> Self {
        Self::Animation(val)
    }
}
impl TryFrom<InputMedia> for crate::types::InputMediaAnimation {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: InputMedia) -> Result<Self, Self::Error> {
        if let InputMedia::Animation(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(InputMedia),
                stringify!(InputMediaAnimation),
            ))
        }
    }
}
impl From<crate::types::InputMediaAudio> for InputMedia {
    fn from(val: crate::types::InputMediaAudio) -> Self {
        Self::Audio(val)
    }
}
impl TryFrom<InputMedia> for crate::types::InputMediaAudio {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: InputMedia) -> Result<Self, Self::Error> {
        if let InputMedia::Audio(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(InputMedia),
                stringify!(InputMediaAudio),
            ))
        }
    }
}
impl From<crate::types::InputMediaDocument> for InputMedia {
    fn from(val: crate::types::InputMediaDocument) -> Self {
        Self::Document(val)
    }
}
impl TryFrom<InputMedia> for crate::types::InputMediaDocument {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: InputMedia) -> Result<Self, Self::Error> {
        if let InputMedia::Document(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(InputMedia),
                stringify!(InputMediaDocument),
            ))
        }
    }
}
impl From<crate::types::InputMediaLivePhoto> for InputMedia {
    fn from(val: crate::types::InputMediaLivePhoto) -> Self {
        Self::LivePhoto(val)
    }
}
impl TryFrom<InputMedia> for crate::types::InputMediaLivePhoto {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: InputMedia) -> Result<Self, Self::Error> {
        if let InputMedia::LivePhoto(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(InputMedia),
                stringify!(InputMediaLivePhoto),
            ))
        }
    }
}
impl From<crate::types::InputMediaPhoto> for InputMedia {
    fn from(val: crate::types::InputMediaPhoto) -> Self {
        Self::Photo(val)
    }
}
impl TryFrom<InputMedia> for crate::types::InputMediaPhoto {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: InputMedia) -> Result<Self, Self::Error> {
        if let InputMedia::Photo(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(InputMedia),
                stringify!(InputMediaPhoto),
            ))
        }
    }
}
impl From<crate::types::InputMediaVideo> for InputMedia {
    fn from(val: crate::types::InputMediaVideo) -> Self {
        Self::Video(val)
    }
}
impl TryFrom<InputMedia> for crate::types::InputMediaVideo {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: InputMedia) -> Result<Self, Self::Error> {
        if let InputMedia::Video(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(InputMedia),
                stringify!(InputMediaVideo),
            ))
        }
    }
}
