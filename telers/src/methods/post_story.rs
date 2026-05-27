use crate::client::Bot;
use serde::Serialize;
/// Posts a story on behalf of a managed business account. Requires the `can_manage_stories` business bot right. Returns Story on success.
/// # Documentation
/// <https://core.telegram.org/bots/api#poststory>
/// # Returns
/// - `crate::types::Story`
#[derive(Clone, Debug, Serialize)]
pub struct PostStory {
    /// Unique identifier of the business connection
    pub business_connection_id: Box<str>,
    /// Content of the story
    pub content: crate::types::InputStoryContent,
    /// Period after which the story is moved to the archive, in seconds; must be one of 6 * 3600, 12 * 3600, 86400, or 2 * 86400
    pub active_period: i64,
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
    /// Pass `true` to keep the story accessible after it expires
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_to_chat_page: Option<bool>,
    /// Pass `true` if the content of the story must be protected from forwarding and screenshotting
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,
}
impl PostStory {
    /// Creates a new `PostStory`.
    ///
    /// # Arguments
    /// * `business_connection_id` - Unique identifier of the business connection
    /// * `content` - Content of the story
    /// * `active_period` - Period after which the story is moved to the archive, in seconds; must be one of 6 * 3600, 12 * 3600, 86400, or 2 * 86400
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0: Into<Box<str>>, T1: Into<crate::types::InputStoryContent>, T2: Into<i64>>(
        business_connection_id: T0,
        content: T1,
        active_period: T2,
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

    /// Unique identifier of the business connection
    #[must_use]
    pub fn business_connection_id<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.business_connection_id = val.into();
        self
    }

    /// Content of the story
    #[must_use]
    pub fn content<T: Into<crate::types::InputStoryContent>>(mut self, val: T) -> Self {
        self.content = val.into();
        self
    }

    /// Period after which the story is moved to the archive, in seconds; must be one of 6 * 3600, 12 * 3600, 86400, or 2 * 86400
    #[must_use]
    pub fn active_period<T: Into<i64>>(mut self, val: T) -> Self {
        self.active_period = val.into();
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

    /// Pass `true` to keep the story accessible after it expires
    #[must_use]
    pub fn post_to_chat_page<T: Into<bool>>(mut self, val: T) -> Self {
        self.post_to_chat_page = Some(val.into());
        self
    }

    /// Pass `true` to keep the story accessible after it expires
    #[must_use]
    pub fn post_to_chat_page_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.post_to_chat_page = val.map(Into::into);
        self
    }

    /// Pass `true` if the content of the story must be protected from forwarding and screenshotting
    #[must_use]
    pub fn protect_content<T: Into<bool>>(mut self, val: T) -> Self {
        self.protect_content = Some(val.into());
        self
    }

    /// Pass `true` if the content of the story must be protected from forwarding and screenshotting
    #[must_use]
    pub fn protect_content_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.protect_content = val.map(Into::into);
        self
    }
}
impl super::TelegramMethod for PostStory {
    type Method = Self;
    type Return = crate::types::Story;

    fn build_request<Client>(mut self, _bot: &Bot<Client>) -> super::Request<Self::Method> {
        let mut files = vec![];
        super::prepare_input_story_content(&mut files, &mut self.content);
        super::Request::new("postStory", self, Some(files))
    }
}
