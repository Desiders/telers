use serde::{Deserialize, Serialize};
/// This object contains information about the quoted part of a message that is replied to by the given message.
/// # Documentation
/// <https://core.telegram.org/bots/api#textquote>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TextQuote {
    /// Text of the quoted part of a message that is replied to by the given message
    pub text: Box<str>,
    /// Special entities that appear in the quote. Currently, only bold, italic, underline, strikethrough, spoiler, `custom_emoji`, and `date_time` entities are kept in quotes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entities: Option<Box<[crate::types::MessageEntity]>>,
    /// Approximate quote position in the original message in UTF-16 code units as specified by the sender
    pub position: i64,
    /// `true`, if the quote was chosen manually by the message sender. Otherwise, the quote was added automatically by the server.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_manual: Option<bool>,
}
impl TextQuote {
    /// Creates a new `TextQuote`.
    ///
    /// # Arguments
    /// * `text` - Text of the quoted part of a message that is replied to by the given message
    /// * `position` - Approximate quote position in the original message in UTF-16 code units as specified by the sender
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0: Into<Box<str>>, T1: Into<i64>>(text: T0, position: T1) -> Self {
        Self {
            text: text.into(),
            entities: None,
            position: position.into(),
            is_manual: None,
        }
    }

    /// Text of the quoted part of a message that is replied to by the given message
    #[must_use]
    pub fn text<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.text = val.into();
        self
    }

    /// Special entities that appear in the quote. Currently, only bold, italic, underline, strikethrough, spoiler, `custom_emoji`, and `date_time` entities are kept in quotes.
    ///
    /// # Notes
    /// Adds multiple elements.
    #[must_use]
    pub fn entities<T: Into<Box<[crate::types::MessageEntity]>>>(mut self, val: T) -> Self {
        self.entities = Some(
            self.entities
                .unwrap_or_default()
                .into_vec()
                .into_iter()
                .chain(val.into())
                .collect(),
        );
        self
    }

    /// Special entities that appear in the quote. Currently, only bold, italic, underline, strikethrough, spoiler, `custom_emoji`, and `date_time` entities are kept in quotes.
    ///
    /// # Notes
    /// Adds a single element.
    #[must_use]
    pub fn entity<T: Into<crate::types::MessageEntity>>(mut self, val: T) -> Self {
        self.entities = Some(
            self.entities
                .unwrap_or_default()
                .into_vec()
                .into_iter()
                .chain(Some(val.into()))
                .collect(),
        );
        self
    }

    /// Special entities that appear in the quote. Currently, only bold, italic, underline, strikethrough, spoiler, `custom_emoji`, and `date_time` entities are kept in quotes.
    ///
    /// # Notes
    /// Adds a single element.
    #[must_use]
    pub fn entities_option<T: Into<Box<[crate::types::MessageEntity]>>>(
        mut self,
        val: Option<T>,
    ) -> Self {
        self.entities = val.map(Into::into);
        self
    }

    /// Approximate quote position in the original message in UTF-16 code units as specified by the sender
    #[must_use]
    pub fn position<T: Into<i64>>(mut self, val: T) -> Self {
        self.position = val.into();
        self
    }

    /// `true`, if the quote was chosen manually by the message sender. Otherwise, the quote was added automatically by the server.
    #[must_use]
    pub fn is_manual<T: Into<bool>>(mut self, val: T) -> Self {
        self.is_manual = Some(val.into());
        self
    }

    /// `true`, if the quote was chosen manually by the message sender. Otherwise, the quote was added automatically by the server.
    #[must_use]
    pub fn is_manual_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.is_manual = val.map(Into::into);
        self
    }
}
