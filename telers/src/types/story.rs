use serde::{Deserialize, Serialize};
/// This object represents a story.
/// # Documentation
/// <https://core.telegram.org/bots/api#story>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Story {
    /// Chat that posted the story
    pub chat: Box<crate::types::Chat>,
    /// Unique identifier for the story in the chat
    pub id: i64,
}
impl Story {
    /// Creates a new `Story`.
    ///
    /// # Arguments
    /// * `chat` - Chat that posted the story
    /// * `id` - Unique identifier for the story in the chat
    #[must_use]
    pub fn new<T0: Into<crate::types::Chat>, T1: Into<i64>>(chat: T0, id: T1) -> Self {
        Self {
            chat: Box::new(chat.into()),
            id: id.into(),
        }
    }

    /// Chat that posted the story
    #[must_use]
    pub fn chat<T: Into<crate::types::Chat>>(self, val: T) -> Self {
        let mut this = self;
        this.chat = Box::new(val.into());
        this
    }

    /// Unique identifier for the story in the chat
    #[must_use]
    pub fn id<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.id = val.into();
        this
    }
}
