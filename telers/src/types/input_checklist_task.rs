use serde::{Deserialize, Serialize};
/// Describes a task to add to a checklist.
/// # Documentation
/// <https://core.telegram.org/bots/api#inputchecklisttask>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InputChecklistTask {
    /// Unique identifier of the task; must be positive and unique among all task identifiers currently present in the checklist
    pub id: i64,
    /// Text of the task; 1-100 characters after entities parsing
    pub text: Box<str>,
    /// Mode for parsing entities in the text. See formatting options for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<Box<str>>,
    /// List of special entities that appear in the text, which can be specified instead of `parse_mode`. Currently, only bold, italic, underline, strikethrough, spoiler, `custom_emoji`, and `date_time` entities are allowed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_entities: Option<Box<[crate::types::MessageEntity]>>,
}
impl InputChecklistTask {
    /// Creates a new `InputChecklistTask`.
    ///
    /// # Arguments
    /// * `id` - Unique identifier of the task; must be positive and unique among all task identifiers currently present in the checklist
    /// * `text` - Text of the task; 1-100 characters after entities parsing
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0: Into<i64>, T1: Into<Box<str>>>(id: T0, text: T1) -> Self {
        Self {
            id: id.into(),
            text: text.into(),
            parse_mode: None,
            text_entities: None,
        }
    }

    /// Unique identifier of the task; must be positive and unique among all task identifiers currently present in the checklist
    #[must_use]
    pub fn id<T: Into<i64>>(mut self, val: T) -> Self {
        self.id = val.into();
        self
    }

    /// Text of the task; 1-100 characters after entities parsing
    #[must_use]
    pub fn text<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.text = val.into();
        self
    }

    /// Mode for parsing entities in the text. See formatting options for more details.
    #[must_use]
    pub fn parse_mode<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.parse_mode = Some(val.into());
        self
    }

    /// Mode for parsing entities in the text. See formatting options for more details.
    #[must_use]
    pub fn parse_mode_option<T: Into<Box<str>>>(mut self, val: Option<T>) -> Self {
        self.parse_mode = val.map(Into::into);
        self
    }

    /// List of special entities that appear in the text, which can be specified instead of `parse_mode`. Currently, only bold, italic, underline, strikethrough, spoiler, `custom_emoji`, and `date_time` entities are allowed.
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

    /// List of special entities that appear in the text, which can be specified instead of `parse_mode`. Currently, only bold, italic, underline, strikethrough, spoiler, `custom_emoji`, and `date_time` entities are allowed.
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

    /// List of special entities that appear in the text, which can be specified instead of `parse_mode`. Currently, only bold, italic, underline, strikethrough, spoiler, `custom_emoji`, and `date_time` entities are allowed.
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
}
