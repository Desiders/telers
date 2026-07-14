use serde::{Deserialize, Serialize};
/// Represents a link to a video animation (H.264/MPEG-4 AVC video without sound) stored on the Telegram servers. By default, this animated MPEG-4 file will be sent by the user with an optional caption. Alternatively, you can use `input_message_content` to send a message with the specified content instead of the animation.
/// # Documentation
/// <https://core.telegram.org/bots/api#inlinequeryresultcachedmpeg4gif>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InlineQueryResultCachedMpeg4Gif {
    /// Unique identifier for this result, 1-64 bytes
    pub id: Box<str>,
    /// A valid file identifier for the MPEG4 file
    pub mpeg4_file_id: Box<str>,
    /// Title for the result
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<Box<str>>,
    /// Caption of the MPEG-4 file to be sent, 0-1024 characters after entities parsing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<Box<str>>,
    /// Mode for parsing entities in the caption. See formatting options for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<Box<str>>,
    /// List of special entities that appear in the caption, which can be specified instead of `parse_mode`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Box<[crate::types::MessageEntity]>>,
    /// Pass `true` if the caption must be shown above the message media
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_caption_above_media: Option<bool>,
    /// Inline keyboard attached to the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<crate::types::InlineKeyboardMarkup>,
    /// Content of the message to be sent instead of the video animation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<crate::types::InputMessageContent>,
}
impl InlineQueryResultCachedMpeg4Gif {
    /// Creates a new `InlineQueryResultCachedMpeg4Gif`.
    ///
    /// # Arguments
    /// * `id` - Unique identifier for this result, 1-64 bytes
    /// * `mpeg4_file_id` - A valid file identifier for the MPEG4 file
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0: Into<Box<str>>, T1: Into<Box<str>>>(id: T0, mpeg4_file_id: T1) -> Self {
        Self {
            id: id.into(),
            mpeg4_file_id: mpeg4_file_id.into(),
            title: None,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            show_caption_above_media: None,
            reply_markup: None,
            input_message_content: None,
        }
    }

    /// Unique identifier for this result, 1-64 bytes
    #[must_use]
    pub fn id<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.id = val.into();
        self
    }

    /// A valid file identifier for the MPEG4 file
    #[must_use]
    pub fn mpeg4_file_id<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.mpeg4_file_id = val.into();
        self
    }

    /// Title for the result
    #[must_use]
    pub fn title<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.title = Some(val.into());
        self
    }

    /// Title for the result
    #[must_use]
    pub fn title_option<T: Into<Box<str>>>(mut self, val: Option<T>) -> Self {
        self.title = val.map(Into::into);
        self
    }

    /// Caption of the MPEG-4 file to be sent, 0-1024 characters after entities parsing
    #[must_use]
    pub fn caption<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.caption = Some(val.into());
        self
    }

    /// Caption of the MPEG-4 file to be sent, 0-1024 characters after entities parsing
    #[must_use]
    pub fn caption_option<T: Into<Box<str>>>(mut self, val: Option<T>) -> Self {
        self.caption = val.map(Into::into);
        self
    }

    /// Mode for parsing entities in the caption. See formatting options for more details.
    #[must_use]
    pub fn parse_mode<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.parse_mode = Some(val.into());
        self
    }

    /// Mode for parsing entities in the caption. See formatting options for more details.
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

    /// Content of the message to be sent instead of the video animation
    #[must_use]
    pub fn input_message_content<T: Into<crate::types::InputMessageContent>>(
        mut self,
        val: T,
    ) -> Self {
        self.input_message_content = Some(val.into());
        self
    }

    /// Content of the message to be sent instead of the video animation
    #[must_use]
    pub fn input_message_content_option<T: Into<crate::types::InputMessageContent>>(
        mut self,
        val: Option<T>,
    ) -> Self {
        self.input_message_content = val.map(Into::into);
        self
    }
}
