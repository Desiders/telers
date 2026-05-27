use serde::{Deserialize, Serialize};
/// An animated profile photo in the MPEG4 format.
/// # Documentation
/// <https://core.telegram.org/bots/api#inputprofilephotoanimated>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InputProfilePhotoAnimated {
    /// The animated profile photo. Profile photos can't be reused and can only be uploaded as a new file, so you can pass `attach://<file_attach_name>` if the photo was uploaded using multipart/form-data under <`file_attach_name`>. More information on Sending Files: <https://core.telegram.org/bots/api#sending-files>
    pub animation: crate::types::InputFile,
    /// Timestamp in seconds of the frame that will be used as the static profile photo. Defaults to 0.0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub main_frame_timestamp: Option<f64>,
}
impl InputProfilePhotoAnimated {
    /// Creates a new `InputProfilePhotoAnimated`.
    ///
    /// # Arguments
    /// * `animation` - The animated profile photo. Profile photos can't be reused and can only be uploaded as a new file, so you can pass `attach://<file_attach_name>` if the photo was uploaded using multipart/form-data under <`file_attach_name`>. More information on Sending Files: <https://core.telegram.org/bots/api#sending-files>
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0: Into<crate::types::InputFile>>(animation: T0) -> Self {
        Self {
            animation: animation.into(),
            main_frame_timestamp: None,
        }
    }

    /// The animated profile photo. Profile photos can't be reused and can only be uploaded as a new file, so you can pass `attach://<file_attach_name>` if the photo was uploaded using multipart/form-data under <`file_attach_name`>. More information on Sending Files: <https://core.telegram.org/bots/api#sending-files>
    #[must_use]
    pub fn animation<T: Into<crate::types::InputFile>>(mut self, val: T) -> Self {
        self.animation = val.into();
        self
    }

    /// Timestamp in seconds of the frame that will be used as the static profile photo. Defaults to 0.0.
    #[must_use]
    pub fn main_frame_timestamp<T: Into<f64>>(mut self, val: T) -> Self {
        self.main_frame_timestamp = Some(val.into());
        self
    }

    /// Timestamp in seconds of the frame that will be used as the static profile photo. Defaults to 0.0.
    #[must_use]
    pub fn main_frame_timestamp_option<T: Into<f64>>(mut self, val: Option<T>) -> Self {
        self.main_frame_timestamp = val.map(Into::into);
        self
    }
}
