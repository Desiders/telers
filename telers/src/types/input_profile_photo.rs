use serde::{Deserialize, Serialize};
/// This object describes a profile photo to set. Currently, it can be one of
/// - [`InputProfilePhotoStatic`]
/// - [`InputProfilePhotoAnimated`]
/// # Documentation
/// <https://core.telegram.org/bots/api#inputprofilephoto>
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum InputProfilePhoto {
    Static(crate::types::InputProfilePhotoStatic),
    Animated(crate::types::InputProfilePhotoAnimated),
}
impl InputProfilePhoto {
    /// Helper method for field `animation`.
    ///
    /// The animated profile photo. Profile photos can't be reused and can only be uploaded as a new file, so you can pass `attach://<file_attach_name>` if the photo was uploaded using multipart/form-data under <`file_attach_name`>. More information on Sending Files: <https://core.telegram.org/bots/api#sending-files>
    #[must_use]
    pub fn animation(&self) -> Option<&crate::types::InputFile> {
        match self {
            Self::Animated(val) => Some(&val.animation),
            Self::Static(_) => None,
        }
    }

    /// Helper method for field `main_frame_timestamp`.
    ///
    /// Timestamp in seconds of the frame that will be used as the static profile photo. Defaults to 0.0.
    #[must_use]
    pub fn main_frame_timestamp(&self) -> Option<f64> {
        match self {
            Self::Animated(val) => val.main_frame_timestamp,
            Self::Static(_) => None,
        }
    }

    /// Helper method for field `photo`.
    ///
    /// The static profile photo. Profile photos can't be reused and can only be uploaded as a new file, so you can pass `attach://<file_attach_name>` if the photo was uploaded using multipart/form-data under <`file_attach_name`>. More information on Sending Files: <https://core.telegram.org/bots/api#sending-files>
    #[must_use]
    pub fn photo(&self) -> Option<&crate::types::InputFile> {
        match self {
            Self::Static(val) => Some(&val.photo),
            Self::Animated(_) => None,
        }
    }
}
impl From<crate::types::InputProfilePhotoStatic> for InputProfilePhoto {
    fn from(val: crate::types::InputProfilePhotoStatic) -> Self {
        Self::Static(val)
    }
}
impl TryFrom<InputProfilePhoto> for crate::types::InputProfilePhotoStatic {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: InputProfilePhoto) -> Result<Self, Self::Error> {
        match val {
            InputProfilePhoto::Static(inner) => Ok(inner),
            InputProfilePhoto::Animated(_) => Err(Self::Error::new(
                stringify!(InputProfilePhoto),
                stringify!(InputProfilePhotoStatic),
            )),
        }
    }
}
impl From<crate::types::InputProfilePhotoAnimated> for InputProfilePhoto {
    fn from(val: crate::types::InputProfilePhotoAnimated) -> Self {
        Self::Animated(val)
    }
}
impl TryFrom<InputProfilePhoto> for crate::types::InputProfilePhotoAnimated {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: InputProfilePhoto) -> Result<Self, Self::Error> {
        match val {
            InputProfilePhoto::Animated(inner) => Ok(inner),
            InputProfilePhoto::Static(_) => Err(Self::Error::new(
                stringify!(InputProfilePhoto),
                stringify!(InputProfilePhotoAnimated),
            )),
        }
    }
}
