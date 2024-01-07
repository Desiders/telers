use super::{ReactionTypeCustomEmoji, ReactionTypeEmoji};

use serde::{Deserialize, Serialize};

/// This object describes the type of a reaction. Currently, it can be one of
/// - [`ReactionTypeEmoji`]
/// - [`ReactionTypeCustomEmoji`]
/// # Documentation
/// <https://core.telegram.org/bots/api#reactiontype>
#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ReactionType {
    Emoji(ReactionTypeEmoji),
    CustomEmoji(ReactionTypeCustomEmoji),
}

impl ReactionType {
    #[must_use]
    pub fn emoji(emoji: impl Into<String>) -> Self {
        Self::Emoji(ReactionTypeEmoji::new(emoji))
    }

    #[must_use]
    pub fn custom_emoji(custom_emoji: impl Into<String>) -> Self {
        Self::CustomEmoji(ReactionTypeCustomEmoji::new(custom_emoji))
    }
}

impl From<ReactionTypeEmoji> for ReactionType {
    #[must_use]
    fn from(emoji: ReactionTypeEmoji) -> Self {
        Self::Emoji(emoji)
    }
}

impl From<ReactionTypeCustomEmoji> for ReactionType {
    #[must_use]
    fn from(custom_emoji: ReactionTypeCustomEmoji) -> Self {
        Self::CustomEmoji(custom_emoji)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_emoji() {
        let data = ReactionType::emoji("👍");
        let json = serde_json::to_string(&data).unwrap();

        assert_eq!(json, r#"{"type":"emoji","emoji":"👍"}"#);
    }

    #[test]
    fn serialize_custom_emoji() {
        let data = ReactionType::custom_emoji("123");
        let json = serde_json::to_string(&data).unwrap();

        assert_eq!(json, r#"{"type":"custom_emoji","custom_emoji":"123"}"#);
    }

    #[test]
    fn deserialize_emoji() {
        let data = r#"{"type":"emoji","emoji":"👍"}"#;
        let emoji: ReactionType = serde_json::from_str(data).unwrap();

        assert_eq!(emoji, ReactionType::emoji("👍"));
    }

    #[test]
    fn deserialize_custom_emoji() {
        let data = r#"{"type":"custom_emoji","custom_emoji":"123"}"#;
        let custom_emoji: ReactionType = serde_json::from_str(data).unwrap();

        assert_eq!(custom_emoji, ReactionType::custom_emoji("123"));
    }
}
