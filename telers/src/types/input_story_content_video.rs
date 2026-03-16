use serde::{Deserialize, Serialize};
/// Describes a video to post as a story.
/// # Documentation
/// <https://core.telegram.org/bots/api#inputstorycontentvideo>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InputStoryContentVideo {
    /// The video to post as a story. The video must be of the size 720x1280, streamable, encoded with H.265 codec, with key frames added each second in the MPEG4 format, and must not exceed 30 MB. The video can't be reused and can only be uploaded as a new file, so you can pass `attach://<file_attach_name>` if the video was uploaded using multipart/form-data under <`file_attach_name`>. More information on Sending Files: <https://core.telegram.org/bots/api#sending-files>
    pub video: crate::types::InputFile,
    /// Precise duration of the video in seconds; 0-60
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<f64>,
    /// Timestamp in seconds of the frame that will be used as the static cover for the story. Defaults to 0.0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cover_frame_timestamp: Option<f64>,
    /// Pass `true` if the video has no sound
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_animation: Option<bool>,
}
impl InputStoryContentVideo {
    /// Creates a new `InputStoryContentVideo`.
    ///
    /// # Arguments
    /// * `video` - The video to post as a story. The video must be of the size 720x1280, streamable, encoded with H.265 codec, with key frames added each second in the MPEG4 format, and must not exceed 30 MB. The video can't be reused and can only be uploaded as a new file, so you can pass `attach://<file_attach_name>` if the video was uploaded using multipart/form-data under <`file_attach_name`>. More information on Sending Files: <https://core.telegram.org/bots/api#sending-files>
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0: Into<crate::types::InputFile>>(video: T0) -> Self {
        Self {
            video: video.into(),
            duration: None,
            cover_frame_timestamp: None,
            is_animation: None,
        }
    }

    /// The video to post as a story. The video must be of the size 720x1280, streamable, encoded with H.265 codec, with key frames added each second in the MPEG4 format, and must not exceed 30 MB. The video can't be reused and can only be uploaded as a new file, so you can pass `attach://<file_attach_name>` if the video was uploaded using multipart/form-data under <`file_attach_name`>. More information on Sending Files: <https://core.telegram.org/bots/api#sending-files>
    #[must_use]
    pub fn video<T: Into<crate::types::InputFile>>(self, val: T) -> Self {
        let mut this = self;
        this.video = val.into();
        this
    }

    /// Precise duration of the video in seconds; 0-60
    #[must_use]
    pub fn duration<T: Into<f64>>(self, val: T) -> Self {
        let mut this = self;
        this.duration = Some(val.into());
        this
    }

    /// Precise duration of the video in seconds; 0-60
    #[must_use]
    pub fn duration_option<T: Into<f64>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.duration = val.map(Into::into);
        this
    }

    /// Timestamp in seconds of the frame that will be used as the static cover for the story. Defaults to 0.0.
    #[must_use]
    pub fn cover_frame_timestamp<T: Into<f64>>(self, val: T) -> Self {
        let mut this = self;
        this.cover_frame_timestamp = Some(val.into());
        this
    }

    /// Timestamp in seconds of the frame that will be used as the static cover for the story. Defaults to 0.0.
    #[must_use]
    pub fn cover_frame_timestamp_option<T: Into<f64>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.cover_frame_timestamp = val.map(Into::into);
        this
    }

    /// Pass `true` if the video has no sound
    #[must_use]
    pub fn is_animation<T: Into<bool>>(self, val: T) -> Self {
        let mut this = self;
        this.is_animation = Some(val.into());
        this
    }

    /// Pass `true` if the video has no sound
    #[must_use]
    pub fn is_animation_option<T: Into<bool>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.is_animation = val.map(Into::into);
        this
    }
}
