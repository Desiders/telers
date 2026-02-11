use crate::types::MessageEntity;

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Describes a task to add to a checklist.
/// # Documentation
/// <https://core.telegram.org/bots/api#inputchecklisttask>
#[skip_serializing_none]
#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub struct InputChecklistTask {
    /// Unique identifier of the task; must be positive and unique among all task identifiers currently present in the checklist
    pub id: i64,
    /// Text of the task; 1-100 characters after entities parsing
    pub text: String,
    /// Mode for parsing entities in the text. See [formatting options](https://core.telegram.org/bots/api#formatting-options) for more details.
    pub parse_mode: Option<String>,
    /// List of special entities that appear in the text, which can be specified instead of `parse_mode`. Currently, only `bold`, `italic`, `underline`, `strikethrough`, `spoiler`, and `custom_emoji` entities are allowed.
    pub text_entities: Option<Vec<MessageEntity>>,
}

impl InputChecklistTask {
    #[must_use]
    pub fn new(id: i64, text: impl Into<String>) -> Self {
        Self {
            id,
            text: text.into(),
            parse_mode: None,
            text_entities: None,
        }
    }

    #[must_use]
    pub fn id(self, val: i64) -> Self {
        Self { id: val, ..self }
    }

    #[must_use]
    pub fn text(self, val: impl Into<String>) -> Self {
        Self {
            text: val.into(),
            ..self
        }
    }

    #[must_use]
    pub fn parse_mode(self, val: impl Into<String>) -> Self {
        Self {
            parse_mode: Some(val.into()),
            ..self
        }
    }

    #[must_use]
    pub fn text_entity(self, val: MessageEntity) -> Self {
        Self {
            text_entities: Some(
                self.text_entities
                    .unwrap_or_default()
                    .into_iter()
                    .chain(Some(val))
                    .collect(),
            ),
            ..self
        }
    }

    #[must_use]
    pub fn text_entities(self, val: impl IntoIterator<Item = MessageEntity>) -> Self {
        Self {
            text_entities: Some(
                self.text_entities
                    .unwrap_or_default()
                    .into_iter()
                    .chain(val)
                    .collect(),
            ),
            ..self
        }
    }
}

impl InputChecklistTask {
    #[must_use]
    pub fn parse_mode_option(self, val: Option<impl Into<String>>) -> Self {
        Self {
            parse_mode: val.map(Into::into),
            ..self
        }
    }

    #[must_use]
    pub fn text_entities_option(
        self,
        val: Option<impl IntoIterator<Item = MessageEntity>>,
    ) -> Self {
        Self {
            text_entities: val.map(|val| {
                self.text_entities
                    .unwrap_or_default()
                    .into_iter()
                    .chain(val)
                    .collect()
            }),
            ..self
        }
    }
}
