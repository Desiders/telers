use serde::{Deserialize, Serialize};
/// Describes a photo to post as a story.
/// # Documentation
/// <https://core.telegram.org/bots/api#inputstorycontentphoto>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InputStoryContentPhoto {
    /// The photo to post as a story. The photo must be of the size 1080x1920 and must not exceed 10 MB. The photo can't be reused and can only be uploaded as a new file, so you can pass `attach://<file_attach_name>` if the photo was uploaded using multipart/form-data under <`file_attach_name`>. More information on Sending Files: <https://core.telegram.org/bots/api#sending-files>
    pub photo: crate::types::InputFile,
}
impl InputStoryContentPhoto {
    /// Creates a new `InputStoryContentPhoto`.
    ///
    /// # Arguments
    /// * `photo` - The photo to post as a story. The photo must be of the size 1080x1920 and must not exceed 10 MB. The photo can't be reused and can only be uploaded as a new file, so you can pass `attach://<file_attach_name>` if the photo was uploaded using multipart/form-data under <`file_attach_name`>. More information on Sending Files: <https://core.telegram.org/bots/api#sending-files>
    #[must_use]
    pub fn new<T0: Into<crate::types::InputFile>>(photo: T0) -> Self {
        Self {
            photo: photo.into(),
        }
    }

    /// The photo to post as a story. The photo must be of the size 1080x1920 and must not exceed 10 MB. The photo can't be reused and can only be uploaded as a new file, so you can pass `attach://<file_attach_name>` if the photo was uploaded using multipart/form-data under <`file_attach_name`>. More information on Sending Files: <https://core.telegram.org/bots/api#sending-files>
    #[must_use]
    pub fn photo<T: Into<crate::types::InputFile>>(mut self, val: T) -> Self {
        self.photo = val.into();
        self
    }
}
