use serde::{Deserialize, Serialize};
/// Represents a link to a video animation (H.264/MPEG-4 AVC video without sound). By default, this animated MPEG-4 file will be sent by the user with optional caption. Alternatively, you can use `input_message_content` to send a message with the specified content instead of the animation.
/// # Documentation
/// <https://core.telegram.org/bots/api#inlinequeryresultmpeg4gif>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InlineQueryResultMpeg4Gif {
    /// Unique identifier for this result, 1-64 bytes
    pub id: Box<str>,
    /// A valid URL for the MPEG4 file
    pub mpeg4_url: Box<str>,
    /// Video width
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mpeg4_width: Option<i64>,
    /// Video height
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mpeg4_height: Option<i64>,
    /// Video duration in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mpeg4_duration: Option<i64>,
    /// URL of the static (JPEG or GIF) or animated (MPEG4) thumbnail for the result
    pub thumbnail_url: Box<str>,
    /// MIME type of the thumbnail, must be one of `image/jpeg`, `image/gif`, or `video/mp4`. Defaults to `image/jpeg`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_mime_type: Option<Box<str>>,
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
    /// Pass `true`, if the caption must be shown above the message media
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_caption_above_media: Option<bool>,
    /// Inline keyboard attached to the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<crate::types::InlineKeyboardMarkup>,
    /// Content of the message to be sent instead of the video animation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<crate::types::InputMessageContent>,
}
impl InlineQueryResultMpeg4Gif {
    /// Creates a new `InlineQueryResultMpeg4Gif`.
    ///
    /// # Arguments
    /// * `id` - Unique identifier for this result, 1-64 bytes
    /// * `mpeg4_url` - A valid URL for the MPEG4 file
    /// * `thumbnail_url` - URL of the static (JPEG or GIF) or animated (MPEG4) thumbnail for the result
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0: Into<Box<str>>, T1: Into<Box<str>>, T2: Into<Box<str>>>(
        id: T0,
        mpeg4_url: T1,
        thumbnail_url: T2,
    ) -> Self {
        Self {
            id: id.into(),
            mpeg4_url: mpeg4_url.into(),
            mpeg4_width: None,
            mpeg4_height: None,
            mpeg4_duration: None,
            thumbnail_url: thumbnail_url.into(),
            thumbnail_mime_type: None,
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
    pub fn id<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.id = val.into();
        this
    }

    /// A valid URL for the MPEG4 file
    #[must_use]
    pub fn mpeg4_url<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.mpeg4_url = val.into();
        this
    }

    /// Video width
    #[must_use]
    pub fn mpeg4_width<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.mpeg4_width = Some(val.into());
        this
    }

    /// Video width
    #[must_use]
    pub fn mpeg4_width_option<T: Into<i64>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.mpeg4_width = val.map(Into::into);
        this
    }

    /// Video height
    #[must_use]
    pub fn mpeg4_height<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.mpeg4_height = Some(val.into());
        this
    }

    /// Video height
    #[must_use]
    pub fn mpeg4_height_option<T: Into<i64>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.mpeg4_height = val.map(Into::into);
        this
    }

    /// Video duration in seconds
    #[must_use]
    pub fn mpeg4_duration<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.mpeg4_duration = Some(val.into());
        this
    }

    /// Video duration in seconds
    #[must_use]
    pub fn mpeg4_duration_option<T: Into<i64>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.mpeg4_duration = val.map(Into::into);
        this
    }

    /// URL of the static (JPEG or GIF) or animated (MPEG4) thumbnail for the result
    #[must_use]
    pub fn thumbnail_url<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.thumbnail_url = val.into();
        this
    }

    /// MIME type of the thumbnail, must be one of `image/jpeg`, `image/gif`, or `video/mp4`. Defaults to `image/jpeg`
    #[must_use]
    pub fn thumbnail_mime_type<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.thumbnail_mime_type = Some(val.into());
        this
    }

    /// MIME type of the thumbnail, must be one of `image/jpeg`, `image/gif`, or `video/mp4`. Defaults to `image/jpeg`
    #[must_use]
    pub fn thumbnail_mime_type_option<T: Into<Box<str>>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.thumbnail_mime_type = val.map(Into::into);
        this
    }

    /// Title for the result
    #[must_use]
    pub fn title<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.title = Some(val.into());
        this
    }

    /// Title for the result
    #[must_use]
    pub fn title_option<T: Into<Box<str>>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.title = val.map(Into::into);
        this
    }

    /// Caption of the MPEG-4 file to be sent, 0-1024 characters after entities parsing
    #[must_use]
    pub fn caption<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.caption = Some(val.into());
        this
    }

    /// Caption of the MPEG-4 file to be sent, 0-1024 characters after entities parsing
    #[must_use]
    pub fn caption_option<T: Into<Box<str>>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.caption = val.map(Into::into);
        this
    }

    /// Mode for parsing entities in the caption. See formatting options for more details.
    #[must_use]
    pub fn parse_mode<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.parse_mode = Some(val.into());
        this
    }

    /// Mode for parsing entities in the caption. See formatting options for more details.
    #[must_use]
    pub fn parse_mode_option<T: Into<Box<str>>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.parse_mode = val.map(Into::into);
        this
    }

    /// List of special entities that appear in the caption, which can be specified instead of `parse_mode`
    ///
    /// # Notes
    /// Adds multiple elements.
    #[must_use]
    pub fn caption_entities<T: Into<Box<[crate::types::MessageEntity]>>>(self, val: T) -> Self {
        let mut this = self;
        this.caption_entities = Some(
            this.caption_entities
                .unwrap_or_default()
                .into_vec()
                .into_iter()
                .chain(val.into())
                .collect(),
        );
        this
    }

    /// List of special entities that appear in the caption, which can be specified instead of `parse_mode`
    ///
    /// # Notes
    /// Adds a single element.
    #[must_use]
    pub fn caption_entity<T: Into<crate::types::MessageEntity>>(self, val: T) -> Self {
        let mut this = self;
        this.caption_entities = Some(
            this.caption_entities
                .unwrap_or_default()
                .into_vec()
                .into_iter()
                .chain(Some(val.into()))
                .collect(),
        );
        this
    }

    /// List of special entities that appear in the caption, which can be specified instead of `parse_mode`
    ///
    /// # Notes
    /// Adds a single element.
    #[must_use]
    pub fn caption_entities_option<T: Into<Box<[crate::types::MessageEntity]>>>(
        self,
        val: Option<T>,
    ) -> Self {
        let mut this = self;
        this.caption_entities = val.map(Into::into);
        this
    }

    /// Pass `true`, if the caption must be shown above the message media
    #[must_use]
    pub fn show_caption_above_media<T: Into<bool>>(self, val: T) -> Self {
        let mut this = self;
        this.show_caption_above_media = Some(val.into());
        this
    }

    /// Pass `true`, if the caption must be shown above the message media
    #[must_use]
    pub fn show_caption_above_media_option<T: Into<bool>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.show_caption_above_media = val.map(Into::into);
        this
    }

    /// Inline keyboard attached to the message
    #[must_use]
    pub fn reply_markup<T: Into<crate::types::InlineKeyboardMarkup>>(self, val: T) -> Self {
        let mut this = self;
        this.reply_markup = Some(val.into());
        this
    }

    /// Inline keyboard attached to the message
    #[must_use]
    pub fn reply_markup_option<T: Into<crate::types::InlineKeyboardMarkup>>(
        self,
        val: Option<T>,
    ) -> Self {
        let mut this = self;
        this.reply_markup = val.map(Into::into);
        this
    }

    /// Content of the message to be sent instead of the video animation
    #[must_use]
    pub fn input_message_content<T: Into<crate::types::InputMessageContent>>(self, val: T) -> Self {
        let mut this = self;
        this.input_message_content = Some(val.into());
        this
    }

    /// Content of the message to be sent instead of the video animation
    #[must_use]
    pub fn input_message_content_option<T: Into<crate::types::InputMessageContent>>(
        self,
        val: Option<T>,
    ) -> Self {
        let mut this = self;
        this.input_message_content = val.map(Into::into);
        this
    }
}
