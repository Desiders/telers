use serde::{Deserialize, Serialize};
/// Represents a general file to be sent.
/// # Documentation
/// <https://core.telegram.org/bots/api#inputmediadocument>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InputMediaDocument {
    /// File to send. Pass a `file_id` to send a file that exists on the Telegram servers (recommended), pass an HTTP URL for Telegram to get a file from the Internet, or pass `attach://<file_attach_name>` to upload a new one using multipart/form-data under <`file_attach_name`> name. More information on Sending Files: <https://core.telegram.org/bots/api#sending-files>
    pub media: crate::types::InputFile,
    /// Thumbnail of the file sent; can be ignored if thumbnail generation for the file is supported server-side. The thumbnail should be in JPEG format and less than 200 kB in size. A thumbnail's width and height should not exceed 320. Ignored if the file is not uploaded using multipart/form-data. Thumbnails can't be reused and can be only uploaded as a new file, so you can pass `attach://<file_attach_name>` if the thumbnail was uploaded using multipart/form-data under <`file_attach_name`>. More information on Sending Files: <https://core.telegram.org/bots/api#sending-files>
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<crate::types::InputFile>,
    /// Caption of the document to be sent, 0-1024 characters after entities parsing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<Box<str>>,
    /// Mode for parsing entities in the document caption. See formatting options for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<Box<str>>,
    /// List of special entities that appear in the caption, which can be specified instead of `parse_mode`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Box<[crate::types::MessageEntity]>>,
    /// Disables automatic server-side content type detection for files uploaded using multipart/form-data. Always `true`, if the document is sent as part of an album.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_content_type_detection: Option<bool>,
}
impl InputMediaDocument {
    /// Creates a new `InputMediaDocument`.
    ///
    /// # Arguments
    /// * `media` - File to send. Pass a `file_id` to send a file that exists on the Telegram servers (recommended), pass an HTTP URL for Telegram to get a file from the Internet, or pass `attach://<file_attach_name>` to upload a new one using multipart/form-data under <`file_attach_name`> name. More information on Sending Files: <https://core.telegram.org/bots/api#sending-files>
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0: Into<crate::types::InputFile>>(media: T0) -> Self {
        Self {
            media: media.into(),
            thumbnail: None,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            disable_content_type_detection: None,
        }
    }

    /// File to send. Pass a `file_id` to send a file that exists on the Telegram servers (recommended), pass an HTTP URL for Telegram to get a file from the Internet, or pass `attach://<file_attach_name>` to upload a new one using multipart/form-data under <`file_attach_name`> name. More information on Sending Files: <https://core.telegram.org/bots/api#sending-files>
    #[must_use]
    pub fn media<T: Into<crate::types::InputFile>>(mut self, val: T) -> Self {
        self.media = val.into();
        self
    }

    /// Thumbnail of the file sent; can be ignored if thumbnail generation for the file is supported server-side. The thumbnail should be in JPEG format and less than 200 kB in size. A thumbnail's width and height should not exceed 320. Ignored if the file is not uploaded using multipart/form-data. Thumbnails can't be reused and can be only uploaded as a new file, so you can pass `attach://<file_attach_name>` if the thumbnail was uploaded using multipart/form-data under <`file_attach_name`>. More information on Sending Files: <https://core.telegram.org/bots/api#sending-files>
    #[must_use]
    pub fn thumbnail<T: Into<crate::types::InputFile>>(mut self, val: T) -> Self {
        self.thumbnail = Some(val.into());
        self
    }

    /// Thumbnail of the file sent; can be ignored if thumbnail generation for the file is supported server-side. The thumbnail should be in JPEG format and less than 200 kB in size. A thumbnail's width and height should not exceed 320. Ignored if the file is not uploaded using multipart/form-data. Thumbnails can't be reused and can be only uploaded as a new file, so you can pass `attach://<file_attach_name>` if the thumbnail was uploaded using multipart/form-data under <`file_attach_name`>. More information on Sending Files: <https://core.telegram.org/bots/api#sending-files>
    #[must_use]
    pub fn thumbnail_option<T: Into<crate::types::InputFile>>(mut self, val: Option<T>) -> Self {
        self.thumbnail = val.map(Into::into);
        self
    }

    /// Caption of the document to be sent, 0-1024 characters after entities parsing
    #[must_use]
    pub fn caption<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.caption = Some(val.into());
        self
    }

    /// Caption of the document to be sent, 0-1024 characters after entities parsing
    #[must_use]
    pub fn caption_option<T: Into<Box<str>>>(mut self, val: Option<T>) -> Self {
        self.caption = val.map(Into::into);
        self
    }

    /// Mode for parsing entities in the document caption. See formatting options for more details.
    #[must_use]
    pub fn parse_mode<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.parse_mode = Some(val.into());
        self
    }

    /// Mode for parsing entities in the document caption. See formatting options for more details.
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

    /// Disables automatic server-side content type detection for files uploaded using multipart/form-data. Always `true`, if the document is sent as part of an album.
    #[must_use]
    pub fn disable_content_type_detection<T: Into<bool>>(mut self, val: T) -> Self {
        self.disable_content_type_detection = Some(val.into());
        self
    }

    /// Disables automatic server-side content type detection for files uploaded using multipart/form-data. Always `true`, if the document is sent as part of an album.
    #[must_use]
    pub fn disable_content_type_detection_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.disable_content_type_detection = val.map(Into::into);
        self
    }
}
