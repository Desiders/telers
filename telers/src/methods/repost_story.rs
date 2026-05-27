use crate::client::Bot;
use serde::Serialize;
/// Reposts a story on behalf of a business account from another business account. Both business accounts must be managed by the same bot, and the story on the source account must have been posted (or reposted) by the bot. Requires the `can_manage_stories` business bot right for both business accounts. Returns Story on success.
/// # Documentation
/// <https://core.telegram.org/bots/api#repoststory>
/// # Returns
/// - `crate::types::Story`
#[derive(Clone, Debug, Serialize)]
pub struct RepostStory {
    /// Unique identifier of the business connection
    pub business_connection_id: Box<str>,
    /// Unique identifier of the chat which posted the story that should be reposted
    pub from_chat_id: i64,
    /// Unique identifier of the story that should be reposted
    pub from_story_id: i64,
    /// Period after which the story is moved to the archive, in seconds; must be one of 6 * 3600, 12 * 3600, 86400, or 2 * 86400
    pub active_period: i64,
    /// Pass `true` to keep the story accessible after it expires
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_to_chat_page: Option<bool>,
    /// Pass `true` if the content of the story must be protected from forwarding and screenshotting
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,
}
impl RepostStory {
    /// Creates a new `RepostStory`.
    ///
    /// # Arguments
    /// * `business_connection_id` - Unique identifier of the business connection
    /// * `from_chat_id` - Unique identifier of the chat which posted the story that should be reposted
    /// * `from_story_id` - Unique identifier of the story that should be reposted
    /// * `active_period` - Period after which the story is moved to the archive, in seconds; must be one of 6 * 3600, 12 * 3600, 86400, or 2 * 86400
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0: Into<Box<str>>, T1: Into<i64>, T2: Into<i64>, T3: Into<i64>>(
        business_connection_id: T0,
        from_chat_id: T1,
        from_story_id: T2,
        active_period: T3,
    ) -> Self {
        Self {
            business_connection_id: business_connection_id.into(),
            from_chat_id: from_chat_id.into(),
            from_story_id: from_story_id.into(),
            active_period: active_period.into(),
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

    /// Unique identifier of the chat which posted the story that should be reposted
    #[must_use]
    pub fn from_chat_id<T: Into<i64>>(mut self, val: T) -> Self {
        self.from_chat_id = val.into();
        self
    }

    /// Unique identifier of the story that should be reposted
    #[must_use]
    pub fn from_story_id<T: Into<i64>>(mut self, val: T) -> Self {
        self.from_story_id = val.into();
        self
    }

    /// Period after which the story is moved to the archive, in seconds; must be one of 6 * 3600, 12 * 3600, 86400, or 2 * 86400
    #[must_use]
    pub fn active_period<T: Into<i64>>(mut self, val: T) -> Self {
        self.active_period = val.into();
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
impl super::TelegramMethod for RepostStory {
    type Method = Self;
    type Return = crate::types::Story;

    fn build_request<Client>(self, _bot: &Bot<Client>) -> super::Request<Self::Method> {
        super::Request::new("repostStory", self, None)
    }
}
