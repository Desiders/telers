use serde::{Deserialize, Serialize};
/// Describes a rich message to be sent. Exactly one of the fields html or markdown must be used.
/// # Documentation
/// <https://core.telegram.org/bots/api#inputrichmessage>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InputRichMessage {
    /// Content of the rich message to send described using HTML formatting. See rich message formatting options for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html: Option<Box<str>>,
    /// Content of the rich message to send described using Markdown formatting. See rich message formatting options for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub markdown: Option<Box<str>>,
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
            html: None,
            markdown: None,
            is_rtl: None,
            skip_entity_detection: None,
        }
    }

    /// Content of the rich message to send described using HTML formatting. See rich message formatting options for more details.
    #[must_use]
    pub fn html<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.html = Some(val.into());
        self
    }

    /// Content of the rich message to send described using HTML formatting. See rich message formatting options for more details.
    #[must_use]
    pub fn html_option<T: Into<Box<str>>>(mut self, val: Option<T>) -> Self {
        self.html = val.map(Into::into);
        self
    }

    /// Content of the rich message to send described using Markdown formatting. See rich message formatting options for more details.
    #[must_use]
    pub fn markdown<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.markdown = Some(val.into());
        self
    }

    /// Content of the rich message to send described using Markdown formatting. See rich message formatting options for more details.
    #[must_use]
    pub fn markdown_option<T: Into<Box<str>>>(mut self, val: Option<T>) -> Self {
        self.markdown = val.map(Into::into);
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
