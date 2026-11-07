/// Represents a link to an article or web page.
/// # Documentation
/// <https://core.telegram.org/bots/api#inlinequeryresultarticle>
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deser", derive(deser::Serialize, deser::Deserialize))]
#[cfg_attr(not(feature = "deser"), derive(serde::Serialize))]
pub struct InlineQueryResultArticle {
    /// Unique identifier for this result, 1-64 Bytes
    pub id: Box<str>,
    /// Title of the result
    pub title: Box<str>,
    /// Content of the message to be sent
    pub input_message_content: crate::types::InputMessageContent,
    /// Inline keyboard attached to the message
    #[cfg_attr(not(feature = "deser"), serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "deser", deser(skip_serializing_if = Option::is_none))]
    pub reply_markup: Option<crate::types::InlineKeyboardMarkup>,
    /// URL of the result
    #[cfg_attr(not(feature = "deser"), serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "deser", deser(skip_serializing_if = Option::is_none))]
    pub url: Option<Box<str>>,
    /// Short description of the result
    #[cfg_attr(not(feature = "deser"), serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "deser", deser(skip_serializing_if = Option::is_none))]
    pub description: Option<Box<str>>,
    /// Url of the thumbnail for the result
    #[cfg_attr(not(feature = "deser"), serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "deser", deser(skip_serializing_if = Option::is_none))]
    pub thumbnail_url: Option<Box<str>>,
    /// Thumbnail width
    #[cfg_attr(not(feature = "deser"), serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "deser", deser(skip_serializing_if = Option::is_none))]
    pub thumbnail_width: Option<i64>,
    /// Thumbnail height
    #[cfg_attr(not(feature = "deser"), serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "deser", deser(skip_serializing_if = Option::is_none))]
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
    pub fn id<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.id = val.into();
        self
    }

    /// Title of the result
    #[must_use]
    pub fn title<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.title = val.into();
        self
    }

    /// Content of the message to be sent
    #[must_use]
    pub fn input_message_content<T: Into<crate::types::InputMessageContent>>(
        mut self,
        val: T,
    ) -> Self {
        self.input_message_content = val.into();
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

    /// URL of the result
    #[must_use]
    pub fn url<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.url = Some(val.into());
        self
    }

    /// URL of the result
    #[must_use]
    pub fn url_option<T: Into<Box<str>>>(mut self, val: Option<T>) -> Self {
        self.url = val.map(Into::into);
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

    /// Url of the thumbnail for the result
    #[must_use]
    pub fn thumbnail_url<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.thumbnail_url = Some(val.into());
        self
    }

    /// Url of the thumbnail for the result
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
#[cfg(not(feature = "deser"))]
impl<'de> serde::Deserialize<'de> for InlineQueryResultArticle {
    #[allow(clippy::too_many_lines)]
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let meta = crate::deserialization::Meta {
            name: "InlineQueryResultArticle",
            fields: &[
                "id",
                "title",
                "input_message_content",
                "reply_markup",
                "url",
                "description",
                "thumbnail_url",
                "thumbnail_width",
                "thumbnail_height",
            ],
            specific: &[
                "title",
                "input_message_content",
                "url",
                "description",
                "thumbnail_url",
                "thumbnail_width",
                "thumbnail_height",
            ],
            sequence: "struct InlineQueryResultArticle with 9 elements",
        };
        let (
            common,
            (
                value_title,
                value_input_message_content,
                value_url,
                value_description,
                value_thumbnail_url,
                value_thumbnail_width,
                value_thumbnail_height,
            ),
        ) = crate::deserialization::inline_query_result::read_0::<
            D,
            Box<str>,
            crate::types::InputMessageContent,
            Option<Box<str>>,
            Option<Box<str>>,
            Option<Box<str>>,
            Option<i64>,
            Option<i64>,
        >(deserializer, meta)?;
        Ok(Self {
            id: common.id,
            title: value_title,
            input_message_content: value_input_message_content,
            reply_markup: common.reply_markup,
            url: value_url,
            description: value_description,
            thumbnail_url: value_thumbnail_url,
            thumbnail_width: value_thumbnail_width,
            thumbnail_height: value_thumbnail_height,
        })
    }
}
