use serde::Serialize;

use crate::types::InputFile;

/// A static profile photo in the .JPG format
/// # Documentation
/// <https://core.telegram.org/bots/api#inputprofilephotostatic>
#[derive(Debug, Hash, PartialEq, Serialize)]
pub struct InputProfilePhotoStatic {
    /// The static profile photo. Profile photos can't bereused and can be only uploaded as a new file, so you can pass `attach://<file_attach_name>` if the photo was uploaded using `multipart/form-data` under `<file_attach_name>`. [`More information on Sending Files`](https://core.telegram.org/bots/api#sending-files).
    pub photo: InputFile,
}

impl InputProfilePhotoStatic {
    #[must_use]
    pub fn new(photo: impl Into<InputFile>) -> Self {
        Self {
            photo: photo.into(),
        }
    }

    #[must_use]
    pub fn photo(self, val: impl Into<InputFile>) -> Self {
        Self { photo: val.into() }
    }
}
