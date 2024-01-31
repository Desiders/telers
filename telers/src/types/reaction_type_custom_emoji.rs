use serde::{Deserialize, Serialize};

/// The reaction is based on a custom emoji.
/// # Documentation
/// <https://core.telegram.org/bots/api#reactiontypecustomemoji>
#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub struct ReactionTypeCustomEmoji {
    /// Custom emoji identifier
    pub custom_emoji: String,
}

impl ReactionTypeCustomEmoji {
    #[must_use]
    pub fn new(custom_emoji: impl Into<String>) -> Self {
        Self {
            custom_emoji: custom_emoji.into(),
        }
    }

    #[must_use]
    pub fn emoji(self, val: impl Into<String>) -> Self {
        Self {
            custom_emoji: val.into(),
        }
    }
}
