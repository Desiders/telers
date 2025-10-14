use super::{ReactionTypeCustomEmoji, ReactionTypeEmoji, ReactionTypePaid};

use serde::{Deserialize, Serialize};

/// This object describes the type of a reaction. Currently, it can be one of
/// - [`ReactionTypeEmoji`]
/// - [`ReactionTypeCustomEmoji`]
/// - [`ReactionTypePaid`]
/// # Documentation
/// <https://core.telegram.org/bots/api#reactiontype>
#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ReactionType {
    Emoji(ReactionTypeEmoji),
    CustomEmoji(ReactionTypeCustomEmoji),
    Paid(ReactionTypePaid),
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

    #[must_use]
    pub const fn paid() -> Self {
        Self::Paid(ReactionTypePaid::new())
    }
}

impl From<ReactionTypeEmoji> for ReactionType {
    fn from(emoji: ReactionTypeEmoji) -> Self {
        Self::Emoji(emoji)
    }
}

impl From<ReactionTypeCustomEmoji> for ReactionType {
    fn from(custom_emoji: ReactionTypeCustomEmoji) -> Self {
        Self::CustomEmoji(custom_emoji)
    }
}

impl From<ReactionTypePaid> for ReactionType {
    fn from(paid: ReactionTypePaid) -> Self {
        Self::Paid(paid)
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
    fn serialize_paid_emoji() {
        let data = ReactionType::paid();
        let json = serde_json::to_string(&data).unwrap();

        assert_eq!(json, r#"{"type":"paid"}"#);
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

    #[test]
    fn deserialize_paid() {
        let data = r#"{"type":"paid"}"#;
        let paid: ReactionType = serde_json::from_str(data).unwrap();

        assert_eq!(paid, ReactionType::paid());
    }
}
