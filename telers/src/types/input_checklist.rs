use crate::types::{InputChecklistTask, MessageEntity};

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Describes a checklist to create.
/// # Documentation
/// <https://core.telegram.org/bots/api#inputchecklist>
#[skip_serializing_none]
#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub struct InputChecklist {
    /// Title of the checklist; 1-255 characters after entities parsing
    pub title: String,
    /// Mode for parsing entities in the title. See [formatting options](https://core.telegram.org/bots/api#formatting-options) for more details.
    pub parse_mode: Option<String>,
    /// List of special entities that appear in the title, which can be specified instead of `parse_mode`. Currently, only `bold`, `italic`, `underline`, `strikethrough`, `spoiler`, and `custom_emoji` entities are allowed.
    pub text_entities: Option<Vec<MessageEntity>>,
    /// List of 1-30 tasks in the checklist
    pub tasks: Vec<InputChecklistTask>,
    /// Pass `true` if other users can add tasks to the checklist
    pub others_can_add_tasks: Option<bool>,
    /// Pass `true` if other users can mark tasks as done or not done in the checklist
    pub others_can_mark_tasks_as_done: Option<bool>,
}

impl InputChecklist {
    #[must_use]
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            parse_mode: None,
            text_entities: None,
            tasks: vec![],
            others_can_add_tasks: None,
            others_can_mark_tasks_as_done: None,
        }
    }

    #[must_use]
    pub fn title(self, val: impl Into<String>) -> Self {
        Self {
            title: val.into(),
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

    #[must_use]
    pub fn task(self, val: InputChecklistTask) -> Self {
        Self {
            tasks: self.tasks.into_iter().chain(Some(val)).collect(),
            ..self
        }
    }

    #[must_use]
    pub fn tasks(self, val: impl IntoIterator<Item = InputChecklistTask>) -> Self {
        Self {
            tasks: self.tasks.into_iter().chain(val).collect(),
            ..self
        }
    }

    #[must_use]
    pub fn others_can_add_tasks(self, val: bool) -> Self {
        Self {
            others_can_add_tasks: Some(val),
            ..self
        }
    }

    #[must_use]
    pub fn others_can_mark_tasks_as_done(self, val: bool) -> Self {
        Self {
            others_can_mark_tasks_as_done: Some(val),
            ..self
        }
    }
}

impl InputChecklist {
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

    #[must_use]
    pub fn others_can_add_tasks_option(self, val: Option<bool>) -> Self {
        Self {
            others_can_add_tasks: val,
            ..self
        }
    }

    #[must_use]
    pub fn others_can_mark_tasks_as_done_option(self, val: Option<bool>) -> Self {
        Self {
            others_can_mark_tasks_as_done: val,
            ..self
        }
    }
}
