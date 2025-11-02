use serde::Serialize;
use serde_with::skip_serializing_none;

use crate::types::InputFile;

/// Describes a video to post as a story.
/// # Documentation
/// <https://core.telegram.org/bots/api#ownedgiftregular>
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct InputStoryContentVideo<'a> {
    /// The video to post as a story. The video must be of the size 720x1280, streamable, encoded with H.265 codec, with key frames added each second in the MPEG4 format, and must not exceed 30 MB. The video can't be reused and can only be uploaded as a new file, so you can pass `attach://<file_attach_name>` if the video was uploaded using `multipart/form-data` under <file_attach_name>. [`More information on Sending Files`](https://core.telegram.org/bots/api#sending-files).
    pub video: InputFile<'a>,
    /// Precise duration of the video in seconds; 0-60
    pub duration: Option<u8>,
    /// Timestamp in seconds of the frame that will be used as the static cover for the story. Defaults to 0.0.
    pub cover_frame_timestamp: Option<f32>,
    /// Pass `true` if the video has no sound
    pub is_animation: Option<bool>,
}
