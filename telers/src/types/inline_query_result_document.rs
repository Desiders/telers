use serde::{Deserialize, Serialize};
/// Represents a link to a file. By default, this file will be sent by the user with an optional caption. Alternatively, you can use `input_message_content` to send a message with the specified content instead of the file. Currently, only .PDF and .ZIP files can be sent using this method.
/// # Documentation
/// <https://core.telegram.org/bots/api#inlinequeryresultdocument>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InlineQueryResultDocument {
    /// Unique identifier for this result, 1-64 bytes
    pub id: Box<str>,
    /// Title for the result
    pub title: Box<str>,
    /// Caption of the document to be sent, 0-1024 characters after entities parsing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<Box<str>>,
    /// Mode for parsing entities in the document caption. See formatting options for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<Box<str>>,
    /// List of special entities that appear in the caption, which can be specified instead of `parse_mode`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Box<[crate::types::MessageEntity]>>,
    /// A valid URL for the file
    pub document_url: Box<str>,
    /// MIME type of the content of the file, either `application/pdf` or `application/zip`
    pub mime_type: Box<str>,
    /// Short description of the result
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Box<str>>,
    /// Inline keyboard attached to the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<crate::types::InlineKeyboardMarkup>,
    /// Content of the message to be sent instead of the file
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<crate::types::InputMessageContent>,
    /// URL of the thumbnail (JPEG only) for the file
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_url: Option<Box<str>>,
    /// Thumbnail width
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_width: Option<i64>,
    /// Thumbnail height
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_height: Option<i64>,
}
impl InlineQueryResultDocument {
    /// Creates a new `InlineQueryResultDocument`.
    ///
    /// # Arguments
    /// * `id` - Unique identifier for this result, 1-64 bytes
    /// * `title` - Title for the result
    /// * `document_url` - A valid URL for the file
    /// * `mime_type` - MIME type of the content of the file, either `application/pdf` or `application/zip`
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0: Into<Box<str>>, T1: Into<Box<str>>, T2: Into<Box<str>>, T3: Into<Box<str>>>(
        id: T0,
        title: T1,
        document_url: T2,
        mime_type: T3,
    ) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            caption: None,
            parse_mode: None,
            caption_entities: None,
            document_url: document_url.into(),
            mime_type: mime_type.into(),
            description: None,
            reply_markup: None,
            input_message_content: None,
            thumbnail_url: None,
            thumbnail_width: None,
            thumbnail_height: None,
        }
    }

    /// Unique identifier for this result, 1-64 bytes
    #[must_use]
    pub fn id<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.id = val.into();
        self
    }

    /// Title for the result
    #[must_use]
    pub fn title<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.title = val.into();
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

    /// A valid URL for the file
    #[must_use]
    pub fn document_url<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.document_url = val.into();
        self
    }

    /// MIME type of the content of the file, either `application/pdf` or `application/zip`
    #[must_use]
    pub fn mime_type<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.mime_type = val.into();
        self
    }

    /// Short description of the result
    #[must_use]
    pub fn description<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.description = Some(val.into());
        self
    }

    /// Short description of the result
    #[must_use]
    pub fn description_option<T: Into<Box<str>>>(mut self, val: Option<T>) -> Self {
        self.description = val.map(Into::into);
        self
    }

    /// Inline keyboard attached to the message
    #[must_use]
    pub fn reply_markup<T: Into<crate::types::InlineKeyboardMarkup>>(mut self, val: T) -> Self {
        self.reply_markup = Some(val.into());
        self
    }

    /// Inline keyboard attached to the message
    #[must_use]
    pub fn reply_markup_option<T: Into<crate::types::InlineKeyboardMarkup>>(
        mut self,
        val: Option<T>,
    ) -> Self {
        self.reply_markup = val.map(Into::into);
        self
    }

    /// Content of the message to be sent instead of the file
    #[must_use]
    pub fn input_message_content<T: Into<crate::types::InputMessageContent>>(
        mut self,
        val: T,
    ) -> Self {
        self.input_message_content = Some(val.into());
        self
    }

    /// Content of the message to be sent instead of the file
    #[must_use]
    pub fn input_message_content_option<T: Into<crate::types::InputMessageContent>>(
        mut self,
        val: Option<T>,
    ) -> Self {
        self.input_message_content = val.map(Into::into);
        self
    }

    /// URL of the thumbnail (JPEG only) for the file
    #[must_use]
    pub fn thumbnail_url<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.thumbnail_url = Some(val.into());
        self
    }

    /// URL of the thumbnail (JPEG only) for the file
    #[must_use]
    pub fn thumbnail_url_option<T: Into<Box<str>>>(mut self, val: Option<T>) -> Self {
        self.thumbnail_url = val.map(Into::into);
        self
    }

    /// Thumbnail width
    #[must_use]
    pub fn thumbnail_width<T: Into<i64>>(mut self, val: T) -> Self {
        self.thumbnail_width = Some(val.into());
        self
    }

    /// Thumbnail width
    #[must_use]
    pub fn thumbnail_width_option<T: Into<i64>>(mut self, val: Option<T>) -> Self {
        self.thumbnail_width = val.map(Into::into);
        self
    }

    /// Thumbnail height
    #[must_use]
    pub fn thumbnail_height<T: Into<i64>>(mut self, val: T) -> Self {
        self.thumbnail_height = Some(val.into());
        self
    }

    /// Thumbnail height
    #[must_use]
    pub fn thumbnail_height_option<T: Into<i64>>(mut self, val: Option<T>) -> Self {
        self.thumbnail_height = val.map(Into::into);
        self
    }
}
