use serde::{Deserialize, Serialize};

/// The reaction is based on an emoji.
/// # Documentation
/// <https://core.telegram.org/bots/api#reactiontypeemoji>
#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub struct ReactionTypeEmoji {
    /// Emoji on which the reaction is based.
    pub emoji: String,
}

impl ReactionTypeEmoji {
    #[must_use]
    pub fn new(emoji: impl Into<String>) -> Self {
        Self {
            emoji: emoji.into(),
        }
    }

    #[must_use]
    pub fn emoji(self, val: impl Into<String>) -> Self {
        Self { emoji: val.into() }
    }
}
