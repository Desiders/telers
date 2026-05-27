use crate::client::Bot;
use serde::Serialize;
/// Deletes a story previously posted by the bot on behalf of a managed business account. Requires the `can_manage_stories` business bot right. Returns `true` on success.
/// # Documentation
/// <https://core.telegram.org/bots/api#deletestory>
/// # Returns
/// - `bool`
#[derive(Clone, Debug, Serialize)]
pub struct DeleteStory {
    /// Unique identifier of the business connection
    pub business_connection_id: Box<str>,
    /// Unique identifier of the story to delete
    pub story_id: i64,
}
impl DeleteStory {
    /// Creates a new `DeleteStory`.
    ///
    /// # Arguments
    /// * `business_connection_id` - Unique identifier of the business connection
    /// * `story_id` - Unique identifier of the story to delete
    #[must_use]
    pub fn new<T0: Into<Box<str>>, T1: Into<i64>>(
        business_connection_id: T0,
        story_id: T1,
    ) -> Self {
        Self {
            business_connection_id: business_connection_id.into(),
            story_id: story_id.into(),
        }
    }

    /// Unique identifier of the business connection
    #[must_use]
    pub fn business_connection_id<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.business_connection_id = val.into();
        self
    }

    /// Unique identifier of the story to delete
    #[must_use]
    pub fn story_id<T: Into<i64>>(mut self, val: T) -> Self {
        self.story_id = val.into();
        self
    }
}
impl super::TelegramMethod for DeleteStory {
    type Method = Self;
    type Return = bool;

    fn build_request<Client>(self, _bot: &Bot<Client>) -> super::Request<Self::Method> {
        super::Request::new("deleteStory", self, None)
    }
}
