use super::ReactionType;

use serde::Deserialize;

/// Represents a reaction added to a message along with the number of times it was added.
/// # Documentation
/// <https://core.telegram.org/bots/api#reactioncount>
#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize)]
pub struct ReactionCount {
    /// Type of the reaction
    #[serde(rename = "type")]
    pub reaction_type: ReactionType,
    /// Number of times the reaction was added
    pub total_count: i64,
}
