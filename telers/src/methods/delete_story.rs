use super::base::{Request, TelegramMethod};

use crate::client::Bot;

use serde::Serialize;

/// Deletes a story previously posted by the bot on behalf of a managed business account. Requires the `can_manage_stories` business bot right.
/// # Documentation
/// <https://core.telegram.org/bots/api#deletestory>
/// # Returns
/// On success, `true` is returned
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize)]
pub struct DeleteStory {
    /// Unique identifier of the business connection
    pub business_connection_id: String,
    /// Unique identifier of the story to delete
    pub story_id: i64,
}

impl DeleteStory {
    #[must_use]
    pub fn new(business_connection_id: impl Into<String>, story_id: i64) -> Self {
        Self {
            business_connection_id: business_connection_id.into(),
            story_id,
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
    pub fn story_id(self, val: i64) -> Self {
        Self {
            story_id: val,
            ..self
        }
    }
}

impl TelegramMethod for DeleteStory {
    type Method = Self;
    type Return = bool;

    fn build_request<Client>(&self, _bot: &Bot<Client>) -> Request<'_, Self::Method> {
        Request::new("deleteStory", self, None)
    }
}

impl AsRef<DeleteStory> for DeleteStory {
    fn as_ref(&self) -> &Self {
        self
    }
}
