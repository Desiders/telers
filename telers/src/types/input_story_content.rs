use serde::{Deserialize, Serialize};
/// This object describes the content of a story to post. Currently, it can be one of
/// - [`InputStoryContentPhoto`]
/// - [`InputStoryContentVideo`]
/// # Documentation
/// <https://core.telegram.org/bots/api#inputstorycontent>
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum InputStoryContent {
    Photo(crate::types::InputStoryContentPhoto),
    Video(crate::types::InputStoryContentVideo),
}
impl InputStoryContent {
    /// Helper method for field `cover_frame_timestamp`.
    ///
    /// Timestamp in seconds of the frame that will be used as the static cover for the story. Defaults to 0.0.
    #[must_use]
    pub fn cover_frame_timestamp(&self) -> Option<f64> {
        match self {
            Self::Video(val) => val.cover_frame_timestamp,
            Self::Photo(_) => None,
        }
    }

    /// Helper method for field `duration`.
    ///
    /// Precise duration of the video in seconds; 0-60
    #[must_use]
    pub fn duration(&self) -> Option<f64> {
        match self {
            Self::Video(val) => val.duration,
            Self::Photo(_) => None,
        }
    }

    /// Helper method for field `is_animation`.
    ///
    /// Pass `true` if the video has no sound
    #[must_use]
    pub fn is_animation(&self) -> Option<bool> {
        match self {
            Self::Video(val) => val.is_animation,
            Self::Photo(_) => None,
        }
    }

    /// Helper method for field `photo`.
    ///
    /// The photo to post as a story. The photo must be of the size 1080x1920 and must not exceed 10 MB. The photo can't be reused and can only be uploaded as a new file, so you can pass `attach://<file_attach_name>` if the photo was uploaded using multipart/form-data under <`file_attach_name`>. More information on Sending Files: <https://core.telegram.org/bots/api#sending-files>
    #[must_use]
    pub fn photo(&self) -> Option<&crate::types::InputFile> {
        match self {
            Self::Photo(val) => Some(&val.photo),
            Self::Video(_) => None,
        }
    }

    /// Helper method for field `video`.
    ///
    /// The video to post as a story. The video must be of the size 720x1280, streamable, encoded with H.265 codec, with key frames added each second in the MPEG4 format, and must not exceed 30 MB. The video can't be reused and can only be uploaded as a new file, so you can pass `attach://<file_attach_name>` if the video was uploaded using multipart/form-data under <`file_attach_name`>. More information on Sending Files: <https://core.telegram.org/bots/api#sending-files>
    #[must_use]
    pub fn video(&self) -> Option<&crate::types::InputFile> {
        match self {
            Self::Video(val) => Some(&val.video),
            Self::Photo(_) => None,
        }
    }
}
impl From<crate::types::InputStoryContentPhoto> for InputStoryContent {
    fn from(val: crate::types::InputStoryContentPhoto) -> Self {
        Self::Photo(val)
    }
}
impl TryFrom<InputStoryContent> for crate::types::InputStoryContentPhoto {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: InputStoryContent) -> Result<Self, Self::Error> {
        match val {
            InputStoryContent::Photo(inner) => Ok(inner),
            InputStoryContent::Video(_) => Err(Self::Error::new(
                stringify!(InputStoryContent),
                stringify!(InputStoryContentPhoto),
            )),
        }
    }
}
impl From<crate::types::InputStoryContentVideo> for InputStoryContent {
    fn from(val: crate::types::InputStoryContentVideo) -> Self {
        Self::Video(val)
    }
}
impl TryFrom<InputStoryContent> for crate::types::InputStoryContentVideo {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: InputStoryContent) -> Result<Self, Self::Error> {
        match val {
            InputStoryContent::Video(inner) => Ok(inner),
            InputStoryContent::Photo(_) => Err(Self::Error::new(
                stringify!(InputStoryContent),
                stringify!(InputStoryContentVideo),
            )),
        }
    }
}
