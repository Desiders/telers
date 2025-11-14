use super::base::{prepare_input_story_content, Request, TelegramMethod};

use crate::{
    client::Bot,
    types::{InputStoryContent, MessageEntity, Story, StoryArea},
};

use serde::Serialize;
use serde_with::skip_serializing_none;

/// Posts a story on behalf of a managed business account. Requires the `can_manage_stories` business bot right.
/// # Documentation
/// <https://core.telegram.org/bots/api#poststory>
/// # Returns
/// Returns [`Story`] on success
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct PostStory<'a> {
    /// Unique identifier of the business connection
    pub business_connection_id: String,
    /// Content of the story
    pub content: InputStoryContent<'a>,
    /// Period after which the story is moved to the archive, in seconds; must be one of `6 * 3600`, `12 * 3600`, `86400`, or `2 * 86400`
    pub active_period: u32,
    /// Caption of the story, 0-2048 characters after entities parsing
    pub caption: Option<String>,
    /// Mode for parsing entities in the story caption. See [`formatting options`](https://core.telegram.org/bots/api#formatting-options) for more details.
    pub parse_mode: Option<String>,
    /// A JSON-serialized list of special entities that appear in the caption, which can be specified instead of `parse_mode`
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// A JSON-serialized list of clickable areas to be shown on the story
    pub areas: Option<Vec<StoryArea>>,
    /// Pass `true` to keep the story accessible after it expires
    pub post_to_chat_page: Option<bool>,
    /// Pass `true` if the content of the story must be protected from forwarding and screenshotting
    pub protect_content: Option<bool>,
}

impl<'a> PostStory<'a> {
    #[must_use]
    pub fn new(
        business_connection_id: impl Into<String>,
        content: impl Into<InputStoryContent<'a>>,
        active_period: impl Into<u32>,
    ) -> Self {
        Self {
            business_connection_id: business_connection_id.into(),
            content: content.into(),
            active_period: active_period.into(),
            caption: None,
            parse_mode: None,
            caption_entities: None,
            areas: None,
            post_to_chat_page: None,
            protect_content: None,
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
    pub fn content(self, val: impl Into<InputStoryContent<'a>>) -> Self {
        Self {
            content: val.into(),
            ..self
        }
    }

    #[must_use]
    pub fn active_period(self, val: impl Into<u32>) -> Self {
        Self {
            active_period: val.into(),
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

    #[must_use]
    pub fn post_to_chat_page(self, val: bool) -> Self {
        Self {
            post_to_chat_page: Some(val),
            ..self
        }
    }

    #[must_use]
    pub fn protect_content(self, val: bool) -> Self {
        Self {
            protect_content: Some(val),
            ..self
        }
    }
}

impl PostStory<'_> {
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

    #[must_use]
    pub fn post_to_chat_page_option(self, val: Option<bool>) -> Self {
        Self {
            post_to_chat_page: val,
            ..self
        }
    }

    #[must_use]
    pub fn protect_content_option(self, val: Option<bool>) -> Self {
        Self {
            protect_content: val,
            ..self
        }
    }
}

impl TelegramMethod for PostStory<'_> {
    type Method = Self;
    type Return = Story;

    fn build_request<Client>(&self, _bot: &Bot<Client>) -> Request<'_, Self::Method> {
        let mut files = vec![];
        prepare_input_story_content(&mut files, &self.content);

        Request::new("postStory", self, Some(files.into()))
    }
}

impl<'a> AsRef<PostStory<'a>> for PostStory<'a> {
    fn as_ref(&self) -> &Self {
        self
    }
}
