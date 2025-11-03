use super::{
    StoryAreaTypeLink, StoryAreaTypeLocation, StoryAreaTypeSuggestedReaction,
    StoryAreaTypeUniqueGift, StoryAreaTypeWeather,
};

use serde::Serialize;

/// Describes the type of a clickable area on a story. Currently, it can be one of
/// - [`StoryAreaTypeLocation`]
/// - [`StoryAreaTypeSuggestedReaction`]
/// - [`StoryAreaTypeLink`]
/// - [`StoryAreaTypeWeather`]
/// - [`StoryAreaTypeUniqueGift`]
/// # Documentation
/// <https://core.telegram.org/bots/api#storyareatype>
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum StoryAreaType {
    Location(StoryAreaTypeLocation),
    SuggestedReaction(StoryAreaTypeSuggestedReaction),
    Link(StoryAreaTypeLink),
    Weather(StoryAreaTypeWeather),
    Unique(StoryAreaTypeUniqueGift),
}

impl From<StoryAreaTypeLocation> for StoryAreaType {
    fn from(area: StoryAreaTypeLocation) -> Self {
        Self::Location(area)
    }
}

impl From<StoryAreaTypeSuggestedReaction> for StoryAreaType {
    fn from(area: StoryAreaTypeSuggestedReaction) -> Self {
        Self::SuggestedReaction(area)
    }
}

impl From<StoryAreaTypeLink> for StoryAreaType {
    fn from(area: StoryAreaTypeLink) -> Self {
        Self::Link(area)
    }
}

impl From<StoryAreaTypeWeather> for StoryAreaType {
    fn from(area: StoryAreaTypeWeather) -> Self {
        Self::Weather(area)
    }
}

impl From<StoryAreaTypeUniqueGift> for StoryAreaType {
    fn from(area: StoryAreaTypeUniqueGift) -> Self {
        Self::Unique(area)
    }
}
