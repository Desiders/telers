use serde::{Deserialize, Serialize};
/// Describes a rich message to be sent. Exactly one of the fields html, markdown, or blocks must be used.
/// # Documentation
/// <https://core.telegram.org/bots/api#inputrichmessage>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InputRichMessage {
    /// Content of the rich message to send described as a list of blocks
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocks: Option<Box<[crate::types::InputRichBlock]>>,
    /// Content of the rich message to send described using HTML formatting. See rich message formatting options for more details. Use media field to specify the media used in the message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html: Option<Box<str>>,
    /// Content of the rich message to send described using Markdown formatting. See rich message formatting options for more details. Use media field to specify the media used in the message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub markdown: Option<Box<str>>,
    /// List of media that are specified in the markdown or html fields using tg://photo?id=, tg://video?id=, and tg://audio?id= links
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media: Option<Box<[crate::types::InputRichMessageMedia]>>,
    /// Pass `true` if the rich message must be shown right-to-left
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_rtl: Option<bool>,
    /// Pass `true` to skip automatic detection of entities (e.g., URLs, email addresses, username mentions, hashtags, cashtags, bot commands, or phone numbers) in the text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_entity_detection: Option<bool>,
}
impl InputRichMessage {
    /// Creates a new `InputRichMessage`.
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new() -> Self {
        Self {
            blocks: None,
            html: None,
            markdown: None,
            media: None,
            is_rtl: None,
            skip_entity_detection: None,
        }
    }

    /// Content of the rich message to send described as a list of blocks
    ///
    /// # Notes
    /// Adds multiple elements.
    #[must_use]
    pub fn blocks<T: Into<Box<[crate::types::InputRichBlock]>>>(mut self, val: T) -> Self {
        self.blocks = Some(
            self.blocks
                .unwrap_or_default()
                .into_vec()
                .into_iter()
                .chain(val.into())
                .collect(),
        );
        self
    }

    /// Content of the rich message to send described as a list of blocks
    ///
    /// # Notes
    /// Adds a single element.
    #[must_use]
    pub fn block<T: Into<crate::types::InputRichBlock>>(mut self, val: T) -> Self {
        self.blocks = Some(
            self.blocks
                .unwrap_or_default()
                .into_vec()
                .into_iter()
                .chain(Some(val.into()))
                .collect(),
        );
        self
    }

    /// Content of the rich message to send described as a list of blocks
    ///
    /// # Notes
    /// Adds a single element.
    #[must_use]
    pub fn blocks_option<T: Into<Box<[crate::types::InputRichBlock]>>>(
        mut self,
        val: Option<T>,
    ) -> Self {
        self.blocks = val.map(Into::into);
        self
    }

    /// Content of the rich message to send described using HTML formatting. See rich message formatting options for more details. Use media field to specify the media used in the message.
    #[must_use]
    pub fn html<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.html = Some(val.into());
        self
    }

    /// Content of the rich message to send described using HTML formatting. See rich message formatting options for more details. Use media field to specify the media used in the message.
    #[must_use]
    pub fn html_option<T: Into<Box<str>>>(mut self, val: Option<T>) -> Self {
        self.html = val.map(Into::into);
        self
    }

    /// Content of the rich message to send described using Markdown formatting. See rich message formatting options for more details. Use media field to specify the media used in the message.
    #[must_use]
    pub fn markdown<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.markdown = Some(val.into());
        self
    }

    /// Content of the rich message to send described using Markdown formatting. See rich message formatting options for more details. Use media field to specify the media used in the message.
    #[must_use]
    pub fn markdown_option<T: Into<Box<str>>>(mut self, val: Option<T>) -> Self {
        self.markdown = val.map(Into::into);
        self
    }

    /// List of media that are specified in the markdown or html fields using tg://photo?id=, tg://video?id=, and tg://audio?id= links
    ///
    /// # Notes
    /// Adds multiple elements.
    #[must_use]
    pub fn media<T: Into<Box<[crate::types::InputRichMessageMedia]>>>(mut self, val: T) -> Self {
        self.media = Some(
            self.media
                .unwrap_or_default()
                .into_vec()
                .into_iter()
                .chain(val.into())
                .collect(),
        );
        self
    }

    /// List of media that are specified in the markdown or html fields using tg://photo?id=, tg://video?id=, and tg://audio?id= links
    ///
    /// # Notes
    /// Adds a single element.
    #[must_use]
    pub fn media_option<T: Into<Box<[crate::types::InputRichMessageMedia]>>>(
        mut self,
        val: Option<T>,
    ) -> Self {
        self.media = val.map(Into::into);
        self
    }

    /// Pass `true` if the rich message must be shown right-to-left
    #[must_use]
    pub fn is_rtl<T: Into<bool>>(mut self, val: T) -> Self {
        self.is_rtl = Some(val.into());
        self
    }

    /// Pass `true` if the rich message must be shown right-to-left
    #[must_use]
    pub fn is_rtl_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.is_rtl = val.map(Into::into);
        self
    }

    /// Pass `true` to skip automatic detection of entities (e.g., URLs, email addresses, username mentions, hashtags, cashtags, bot commands, or phone numbers) in the text
    #[must_use]
    pub fn skip_entity_detection<T: Into<bool>>(mut self, val: T) -> Self {
        self.skip_entity_detection = Some(val.into());
        self
    }

    /// Pass `true` to skip automatic detection of entities (e.g., URLs, email addresses, username mentions, hashtags, cashtags, bot commands, or phone numbers) in the text
    #[must_use]
    pub fn skip_entity_detection_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.skip_entity_detection = val.map(Into::into);
        self
    }
}
impl Default for InputRichMessage {
    fn default() -> Self {
        Self::new()
    }
}
