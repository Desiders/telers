use serde::{Deserialize, Serialize};
/// Represents a link to an article or web page.
/// # Documentation
/// <https://core.telegram.org/bots/api#inlinequeryresultarticle>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InlineQueryResultArticle {
    /// Unique identifier for this result, 1-64 Bytes
    pub id: Box<str>,
    /// Title of the result
    pub title: Box<str>,
    /// Content of the message to be sent
    pub input_message_content: crate::types::InputMessageContent,
    /// Inline keyboard attached to the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<crate::types::InlineKeyboardMarkup>,
    /// URL of the result
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<Box<str>>,
    /// Short description of the result
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Box<str>>,
    /// Url of the thumbnail for the result
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_url: Option<Box<str>>,
    /// Thumbnail width
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_width: Option<i64>,
    /// Thumbnail height
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_height: Option<i64>,
}
impl InlineQueryResultArticle {
    /// Creates a new `InlineQueryResultArticle`.
    ///
    /// # Arguments
    /// * `id` - Unique identifier for this result, 1-64 Bytes
    /// * `title` - Title of the result
    /// * `input_message_content` - Content of the message to be sent
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<
        T0: Into<Box<str>>,
        T1: Into<Box<str>>,
        T2: Into<crate::types::InputMessageContent>,
    >(
        id: T0,
        title: T1,
        input_message_content: T2,
    ) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            input_message_content: input_message_content.into(),
            reply_markup: None,
            url: None,
            description: None,
            thumbnail_url: None,
            thumbnail_width: None,
            thumbnail_height: None,
        }
    }

    /// Unique identifier for this result, 1-64 Bytes
    #[must_use]
    pub fn id<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.id = val.into();
        this
    }

    /// Title of the result
    #[must_use]
    pub fn title<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.title = val.into();
        this
    }

    /// Content of the message to be sent
    #[must_use]
    pub fn input_message_content<T: Into<crate::types::InputMessageContent>>(self, val: T) -> Self {
        let mut this = self;
        this.input_message_content = val.into();
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

    /// URL of the result
    #[must_use]
    pub fn url<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.url = Some(val.into());
        this
    }

    /// URL of the result
    #[must_use]
    pub fn url_option<T: Into<Box<str>>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.url = val.map(Into::into);
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

    /// Url of the thumbnail for the result
    #[must_use]
    pub fn thumbnail_url<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.thumbnail_url = Some(val.into());
        this
    }

    /// Url of the thumbnail for the result
    #[must_use]
    pub fn thumbnail_url_option<T: Into<Box<str>>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.thumbnail_url = val.map(Into::into);
        this
    }

    /// Thumbnail width
    #[must_use]
    pub fn thumbnail_width<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.thumbnail_width = Some(val.into());
        this
    }

    /// Thumbnail width
    #[must_use]
    pub fn thumbnail_width_option<T: Into<i64>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.thumbnail_width = val.map(Into::into);
        this
    }

    /// Thumbnail height
    #[must_use]
    pub fn thumbnail_height<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.thumbnail_height = Some(val.into());
        this
    }

    /// Thumbnail height
    #[must_use]
    pub fn thumbnail_height_option<T: Into<i64>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.thumbnail_height = val.map(Into::into);
        this
    }
}
