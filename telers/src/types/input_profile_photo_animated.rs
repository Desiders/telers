use serde::Serialize;

use crate::types::InputFile;

/// An animated profile photo in the MPEG4 format.
/// # Documentation
/// <https://core.telegram.org/bots/api#inputprofilephotoanimated>
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct InputProfilePhotoAnimated<'a> {
    /// The animated profile photo. Profile photos can't bereused and can be only uploaded as a new file, so you can pass `attach://<file_attach_name>` if the photo was uploaded using `multipart/form-data` under <file_attach_name>. [`More information on Sending Files`](https://core.telegram.org/bots/api#sending-files).
    pub animation: InputFile<'a>,
    /// Timestamp in seconds of the frame that will be used as the static profile photo. Defaults to 0.0.
    pub main_frame_timestamp: Option<f64>,
}

impl<'a> InputProfilePhotoAnimated<'a> {
    #[must_use]
    pub fn new(animation: impl Into<InputFile<'a>>) -> Self {
        Self {
            animation: animation.into(),
            main_frame_timestamp: None,
        }
    }

    #[must_use]
    pub fn animation(self, val: impl Into<InputFile<'a>>) -> Self {
        Self {
            animation: val.into(),
            ..self
        }
    }

    #[must_use]
    pub fn main_frame_timestamp(self, val: f64) -> Self {
        Self {
            main_frame_timestamp: Some(val),
            ..self
        }
    }
}

impl InputProfilePhotoAnimated<'_> {
    #[must_use]
    pub fn main_frame_timestamp_option(self, val: Option<f64>) -> Self {
        Self {
            main_frame_timestamp: val,
            ..self
        }
    }
}
