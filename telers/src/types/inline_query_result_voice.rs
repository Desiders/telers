use serde::{Deserialize, Serialize};
/// Represents a link to a voice recording in an .OGG container encoded with OPUS. By default, this voice recording will be sent by the user. Alternatively, you can use `input_message_content` to send a message with the specified content instead of the the voice message.
/// # Documentation
/// <https://core.telegram.org/bots/api#inlinequeryresultvoice>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InlineQueryResultVoice {
    /// Unique identifier for this result, 1-64 bytes
    pub id: Box<str>,
    /// A valid URL for the voice recording
    pub voice_url: Box<str>,
    /// Recording title
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
    /// Recording duration in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_duration: Option<i64>,
    /// Inline keyboard attached to the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<crate::types::InlineKeyboardMarkup>,
    /// Content of the message to be sent instead of the voice recording
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<crate::types::InputMessageContent>,
}
impl InlineQueryResultVoice {
    /// Creates a new `InlineQueryResultVoice`.
    ///
    /// # Arguments
    /// * `id` - Unique identifier for this result, 1-64 bytes
    /// * `voice_url` - A valid URL for the voice recording
    /// * `title` - Recording title
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0: Into<Box<str>>, T1: Into<Box<str>>, T2: Into<Box<str>>>(
        id: T0,
        voice_url: T1,
        title: T2,
    ) -> Self {
        Self {
            id: id.into(),
            voice_url: voice_url.into(),
            title: title.into(),
            caption: None,
            parse_mode: None,
            caption_entities: None,
            voice_duration: None,
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

    /// A valid URL for the voice recording
    #[must_use]
    pub fn voice_url<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.voice_url = val.into();
        self
    }

    /// Recording title
    #[must_use]
    pub fn title<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.title = val.into();
        self
    }

    /// Caption, 0-1024 characters after entities parsing
    #[must_use]
    pub fn caption<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.caption = Some(val.into());
        self
    }

    /// Caption, 0-1024 characters after entities parsing
    #[must_use]
    pub fn caption_option<T: Into<Box<str>>>(mut self, val: Option<T>) -> Self {
        self.caption = val.map(Into::into);
        self
    }

    /// Mode for parsing entities in the voice message caption. See formatting options for more details.
    #[must_use]
    pub fn parse_mode<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.parse_mode = Some(val.into());
        self
    }

    /// Mode for parsing entities in the voice message caption. See formatting options for more details.
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

    /// Recording duration in seconds
    #[must_use]
    pub fn voice_duration<T: Into<i64>>(mut self, val: T) -> Self {
        self.voice_duration = Some(val.into());
        self
    }

    /// Recording duration in seconds
    #[must_use]
    pub fn voice_duration_option<T: Into<i64>>(mut self, val: Option<T>) -> Self {
        self.voice_duration = val.map(Into::into);
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

    /// Content of the message to be sent instead of the voice recording
    #[must_use]
    pub fn input_message_content<T: Into<crate::types::InputMessageContent>>(
        mut self,
        val: T,
    ) -> Self {
        self.input_message_content = Some(val.into());
        self
    }

    /// Content of the message to be sent instead of the voice recording
    #[must_use]
    pub fn input_message_content_option<T: Into<crate::types::InputMessageContent>>(
        mut self,
        val: Option<T>,
    ) -> Self {
        self.input_message_content = val.map(Into::into);
        self
    }
}
