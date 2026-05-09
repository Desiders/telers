use crate::client::Bot;
use serde::Serialize;
/// Edits a story previously posted by the bot on behalf of a managed business account. Requires the `can_manage_stories` business bot right. Returns Story on success.
/// # Documentation
/// <https://core.telegram.org/bots/api#editstory>
/// # Returns
/// - `crate::types::Story`
#[derive(Clone, Debug, Serialize)]
pub struct EditStory {
    /// Unique identifier of the business connection
    pub business_connection_id: Box<str>,
    /// Unique identifier of the story to edit
    pub story_id: i64,
    /// Content of the story
    pub content: crate::types::InputStoryContent,
    /// Caption of the story, 0-2048 characters after entities parsing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<Box<str>>,
    /// Mode for parsing entities in the story caption. See formatting options for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<Box<str>>,
    /// A JSON-serialized list of special entities that appear in the caption, which can be specified instead of `parse_mode`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Box<[crate::types::MessageEntity]>>,
    /// A JSON-serialized list of clickable areas to be shown on the story
    #[serde(skip_serializing_if = "Option::is_none")]
    pub areas: Option<Box<[crate::types::StoryArea]>>,
}
impl EditStory {
    /// Creates a new `EditStory`.
    ///
    /// # Arguments
    /// * `business_connection_id` - Unique identifier of the business connection
    /// * `story_id` - Unique identifier of the story to edit
    /// * `content` - Content of the story
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0: Into<Box<str>>, T1: Into<i64>, T2: Into<crate::types::InputStoryContent>>(
        business_connection_id: T0,
        story_id: T1,
        content: T2,
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

    /// Unique identifier of the business connection
    #[must_use]
    pub fn business_connection_id<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.business_connection_id = val.into();
        self
    }

    /// Unique identifier of the story to edit
    #[must_use]
    pub fn story_id<T: Into<i64>>(mut self, val: T) -> Self {
        self.story_id = val.into();
        self
    }

    /// Content of the story
    #[must_use]
    pub fn content<T: Into<crate::types::InputStoryContent>>(mut self, val: T) -> Self {
        self.content = val.into();
        self
    }

    /// Caption of the story, 0-2048 characters after entities parsing
    #[must_use]
    pub fn caption<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.caption = Some(val.into());
        self
    }

    /// Caption of the story, 0-2048 characters after entities parsing
    #[must_use]
    pub fn caption_option<T: Into<Box<str>>>(mut self, val: Option<T>) -> Self {
        self.caption = val.map(Into::into);
        self
    }

    /// Mode for parsing entities in the story caption. See formatting options for more details.
    #[must_use]
    pub fn parse_mode<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.parse_mode = Some(val.into());
        self
    }

    /// Mode for parsing entities in the story caption. See formatting options for more details.
    #[must_use]
    pub fn parse_mode_option<T: Into<Box<str>>>(mut self, val: Option<T>) -> Self {
        self.parse_mode = val.map(Into::into);
        self
    }

    /// A JSON-serialized list of special entities that appear in the caption, which can be specified instead of `parse_mode`
    ///
    /// # Notes
    /// Adds multiple elements.
    #[must_use]
    pub fn caption_entities<
        TItem: Into<crate::types::MessageEntity>,
        T: IntoIterator<Item = TItem>,
    >(
        mut self,
        val: T,
    ) -> Self {
        self.caption_entities = Some(
            self.caption_entities
                .unwrap_or_default()
                .into_vec()
                .into_iter()
                .chain(val.into_iter().map(Into::into))
                .collect(),
        );
        self
    }

    /// A JSON-serialized list of special entities that appear in the caption, which can be specified instead of `parse_mode`
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

    /// A JSON-serialized list of special entities that appear in the caption, which can be specified instead of `parse_mode`
    ///
    /// # Notes
    /// Adds multiple elements.
    #[must_use]
    pub fn caption_entities_option<
        TItem: Into<crate::types::MessageEntity>,
        T: IntoIterator<Item = TItem>,
    >(
        mut self,
        val: Option<T>,
    ) -> Self {
        self.caption_entities = val.map(|v| v.into_iter().map(Into::into).collect());
        self
    }

    /// A JSON-serialized list of clickable areas to be shown on the story
    ///
    /// # Notes
    /// Adds multiple elements.
    #[must_use]
    pub fn areas<TItem: Into<crate::types::StoryArea>, T: IntoIterator<Item = TItem>>(
        mut self,
        val: T,
    ) -> Self {
        self.areas = Some(
            self.areas
                .unwrap_or_default()
                .into_vec()
                .into_iter()
                .chain(val.into_iter().map(Into::into))
                .collect(),
        );
        self
    }

    /// A JSON-serialized list of clickable areas to be shown on the story
    ///
    /// # Notes
    /// Adds a single element.
    #[must_use]
    pub fn area<T: Into<crate::types::StoryArea>>(mut self, val: T) -> Self {
        self.areas = Some(
            self.areas
                .unwrap_or_default()
                .into_vec()
                .into_iter()
                .chain(Some(val.into()))
                .collect(),
        );
        self
    }

    /// A JSON-serialized list of clickable areas to be shown on the story
    ///
    /// # Notes
    /// Adds multiple elements.
    #[must_use]
    pub fn areas_option<TItem: Into<crate::types::StoryArea>, T: IntoIterator<Item = TItem>>(
        mut self,
        val: Option<T>,
    ) -> Self {
        self.areas = val.map(|v| v.into_iter().map(Into::into).collect());
        self
    }
}
impl super::TelegramMethod for EditStory {
    type Method = Self;
    type Return = crate::types::Story;

    fn build_request<Client>(mut self, _bot: &Bot<Client>) -> super::Request<Self::Method> {
        let mut files = vec![];
        super::prepare_input_story_content(&mut files, &mut self.content);
        super::Request::new("editStory", self, Some(files))
    }
}
