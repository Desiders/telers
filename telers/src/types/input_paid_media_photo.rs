use serde::{Deserialize, Serialize};
/// The paid media to send is a photo.
/// # Documentation
/// <https://core.telegram.org/bots/api#inputpaidmediaphoto>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InputPaidMediaPhoto {
    /// File to send. Pass a `file_id` to send a file that exists on the Telegram servers (recommended), pass an HTTP URL for Telegram to get a file from the Internet, or pass `attach://<file_attach_name>` to upload a new one using multipart/form-data under <`file_attach_name`> name. More information on Sending Files: <https://core.telegram.org/bots/api#sending-files>
    pub media: crate::types::InputFile,
}
impl InputPaidMediaPhoto {
    /// Creates a new `InputPaidMediaPhoto`.
    ///
    /// # Arguments
    /// * `media` - File to send. Pass a `file_id` to send a file that exists on the Telegram servers (recommended), pass an HTTP URL for Telegram to get a file from the Internet, or pass `attach://<file_attach_name>` to upload a new one using multipart/form-data under <`file_attach_name`> name. More information on Sending Files: <https://core.telegram.org/bots/api#sending-files>
    #[must_use]
    pub fn new<T0: Into<crate::types::InputFile>>(media: T0) -> Self {
        Self {
            media: media.into(),
        }
    }

    /// File to send. Pass a `file_id` to send a file that exists on the Telegram servers (recommended), pass an HTTP URL for Telegram to get a file from the Internet, or pass `attach://<file_attach_name>` to upload a new one using multipart/form-data under <`file_attach_name`> name. More information on Sending Files: <https://core.telegram.org/bots/api#sending-files>
    #[must_use]
    pub fn media<T: Into<crate::types::InputFile>>(mut self, val: T) -> Self {
        self.media = val.into();
        self
    }
}
