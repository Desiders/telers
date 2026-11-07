/// The paid media to send is a live photo.
/// # Documentation
/// <https://core.telegram.org/bots/api#inputpaidmedialivephoto>
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deser", derive(deser::Serialize, deser::Deserialize))]
#[cfg_attr(not(feature = "deser"), derive(serde::Serialize))]
pub struct InputPaidMediaLivePhoto {
    /// Video of the live photo to send. Pass a `file_id` to send a file that exists on the Telegram servers (recommended) or pass `attach://<file_attach_name>` to upload a new one using multipart/form-data under <`file_attach_name`> name. More information on Sending Files: <https://core.telegram.org/bots/api#sending-files>. Sending live photos by a URL is currently unsupported.
    pub media: crate::types::InputFile,
    /// The static photo to send. Pass a `file_id` to send a file that exists on the Telegram servers (recommended) or pass `attach://<file_attach_name>` to upload a new one using multipart/form-data under <`file_attach_name`> name. More information on Sending Files: <https://core.telegram.org/bots/api#sending-files>. Sending live photos by a URL is currently unsupported.
    pub photo: crate::types::InputFile,
}
impl InputPaidMediaLivePhoto {
    /// Creates a new `InputPaidMediaLivePhoto`.
    ///
    /// # Arguments
    /// * `media` - Video of the live photo to send. Pass a `file_id` to send a file that exists on the Telegram servers (recommended) or pass `attach://<file_attach_name>` to upload a new one using multipart/form-data under <`file_attach_name`> name. More information on Sending Files: <https://core.telegram.org/bots/api#sending-files>. Sending live photos by a URL is currently unsupported.
    /// * `photo` - The static photo to send. Pass a `file_id` to send a file that exists on the Telegram servers (recommended) or pass `attach://<file_attach_name>` to upload a new one using multipart/form-data under <`file_attach_name`> name. More information on Sending Files: <https://core.telegram.org/bots/api#sending-files>. Sending live photos by a URL is currently unsupported.
    #[must_use]
    pub fn new<T0: Into<crate::types::InputFile>, T1: Into<crate::types::InputFile>>(
        media: T0,
        photo: T1,
    ) -> Self {
        Self {
            media: media.into(),
            photo: photo.into(),
        }
    }

    /// Video of the live photo to send. Pass a `file_id` to send a file that exists on the Telegram servers (recommended) or pass `attach://<file_attach_name>` to upload a new one using multipart/form-data under <`file_attach_name`> name. More information on Sending Files: <https://core.telegram.org/bots/api#sending-files>. Sending live photos by a URL is currently unsupported.
    #[must_use]
    pub fn media<T: Into<crate::types::InputFile>>(mut self, val: T) -> Self {
        self.media = val.into();
        self
    }

    /// The static photo to send. Pass a `file_id` to send a file that exists on the Telegram servers (recommended) or pass `attach://<file_attach_name>` to upload a new one using multipart/form-data under <`file_attach_name`> name. More information on Sending Files: <https://core.telegram.org/bots/api#sending-files>. Sending live photos by a URL is currently unsupported.
    #[must_use]
    pub fn photo<T: Into<crate::types::InputFile>>(mut self, val: T) -> Self {
        self.photo = val.into();
        self
    }
}
#[cfg(not(feature = "deser"))]
impl<'de> serde::Deserialize<'de> for InputPaidMediaLivePhoto {
    #[allow(clippy::too_many_lines)]
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let meta = crate::deserialization::Meta {
            name: "InputPaidMediaLivePhoto",
            fields: &["media", "photo"],
            specific: &["photo"],
            sequence: "struct InputPaidMediaLivePhoto with 2 elements",
        };
        let (common, (value_photo,)) = crate::deserialization::input_paid_media::read_0::<
            D,
            crate::types::InputFile,
        >(deserializer, meta)?;
        Ok(Self {
            media: common.media,
            photo: value_photo,
        })
    }
}
