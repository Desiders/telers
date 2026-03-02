use serde::{Deserialize, Serialize};
/// Represents a link to a page containing an embedded video player or a video file. By default, this video file will be sent by the user with an optional caption. Alternatively, you can use `input_message_content` to send a message with the specified content instead of the video.
/// # Documentation
/// <https://core.telegram.org/bots/api#inlinequeryresultvideo>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InlineQueryResultVideo {
    /// Unique identifier for this result, 1-64 bytes
    pub id: Box<str>,
    /// A valid URL for the embedded video player or video file
    pub video_url: Box<str>,
    /// MIME type of the content of the video URL, `text/html` or `video/mp4`
    pub mime_type: Box<str>,
    /// URL of the thumbnail (JPEG only) for the video
    pub thumbnail_url: Box<str>,
    /// Title for the result
    pub title: Box<str>,
    /// Caption of the video to be sent, 0-1024 characters after entities parsing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<Box<str>>,
    /// Mode for parsing entities in the video caption. See formatting options for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<Box<str>>,
    /// List of special entities that appear in the caption, which can be specified instead of `parse_mode`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Box<[crate::types::MessageEntity]>>,
    /// Pass `true`, if the caption must be shown above the message media
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_caption_above_media: Option<bool>,
    /// Video width
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_width: Option<i64>,
    /// Video height
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_height: Option<i64>,
    /// Video duration in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_duration: Option<i64>,
    /// Short description of the result
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Box<str>>,
    /// Inline keyboard attached to the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<crate::types::InlineKeyboardMarkup>,
    /// Content of the message to be sent instead of the video. This field is required if [`crate::types::InlineQueryResultVideo`] is used to send an HTML-page as a result (e.g., a `YouTube` video).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<crate::types::InputMessageContent>,
}
impl InlineQueryResultVideo {
    /// Creates a new `InlineQueryResultVideo`.
    ///
    /// # Arguments
    /// * `id` - Unique identifier for this result, 1-64 bytes
    /// * `video_url` - A valid URL for the embedded video player or video file
    /// * `mime_type` - MIME type of the content of the video URL, `text/html` or `video/mp4`
    /// * `thumbnail_url` - URL of the thumbnail (JPEG only) for the video
    /// * `title` - Title for the result
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<
        T0: Into<Box<str>>,
        T1: Into<Box<str>>,
        T2: Into<Box<str>>,
        T3: Into<Box<str>>,
        T4: Into<Box<str>>,
    >(
        id: T0,
        video_url: T1,
        mime_type: T2,
        thumbnail_url: T3,
        title: T4,
    ) -> Self {
        Self {
            id: id.into(),
            video_url: video_url.into(),
            mime_type: mime_type.into(),
            thumbnail_url: thumbnail_url.into(),
            title: title.into(),
            caption: None,
            parse_mode: None,
            caption_entities: None,
            show_caption_above_media: None,
            video_width: None,
            video_height: None,
            video_duration: None,
            description: None,
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

    /// A valid URL for the embedded video player or video file
    #[must_use]
    pub fn video_url<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.video_url = val.into();
        this
    }

    /// MIME type of the content of the video URL, `text/html` or `video/mp4`
    #[must_use]
    pub fn mime_type<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.mime_type = val.into();
        this
    }

    /// URL of the thumbnail (JPEG only) for the video
    #[must_use]
    pub fn thumbnail_url<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.thumbnail_url = val.into();
        this
    }

    /// Title for the result
    #[must_use]
    pub fn title<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.title = val.into();
        this
    }

    /// Caption of the video to be sent, 0-1024 characters after entities parsing
    #[must_use]
    pub fn caption<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.caption = Some(val.into());
        this
    }

    /// Caption of the video to be sent, 0-1024 characters after entities parsing
    #[must_use]
    pub fn caption_option<T: Into<Box<str>>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.caption = val.map(Into::into);
        this
    }

    /// Mode for parsing entities in the video caption. See formatting options for more details.
    #[must_use]
    pub fn parse_mode<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.parse_mode = Some(val.into());
        this
    }

    /// Mode for parsing entities in the video caption. See formatting options for more details.
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

    /// Video width
    #[must_use]
    pub fn video_width<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.video_width = Some(val.into());
        this
    }

    /// Video width
    #[must_use]
    pub fn video_width_option<T: Into<i64>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.video_width = val.map(Into::into);
        this
    }

    /// Video height
    #[must_use]
    pub fn video_height<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.video_height = Some(val.into());
        this
    }

    /// Video height
    #[must_use]
    pub fn video_height_option<T: Into<i64>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.video_height = val.map(Into::into);
        this
    }

    /// Video duration in seconds
    #[must_use]
    pub fn video_duration<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.video_duration = Some(val.into());
        this
    }

    /// Video duration in seconds
    #[must_use]
    pub fn video_duration_option<T: Into<i64>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.video_duration = val.map(Into::into);
        this
    }

    /// Short description of the result
    #[must_use]
    pub fn description<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.description = Some(val.into());
        this
    }

    /// Short description of the result
    #[must_use]
    pub fn description_option<T: Into<Box<str>>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.description = val.map(Into::into);
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

    /// Content of the message to be sent instead of the video. This field is required if [`crate::types::InlineQueryResultVideo`] is used to send an HTML-page as a result (e.g., a `YouTube` video).
    #[must_use]
    pub fn input_message_content<T: Into<crate::types::InputMessageContent>>(self, val: T) -> Self {
        let mut this = self;
        this.input_message_content = Some(val.into());
        this
    }

    /// Content of the message to be sent instead of the video. This field is required if [`crate::types::InlineQueryResultVideo`] is used to send an HTML-page as a result (e.g., a `YouTube` video).
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
