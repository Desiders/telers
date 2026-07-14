use serde::{Deserialize, Serialize};
/// Represents a live photo to be sent.
/// # Documentation
/// <https://core.telegram.org/bots/api#inputmedialivephoto>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InputMediaLivePhoto {
    /// Video of the live photo to send. Pass a `file_id` to send a file that exists on the Telegram servers (recommended) or pass `attach://<file_attach_name>` to upload a new one using multipart/form-data under <`file_attach_name`> name. More information on Sending Files: <https://core.telegram.org/bots/api#sending-files>. Sending live photos by a URL is currently unsupported.
    pub media: crate::types::InputFile,
    /// The static photo to send. Pass a `file_id` to send a file that exists on the Telegram servers (recommended) or pass `attach://<file_attach_name>` to upload a new one using multipart/form-data under <`file_attach_name`> name. More information on Sending Files: <https://core.telegram.org/bots/api#sending-files>. Sending live photos by a URL is currently unsupported.
    pub photo: crate::types::InputFile,
    /// Caption of the live photo to be sent, 0-1024 characters after entities parsing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<Box<str>>,
    /// Mode for parsing entities in the live photo caption. See formatting options for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<Box<str>>,
    /// List of special entities that appear in the caption, which can be specified instead of `parse_mode`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Box<[crate::types::MessageEntity]>>,
    /// Pass `true` if the caption must be shown above the message media
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_caption_above_media: Option<bool>,
    /// Pass `true` if the live photo needs to be covered with a spoiler animation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_spoiler: Option<bool>,
}
impl InputMediaLivePhoto {
    /// Creates a new `InputMediaLivePhoto`.
    ///
    /// # Arguments
    /// * `media` - Video of the live photo to send. Pass a `file_id` to send a file that exists on the Telegram servers (recommended) or pass `attach://<file_attach_name>` to upload a new one using multipart/form-data under <`file_attach_name`> name. More information on Sending Files: <https://core.telegram.org/bots/api#sending-files>. Sending live photos by a URL is currently unsupported.
    /// * `photo` - The static photo to send. Pass a `file_id` to send a file that exists on the Telegram servers (recommended) or pass `attach://<file_attach_name>` to upload a new one using multipart/form-data under <`file_attach_name`> name. More information on Sending Files: <https://core.telegram.org/bots/api#sending-files>. Sending live photos by a URL is currently unsupported.
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0: Into<crate::types::InputFile>, T1: Into<crate::types::InputFile>>(
        media: T0,
        photo: T1,
    ) -> Self {
        Self {
            media: media.into(),
            photo: photo.into(),
            caption: None,
            parse_mode: None,
            caption_entities: None,
            show_caption_above_media: None,
            has_spoiler: None,
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

    /// Caption of the live photo to be sent, 0-1024 characters after entities parsing
    #[must_use]
    pub fn caption<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.caption = Some(val.into());
        self
    }

    /// Caption of the live photo to be sent, 0-1024 characters after entities parsing
    #[must_use]
    pub fn caption_option<T: Into<Box<str>>>(mut self, val: Option<T>) -> Self {
        self.caption = val.map(Into::into);
        self
    }

    /// Mode for parsing entities in the live photo caption. See formatting options for more details.
    #[must_use]
    pub fn parse_mode<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.parse_mode = Some(val.into());
        self
    }

    /// Mode for parsing entities in the live photo caption. See formatting options for more details.
    #[must_use]
    pub fn parse_mode_option<T: Into<Box<str>>>(mut self, val: Option<T>) -> Self {
        self.parse_mode = val.map(Into::into);
        self
    }

    /// List of special entities that appear in the caption, which can be specified instead of `parse_mode`
    ///
    /// # Notes
    /// Adds multiple elements.
    #[must_use]
    pub fn caption_entities<T: Into<Box<[crate::types::MessageEntity]>>>(mut self, val: T) -> Self {
        self.caption_entities = Some(
            self.caption_entities
                .unwrap_or_default()
                .into_vec()
                .into_iter()
                .chain(val.into())
                .collect(),
        );
        self
    }

    /// List of special entities that appear in the caption, which can be specified instead of `parse_mode`
    ///
    /// # Notes
    /// Adds a single element.
    #[must_use]
    pub fn caption_entity<T: Into<crate::types::MessageEntity>>(mut self, val: T) -> Self {
        self.caption_entities = Some(
            self.caption_entities
                .unwrap_or_default()
                .into_vec()
                .into_iter()
                .chain(Some(val.into()))
                .collect(),
        );
        self
    }

    /// List of special entities that appear in the caption, which can be specified instead of `parse_mode`
    ///
    /// # Notes
    /// Adds a single element.
    #[must_use]
    pub fn caption_entities_option<T: Into<Box<[crate::types::MessageEntity]>>>(
        mut self,
        val: Option<T>,
    ) -> Self {
        self.caption_entities = val.map(Into::into);
        self
    }

    /// Pass `true` if the caption must be shown above the message media
    #[must_use]
    pub fn show_caption_above_media<T: Into<bool>>(mut self, val: T) -> Self {
        self.show_caption_above_media = Some(val.into());
        self
    }

    /// Pass `true` if the caption must be shown above the message media
    #[must_use]
    pub fn show_caption_above_media_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.show_caption_above_media = val.map(Into::into);
        self
    }

    /// Pass `true` if the live photo needs to be covered with a spoiler animation
    #[must_use]
    pub fn has_spoiler<T: Into<bool>>(mut self, val: T) -> Self {
        self.has_spoiler = Some(val.into());
        self
    }

    /// Pass `true` if the live photo needs to be covered with a spoiler animation
    #[must_use]
    pub fn has_spoiler_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.has_spoiler = val.map(Into::into);
        self
    }
}
