use serde::{Deserialize, Serialize};
/// Represents a link to a voice message stored on the Telegram servers. By default, this voice message will be sent by the user. Alternatively, you can use `input_message_content` to send a message with the specified content instead of the voice message.
/// # Documentation
/// <https://core.telegram.org/bots/api#inlinequeryresultcachedvoice>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InlineQueryResultCachedVoice {
    /// Unique identifier for this result, 1-64 bytes
    pub id: Box<str>,
    /// A valid file identifier for the voice message
    pub voice_file_id: Box<str>,
    /// Voice message title
    pub title: Box<str>,
    /// Caption, 0-1024 characters after entities parsing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<Box<str>>,
    /// Mode for parsing entities in the voice message caption. See formatting options for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<Box<str>>,
    /// List of special entities that appear in the caption, which can be specified instead of `parse_mode`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Box<[crate::types::MessageEntity]>>,
    /// Inline keyboard attached to the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<crate::types::InlineKeyboardMarkup>,
    /// Content of the message to be sent instead of the voice message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<crate::types::InputMessageContent>,
}
impl InlineQueryResultCachedVoice {
    /// Creates a new `InlineQueryResultCachedVoice`.
    ///
    /// # Arguments
    /// * `id` - Unique identifier for this result, 1-64 bytes
    /// * `voice_file_id` - A valid file identifier for the voice message
    /// * `title` - Voice message title
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0: Into<Box<str>>, T1: Into<Box<str>>, T2: Into<Box<str>>>(
        id: T0,
        voice_file_id: T1,
        title: T2,
    ) -> Self {
        Self {
            id: id.into(),
            voice_file_id: voice_file_id.into(),
            title: title.into(),
            caption: None,
            parse_mode: None,
            caption_entities: None,
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

    /// A valid file identifier for the voice message
    #[must_use]
    pub fn voice_file_id<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.voice_file_id = val.into();
        this
    }

    /// Voice message title
    #[must_use]
    pub fn title<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.title = val.into();
        this
    }

    /// Caption, 0-1024 characters after entities parsing
    #[must_use]
    pub fn caption<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.caption = Some(val.into());
        this
    }

    /// Caption, 0-1024 characters after entities parsing
    #[must_use]
    pub fn caption_option<T: Into<Box<str>>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.caption = val.map(Into::into);
        this
    }

    /// Mode for parsing entities in the voice message caption. See formatting options for more details.
    #[must_use]
    pub fn parse_mode<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.parse_mode = Some(val.into());
        this
    }

    /// Mode for parsing entities in the voice message caption. See formatting options for more details.
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

    /// Content of the message to be sent instead of the voice message
    #[must_use]
    pub fn input_message_content<T: Into<crate::types::InputMessageContent>>(self, val: T) -> Self {
        let mut this = self;
        this.input_message_content = Some(val.into());
        this
    }

    /// Content of the message to be sent instead of the voice message
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
