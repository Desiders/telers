use serde::{Deserialize, Serialize};
/// A static profile photo in the .JPG format.
/// # Documentation
/// <https://core.telegram.org/bots/api#inputprofilephotostatic>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InputProfilePhotoStatic {
    /// The static profile photo. Profile photos can't be reused and can only be uploaded as a new file, so you can pass `attach://<file_attach_name>` if the photo was uploaded using multipart/form-data under <`file_attach_name`>. More information on Sending Files: <https://core.telegram.org/bots/api#sending-files>
    pub photo: crate::types::InputFile,
}
impl InputProfilePhotoStatic {
    /// Creates a new `InputProfilePhotoStatic`.
    ///
    /// # Arguments
    /// * `photo` - The static profile photo. Profile photos can't be reused and can only be uploaded as a new file, so you can pass `attach://<file_attach_name>` if the photo was uploaded using multipart/form-data under <`file_attach_name`>. More information on Sending Files: <https://core.telegram.org/bots/api#sending-files>
    #[must_use]
    pub fn new<T0: Into<crate::types::InputFile>>(photo: T0) -> Self {
        Self {
            photo: photo.into(),
        }
    }

    /// The static profile photo. Profile photos can't be reused and can only be uploaded as a new file, so you can pass `attach://<file_attach_name>` if the photo was uploaded using multipart/form-data under <`file_attach_name`>. More information on Sending Files: <https://core.telegram.org/bots/api#sending-files>
    #[must_use]
    pub fn photo<T: Into<crate::types::InputFile>>(self, val: T) -> Self {
        let mut this = self;
        this.photo = val.into();
        this
    }
}
