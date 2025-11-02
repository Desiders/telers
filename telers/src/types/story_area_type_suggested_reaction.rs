use serde::Serialize;
use serde_with::skip_serializing_none;

use crate::types::ReactionType;

/// Describes a story area pointing to a suggested reaction. Currently, a story can have up to 5 suggested reaction areas.
/// # Documentation
/// <https://core.telegram.org/bots/api#storyareatypesuggestedreaction>
#[skip_serializing_none]
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize)]
pub struct StoryAreaTypeSuggestedReaction {
    /// Type of the reaction
    pub reaction_type: ReactionType,
    /// Pass `true` if the reaction area has a dark background
    pub is_dark: Option<bool>,
    /// Pass `true` if reaction area corner is flipped
    pub is_flipped: Option<bool>,
}

impl StoryAreaTypeSuggestedReaction {
    pub fn new(reaction_type: impl Into<ReactionType>) -> Self {
        Self {
            reaction_type: reaction_type.into(),
            is_dark: None,
            is_flipped: None,
        }
    }

    pub fn reaction_type(self, val: impl Into<ReactionType>) -> Self {
        Self {
            reaction_type: val.into(),
            ..self
        }
    }

    pub fn is_dark(self, val: bool) -> Self {
        Self {
            is_dark: Some(val),
            ..self
        }
    }

    pub fn is_flipped(self, val: bool) -> Self {
        Self {
            is_flipped: Some(val),
            ..self
        }
    }
}

impl StoryAreaTypeSuggestedReaction {
    pub fn is_dark_option(self, val: Option<bool>) -> Self {
        Self {
            is_dark: val,
            ..self
        }
    }

    pub fn is_flipped_option(self, val: Option<bool>) -> Self {
        Self {
            is_flipped: val,
            ..self
        }
    }
}
