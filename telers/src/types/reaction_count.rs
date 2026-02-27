use serde::{Deserialize, Serialize};
/// Represents a reaction added to a message along with the number of times it was added.
/// # Documentation
/// <https://core.telegram.org/bots/api#reactioncount>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ReactionCount {
    /// Type of the reaction
    pub r#type: crate::types::ReactionType,
    /// Number of times the reaction was added
    pub total_count: i64,
}
impl ReactionCount {
    /// Creates a new `ReactionCount`.
    ///
    /// # Arguments
    /// * `type` - Type of the reaction
    /// * `total_count` - Number of times the reaction was added
    #[must_use]
    pub fn new<T0: Into<crate::types::ReactionType>, T1: Into<i64>>(
        r#type: T0,
        total_count: T1,
    ) -> Self {
        Self {
            r#type: r#type.into(),
            total_count: total_count.into(),
        }
    }

    /// Type of the reaction
    #[must_use]
    pub fn r#type<T: Into<crate::types::ReactionType>>(self, val: T) -> Self {
        let mut this = self;
        this.r#type = val.into();
        this
    }

    /// Number of times the reaction was added
    #[must_use]
    pub fn total_count<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.total_count = val.into();
        this
    }
}
