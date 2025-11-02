use super::base::{prepare_input_story_content, Request, TelegramMethod};

use crate::{
    client::Bot,
    types::{InputStoryContent, MessageEntity, Story, StoryArea},
};

use serde::Serialize;
use serde_with::skip_serializing_none;

/// Edits a story previously posted by the bot on behalf of a managed business account. Requires the `can_manage_stories` business bot right.
/// # Documentation
/// <https://core.telegram.org/bots/api#editstory>
/// # Returns
/// Returns [`Story`] on success
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct EditStory<'a> {
    /// Unique identifier of the business connection
    pub business_connection_id: String,
    /// Unique identifier of the story to edit
    pub story_id: String,
    /// Content of the story
    pub content: InputStoryContent<'a>,
    /// Caption of the story, 0-2048 characters after entities parsing
    pub caption: Option<String>,
    /// Mode for parsing entities in the story caption. See [`formatting options`](https://core.telegram.org/bots/api#formatting-options) for more details.
    pub parse_mode: Option<String>,
    /// A JSON-serialized list of special entities that appear in the caption, which can be specified instead of `parse_mode`
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// A JSON-serialized list of clickable areas to be shown on the story
    pub areas: Option<Vec<StoryArea>>,
}

impl<'a> EditStory<'a> {
    #[must_use]
    pub fn new(
        business_connection_id: impl Into<String>,
        story_id: impl Into<String>,
        content: impl Into<InputStoryContent<'a>>,
    ) -> Self {
        Self {
            business_connection_id: business_connection_id.into(),
            story_id: story_id.into(),
            content: content.into(),
            caption: None,
            parse_mode: None,
            caption_entities: None,
            areas: None,
        }
    }

    #[must_use]
    pub fn business_connection_id(self, val: impl Into<String>) -> Self {
        Self {
            business_connection_id: val.into(),
            ..self
        }
    }

    #[must_use]
    pub fn story_id(self, val: impl Into<String>) -> Self {
        Self {
            story_id: val.into(),
            ..self
        }
    }

    #[must_use]
    pub fn content(self, val: impl Into<InputStoryContent<'a>>) -> Self {
        Self {
            content: val.into(),
            ..self
        }
    }

    #[must_use]
    pub fn caption(self, val: impl Into<String>) -> Self {
        Self {
            caption: Some(val.into()),
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
    pub fn caption_entity(self, val: MessageEntity) -> Self {
        Self {
            caption_entities: Some(
                self.caption_entities
                    .unwrap_or_default()
                    .into_iter()
                    .chain(Some(val))
                    .collect(),
            ),
            ..self
        }
    }

    #[must_use]
    pub fn caption_entities(self, val: impl IntoIterator<Item = MessageEntity>) -> Self {
        Self {
            caption_entities: Some(
                self.caption_entities
                    .unwrap_or_default()
                    .into_iter()
                    .chain(val)
                    .collect(),
            ),
            ..self
        }
    }

    #[must_use]
    pub fn area(self, val: StoryArea) -> Self {
        Self {
            areas: Some(
                self.areas
                    .unwrap_or_default()
                    .into_iter()
                    .chain(Some(val))
                    .collect(),
            ),
            ..self
        }
    }

    #[must_use]
    pub fn areas(self, val: impl IntoIterator<Item = StoryArea>) -> Self {
        Self {
            areas: Some(
                self.areas
                    .unwrap_or_default()
                    .into_iter()
                    .chain(val)
                    .collect(),
            ),
            ..self
        }
    }
}

impl EditStory<'_> {
    #[must_use]
    pub fn caption_option(self, val: Option<impl Into<String>>) -> Self {
        Self {
            caption: val.map(Into::into),
            ..self
        }
    }

    #[must_use]
    pub fn parse_mode_option(self, val: Option<impl Into<String>>) -> Self {
        Self {
            parse_mode: val.map(Into::into),
            ..self
        }
    }

    #[must_use]
    pub fn caption_entities_option(
        self,
        val: Option<impl IntoIterator<Item = MessageEntity>>,
    ) -> Self {
        Self {
            caption_entities: val.map(|val| {
                self.caption_entities
                    .unwrap_or_default()
                    .into_iter()
                    .chain(val)
                    .collect()
            }),
            ..self
        }
    }

    #[must_use]
    pub fn areas_option(self, val: Option<impl IntoIterator<Item = StoryArea>>) -> Self {
        Self {
            areas: val.map(|val| {
                self.areas
                    .unwrap_or_default()
                    .into_iter()
                    .chain(val)
                    .collect()
            }),
            ..self
        }
    }
}

impl TelegramMethod for EditStory<'_> {
    type Method = Self;
    type Return = Story;

    fn build_request<Client>(&self, _bot: &Bot<Client>) -> Request<Self::Method> {
        let mut files = vec![];
        prepare_input_story_content(&mut files, &self.content);

        Request::new("editStory", self, Some(files.into()))
    }
}

impl<'a> AsRef<EditStory<'a>> for EditStory<'a> {
    fn as_ref(&self) -> &Self {
        self
    }
}
