use serde::{Deserialize, Serialize};
/// Describes a story area pointing to a suggested reaction. Currently, a story can have up to 5 suggested reaction areas.
/// # Documentation
/// <https://core.telegram.org/bots/api#storyareatypesuggestedreaction>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StoryAreaTypeSuggestedReaction {
    /// Type of the reaction
    pub reaction_type: crate::types::ReactionType,
    /// Pass `true` if the reaction area has a dark background
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_dark: Option<bool>,
    /// Pass `true` if reaction area corner is flipped
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_flipped: Option<bool>,
}
impl StoryAreaTypeSuggestedReaction {
    /// Creates a new `StoryAreaTypeSuggestedReaction`.
    ///
    /// # Arguments
    /// * `reaction_type` - Type of the reaction
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0: Into<crate::types::ReactionType>>(reaction_type: T0) -> Self {
        Self {
            reaction_type: reaction_type.into(),
            is_dark: None,
            is_flipped: None,
        }
    }

    /// Type of the reaction
    #[must_use]
    pub fn reaction_type<T: Into<crate::types::ReactionType>>(mut self, val: T) -> Self {
        self.reaction_type = val.into();
        self
    }

    /// Pass `true` if the reaction area has a dark background
    #[must_use]
    pub fn is_dark<T: Into<bool>>(mut self, val: T) -> Self {
        self.is_dark = Some(val.into());
        self
    }

    /// Pass `true` if the reaction area has a dark background
    #[must_use]
    pub fn is_dark_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.is_dark = val.map(Into::into);
        self
    }

    /// Pass `true` if reaction area corner is flipped
    #[must_use]
    pub fn is_flipped<T: Into<bool>>(mut self, val: T) -> Self {
        self.is_flipped = Some(val.into());
        self
    }

    /// Pass `true` if reaction area corner is flipped
    #[must_use]
    pub fn is_flipped_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.is_flipped = val.map(Into::into);
        self
    }
}
