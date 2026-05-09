use serde::{Deserialize, Serialize};
/// The reaction is based on a custom emoji.
/// # Documentation
/// <https://core.telegram.org/bots/api#reactiontypecustomemoji>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ReactionTypeCustomEmoji {
    /// Custom emoji identifier
    pub custom_emoji_id: Box<str>,
}
impl ReactionTypeCustomEmoji {
    /// Creates a new `ReactionTypeCustomEmoji`.
    ///
    /// # Arguments
    /// * `custom_emoji_id` - Custom emoji identifier
    #[must_use]
    pub fn new<T0: Into<Box<str>>>(custom_emoji_id: T0) -> Self {
        Self {
            custom_emoji_id: custom_emoji_id.into(),
        }
    }

    /// Custom emoji identifier
    #[must_use]
    pub fn custom_emoji_id<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.custom_emoji_id = val.into();
        self
    }
}
