use serde::{Deserialize, Serialize};
/// This object contains information about one answer option in a poll to be sent.
/// # Documentation
/// <https://core.telegram.org/bots/api#inputpolloption>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InputPollOption {
    /// Option text, 1-100 characters
    pub text: Box<str>,
    /// Mode for parsing entities in the text. See formatting options for more details. Currently, only custom emoji entities are allowed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_parse_mode: Option<Box<str>>,
    /// A JSON-serialized list of special entities that appear in the poll option text. It can be specified instead of `text_parse_mode`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_entities: Option<Box<[crate::types::MessageEntity]>>,
    /// Media added to the poll option
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media: Option<crate::types::InputPollOptionMedia>,
}
impl InputPollOption {
    /// Creates a new `InputPollOption`.
    ///
    /// # Arguments
    /// * `text` - Option text, 1-100 characters
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0: Into<Box<str>>>(text: T0) -> Self {
        Self {
            text: text.into(),
            text_parse_mode: None,
            text_entities: None,
            media: None,
        }
    }

    /// Option text, 1-100 characters
    #[must_use]
    pub fn text<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.text = val.into();
        self
    }

    /// Mode for parsing entities in the text. See formatting options for more details. Currently, only custom emoji entities are allowed.
    #[must_use]
    pub fn text_parse_mode<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.text_parse_mode = Some(val.into());
        self
    }

    /// Mode for parsing entities in the text. See formatting options for more details. Currently, only custom emoji entities are allowed.
    #[must_use]
    pub fn text_parse_mode_option<T: Into<Box<str>>>(mut self, val: Option<T>) -> Self {
        self.text_parse_mode = val.map(Into::into);
        self
    }

    /// A JSON-serialized list of special entities that appear in the poll option text. It can be specified instead of `text_parse_mode`.
    ///
    /// # Notes
    /// Adds multiple elements.
    #[must_use]
    pub fn text_entities<T: Into<Box<[crate::types::MessageEntity]>>>(mut self, val: T) -> Self {
        self.text_entities = Some(
            self.text_entities
                .unwrap_or_default()
                .into_vec()
                .into_iter()
                .chain(val.into())
                .collect(),
        );
        self
    }

    /// A JSON-serialized list of special entities that appear in the poll option text. It can be specified instead of `text_parse_mode`.
    ///
    /// # Notes
    /// Adds a single element.
    #[must_use]
    pub fn text_entity<T: Into<crate::types::MessageEntity>>(mut self, val: T) -> Self {
        self.text_entities = Some(
            self.text_entities
                .unwrap_or_default()
                .into_vec()
                .into_iter()
                .chain(Some(val.into()))
                .collect(),
        );
        self
    }

    /// A JSON-serialized list of special entities that appear in the poll option text. It can be specified instead of `text_parse_mode`.
    ///
    /// # Notes
    /// Adds a single element.
    #[must_use]
    pub fn text_entities_option<T: Into<Box<[crate::types::MessageEntity]>>>(
        mut self,
        val: Option<T>,
    ) -> Self {
        self.text_entities = val.map(Into::into);
        self
    }

    /// Media added to the poll option
    #[must_use]
    pub fn media<T: Into<crate::types::InputPollOptionMedia>>(mut self, val: T) -> Self {
        self.media = Some(val.into());
        self
    }

    /// Media added to the poll option
    #[must_use]
    pub fn media_option<T: Into<crate::types::InputPollOptionMedia>>(
        mut self,
        val: Option<T>,
    ) -> Self {
        self.media = val.map(Into::into);
        self
    }
}
